use num_traits::cast::{FromPrimitive, ToPrimitive};
use sea_orm::prelude::Decimal;

/// Convert f64 to Decimal, returning Decimal::ZERO on failure.
pub fn dec_from_f64(v: f64) -> Decimal {
    FromPrimitive::from_f64(v).unwrap_or_default()
}

/// Convert Decimal to f64, returning 0.0 on failure.
pub fn dec_to_f64(d: &Decimal) -> f64 {
    ToPrimitive::to_f64(d).unwrap_or(0.0)
}


