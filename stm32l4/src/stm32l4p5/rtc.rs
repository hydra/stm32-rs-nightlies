///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - time register
    pub tr: TR,
    ///0x04 - date register
    pub dr: DR,
    ///0x08 - sub second register
    pub ssr: SSR,
    ///0x0c - initialization and status register
    pub icsr: ICSR,
    ///0x10 - prescaler register
    pub prer: PRER,
    ///0x14 - wakeup timer register
    pub wutr: WUTR,
    ///0x18 - control register
    pub cr: CR,
    _reserved7: [u8; 0x08],
    ///0x24 - write protection register
    pub wpr: WPR,
    ///0x28 - calibration register
    pub calr: CALR,
    ///0x2c - shift control register
    pub shiftr: SHIFTR,
    ///0x30 - time stamp time register
    pub tstr: TSTR,
    ///0x34 - time stamp date register
    pub tsdr: TSDR,
    ///0x38 - timestamp sub second register
    pub tsssr: TSSSR,
    _reserved13: [u8; 0x04],
    ///0x40 - alarm A register
    pub alrmar: ALRMAR,
    ///0x44 - alarm A sub second register
    pub alrmassr: ALRMASSR,
    ///0x48 - alarm B register
    pub alrmbr: ALRMBR,
    ///0x4c - alarm B sub second register
    pub alrmbssr: ALRMBSSR,
    ///0x50 - RTC status register
    pub sr: SR,
    ///0x54 - RTC masked interrupt status register
    pub misr: MISR,
    _reserved19: [u8; 0x04],
    ///0x5c - RTC status clear register
    pub scr: SCR,
    _reserved20: [u8; 0x10],
    ///0x70 - RTC alarm A binary mode register
    pub alrabinr: ALRABINR,
    ///0x74 - RTC alarm B binary mode register
    pub alrbbinr: ALRBBINR,
}
///TR (rw) register accessor: an alias for `Reg<TR_SPEC>`
pub type TR = crate::Reg<tr::TR_SPEC>;
///time register
pub mod tr;
///DR (rw) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///date register
pub mod dr;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///control register
pub mod cr;
///ICSR (rw) register accessor: an alias for `Reg<ICSR_SPEC>`
pub type ICSR = crate::Reg<icsr::ICSR_SPEC>;
///initialization and status register
pub mod icsr;
///PRER (rw) register accessor: an alias for `Reg<PRER_SPEC>`
pub type PRER = crate::Reg<prer::PRER_SPEC>;
///prescaler register
pub mod prer;
///WUTR (rw) register accessor: an alias for `Reg<WUTR_SPEC>`
pub type WUTR = crate::Reg<wutr::WUTR_SPEC>;
///wakeup timer register
pub mod wutr;
///ALRMAR (rw) register accessor: an alias for `Reg<ALRMAR_SPEC>`
pub type ALRMAR = crate::Reg<alrmar::ALRMAR_SPEC>;
///alarm A register
pub mod alrmar;
///ALRMBR (rw) register accessor: an alias for `Reg<ALRMBR_SPEC>`
pub type ALRMBR = crate::Reg<alrmbr::ALRMBR_SPEC>;
///alarm B register
pub mod alrmbr;
///WPR (w) register accessor: an alias for `Reg<WPR_SPEC>`
pub type WPR = crate::Reg<wpr::WPR_SPEC>;
///write protection register
pub mod wpr;
///SSR (r) register accessor: an alias for `Reg<SSR_SPEC>`
pub type SSR = crate::Reg<ssr::SSR_SPEC>;
///sub second register
pub mod ssr;
///SHIFTR (w) register accessor: an alias for `Reg<SHIFTR_SPEC>`
pub type SHIFTR = crate::Reg<shiftr::SHIFTR_SPEC>;
///shift control register
pub mod shiftr;
///TSTR (r) register accessor: an alias for `Reg<TSTR_SPEC>`
pub type TSTR = crate::Reg<tstr::TSTR_SPEC>;
///time stamp time register
pub mod tstr;
///TSDR (r) register accessor: an alias for `Reg<TSDR_SPEC>`
pub type TSDR = crate::Reg<tsdr::TSDR_SPEC>;
///time stamp date register
pub mod tsdr;
///TSSSR (r) register accessor: an alias for `Reg<TSSSR_SPEC>`
pub type TSSSR = crate::Reg<tsssr::TSSSR_SPEC>;
///timestamp sub second register
pub mod tsssr;
///CALR (rw) register accessor: an alias for `Reg<CALR_SPEC>`
pub type CALR = crate::Reg<calr::CALR_SPEC>;
///calibration register
pub mod calr;
///ALRMASSR (rw) register accessor: an alias for `Reg<ALRMASSR_SPEC>`
pub type ALRMASSR = crate::Reg<alrmassr::ALRMASSR_SPEC>;
///alarm A sub second register
pub mod alrmassr;
///ALRMBSSR (rw) register accessor: an alias for `Reg<ALRMBSSR_SPEC>`
pub type ALRMBSSR = crate::Reg<alrmbssr::ALRMBSSR_SPEC>;
///alarm B sub second register
pub mod alrmbssr;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///RTC status register
pub mod sr;
///MISR (rw) register accessor: an alias for `Reg<MISR_SPEC>`
pub type MISR = crate::Reg<misr::MISR_SPEC>;
///RTC masked interrupt status register
pub mod misr;
///SCR (rw) register accessor: an alias for `Reg<SCR_SPEC>`
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
