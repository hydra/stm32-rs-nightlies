///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Access control register
    pub acr: ACR,
    _reserved1: [u8; 0x04],
    ///0x08 - Flash key register
    pub keyr: KEYR,
    ///0x0c - Option byte key register
    pub optkeyr: OPTKEYR,
    ///0x10 - Status register
    pub sr: SR,
    ///0x14 - Flash control register
    pub cr: CR,
    ///0x18 - Flash ECC register
    pub eccr: ECCR,
    _reserved6: [u8; 0x04],
    ///0x20 - Flash option register
    pub optr: OPTR,
    ///0x24 - Flash PCROP zone A Start address register
    pub pcrop1asr: PCROP1ASR,
    ///0x28 - Flash PCROP zone A End address register
    pub pcrop1aer: PCROP1AER,
    ///0x2c - Flash WRP area A address register
    pub wrp1ar: WRP1AR,
    ///0x30 - Flash WRP area B address register
    pub wrp1br: WRP1BR,
    ///0x34 - Flash PCROP zone B Start address register
    pub pcrop1bsr: PCROP1BSR,
    ///0x38 - Flash PCROP zone B End address register
    pub pcrop1ber: PCROP1BER,
}
///ACR (rw) register accessor: an alias for `Reg<ACR_SPEC>`
pub type ACR = crate::Reg<acr::ACR_SPEC>;
///Access control register
pub mod acr;
///KEYR (w) register accessor: an alias for `Reg<KEYR_SPEC>`
pub type KEYR = crate::Reg<keyr::KEYR_SPEC>;
///Flash key register
pub mod keyr;
///OPTKEYR (w) register accessor: an alias for `Reg<OPTKEYR_SPEC>`
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYR_SPEC>;
///Option byte key register
pub mod optkeyr;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///Status register
pub mod sr;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Flash control register
pub mod cr;
///ECCR (rw) register accessor: an alias for `Reg<ECCR_SPEC>`
pub type ECCR = crate::Reg<eccr::ECCR_SPEC>;
///Flash ECC register
pub mod eccr;
///OPTR (rw) register accessor: an alias for `Reg<OPTR_SPEC>`
pub type OPTR = crate::Reg<optr::OPTR_SPEC>;
///Flash option register
pub mod optr;
///PCROP1ASR (rw) register accessor: an alias for `Reg<PCROP1ASR_SPEC>`
pub type PCROP1ASR = crate::Reg<pcrop1asr::PCROP1ASR_SPEC>;
///Flash PCROP zone A Start address register
pub mod pcrop1asr;
///PCROP1AER (rw) register accessor: an alias for `Reg<PCROP1AER_SPEC>`
pub type PCROP1AER = crate::Reg<pcrop1aer::PCROP1AER_SPEC>;
///Flash PCROP zone A End address register
pub mod pcrop1aer;
///WRP1AR (rw) register accessor: an alias for `Reg<WRP1AR_SPEC>`
pub type WRP1AR = crate::Reg<wrp1ar::WRP1AR_SPEC>;
///Flash WRP area A address register
pub mod wrp1ar;
///WRP1BR (rw) register accessor: an alias for `Reg<WRP1BR_SPEC>`
pub type WRP1BR = crate::Reg<wrp1br::WRP1BR_SPEC>;
///Flash WRP area B address register
pub mod wrp1br;
///PCROP1BSR (rw) register accessor: an alias for `Reg<PCROP1BSR_SPEC>`
pub type PCROP1BSR = crate::Reg<pcrop1bsr::PCROP1BSR_SPEC>;
///Flash PCROP zone B Start address register
pub mod pcrop1bsr;
///PCROP1BER (rw) register accessor: an alias for `Reg<PCROP1BER_SPEC>`
pub type PCROP1BER = crate::Reg<pcrop1ber::PCROP1BER_SPEC>;
///Flash PCROP zone B End address register
pub mod pcrop1ber;
