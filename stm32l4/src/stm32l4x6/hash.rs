///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: CR,
    ///0x04 - data input register
    pub din: DIN,
    ///0x08 - start register
    pub str: STR,
    ///0x0c - digest registers
    pub hr: [HR; 1],
    _reserved4: [u8; 0x10],
    ///0x20 - interrupt enable register
    pub imr: IMR,
    ///0x24 - status register
    pub sr: SR,
    _reserved6: [u8; 0xd0],
    ///0xf8..0x1d0 - context swap registers
    pub csr: [CSR; 54],
    _reserved7: [u8; 0x0140],
    ///0x310..0x330 - HASH digest register
    pub hash_hr: [HASH_HR; 8],
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///control register
pub mod cr;
///DIN (rw) register accessor: an alias for `Reg<DIN_SPEC>`
pub type DIN = crate::Reg<din::DIN_SPEC>;
///data input register
pub mod din;
///STR (rw) register accessor: an alias for `Reg<STR_SPEC>`
pub type STR = crate::Reg<str::STR_SPEC>;
///start register
pub mod str;
///HR (r) register accessor: an alias for `Reg<HR_SPEC>`
pub type HR = crate::Reg<hr::HR_SPEC>;
///digest registers
pub mod hr;
///IMR (rw) register accessor: an alias for `Reg<IMR_SPEC>`
pub type IMR = crate::Reg<imr::IMR_SPEC>;
///interrupt enable register
pub mod imr;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///status register
pub mod sr;
///CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///context swap registers
pub mod csr;
///HASH_HR (r) register accessor: an alias for `Reg<HASH_HR_SPEC>`
pub type HASH_HR = crate::Reg<hash_hr::HASH_HR_SPEC>;
///HASH digest register
pub mod hash_hr;
