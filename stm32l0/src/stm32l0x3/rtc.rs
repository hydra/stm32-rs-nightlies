///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - RTC time register
    pub tr: TR,
    ///0x04 - RTC date register
    pub dr: DR,
    ///0x08 - RTC control register
    pub cr: CR,
    ///0x0c - RTC initialization and status register
    pub isr: ISR,
    ///0x10 - RTC prescaler register
    pub prer: PRER,
    ///0x14 - RTC wakeup timer register
    pub wutr: WUTR,
    _reserved6: [u8; 0x04],
    ///0x1c..0x24 - Alarm register
    pub alrmr: [ALRMR; 2],
    ///0x24 - write protection register
    pub wpr: WPR,
    ///0x28 - RTC sub second register
    pub ssr: SSR,
    ///0x2c - RTC shift control register
    pub shiftr: SHIFTR,
    ///0x30 - RTC timestamp time register
    pub tstr: TSTR,
    ///0x34 - RTC timestamp date register
    pub tsdr: TSDR,
    ///0x38 - RTC time-stamp sub second register
    pub tsssr: TSSSR,
    ///0x3c - RTC calibration register
    pub calr: CALR,
    ///0x40 - RTC tamper configuration register
    pub tampcr: TAMPCR,
    ///0x44..0x4c - Alarm sub-second register
    pub alrmssr: [ALRMSSR; 2],
    ///0x4c - option register
    pub or: OR,
    ///0x50..0x64 - RTC backup registers
    pub bkpr: [BKPR; 5],
}
impl RegisterBlock {
    ///0x1c - Alarm register
    #[inline(always)]
    pub fn alrmar(&self) -> &ALRMR {
        &self.alrmr[0]
    }
    ///0x20 - Alarm register
    #[inline(always)]
    pub fn alrmbr(&self) -> &ALRMR {
        &self.alrmr[1]
    }
    ///0x44 - Alarm sub-second register
    #[inline(always)]
    pub fn alrmassr(&self) -> &ALRMSSR {
        &self.alrmssr[0]
    }
    ///0x48 - Alarm sub-second register
    #[inline(always)]
    pub fn alrmbssr(&self) -> &ALRMSSR {
        &self.alrmssr[1]
    }
}
///TR (rw) register accessor: an alias for `Reg<TR_SPEC>`
pub type TR = crate::Reg<tr::TR_SPEC>;
///RTC time register
pub mod tr;
///DR (rw) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///RTC date register
pub mod dr;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///RTC control register
pub mod cr;
///ISR (rw) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///RTC initialization and status register
pub mod isr;
///PRER (rw) register accessor: an alias for `Reg<PRER_SPEC>`
pub type PRER = crate::Reg<prer::PRER_SPEC>;
///RTC prescaler register
pub mod prer;
///WUTR (rw) register accessor: an alias for `Reg<WUTR_SPEC>`
pub type WUTR = crate::Reg<wutr::WUTR_SPEC>;
///RTC wakeup timer register
pub mod wutr;
///ALRMR (rw) register accessor: an alias for `Reg<ALRMR_SPEC>`
pub type ALRMR = crate::Reg<alrmr::ALRMR_SPEC>;
///Alarm register
pub mod alrmr;
///WPR (w) register accessor: an alias for `Reg<WPR_SPEC>`
pub type WPR = crate::Reg<wpr::WPR_SPEC>;
///write protection register
pub mod wpr;
///SSR (r) register accessor: an alias for `Reg<SSR_SPEC>`
pub type SSR = crate::Reg<ssr::SSR_SPEC>;
///RTC sub second register
pub mod ssr;
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
///CALR (rw) register accessor: an alias for `Reg<CALR_SPEC>`
pub type CALR = crate::Reg<calr::CALR_SPEC>;
///RTC calibration register
pub mod calr;
///TAMPCR (rw) register accessor: an alias for `Reg<TAMPCR_SPEC>`
pub type TAMPCR = crate::Reg<tampcr::TAMPCR_SPEC>;
///RTC tamper configuration register
pub mod tampcr;
///ALRMSSR (rw) register accessor: an alias for `Reg<ALRMSSR_SPEC>`
pub type ALRMSSR = crate::Reg<alrmssr::ALRMSSR_SPEC>;
///Alarm sub-second register
pub mod alrmssr;
///OR (rw) register accessor: an alias for `Reg<OR_SPEC>`
pub type OR = crate::Reg<or::OR_SPEC>;
///option register
pub mod or;
///BKPR (rw) register accessor: an alias for `Reg<BKPR_SPEC>`
pub type BKPR = crate::Reg<bkpr::BKPR_SPEC>;
///RTC backup registers
pub mod bkpr;
