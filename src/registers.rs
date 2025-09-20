//!
//! レジスタの定数値を列挙するためのモジュール
//!

pub const HCR_EL2_API: u64 = 1 << 41;
pub const HCR_EL2_RW: u64 = 1 << 31;

pub const SPSR_EL2_M_EL1H: u64 = 0b0101;
