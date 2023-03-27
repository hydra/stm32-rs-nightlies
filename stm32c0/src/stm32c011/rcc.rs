///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - RCC clock control register
    pub cr: CR,
    ///0x04 - RCC internal clock source calibration register
    pub icscr: ICSCR,
    ///0x08 - RCC clock configuration register
    pub cfgr: CFGR,
    _reserved3: [u8; 0x0c],
    ///0x18 - RCC clock interrupt enable register
    pub cier: CIER,
    ///0x1c - RCC clock interrupt flag register
    pub cifr: CIFR,
    ///0x20 - RCC clock interrupt clear register
    pub cicr: CICR,
    ///0x24 - RCC I/O port reset register
    pub ioprstr: IOPRSTR,
    ///0x28 - RCC AHB peripheral reset register
    pub ahbrstr: AHBRSTR,
    ///0x2c - RCC APB peripheral reset register 1
    pub apbrstr1: APBRSTR1,
    ///0x30 - RCC APB peripheral reset register 2
    pub apbrstr2: APBRSTR2,
    ///0x34 - RCC I/O port clock enable register
    pub iopenr: IOPENR,
    ///0x38 - RCC AHB peripheral clock enable register
    pub ahbenr: AHBENR,
    ///0x3c - RCC APB peripheral clock enable register 1
    pub apbenr1: APBENR1,
    ///0x40 - RCC APB peripheral clock enable register 2
    pub apbenr2: APBENR2,
    ///0x44 - RCC I/O port in Sleep mode clock enable register
    pub iopsmenr: IOPSMENR,
    ///0x48 - RCC AHB peripheral clock enable in Sleep/Stop mode register
    pub ahbsmenr: AHBSMENR,
    ///0x4c - RCC APB peripheral clock enable in Sleep/Stop mode register 1
    pub apbsmenr1: APBSMENR1,
    ///0x50 - RCC APB peripheral clock enable in Sleep/Stop mode register 2
    pub apbsmenr2: APBSMENR2,
    ///0x54 - RCC peripherals independent clock configuration register
    pub ccipr: CCIPR,
    _reserved19: [u8; 0x04],
    ///0x5c - RCC control/status register 1
    pub csr1: CSR1,
    ///0x60 - RCC control/status register 2
    pub csr2: CSR2,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///RCC clock control register
pub mod cr;
///ICSCR (rw) register accessor: an alias for `Reg<ICSCR_SPEC>`
pub type ICSCR = crate::Reg<icscr::ICSCR_SPEC>;
///RCC internal clock source calibration register
pub mod icscr;
///CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
///RCC clock configuration register
pub mod cfgr;
///CIER (rw) register accessor: an alias for `Reg<CIER_SPEC>`
pub type CIER = crate::Reg<cier::CIER_SPEC>;
///RCC clock interrupt enable register
pub mod cier;
///CIFR (r) register accessor: an alias for `Reg<CIFR_SPEC>`
pub type CIFR = crate::Reg<cifr::CIFR_SPEC>;
///RCC clock interrupt flag register
pub mod cifr;
///CICR (w) register accessor: an alias for `Reg<CICR_SPEC>`
pub type CICR = crate::Reg<cicr::CICR_SPEC>;
///RCC clock interrupt clear register
pub mod cicr;
///IOPRSTR (rw) register accessor: an alias for `Reg<IOPRSTR_SPEC>`
pub type IOPRSTR = crate::Reg<ioprstr::IOPRSTR_SPEC>;
///RCC I/O port reset register
pub mod ioprstr;
///AHBRSTR (rw) register accessor: an alias for `Reg<AHBRSTR_SPEC>`
pub type AHBRSTR = crate::Reg<ahbrstr::AHBRSTR_SPEC>;
///RCC AHB peripheral reset register
pub mod ahbrstr;
///APBRSTR1 (rw) register accessor: an alias for `Reg<APBRSTR1_SPEC>`
pub type APBRSTR1 = crate::Reg<apbrstr1::APBRSTR1_SPEC>;
///RCC APB peripheral reset register 1
pub mod apbrstr1;
///APBRSTR2 (rw) register accessor: an alias for `Reg<APBRSTR2_SPEC>`
pub type APBRSTR2 = crate::Reg<apbrstr2::APBRSTR2_SPEC>;
///RCC APB peripheral reset register 2
pub mod apbrstr2;
///IOPENR (rw) register accessor: an alias for `Reg<IOPENR_SPEC>`
pub type IOPENR = crate::Reg<iopenr::IOPENR_SPEC>;
///RCC I/O port clock enable register
pub mod iopenr;
///AHBENR (rw) register accessor: an alias for `Reg<AHBENR_SPEC>`
pub type AHBENR = crate::Reg<ahbenr::AHBENR_SPEC>;
///RCC AHB peripheral clock enable register
pub mod ahbenr;
///APBENR1 (rw) register accessor: an alias for `Reg<APBENR1_SPEC>`
pub type APBENR1 = crate::Reg<apbenr1::APBENR1_SPEC>;
///RCC APB peripheral clock enable register 1
pub mod apbenr1;
///APBENR2 (rw) register accessor: an alias for `Reg<APBENR2_SPEC>`
pub type APBENR2 = crate::Reg<apbenr2::APBENR2_SPEC>;
///RCC APB peripheral clock enable register 2
pub mod apbenr2;
///IOPSMENR (rw) register accessor: an alias for `Reg<IOPSMENR_SPEC>`
pub type IOPSMENR = crate::Reg<iopsmenr::IOPSMENR_SPEC>;
///RCC I/O port in Sleep mode clock enable register
pub mod iopsmenr;
///AHBSMENR (rw) register accessor: an alias for `Reg<AHBSMENR_SPEC>`
pub type AHBSMENR = crate::Reg<ahbsmenr::AHBSMENR_SPEC>;
///RCC AHB peripheral clock enable in Sleep/Stop mode register
pub mod ahbsmenr;
///APBSMENR1 (rw) register accessor: an alias for `Reg<APBSMENR1_SPEC>`
pub type APBSMENR1 = crate::Reg<apbsmenr1::APBSMENR1_SPEC>;
///RCC APB peripheral clock enable in Sleep/Stop mode register 1
pub mod apbsmenr1;
///APBSMENR2 (rw) register accessor: an alias for `Reg<APBSMENR2_SPEC>`
pub type APBSMENR2 = crate::Reg<apbsmenr2::APBSMENR2_SPEC>;
///RCC APB peripheral clock enable in Sleep/Stop mode register 2
pub mod apbsmenr2;
///CCIPR (rw) register accessor: an alias for `Reg<CCIPR_SPEC>`
pub type CCIPR = crate::Reg<ccipr::CCIPR_SPEC>;
///RCC peripherals independent clock configuration register
pub mod ccipr;
///CSR1 (rw) register accessor: an alias for `Reg<CSR1_SPEC>`
pub type CSR1 = crate::Reg<csr1::CSR1_SPEC>;
///RCC control/status register 1
pub mod csr1;
///CSR2 (rw) register accessor: an alias for `Reg<CSR2_SPEC>`
pub type CSR2 = crate::Reg<csr2::CSR2_SPEC>;
///RCC control/status register 2
pub mod csr2;
