///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - UART4_SR
    pub sr: SR,
    ///0x04 - UART4_DR
    pub dr: DR,
    ///0x08 - UART4_BRR
    pub brr: BRR,
    ///0x0c - UART4_CR1
    pub cr1: CR1,
    ///0x10 - UART4_CR2
    pub cr2: CR2,
    ///0x14 - UART4_CR3
    pub cr3: CR3,
}
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///UART4_SR
pub mod sr;
///DR (rw) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///UART4_DR
pub mod dr;
///BRR (rw) register accessor: an alias for `Reg<BRR_SPEC>`
pub type BRR = crate::Reg<brr::BRR_SPEC>;
///UART4_BRR
pub mod brr;
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///UART4_CR1
pub mod cr1;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///UART4_CR2
pub mod cr2;
///CR3 (rw) register accessor: an alias for `Reg<CR3_SPEC>`
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
///UART4_CR3
pub mod cr3;
