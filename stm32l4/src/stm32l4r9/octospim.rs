///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    ///0x04 - OctoSPI IO Manager Port 1 Configuration Register
    pub p1cr: P1CR,
    ///0x08 - OctoSPI IO Manager Port 2 Configuration Register
    pub p2cr: P2CR,
}
///P1CR (rw) register accessor: an alias for `Reg<P1CR_SPEC>`
pub type P1CR = crate::Reg<p1cr::P1CR_SPEC>;
///OctoSPI IO Manager Port 1 Configuration Register
pub mod p1cr;
///P2CR (rw) register accessor: an alias for `Reg<P2CR_SPEC>`
pub type P2CR = crate::Reg<p2cr::P2CR_SPEC>;
///OctoSPI IO Manager Port 2 Configuration Register
pub mod p2cr;
