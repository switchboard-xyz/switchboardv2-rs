use super::error::SwitchboardError;
use anchor_lang::{zero_copy, AnchorDeserialize, AnchorSerialize};
use core::cmp::Ordering;
use rust_decimal::Decimal;
use solana_program::program_error::ProgramError;
use std::convert::From;
use std::convert::TryInto;

#[zero_copy]
#[derive(Default, Debug, Eq, PartialEq, AnchorDeserialize)]
pub struct SwitchboardDecimal {
    pub mantissa: i128,
    pub scale: u32,
}

#[derive(Default, Eq, PartialEq, Copy, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct BorshDecimal {
    pub mantissa: i128,
    pub scale: u32,
}

impl From<SwitchboardDecimal> for Decimal {
    fn from(item: SwitchboardDecimal) -> Self {
        Decimal::from_i128_with_scale(item.mantissa, item.scale)
    }
}

impl SwitchboardDecimal {
    pub fn new(mantissa: i128, scale: u32) -> SwitchboardDecimal {
        Self { mantissa, scale }
    }
    pub fn from_rust_decimal(d: Decimal) -> SwitchboardDecimal {
        Self::new(d.mantissa(), d.scale())
    }
}

impl TryInto<Decimal> for &SwitchboardDecimal {
    type Error = ProgramError;
    fn try_into(self) -> core::result::Result<Decimal, ProgramError> {
        Decimal::try_from_i128_with_scale(self.mantissa, self.scale)
            .map_err(|_| ProgramError::from(SwitchboardError::DecimalConversionError))
    }
}

impl From<SwitchboardDecimal> for BorshDecimal {
    fn from(s: SwitchboardDecimal) -> Self {
        Self {
            mantissa: s.mantissa,
            scale: s.scale,
        }
    }
}

impl Into<SwitchboardDecimal> for BorshDecimal {
    fn into(self) -> SwitchboardDecimal {
        SwitchboardDecimal {
            mantissa: self.mantissa,
            scale: self.scale,
        }
    }
}

impl Ord for SwitchboardDecimal {
    fn cmp(&self, other: &Self) -> Ordering {
        let s: Decimal = self.try_into().unwrap();
        let other: Decimal = other.try_into().unwrap();
        s.cmp(&other)
    }
}

impl PartialOrd for SwitchboardDecimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let s: Decimal = self.try_into().unwrap();
        let other: Decimal = other.try_into().unwrap();
        s.partial_cmp(&other)
    }
    fn lt(&self, other: &Self) -> bool {
        let s: Decimal = self.try_into().unwrap();
        let other: Decimal = other.try_into().unwrap();
        s < other
    }
    fn le(&self, other: &Self) -> bool {
        let s: Decimal = self.try_into().unwrap();
        let other: Decimal = other.try_into().unwrap();
        s <= other
    }
    fn gt(&self, other: &Self) -> bool {
        let s: Decimal = self.try_into().unwrap();
        let other: Decimal = other.try_into().unwrap();
        s > other
    }
    fn ge(&self, other: &Self) -> bool {
        let s: Decimal = self.try_into().unwrap();
        let other: Decimal = other.try_into().unwrap();
        s >= other
    }
}

impl From<SwitchboardDecimal> for bool {
    fn from(s: SwitchboardDecimal) -> Self {
        let val: Decimal = s.into();
        val.round().mantissa() != 0
    }
}
