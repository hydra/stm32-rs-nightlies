///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - TZIC interrupt enable register 1
    pub ier1: IER1,
    ///0x04 - TZIC interrupt enable register 2
    pub ier2: IER2,
    ///0x08 - TZIC interrupt enable register 3
    pub ier3: IER3,
    _reserved3: [u8; 0x04],
    ///0x10 - TZIC interrupt status register 1
    pub sr1: SR1,
    ///0x14 - TZIC interrupt status register 2
    pub sr2: SR2,
    ///0x18 - TZIC interrupt status register 3
    pub sr3: SR3,
    _reserved6: [u8; 0x04],
    ///0x20 - TZIC interrupt clear register 1
    pub fcr1: FCR1,
    ///0x24 - TZIC interrupt clear register 2
    pub fcr2: FCR2,
    ///0x28 - TZIC interrupt clear register 3
    pub fcr3: FCR3,
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
///SR1 (r) register accessor: an alias for `Reg<SR1_SPEC>`
pub type SR1 = crate::Reg<sr1::SR1_SPEC>;
///TZIC interrupt status register 1
pub mod sr1;
///SR2 (rw) register accessor: an alias for `Reg<SR2_SPEC>`
pub type SR2 = crate::Reg<sr2::SR2_SPEC>;
///TZIC interrupt status register 2
pub mod sr2;
///SR3 (rw) register accessor: an alias for `Reg<SR3_SPEC>`
pub type SR3 = crate::Reg<sr3::SR3_SPEC>;
///TZIC interrupt status register 3
pub mod sr3;
///FCR1 (w) register accessor: an alias for `Reg<FCR1_SPEC>`
pub type FCR1 = crate::Reg<fcr1::FCR1_SPEC>;
///TZIC interrupt clear register 1
pub mod fcr1;
///FCR2 (rw) register accessor: an alias for `Reg<FCR2_SPEC>`
pub type FCR2 = crate::Reg<fcr2::FCR2_SPEC>;
///TZIC interrupt clear register 2
pub mod fcr2;
///FCR3 (rw) register accessor: an alias for `Reg<FCR3_SPEC>`
pub type FCR3 = crate::Reg<fcr3::FCR3_SPEC>;
///TZIC interrupt clear register 3
pub mod fcr3;
