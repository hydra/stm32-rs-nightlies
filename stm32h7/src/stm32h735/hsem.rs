///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00..0x80 - HSEM register HSEM_R%s HSEM_R31
    pub r: [R; 32],
    ///0x80..0x100 - HSEM Read lock register
    pub rlr: [RLR; 32],
    ///0x100 - HSEM Interrupt enable register
    pub c1ier: C1IER,
    ///0x104 - HSEM Interrupt clear register
    pub c1icr: C1ICR,
    ///0x108 - HSEM Interrupt status register
    pub c1isr: C1ISR,
    ///0x10c - HSEM Masked interrupt status register
    pub c1misr: C1MISR,
    _reserved6: [u8; 0x30],
    ///0x140 - HSEM Clear register
    pub cr: CR,
    ///0x144 - HSEM Interrupt clear register
    pub keyr: KEYR,
}
///R (rw) register accessor: an alias for `Reg<R_SPEC>`
pub type R = crate::Reg<r::R_SPEC>;
///HSEM register HSEM_R%s HSEM_R31
pub mod r;
///RLR (r) register accessor: an alias for `Reg<RLR_SPEC>`
pub type RLR = crate::Reg<rlr::RLR_SPEC>;
///HSEM Read lock register
pub mod rlr;
///C1IER (rw) register accessor: an alias for `Reg<C1IER_SPEC>`
pub type C1IER = crate::Reg<c1ier::C1IER_SPEC>;
///HSEM Interrupt enable register
pub mod c1ier;
///C1ICR (r) register accessor: an alias for `Reg<C1ICR_SPEC>`
pub type C1ICR = crate::Reg<c1icr::C1ICR_SPEC>;
///HSEM Interrupt clear register
pub mod c1icr;
///C1ISR (r) register accessor: an alias for `Reg<C1ISR_SPEC>`
pub type C1ISR = crate::Reg<c1isr::C1ISR_SPEC>;
///HSEM Interrupt status register
pub mod c1isr;
///C1MISR (r) register accessor: an alias for `Reg<C1MISR_SPEC>`
pub type C1MISR = crate::Reg<c1misr::C1MISR_SPEC>;
///HSEM Masked interrupt status register
pub mod c1misr;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///HSEM Clear register
pub mod cr;
///KEYR (rw) register accessor: an alias for `Reg<KEYR_SPEC>`
pub type KEYR = crate::Reg<keyr::KEYR_SPEC>;
///HSEM Interrupt clear register
pub mod keyr;
