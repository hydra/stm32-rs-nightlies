///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SysTick control and status register
    pub ctrl: CTRL,
    ///0x04 - SysTick reload value register
    pub load_: LOAD_,
    ///0x08 - SysTick current value register
    pub val: VAL,
    ///0x0c - SysTick calibration value register
    pub calib: CALIB,
}
///CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
///SysTick control and status register
pub mod ctrl;
///LOAD_ (rw) register accessor: an alias for `Reg<LOAD__SPEC>`
pub type LOAD_ = crate::Reg<load_::LOAD__SPEC>;
///SysTick reload value register
pub mod load_;
///VAL (rw) register accessor: an alias for `Reg<VAL_SPEC>`
pub type VAL = crate::Reg<val::VAL_SPEC>;
///SysTick current value register
pub mod val;
///CALIB (rw) register accessor: an alias for `Reg<CALIB_SPEC>`
pub type CALIB = crate::Reg<calib::CALIB_SPEC>;
///SysTick calibration value register
pub mod calib;
