///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Comparator status register
    pub sr: SR,
    ///0x04 - Comparator interrupt clear flag register
    pub icfr: ICFR,
    ///0x08 - Comparator option register
    pub or: OR,
    ///0x0c - Comparator configuration register 1
    pub cfgr1: CFGR1,
    ///0x10 - Comparator configuration register 2
    pub cfgr2: CFGR2,
}
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///Comparator status register
pub mod sr;
///ICFR (w) register accessor: an alias for `Reg<ICFR_SPEC>`
pub type ICFR = crate::Reg<icfr::ICFR_SPEC>;
///Comparator interrupt clear flag register
pub mod icfr;
///OR (rw) register accessor: an alias for `Reg<OR_SPEC>`
pub type OR = crate::Reg<or::OR_SPEC>;
///Comparator option register
pub mod or;
///CFGR1 (rw) register accessor: an alias for `Reg<CFGR1_SPEC>`
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
///Comparator configuration register 1
pub mod cfgr1;
///CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
///Comparator configuration register 2
pub mod cfgr2;
