///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - OTG_FS host configuration register (OTG_FS_HCFG)
    pub hcfg: HCFG,
    ///0x04 - OTG_FS Host frame interval register
    pub hfir: HFIR,
    ///0x08 - OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)
    pub hfnum: HFNUM,
    _reserved3: [u8; 0x04],
    ///0x10 - OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)
    pub hptxsts: HPTXSTS,
    ///0x14 - OTG_FS Host all channels interrupt register
    pub haint: HAINT,
    ///0x18 - OTG_FS host all channels interrupt mask register
    pub haintmsk: HAINTMSK,
    _reserved6: [u8; 0x24],
    ///0x40 - OTG_FS host port control and status register (OTG_FS_HPRT)
    pub hprt: HPRT,
    _reserved7: [u8; 0xbc],
    ///0x100 - OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
    pub hcchar0: HCCHAR0,
    _reserved8: [u8; 0x04],
    ///0x108 - OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
    pub hcint0: HCINT0,
    ///0x10c - OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
    pub hcintmsk0: HCINTMSK0,
    ///0x110 - OTG_FS host channel-0 transfer size register
    pub hctsiz0: HCTSIZ0,
    _reserved11: [u8; 0x0c],
    ///0x120 - OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)
    pub hcchar1: HCCHAR1,
    _reserved12: [u8; 0x04],
    ///0x128 - OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)
    pub hcint1: HCINT1,
    ///0x12c - OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)
    pub hcintmsk1: HCINTMSK1,
    ///0x130 - OTG_FS host channel-1 transfer size register
    pub hctsiz1: HCTSIZ1,
    _reserved15: [u8; 0x0c],
    ///0x140 - OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)
    pub hcchar2: HCCHAR2,
    _reserved16: [u8; 0x04],
    ///0x148 - OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)
    pub hcint2: HCINT2,
    ///0x14c - OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)
    pub hcintmsk2: HCINTMSK2,
    ///0x150 - OTG_FS host channel-2 transfer size register
    pub hctsiz2: HCTSIZ2,
    _reserved19: [u8; 0x0c],
    ///0x160 - OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)
    pub hcchar3: HCCHAR3,
    _reserved20: [u8; 0x04],
    ///0x168 - OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)
    pub hcint3: HCINT3,
    ///0x16c - OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)
    pub hcintmsk3: HCINTMSK3,
    ///0x170 - OTG_FS host channel-3 transfer size register
    pub hctsiz3: HCTSIZ3,
    _reserved23: [u8; 0x0c],
    ///0x180 - OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)
    pub hcchar4: HCCHAR4,
    _reserved24: [u8; 0x04],
    ///0x188 - OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)
    pub hcint4: HCINT4,
    ///0x18c - OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)
    pub hcintmsk4: HCINTMSK4,
    ///0x190 - OTG_FS host channel-x transfer size register
    pub hctsiz4: HCTSIZ4,
    _reserved27: [u8; 0x0c],
    ///0x1a0 - OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)
    pub hcchar5: HCCHAR5,
    _reserved28: [u8; 0x04],
    ///0x1a8 - OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)
    pub hcint5: HCINT5,
    ///0x1ac - OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)
    pub hcintmsk5: HCINTMSK5,
    ///0x1b0 - OTG_FS host channel-5 transfer size register
    pub hctsiz5: HCTSIZ5,
    _reserved31: [u8; 0x0c],
    ///0x1c0 - OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)
    pub hcchar6: HCCHAR6,
    _reserved32: [u8; 0x04],
    ///0x1c8 - OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)
    pub hcint6: HCINT6,
    ///0x1cc - OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)
    pub hcintmsk6: HCINTMSK6,
    ///0x1d0 - OTG_FS host channel-6 transfer size register
    pub hctsiz6: HCTSIZ6,
    _reserved35: [u8; 0x0c],
    ///0x1e0 - OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)
    pub hcchar7: HCCHAR7,
    _reserved36: [u8; 0x04],
    ///0x1e8 - OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)
    pub hcint7: HCINT7,
    ///0x1ec - OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)
    pub hcintmsk7: HCINTMSK7,
    ///0x1f0 - OTG_FS host channel-7 transfer size register
    pub hctsiz7: HCTSIZ7,
}
///HCFG (rw) register accessor: an alias for `Reg<HCFG_SPEC>`
pub type HCFG = crate::Reg<hcfg::HCFG_SPEC>;
///OTG_FS host configuration register (OTG_FS_HCFG)
pub mod hcfg;
///HFIR (rw) register accessor: an alias for `Reg<HFIR_SPEC>`
pub type HFIR = crate::Reg<hfir::HFIR_SPEC>;
///OTG_FS Host frame interval register
pub mod hfir;
///HFNUM (r) register accessor: an alias for `Reg<HFNUM_SPEC>`
pub type HFNUM = crate::Reg<hfnum::HFNUM_SPEC>;
///OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)
pub mod hfnum;
///HPTXSTS (rw) register accessor: an alias for `Reg<HPTXSTS_SPEC>`
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTS_SPEC>;
///OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)
pub mod hptxsts;
///HAINT (r) register accessor: an alias for `Reg<HAINT_SPEC>`
pub type HAINT = crate::Reg<haint::HAINT_SPEC>;
///OTG_FS Host all channels interrupt register
pub mod haint;
///HAINTMSK (rw) register accessor: an alias for `Reg<HAINTMSK_SPEC>`
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSK_SPEC>;
///OTG_FS host all channels interrupt mask register
pub mod haintmsk;
///HPRT (rw) register accessor: an alias for `Reg<HPRT_SPEC>`
pub type HPRT = crate::Reg<hprt::HPRT_SPEC>;
///OTG_FS host port control and status register (OTG_FS_HPRT)
pub mod hprt;
///HCCHAR0 (rw) register accessor: an alias for `Reg<HCCHAR0_SPEC>`
pub type HCCHAR0 = crate::Reg<hcchar0::HCCHAR0_SPEC>;
///OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
pub mod hcchar0;
///HCCHAR1 (rw) register accessor: an alias for `Reg<HCCHAR1_SPEC>`
pub type HCCHAR1 = crate::Reg<hcchar1::HCCHAR1_SPEC>;
///OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)
pub mod hcchar1;
///HCCHAR2 (rw) register accessor: an alias for `Reg<HCCHAR2_SPEC>`
pub type HCCHAR2 = crate::Reg<hcchar2::HCCHAR2_SPEC>;
///OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)
pub mod hcchar2;
///HCCHAR3 (rw) register accessor: an alias for `Reg<HCCHAR3_SPEC>`
pub type HCCHAR3 = crate::Reg<hcchar3::HCCHAR3_SPEC>;
///OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)
pub mod hcchar3;
///HCCHAR4 (rw) register accessor: an alias for `Reg<HCCHAR4_SPEC>`
pub type HCCHAR4 = crate::Reg<hcchar4::HCCHAR4_SPEC>;
///OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)
pub mod hcchar4;
///HCCHAR5 (rw) register accessor: an alias for `Reg<HCCHAR5_SPEC>`
pub type HCCHAR5 = crate::Reg<hcchar5::HCCHAR5_SPEC>;
///OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)
pub mod hcchar5;
///HCCHAR6 (rw) register accessor: an alias for `Reg<HCCHAR6_SPEC>`
pub type HCCHAR6 = crate::Reg<hcchar6::HCCHAR6_SPEC>;
///OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)
pub mod hcchar6;
///HCCHAR7 (rw) register accessor: an alias for `Reg<HCCHAR7_SPEC>`
pub type HCCHAR7 = crate::Reg<hcchar7::HCCHAR7_SPEC>;
///OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)
pub mod hcchar7;
///HCINT0 (rw) register accessor: an alias for `Reg<HCINT0_SPEC>`
pub type HCINT0 = crate::Reg<hcint0::HCINT0_SPEC>;
///OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)
pub mod hcint0;
///HCINT1 (rw) register accessor: an alias for `Reg<HCINT1_SPEC>`
pub type HCINT1 = crate::Reg<hcint1::HCINT1_SPEC>;
///OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)
pub mod hcint1;
///HCINT2 (rw) register accessor: an alias for `Reg<HCINT2_SPEC>`
pub type HCINT2 = crate::Reg<hcint2::HCINT2_SPEC>;
///OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)
pub mod hcint2;
///HCINT3 (rw) register accessor: an alias for `Reg<HCINT3_SPEC>`
pub type HCINT3 = crate::Reg<hcint3::HCINT3_SPEC>;
///OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)
pub mod hcint3;
///HCINT4 (rw) register accessor: an alias for `Reg<HCINT4_SPEC>`
pub type HCINT4 = crate::Reg<hcint4::HCINT4_SPEC>;
///OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)
pub mod hcint4;
///HCINT5 (rw) register accessor: an alias for `Reg<HCINT5_SPEC>`
pub type HCINT5 = crate::Reg<hcint5::HCINT5_SPEC>;
///OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)
pub mod hcint5;
///HCINT6 (rw) register accessor: an alias for `Reg<HCINT6_SPEC>`
pub type HCINT6 = crate::Reg<hcint6::HCINT6_SPEC>;
///OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)
pub mod hcint6;
///HCINT7 (rw) register accessor: an alias for `Reg<HCINT7_SPEC>`
pub type HCINT7 = crate::Reg<hcint7::HCINT7_SPEC>;
///OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)
pub mod hcint7;
///HCINTMSK0 (rw) register accessor: an alias for `Reg<HCINTMSK0_SPEC>`
pub type HCINTMSK0 = crate::Reg<hcintmsk0::HCINTMSK0_SPEC>;
///OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)
pub mod hcintmsk0;
///HCINTMSK1 (rw) register accessor: an alias for `Reg<HCINTMSK1_SPEC>`
pub type HCINTMSK1 = crate::Reg<hcintmsk1::HCINTMSK1_SPEC>;
///OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)
pub mod hcintmsk1;
///HCINTMSK2 (rw) register accessor: an alias for `Reg<HCINTMSK2_SPEC>`
pub type HCINTMSK2 = crate::Reg<hcintmsk2::HCINTMSK2_SPEC>;
///OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)
pub mod hcintmsk2;
///HCINTMSK3 (rw) register accessor: an alias for `Reg<HCINTMSK3_SPEC>`
pub type HCINTMSK3 = crate::Reg<hcintmsk3::HCINTMSK3_SPEC>;
///OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)
pub mod hcintmsk3;
///HCINTMSK4 (rw) register accessor: an alias for `Reg<HCINTMSK4_SPEC>`
pub type HCINTMSK4 = crate::Reg<hcintmsk4::HCINTMSK4_SPEC>;
///OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)
pub mod hcintmsk4;
///HCINTMSK5 (rw) register accessor: an alias for `Reg<HCINTMSK5_SPEC>`
pub type HCINTMSK5 = crate::Reg<hcintmsk5::HCINTMSK5_SPEC>;
///OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)
pub mod hcintmsk5;
///HCINTMSK6 (rw) register accessor: an alias for `Reg<HCINTMSK6_SPEC>`
pub type HCINTMSK6 = crate::Reg<hcintmsk6::HCINTMSK6_SPEC>;
///OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)
pub mod hcintmsk6;
///HCINTMSK7 (rw) register accessor: an alias for `Reg<HCINTMSK7_SPEC>`
pub type HCINTMSK7 = crate::Reg<hcintmsk7::HCINTMSK7_SPEC>;
///OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)
pub mod hcintmsk7;
///HCTSIZ0 (rw) register accessor: an alias for `Reg<HCTSIZ0_SPEC>`
pub type HCTSIZ0 = crate::Reg<hctsiz0::HCTSIZ0_SPEC>;
///OTG_FS host channel-0 transfer size register
pub mod hctsiz0;
///HCTSIZ1 (rw) register accessor: an alias for `Reg<HCTSIZ1_SPEC>`
pub type HCTSIZ1 = crate::Reg<hctsiz1::HCTSIZ1_SPEC>;
///OTG_FS host channel-1 transfer size register
pub mod hctsiz1;
///HCTSIZ2 (rw) register accessor: an alias for `Reg<HCTSIZ2_SPEC>`
pub type HCTSIZ2 = crate::Reg<hctsiz2::HCTSIZ2_SPEC>;
///OTG_FS host channel-2 transfer size register
pub mod hctsiz2;
///HCTSIZ3 (rw) register accessor: an alias for `Reg<HCTSIZ3_SPEC>`
pub type HCTSIZ3 = crate::Reg<hctsiz3::HCTSIZ3_SPEC>;
///OTG_FS host channel-3 transfer size register
pub mod hctsiz3;
///HCTSIZ4 (rw) register accessor: an alias for `Reg<HCTSIZ4_SPEC>`
pub type HCTSIZ4 = crate::Reg<hctsiz4::HCTSIZ4_SPEC>;
///OTG_FS host channel-x transfer size register
pub mod hctsiz4;
///HCTSIZ5 (rw) register accessor: an alias for `Reg<HCTSIZ5_SPEC>`
pub type HCTSIZ5 = crate::Reg<hctsiz5::HCTSIZ5_SPEC>;
///OTG_FS host channel-5 transfer size register
pub mod hctsiz5;
///HCTSIZ6 (rw) register accessor: an alias for `Reg<HCTSIZ6_SPEC>`
pub type HCTSIZ6 = crate::Reg<hctsiz6::HCTSIZ6_SPEC>;
///OTG_FS host channel-6 transfer size register
pub mod hctsiz6;
///HCTSIZ7 (rw) register accessor: an alias for `Reg<HCTSIZ7_SPEC>`
pub type HCTSIZ7 = crate::Reg<hctsiz7::HCTSIZ7_SPEC>;
///OTG_FS host channel-7 transfer size register
pub mod hctsiz7;
