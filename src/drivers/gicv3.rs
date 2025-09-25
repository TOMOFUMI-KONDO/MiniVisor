//!
//! GICv3 割り込み管理モジュール
//!

use crate::asm;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GicGroup {
    NonSecureGroup1,
}

pub struct GicDistributor {
    base_address: usize,
}

impl GicDistributor {
    const GICD_MMIO_SIZE: usize = 0x10000;
    const GICD_CTLR: usize = 0x00;
    const GICD_CTLR_RWP: u32 = 1 << 31;
    const GICD_CTLR_ARE: u32 = 1 << 5;
    const GICD_CTLR_ENABLE_GRP1NS: u32 = 1 << 1;
    const GICD_IGROUPR: usize = 0x0080;
    const GICD_ISENABLER: usize = 0x0100;
    const GICD_ICENABLER: usize = 0x0180;
    const GICD_ISPENDR: usize = 0x0200;
    const GICD_ICPENDR: usize = 0x0280;
    const GICD_IPRIORITYR: usize = 0x0400;
    const GICD_ICFGR: usize = 0x0C00;
    const GICD_IGRPMODR: usize = 0x0D00;
    const GICD_IROUTER: usize = 0x6100;

    pub fn init(&self) {
        self.write_register(Self::GICD_CTLR, Self::GICD_CTLR_ARE);
        self.wait_rwp();
        self.write_register(
            Self::GICD_CTLR,
            Self::GICD_CTLR_ARE | Self::GICD_CTLR_ENABLE_GRP1NS,
        );
    }

    // 設定する割り込みIDのビット範囲を一度0埋めしてから `priority` で上書きする。
    pub fn set_priority(&self, int_id: u32, priority: u8) {
        let register_index = ((int_id >> 2) as usize) * size_of::<u32>();
        let register_offset = (int_id & 0b11) << 3;
        self.write_register(
            Self::GICD_IPRIORITYR + register_index,
            (self.read_register(Self::GICD_IPRIORITYR + register_index)
                & !(0xFF << register_offset))
                | ((priority as u32) << register_offset),
        );
    }

    pub fn set_group(&self, int_id: u32, group: GicGroup) {
        let register_index = ((int_id / u32::BITS) as usize) * size_of::<u32>();
        let register_offset = int_id & (u32::BITS - 1);

        // Non Secure World のグループ1割り込みのみ対応
        let data = match group {
            GicGroup::NonSecureGroup1 => 1,
        };
        self.write_register(
            Self::GICD_IGROUPR + register_index,
            (self.read_register(Self::GICD_IGROUPR + register_index) & !(1 << register_offset))
                | (data << register_offset),
        );

        // Non Secure World のグループ1割り込みのみ対応
        let data = match group {
            GicGroup::NonSecureGroup1 => 0,
        };
        self.write_register(
            Self::GICD_IGRPMODR + register_index,
            (self.read_register(Self::GICD_IGRPMODR + register_index) & !(1 << register_offset))
                | (data << register_offset),
        );
    }

    pub fn set_trigger_mode(&self, int_id: u32, is_level_trigger: bool) {
        let register_index = ((int_id >> 4) as usize) * size_of::<u32>();
        let register_offset = int_id & 0b1111 << 1;
        // TODO
    }

    fn wait_rwp(&self) {
        while (self.read_register(Self::GICD_CTLR) & Self::GICD_CTLR_RWP) != 0 {
            core::hint::spin_loop();
        }
    }

    fn read_register(&self, register: usize) -> u32 {
        unsafe { core::ptr::read_volatile((self.base_address + register) as *const u32) }
    }

    fn write_register(&self, register: usize, data: u32) {
        unsafe { core::ptr::write_volatile((self.base_address + register) as *mut u32, data) }
    }
}
