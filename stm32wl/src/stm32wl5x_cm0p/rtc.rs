///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Time register
    pub tr: TR,
    ///0x04 - Date register
    pub dr: DR,
    ///0x08 - Sub second register
    pub ssr: SSR,
    ///0x0c - Initialization control and status register
    pub icsr: ICSR,
    ///0x10 - Pre-scaler register
    pub prer: PRER,
    ///0x14 - Wakeup timer register
    pub wutr: WUTR,
    ///0x18 - Control register
    pub cr: CR,
    _reserved7: [u8; 0x08],
    ///0x24 - Write protection register
    pub wpr: WPR,
    ///0x28 - Calibration register
    pub calr: CALR,
    ///0x2c - Shift control register
    pub shiftr: SHIFTR,
    ///0x30 - Timestamp time register
    pub tstr: TSTR,
    ///0x34 - Timestamp date register
    pub tsdr: TSDR,
    ///0x38 - Timestamp sub second register
    pub tsssr: TSSSR,
    _reserved13: [u8; 0x04],
    ///0x40 - Alarm register
    pub alrmar: ALRMR,
    ///0x44 - Alarm sub-second register
    pub alrmassr: ALRMSSR,
    ///0x48 - Alarm register
    pub alrmbr: ALRMR,
    ///0x4c - Alarm sub-second register
    pub alrmbssr: ALRMSSR,
    ///0x50 - Status register (interrupts)
    pub sr: SR,
    ///0x54 - Masked interrupt status register
    pub misr: MISR,
    _reserved19: [u8; 0x04],
    ///0x5c - Status clear register (interrupts)
    pub scr: SCR,
    _reserved20: [u8; 0x10],
    ///0x70..0x78 - RTC alarm A binary mode register
    pub alrbinr: [ALRBINR; 2],
}
impl RegisterBlock {
    ///0x70 - RTC alarm A binary mode register
    #[inline(always)]
    pub fn alrabinr(&self) -> &ALRBINR {
        &self.alrbinr[0]
    }
    ///0x74 - RTC alarm A binary mode register
    #[inline(always)]
    pub fn alrbbinr(&self) -> &ALRBINR {
        &self.alrbinr[1]
    }
}
///TR (rw) register accessor: an alias for `Reg<TR_SPEC>`
pub type TR = crate::Reg<tr::TR_SPEC>;
///Time register
pub mod tr;
///DR (rw) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///Date register
pub mod dr;
///SSR (r) register accessor: an alias for `Reg<SSR_SPEC>`
pub type SSR = crate::Reg<ssr::SSR_SPEC>;
///Sub second register
pub mod ssr;
///ICSR (rw) register accessor: an alias for `Reg<ICSR_SPEC>`
pub type ICSR = crate::Reg<icsr::ICSR_SPEC>;
///Initialization control and status register
pub mod icsr;
///PRER (rw) register accessor: an alias for `Reg<PRER_SPEC>`
pub type PRER = crate::Reg<prer::PRER_SPEC>;
///Pre-scaler register
pub mod prer;
///WUTR (rw) register accessor: an alias for `Reg<WUTR_SPEC>`
pub type WUTR = crate::Reg<wutr::WUTR_SPEC>;
///Wakeup timer register
pub mod wutr;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Control register
pub mod cr;
///WPR (w) register accessor: an alias for `Reg<WPR_SPEC>`
pub type WPR = crate::Reg<wpr::WPR_SPEC>;
///Write protection register
pub mod wpr;
///CALR (rw) register accessor: an alias for `Reg<CALR_SPEC>`
pub type CALR = crate::Reg<calr::CALR_SPEC>;
///Calibration register
pub mod calr;
///SHIFTR (w) register accessor: an alias for `Reg<SHIFTR_SPEC>`
pub type SHIFTR = crate::Reg<shiftr::SHIFTR_SPEC>;
///Shift control register
pub mod shiftr;
///TSTR (r) register accessor: an alias for `Reg<TSTR_SPEC>`
pub type TSTR = crate::Reg<tstr::TSTR_SPEC>;
///Timestamp time register
pub mod tstr;
///TSDR (r) register accessor: an alias for `Reg<TSDR_SPEC>`
pub type TSDR = crate::Reg<tsdr::TSDR_SPEC>;
///Timestamp date register
pub mod tsdr;
///TSSSR (r) register accessor: an alias for `Reg<TSSSR_SPEC>`
pub type TSSSR = crate::Reg<tsssr::TSSSR_SPEC>;
///Timestamp sub second register
pub mod tsssr;
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
///Status register (interrupts)
pub mod sr;
///MISR (r) register accessor: an alias for `Reg<MISR_SPEC>`
pub type MISR = crate::Reg<misr::MISR_SPEC>;
///Masked interrupt status register
pub mod misr;
///SCR (w) register accessor: an alias for `Reg<SCR_SPEC>`
pub type SCR = crate::Reg<scr::SCR_SPEC>;
///Status clear register (interrupts)
pub mod scr;
///ALRBINR (rw) register accessor: an alias for `Reg<ALRBINR_SPEC>`
pub type ALRBINR = crate::Reg<alrbinr::ALRBINR_SPEC>;
///RTC alarm A binary mode register
pub mod alrbinr;
