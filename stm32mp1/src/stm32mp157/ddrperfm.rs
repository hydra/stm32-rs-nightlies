///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Write-only register. A read request returns all zeros.
    pub ddrperfm_ctl: DDRPERFM_CTL,
    ///0x04 - DDRPERFM configurationl register
    pub ddrperfm_cfg: DDRPERFM_CFG,
    ///0x08 - DDRPERFM status register
    pub ddrperfm_status: DDRPERFM_STATUS,
    ///0x0c - Write-only register. A read request returns all zeros
    pub ddrperfm_ccr: DDRPERFM_CCR,
    ///0x10 - DDRPERFM interrupt enable register
    pub ddrperfm_ier: DDRPERFM_IER,
    ///0x14 - DDRPERFM interrupt status register
    pub ddrperfm_isr: DDRPERFM_ISR,
    ///0x18 - Write-only register. A read request returns all zeros
    pub ddrperfm_icr: DDRPERFM_ICR,
    _reserved7: [u8; 0x04],
    ///0x20 - DDRPERFM time counter register
    pub ddrperfm_tcnt: DDRPERFM_TCNT,
    _reserved8: [u8; 0x3c],
    ///0x60 - DDRPERFM event counter 0 register
    pub ddrperfm_cnt0: DDRPERFM_CNT0,
    _reserved9: [u8; 0x04],
    ///0x68 - DDRPERFM event counter 1 register
    pub ddrperfm_cnt1: DDRPERFM_CNT1,
    _reserved10: [u8; 0x04],
    ///0x70 - DDRPERFM event counter 2 register
    pub ddrperfm_cnt2: DDRPERFM_CNT2,
    _reserved11: [u8; 0x04],
    ///0x78 - DDRPERFM event counter 3 register
    pub ddrperfm_cnt3: DDRPERFM_CNT3,
    _reserved12: [u8; 0x0374],
    ///0x3f0 - DDRPERFM hardware configuration register
    pub ddrperfm_hwcfg: DDRPERFM_HWCFG,
    ///0x3f4 - DDRPERFM version register
    pub ddrperfm_ver: DDRPERFM_VER,
    ///0x3f8 - DDRPERFM ID register
    pub ddrperfm_id: DDRPERFM_ID,
    ///0x3fc - DDRPERFM magic ID register
    pub ddrperfm_sid: DDRPERFM_SID,
}
///DDRPERFM_CTL (w) register accessor: an alias for `Reg<DDRPERFM_CTL_SPEC>`
pub type DDRPERFM_CTL = crate::Reg<ddrperfm_ctl::DDRPERFM_CTL_SPEC>;
///Write-only register. A read request returns all zeros.
pub mod ddrperfm_ctl;
///DDRPERFM_CFG (rw) register accessor: an alias for `Reg<DDRPERFM_CFG_SPEC>`
pub type DDRPERFM_CFG = crate::Reg<ddrperfm_cfg::DDRPERFM_CFG_SPEC>;
///DDRPERFM configurationl register
pub mod ddrperfm_cfg;
///DDRPERFM_STATUS (r) register accessor: an alias for `Reg<DDRPERFM_STATUS_SPEC>`
pub type DDRPERFM_STATUS = crate::Reg<ddrperfm_status::DDRPERFM_STATUS_SPEC>;
///DDRPERFM status register
pub mod ddrperfm_status;
///DDRPERFM_CCR (w) register accessor: an alias for `Reg<DDRPERFM_CCR_SPEC>`
pub type DDRPERFM_CCR = crate::Reg<ddrperfm_ccr::DDRPERFM_CCR_SPEC>;
///Write-only register. A read request returns all zeros
pub mod ddrperfm_ccr;
///DDRPERFM_IER (rw) register accessor: an alias for `Reg<DDRPERFM_IER_SPEC>`
pub type DDRPERFM_IER = crate::Reg<ddrperfm_ier::DDRPERFM_IER_SPEC>;
///DDRPERFM interrupt enable register
pub mod ddrperfm_ier;
///DDRPERFM_ISR (r) register accessor: an alias for `Reg<DDRPERFM_ISR_SPEC>`
pub type DDRPERFM_ISR = crate::Reg<ddrperfm_isr::DDRPERFM_ISR_SPEC>;
///DDRPERFM interrupt status register
pub mod ddrperfm_isr;
///DDRPERFM_ICR (w) register accessor: an alias for `Reg<DDRPERFM_ICR_SPEC>`
pub type DDRPERFM_ICR = crate::Reg<ddrperfm_icr::DDRPERFM_ICR_SPEC>;
///Write-only register. A read request returns all zeros
pub mod ddrperfm_icr;
///DDRPERFM_TCNT (r) register accessor: an alias for `Reg<DDRPERFM_TCNT_SPEC>`
pub type DDRPERFM_TCNT = crate::Reg<ddrperfm_tcnt::DDRPERFM_TCNT_SPEC>;
///DDRPERFM time counter register
pub mod ddrperfm_tcnt;
///DDRPERFM_CNT0 (r) register accessor: an alias for `Reg<DDRPERFM_CNT0_SPEC>`
pub type DDRPERFM_CNT0 = crate::Reg<ddrperfm_cnt0::DDRPERFM_CNT0_SPEC>;
///DDRPERFM event counter 0 register
pub mod ddrperfm_cnt0;
///DDRPERFM_CNT1 (r) register accessor: an alias for `Reg<DDRPERFM_CNT1_SPEC>`
pub type DDRPERFM_CNT1 = crate::Reg<ddrperfm_cnt1::DDRPERFM_CNT1_SPEC>;
///DDRPERFM event counter 1 register
pub mod ddrperfm_cnt1;
///DDRPERFM_CNT2 (r) register accessor: an alias for `Reg<DDRPERFM_CNT2_SPEC>`
pub type DDRPERFM_CNT2 = crate::Reg<ddrperfm_cnt2::DDRPERFM_CNT2_SPEC>;
///DDRPERFM event counter 2 register
pub mod ddrperfm_cnt2;
///DDRPERFM_CNT3 (r) register accessor: an alias for `Reg<DDRPERFM_CNT3_SPEC>`
pub type DDRPERFM_CNT3 = crate::Reg<ddrperfm_cnt3::DDRPERFM_CNT3_SPEC>;
///DDRPERFM event counter 3 register
pub mod ddrperfm_cnt3;
///DDRPERFM_HWCFG (r) register accessor: an alias for `Reg<DDRPERFM_HWCFG_SPEC>`
pub type DDRPERFM_HWCFG = crate::Reg<ddrperfm_hwcfg::DDRPERFM_HWCFG_SPEC>;
///DDRPERFM hardware configuration register
pub mod ddrperfm_hwcfg;
///DDRPERFM_VER (r) register accessor: an alias for `Reg<DDRPERFM_VER_SPEC>`
pub type DDRPERFM_VER = crate::Reg<ddrperfm_ver::DDRPERFM_VER_SPEC>;
///DDRPERFM version register
pub mod ddrperfm_ver;
///DDRPERFM_ID (r) register accessor: an alias for `Reg<DDRPERFM_ID_SPEC>`
pub type DDRPERFM_ID = crate::Reg<ddrperfm_id::DDRPERFM_ID_SPEC>;
///DDRPERFM ID register
pub mod ddrperfm_id;
///DDRPERFM_SID (r) register accessor: an alias for `Reg<DDRPERFM_SID_SPEC>`
pub type DDRPERFM_SID = crate::Reg<ddrperfm_sid::DDRPERFM_SID_SPEC>;
///DDRPERFM magic ID register
pub mod ddrperfm_sid;
