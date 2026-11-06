//! Table-driven broker wire-token → canonical-enum translation.
//!
//! The `BrokerEnumMap<T>` retrofit pattern: every broker adapter used to
//! translate broker-specific wire enums (order status, side, order type,
//! exchange segment) into this repo's canonical enums with ad-hoc per-broker
//! `match`/if-chains, each carrying its own silent fallback defaults. That
//! duplicates the same logic across every adapter and makes an unmapped wire
//! token's behavior implicit.
//!
//! This module replaces those chains with one declarative data table per
//! broker axis. The table *is* the contract — adding a broker or a token means
//! editing a table, not adding match arms — and lookups are uniform and
//! case-insensitive (brokers emit both `COMPLETE` and `complete`).
//!
//! `OrderType` carries price payloads on the wire record, so its tables map
//! tokens to an [`OrderTypeKind`] discriminant and the adapter applies the
//! decoded prices via [`OrderTypeKind::into_order_type`].

use vyapaar_core::{OrderType, Price};

/// A compile-time data table mapping broker-specific wire tokens to a
/// canonical enum value.
///
/// Zero-allocation linear lookup (tables are 2–8 entries), WASM-portable, and
/// pure: no I/O, no clock, no floats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BrokerEnumMap<T: 'static> {
    table: &'static [(&'static str, T)],
}

impl<T: 'static> BrokerEnumMap<T> {
    /// A table-driven wire-token → canonical-value map.
    ///
    /// Must be constructed from a `'static` slice so it can live in a `const`
    /// next to the adapter it documents.
    pub const fn new(table: &'static [(&'static str, T)]) -> Self {
        Self { table }
    }

    /// The raw `(wire token, canonical value)` pairs.
    pub fn entries(&self) -> &'static [(&'static str, T)] {
        self.table
    }

    /// Every wire token this table knows, in declaration order.
    pub fn tokens(&self) -> impl Iterator<Item = &'static str> {
        self.table.iter().map(|(token, _)| *token)
    }

    /// Number of wire tokens mapped.
    pub fn len(&self) -> usize {
        self.table.len()
    }

    /// Whether this table maps no tokens.
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }
}

impl<T: Copy> BrokerEnumMap<T> {
    /// Look up a wire token (ASCII case-insensitive) and return its canonical
    /// value.
    ///
    /// Unmapped tokens return `None`; the adapter supplies the fallback that
    /// matches its broker's documented behavior (e.g. an unknown order status
    /// means [`vyapaar_core::OrderStatus::PendingNew`], never a panic).
    pub fn get(&self, wire: &str) -> Option<T> {
        self.table
            .iter()
            .find(|(token, _)| token.eq_ignore_ascii_case(wire))
            .map(|(_, value)| *value)
    }
}

/// Discriminant of [`OrderType`] without the price payloads.
///
/// Wire order-type tokens name the *shape* of an order (`LIMIT`, `SL-M`, `SL`,
/// `MARKET`); the price/trigger values ride alongside in the wire record.
/// Broker tables therefore map tokens to a [`OrderTypeKind`] and the adapter
/// fills the payloads in with [`OrderTypeKind::into_order_type`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrderTypeKind {
    /// See [`OrderType::Market`].
    Market,
    /// See [`OrderType::Limit`].
    Limit,
    /// See [`OrderType::StopLoss`].
    StopLoss,
    /// See [`OrderType::StopLimit`].
    StopLimit,
}

impl OrderTypeKind {
    /// The canonical [`OrderType`] of this shape, carrying the wire record's
    /// `price` (limit leg) and `trigger` (stop leg) values.
    pub fn into_order_type(self, price: Price, trigger: Price) -> OrderType {
        match self {
            Self::Market => OrderType::Market,
            Self::Limit => OrderType::Limit(price),
            Self::StopLoss => OrderType::StopLoss(trigger),
            Self::StopLimit => OrderType::StopLimit {
                trigger,
                limit: price,
            },
        }
    }
}
