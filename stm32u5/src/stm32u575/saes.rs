///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: CR,
    ///0x04 - status register
    pub sr: SR,
    ///0x08 - data input register
    pub dinr: DINR,
    ///0x0c - data output register
    pub doutr: DOUTR,
    ///0x10 - key register 0
    pub keyr0: KEYR0,
    ///0x14 - key register 1
    pub keyr1: KEYR1,
    ///0x18 - key register 2
    pub keyr2: KEYR2,
    ///0x1c - key register 3
    pub keyr3: KEYR3,
    ///0x20 - initialization vector register 0
    pub ivr0: IVR0,
    ///0x24 - initialization vector register 1
    pub ivr1: IVR1,
    ///0x28 - initialization vector register 2
    pub ivr2: IVR2,
    ///0x2c - initialization vector register 3
    pub ivr3: IVR3,
    ///0x30 - key register 4
    pub keyr4: KEYR4,
    ///0x34 - key register 5
    pub keyr5: KEYR5,
    ///0x38 - key register 6
    pub keyr6: KEYR6,
    ///0x3c - key register 7
    pub keyr7: KEYR7,
    _reserved16: [u8; 0xc0],
    ///0x100 - configuration register
    pub dpacfgr: DPACFGR,
    _reserved17: [u8; 0x01fc],
    ///0x300 - interrupt enable register
    pub ier: IER,
    ///0x304 - interrupt status register
    pub isr: ISR,
    ///0x308 - interrupt clear register
    pub icr: ICR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///control register
pub mod cr;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///status register
pub mod sr;
///DINR (w) register accessor: an alias for `Reg<DINR_SPEC>`
pub type DINR = crate::Reg<dinr::DINR_SPEC>;
///data input register
pub mod dinr;
///DOUTR (r) register accessor: an alias for `Reg<DOUTR_SPEC>`
pub type DOUTR = crate::Reg<doutr::DOUTR_SPEC>;
///data output register
pub mod doutr;
///KEYR0 (w) register accessor: an alias for `Reg<KEYR0_SPEC>`
pub type KEYR0 = crate::Reg<keyr0::KEYR0_SPEC>;
///key register 0
pub mod keyr0;
///KEYR1 (w) register accessor: an alias for `Reg<KEYR1_SPEC>`
pub type KEYR1 = crate::Reg<keyr1::KEYR1_SPEC>;
///key register 1
pub mod keyr1;
///KEYR2 (w) register accessor: an alias for `Reg<KEYR2_SPEC>`
pub type KEYR2 = crate::Reg<keyr2::KEYR2_SPEC>;
///key register 2
pub mod keyr2;
///KEYR3 (w) register accessor: an alias for `Reg<KEYR3_SPEC>`
pub type KEYR3 = crate::Reg<keyr3::KEYR3_SPEC>;
///key register 3
pub mod keyr3;
///IVR0 (rw) register accessor: an alias for `Reg<IVR0_SPEC>`
pub type IVR0 = crate::Reg<ivr0::IVR0_SPEC>;
///initialization vector register 0
pub mod ivr0;
///IVR1 (rw) register accessor: an alias for `Reg<IVR1_SPEC>`
pub type IVR1 = crate::Reg<ivr1::IVR1_SPEC>;
///initialization vector register 1
pub mod ivr1;
///IVR2 (rw) register accessor: an alias for `Reg<IVR2_SPEC>`
pub type IVR2 = crate::Reg<ivr2::IVR2_SPEC>;
///initialization vector register 2
pub mod ivr2;
///IVR3 (rw) register accessor: an alias for `Reg<IVR3_SPEC>`
pub type IVR3 = crate::Reg<ivr3::IVR3_SPEC>;
///initialization vector register 3
pub mod ivr3;
///KEYR4 (w) register accessor: an alias for `Reg<KEYR4_SPEC>`
pub type KEYR4 = crate::Reg<keyr4::KEYR4_SPEC>;
///key register 4
pub mod keyr4;
///KEYR5 (w) register accessor: an alias for `Reg<KEYR5_SPEC>`
pub type KEYR5 = crate::Reg<keyr5::KEYR5_SPEC>;
///key register 5
pub mod keyr5;
///KEYR6 (w) register accessor: an alias for `Reg<KEYR6_SPEC>`
pub type KEYR6 = crate::Reg<keyr6::KEYR6_SPEC>;
///key register 6
pub mod keyr6;
///KEYR7 (w) register accessor: an alias for `Reg<KEYR7_SPEC>`
pub type KEYR7 = crate::Reg<keyr7::KEYR7_SPEC>;
///key register 7
pub mod keyr7;
///DPACFGR (rw) register accessor: an alias for `Reg<DPACFGR_SPEC>`
pub type DPACFGR = crate::Reg<dpacfgr::DPACFGR_SPEC>;
///configuration register
pub mod dpacfgr;
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///interrupt enable register
pub mod ier;
///ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///interrupt status register
pub mod isr;
///ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///interrupt clear register
pub mod icr;
