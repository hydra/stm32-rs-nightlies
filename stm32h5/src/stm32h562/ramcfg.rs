///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - RAMCFG memory 1 control register
    pub m1cr: M1CR,
    _reserved1: [u8; 0x04],
    ///0x08 - RAMCFG memory interrupt status register
    pub m1isr: M1ISR,
    _reserved2: [u8; 0x1c],
    ///0x28 - RAMCFG memory 1 erase key register
    pub m1erkeyr: M1ERKEYR,
    _reserved3: [u8; 0x14],
    ///0x40 - RAMCFG memory 2 control register
    pub m2cr: M2CR,
    ///0x44 - RAMCFG memory 2 interrupt enable register
    pub m2ier: M2IER,
    ///0x48 - RAMCFG memory interrupt status register
    pub m2isr: M2ISR,
    ///0x4c - RAMCFG memory 2 ECC single error address register
    pub m2sear: M2SEAR,
    ///0x50 - RAMCFG memory 2 ECC double error address register
    pub m2dear: M2DEAR,
    ///0x54 - RAMCFG memory 2 interrupt clear register 2
    pub m2icr: M2ICR,
    ///0x58 - RAMCFG memory 2 write protection register 1
    pub m2wpr1: M2WPR1,
    ///0x5c - RAMCFG memory 2 write protection register 2
    pub m2wpr2: M2WPR2,
    _reserved11: [u8; 0x04],
    ///0x64 - RAMCFG memory 2 ECC key register
    pub m2ecckeyr: M2ECCKEYR,
    ///0x68 - RAMCFG memory 2 erase key register
    pub m2erkeyr: M2ERKEYR,
    _reserved13: [u8; 0x14],
    ///0x80 - RAMCFG memory 3 control register
    pub m3cr: M3CR,
    ///0x84 - RAMCFG memory 3 interrupt enable register
    pub m3ier: M3IER,
    ///0x88 - RAMCFG memory interrupt status register
    pub m3isr: M3ISR,
    ///0x8c - RAMCFG memory 3 ECC single error address register
    pub m3sear: M3SEAR,
    ///0x90 - RAMCFG memory 3 ECC double error address register
    pub m3dear: M3DEAR,
    ///0x94 - RAMCFG memory 3 interrupt clear register 3
    pub m3icr: M3ICR,
    _reserved19: [u8; 0x0c],
    ///0xa4 - RAMCFG memory 3 ECC key register
    pub m3ecckeyr: M3ECCKEYR,
    ///0xa8 - RAMCFG memory 3 erase key register
    pub m3erkeyr: M3ERKEYR,
    _reserved21: [u8; 0x3c],
    ///0xe8 - RAMCFG memory 4 erase key register
    pub m4erkeyr: M4ERKEYR,
    _reserved22: [u8; 0x14],
    ///0x100 - RAMCFG memory 5 control register
    pub m5cr: M5CR,
    ///0x104 - RAMCFG memory 5 interrupt enable register
    pub m5ier: M5IER,
    ///0x108 - RAMCFG memory interrupt status register
    pub m5isr: M5ISR,
    ///0x10c - RAMCFG memory 5 ECC single error address register
    pub m5sear: M5SEAR,
    ///0x110 - RAMCFG memory 5 ECC double error address register
    pub m5dear: M5DEAR,
    ///0x114 - RAMCFG memory 5 interrupt clear register 5
    pub m5icr: M5ICR,
    _reserved28: [u8; 0x0c],
    ///0x124 - RAMCFG memory 5 ECC key register
    pub m5ecckeyr: M5ECCKEYR,
    ///0x128 - RAMCFG memory 5 erase key register
    pub m5erkeyr: M5ERKEYR,
}
///M1CR (rw) register accessor: an alias for `Reg<M1CR_SPEC>`
pub type M1CR = crate::Reg<m1cr::M1CR_SPEC>;
///RAMCFG memory 1 control register
pub mod m1cr;
///M1ISR (r) register accessor: an alias for `Reg<M1ISR_SPEC>`
pub type M1ISR = crate::Reg<m1isr::M1ISR_SPEC>;
///RAMCFG memory interrupt status register
pub mod m1isr;
///M1ERKEYR (w) register accessor: an alias for `Reg<M1ERKEYR_SPEC>`
pub type M1ERKEYR = crate::Reg<m1erkeyr::M1ERKEYR_SPEC>;
///RAMCFG memory 1 erase key register
pub mod m1erkeyr;
///M2CR (rw) register accessor: an alias for `Reg<M2CR_SPEC>`
pub type M2CR = crate::Reg<m2cr::M2CR_SPEC>;
///RAMCFG memory 2 control register
pub mod m2cr;
///M2IER (rw) register accessor: an alias for `Reg<M2IER_SPEC>`
pub type M2IER = crate::Reg<m2ier::M2IER_SPEC>;
///RAMCFG memory 2 interrupt enable register
pub mod m2ier;
///M2ISR (r) register accessor: an alias for `Reg<M2ISR_SPEC>`
pub type M2ISR = crate::Reg<m2isr::M2ISR_SPEC>;
///RAMCFG memory interrupt status register
pub mod m2isr;
///M2SEAR (r) register accessor: an alias for `Reg<M2SEAR_SPEC>`
pub type M2SEAR = crate::Reg<m2sear::M2SEAR_SPEC>;
///RAMCFG memory 2 ECC single error address register
pub mod m2sear;
///M2DEAR (r) register accessor: an alias for `Reg<M2DEAR_SPEC>`
pub type M2DEAR = crate::Reg<m2dear::M2DEAR_SPEC>;
///RAMCFG memory 2 ECC double error address register
pub mod m2dear;
///M2ICR (rw) register accessor: an alias for `Reg<M2ICR_SPEC>`
pub type M2ICR = crate::Reg<m2icr::M2ICR_SPEC>;
///RAMCFG memory 2 interrupt clear register 2
pub mod m2icr;
///M2WPR1 (rw) register accessor: an alias for `Reg<M2WPR1_SPEC>`
pub type M2WPR1 = crate::Reg<m2wpr1::M2WPR1_SPEC>;
///RAMCFG memory 2 write protection register 1
pub mod m2wpr1;
///M2WPR2 (rw) register accessor: an alias for `Reg<M2WPR2_SPEC>`
pub type M2WPR2 = crate::Reg<m2wpr2::M2WPR2_SPEC>;
///RAMCFG memory 2 write protection register 2
pub mod m2wpr2;
///M2ECCKEYR (w) register accessor: an alias for `Reg<M2ECCKEYR_SPEC>`
pub type M2ECCKEYR = crate::Reg<m2ecckeyr::M2ECCKEYR_SPEC>;
///RAMCFG memory 2 ECC key register
pub mod m2ecckeyr;
///M2ERKEYR (w) register accessor: an alias for `Reg<M2ERKEYR_SPEC>`
pub type M2ERKEYR = crate::Reg<m2erkeyr::M2ERKEYR_SPEC>;
///RAMCFG memory 2 erase key register
pub mod m2erkeyr;
///M3CR (rw) register accessor: an alias for `Reg<M3CR_SPEC>`
pub type M3CR = crate::Reg<m3cr::M3CR_SPEC>;
///RAMCFG memory 3 control register
pub mod m3cr;
///M3IER (rw) register accessor: an alias for `Reg<M3IER_SPEC>`
pub type M3IER = crate::Reg<m3ier::M3IER_SPEC>;
///RAMCFG memory 3 interrupt enable register
pub mod m3ier;
///M3ISR (r) register accessor: an alias for `Reg<M3ISR_SPEC>`
pub type M3ISR = crate::Reg<m3isr::M3ISR_SPEC>;
///RAMCFG memory interrupt status register
pub mod m3isr;
///M3SEAR (r) register accessor: an alias for `Reg<M3SEAR_SPEC>`
pub type M3SEAR = crate::Reg<m3sear::M3SEAR_SPEC>;
///RAMCFG memory 3 ECC single error address register
pub mod m3sear;
///M3DEAR (r) register accessor: an alias for `Reg<M3DEAR_SPEC>`
pub type M3DEAR = crate::Reg<m3dear::M3DEAR_SPEC>;
///RAMCFG memory 3 ECC double error address register
pub mod m3dear;
///M3ICR (rw) register accessor: an alias for `Reg<M3ICR_SPEC>`
pub type M3ICR = crate::Reg<m3icr::M3ICR_SPEC>;
///RAMCFG memory 3 interrupt clear register 3
pub mod m3icr;
///M3ECCKEYR (w) register accessor: an alias for `Reg<M3ECCKEYR_SPEC>`
pub type M3ECCKEYR = crate::Reg<m3ecckeyr::M3ECCKEYR_SPEC>;
///RAMCFG memory 3 ECC key register
pub mod m3ecckeyr;
///M3ERKEYR (w) register accessor: an alias for `Reg<M3ERKEYR_SPEC>`
pub type M3ERKEYR = crate::Reg<m3erkeyr::M3ERKEYR_SPEC>;
///RAMCFG memory 3 erase key register
pub mod m3erkeyr;
///M4ERKEYR (w) register accessor: an alias for `Reg<M4ERKEYR_SPEC>`
pub type M4ERKEYR = crate::Reg<m4erkeyr::M4ERKEYR_SPEC>;
///RAMCFG memory 4 erase key register
pub mod m4erkeyr;
///M5CR (rw) register accessor: an alias for `Reg<M5CR_SPEC>`
pub type M5CR = crate::Reg<m5cr::M5CR_SPEC>;
///RAMCFG memory 5 control register
pub mod m5cr;
///M5IER (rw) register accessor: an alias for `Reg<M5IER_SPEC>`
pub type M5IER = crate::Reg<m5ier::M5IER_SPEC>;
///RAMCFG memory 5 interrupt enable register
pub mod m5ier;
///M5ISR (r) register accessor: an alias for `Reg<M5ISR_SPEC>`
pub type M5ISR = crate::Reg<m5isr::M5ISR_SPEC>;
///RAMCFG memory interrupt status register
pub mod m5isr;
///M5SEAR (r) register accessor: an alias for `Reg<M5SEAR_SPEC>`
pub type M5SEAR = crate::Reg<m5sear::M5SEAR_SPEC>;
///RAMCFG memory 5 ECC single error address register
pub mod m5sear;
///M5DEAR (r) register accessor: an alias for `Reg<M5DEAR_SPEC>`
pub type M5DEAR = crate::Reg<m5dear::M5DEAR_SPEC>;
///RAMCFG memory 5 ECC double error address register
pub mod m5dear;
///M5ICR (rw) register accessor: an alias for `Reg<M5ICR_SPEC>`
pub type M5ICR = crate::Reg<m5icr::M5ICR_SPEC>;
///RAMCFG memory 5 interrupt clear register 5
pub mod m5icr;
///M5ECCKEYR (w) register accessor: an alias for `Reg<M5ECCKEYR_SPEC>`
pub type M5ECCKEYR = crate::Reg<m5ecckeyr::M5ECCKEYR_SPEC>;
///RAMCFG memory 5 ECC key register
pub mod m5ecckeyr;
///M5ERKEYR (w) register accessor: an alias for `Reg<M5ERKEYR_SPEC>`
pub type M5ERKEYR = crate::Reg<m5erkeyr::M5ERKEYR_SPEC>;
///RAMCFG memory 5 erase key register
pub mod m5erkeyr;
