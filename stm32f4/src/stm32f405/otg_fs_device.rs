///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - OTG_FS device configuration register (OTG_FS_DCFG)
    pub dcfg: DCFG,
    ///0x04 - OTG_FS device control register (OTG_FS_DCTL)
    pub dctl: DCTL,
    ///0x08 - OTG_FS device status register (OTG_FS_DSTS)
    pub dsts: DSTS,
    _reserved3: [u8; 0x04],
    ///0x10 - OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)
    pub diepmsk: DIEPMSK,
    ///0x14 - OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)
    pub doepmsk: DOEPMSK,
    ///0x18 - OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)
    pub daint: DAINT,
    ///0x1c - OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)
    pub daintmsk: DAINTMSK,
    _reserved7: [u8; 0x08],
    ///0x28 - OTG_FS device VBUS discharge time register
    pub dvbusdis: DVBUSDIS,
    ///0x2c - OTG_FS device VBUS pulsing time register
    pub dvbuspulse: DVBUSPULSE,
    _reserved9: [u8; 0x04],
    ///0x34 - OTG_FS device IN endpoint FIFO empty interrupt mask register
    pub diepempmsk: DIEPEMPMSK,
    _reserved10: [u8; 0xc8],
    ///0x100..0x11c - Device IN endpoint 0
    pub diep0: DIEP0,
    _reserved11: [u8; 0x04],
    ///0x120..0x1c0 - Device IN endpoint X
    pub diep: [DIEP; 5],
    _reserved12: [u8; 0x0140],
    ///0x300..0x314 - Device OUT endpoint 0
    pub doep0: DOEP0,
    _reserved13: [u8; 0x0c],
    ///0x320..0x3c0 - Device IN endpoint X
    pub doep: [DOEP; 5],
}
impl RegisterBlock {
    ///0x120..0x140 - Device IN endpoint X
    #[inline(always)]
    pub fn diep1(&self) -> &DIEP {
        &self.diep[0]
    }
    ///0x140..0x160 - Device IN endpoint X
    #[inline(always)]
    pub fn diep2(&self) -> &DIEP {
        &self.diep[1]
    }
    ///0x160..0x180 - Device IN endpoint X
    #[inline(always)]
    pub fn diep3(&self) -> &DIEP {
        &self.diep[2]
    }
    ///0x180..0x1a0 - Device IN endpoint X
    #[inline(always)]
    pub fn diep4(&self) -> &DIEP {
        &self.diep[3]
    }
    ///0x1a0..0x1c0 - Device IN endpoint X
    #[inline(always)]
    pub fn diep5(&self) -> &DIEP {
        &self.diep[4]
    }
    ///0x320..0x340 - Device IN endpoint X
    #[inline(always)]
    pub fn doep1(&self) -> &DOEP {
        &self.doep[0]
    }
    ///0x340..0x360 - Device IN endpoint X
    #[inline(always)]
    pub fn doep2(&self) -> &DOEP {
        &self.doep[1]
    }
    ///0x360..0x380 - Device IN endpoint X
    #[inline(always)]
    pub fn doep3(&self) -> &DOEP {
        &self.doep[2]
    }
    ///0x380..0x3a0 - Device IN endpoint X
    #[inline(always)]
    pub fn doep4(&self) -> &DOEP {
        &self.doep[3]
    }
    ///0x3a0..0x3c0 - Device IN endpoint X
    #[inline(always)]
    pub fn doep5(&self) -> &DOEP {
        &self.doep[4]
    }
}
///DCFG (rw) register accessor: an alias for `Reg<DCFG_SPEC>`
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
///OTG_FS device configuration register (OTG_FS_DCFG)
pub mod dcfg;
///DCTL (rw) register accessor: an alias for `Reg<DCTL_SPEC>`
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
///OTG_FS device control register (OTG_FS_DCTL)
pub mod dctl;
///DSTS (r) register accessor: an alias for `Reg<DSTS_SPEC>`
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
///OTG_FS device status register (OTG_FS_DSTS)
pub mod dsts;
///DIEPMSK (rw) register accessor: an alias for `Reg<DIEPMSK_SPEC>`
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
///OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)
pub mod diepmsk;
///DOEPMSK (rw) register accessor: an alias for `Reg<DOEPMSK_SPEC>`
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
///OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)
pub mod doepmsk;
///DAINT (r) register accessor: an alias for `Reg<DAINT_SPEC>`
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
///OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)
pub mod daint;
///DAINTMSK (rw) register accessor: an alias for `Reg<DAINTMSK_SPEC>`
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
///OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)
pub mod daintmsk;
///DVBUSDIS (rw) register accessor: an alias for `Reg<DVBUSDIS_SPEC>`
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDIS_SPEC>;
///OTG_FS device VBUS discharge time register
pub mod dvbusdis;
///DVBUSPULSE (rw) register accessor: an alias for `Reg<DVBUSPULSE_SPEC>`
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>;
///OTG_FS device VBUS pulsing time register
pub mod dvbuspulse;
///DIEPEMPMSK (rw) register accessor: an alias for `Reg<DIEPEMPMSK_SPEC>`
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
///OTG_FS device IN endpoint FIFO empty interrupt mask register
pub mod diepempmsk;
///Device IN endpoint 0
pub use self::diep0::DIEP0;
///Cluster
///Device IN endpoint 0
pub mod diep0;
///Device IN endpoint X
pub use self::diep::DIEP;
///Cluster
///Device IN endpoint X
pub mod diep;
///Device OUT endpoint 0
pub use self::doep0::DOEP0;
///Cluster
///Device OUT endpoint 0
pub mod doep0;
///Device IN endpoint X
pub use self::doep::DOEP;
///Cluster
///Device IN endpoint X
pub mod doep;
