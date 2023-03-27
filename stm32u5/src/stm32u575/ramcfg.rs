///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - RAMCFG SRAM x control register
    pub ram1cr: RAM1CR,
    _reserved1: [u8; 0x04],
    ///0x08 - RAMCFG RAMx interrupt status register
    pub ram1isr: RAM1ISR,
    _reserved2: [u8; 0x1c],
    ///0x28 - RAMCFG SRAM x erase key register
    pub ram1erkeyr: RAM1ERKEYR,
    _reserved3: [u8; 0x14],
    ///0x40 - RAMCFG SRAM x control register
    pub ram2cr: RAM2CR,
    ///0x44 - RAMCFG SRAM x interrupt enable register
    pub ram2ier: RAM2IER,
    ///0x48 - RAMCFG RAMx interrupt status register
    pub ram2isr: RAM2ISR,
    ///0x4c - RAMCFG RAM x ECC single error address register
    pub ram2sear: RAM2SEAR,
    ///0x50 - RAMCFG RAM x ECC double error address register
    pub ram2dear: RAM2DEAR,
    ///0x54 - RAMCFG RAM x interrupt clear register x
    pub ram2icr: RAM2ICR,
    ///0x58 - RAMCFG SRAM2 write protection register 1
    pub ram2wpr1: RAM2WPR1,
    ///0x5c - RAMCFG SRAM2 write protection register 2
    pub ram2wpr2: RAM2WPR2,
    _reserved11: [u8; 0x04],
    ///0x64 - RAMCFG SRAM x ECC key register
    pub ram2ecckeyr: RAM2ECCKEYR,
    ///0x68 - RAMCFG SRAM x erase key register
    pub ram2erkeyr: RAM2ERKEYR,
    _reserved13: [u8; 0x14],
    ///0x80 - RAMCFG SRAM x control register
    pub ram3cr: RAM3CR,
    ///0x84 - RAMCFG SRAM x interrupt enable register
    pub ram3ier: RAM3IER,
    ///0x88 - RAMCFG RAMx interrupt status register
    pub ram3isr: RAM3ISR,
    ///0x8c - RAMCFG RAM x ECC single error address register
    pub ram3sear: RAM3SEAR,
    ///0x90 - RAMCFG RAM x ECC double error address register
    pub ram3dear: RAM3DEAR,
    ///0x94 - RAMCFG RAM x interrupt clear register x
    pub ram3icr: RAM3ICR,
    _reserved19: [u8; 0x0c],
    ///0xa4 - RAMCFG SRAM x ECC key register
    pub ram3ecckeyr: RAM3ECCKEYR,
    ///0xa8 - RAMCFG SRAM x erase key register
    pub ram3erkeyr: RAM3ERKEYR,
    _reserved21: [u8; 0x14],
    ///0xc0 - RAMCFG SRAM x control register
    pub ram4cr: RAM4CR,
    _reserved22: [u8; 0x04],
    ///0xc8 - RAMCFG RAMx interrupt status register
    pub ram4isr: RAM4ISR,
    _reserved23: [u8; 0x1c],
    ///0xe8 - RAMCFG SRAM x erase key register
    pub ram4erkeyr: RAM4ERKEYR,
    _reserved24: [u8; 0x14],
    ///0x100 - RAMCFG SRAM x control register
    pub ram5cr: RAM5CR,
    ///0x104 - RAMCFG SRAM x interrupt enable register
    pub ram5ier: RAM5IER,
    ///0x108 - RAMCFG RAMx interrupt status register
    pub ram5isr: RAM5ISR,
    ///0x10c - RAMCFG RAM x ECC single error address register
    pub ram5sear: RAM5SEAR,
    ///0x110 - RAMCFG RAM x ECC double error address register
    pub ram5dear: RAM5DEAR,
    ///0x114 - RAMCFG RAM x interrupt clear register x
    pub ram5icr: RAM5ICR,
}
///RAM1CR (rw) register accessor: an alias for `Reg<RAM1CR_SPEC>`
pub type RAM1CR = crate::Reg<ram1cr::RAM1CR_SPEC>;
///RAMCFG SRAM x control register
pub mod ram1cr;
///RAM1ISR (r) register accessor: an alias for `Reg<RAM1ISR_SPEC>`
pub type RAM1ISR = crate::Reg<ram1isr::RAM1ISR_SPEC>;
///RAMCFG RAMx interrupt status register
pub mod ram1isr;
///RAM1ERKEYR (w) register accessor: an alias for `Reg<RAM1ERKEYR_SPEC>`
pub type RAM1ERKEYR = crate::Reg<ram1erkeyr::RAM1ERKEYR_SPEC>;
///RAMCFG SRAM x erase key register
pub mod ram1erkeyr;
///RAM2CR (rw) register accessor: an alias for `Reg<RAM2CR_SPEC>`
pub type RAM2CR = crate::Reg<ram2cr::RAM2CR_SPEC>;
///RAMCFG SRAM x control register
pub mod ram2cr;
///RAM2IER (rw) register accessor: an alias for `Reg<RAM2IER_SPEC>`
pub type RAM2IER = crate::Reg<ram2ier::RAM2IER_SPEC>;
///RAMCFG SRAM x interrupt enable register
pub mod ram2ier;
///RAM2ISR (r) register accessor: an alias for `Reg<RAM2ISR_SPEC>`
pub type RAM2ISR = crate::Reg<ram2isr::RAM2ISR_SPEC>;
///RAMCFG RAMx interrupt status register
pub mod ram2isr;
///RAM2SEAR (r) register accessor: an alias for `Reg<RAM2SEAR_SPEC>`
pub type RAM2SEAR = crate::Reg<ram2sear::RAM2SEAR_SPEC>;
///RAMCFG RAM x ECC single error address register
pub mod ram2sear;
///RAM2DEAR (r) register accessor: an alias for `Reg<RAM2DEAR_SPEC>`
pub type RAM2DEAR = crate::Reg<ram2dear::RAM2DEAR_SPEC>;
///RAMCFG RAM x ECC double error address register
pub mod ram2dear;
///RAM2ICR (rw) register accessor: an alias for `Reg<RAM2ICR_SPEC>`
pub type RAM2ICR = crate::Reg<ram2icr::RAM2ICR_SPEC>;
///RAMCFG RAM x interrupt clear register x
pub mod ram2icr;
///RAM2WPR1 (rw) register accessor: an alias for `Reg<RAM2WPR1_SPEC>`
pub type RAM2WPR1 = crate::Reg<ram2wpr1::RAM2WPR1_SPEC>;
///RAMCFG SRAM2 write protection register 1
pub mod ram2wpr1;
///RAM2WPR2 (rw) register accessor: an alias for `Reg<RAM2WPR2_SPEC>`
pub type RAM2WPR2 = crate::Reg<ram2wpr2::RAM2WPR2_SPEC>;
///RAMCFG SRAM2 write protection register 2
pub mod ram2wpr2;
///RAM2ECCKEYR (w) register accessor: an alias for `Reg<RAM2ECCKEYR_SPEC>`
pub type RAM2ECCKEYR = crate::Reg<ram2ecckeyr::RAM2ECCKEYR_SPEC>;
///RAMCFG SRAM x ECC key register
pub mod ram2ecckeyr;
///RAM2ERKEYR (w) register accessor: an alias for `Reg<RAM2ERKEYR_SPEC>`
pub type RAM2ERKEYR = crate::Reg<ram2erkeyr::RAM2ERKEYR_SPEC>;
///RAMCFG SRAM x erase key register
pub mod ram2erkeyr;
///RAM3CR (rw) register accessor: an alias for `Reg<RAM3CR_SPEC>`
pub type RAM3CR = crate::Reg<ram3cr::RAM3CR_SPEC>;
///RAMCFG SRAM x control register
pub mod ram3cr;
///RAM3IER (rw) register accessor: an alias for `Reg<RAM3IER_SPEC>`
pub type RAM3IER = crate::Reg<ram3ier::RAM3IER_SPEC>;
///RAMCFG SRAM x interrupt enable register
pub mod ram3ier;
///RAM3ISR (r) register accessor: an alias for `Reg<RAM3ISR_SPEC>`
pub type RAM3ISR = crate::Reg<ram3isr::RAM3ISR_SPEC>;
///RAMCFG RAMx interrupt status register
pub mod ram3isr;
///RAM3SEAR (r) register accessor: an alias for `Reg<RAM3SEAR_SPEC>`
pub type RAM3SEAR = crate::Reg<ram3sear::RAM3SEAR_SPEC>;
///RAMCFG RAM x ECC single error address register
pub mod ram3sear;
///RAM3DEAR (r) register accessor: an alias for `Reg<RAM3DEAR_SPEC>`
pub type RAM3DEAR = crate::Reg<ram3dear::RAM3DEAR_SPEC>;
///RAMCFG RAM x ECC double error address register
pub mod ram3dear;
///RAM3ICR (rw) register accessor: an alias for `Reg<RAM3ICR_SPEC>`
pub type RAM3ICR = crate::Reg<ram3icr::RAM3ICR_SPEC>;
///RAMCFG RAM x interrupt clear register x
pub mod ram3icr;
///RAM3ECCKEYR (w) register accessor: an alias for `Reg<RAM3ECCKEYR_SPEC>`
pub type RAM3ECCKEYR = crate::Reg<ram3ecckeyr::RAM3ECCKEYR_SPEC>;
///RAMCFG SRAM x ECC key register
pub mod ram3ecckeyr;
///RAM3ERKEYR (w) register accessor: an alias for `Reg<RAM3ERKEYR_SPEC>`
pub type RAM3ERKEYR = crate::Reg<ram3erkeyr::RAM3ERKEYR_SPEC>;
///RAMCFG SRAM x erase key register
pub mod ram3erkeyr;
///RAM4CR (rw) register accessor: an alias for `Reg<RAM4CR_SPEC>`
pub type RAM4CR = crate::Reg<ram4cr::RAM4CR_SPEC>;
///RAMCFG SRAM x control register
pub mod ram4cr;
///RAM4ISR (r) register accessor: an alias for `Reg<RAM4ISR_SPEC>`
pub type RAM4ISR = crate::Reg<ram4isr::RAM4ISR_SPEC>;
///RAMCFG RAMx interrupt status register
pub mod ram4isr;
///RAM4ERKEYR (w) register accessor: an alias for `Reg<RAM4ERKEYR_SPEC>`
pub type RAM4ERKEYR = crate::Reg<ram4erkeyr::RAM4ERKEYR_SPEC>;
///RAMCFG SRAM x erase key register
pub mod ram4erkeyr;
///RAM5CR (rw) register accessor: an alias for `Reg<RAM5CR_SPEC>`
pub type RAM5CR = crate::Reg<ram5cr::RAM5CR_SPEC>;
///RAMCFG SRAM x control register
pub mod ram5cr;
///RAM5IER (rw) register accessor: an alias for `Reg<RAM5IER_SPEC>`
pub type RAM5IER = crate::Reg<ram5ier::RAM5IER_SPEC>;
///RAMCFG SRAM x interrupt enable register
pub mod ram5ier;
///RAM5ISR (r) register accessor: an alias for `Reg<RAM5ISR_SPEC>`
pub type RAM5ISR = crate::Reg<ram5isr::RAM5ISR_SPEC>;
///RAMCFG RAMx interrupt status register
pub mod ram5isr;
///RAM5SEAR (r) register accessor: an alias for `Reg<RAM5SEAR_SPEC>`
pub type RAM5SEAR = crate::Reg<ram5sear::RAM5SEAR_SPEC>;
///RAMCFG RAM x ECC single error address register
pub mod ram5sear;
///RAM5DEAR (r) register accessor: an alias for `Reg<RAM5DEAR_SPEC>`
pub type RAM5DEAR = crate::Reg<ram5dear::RAM5DEAR_SPEC>;
///RAMCFG RAM x ECC double error address register
pub mod ram5dear;
///RAM5ICR (rw) register accessor: an alias for `Reg<RAM5ICR_SPEC>`
pub type RAM5ICR = crate::Reg<ram5icr::RAM5ICR_SPEC>;
///RAMCFG RAM x interrupt clear register x
pub mod ram5icr;
