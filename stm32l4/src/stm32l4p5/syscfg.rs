///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - memory remap register
    pub memrmp: MEMRMP,
    ///0x04 - configuration register 1
    pub cfgr1: CFGR1,
    ///0x08 - external interrupt configuration register 1
    pub exticr1: EXTICR1,
    ///0x0c - external interrupt configuration register 2
    pub exticr2: EXTICR2,
    ///0x10 - external interrupt configuration register 3
    pub exticr3: EXTICR3,
    ///0x14 - external interrupt configuration register 4
    pub exticr4: EXTICR4,
    ///0x18 - SCSR
    pub scsr: SCSR,
    ///0x1c - CFGR2
    pub cfgr2: CFGR2,
    ///0x20 - SWPR
    pub swpr: SWPR,
    ///0x24 - SKR
    pub skr: SKR,
    ///0x28 - write protection register 2
    pub swpr2: SWPR2,
}
///MEMRMP (rw) register accessor: an alias for `Reg<MEMRMP_SPEC>`
pub type MEMRMP = crate::Reg<memrmp::MEMRMP_SPEC>;
///memory remap register
pub mod memrmp;
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
///SCSR (rw) register accessor: an alias for `Reg<SCSR_SPEC>`
pub type SCSR = crate::Reg<scsr::SCSR_SPEC>;
///SCSR
pub mod scsr;
///CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
///CFGR2
pub mod cfgr2;
///SWPR (w) register accessor: an alias for `Reg<SWPR_SPEC>`
pub type SWPR = crate::Reg<swpr::SWPR_SPEC>;
///SWPR
pub mod swpr;
///SKR (w) register accessor: an alias for `Reg<SKR_SPEC>`
pub type SKR = crate::Reg<skr::SKR_SPEC>;
///SKR
pub mod skr;
///SWPR2 (rw) register accessor: an alias for `Reg<SWPR2_SPEC>`
pub type SWPR2 = crate::Reg<swpr2::SWPR2_SPEC>;
///write protection register 2
pub mod swpr2;
