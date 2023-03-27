///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TZIC interrupt enable register 1
    pub ier1: IER1,
    ///0x04 - TZIC interrupt enable register 2
    pub ier2: IER2,
    ///0x08 - TZIC interrupt enable register 3
    pub ier3: IER3,
    ///0x0c - TZIC interrupt enable register 4
    pub ier4: IER4,
    ///0x10 - TZIC status register 1
    pub sr1: SR1,
    ///0x14 - TZIC status register 2
    pub sr2: SR2,
    ///0x18 - TZIC status register 3
    pub sr3: SR3,
    ///0x1c - TZIC status register 4
    pub sr4: SR4,
    ///0x20 - TZIC flag clear register 1
    pub fcr1: FCR1,
    ///0x24 - TZIC flag clear register 2
    pub fcr2: FCR2,
    ///0x28 - TZIC flag clear register 3
    pub fcr3: FCR3,
    ///0x2c - TZIC flag clear register 3
    pub fcr4: FCR4,
}
///IER1 (rw) register accessor: an alias for `Reg<IER1_SPEC>`
pub type IER1 = crate::Reg<ier1::IER1_SPEC>;
///TZIC interrupt enable register 1
pub mod ier1;
///IER2 (rw) register accessor: an alias for `Reg<IER2_SPEC>`
pub type IER2 = crate::Reg<ier2::IER2_SPEC>;
///TZIC interrupt enable register 2
pub mod ier2;
///IER3 (rw) register accessor: an alias for `Reg<IER3_SPEC>`
pub type IER3 = crate::Reg<ier3::IER3_SPEC>;
///TZIC interrupt enable register 3
pub mod ier3;
///IER4 (rw) register accessor: an alias for `Reg<IER4_SPEC>`
pub type IER4 = crate::Reg<ier4::IER4_SPEC>;
///TZIC interrupt enable register 4
pub mod ier4;
///SR1 (r) register accessor: an alias for `Reg<SR1_SPEC>`
pub type SR1 = crate::Reg<sr1::SR1_SPEC>;
///TZIC status register 1
pub mod sr1;
///SR2 (r) register accessor: an alias for `Reg<SR2_SPEC>`
pub type SR2 = crate::Reg<sr2::SR2_SPEC>;
///TZIC status register 2
pub mod sr2;
///SR3 (r) register accessor: an alias for `Reg<SR3_SPEC>`
pub type SR3 = crate::Reg<sr3::SR3_SPEC>;
///TZIC status register 3
pub mod sr3;
///SR4 (r) register accessor: an alias for `Reg<SR4_SPEC>`
pub type SR4 = crate::Reg<sr4::SR4_SPEC>;
///TZIC status register 4
pub mod sr4;
///FCR1 (w) register accessor: an alias for `Reg<FCR1_SPEC>`
pub type FCR1 = crate::Reg<fcr1::FCR1_SPEC>;
///TZIC flag clear register 1
pub mod fcr1;
///FCR2 (w) register accessor: an alias for `Reg<FCR2_SPEC>`
pub type FCR2 = crate::Reg<fcr2::FCR2_SPEC>;
///TZIC flag clear register 2
pub mod fcr2;
///FCR3 (w) register accessor: an alias for `Reg<FCR3_SPEC>`
pub type FCR3 = crate::Reg<fcr3::FCR3_SPEC>;
///TZIC flag clear register 3
pub mod fcr3;
///FCR4 (w) register accessor: an alias for `Reg<FCR4_SPEC>`
pub type FCR4 = crate::Reg<fcr4::FCR4_SPEC>;
///TZIC flag clear register 3
pub mod fcr4;
