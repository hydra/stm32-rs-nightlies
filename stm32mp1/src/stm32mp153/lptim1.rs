///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - LPTIM interrupt and status register
    pub lptim_isr: LPTIM_ISR,
    ///0x04 - LPTIM interrupt clear register
    pub lptim_icr: LPTIM_ICR,
    ///0x08 - LPTIM interrupt enable register
    pub lptim_ier: LPTIM_IER,
    ///0x0c - LPTIM configuration register
    pub lptim_cfgr: LPTIM_CFGR,
    ///0x10 - LPTIM control register
    pub lptim_cr: LPTIM_CR,
    ///0x14 - LPTIM compare register
    pub lptim_cmp: LPTIM_CMP,
    ///0x18 - LPTIM autoreload register
    pub lptim_arr: LPTIM_ARR,
    ///0x1c - LPTIM counter register
    pub lptim_cnt: LPTIM_CNT,
    _reserved8: [u8; 0x04],
    ///0x24 - LPTIM configuration register 2
    pub lptim_cfgr2: LPTIM_CFGR2,
    _reserved9: [u8; 0x03c8],
    ///0x3f0 - LPTIM 1 peripheral hardware configuration register
    pub lptim1_hwcfgr: LPTIM1_HWCFGR,
    ///0x3f4 - LPTIM peripheral version identification register
    pub lptim_verr: LPTIM_VERR,
    ///0x3f8 - LPTIM peripheral type identification register
    pub lptim_pidr: LPTIM_PIDR,
    ///0x3fc - LPTIM registers map size identification register
    pub lptim_sidr: LPTIM_SIDR,
}
///LPTIM_ISR (r) register accessor: an alias for `Reg<LPTIM_ISR_SPEC>`
pub type LPTIM_ISR = crate::Reg<lptim_isr::LPTIM_ISR_SPEC>;
///LPTIM interrupt and status register
pub mod lptim_isr;
///LPTIM_ICR (w) register accessor: an alias for `Reg<LPTIM_ICR_SPEC>`
pub type LPTIM_ICR = crate::Reg<lptim_icr::LPTIM_ICR_SPEC>;
///LPTIM interrupt clear register
pub mod lptim_icr;
///LPTIM_IER (rw) register accessor: an alias for `Reg<LPTIM_IER_SPEC>`
pub type LPTIM_IER = crate::Reg<lptim_ier::LPTIM_IER_SPEC>;
///LPTIM interrupt enable register
pub mod lptim_ier;
///LPTIM_CFGR (rw) register accessor: an alias for `Reg<LPTIM_CFGR_SPEC>`
pub type LPTIM_CFGR = crate::Reg<lptim_cfgr::LPTIM_CFGR_SPEC>;
///LPTIM configuration register
pub mod lptim_cfgr;
///LPTIM_CR (rw) register accessor: an alias for `Reg<LPTIM_CR_SPEC>`
pub type LPTIM_CR = crate::Reg<lptim_cr::LPTIM_CR_SPEC>;
///LPTIM control register
pub mod lptim_cr;
///LPTIM_CMP (rw) register accessor: an alias for `Reg<LPTIM_CMP_SPEC>`
pub type LPTIM_CMP = crate::Reg<lptim_cmp::LPTIM_CMP_SPEC>;
///LPTIM compare register
pub mod lptim_cmp;
///LPTIM_ARR (rw) register accessor: an alias for `Reg<LPTIM_ARR_SPEC>`
pub type LPTIM_ARR = crate::Reg<lptim_arr::LPTIM_ARR_SPEC>;
///LPTIM autoreload register
pub mod lptim_arr;
///LPTIM_CNT (r) register accessor: an alias for `Reg<LPTIM_CNT_SPEC>`
pub type LPTIM_CNT = crate::Reg<lptim_cnt::LPTIM_CNT_SPEC>;
///LPTIM counter register
pub mod lptim_cnt;
///LPTIM_CFGR2 (rw) register accessor: an alias for `Reg<LPTIM_CFGR2_SPEC>`
pub type LPTIM_CFGR2 = crate::Reg<lptim_cfgr2::LPTIM_CFGR2_SPEC>;
///LPTIM configuration register 2
pub mod lptim_cfgr2;
///LPTIM1_HWCFGR (r) register accessor: an alias for `Reg<LPTIM1_HWCFGR_SPEC>`
pub type LPTIM1_HWCFGR = crate::Reg<lptim1_hwcfgr::LPTIM1_HWCFGR_SPEC>;
///LPTIM 1 peripheral hardware configuration register
pub mod lptim1_hwcfgr;
///LPTIM_VERR (r) register accessor: an alias for `Reg<LPTIM_VERR_SPEC>`
pub type LPTIM_VERR = crate::Reg<lptim_verr::LPTIM_VERR_SPEC>;
///LPTIM peripheral version identification register
pub mod lptim_verr;
///LPTIM_PIDR (r) register accessor: an alias for `Reg<LPTIM_PIDR_SPEC>`
pub type LPTIM_PIDR = crate::Reg<lptim_pidr::LPTIM_PIDR_SPEC>;
///LPTIM peripheral type identification register
pub mod lptim_pidr;
///LPTIM_SIDR (r) register accessor: an alias for `Reg<LPTIM_SIDR_SPEC>`
pub type LPTIM_SIDR = crate::Reg<lptim_sidr::LPTIM_SIDR_SPEC>;
///LPTIM registers map size identification register
pub mod lptim_sidr;
