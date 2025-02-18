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
    ///0x14 - ADC sample time register 1
    pub adc_smpr1: ADC_SMPR1,
    ///0x18 - ADC sample time register 2
    pub adc_smpr2: ADC_SMPR2,
    ///0x1c - ADC channel preselection register
    pub adc_pcsel: ADC_PCSEL,
    _reserved8: [u8; 0x10],
    ///0x30 - ADC regular sequence register 1
    pub adc_sqr1: ADC_SQR1,
    ///0x34 - ADC regular sequence register 2
    pub adc_sqr2: ADC_SQR2,
    ///0x38 - ADC regular sequence register 3
    pub adc_sqr3: ADC_SQR3,
    ///0x3c - ADC regular sequence register 4
    pub adc_sqr4: ADC_SQR4,
    ///0x40 - ADC regular Data Register
    pub adc_dr: ADC_DR,
    _reserved13: [u8; 0x08],
    ///0x4c - ADC injected sequence register
    pub adc_jsqr: ADC_JSQR,
    _reserved14: [u8; 0x10],
    ///0x60 - ADC offset register
    pub adc_ofr1: ADC_OFR1,
    ///0x64 - ADC offset register
    pub adc_ofr2: ADC_OFR2,
    ///0x68 - ADC offset register
    pub adc_ofr3: ADC_OFR3,
    ///0x6c - ADC offset register
    pub adc_ofr4: ADC_OFR4,
    ///0x70 - ADC gain compensation register
    pub adc_gcomp: ADC_GCOMP,
    _reserved19: [u8; 0x0c],
    ///0x80 - ADC injected data register
    pub adc_jdr1: ADC_JDR1,
    ///0x84 - ADC injected data register
    pub adc_jdr2: ADC_JDR2,
    ///0x88 - ADC injected data register
    pub adc_jdr3: ADC_JDR3,
    ///0x8c - ADC injected data register
    pub adc_jdr4: ADC_JDR4,
    _reserved23: [u8; 0x10],
    ///0xa0 - ADC analog watchdog 2 configuration register
    pub adc_awd2cr: ADC_AWD2CR,
    ///0xa4 - ADC analog watchdog 3 configuration register
    pub adc_awd3cr: ADC_AWD3CR,
    ///0xa8 - ADC watchdog threshold register 1
    pub adc_ltr1: ADC_LTR1,
    ///0xac - ADC watchdog threshold register 1
    pub adc_htr1: ADC_HTR1,
    ///0xb0 - ADC watchdog lower threshold register 2
    pub adc_ltr2: ADC_LTR2,
    ///0xb4 - ADC watchdog higher threshold register 2
    pub adc_htr2: ADC_HTR2,
    ///0xb8 - ADC watchdog lower threshold register 3
    pub adc_ltr3: ADC_LTR3,
    ///0xbc - ADC watchdog higher threshold register 3
    pub adc_htr3: ADC_HTR3,
    ///0xc0 - ADC differential mode selection register
    pub adc_difsel: ADC_DIFSEL,
    ///0xc4 - ADC user control register
    pub adc_calfact: ADC_CALFACT,
    ///0xc8 - ADC calibration factor register
    pub adc_calfact2: ADC_CALFACT2,
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
///ADC_SMPR1 (rw) register accessor: an alias for `Reg<ADC_SMPR1_SPEC>`
pub type ADC_SMPR1 = crate::Reg<adc_smpr1::ADC_SMPR1_SPEC>;
///ADC sample time register 1
pub mod adc_smpr1;
///ADC_SMPR2 (rw) register accessor: an alias for `Reg<ADC_SMPR2_SPEC>`
pub type ADC_SMPR2 = crate::Reg<adc_smpr2::ADC_SMPR2_SPEC>;
///ADC sample time register 2
pub mod adc_smpr2;
///ADC_PCSEL (rw) register accessor: an alias for `Reg<ADC_PCSEL_SPEC>`
pub type ADC_PCSEL = crate::Reg<adc_pcsel::ADC_PCSEL_SPEC>;
///ADC channel preselection register
pub mod adc_pcsel;
///ADC_SQR1 (rw) register accessor: an alias for `Reg<ADC_SQR1_SPEC>`
pub type ADC_SQR1 = crate::Reg<adc_sqr1::ADC_SQR1_SPEC>;
///ADC regular sequence register 1
pub mod adc_sqr1;
///ADC_SQR2 (rw) register accessor: an alias for `Reg<ADC_SQR2_SPEC>`
pub type ADC_SQR2 = crate::Reg<adc_sqr2::ADC_SQR2_SPEC>;
///ADC regular sequence register 2
pub mod adc_sqr2;
///ADC_SQR3 (rw) register accessor: an alias for `Reg<ADC_SQR3_SPEC>`
pub type ADC_SQR3 = crate::Reg<adc_sqr3::ADC_SQR3_SPEC>;
///ADC regular sequence register 3
pub mod adc_sqr3;
///ADC_SQR4 (rw) register accessor: an alias for `Reg<ADC_SQR4_SPEC>`
pub type ADC_SQR4 = crate::Reg<adc_sqr4::ADC_SQR4_SPEC>;
///ADC regular sequence register 4
pub mod adc_sqr4;
///ADC_DR (r) register accessor: an alias for `Reg<ADC_DR_SPEC>`
pub type ADC_DR = crate::Reg<adc_dr::ADC_DR_SPEC>;
///ADC regular Data Register
pub mod adc_dr;
///ADC_JSQR (rw) register accessor: an alias for `Reg<ADC_JSQR_SPEC>`
pub type ADC_JSQR = crate::Reg<adc_jsqr::ADC_JSQR_SPEC>;
///ADC injected sequence register
pub mod adc_jsqr;
///ADC_OFR1 (rw) register accessor: an alias for `Reg<ADC_OFR1_SPEC>`
pub type ADC_OFR1 = crate::Reg<adc_ofr1::ADC_OFR1_SPEC>;
///ADC offset register
pub mod adc_ofr1;
///ADC_OFR2 (rw) register accessor: an alias for `Reg<ADC_OFR2_SPEC>`
pub type ADC_OFR2 = crate::Reg<adc_ofr2::ADC_OFR2_SPEC>;
///ADC offset register
pub mod adc_ofr2;
///ADC_OFR3 (rw) register accessor: an alias for `Reg<ADC_OFR3_SPEC>`
pub type ADC_OFR3 = crate::Reg<adc_ofr3::ADC_OFR3_SPEC>;
///ADC offset register
pub mod adc_ofr3;
///ADC_OFR4 (rw) register accessor: an alias for `Reg<ADC_OFR4_SPEC>`
pub type ADC_OFR4 = crate::Reg<adc_ofr4::ADC_OFR4_SPEC>;
///ADC offset register
pub mod adc_ofr4;
///ADC_GCOMP (rw) register accessor: an alias for `Reg<ADC_GCOMP_SPEC>`
pub type ADC_GCOMP = crate::Reg<adc_gcomp::ADC_GCOMP_SPEC>;
///ADC gain compensation register
pub mod adc_gcomp;
///ADC_JDR1 (r) register accessor: an alias for `Reg<ADC_JDR1_SPEC>`
pub type ADC_JDR1 = crate::Reg<adc_jdr1::ADC_JDR1_SPEC>;
///ADC injected data register
pub mod adc_jdr1;
///ADC_JDR2 (r) register accessor: an alias for `Reg<ADC_JDR2_SPEC>`
pub type ADC_JDR2 = crate::Reg<adc_jdr2::ADC_JDR2_SPEC>;
///ADC injected data register
pub mod adc_jdr2;
///ADC_JDR3 (r) register accessor: an alias for `Reg<ADC_JDR3_SPEC>`
pub type ADC_JDR3 = crate::Reg<adc_jdr3::ADC_JDR3_SPEC>;
///ADC injected data register
pub mod adc_jdr3;
///ADC_JDR4 (r) register accessor: an alias for `Reg<ADC_JDR4_SPEC>`
pub type ADC_JDR4 = crate::Reg<adc_jdr4::ADC_JDR4_SPEC>;
///ADC injected data register
pub mod adc_jdr4;
///ADC_AWD2CR (rw) register accessor: an alias for `Reg<ADC_AWD2CR_SPEC>`
pub type ADC_AWD2CR = crate::Reg<adc_awd2cr::ADC_AWD2CR_SPEC>;
///ADC analog watchdog 2 configuration register
pub mod adc_awd2cr;
///ADC_AWD3CR (rw) register accessor: an alias for `Reg<ADC_AWD3CR_SPEC>`
pub type ADC_AWD3CR = crate::Reg<adc_awd3cr::ADC_AWD3CR_SPEC>;
///ADC analog watchdog 3 configuration register
pub mod adc_awd3cr;
///ADC_LTR1 (rw) register accessor: an alias for `Reg<ADC_LTR1_SPEC>`
pub type ADC_LTR1 = crate::Reg<adc_ltr1::ADC_LTR1_SPEC>;
///ADC watchdog threshold register 1
pub mod adc_ltr1;
///ADC_HTR1 (rw) register accessor: an alias for `Reg<ADC_HTR1_SPEC>`
pub type ADC_HTR1 = crate::Reg<adc_htr1::ADC_HTR1_SPEC>;
///ADC watchdog threshold register 1
pub mod adc_htr1;
///ADC_LTR2 (rw) register accessor: an alias for `Reg<ADC_LTR2_SPEC>`
pub type ADC_LTR2 = crate::Reg<adc_ltr2::ADC_LTR2_SPEC>;
///ADC watchdog lower threshold register 2
pub mod adc_ltr2;
///ADC_HTR2 (rw) register accessor: an alias for `Reg<ADC_HTR2_SPEC>`
pub type ADC_HTR2 = crate::Reg<adc_htr2::ADC_HTR2_SPEC>;
///ADC watchdog higher threshold register 2
pub mod adc_htr2;
///ADC_LTR3 (rw) register accessor: an alias for `Reg<ADC_LTR3_SPEC>`
pub type ADC_LTR3 = crate::Reg<adc_ltr3::ADC_LTR3_SPEC>;
///ADC watchdog lower threshold register 3
pub mod adc_ltr3;
///ADC_HTR3 (rw) register accessor: an alias for `Reg<ADC_HTR3_SPEC>`
pub type ADC_HTR3 = crate::Reg<adc_htr3::ADC_HTR3_SPEC>;
///ADC watchdog higher threshold register 3
pub mod adc_htr3;
///ADC_DIFSEL (rw) register accessor: an alias for `Reg<ADC_DIFSEL_SPEC>`
pub type ADC_DIFSEL = crate::Reg<adc_difsel::ADC_DIFSEL_SPEC>;
///ADC differential mode selection register
pub mod adc_difsel;
///ADC_CALFACT (rw) register accessor: an alias for `Reg<ADC_CALFACT_SPEC>`
pub type ADC_CALFACT = crate::Reg<adc_calfact::ADC_CALFACT_SPEC>;
///ADC user control register
pub mod adc_calfact;
///ADC_CALFACT2 (rw) register accessor: an alias for `Reg<ADC_CALFACT2_SPEC>`
pub type ADC_CALFACT2 = crate::Reg<adc_calfact2::ADC_CALFACT2_SPEC>;
///ADC calibration factor register
pub mod adc_calfact2;
