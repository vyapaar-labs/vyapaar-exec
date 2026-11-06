//! Upstox v2 wire-token → canonical-enum mapping table tests.
//!
//! Pins the table-driven vocabulary defined in [`vyapaar_exec_upstox::mapping`]
//! so the ad-hoc match chains they replaced cannot silently regress.

use serde_json::json;
use vyapaar_core::{Exchange, OrderSide, OrderStatus, OrderType, Price};
use vyapaar_exec_upstox::mapping;

#[test]
fn order_status_maps_upstox_vocabulary() {
    assert_eq!(mapping::order_status("COMPLETE"), OrderStatus::Filled);
    assert_eq!(mapping::order_status("FILLED"), OrderStatus::Filled);
    assert_eq!(mapping::order_status("REJECTED"), OrderStatus::Rejected);
    assert_eq!(mapping::order_status("CANCELLED"), OrderStatus::Cancelled);
    assert_eq!(mapping::order_status("OPEN"), OrderStatus::New);
    assert_eq!(mapping::order_status("NEW"), OrderStatus::New);
    assert_eq!(
        mapping::order_status("TRIGGER PENDING"),
        OrderStatus::PendingNew
    );
    assert_eq!(
        mapping::order_status("PUT ORDER REQ RECEIVED"),
        OrderStatus::PendingNew
    );
    // Unmapped/transitional statuses fall back to pending-new.
    assert_eq!(
        mapping::order_status("VALIDATION PENDING"),
        OrderStatus::PendingNew
    );
}

#[test]
fn order_status_is_case_insensitive() {
    assert_eq!(mapping::order_status("complete"), OrderStatus::Filled);
    assert_eq!(mapping::order_status("open"), OrderStatus::New);
}

#[test]
fn order_side_maps_buy_and_sell() {
    assert_eq!(mapping::order_side("BUY"), OrderSide::Buy);
    assert_eq!(mapping::order_side("SELL"), OrderSide::Sell);
    // Unmapped transaction types default to Buy.
    assert_eq!(mapping::order_side("SHORT"), OrderSide::Buy);
}

#[test]
fn exchange_maps_segment_tokens() {
    assert_eq!(mapping::exchange("BSE"), Exchange::Bse);
    assert_eq!(mapping::exchange("BSE_EQ"), Exchange::Bse);
    assert_eq!(mapping::exchange("BSE_FO"), Exchange::Bse);
    assert_eq!(mapping::exchange("MCX"), Exchange::Mcx);
    assert_eq!(mapping::exchange("MCX_EQ"), Exchange::Mcx);
    assert_eq!(mapping::exchange("MCX_FO"), Exchange::Mcx);
    assert_eq!(mapping::exchange("NSE_EQ"), Exchange::Nse);
    // Unmapped/unknown segments default to NSE.
    assert_eq!(mapping::exchange("NFO"), Exchange::Nse);
    assert_eq!(mapping::exchange(""), Exchange::Nse);
}

#[test]
fn order_type_maps_shapes_and_prices() {
    let price = Price::from_rupees("470.25").unwrap();
    let trigger = Price::from_rupees("51200").unwrap();

    assert_eq!(
        mapping::order_type("LIMIT", Some(json!("470.25")), None),
        OrderType::Limit(price)
    );
    assert_eq!(
        mapping::order_type("SL-M", None, Some(json!("51200"))),
        OrderType::StopLoss(trigger)
    );
    assert_eq!(mapping::order_type("MARKET", None, None), OrderType::Market);
    // Unmapped order types (AMO scheduling tokens etc.) default to market.
    assert_eq!(mapping::order_type("AMO", None, None), OrderType::Market);
}
