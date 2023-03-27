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
    ///0x14 - wakeup timer register
    pub wutr: WUTR,
    ///0x18 - calibration register
    pub calibr: CALIBR,
    ///0x1c - alarm A register
    pub alrmar: ALRMAR,
    ///0x20 - alarm B register
    pub alrmbr: ALRMBR,
    ///0x24 - write protection register
    pub wpr: WPR,
    _reserved10: [u8; 0x08],
    ///0x30 - time stamp time register
    pub tstr: TSTR,
    ///0x34 - time stamp date register
    pub tsdr: TSDR,
    _reserved12: [u8; 0x08],
    ///0x40 - tamper and alternate function configuration register
    pub tafcr: TAFCR,
    _reserved13: [u8; 0x0c],
    ///0x50..0xa0 - backup register
    pub bkpr: [BKPR; 20],
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
///WUTR (rw) register accessor: an alias for `Reg<WUTR_SPEC>`
pub type WUTR = crate::Reg<wutr::WUTR_SPEC>;
///wakeup timer register
pub mod wutr;
///CALIBR (rw) register accessor: an alias for `Reg<CALIBR_SPEC>`
pub type CALIBR = crate::Reg<calibr::CALIBR_SPEC>;
///calibration register
pub mod calibr;
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
///TSTR (r) register accessor: an alias for `Reg<TSTR_SPEC>`
pub type TSTR = crate::Reg<tstr::TSTR_SPEC>;
///time stamp time register
pub mod tstr;
///TSDR (r) register accessor: an alias for `Reg<TSDR_SPEC>`
pub type TSDR = crate::Reg<tsdr::TSDR_SPEC>;
///time stamp date register
pub mod tsdr;
///TAFCR (rw) register accessor: an alias for `Reg<TAFCR_SPEC>`
pub type TAFCR = crate::Reg<tafcr::TAFCR_SPEC>;
///tamper and alternate function configuration register
pub mod tafcr;
///BKPR (rw) register accessor: an alias for `Reg<BKPR_SPEC>`
pub type BKPR = crate::Reg<bkpr::BKPR_SPEC>;
///backup register
pub mod bkpr;
