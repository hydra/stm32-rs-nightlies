///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: CR,
    ///0x04 - interrupt enable register
    pub ier: IER,
    ///0x08 - interrupt clear register
    pub icr: ICR,
    ///0x0c - interrupt status register
    pub isr: ISR,
    ///0x10 - I/O hysteresis control register
    pub iohcr: IOHCR,
    _reserved5: [u8; 0x04],
    ///0x18 - I/O analog switch control register
    pub ioascr: IOASCR,
    _reserved6: [u8; 0x04],
    ///0x20 - I/O sampling control register
    pub ioscr: IOSCR,
    _reserved7: [u8; 0x04],
    ///0x28 - I/O channel control register
    pub ioccr: IOCCR,
    _reserved8: [u8; 0x04],
    ///0x30 - I/O group control status register
    pub iogcsr: IOGCSR,
    ///0x34..0x54 - I/O group x counter register
    pub iogcr: [IOGCR; 8],
}
impl RegisterBlock {
    ///0x34 - I/O group x counter register
    #[inline(always)]
    pub fn iog1cr(&self) -> &IOGCR {
        &self.iogcr[0]
    }
    ///0x38 - I/O group x counter register
    #[inline(always)]
    pub fn iog2cr(&self) -> &IOGCR {
        &self.iogcr[1]
    }
    ///0x3c - I/O group x counter register
    #[inline(always)]
    pub fn iog3cr(&self) -> &IOGCR {
        &self.iogcr[2]
    }
    ///0x40 - I/O group x counter register
    #[inline(always)]
    pub fn iog4cr(&self) -> &IOGCR {
        &self.iogcr[3]
    }
    ///0x44 - I/O group x counter register
    #[inline(always)]
    pub fn iog5cr(&self) -> &IOGCR {
        &self.iogcr[4]
    }
    ///0x48 - I/O group x counter register
    #[inline(always)]
    pub fn iog6cr(&self) -> &IOGCR {
        &self.iogcr[5]
    }
    ///0x4c - I/O group x counter register
    #[inline(always)]
    pub fn iog7cr(&self) -> &IOGCR {
        &self.iogcr[6]
    }
    ///0x50 - I/O group x counter register
    #[inline(always)]
    pub fn iog8cr(&self) -> &IOGCR {
        &self.iogcr[7]
    }
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///control register
pub mod cr;
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///interrupt enable register
pub mod ier;
///ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///interrupt clear register
pub mod icr;
///ISR (rw) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///interrupt status register
pub mod isr;
///IOHCR (rw) register accessor: an alias for `Reg<IOHCR_SPEC>`
pub type IOHCR = crate::Reg<iohcr::IOHCR_SPEC>;
///I/O hysteresis control register
pub mod iohcr;
///IOASCR (rw) register accessor: an alias for `Reg<IOASCR_SPEC>`
pub type IOASCR = crate::Reg<ioascr::IOASCR_SPEC>;
///I/O analog switch control register
pub mod ioascr;
///IOSCR (rw) register accessor: an alias for `Reg<IOSCR_SPEC>`
pub type IOSCR = crate::Reg<ioscr::IOSCR_SPEC>;
///I/O sampling control register
pub mod ioscr;
///IOCCR (rw) register accessor: an alias for `Reg<IOCCR_SPEC>`
pub type IOCCR = crate::Reg<ioccr::IOCCR_SPEC>;
///I/O channel control register
pub mod ioccr;
///IOGCSR (rw) register accessor: an alias for `Reg<IOGCSR_SPEC>`
pub type IOGCSR = crate::Reg<iogcsr::IOGCSR_SPEC>;
///I/O group control status register
pub mod iogcsr;
///IOGCR (r) register accessor: an alias for `Reg<IOGCR_SPEC>`
pub type IOGCR = crate::Reg<iogcr::IOGCR_SPEC>;
///I/O group x counter register
pub mod iogcr;
