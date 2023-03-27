///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - SAES control register
    pub cr: CR,
    ///0x04 - SAES status register
    pub sr: SR,
    ///0x08 - SAES data input register
    pub dinr: DINR,
    ///0x0c - SAES data output register
    pub doutr: DOUTR,
    ///0x10 - SAES key register 0
    pub keyr0: KEYR0,
    ///0x14 - SAES key register 1
    pub keyr1: KEYR1,
    ///0x18 - SAES key register 2
    pub keyr2: KEYR2,
    ///0x1c - SAES key register 3
    pub keyr3: KEYR3,
    ///0x20 - SAES initialization vector register 0
    pub ivr0: IVR0,
    ///0x24 - SAES initialization vector register 1
    pub ivr1: IVR1,
    ///0x28 - SAES initialization vector register 2
    pub ivr2: IVR2,
    ///0x2c - SAES initialization vector register 3
    pub ivr3: IVR3,
    ///0x30 - SAES key register 4
    pub keyr4: KEYR4,
    ///0x34 - SAES key register 5
    pub keyr5: KEYR5,
    ///0x38 - SAES key register 6
    pub keyr6: KEYR6,
    ///0x3c - SAES key register 7
    pub keyr7: KEYR7,
    ///0x40 - SAES suspend registers
    pub susp0r: SUSP0R,
    ///0x44 - SAES suspend registers
    pub susp1r: SUSP1R,
    ///0x48 - SAES suspend registers
    pub susp2r: SUSP2R,
    ///0x4c - SAES suspend registers
    pub susp3r: SUSP3R,
    ///0x50 - SAES suspend registers
    pub susp4r: SUSP4R,
    ///0x54 - SAES suspend registers
    pub susp5r: SUSP5R,
    ///0x58 - SAES suspend registers
    pub susp6r: SUSP6R,
    ///0x5c - SAES suspend registers
    pub susp7r: SUSP7R,
    _reserved24: [u8; 0x02a0],
    ///0x300 - SAES interrupt enable register
    pub ier: IER,
    ///0x304 - SAES interrupt status register
    pub isr: ISR,
    ///0x308 - SAES interrupt clear register
    pub icr: ICR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///SAES control register
pub mod cr;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///SAES status register
pub mod sr;
///DINR (w) register accessor: an alias for `Reg<DINR_SPEC>`
pub type DINR = crate::Reg<dinr::DINR_SPEC>;
///SAES data input register
pub mod dinr;
///DOUTR (r) register accessor: an alias for `Reg<DOUTR_SPEC>`
pub type DOUTR = crate::Reg<doutr::DOUTR_SPEC>;
///SAES data output register
pub mod doutr;
///KEYR0 (w) register accessor: an alias for `Reg<KEYR0_SPEC>`
pub type KEYR0 = crate::Reg<keyr0::KEYR0_SPEC>;
///SAES key register 0
pub mod keyr0;
///KEYR1 (w) register accessor: an alias for `Reg<KEYR1_SPEC>`
pub type KEYR1 = crate::Reg<keyr1::KEYR1_SPEC>;
///SAES key register 1
pub mod keyr1;
///KEYR2 (w) register accessor: an alias for `Reg<KEYR2_SPEC>`
pub type KEYR2 = crate::Reg<keyr2::KEYR2_SPEC>;
///SAES key register 2
pub mod keyr2;
///KEYR3 (w) register accessor: an alias for `Reg<KEYR3_SPEC>`
pub type KEYR3 = crate::Reg<keyr3::KEYR3_SPEC>;
///SAES key register 3
pub mod keyr3;
///IVR0 (rw) register accessor: an alias for `Reg<IVR0_SPEC>`
pub type IVR0 = crate::Reg<ivr0::IVR0_SPEC>;
///SAES initialization vector register 0
pub mod ivr0;
///IVR1 (rw) register accessor: an alias for `Reg<IVR1_SPEC>`
pub type IVR1 = crate::Reg<ivr1::IVR1_SPEC>;
///SAES initialization vector register 1
pub mod ivr1;
///IVR2 (rw) register accessor: an alias for `Reg<IVR2_SPEC>`
pub type IVR2 = crate::Reg<ivr2::IVR2_SPEC>;
///SAES initialization vector register 2
pub mod ivr2;
///IVR3 (rw) register accessor: an alias for `Reg<IVR3_SPEC>`
pub type IVR3 = crate::Reg<ivr3::IVR3_SPEC>;
///SAES initialization vector register 3
pub mod ivr3;
///KEYR4 (w) register accessor: an alias for `Reg<KEYR4_SPEC>`
pub type KEYR4 = crate::Reg<keyr4::KEYR4_SPEC>;
///SAES key register 4
pub mod keyr4;
///KEYR5 (w) register accessor: an alias for `Reg<KEYR5_SPEC>`
pub type KEYR5 = crate::Reg<keyr5::KEYR5_SPEC>;
///SAES key register 5
pub mod keyr5;
///KEYR6 (w) register accessor: an alias for `Reg<KEYR6_SPEC>`
pub type KEYR6 = crate::Reg<keyr6::KEYR6_SPEC>;
///SAES key register 6
pub mod keyr6;
///KEYR7 (w) register accessor: an alias for `Reg<KEYR7_SPEC>`
pub type KEYR7 = crate::Reg<keyr7::KEYR7_SPEC>;
///SAES key register 7
pub mod keyr7;
///SUSP0R (rw) register accessor: an alias for `Reg<SUSP0R_SPEC>`
pub type SUSP0R = crate::Reg<susp0r::SUSP0R_SPEC>;
///SAES suspend registers
pub mod susp0r;
///SUSP1R (rw) register accessor: an alias for `Reg<SUSP1R_SPEC>`
pub type SUSP1R = crate::Reg<susp1r::SUSP1R_SPEC>;
///SAES suspend registers
pub mod susp1r;
///SUSP2R (rw) register accessor: an alias for `Reg<SUSP2R_SPEC>`
pub type SUSP2R = crate::Reg<susp2r::SUSP2R_SPEC>;
///SAES suspend registers
pub mod susp2r;
///SUSP3R (rw) register accessor: an alias for `Reg<SUSP3R_SPEC>`
pub type SUSP3R = crate::Reg<susp3r::SUSP3R_SPEC>;
///SAES suspend registers
pub mod susp3r;
///SUSP4R (rw) register accessor: an alias for `Reg<SUSP4R_SPEC>`
pub type SUSP4R = crate::Reg<susp4r::SUSP4R_SPEC>;
///SAES suspend registers
pub mod susp4r;
///SUSP5R (rw) register accessor: an alias for `Reg<SUSP5R_SPEC>`
pub type SUSP5R = crate::Reg<susp5r::SUSP5R_SPEC>;
///SAES suspend registers
pub mod susp5r;
///SUSP6R (rw) register accessor: an alias for `Reg<SUSP6R_SPEC>`
pub type SUSP6R = crate::Reg<susp6r::SUSP6R_SPEC>;
///SAES suspend registers
pub mod susp6r;
///SUSP7R (rw) register accessor: an alias for `Reg<SUSP7R_SPEC>`
pub type SUSP7R = crate::Reg<susp7r::SUSP7R_SPEC>;
///SAES suspend registers
pub mod susp7r;
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///SAES interrupt enable register
pub mod ier;
///ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///SAES interrupt status register
pub mod isr;
///ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///SAES interrupt clear register
pub mod icr;
