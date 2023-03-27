///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: CR,
    ///0x04 - Status register
    pub sr: SR,
    ///0x08 - Data input register
    pub dinr: DINR,
    ///0x0c - Data output register
    pub doutr: DOUTR,
    ///0x10 - AES Key register 0
    pub keyr0: KEYR0,
    ///0x14 - AES Key register 1
    pub keyr1: KEYR1,
    ///0x18 - AES Key register 2
    pub keyr2: KEYR2,
    ///0x1c - AES Key register 3
    pub keyr3: KEYR3,
    ///0x20 - Initialization Vector Register 0
    pub ivr0: IVR0,
    ///0x24 - Initialization Vector Register 1
    pub ivr1: IVR1,
    ///0x28 - Initialization Vector Register 2
    pub ivr2: IVR2,
    ///0x2c - Initialization Vector Register 3
    pub ivr3: IVR3,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///control register
pub mod cr;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///Status register
pub mod sr;
///DINR (rw) register accessor: an alias for `Reg<DINR_SPEC>`
pub type DINR = crate::Reg<dinr::DINR_SPEC>;
///Data input register
pub mod dinr;
///DOUTR (r) register accessor: an alias for `Reg<DOUTR_SPEC>`
pub type DOUTR = crate::Reg<doutr::DOUTR_SPEC>;
///Data output register
pub mod doutr;
///KEYR0 (rw) register accessor: an alias for `Reg<KEYR0_SPEC>`
pub type KEYR0 = crate::Reg<keyr0::KEYR0_SPEC>;
///AES Key register 0
pub mod keyr0;
///KEYR1 (rw) register accessor: an alias for `Reg<KEYR1_SPEC>`
pub type KEYR1 = crate::Reg<keyr1::KEYR1_SPEC>;
///AES Key register 1
pub mod keyr1;
///KEYR2 (rw) register accessor: an alias for `Reg<KEYR2_SPEC>`
pub type KEYR2 = crate::Reg<keyr2::KEYR2_SPEC>;
///AES Key register 2
pub mod keyr2;
///KEYR3 (rw) register accessor: an alias for `Reg<KEYR3_SPEC>`
pub type KEYR3 = crate::Reg<keyr3::KEYR3_SPEC>;
///AES Key register 3
pub mod keyr3;
///IVR0 (rw) register accessor: an alias for `Reg<IVR0_SPEC>`
pub type IVR0 = crate::Reg<ivr0::IVR0_SPEC>;
///Initialization Vector Register 0
pub mod ivr0;
///IVR1 (rw) register accessor: an alias for `Reg<IVR1_SPEC>`
pub type IVR1 = crate::Reg<ivr1::IVR1_SPEC>;
///Initialization Vector Register 1
pub mod ivr1;
///IVR2 (rw) register accessor: an alias for `Reg<IVR2_SPEC>`
pub type IVR2 = crate::Reg<ivr2::IVR2_SPEC>;
///Initialization Vector Register 2
pub mod ivr2;
///IVR3 (rw) register accessor: an alias for `Reg<IVR3_SPEC>`
pub type IVR3 = crate::Reg<ivr3::IVR3_SPEC>;
///Initialization Vector Register 3
pub mod ivr3;
