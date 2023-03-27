///Register block
#[repr(C)]
pub struct CH {
    ///0x00 - DFSDM channel configuration 0 register 1
    pub cfgr1: CFGR1,
    ///0x04 - DFSDM channel configuration 0 register 2
    pub cfgr2: CFGR2,
    ///0x08 - DFSDM analog watchdog and short-circuit detector register
    pub awscdr: AWSCDR,
    ///0x0c - DFSDM channel watchdog filter data register
    pub wdatr: WDATR,
    ///0x10 - DFSDM channel data input register
    pub datinr: DATINR,
    _reserved_end: [u8; 0x0c],
}
///CFGR1 (rw) register accessor: an alias for `Reg<CFGR1_SPEC>`
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
///DFSDM channel configuration 0 register 1
pub mod cfgr1;
///CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
///DFSDM channel configuration 0 register 2
pub mod cfgr2;
///AWSCDR (rw) register accessor: an alias for `Reg<AWSCDR_SPEC>`
pub type AWSCDR = crate::Reg<awscdr::AWSCDR_SPEC>;
///DFSDM analog watchdog and short-circuit detector register
pub mod awscdr;
///WDATR (r) register accessor: an alias for `Reg<WDATR_SPEC>`
pub type WDATR = crate::Reg<wdatr::WDATR_SPEC>;
///DFSDM channel watchdog filter data register
pub mod wdatr;
///DATINR (rw) register accessor: an alias for `Reg<DATINR_SPEC>`
pub type DATINR = crate::Reg<datinr::DATINR_SPEC>;
///DFSDM channel data input register
pub mod datinr;
