///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Instruction and Data Tightly-Coupled Memory Control Registers
    pub itcmcr: ITCMCR,
    ///0x04 - Instruction and Data Tightly-Coupled Memory Control Registers
    pub dtcmcr: DTCMCR,
    ///0x08 - AHBP Control register
    pub ahbpcr: AHBPCR,
    ///0x0c - Auxiliary Cache Control register
    pub cacr: CACR,
    ///0x10 - AHB Slave Control register
    pub ahbscr: AHBSCR,
    _reserved5: [u8; 0x04],
    ///0x18 - Auxiliary Bus Fault Status register
    pub abfsr: ABFSR,
}
///ITCMCR (rw) register accessor: an alias for `Reg<ITCMCR_SPEC>`
pub type ITCMCR = crate::Reg<itcmcr::ITCMCR_SPEC>;
///Instruction and Data Tightly-Coupled Memory Control Registers
pub mod itcmcr;
///DTCMCR (rw) register accessor: an alias for `Reg<DTCMCR_SPEC>`
pub type DTCMCR = crate::Reg<dtcmcr::DTCMCR_SPEC>;
///Instruction and Data Tightly-Coupled Memory Control Registers
pub mod dtcmcr;
///AHBPCR (rw) register accessor: an alias for `Reg<AHBPCR_SPEC>`
pub type AHBPCR = crate::Reg<ahbpcr::AHBPCR_SPEC>;
///AHBP Control register
pub mod ahbpcr;
///CACR (rw) register accessor: an alias for `Reg<CACR_SPEC>`
pub type CACR = crate::Reg<cacr::CACR_SPEC>;
///Auxiliary Cache Control register
pub mod cacr;
///AHBSCR (rw) register accessor: an alias for `Reg<AHBSCR_SPEC>`
pub type AHBSCR = crate::Reg<ahbscr::AHBSCR_SPEC>;
///AHB Slave Control register
pub mod ahbscr;
///ABFSR (rw) register accessor: an alias for `Reg<ABFSR_SPEC>`
pub type ABFSR = crate::Reg<abfsr::ABFSR_SPEC>;
///Auxiliary Bus Fault Status register
pub mod abfsr;
