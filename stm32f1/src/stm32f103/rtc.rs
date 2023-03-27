///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - RTC Control Register High
    pub crh: CRH,
    ///0x04 - RTC Control Register Low
    pub crl: CRL,
    ///0x08 - RTC Prescaler Load Register High
    pub prlh: PRLH,
    ///0x0c - RTC Prescaler Load Register Low
    pub prll: PRLL,
    ///0x10 - RTC Prescaler Divider Register High
    pub divh: DIVH,
    ///0x14 - RTC Prescaler Divider Register Low
    pub divl: DIVL,
    ///0x18 - RTC Counter Register High
    pub cnth: CNTH,
    ///0x1c - RTC Counter Register Low
    pub cntl: CNTL,
    ///0x20 - RTC Alarm Register High
    pub alrh: ALRH,
    ///0x24 - RTC Alarm Register Low
    pub alrl: ALRL,
}
///CRH (rw) register accessor: an alias for `Reg<CRH_SPEC>`
pub type CRH = crate::Reg<crh::CRH_SPEC>;
///RTC Control Register High
pub mod crh;
///CRL (rw) register accessor: an alias for `Reg<CRL_SPEC>`
pub type CRL = crate::Reg<crl::CRL_SPEC>;
///RTC Control Register Low
pub mod crl;
///PRLH (w) register accessor: an alias for `Reg<PRLH_SPEC>`
pub type PRLH = crate::Reg<prlh::PRLH_SPEC>;
///RTC Prescaler Load Register High
pub mod prlh;
///PRLL (w) register accessor: an alias for `Reg<PRLL_SPEC>`
pub type PRLL = crate::Reg<prll::PRLL_SPEC>;
///RTC Prescaler Load Register Low
pub mod prll;
///DIVH (r) register accessor: an alias for `Reg<DIVH_SPEC>`
pub type DIVH = crate::Reg<divh::DIVH_SPEC>;
///RTC Prescaler Divider Register High
pub mod divh;
///DIVL (r) register accessor: an alias for `Reg<DIVL_SPEC>`
pub type DIVL = crate::Reg<divl::DIVL_SPEC>;
///RTC Prescaler Divider Register Low
pub mod divl;
///CNTH (rw) register accessor: an alias for `Reg<CNTH_SPEC>`
pub type CNTH = crate::Reg<cnth::CNTH_SPEC>;
///RTC Counter Register High
pub mod cnth;
///CNTL (rw) register accessor: an alias for `Reg<CNTL_SPEC>`
pub type CNTL = crate::Reg<cntl::CNTL_SPEC>;
///RTC Counter Register Low
pub mod cntl;
///ALRH (w) register accessor: an alias for `Reg<ALRH_SPEC>`
pub type ALRH = crate::Reg<alrh::ALRH_SPEC>;
///RTC Alarm Register High
pub mod alrh;
///ALRL (w) register accessor: an alias for `Reg<ALRL_SPEC>`
pub type ALRL = crate::Reg<alrl::ALRL_SPEC>;
///RTC Alarm Register Low
pub mod alrl;
