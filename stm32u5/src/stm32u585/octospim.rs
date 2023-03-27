///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: CR,
    ///0x04 - OCTOSPI I/O manager Port 1 configuration register
    pub p1cr: P1CR,
    ///0x08 - OCTOSPI I/O manager Port 2 configuration register
    pub p2cr: P2CR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///control register
pub mod cr;
///P1CR (rw) register accessor: an alias for `Reg<P1CR_SPEC>`
pub type P1CR = crate::Reg<p1cr::P1CR_SPEC>;
///OCTOSPI I/O manager Port 1 configuration register
pub mod p1cr;
///P2CR (rw) register accessor: an alias for `Reg<P2CR_SPEC>`
pub type P2CR = crate::Reg<p2cr::P2CR_SPEC>;
///OCTOSPI I/O manager Port 2 configuration register
pub mod p2cr;
