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
    ///0x100..0x200 - Host channel
    pub hc: [HC; 8],
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
///Host channel
pub use self::hc::HC;
///Cluster
///Host channel
pub mod hc;
