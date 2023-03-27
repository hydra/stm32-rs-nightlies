///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - status register
    pub sr: SR,
    ///0x04 - control register 1
    pub cr1: CR1,
    ///0x08 - control register 2
    pub cr2: CR2,
    ///0x0c - sample time register 1
    pub smpr1: SMPR1,
    ///0x10 - sample time register 2
    pub smpr2: SMPR2,
    ///0x14 - sample time register 3
    pub smpr3: SMPR3,
    ///0x18 - injected channel data offset register x
    pub jofr1: JOFR1,
    ///0x1c - injected channel data offset register x
    pub jofr2: JOFR2,
    ///0x20 - injected channel data offset register x
    pub jofr3: JOFR3,
    ///0x24 - injected channel data offset register x
    pub jofr4: JOFR4,
    ///0x28 - watchdog higher threshold register
    pub htr: HTR,
    ///0x2c - watchdog lower threshold register
    pub ltr: LTR,
    ///0x30 - regular sequence register 1
    pub sqr1: SQR1,
    ///0x34 - regular sequence register 2
    pub sqr2: SQR2,
    ///0x38 - regular sequence register 3
    pub sqr3: SQR3,
    ///0x3c - regular sequence register 4
    pub sqr4: SQR4,
    ///0x40 - regular sequence register 5
    pub sqr5: SQR5,
    ///0x44 - injected sequence register
    pub jsqr: JSQR,
    ///0x48 - injected data register x
    pub jdr1: JDR1,
    ///0x4c - injected data register x
    pub jdr2: JDR2,
    ///0x50 - injected data register x
    pub jdr3: JDR3,
    ///0x54 - injected data register x
    pub jdr4: JDR4,
    ///0x58 - regular data register
    pub dr: DR,
    ///0x5c - sample time register 0
    pub smpr0: SMPR0,
    _reserved24: [u8; 0x02a0],
    ///0x300 - ADC common status register
    pub csr: CSR,
    ///0x304 - ADC common control register
    pub ccr: CCR,
}
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///status register
pub mod sr;
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///control register 1
pub mod cr1;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///control register 2
pub mod cr2;
///SMPR1 (rw) register accessor: an alias for `Reg<SMPR1_SPEC>`
pub type SMPR1 = crate::Reg<smpr1::SMPR1_SPEC>;
///sample time register 1
pub mod smpr1;
///SMPR2 (rw) register accessor: an alias for `Reg<SMPR2_SPEC>`
pub type SMPR2 = crate::Reg<smpr2::SMPR2_SPEC>;
///sample time register 2
pub mod smpr2;
///SMPR3 (rw) register accessor: an alias for `Reg<SMPR3_SPEC>`
pub type SMPR3 = crate::Reg<smpr3::SMPR3_SPEC>;
///sample time register 3
pub mod smpr3;
///JOFR1 (rw) register accessor: an alias for `Reg<JOFR1_SPEC>`
pub type JOFR1 = crate::Reg<jofr1::JOFR1_SPEC>;
///injected channel data offset register x
pub mod jofr1;
///JOFR2 (rw) register accessor: an alias for `Reg<JOFR2_SPEC>`
pub type JOFR2 = crate::Reg<jofr2::JOFR2_SPEC>;
///injected channel data offset register x
pub mod jofr2;
///JOFR3 (rw) register accessor: an alias for `Reg<JOFR3_SPEC>`
pub type JOFR3 = crate::Reg<jofr3::JOFR3_SPEC>;
///injected channel data offset register x
pub mod jofr3;
///JOFR4 (rw) register accessor: an alias for `Reg<JOFR4_SPEC>`
pub type JOFR4 = crate::Reg<jofr4::JOFR4_SPEC>;
///injected channel data offset register x
pub mod jofr4;
///HTR (rw) register accessor: an alias for `Reg<HTR_SPEC>`
pub type HTR = crate::Reg<htr::HTR_SPEC>;
///watchdog higher threshold register
pub mod htr;
///LTR (rw) register accessor: an alias for `Reg<LTR_SPEC>`
pub type LTR = crate::Reg<ltr::LTR_SPEC>;
///watchdog lower threshold register
pub mod ltr;
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
///SQR5 (rw) register accessor: an alias for `Reg<SQR5_SPEC>`
pub type SQR5 = crate::Reg<sqr5::SQR5_SPEC>;
///regular sequence register 5
pub mod sqr5;
///JSQR (rw) register accessor: an alias for `Reg<JSQR_SPEC>`
pub type JSQR = crate::Reg<jsqr::JSQR_SPEC>;
///injected sequence register
pub mod jsqr;
///JDR1 (r) register accessor: an alias for `Reg<JDR1_SPEC>`
pub type JDR1 = crate::Reg<jdr1::JDR1_SPEC>;
///injected data register x
pub mod jdr1;
///JDR2 (r) register accessor: an alias for `Reg<JDR2_SPEC>`
pub type JDR2 = crate::Reg<jdr2::JDR2_SPEC>;
///injected data register x
pub mod jdr2;
///JDR3 (r) register accessor: an alias for `Reg<JDR3_SPEC>`
pub type JDR3 = crate::Reg<jdr3::JDR3_SPEC>;
///injected data register x
pub mod jdr3;
///JDR4 (r) register accessor: an alias for `Reg<JDR4_SPEC>`
pub type JDR4 = crate::Reg<jdr4::JDR4_SPEC>;
///injected data register x
pub mod jdr4;
///DR (r) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///regular data register
pub mod dr;
///SMPR0 (rw) register accessor: an alias for `Reg<SMPR0_SPEC>`
pub type SMPR0 = crate::Reg<smpr0::SMPR0_SPEC>;
///sample time register 0
pub mod smpr0;
///CSR (r) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///ADC common status register
pub mod csr;
///CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
///ADC common control register
pub mod ccr;
