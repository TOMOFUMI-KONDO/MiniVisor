//!
//! Stage2 Paging の実装
//!

use crate::asm;
use crate::registers::*;

#[derive(Clone)]
struct Descriptor(u64);

pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;

impl Descriptor {
    const TABLE_ADDRESS_MASK: u64 = ((1 << 52) - 1) & !(PAGE_SIZE as u64 - 1);
    const OUTPUT_ADDRESS_MASK: u64 = ((1 << 50) - 1) & !(PAGE_SIZE as u64 - 1);

    const AF_OFFSET: u64 = 10;
    const AF: u64 = 1 << Self::AF_OFFSET;

    const SH_OFFSET: u64 = 8;
    const SH: u64 = 0b11 << Self::SH_OFFSET;

    const S2AP_OFFSET: u64 = 6;
    const S2AP: u64 = 0b11 << Self::S2AP_OFFSET;

    const fn new() -> Self {
        Self(0)
    }

    fn init(&mut self) {
        *self = Self::new();
    }

    fn validate_as_page_descriptor(&mut self) {
        self.0 |= 0b11;
    }

    fn validate_as_table_descriptor(&mut self) {
        self.0 |= 0b11;
    }

    fn validate_as_block_descriptor(&mut self) {
        self.0 |= 0b01;
    }

    const fn is_table_descriptor(&self) -> bool {
        (self.0 & 0b11) == 0b11
    }

    const fn get_next_level_table_address(&self) -> usize {
        (self.0 & Self::TABLE_ADDRESS_MASK) as usize
    }

    const fn set_output_address(&mut self, output_address: usize) {
        self.0 = (self.0 & !Self::OUTPUT_ADDRESS_MASK) | (output_address as u64) | Self::AF;
    }

    const fn set_permission(&mut self, permission: u64) {
        self.0 = (self.0 & !Self::S2AP) | (permission << Self::S2AP_OFFSET);
    }
}

fn number_of_concatenated_page_tables(t0sz: u8, first_level: i8) -> usize {
    if t0sz > (43 - ((3 - first_level) as u8) * 9) {
        1
    } else {
        2usize.pow(((43 - ((3 - first_level) as u8) * 9) - t0sz) as u32)
    }
}

pub fn init_stage2_translation_table() {
    let ps = asm::get_id_aa64mmfr0_el1() & ID_AA64MMFR0_EL1_PARANGE;
    let (t0sz, initial_lookup_level) = match ps {
        0b000 => (32u64, 1i8),
        0b001 => (28u64, 1i8),
        0b010 => (24u64, 1i8),
        0b011 => (22u64, 1i8),
        0b100 => (20u64, 0i8),
        0b101 => (16u64, 0i8),
        _ => (16u64, 0i8),
    };
    // let num_of_descriptors =
}
