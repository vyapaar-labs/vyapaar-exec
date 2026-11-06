//! Zerodha Kite Connect v3 wire-token → canonical-enum mapping tables.
//!
//! Table-driven replacement for the ad-hoc per-broker match chains that
//! previously translated Kite's order status, side, order-type and exchange
//! tokens inside `lib.rs`/`ticker.rs`/`postback.rs`. Every Kite wire token an
//! adapter can meet is declared here once, as data; the REST, Ticker and
//! postback paths all look the same tables up through the small functions
//! below, and an unmapped token's fallback is documented on each table.
//!
//! Lookups are ASCII case-insensitive — Kite emits both `COMPLETE` and
//! `complete` in the wild.

use serde_json::Value;
use vyapaar_core::{Exchange, OrderSide, OrderStatus, OrderType, Price};
use vyapaar_exec_core::wire;
use vyapaar_exec_core::wiring::{BrokerEnumMap, OrderTypeKind};

/// Kite Connect order-status tokens → canonical [`OrderStatus`].
///
/// Kite's acceptance/queueing statuses (`VALIDATION PENDING`, `OPEN PENDING`,
/// `TRIGGER PENDING`, the AMO queue legs) are all "accepted but not working"
/// and intentionally omitted: they fall back to [`OrderStatus::PendingNew`].
pub const KITE_ORDER_STATUS: BrokerEnumMap<OrderStatus> = BrokerEnumMap::new(&[
    ("COMPLETE", OrderStatus::Filled),
    ("REJECTED", OrderStatus::Rejected),
    ("CANCELLED", OrderStatus::Cancelled),
    ("OPEN", OrderStatus::New),
]);

/// Translate a Kite order-status token into the canonical status.
pub fn order_status(status: &str) -> OrderStatus {
    KITE_ORDER_STATUS
        .get(status)
        .unwrap_or(OrderStatus::PendingNew)
}

/// Kite Connect order-side (`transaction_type`) tokens → canonical [`OrderSide`].
pub const KITE_ORDER_SIDE: BrokerEnumMap<OrderSide> =
    BrokerEnumMap::new(&[("BUY", OrderSide::Buy), ("SELL", OrderSide::Sell)]);

/// Translate a Kite `transaction_type` token into the canonical side.
///
/// Unmapped tokens default to [`OrderSide::Buy`], matching Kite's lenient
/// parse of unknown transaction types.
pub fn order_side(transaction_type: &str) -> OrderSide {
    KITE_ORDER_SIDE
        .get(transaction_type)
        .unwrap_or(OrderSide::Buy)
}

/// Kite Connect exchange tokens → canonical [`Exchange`].
///
/// Only the equities segments Kite serves (`NSE`/`BSE`) and MCX futures are
/// enumerated; anything else — including derivative segments this adapter
/// does not yet route — falls back to [`Exchange::Nse`].
pub const KITE_EXCHANGE: BrokerEnumMap<Exchange> = BrokerEnumMap::new(&[
    ("NSE", Exchange::Nse),
    ("BSE", Exchange::Bse),
    ("MCX", Exchange::Mcx),
]);

/// Translate a Kite exchange token into the canonical exchange (default NSE).
pub fn exchange(exchange: &str) -> Exchange {
    KITE_EXCHANGE.get(exchange).unwrap_or(Exchange::Nse)
}

/// Kite Connect order-type tokens → order-type *shape*.
///
/// Kite's limit/trigger prices ride on the wire record next to the token, so
/// the table maps to an [`OrderTypeKind`] discriminant and [`order_type`]
/// applies the decoded prices.
pub const KITE_ORDER_TYPE: BrokerEnumMap<OrderTypeKind> = BrokerEnumMap::new(&[
    ("MARKET", OrderTypeKind::Market),
    ("LIMIT", OrderTypeKind::Limit),
    ("SL-M", OrderTypeKind::StopLoss),
    ("SL", OrderTypeKind::StopLimit),
]);

/// Translate the Kite order-type token + price/trigger trio into [`OrderType`].
///
/// Unmapped Kite varieties (CO/BO/meta) default to [`OrderType::Market`],
/// matching the previous match-chain behavior.
pub fn order_type(order_type: &str, price: Option<Value>, trigger: Option<Value>) -> OrderType {
    let price = price
        .and_then(|v| wire::price_from_json(&v))
        .unwrap_or(Price::ZERO);
    let trigger = trigger
        .and_then(|v| wire::price_from_json(&v))
        .unwrap_or(Price::ZERO);
    KITE_ORDER_TYPE
        .get(order_type)
        .unwrap_or(OrderTypeKind::Market)
        .into_order_type(price, trigger)
}

/// Kite Ticker order-update statuses that translate to a canonical status.
///
/// The Ticker's ack vocabulary extends the REST one with the acceptance/queue
/// legs. Fill, reject and cancel statuses are *not* in this table — the Ticker
/// stream dispatches those as events, not statuses — so `get` returns `None`
/// for them.
pub const KITE_TICKER_STATUS: BrokerEnumMap<OrderStatus> = BrokerEnumMap::new(&[
    ("OPEN", OrderStatus::New),
    ("OPEN PENDING", OrderStatus::PendingNew),
    ("PUT ORDER REQ RECEIVED", OrderStatus::PendingNew),
    ("VALIDATION PENDING", OrderStatus::PendingNew),
    ("AMO QUEUE PENDING", OrderStatus::PendingNew),
    ("TRIGGER PENDING", OrderStatus::PendingNew),
    ("AMO CANCEL PENDING", OrderStatus::PendingNew),
]);

/// Translate a Kite Ticker order-status token into an optional canonical
/// status (`None` for statuses the stream dispatches as events).
pub fn ticker_status_event(status: &str) -> Option<OrderStatus> {
    KITE_TICKER_STATUS.get(status)
}
