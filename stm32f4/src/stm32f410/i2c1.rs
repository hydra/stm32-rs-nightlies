///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Control register 1
    pub cr1: CR1,
    ///0x04 - Control register 2
    pub cr2: CR2,
    ///0x08 - Own address register 1
    pub oar1: OAR1,
    ///0x0c - Own address register 2
    pub oar2: OAR2,
    ///0x10 - Data register
    pub dr: DR,
    ///0x14 - Status register 1
    pub sr1: SR1,
    ///0x18 - Status register 2
    pub sr2: SR2,
    ///0x1c - Clock control register
    pub ccr: CCR,
    ///0x20 - TRISE register
    pub trise: TRISE,
    ///0x24 - FLTR register
    pub fltr: FLTR,
}
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///Control register 1
pub mod cr1;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///Control register 2
pub mod cr2;
///OAR1 (rw) register accessor: an alias for `Reg<OAR1_SPEC>`
pub type OAR1 = crate::Reg<oar1::OAR1_SPEC>;
///Own address register 1
pub mod oar1;
///OAR2 (rw) register accessor: an alias for `Reg<OAR2_SPEC>`
pub type OAR2 = crate::Reg<oar2::OAR2_SPEC>;
///Own address register 2
pub mod oar2;
///DR (rw) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///Data register
pub mod dr;
///SR1 (rw) register accessor: an alias for `Reg<SR1_SPEC>`
pub type SR1 = crate::Reg<sr1::SR1_SPEC>;
///Status register 1
pub mod sr1;
///SR2 (r) register accessor: an alias for `Reg<SR2_SPEC>`
pub type SR2 = crate::Reg<sr2::SR2_SPEC>;
///Status register 2
pub mod sr2;
///CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
///Clock control register
pub mod ccr;
///TRISE (rw) register accessor: an alias for `Reg<TRISE_SPEC>`
pub type TRISE = crate::Reg<trise::TRISE_SPEC>;
///TRISE register
pub mod trise;
///FLTR (rw) register accessor: an alias for `Reg<FLTR_SPEC>`
pub type FLTR = crate::Reg<fltr::FLTR_SPEC>;
///FLTR register
pub mod fltr;
