///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - RTC time register
    pub tr: TR,
    ///0x04 - RTC date register
    pub dr: DR,
    ///0x08 - RTC subsecond register
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
    _reserved8: [u8; 0x04],
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
    ///0x38 - RTC timestamp subsecond register
    pub tsssr: TSSSR,
    _reserved14: [u8; 0x04],
    ///0x40 - RTC alarm A register
    pub alrmar: ALRMAR,
    ///0x44 - RTC alarm A subsecond register
    pub alrmassr: ALRMASSR,
    ///0x48 - RTC alarm B register
    pub alrmbr: ALRMBR,
    ///0x4c - RTC alarm B subsecond register
    pub alrmbssr: ALRMBSSR,
    ///0x50 - RTC status register
    pub sr: SR,
    ///0x54 - RTC masked interrupt status register
    pub misr: MISR,
    _reserved20: [u8; 0x04],
    ///0x5c - RTC status clear register
    pub scr: SCR,
    _reserved21: [u8; 0x10],
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
///RTC subsecond register
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
///TSTR (r) register accessor: an alias for `Reg<TSTR_SPEC>`
pub type TSTR = crate::Reg<tstr::TSTR_SPEC>;
///RTC timestamp time register
pub mod tstr;
///TSDR (r) register accessor: an alias for `Reg<TSDR_SPEC>`
pub type TSDR = crate::Reg<tsdr::TSDR_SPEC>;
///RTC timestamp date register
pub mod tsdr;
///TSSSR (r) register accessor: an alias for `Reg<TSSSR_SPEC>`
pub type TSSSR = crate::Reg<tsssr::TSSSR_SPEC>;
///RTC timestamp subsecond register
pub mod tsssr;
///ALRMAR (rw) register accessor: an alias for `Reg<ALRMAR_SPEC>`
pub type ALRMAR = crate::Reg<alrmar::ALRMAR_SPEC>;
///RTC alarm A register
pub mod alrmar;
///ALRMASSR (rw) register accessor: an alias for `Reg<ALRMASSR_SPEC>`
pub type ALRMASSR = crate::Reg<alrmassr::ALRMASSR_SPEC>;
///RTC alarm A subsecond register
pub mod alrmassr;
///ALRMBR (rw) register accessor: an alias for `Reg<ALRMBR_SPEC>`
pub type ALRMBR = crate::Reg<alrmbr::ALRMBR_SPEC>;
///RTC alarm B register
pub mod alrmbr;
///ALRMBSSR (rw) register accessor: an alias for `Reg<ALRMBSSR_SPEC>`
pub type ALRMBSSR = crate::Reg<alrmbssr::ALRMBSSR_SPEC>;
///RTC alarm B subsecond register
pub mod alrmbssr;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///RTC status register
pub mod sr;
///MISR (r) register accessor: an alias for `Reg<MISR_SPEC>`
pub type MISR = crate::Reg<misr::MISR_SPEC>;
///RTC masked interrupt status register
pub mod misr;
///SCR (w) register accessor: an alias for `Reg<SCR_SPEC>`
pub type SCR = crate::Reg<scr::SCR_SPEC>;
///RTC status clear register
pub mod scr;
///ALRABINR (rw) register accessor: an alias for `Reg<ALRABINR_SPEC>`
pub type ALRABINR = crate::Reg<alrabinr::ALRABINR_SPEC>;
///RTC alarm A binary mode register
pub mod alrabinr;
///ALRBBINR (rw) register accessor: an alias for `Reg<ALRBBINR_SPEC>`
pub type ALRBBINR = crate::Reg<alrbbinr::ALRBBINR_SPEC>;
///RTC alarm B binary mode register
pub mod alrbbinr;
