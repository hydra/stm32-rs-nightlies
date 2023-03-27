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
    ///0x100 - OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)
    pub diepctl0: DIEPCTL0,
    _reserved11: [u8; 0x04],
    ///0x108 - device endpoint-x interrupt register
    pub diepint0: DIEPINT0,
    _reserved12: [u8; 0x04],
    ///0x110 - device endpoint-0 transfer size register
    pub dieptsiz0: DIEPTSIZ0,
    _reserved13: [u8; 0x04],
    ///0x118 - OTG_FS device IN endpoint transmit FIFO status register
    pub dtxfsts0: DTXFSTS0,
    _reserved14: [u8; 0x04],
    ///0x120 - OTG device endpoint-1 control register
    pub diepctl1: DIEPCTL1,
    _reserved15: [u8; 0x04],
    ///0x128 - device endpoint-1 interrupt register
    pub diepint1: DIEPINT1,
    _reserved16: [u8; 0x04],
    ///0x130 - device endpoint-1 transfer size register
    pub dieptsiz1: DIEPTSIZ1,
    _reserved17: [u8; 0x04],
    ///0x138 - OTG_FS device IN endpoint transmit FIFO status register
    pub dtxfsts1: DTXFSTS1,
    _reserved18: [u8; 0x04],
    ///0x140 - OTG device endpoint-2 control register
    pub diepctl2: DIEPCTL2,
    _reserved19: [u8; 0x04],
    ///0x148 - device endpoint-2 interrupt register
    pub diepint2: DIEPINT2,
    _reserved20: [u8; 0x04],
    ///0x150 - device endpoint-2 transfer size register
    pub dieptsiz2: DIEPTSIZ2,
    _reserved21: [u8; 0x04],
    ///0x158 - OTG_FS device IN endpoint transmit FIFO status register
    pub dtxfsts2: DTXFSTS2,
    _reserved22: [u8; 0x04],
    ///0x160 - OTG device endpoint-3 control register
    pub diepctl3: DIEPCTL3,
    _reserved23: [u8; 0x04],
    ///0x168 - device endpoint-3 interrupt register
    pub diepint3: DIEPINT3,
    _reserved24: [u8; 0x04],
    ///0x170 - device endpoint-3 transfer size register
    pub dieptsiz3: DIEPTSIZ3,
    _reserved25: [u8; 0x04],
    ///0x178 - OTG_FS device IN endpoint transmit FIFO status register
    pub dtxfsts3: DTXFSTS3,
    _reserved26: [u8; 0x0184],
    ///0x300 - device endpoint-0 control register
    pub doepctl0: DOEPCTL0,
    _reserved27: [u8; 0x04],
    ///0x308 - device endpoint-0 interrupt register
    pub doepint0: DOEPINT0,
    _reserved28: [u8; 0x04],
    ///0x310 - device OUT endpoint-0 transfer size register
    pub doeptsiz0: DOEPTSIZ0,
    _reserved29: [u8; 0x0c],
    ///0x320 - device endpoint-1 control register
    pub doepctl1: DOEPCTL1,
    _reserved30: [u8; 0x04],
    ///0x328 - device endpoint-1 interrupt register
    pub doepint1: DOEPINT1,
    _reserved31: [u8; 0x04],
    ///0x330 - device OUT endpoint-1 transfer size register
    pub doeptsiz1: DOEPTSIZ1,
    _reserved32: [u8; 0x0c],
    ///0x340 - device endpoint-2 control register
    pub doepctl2: DOEPCTL2,
    _reserved33: [u8; 0x04],
    ///0x348 - device endpoint-2 interrupt register
    pub doepint2: DOEPINT2,
    _reserved34: [u8; 0x04],
    ///0x350 - device OUT endpoint-2 transfer size register
    pub doeptsiz2: DOEPTSIZ2,
    _reserved35: [u8; 0x0c],
    ///0x360 - device endpoint-3 control register
    pub doepctl3: DOEPCTL3,
    _reserved36: [u8; 0x04],
    ///0x368 - device endpoint-3 interrupt register
    pub doepint3: DOEPINT3,
    _reserved37: [u8; 0x04],
    ///0x370 - device OUT endpoint-3 transfer size register
    pub doeptsiz3: DOEPTSIZ3,
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
///DIEPCTL0 (rw) register accessor: an alias for `Reg<DIEPCTL0_SPEC>`
pub type DIEPCTL0 = crate::Reg<diepctl0::DIEPCTL0_SPEC>;
///OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)
pub mod diepctl0;
///DIEPCTL1 (rw) register accessor: an alias for `Reg<DIEPCTL1_SPEC>`
pub type DIEPCTL1 = crate::Reg<diepctl1::DIEPCTL1_SPEC>;
///OTG device endpoint-1 control register
pub mod diepctl1;
///DIEPCTL2 (rw) register accessor: an alias for `Reg<DIEPCTL2_SPEC>`
pub type DIEPCTL2 = crate::Reg<diepctl2::DIEPCTL2_SPEC>;
///OTG device endpoint-2 control register
pub mod diepctl2;
///DIEPCTL3 (rw) register accessor: an alias for `Reg<DIEPCTL3_SPEC>`
pub type DIEPCTL3 = crate::Reg<diepctl3::DIEPCTL3_SPEC>;
///OTG device endpoint-3 control register
pub mod diepctl3;
///DOEPCTL0 (rw) register accessor: an alias for `Reg<DOEPCTL0_SPEC>`
pub type DOEPCTL0 = crate::Reg<doepctl0::DOEPCTL0_SPEC>;
///device endpoint-0 control register
pub mod doepctl0;
///DOEPCTL1 (rw) register accessor: an alias for `Reg<DOEPCTL1_SPEC>`
pub type DOEPCTL1 = crate::Reg<doepctl1::DOEPCTL1_SPEC>;
///device endpoint-1 control register
pub mod doepctl1;
///DOEPCTL2 (rw) register accessor: an alias for `Reg<DOEPCTL2_SPEC>`
pub type DOEPCTL2 = crate::Reg<doepctl2::DOEPCTL2_SPEC>;
///device endpoint-2 control register
pub mod doepctl2;
///DOEPCTL3 (rw) register accessor: an alias for `Reg<DOEPCTL3_SPEC>`
pub type DOEPCTL3 = crate::Reg<doepctl3::DOEPCTL3_SPEC>;
///device endpoint-3 control register
pub mod doepctl3;
///DIEPINT0 (rw) register accessor: an alias for `Reg<DIEPINT0_SPEC>`
pub type DIEPINT0 = crate::Reg<diepint0::DIEPINT0_SPEC>;
///device endpoint-x interrupt register
pub mod diepint0;
///DIEPINT1 (rw) register accessor: an alias for `Reg<DIEPINT1_SPEC>`
pub type DIEPINT1 = crate::Reg<diepint1::DIEPINT1_SPEC>;
///device endpoint-1 interrupt register
pub mod diepint1;
///DIEPINT2 (rw) register accessor: an alias for `Reg<DIEPINT2_SPEC>`
pub type DIEPINT2 = crate::Reg<diepint2::DIEPINT2_SPEC>;
///device endpoint-2 interrupt register
pub mod diepint2;
///DIEPINT3 (rw) register accessor: an alias for `Reg<DIEPINT3_SPEC>`
pub type DIEPINT3 = crate::Reg<diepint3::DIEPINT3_SPEC>;
///device endpoint-3 interrupt register
pub mod diepint3;
///DOEPINT0 (rw) register accessor: an alias for `Reg<DOEPINT0_SPEC>`
pub type DOEPINT0 = crate::Reg<doepint0::DOEPINT0_SPEC>;
///device endpoint-0 interrupt register
pub mod doepint0;
///DOEPINT1 (rw) register accessor: an alias for `Reg<DOEPINT1_SPEC>`
pub type DOEPINT1 = crate::Reg<doepint1::DOEPINT1_SPEC>;
///device endpoint-1 interrupt register
pub mod doepint1;
///DOEPINT2 (rw) register accessor: an alias for `Reg<DOEPINT2_SPEC>`
pub type DOEPINT2 = crate::Reg<doepint2::DOEPINT2_SPEC>;
///device endpoint-2 interrupt register
pub mod doepint2;
///DOEPINT3 (rw) register accessor: an alias for `Reg<DOEPINT3_SPEC>`
pub type DOEPINT3 = crate::Reg<doepint3::DOEPINT3_SPEC>;
///device endpoint-3 interrupt register
pub mod doepint3;
///DIEPTSIZ0 (rw) register accessor: an alias for `Reg<DIEPTSIZ0_SPEC>`
pub type DIEPTSIZ0 = crate::Reg<dieptsiz0::DIEPTSIZ0_SPEC>;
///device endpoint-0 transfer size register
pub mod dieptsiz0;
///DOEPTSIZ0 (rw) register accessor: an alias for `Reg<DOEPTSIZ0_SPEC>`
pub type DOEPTSIZ0 = crate::Reg<doeptsiz0::DOEPTSIZ0_SPEC>;
///device OUT endpoint-0 transfer size register
pub mod doeptsiz0;
///DIEPTSIZ1 (rw) register accessor: an alias for `Reg<DIEPTSIZ1_SPEC>`
pub type DIEPTSIZ1 = crate::Reg<dieptsiz1::DIEPTSIZ1_SPEC>;
///device endpoint-1 transfer size register
pub mod dieptsiz1;
///DIEPTSIZ2 (rw) register accessor: an alias for `Reg<DIEPTSIZ2_SPEC>`
pub type DIEPTSIZ2 = crate::Reg<dieptsiz2::DIEPTSIZ2_SPEC>;
///device endpoint-2 transfer size register
pub mod dieptsiz2;
///DIEPTSIZ3 (rw) register accessor: an alias for `Reg<DIEPTSIZ3_SPEC>`
pub type DIEPTSIZ3 = crate::Reg<dieptsiz3::DIEPTSIZ3_SPEC>;
///device endpoint-3 transfer size register
pub mod dieptsiz3;
///DTXFSTS0 (r) register accessor: an alias for `Reg<DTXFSTS0_SPEC>`
pub type DTXFSTS0 = crate::Reg<dtxfsts0::DTXFSTS0_SPEC>;
///OTG_FS device IN endpoint transmit FIFO status register
pub mod dtxfsts0;
///DTXFSTS1 (r) register accessor: an alias for `Reg<DTXFSTS1_SPEC>`
pub type DTXFSTS1 = crate::Reg<dtxfsts1::DTXFSTS1_SPEC>;
///OTG_FS device IN endpoint transmit FIFO status register
pub mod dtxfsts1;
///DTXFSTS2 (r) register accessor: an alias for `Reg<DTXFSTS2_SPEC>`
pub type DTXFSTS2 = crate::Reg<dtxfsts2::DTXFSTS2_SPEC>;
///OTG_FS device IN endpoint transmit FIFO status register
pub mod dtxfsts2;
///DTXFSTS3 (r) register accessor: an alias for `Reg<DTXFSTS3_SPEC>`
pub type DTXFSTS3 = crate::Reg<dtxfsts3::DTXFSTS3_SPEC>;
///OTG_FS device IN endpoint transmit FIFO status register
pub mod dtxfsts3;
///DOEPTSIZ1 (rw) register accessor: an alias for `Reg<DOEPTSIZ1_SPEC>`
pub type DOEPTSIZ1 = crate::Reg<doeptsiz1::DOEPTSIZ1_SPEC>;
///device OUT endpoint-1 transfer size register
pub mod doeptsiz1;
///DOEPTSIZ2 (rw) register accessor: an alias for `Reg<DOEPTSIZ2_SPEC>`
pub type DOEPTSIZ2 = crate::Reg<doeptsiz2::DOEPTSIZ2_SPEC>;
///device OUT endpoint-2 transfer size register
pub mod doeptsiz2;
///DOEPTSIZ3 (rw) register accessor: an alias for `Reg<DOEPTSIZ3_SPEC>`
pub type DOEPTSIZ3 = crate::Reg<doeptsiz3::DOEPTSIZ3_SPEC>;
///device OUT endpoint-3 transfer size register
pub mod doeptsiz3;
