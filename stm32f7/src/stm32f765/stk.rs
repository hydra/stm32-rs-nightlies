///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SysTick control and status register
    pub csr: CSR,
    ///0x04 - SysTick reload value register
    pub rvr: RVR,
    ///0x08 - SysTick current value register
    pub cvr: CVR,
    ///0x0c - SysTick calibration value register
    pub calib: CALIB,
}
///CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///SysTick control and status register
pub mod csr;
///RVR (rw) register accessor: an alias for `Reg<RVR_SPEC>`
pub type RVR = crate::Reg<rvr::RVR_SPEC>;
///SysTick reload value register
pub mod rvr;
///CVR (rw) register accessor: an alias for `Reg<CVR_SPEC>`
pub type CVR = crate::Reg<cvr::CVR_SPEC>;
///SysTick current value register
pub mod cvr;
///CALIB (rw) register accessor: an alias for `Reg<CALIB_SPEC>`
pub type CALIB = crate::Reg<calib::CALIB_SPEC>;
///SysTick calibration value register
pub mod calib;
