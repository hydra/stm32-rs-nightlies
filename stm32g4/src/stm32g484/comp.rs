///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Comparator control/status register
    pub c1csr: C1CSR,
    ///0x04 - Comparator control/status register
    pub c2csr: C2CSR,
    ///0x08 - Comparator control/status register
    pub c3csr: C3CSR,
    ///0x0c - Comparator control/status register
    pub c4csr: C4CSR,
    ///0x10 - Comparator control/status register
    pub c5csr: C5CSR,
    ///0x14 - Comparator control/status register
    pub c6csr: C6CSR,
    ///0x18 - Comparator control/status register
    pub c7csr: C7CSR,
}
///C1CSR (rw) register accessor: an alias for `Reg<C1CSR_SPEC>`
pub type C1CSR = crate::Reg<c1csr::C1CSR_SPEC>;
///Comparator control/status register
pub mod c1csr;
///C2CSR (rw) register accessor: an alias for `Reg<C2CSR_SPEC>`
pub type C2CSR = crate::Reg<c2csr::C2CSR_SPEC>;
///Comparator control/status register
pub mod c2csr;
///C3CSR (rw) register accessor: an alias for `Reg<C3CSR_SPEC>`
pub type C3CSR = crate::Reg<c3csr::C3CSR_SPEC>;
///Comparator control/status register
pub mod c3csr;
///C4CSR (rw) register accessor: an alias for `Reg<C4CSR_SPEC>`
pub type C4CSR = crate::Reg<c4csr::C4CSR_SPEC>;
///Comparator control/status register
pub mod c4csr;
///C5CSR (rw) register accessor: an alias for `Reg<C5CSR_SPEC>`
pub type C5CSR = crate::Reg<c5csr::C5CSR_SPEC>;
///Comparator control/status register
pub mod c5csr;
///C6CSR (rw) register accessor: an alias for `Reg<C6CSR_SPEC>`
pub type C6CSR = crate::Reg<c6csr::C6CSR_SPEC>;
///Comparator control/status register
pub mod c6csr;
///C7CSR (rw) register accessor: an alias for `Reg<C7CSR_SPEC>`
pub type C7CSR = crate::Reg<c7csr::C7CSR_SPEC>;
///Comparator control/status register
pub mod c7csr;
