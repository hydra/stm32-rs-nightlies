///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - CRS control register
    pub cr: CR,
    ///0x04 - This register can be written only when the frequency error counter is disabled (CEN bit is cleared in CRS_CR). When the counter is enabled, this register is write-protected.
    pub cfgr: CFGR,
    ///0x08 - CRS interrupt and status register
    pub isr: ISR,
    ///0x0c - CRS interrupt flag clear register
    pub icr: ICR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///CRS control register
pub mod cr;
///CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
///This register can be written only when the frequency error counter is disabled (CEN bit is cleared in CRS_CR). When the counter is enabled, this register is write-protected.
pub mod cfgr;
///ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///CRS interrupt and status register
pub mod isr;
///ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///CRS interrupt flag clear register
pub mod icr;
