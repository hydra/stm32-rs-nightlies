///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - ADC interrupt and status register
    pub adc_isr: ADC_ISR,
    ///0x04 - ADC interrupt enable register
    pub adc_ier: ADC_IER,
    ///0x08 - ADC control register
    pub adc_cr: ADC_CR,
    ///0x0c - ADC configuration register
    pub adc_cfgr1: ADC_CFGR1,
    ///0x10 - ADC configuration register 2
    pub adc_cfgr2: ADC_CFGR2,
    ///0x14 - ADC sample time register
    pub adc_smpr: ADC_SMPR,
    _reserved6: [u8; 0x08],
    ///0x20 - ADC watchdog threshold register
    pub adc_awd1tr: ADC_AWD1TR,
    ///0x24 - ADC watchdog threshold register
    pub adc_awd2tr: ADC_AWD2TR,
    _reserved_8_adc_chselrmod: [u8; 0x04],
    ///0x2c - ADC watchdog threshold register
    pub adc_awd3tr: ADC_AWD3TR,
    _reserved10: [u8; 0x10],
    ///0x40 - ADC data register
    pub adc_dr: ADC_DR,
    ///0x44 - ADC data register
    pub adc_pwr: ADC_PWR,
    _reserved12: [u8; 0x58],
    ///0xa0 - ADC Analog Watchdog 2 Configuration register
    pub adc_awd2cr: ADC_AWD2CR,
    ///0xa4 - ADC Analog Watchdog 3 Configuration register
    pub adc_awd3cr: ADC_AWD3CR,
    _reserved14: [u8; 0x1c],
    ///0xc4 - ADC Calibration factor
    pub adc_calfact: ADC_CALFACT,
    _reserved15: [u8; 0x08],
    ///0xd0 - ADC option register
    pub adc_or: ADC_OR,
    _reserved16: [u8; 0x0234],
    ///0x308 - ADC common configuration register
    pub adc_ccr: ADC_CCR,
}
impl RegisterBlock {
    ///0x28 - ADC channel selection register \[alternate\]
    #[inline(always)]
    pub const fn adc_chselrmod1(&self) -> &ADC_CHSELRMOD1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    ///0x28 - ADC channel selection register \[alternate\]
    #[inline(always)]
    pub const fn adc_chselrmod0(&self) -> &ADC_CHSELRMOD0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
}
///ADC_ISR (rw) register accessor: an alias for `Reg<ADC_ISR_SPEC>`
pub type ADC_ISR = crate::Reg<adc_isr::ADC_ISR_SPEC>;
///ADC interrupt and status register
pub mod adc_isr;
///ADC_IER (rw) register accessor: an alias for `Reg<ADC_IER_SPEC>`
pub type ADC_IER = crate::Reg<adc_ier::ADC_IER_SPEC>;
///ADC interrupt enable register
pub mod adc_ier;
///ADC_CR (rw) register accessor: an alias for `Reg<ADC_CR_SPEC>`
pub type ADC_CR = crate::Reg<adc_cr::ADC_CR_SPEC>;
///ADC control register
pub mod adc_cr;
///ADC_CFGR1 (rw) register accessor: an alias for `Reg<ADC_CFGR1_SPEC>`
pub type ADC_CFGR1 = crate::Reg<adc_cfgr1::ADC_CFGR1_SPEC>;
///ADC configuration register
pub mod adc_cfgr1;
///ADC_CFGR2 (rw) register accessor: an alias for `Reg<ADC_CFGR2_SPEC>`
pub type ADC_CFGR2 = crate::Reg<adc_cfgr2::ADC_CFGR2_SPEC>;
///ADC configuration register 2
pub mod adc_cfgr2;
///ADC_SMPR (rw) register accessor: an alias for `Reg<ADC_SMPR_SPEC>`
pub type ADC_SMPR = crate::Reg<adc_smpr::ADC_SMPR_SPEC>;
///ADC sample time register
pub mod adc_smpr;
///ADC_AWD1TR (rw) register accessor: an alias for `Reg<ADC_AWD1TR_SPEC>`
pub type ADC_AWD1TR = crate::Reg<adc_awd1tr::ADC_AWD1TR_SPEC>;
///ADC watchdog threshold register
pub mod adc_awd1tr;
///ADC_AWD2TR (rw) register accessor: an alias for `Reg<ADC_AWD2TR_SPEC>`
pub type ADC_AWD2TR = crate::Reg<adc_awd2tr::ADC_AWD2TR_SPEC>;
///ADC watchdog threshold register
pub mod adc_awd2tr;
///ADC_CHSELRMOD0 (rw) register accessor: an alias for `Reg<ADC_CHSELRMOD0_SPEC>`
pub type ADC_CHSELRMOD0 = crate::Reg<adc_chselrmod0::ADC_CHSELRMOD0_SPEC>;
///ADC channel selection register \[alternate\]
pub mod adc_chselrmod0;
///ADC_CHSELRMOD1 (rw) register accessor: an alias for `Reg<ADC_CHSELRMOD1_SPEC>`
pub type ADC_CHSELRMOD1 = crate::Reg<adc_chselrmod1::ADC_CHSELRMOD1_SPEC>;
///ADC channel selection register \[alternate\]
pub mod adc_chselrmod1;
///ADC_AWD3TR (rw) register accessor: an alias for `Reg<ADC_AWD3TR_SPEC>`
pub type ADC_AWD3TR = crate::Reg<adc_awd3tr::ADC_AWD3TR_SPEC>;
///ADC watchdog threshold register
pub mod adc_awd3tr;
///ADC_DR (r) register accessor: an alias for `Reg<ADC_DR_SPEC>`
pub type ADC_DR = crate::Reg<adc_dr::ADC_DR_SPEC>;
///ADC data register
pub mod adc_dr;
///ADC_PWR (rw) register accessor: an alias for `Reg<ADC_PWR_SPEC>`
pub type ADC_PWR = crate::Reg<adc_pwr::ADC_PWR_SPEC>;
///ADC data register
pub mod adc_pwr;
///ADC_AWD2CR (rw) register accessor: an alias for `Reg<ADC_AWD2CR_SPEC>`
pub type ADC_AWD2CR = crate::Reg<adc_awd2cr::ADC_AWD2CR_SPEC>;
///ADC Analog Watchdog 2 Configuration register
pub mod adc_awd2cr;
///ADC_AWD3CR (rw) register accessor: an alias for `Reg<ADC_AWD3CR_SPEC>`
pub type ADC_AWD3CR = crate::Reg<adc_awd3cr::ADC_AWD3CR_SPEC>;
///ADC Analog Watchdog 3 Configuration register
pub mod adc_awd3cr;
///ADC_CALFACT (rw) register accessor: an alias for `Reg<ADC_CALFACT_SPEC>`
pub type ADC_CALFACT = crate::Reg<adc_calfact::ADC_CALFACT_SPEC>;
///ADC Calibration factor
pub mod adc_calfact;
///ADC_OR (rw) register accessor: an alias for `Reg<ADC_OR_SPEC>`
pub type ADC_OR = crate::Reg<adc_or::ADC_OR_SPEC>;
///ADC option register
pub mod adc_or;
///ADC_CCR (rw) register accessor: an alias for `Reg<ADC_CCR_SPEC>`
pub type ADC_CCR = crate::Reg<adc_ccr::ADC_CCR_SPEC>;
///ADC common configuration register
pub mod adc_ccr;
