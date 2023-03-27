///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - OTG_HS device configuration register
    pub dcfg: DCFG,
    ///0x04 - OTG_HS device control register
    pub dctl: DCTL,
    ///0x08 - OTG_HS device status register
    pub dsts: DSTS,
    _reserved3: [u8; 0x04],
    ///0x10 - OTG_HS device IN endpoint common interrupt mask register
    pub diepmsk: DIEPMSK,
    ///0x14 - OTG_HS device OUT endpoint common interrupt mask register
    pub doepmsk: DOEPMSK,
    ///0x18 - OTG_HS device all endpoints interrupt register
    pub daint: DAINT,
    ///0x1c - OTG_HS all endpoints interrupt mask register
    pub daintmsk: DAINTMSK,
    _reserved7: [u8; 0x08],
    ///0x28 - OTG_HS device VBUS discharge time register
    pub dvbusdis: DVBUSDIS,
    ///0x2c - OTG_HS device VBUS pulsing time register
    pub dvbuspulse: DVBUSPULSE,
    ///0x30 - OTG_HS Device threshold control register
    pub dthrctl: DTHRCTL,
    ///0x34 - OTG_HS device IN endpoint FIFO empty interrupt mask register
    pub diepempmsk: DIEPEMPMSK,
    ///0x38 - OTG_HS device each endpoint interrupt register
    pub deachint: DEACHINT,
    ///0x3c - OTG_HS device each endpoint interrupt register mask
    pub deachintmsk: DEACHINTMSK,
    _reserved13: [u8; 0x04],
    ///0x44 -
    pub diepeachmsk1: DIEPEACHMSK1,
    _reserved14: [u8; 0x3c],
    ///0x84 -
    pub doepeachmsk1: DOEPEACHMSK1,
    _reserved15: [u8; 0x78],
    ///0x100..0x11c - Device IN endpoint 0
    pub diep0: DIEP0,
    _reserved16: [u8; 0x04],
    ///0x120..0x220 - Device IN endpoint X
    pub diep: [DIEP; 8],
    _reserved17: [u8; 0xe0],
    ///0x300..0x318 - Device OUT endpoint 0
    pub doep0: DOEP0,
    _reserved18: [u8; 0x08],
    ///0x320..0x420 - Device IN endpoint X
    pub doep: [DOEP; 8],
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
    ///0x1c0..0x1e0 - Device IN endpoint X
    #[inline(always)]
    pub fn diep6(&self) -> &DIEP {
        &self.diep[5]
    }
    ///0x1e0..0x200 - Device IN endpoint X
    #[inline(always)]
    pub fn diep7(&self) -> &DIEP {
        &self.diep[6]
    }
    ///0x200..0x220 - Device IN endpoint X
    #[inline(always)]
    pub fn diep8(&self) -> &DIEP {
        &self.diep[7]
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
    ///0x3c0..0x3e0 - Device IN endpoint X
    #[inline(always)]
    pub fn doep6(&self) -> &DOEP {
        &self.doep[5]
    }
    ///0x3e0..0x400 - Device IN endpoint X
    #[inline(always)]
    pub fn doep7(&self) -> &DOEP {
        &self.doep[6]
    }
    ///0x400..0x420 - Device IN endpoint X
    #[inline(always)]
    pub fn doep8(&self) -> &DOEP {
        &self.doep[7]
    }
}
///DCFG (rw) register accessor: an alias for `Reg<DCFG_SPEC>`
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
///OTG_HS device configuration register
pub mod dcfg;
///DCTL (rw) register accessor: an alias for `Reg<DCTL_SPEC>`
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
///OTG_HS device control register
pub mod dctl;
///DSTS (r) register accessor: an alias for `Reg<DSTS_SPEC>`
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
///OTG_HS device status register
pub mod dsts;
///DIEPMSK (rw) register accessor: an alias for `Reg<DIEPMSK_SPEC>`
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
///OTG_HS device IN endpoint common interrupt mask register
pub mod diepmsk;
///DOEPMSK (rw) register accessor: an alias for `Reg<DOEPMSK_SPEC>`
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
///OTG_HS device OUT endpoint common interrupt mask register
pub mod doepmsk;
///DAINT (r) register accessor: an alias for `Reg<DAINT_SPEC>`
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
///OTG_HS device all endpoints interrupt register
pub mod daint;
///DAINTMSK (rw) register accessor: an alias for `Reg<DAINTMSK_SPEC>`
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
///OTG_HS all endpoints interrupt mask register
pub mod daintmsk;
///DVBUSDIS (rw) register accessor: an alias for `Reg<DVBUSDIS_SPEC>`
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDIS_SPEC>;
///OTG_HS device VBUS discharge time register
pub mod dvbusdis;
///DVBUSPULSE (rw) register accessor: an alias for `Reg<DVBUSPULSE_SPEC>`
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>;
///OTG_HS device VBUS pulsing time register
pub mod dvbuspulse;
///DTHRCTL (rw) register accessor: an alias for `Reg<DTHRCTL_SPEC>`
pub type DTHRCTL = crate::Reg<dthrctl::DTHRCTL_SPEC>;
///OTG_HS Device threshold control register
pub mod dthrctl;
///DIEPEMPMSK (rw) register accessor: an alias for `Reg<DIEPEMPMSK_SPEC>`
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
///OTG_HS device IN endpoint FIFO empty interrupt mask register
pub mod diepempmsk;
///DEACHINT (rw) register accessor: an alias for `Reg<DEACHINT_SPEC>`
pub type DEACHINT = crate::Reg<deachint::DEACHINT_SPEC>;
///OTG_HS device each endpoint interrupt register
pub mod deachint;
///DEACHINTMSK (rw) register accessor: an alias for `Reg<DEACHINTMSK_SPEC>`
pub type DEACHINTMSK = crate::Reg<deachintmsk::DEACHINTMSK_SPEC>;
///OTG_HS device each endpoint interrupt register mask
pub mod deachintmsk;
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
///DIEPEACHMSK1 (rw) register accessor: an alias for `Reg<DIEPEACHMSK1_SPEC>`
pub type DIEPEACHMSK1 = crate::Reg<diepeachmsk1::DIEPEACHMSK1_SPEC>;
///
pub mod diepeachmsk1;
///DOEPEACHMSK1 (rw) register accessor: an alias for `Reg<DOEPEACHMSK1_SPEC>`
pub type DOEPEACHMSK1 = crate::Reg<doepeachmsk1::DOEPEACHMSK1_SPEC>;
///
pub mod doepeachmsk1;
