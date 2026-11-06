//! Upstox v2 wire-token → canonical-enum mapping tables.
//!
//! Table-driven replacement for the ad-hoc match chains that previously
//! translated Upstox's order status, side, order-type and exchange-segment
//! tokens inside `lib.rs`/`ws.rs`. Every Upstox wire token an adapter can
//! meet is declared here once, as data, and the REST and WebSocket feed paths
//! both look the same tables up through the small functions below.
//!
//! Lookups are ASCII case-insensitive — Upstox emits both `COMPLETE` and
//! `complete` in the wild.

use serde_json::Value;
use vyapaar_core::{Exchange, OrderSide, OrderStatus, OrderType, Price};
use vyapaar_exec_core::wire;
use vyapaar_exec_core::wiring::{BrokerEnumMap, OrderTypeKind};

/// Upstox v2 order-status tokens → canonical [`OrderStatus`].
///
/// Both the REST order book (`complete`, `rejected`, `cancelled`, `open`) and
/// the portfolio WebSocket feed (`new`, `trigger pending`, `put order req
/// received`) share this vocabulary; unmapped/transitional statuses fall back
/// to [`OrderStatus::PendingNew`].
pub const UPSTOX_ORDER_STATUS: BrokerEnumMap<OrderStatus> = BrokerEnumMap::new(&[
    ("COMPLETE", OrderStatus::Filled),
    ("FILLED", OrderStatus::Filled),
    ("REJECTED", OrderStatus::Rejected),
    ("CANCELLED", OrderStatus::Cancelled),
    ("OPEN", OrderStatus::New),
    ("NEW", OrderStatus::New),
    ("TRIGGER PENDING", OrderStatus::PendingNew),
    ("PUT ORDER REQ RECEIVED", OrderStatus::PendingNew),
]);

/// Translate an Upstox order-status token into the canonical status.
pub fn order_status(status: &str) -> OrderStatus {
    UPSTOX_ORDER_STATUS
        .get(status)
        .unwrap_or(OrderStatus::PendingNew)
}

/// Upstox order-side (`transaction_type`) tokens → canonical [`OrderSide`].
pub const UPSTOX_ORDER_SIDE: BrokerEnumMap<OrderSide> =
    BrokerEnumMap::new(&[("BUY", OrderSide::Buy), ("SELL", OrderSide::Sell)]);

/// Translate an Upstox `transaction_type` token into the canonical side.
///
/// Unmapped tokens default to [`OrderSide::Buy`], matching the previous
/// match-chain behavior.
pub fn order_side(transaction_type: &str) -> OrderSide {
    UPSTOX_ORDER_SIDE
        .get(transaction_type)
        .unwrap_or(OrderSide::Buy)
}

/// Upstox exchange-segment tokens → canonical [`Exchange`].
///
/// Upstox reports exchanges as instrument-token segments (`NSE_EQ`,
/// `MCX_FO`, `BSE_EQ`, …) as well as bare codes; every documented segment is
/// enumerated here and unknown segments fall back to [`Exchange::Nse`].
pub const UPSTOX_EXCHANGE: BrokerEnumMap<Exchange> = BrokerEnumMap::new(&[
    ("NSE", Exchange::Nse),
    ("NSE_EQ", Exchange::Nse),
    ("NSE_FO", Exchange::Nse),
    ("NSE_CDS", Exchange::Nse),
    ("BSE", Exchange::Bse),
    ("BSE_EQ", Exchange::Bse),
    ("BSE_FO", Exchange::Bse),
    ("MCX", Exchange::Mcx),
    ("MCX_EQ", Exchange::Mcx),
    ("MCX_FO", Exchange::Mcx),
]);

/// Translate an Upstox exchange-segment token into the canonical exchange
/// (default NSE).
pub fn exchange(exchange: &str) -> Exchange {
    UPSTOX_EXCHANGE.get(exchange).unwrap_or(Exchange::Nse)
}

/// Upstox order-type tokens → order-type *shape*.
///
/// Upstox's limit/trigger prices ride on the wire record next to the token, so
/// the table maps to an [`OrderTypeKind`] discriminant and [`order_type`]
/// applies the decoded prices.
pub const UPSTOX_ORDER_TYPE: BrokerEnumMap<OrderTypeKind> = BrokerEnumMap::new(&[
    ("MARKET", OrderTypeKind::Market),
    ("LIMIT", OrderTypeKind::Limit),
    ("SL-M", OrderTypeKind::StopLoss),
    ("SL", OrderTypeKind::StopLimit),
]);

/// Translate the Upstox order-type token + price/trigger trio into [`OrderType`].
///
/// Unmapped order types (AMO scheduling legs, exotic varieties) default to
/// [`OrderType::Market`], matching the previous match-chain behavior.
pub fn order_type(order_type: &str, price: Option<Value>, trigger: Option<Value>) -> OrderType {
    let price = price
        .and_then(|v| wire::price_from_json(&v))
        .unwrap_or(Price::ZERO);
    let trigger = trigger
        .and_then(|v| wire::price_from_json(&v))
        .unwrap_or(Price::ZERO);
    UPSTOX_ORDER_TYPE
        .get(order_type)
        .unwrap_or(OrderTypeKind::Market)
        .into_order_type(price, trigger)
}
