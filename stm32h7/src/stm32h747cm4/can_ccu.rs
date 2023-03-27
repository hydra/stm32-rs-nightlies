///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Clock Calibration Unit Core Release Register
    pub crel: CREL,
    ///0x04 - Calibration Configuration Register
    pub ccfg: CCFG,
    ///0x08 - Calibration Status Register
    pub cstat: CSTAT,
    ///0x0c - Calibration Watchdog Register
    pub cwd: CWD,
    ///0x10 - Clock Calibration Unit Interrupt Register
    pub ir: IR,
    ///0x14 - Clock Calibration Unit Interrupt Enable Register
    pub ie: IE,
}
///CREL (rw) register accessor: an alias for `Reg<CREL_SPEC>`
pub type CREL = crate::Reg<crel::CREL_SPEC>;
///Clock Calibration Unit Core Release Register
pub mod crel;
///CCFG (rw) register accessor: an alias for `Reg<CCFG_SPEC>`
pub type CCFG = crate::Reg<ccfg::CCFG_SPEC>;
///Calibration Configuration Register
pub mod ccfg;
///CSTAT (rw) register accessor: an alias for `Reg<CSTAT_SPEC>`
pub type CSTAT = crate::Reg<cstat::CSTAT_SPEC>;
///Calibration Status Register
pub mod cstat;
///CWD (rw) register accessor: an alias for `Reg<CWD_SPEC>`
pub type CWD = crate::Reg<cwd::CWD_SPEC>;
///Calibration Watchdog Register
pub mod cwd;
///IR (rw) register accessor: an alias for `Reg<IR_SPEC>`
pub type IR = crate::Reg<ir::IR_SPEC>;
///Clock Calibration Unit Interrupt Register
pub mod ir;
///IE (rw) register accessor: an alias for `Reg<IE_SPEC>`
pub type IE = crate::Reg<ie::IE_SPEC>;
///Clock Calibration Unit Interrupt Enable Register
pub mod ie;
