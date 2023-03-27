///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - ADC interrupt and status register
    pub isr: ISR,
    ///0x04 - ADC interrupt enable register
    pub ier: IER,
    ///0x08 - ADC control register
    pub cr: CR,
    ///0x0c - ADC configuration register 1
    pub cfgr1: CFGR1,
    ///0x10 - ADC configuration register 2
    pub cfgr2: CFGR2,
    ///0x14 - ADC sampling time register
    pub smpr: SMPR,
    _reserved6: [u8; 0x08],
    ///0x20 - watchdog threshold register
    pub awd1tr: AWD1TR,
    ///0x24 - watchdog threshold register
    pub awd2tr: AWD2TR,
    _reserved_8_chselr0: [u8; 0x04],
    ///0x2c - watchdog threshold register
    pub awd3tr: AWD3TR,
    _reserved10: [u8; 0x10],
    ///0x40 - ADC group regular conversion data register
    pub dr: DR,
    _reserved11: [u8; 0x5c],
    ///0xa0 - ADC analog watchdog 2 configuration register
    pub awd2cr: AWD2CR,
    ///0xa4 - ADC analog watchdog 3 configuration register
    pub awd3cr: AWD3CR,
    _reserved13: [u8; 0x0c],
    ///0xb4 - ADC calibration factors register
    pub calfact: CALFACT,
    _reserved14: [u8; 0x0250],
    ///0x308 - ADC common control register
    pub ccr: CCR,
    _reserved15: [u8; 0xcc],
    ///0x3d8 - Hardware Configuration Register
    pub hwcfgr6: HWCFGR6,
    ///0x3dc - Hardware Configuration Register
    pub hwcfgr5: HWCFGR5,
    ///0x3e0 - Hardware Configuration Register
    pub hwcfgr4: HWCFGR4,
    ///0x3e4 - Hardware Configuration Register
    pub hwcfgr3: HWCFGR3,
    ///0x3e8 - Hardware Configuration Register
    pub hwcfgr2: HWCFGR2,
    ///0x3ec - Hardware Configuration Register
    pub hwcfgr1: HWCFGR1,
    ///0x3f0 - Hardware Configuration Register
    pub hwcfgr0: HWCFGR0,
    ///0x3f4 - EXTI IP Version register
    pub verr: VERR,
    ///0x3f8 - EXTI Identification register
    pub ipidr: IPIDR,
    ///0x3fc - EXTI Size ID register
    pub sidr: SIDR,
}
impl RegisterBlock {
    ///0x28 - channel selection register CHSELRMOD = 1 in ADC_CFGR1
    #[inline(always)]
    pub const fn chselr1(&self) -> &CHSELR1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    ///0x28 - channel selection register
    #[inline(always)]
    pub const fn chselr0(&self) -> &CHSELR0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
}
///ISR (rw) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///ADC interrupt and status register
pub mod isr;
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///ADC interrupt enable register
pub mod ier;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///ADC control register
pub mod cr;
///CFGR1 (rw) register accessor: an alias for `Reg<CFGR1_SPEC>`
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
///ADC configuration register 1
pub mod cfgr1;
///CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
///ADC configuration register 2
pub mod cfgr2;
///SMPR (rw) register accessor: an alias for `Reg<SMPR_SPEC>`
pub type SMPR = crate::Reg<smpr::SMPR_SPEC>;
///ADC sampling time register
pub mod smpr;
///AWD1TR (rw) register accessor: an alias for `Reg<AWD1TR_SPEC>`
pub type AWD1TR = crate::Reg<awd1tr::AWD1TR_SPEC>;
///watchdog threshold register
pub mod awd1tr;
///AWD2TR (rw) register accessor: an alias for `Reg<AWD2TR_SPEC>`
pub type AWD2TR = crate::Reg<awd2tr::AWD2TR_SPEC>;
///watchdog threshold register
pub mod awd2tr;
///CHSELR0 (rw) register accessor: an alias for `Reg<CHSELR0_SPEC>`
pub type CHSELR0 = crate::Reg<chselr0::CHSELR0_SPEC>;
///channel selection register
pub mod chselr0;
///CHSELR1 (rw) register accessor: an alias for `Reg<CHSELR1_SPEC>`
pub type CHSELR1 = crate::Reg<chselr1::CHSELR1_SPEC>;
///channel selection register CHSELRMOD = 1 in ADC_CFGR1
pub mod chselr1;
///AWD3TR (rw) register accessor: an alias for `Reg<AWD3TR_SPEC>`
pub type AWD3TR = crate::Reg<awd3tr::AWD3TR_SPEC>;
///watchdog threshold register
pub mod awd3tr;
///DR (r) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///ADC group regular conversion data register
pub mod dr;
///AWD2CR (rw) register accessor: an alias for `Reg<AWD2CR_SPEC>`
pub type AWD2CR = crate::Reg<awd2cr::AWD2CR_SPEC>;
///ADC analog watchdog 2 configuration register
pub mod awd2cr;
///AWD3CR (rw) register accessor: an alias for `Reg<AWD3CR_SPEC>`
pub type AWD3CR = crate::Reg<awd3cr::AWD3CR_SPEC>;
///ADC analog watchdog 3 configuration register
pub mod awd3cr;
///CALFACT (rw) register accessor: an alias for `Reg<CALFACT_SPEC>`
pub type CALFACT = crate::Reg<calfact::CALFACT_SPEC>;
///ADC calibration factors register
pub mod calfact;
///CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
///ADC common control register
pub mod ccr;
///HWCFGR6 (rw) register accessor: an alias for `Reg<HWCFGR6_SPEC>`
pub type HWCFGR6 = crate::Reg<hwcfgr6::HWCFGR6_SPEC>;
///Hardware Configuration Register
pub mod hwcfgr6;
///HWCFGR5 (rw) register accessor: an alias for `Reg<HWCFGR5_SPEC>`
pub type HWCFGR5 = crate::Reg<hwcfgr5::HWCFGR5_SPEC>;
///Hardware Configuration Register
pub mod hwcfgr5;
///HWCFGR4 (rw) register accessor: an alias for `Reg<HWCFGR4_SPEC>`
pub type HWCFGR4 = crate::Reg<hwcfgr4::HWCFGR4_SPEC>;
///Hardware Configuration Register
pub mod hwcfgr4;
///HWCFGR3 (rw) register accessor: an alias for `Reg<HWCFGR3_SPEC>`
pub type HWCFGR3 = crate::Reg<hwcfgr3::HWCFGR3_SPEC>;
///Hardware Configuration Register
pub mod hwcfgr3;
///HWCFGR2 (rw) register accessor: an alias for `Reg<HWCFGR2_SPEC>`
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2_SPEC>;
///Hardware Configuration Register
pub mod hwcfgr2;
///HWCFGR1 (rw) register accessor: an alias for `Reg<HWCFGR1_SPEC>`
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1_SPEC>;
///Hardware Configuration Register
pub mod hwcfgr1;
///HWCFGR0 (r) register accessor: an alias for `Reg<HWCFGR0_SPEC>`
pub type HWCFGR0 = crate::Reg<hwcfgr0::HWCFGR0_SPEC>;
///Hardware Configuration Register
pub mod hwcfgr0;
///VERR (r) register accessor: an alias for `Reg<VERR_SPEC>`
pub type VERR = crate::Reg<verr::VERR_SPEC>;
///EXTI IP Version register
pub mod verr;
///IPIDR (r) register accessor: an alias for `Reg<IPIDR_SPEC>`
pub type IPIDR = crate::Reg<ipidr::IPIDR_SPEC>;
///EXTI Identification register
pub mod ipidr;
///SIDR (r) register accessor: an alias for `Reg<SIDR_SPEC>`
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
///EXTI Size ID register
pub mod sidr;
