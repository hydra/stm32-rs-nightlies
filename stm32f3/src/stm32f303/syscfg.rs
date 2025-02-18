///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - configuration register 1
    pub cfgr1: CFGR1,
    ///0x04 - CCM SRAM protection register
    pub rcr: RCR,
    ///0x08 - external interrupt configuration register 1
    pub exticr1: EXTICR1,
    ///0x0c - external interrupt configuration register 2
    pub exticr2: EXTICR2,
    ///0x10 - external interrupt configuration register 3
    pub exticr3: EXTICR3,
    ///0x14 - external interrupt configuration register 4
    pub exticr4: EXTICR4,
    ///0x18 - configuration register 2
    pub cfgr2: CFGR2,
    _reserved7: [u8; 0x2c],
    ///0x48 - configuration register 4
    pub cfgr4: CFGR4,
    _reserved8: [u8; 0x04],
    ///0x50 - configuration register 3
    pub cfgr3: CFGR3,
}
///CFGR1 (rw) register accessor: an alias for `Reg<CFGR1_SPEC>`
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
///configuration register 1
pub mod cfgr1;
///EXTICR1 (rw) register accessor: an alias for `Reg<EXTICR1_SPEC>`
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1_SPEC>;
///external interrupt configuration register 1
pub mod exticr1;
///EXTICR2 (rw) register accessor: an alias for `Reg<EXTICR2_SPEC>`
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2_SPEC>;
///external interrupt configuration register 2
pub mod exticr2;
///EXTICR3 (rw) register accessor: an alias for `Reg<EXTICR3_SPEC>`
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3_SPEC>;
///external interrupt configuration register 3
pub mod exticr3;
///EXTICR4 (rw) register accessor: an alias for `Reg<EXTICR4_SPEC>`
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4_SPEC>;
///external interrupt configuration register 4
pub mod exticr4;
///CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
///configuration register 2
pub mod cfgr2;
///RCR (rw) register accessor: an alias for `Reg<RCR_SPEC>`
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
///CCM SRAM protection register
pub mod rcr;
///CFGR3 (rw) register accessor: an alias for `Reg<CFGR3_SPEC>`
pub type CFGR3 = crate::Reg<cfgr3::CFGR3_SPEC>;
///configuration register 3
pub mod cfgr3;
///CFGR4 (rw) register accessor: an alias for `Reg<CFGR4_SPEC>`
pub type CFGR4 = crate::Reg<cfgr4::CFGR4_SPEC>;
///configuration register 4
pub mod cfgr4;
