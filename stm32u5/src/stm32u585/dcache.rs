///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DCACHE control register
    pub dcache_cr: DCACHE_CR,
    ///0x04 - DCACHE status register
    pub dcache_sr: DCACHE_SR,
    ///0x08 - DCACHE interrupt enable register
    pub dcache_ier: DCACHE_IER,
    ///0x0c - DCACHE flag clear register
    pub dcache_fcr: DCACHE_FCR,
    ///0x10 - DCACHE read-hit monitor register
    pub dcache_rhmonr: DCACHE_RHMONR,
    ///0x14 - DCACHE read-miss monitor register
    pub dcache_rmmonr: DCACHE_RMMONR,
    _reserved6: [u8; 0x08],
    ///0x20 - write-hit monitor register
    pub dcache_whmonr: DCACHE_WHMONR,
    ///0x24 - write-miss monitor register
    pub dcache_wmmonr: DCACHE_WMMONR,
    ///0x28 - command range start address register
    pub dcache_cmdrsaddrr: DCACHE_CMDRSADDRR,
    ///0x2c - command range start address register
    pub dcache_cmdreaddrr: DCACHE_CMDREADDRR,
}
///DCACHE_CR (rw) register accessor: an alias for `Reg<DCACHE_CR_SPEC>`
pub type DCACHE_CR = crate::Reg<dcache_cr::DCACHE_CR_SPEC>;
///DCACHE control register
pub mod dcache_cr;
///DCACHE_SR (r) register accessor: an alias for `Reg<DCACHE_SR_SPEC>`
pub type DCACHE_SR = crate::Reg<dcache_sr::DCACHE_SR_SPEC>;
///DCACHE status register
pub mod dcache_sr;
///DCACHE_IER (rw) register accessor: an alias for `Reg<DCACHE_IER_SPEC>`
pub type DCACHE_IER = crate::Reg<dcache_ier::DCACHE_IER_SPEC>;
///DCACHE interrupt enable register
pub mod dcache_ier;
///DCACHE_FCR (w) register accessor: an alias for `Reg<DCACHE_FCR_SPEC>`
pub type DCACHE_FCR = crate::Reg<dcache_fcr::DCACHE_FCR_SPEC>;
///DCACHE flag clear register
pub mod dcache_fcr;
///DCACHE_RHMONR (r) register accessor: an alias for `Reg<DCACHE_RHMONR_SPEC>`
pub type DCACHE_RHMONR = crate::Reg<dcache_rhmonr::DCACHE_RHMONR_SPEC>;
///DCACHE read-hit monitor register
pub mod dcache_rhmonr;
///DCACHE_RMMONR (r) register accessor: an alias for `Reg<DCACHE_RMMONR_SPEC>`
pub type DCACHE_RMMONR = crate::Reg<dcache_rmmonr::DCACHE_RMMONR_SPEC>;
///DCACHE read-miss monitor register
pub mod dcache_rmmonr;
///DCACHE_WHMONR (r) register accessor: an alias for `Reg<DCACHE_WHMONR_SPEC>`
pub type DCACHE_WHMONR = crate::Reg<dcache_whmonr::DCACHE_WHMONR_SPEC>;
///write-hit monitor register
pub mod dcache_whmonr;
///DCACHE_WMMONR (r) register accessor: an alias for `Reg<DCACHE_WMMONR_SPEC>`
pub type DCACHE_WMMONR = crate::Reg<dcache_wmmonr::DCACHE_WMMONR_SPEC>;
///write-miss monitor register
pub mod dcache_wmmonr;
///DCACHE_CMDRSADDRR (rw) register accessor: an alias for `Reg<DCACHE_CMDRSADDRR_SPEC>`
pub type DCACHE_CMDRSADDRR = crate::Reg<dcache_cmdrsaddrr::DCACHE_CMDRSADDRR_SPEC>;
///command range start address register
pub mod dcache_cmdrsaddrr;
///DCACHE_CMDREADDRR (rw) register accessor: an alias for `Reg<DCACHE_CMDREADDRR_SPEC>`
pub type DCACHE_CMDREADDRR = crate::Reg<dcache_cmdreaddrr::DCACHE_CMDREADDRR_SPEC>;
///command range start address register
pub mod dcache_cmdreaddrr;
