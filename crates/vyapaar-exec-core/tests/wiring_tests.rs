//! Table-driven broker wire-token → canonical-enum mapping tests.
//!
//! Covers the generic [`BrokerEnumMap`] lookup table and the [`OrderTypeKind`]
//! discriminant conversion that broker adapters build their per-broker wire
//! vocabularies on. Lives in `tests/` parallel to `src/` per the workspace
//! TDD rule (no inline `#[cfg(test)]` in production files).

use vyapaar_core::{OrderSide, OrderStatus, OrderType, Price};
use vyapaar_exec_core::wiring::{BrokerEnumMap, OrderTypeKind};

#[test]
fn get_maps_exact_tokens() {
    let statuses: BrokerEnumMap<OrderStatus> = BrokerEnumMap::new(&[
        ("COMPLETE", OrderStatus::Filled),
        ("REJECTED", OrderStatus::Rejected),
    ]);
    assert_eq!(statuses.get("COMPLETE"), Some(OrderStatus::Filled));
    assert_eq!(statuses.get("REJECTED"), Some(OrderStatus::Rejected));
}

#[test]
fn get_is_ascii_case_insensitive() {
    let statuses: BrokerEnumMap<OrderStatus> = BrokerEnumMap::new(&[
        ("COMPLETE", OrderStatus::Filled),
        ("OPEN", OrderStatus::New),
    ]);
    assert_eq!(statuses.get("complete"), Some(OrderStatus::Filled));
    assert_eq!(statuses.get("Open"), Some(OrderStatus::New));
    assert_eq!(statuses.get("cOmPlEtE"), Some(OrderStatus::Filled));
}

#[test]
fn get_returns_none_for_unmapped_tokens() {
    let statuses: BrokerEnumMap<OrderStatus> =
        BrokerEnumMap::new(&[("COMPLETE", OrderStatus::Filled)]);
    assert_eq!(statuses.get("PARTIALLY FILLED"), None);
    assert_eq!(statuses.get(""), None);
}

#[test]
fn entries_and_tokens_reflect_the_table() {
    let sides: BrokerEnumMap<OrderSide> =
        BrokerEnumMap::new(&[("BUY", OrderSide::Buy), ("SELL", OrderSide::Sell)]);
    assert_eq!(sides.len(), 2);
    assert!(!sides.is_empty());

    let entries = sides.entries();
    assert_eq!(entries[0], ("BUY", OrderSide::Buy));
    assert_eq!(entries[1], ("SELL", OrderSide::Sell));

    let tokens: Vec<&'static str> = sides.tokens().collect();
    assert_eq!(tokens, vec!["BUY", "SELL"]);
}

#[test]
fn empty_table_maps_nothing() {
    let empty: BrokerEnumMap<OrderStatus> = BrokerEnumMap::new(&[]);
    assert!(empty.is_empty());
    assert_eq!(empty.len(), 0);
    assert_eq!(empty.get("COMPLETE"), None);
}

#[test]
fn order_type_kind_into_order_type_preserves_payloads() {
    let price = Price::from_rupees("1550.00").unwrap();
    let trigger = Price::from_rupees("1545.50").unwrap();

    assert_eq!(
        OrderTypeKind::Market.into_order_type(price, trigger),
        OrderType::Market
    );
    assert_eq!(
        OrderTypeKind::Limit.into_order_type(price, trigger),
        OrderType::Limit(price)
    );
    assert_eq!(
        OrderTypeKind::StopLoss.into_order_type(price, trigger),
        OrderType::StopLoss(trigger)
    );
    assert_eq!(
        OrderTypeKind::StopLimit.into_order_type(price, trigger),
        OrderType::StopLimit {
            trigger,
            limit: price
        }
    );
}
