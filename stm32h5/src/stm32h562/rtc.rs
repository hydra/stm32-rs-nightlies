///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - RTC time register
    pub tr: TR,
    ///0x04 - RTC date register
    pub dr: DR,
    ///0x08 - RTC sub second register
    pub ssr: SSR,
    ///0x0c - RTC initialization control and status register
    pub icsr: ICSR,
    ///0x10 - RTC prescaler register
    pub prer: PRER,
    ///0x14 - RTC wakeup timer register
    pub wutr: WUTR,
    ///0x18 - RTC control register
    pub cr: CR,
    ///0x1c - RTC privilege mode control register
    pub privcfgr: PRIVCFGR,
    ///0x20 - RTC secure configuration register
    pub seccfgr: SECCFGR,
    ///0x24 - RTC write protection register
    pub wpr: WPR,
    ///0x28 - RTC calibration register
    pub calr: CALR,
    ///0x2c - RTC shift control register
    pub shiftr: SHIFTR,
    ///0x30 - RTC timestamp time register
    pub tstr: TSTR,
    ///0x34 - RTC timestamp date register
    pub tsdr: TSDR,
    ///0x38 - RTC timestamp sub second register
    pub tsssr: TSSSR,
    _reserved15: [u8; 0x04],
    ///0x40 - Alarm register
    pub alrmar: ALRMR,
    ///0x44 - Alarm sub-second register
    pub alrmassr: ALRMSSR,
    ///0x48 - Alarm register
    pub alrmbr: ALRMR,
    ///0x4c - Alarm sub-second register
    pub alrmbssr: ALRMSSR,
    ///0x50 - RTC status register
    pub sr: SR,
    ///0x54 - RTC non-secure masked interrupt status register
    pub misr: MISR,
    ///0x58 - RTC secure masked interrupt status register
    pub smisr: SMISR,
    ///0x5c - RTC status clear register
    pub scr: SCR,
    ///0x60 - RTC option register
    pub or: OR,
    _reserved24: [u8; 0x0c],
    ///0x70 - RTC alarm A binary mode register
    pub alrabinr: ALRABINR,
    ///0x74 - RTC alarm B binary mode register
    pub alrbbinr: ALRBBINR,
}
///TR (rw) register accessor: an alias for `Reg<TR_SPEC>`
pub type TR = crate::Reg<tr::TR_SPEC>;
///RTC time register
pub mod tr;
///DR (rw) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///RTC date register
pub mod dr;
///SSR (r) register accessor: an alias for `Reg<SSR_SPEC>`
pub type SSR = crate::Reg<ssr::SSR_SPEC>;
///RTC sub second register
pub mod ssr;
///ICSR (rw) register accessor: an alias for `Reg<ICSR_SPEC>`
pub type ICSR = crate::Reg<icsr::ICSR_SPEC>;
///RTC initialization control and status register
pub mod icsr;
///PRER (rw) register accessor: an alias for `Reg<PRER_SPEC>`
pub type PRER = crate::Reg<prer::PRER_SPEC>;
///RTC prescaler register
pub mod prer;
///WUTR (rw) register accessor: an alias for `Reg<WUTR_SPEC>`
pub type WUTR = crate::Reg<wutr::WUTR_SPEC>;
///RTC wakeup timer register
pub mod wutr;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///RTC control register
pub mod cr;
///PRIVCFGR (rw) register accessor: an alias for `Reg<PRIVCFGR_SPEC>`
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGR_SPEC>;
///RTC privilege mode control register
pub mod privcfgr;
///SECCFGR (rw) register accessor: an alias for `Reg<SECCFGR_SPEC>`
pub type SECCFGR = crate::Reg<seccfgr::SECCFGR_SPEC>;
///RTC secure configuration register
pub mod seccfgr;
///WPR (w) register accessor: an alias for `Reg<WPR_SPEC>`
pub type WPR = crate::Reg<wpr::WPR_SPEC>;
///RTC write protection register
pub mod wpr;
///CALR (rw) register accessor: an alias for `Reg<CALR_SPEC>`
pub type CALR = crate::Reg<calr::CALR_SPEC>;
///RTC calibration register
pub mod calr;
///SHIFTR (w) register accessor: an alias for `Reg<SHIFTR_SPEC>`
pub type SHIFTR = crate::Reg<shiftr::SHIFTR_SPEC>;
///RTC shift control register
pub mod shiftr;
pub use dr as tsdr;
pub use ssr as tsssr;
pub use tr as tstr;
pub use DR as TSDR;
pub use SSR as TSSSR;
pub use TR as TSTR;
///ALRMR (rw) register accessor: an alias for `Reg<ALRMR_SPEC>`
pub type ALRMR = crate::Reg<alrmr::ALRMR_SPEC>;
///Alarm register
pub mod alrmr;
///ALRMSSR (rw) register accessor: an alias for `Reg<ALRMSSR_SPEC>`
pub type ALRMSSR = crate::Reg<alrmssr::ALRMSSR_SPEC>;
///Alarm sub-second register
pub mod alrmssr;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///RTC status register
pub mod sr;
///MISR (r) register accessor: an alias for `Reg<MISR_SPEC>`
pub type MISR = crate::Reg<misr::MISR_SPEC>;
///RTC non-secure masked interrupt status register
pub mod misr;
///SMISR (r) register accessor: an alias for `Reg<SMISR_SPEC>`
pub type SMISR = crate::Reg<smisr::SMISR_SPEC>;
///RTC secure masked interrupt status register
pub mod smisr;
///SCR (w) register accessor: an alias for `Reg<SCR_SPEC>`
pub type SCR = crate::Reg<scr::SCR_SPEC>;
///RTC status clear register
pub mod scr;
///OR (rw) register accessor: an alias for `Reg<OR_SPEC>`
pub type OR = crate::Reg<or::OR_SPEC>;
///RTC option register
pub mod or;
///ALRABINR (rw) register accessor: an alias for `Reg<ALRABINR_SPEC>`
pub type ALRABINR = crate::Reg<alrabinr::ALRABINR_SPEC>;
///RTC alarm A binary mode register
pub mod alrabinr;
///ALRBBINR (rw) register accessor: an alias for `Reg<ALRBBINR_SPEC>`
pub type ALRBBINR = crate::Reg<alrbbinr::ALRBBINR_SPEC>;
///RTC alarm B binary mode register
pub mod alrbbinr;
