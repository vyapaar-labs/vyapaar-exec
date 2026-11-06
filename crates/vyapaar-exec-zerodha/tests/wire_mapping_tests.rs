//! Zerodha Kite Connect v3 wire-token → canonical-enum mapping table tests.
//!
//! Pins the table-driven vocabulary defined in [`vyapaar_exec_zerodha::mapping`]
//! so the ad-hoc match chains they replaced cannot silently regress.

use serde_json::json;
use vyapaar_core::{Exchange, OrderSide, OrderStatus, OrderType, Price};
use vyapaar_exec_zerodha::mapping;

#[test]
fn order_status_maps_kite_vocabulary() {
    assert_eq!(mapping::order_status("COMPLETE"), OrderStatus::Filled);
    assert_eq!(mapping::order_status("REJECTED"), OrderStatus::Rejected);
    assert_eq!(mapping::order_status("CANCELLED"), OrderStatus::Cancelled);
    assert_eq!(mapping::order_status("OPEN"), OrderStatus::New);
}

#[test]
fn order_status_defaults_intermediate_statuses_to_pending_new() {
    // Kite's acceptance/queueing statuses are all "not yet working".
    assert_eq!(
        mapping::order_status("VALIDATION PENDING"),
        OrderStatus::PendingNew
    );
    assert_eq!(
        mapping::order_status("OPEN PENDING"),
        OrderStatus::PendingNew
    );
    assert_eq!(
        mapping::order_status("AMO QUEUE PENDING"),
        OrderStatus::PendingNew
    );
    assert_eq!(
        mapping::order_status("unknown status"),
        OrderStatus::PendingNew
    );
}

#[test]
fn order_status_is_case_insensitive() {
    assert_eq!(mapping::order_status("complete"), OrderStatus::Filled);
    assert_eq!(mapping::order_status("Open"), OrderStatus::New);
}

#[test]
fn order_side_maps_buy_and_sell() {
    assert_eq!(mapping::order_side("BUY"), OrderSide::Buy);
    assert_eq!(mapping::order_side("SELL"), OrderSide::Sell);
    // Unmapped transaction types default to Buy (Kite's lenient parse).
    assert_eq!(mapping::order_side("unknown"), OrderSide::Buy);
}

#[test]
fn exchange_maps_bse_mcx_nse() {
    assert_eq!(mapping::exchange("BSE"), Exchange::Bse);
    assert_eq!(mapping::exchange("MCX"), Exchange::Mcx);
    assert_eq!(mapping::exchange("NSE"), Exchange::Nse);
    // Derivative/other segments default to NSE in the current adapter scope.
    assert_eq!(mapping::exchange("NFO"), Exchange::Nse);
    assert_eq!(mapping::exchange(""), Exchange::Nse);
}

#[test]
fn order_type_maps_shapes_and_prices() {
    let price = Price::from_rupees("1550.00").unwrap();
    let trigger = Price::from_rupees("1545.50").unwrap();

    assert_eq!(
        mapping::order_type("LIMIT", Some(json!("1550.00")), None),
        OrderType::Limit(price)
    );
    assert_eq!(
        mapping::order_type("SL-M", None, Some(json!("1545.50"))),
        OrderType::StopLoss(trigger)
    );
    assert_eq!(
        mapping::order_type("SL", Some(json!("1550.00")), Some(json!("1545.50"))),
        OrderType::StopLimit {
            trigger,
            limit: price
        }
    );
    assert_eq!(mapping::order_type("MARKET", None, None), OrderType::Market);
    // Unmapped Kite varieties (CO, BO, meta) default to market.
    assert_eq!(mapping::order_type("CO", None, None), OrderType::Market);
}

#[test]
fn ticker_status_event_maps_ack_vocabulary() {
    assert_eq!(mapping::ticker_status_event("OPEN"), Some(OrderStatus::New));
    assert_eq!(
        mapping::ticker_status_event("VALIDATION PENDING"),
        Some(OrderStatus::PendingNew)
    );
    assert_eq!(
        mapping::ticker_status_event("AMO CANCEL PENDING"),
        Some(OrderStatus::PendingNew)
    );
    // Fill/reject/cancel statuses are dispatched as events, not statuses.
    assert_eq!(mapping::ticker_status_event("COMPLETE"), None);
    assert_eq!(mapping::ticker_status_event("REJECTED"), None);
}
