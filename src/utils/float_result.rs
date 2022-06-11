use crate::*;

pub type FloatResult = Result<Float, String>;

pub trait FloatResultExt {
    fn ok_zero(format: Format) -> Self;
}

impl FloatResultExt for FloatResult {
    /// Get a FloatResult with an Ok variant of result with float set to all zero.
    fn ok_zero(format: Format) -> Self {
        Ok(Float::from_bits(format, BitPattern::from_value(0u32)).unwrap())
    }
}