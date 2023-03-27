///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - interrupt and status register
    pub isr: ISR,
    ///0x04 - interrupt enable register
    pub ier: IER,
    ///0x08 - control register
    pub cr: CR,
    ///0x0c - configuration register
    pub cfgr: CFGR,
    ///0x10 - configuration register
    pub cfgr2: CFGR2,
    ///0x14 - sample time register 1
    pub smpr1: SMPR1,
    ///0x18 - sample time register 2
    pub smpr2: SMPR2,
    _reserved7: [u8; 0x04],
    ///0x20 - watchdog threshold register 1
    pub tr1: TR1,
    ///0x24 - watchdog threshold register
    pub tr2: TR2,
    ///0x28 - watchdog threshold register 3
    pub tr3: TR3,
    _reserved10: [u8; 0x04],
    ///0x30 - regular sequence register 1
    pub sqr1: SQR1,
    ///0x34 - regular sequence register 2
    pub sqr2: SQR2,
    ///0x38 - regular sequence register 3
    pub sqr3: SQR3,
    ///0x3c - regular sequence register 4
    pub sqr4: SQR4,
    ///0x40 - regular Data Register
    pub dr: DR,
    _reserved15: [u8; 0x08],
    ///0x4c - injected sequence register
    pub jsqr: JSQR,
    _reserved16: [u8; 0x10],
    ///0x60 - offset register 1
    pub ofr1: OFR1,
    ///0x64 - offset register 2
    pub ofr2: OFR2,
    ///0x68 - offset register 3
    pub ofr3: OFR3,
    ///0x6c - offset register 4
    pub ofr4: OFR4,
    _reserved20: [u8; 0x10],
    ///0x80 - injected data register 1
    pub jdr1: JDR1,
    ///0x84 - injected data register 2
    pub jdr2: JDR2,
    ///0x88 - injected data register 3
    pub jdr3: JDR3,
    ///0x8c - injected data register 4
    pub jdr4: JDR4,
    _reserved24: [u8; 0x10],
    ///0xa0 - Analog Watchdog 2 Configuration Register
    pub awd2cr: AWD2CR,
    ///0xa4 - Analog Watchdog 3 Configuration Register
    pub awd3cr: AWD3CR,
    _reserved26: [u8; 0x08],
    ///0xb0 - Differential Mode Selection Register 2
    pub difsel: DIFSEL,
    ///0xb4 - Calibration Factors
    pub calfact: CALFACT,
    _reserved28: [u8; 0x08],
    ///0xc0 - Gain compensation Register
    pub gcomp: GCOMP,
}
///ISR (rw) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///interrupt and status register
pub mod isr;
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///interrupt enable register
pub mod ier;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///control register
pub mod cr;
///CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
///configuration register
pub mod cfgr;
///CFGR2 (rw) register accessor: an alias for `Reg<CFGR2_SPEC>`
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
///configuration register
pub mod cfgr2;
///SMPR1 (rw) register accessor: an alias for `Reg<SMPR1_SPEC>`
pub type SMPR1 = crate::Reg<smpr1::SMPR1_SPEC>;
///sample time register 1
pub mod smpr1;
///SMPR2 (rw) register accessor: an alias for `Reg<SMPR2_SPEC>`
pub type SMPR2 = crate::Reg<smpr2::SMPR2_SPEC>;
///sample time register 2
pub mod smpr2;
///TR1 (rw) register accessor: an alias for `Reg<TR1_SPEC>`
pub type TR1 = crate::Reg<tr1::TR1_SPEC>;
///watchdog threshold register 1
pub mod tr1;
///TR2 (rw) register accessor: an alias for `Reg<TR2_SPEC>`
pub type TR2 = crate::Reg<tr2::TR2_SPEC>;
///watchdog threshold register
pub mod tr2;
///TR3 (rw) register accessor: an alias for `Reg<TR3_SPEC>`
pub type TR3 = crate::Reg<tr3::TR3_SPEC>;
///watchdog threshold register 3
pub mod tr3;
///SQR1 (rw) register accessor: an alias for `Reg<SQR1_SPEC>`
pub type SQR1 = crate::Reg<sqr1::SQR1_SPEC>;
///regular sequence register 1
pub mod sqr1;
///SQR2 (rw) register accessor: an alias for `Reg<SQR2_SPEC>`
pub type SQR2 = crate::Reg<sqr2::SQR2_SPEC>;
///regular sequence register 2
pub mod sqr2;
///SQR3 (rw) register accessor: an alias for `Reg<SQR3_SPEC>`
pub type SQR3 = crate::Reg<sqr3::SQR3_SPEC>;
///regular sequence register 3
pub mod sqr3;
///SQR4 (rw) register accessor: an alias for `Reg<SQR4_SPEC>`
pub type SQR4 = crate::Reg<sqr4::SQR4_SPEC>;
///regular sequence register 4
pub mod sqr4;
///DR (r) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///regular Data Register
pub mod dr;
///JSQR (rw) register accessor: an alias for `Reg<JSQR_SPEC>`
pub type JSQR = crate::Reg<jsqr::JSQR_SPEC>;
///injected sequence register
pub mod jsqr;
///OFR1 (rw) register accessor: an alias for `Reg<OFR1_SPEC>`
pub type OFR1 = crate::Reg<ofr1::OFR1_SPEC>;
///offset register 1
pub mod ofr1;
///OFR2 (rw) register accessor: an alias for `Reg<OFR2_SPEC>`
pub type OFR2 = crate::Reg<ofr2::OFR2_SPEC>;
///offset register 2
pub mod ofr2;
///OFR3 (rw) register accessor: an alias for `Reg<OFR3_SPEC>`
pub type OFR3 = crate::Reg<ofr3::OFR3_SPEC>;
///offset register 3
pub mod ofr3;
///OFR4 (rw) register accessor: an alias for `Reg<OFR4_SPEC>`
pub type OFR4 = crate::Reg<ofr4::OFR4_SPEC>;
///offset register 4
pub mod ofr4;
///JDR1 (r) register accessor: an alias for `Reg<JDR1_SPEC>`
pub type JDR1 = crate::Reg<jdr1::JDR1_SPEC>;
///injected data register 1
pub mod jdr1;
///JDR2 (r) register accessor: an alias for `Reg<JDR2_SPEC>`
pub type JDR2 = crate::Reg<jdr2::JDR2_SPEC>;
///injected data register 2
pub mod jdr2;
///JDR3 (r) register accessor: an alias for `Reg<JDR3_SPEC>`
pub type JDR3 = crate::Reg<jdr3::JDR3_SPEC>;
///injected data register 3
pub mod jdr3;
///JDR4 (r) register accessor: an alias for `Reg<JDR4_SPEC>`
pub type JDR4 = crate::Reg<jdr4::JDR4_SPEC>;
///injected data register 4
pub mod jdr4;
///AWD2CR (rw) register accessor: an alias for `Reg<AWD2CR_SPEC>`
pub type AWD2CR = crate::Reg<awd2cr::AWD2CR_SPEC>;
///Analog Watchdog 2 Configuration Register
pub mod awd2cr;
///AWD3CR (rw) register accessor: an alias for `Reg<AWD3CR_SPEC>`
pub type AWD3CR = crate::Reg<awd3cr::AWD3CR_SPEC>;
///Analog Watchdog 3 Configuration Register
pub mod awd3cr;
///DIFSEL (rw) register accessor: an alias for `Reg<DIFSEL_SPEC>`
pub type DIFSEL = crate::Reg<difsel::DIFSEL_SPEC>;
///Differential Mode Selection Register 2
pub mod difsel;
///CALFACT (rw) register accessor: an alias for `Reg<CALFACT_SPEC>`
pub type CALFACT = crate::Reg<calfact::CALFACT_SPEC>;
///Calibration Factors
pub mod calfact;
///GCOMP (rw) register accessor: an alias for `Reg<GCOMP_SPEC>`
pub type GCOMP = crate::Reg<gcomp::GCOMP_SPEC>;
///Gain compensation Register
pub mod gcomp;
