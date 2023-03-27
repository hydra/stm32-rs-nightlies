///Register block
#[repr(C)]
pub struct FLT {
    ///0x00 - control register 1
    pub cr1: CR1,
    ///0x04 - control register 2
    pub cr2: CR2,
    ///0x08 - interrupt and status register
    pub isr: ISR,
    ///0x0c - interrupt flag clear register
    pub icr: ICR,
    ///0x10 - injected channel group selection register
    pub jchgr: JCHGR,
    ///0x14 - filter control register
    pub fcr: FCR,
    ///0x18 - data register for injected group
    pub jdatar: JDATAR,
    ///0x1c - data register for the regular channel
    pub rdatar: RDATAR,
    ///0x20 - analog watchdog high threshold register
    pub awhtr: AWHTR,
    ///0x24 - analog watchdog low threshold register
    pub awltr: AWLTR,
    ///0x28 - analog watchdog status register
    pub awsr: AWSR,
    ///0x2c - analog watchdog clear flag register
    pub awcfr: AWCFR,
    ///0x30 - Extremes detector maximum register
    pub exmax: EXMAX,
    ///0x34 - Extremes detector minimum register
    pub exmin: EXMIN,
    ///0x38 - conversion timer register
    pub cnvtimr: CNVTIMR,
    _reserved_end: [u8; 0x44],
}
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///control register 1
pub mod cr1;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///control register 2
pub mod cr2;
///ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///interrupt and status register
pub mod isr;
///ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///interrupt flag clear register
pub mod icr;
///JCHGR (rw) register accessor: an alias for `Reg<JCHGR_SPEC>`
pub type JCHGR = crate::Reg<jchgr::JCHGR_SPEC>;
///injected channel group selection register
pub mod jchgr;
///FCR (rw) register accessor: an alias for `Reg<FCR_SPEC>`
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
///filter control register
pub mod fcr;
///JDATAR (r) register accessor: an alias for `Reg<JDATAR_SPEC>`
pub type JDATAR = crate::Reg<jdatar::JDATAR_SPEC>;
///data register for injected group
pub mod jdatar;
///RDATAR (r) register accessor: an alias for `Reg<RDATAR_SPEC>`
pub type RDATAR = crate::Reg<rdatar::RDATAR_SPEC>;
///data register for the regular channel
pub mod rdatar;
///AWHTR (rw) register accessor: an alias for `Reg<AWHTR_SPEC>`
pub type AWHTR = crate::Reg<awhtr::AWHTR_SPEC>;
///analog watchdog high threshold register
pub mod awhtr;
///AWLTR (rw) register accessor: an alias for `Reg<AWLTR_SPEC>`
pub type AWLTR = crate::Reg<awltr::AWLTR_SPEC>;
///analog watchdog low threshold register
pub mod awltr;
///AWSR (r) register accessor: an alias for `Reg<AWSR_SPEC>`
pub type AWSR = crate::Reg<awsr::AWSR_SPEC>;
///analog watchdog status register
pub mod awsr;
///AWCFR (rw) register accessor: an alias for `Reg<AWCFR_SPEC>`
pub type AWCFR = crate::Reg<awcfr::AWCFR_SPEC>;
///analog watchdog clear flag register
pub mod awcfr;
///EXMAX (r) register accessor: an alias for `Reg<EXMAX_SPEC>`
pub type EXMAX = crate::Reg<exmax::EXMAX_SPEC>;
///Extremes detector maximum register
pub mod exmax;
///EXMIN (r) register accessor: an alias for `Reg<EXMIN_SPEC>`
pub type EXMIN = crate::Reg<exmin::EXMIN_SPEC>;
///Extremes detector minimum register
pub mod exmin;
///CNVTIMR (r) register accessor: an alias for `Reg<CNVTIMR_SPEC>`
pub type CNVTIMR = crate::Reg<cnvtimr::CNVTIMR_SPEC>;
///conversion timer register
pub mod cnvtimr;
