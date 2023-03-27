///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Status register
    pub sr: SR,
    ///0x04 - Data register
    pub dr: DR,
    ///0x08 - Baud rate register
    pub brr: BRR,
    ///0x0c - Control register 1
    pub cr1: CR1,
    ///0x10 - Control register 2
    pub cr2: CR2,
    ///0x14 - Control register 3
    pub cr3: CR3,
    ///0x18 - Guard time and prescaler register
    pub gtpr: GTPR,
}
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///Status register
pub mod sr;
///DR (rw) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///Data register
pub mod dr;
///BRR (rw) register accessor: an alias for `Reg<BRR_SPEC>`
pub type BRR = crate::Reg<brr::BRR_SPEC>;
///Baud rate register
pub mod brr;
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///Control register 1
pub mod cr1;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///Control register 2
pub mod cr2;
///CR3 (rw) register accessor: an alias for `Reg<CR3_SPEC>`
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
///Control register 3
pub mod cr3;
///GTPR (rw) register accessor: an alias for `Reg<GTPR_SPEC>`
pub type GTPR = crate::Reg<gtpr::GTPR_SPEC>;
///Guard time and prescaler register
pub mod gtpr;
