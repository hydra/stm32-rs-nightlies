///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Data register
    pub dr: DR,
    ///0x04 - Independent Data register
    pub idr: IDR,
    ///0x08 - Control register
    pub cr: CR,
}
///DR (rw) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///Data register
pub mod dr;
///IDR (rw) register accessor: an alias for `Reg<IDR_SPEC>`
pub type IDR = crate::Reg<idr::IDR_SPEC>;
///Independent Data register
pub mod idr;
///CR (w) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Control register
pub mod cr;
