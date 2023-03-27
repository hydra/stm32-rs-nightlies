///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - FMAC X1 buffer configuration register
    pub x1bufcfg: X1BUFCFG,
    ///0x04 - FMAC X2 buffer configuration register
    pub x2bufcfg: X2BUFCFG,
    ///0x08 - FMAC Y buffer configuration register
    pub ybufcfg: YBUFCFG,
    ///0x0c - FMAC parameter register
    pub param: PARAM,
    ///0x10 - FMAC control register
    pub cr: CR,
    ///0x14 - FMAC status register
    pub sr: SR,
    ///0x18 - FMAC write data register
    pub wdata: WDATA,
    ///0x1c - FMAC read data register
    pub rdata: RDATA,
}
///X1BUFCFG (rw) register accessor: an alias for `Reg<X1BUFCFG_SPEC>`
pub type X1BUFCFG = crate::Reg<x1bufcfg::X1BUFCFG_SPEC>;
///FMAC X1 buffer configuration register
pub mod x1bufcfg;
///X2BUFCFG (rw) register accessor: an alias for `Reg<X2BUFCFG_SPEC>`
pub type X2BUFCFG = crate::Reg<x2bufcfg::X2BUFCFG_SPEC>;
///FMAC X2 buffer configuration register
pub mod x2bufcfg;
///YBUFCFG (rw) register accessor: an alias for `Reg<YBUFCFG_SPEC>`
pub type YBUFCFG = crate::Reg<ybufcfg::YBUFCFG_SPEC>;
///FMAC Y buffer configuration register
pub mod ybufcfg;
///PARAM (rw) register accessor: an alias for `Reg<PARAM_SPEC>`
pub type PARAM = crate::Reg<param::PARAM_SPEC>;
///FMAC parameter register
pub mod param;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///FMAC control register
pub mod cr;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///FMAC status register
pub mod sr;
///WDATA (w) register accessor: an alias for `Reg<WDATA_SPEC>`
pub type WDATA = crate::Reg<wdata::WDATA_SPEC>;
///FMAC write data register
pub mod wdata;
///RDATA (r) register accessor: an alias for `Reg<RDATA_SPEC>`
pub type RDATA = crate::Reg<rdata::RDATA_SPEC>;
///FMAC read data register
pub mod rdata;
