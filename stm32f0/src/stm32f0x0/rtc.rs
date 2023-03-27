///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - time register
    pub tr: TR,
    ///0x04 - date register
    pub dr: DR,
    ///0x08 - control register
    pub cr: CR,
    ///0x0c - initialization and status register
    pub isr: ISR,
    ///0x10 - prescaler register
    pub prer: PRER,
    _reserved5: [u8; 0x08],
    ///0x1c - alarm A register
    pub alrmar: ALRMAR,
    _reserved6: [u8; 0x04],
    ///0x24 - write protection register
    pub wpr: WPR,
    ///0x28 - sub second register
    pub ssr: SSR,
    ///0x2c - shift control register
    pub shiftr: SHIFTR,
    ///0x30 - timestamp time register
    pub tstr: TSTR,
    ///0x34 - timestamp date register
    pub tsdr: TSDR,
    ///0x38 - time-stamp sub second register
    pub tsssr: TSSSR,
    ///0x3c - calibration register
    pub calr: CALR,
    ///0x40 - tamper and alternate function configuration register
    pub tafcr: TAFCR,
    ///0x44 - alarm A sub second register
    pub alrmassr: ALRMASSR,
    _reserved15: [u8; 0x08],
    ///0x50..0x64 - backup register
    pub bkpr: [BKPR; 5],
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
///ISR (rw) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///initialization and status register
pub mod isr;
///PRER (rw) register accessor: an alias for `Reg<PRER_SPEC>`
pub type PRER = crate::Reg<prer::PRER_SPEC>;
///prescaler register
pub mod prer;
///ALRMAR (rw) register accessor: an alias for `Reg<ALRMAR_SPEC>`
pub type ALRMAR = crate::Reg<alrmar::ALRMAR_SPEC>;
///alarm A register
pub mod alrmar;
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
///timestamp time register
pub mod tstr;
///TSDR (r) register accessor: an alias for `Reg<TSDR_SPEC>`
pub type TSDR = crate::Reg<tsdr::TSDR_SPEC>;
///timestamp date register
pub mod tsdr;
///TSSSR (r) register accessor: an alias for `Reg<TSSSR_SPEC>`
pub type TSSSR = crate::Reg<tsssr::TSSSR_SPEC>;
///time-stamp sub second register
pub mod tsssr;
///CALR (rw) register accessor: an alias for `Reg<CALR_SPEC>`
pub type CALR = crate::Reg<calr::CALR_SPEC>;
///calibration register
pub mod calr;
///TAFCR (rw) register accessor: an alias for `Reg<TAFCR_SPEC>`
pub type TAFCR = crate::Reg<tafcr::TAFCR_SPEC>;
///tamper and alternate function configuration register
pub mod tafcr;
///ALRMASSR (rw) register accessor: an alias for `Reg<ALRMASSR_SPEC>`
pub type ALRMASSR = crate::Reg<alrmassr::ALRMASSR_SPEC>;
///alarm A sub second register
pub mod alrmassr;
///BKPR (rw) register accessor: an alias for `Reg<BKPR_SPEC>`
pub type BKPR = crate::Reg<bkpr::BKPR_SPEC>;
///backup register
pub mod bkpr;
