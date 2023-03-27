///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - RCC clock control register
    pub rcc_cr: RCC_CR,
    _reserved1: [u8; 0x04],
    ///0x08 - RCC internal clock sources calibration register 1
    pub rcc_icscr1: RCC_ICSCR1,
    ///0x0c - RCC internal clock sources calibration register 2
    pub rcc_icscr2: RCC_ICSCR2,
    ///0x10 - RCC internal clock sources calibration register 3
    pub rcc_icscr3: RCC_ICSCR3,
    ///0x14 - RCC clock recovery RC register
    pub rcc_crrcr: RCC_CRRCR,
    _reserved5: [u8; 0x04],
    ///0x1c - RCC clock configuration register 1
    pub rcc_cfgr1: RCC_CFGR1,
    ///0x20 - RCC clock configuration register 2
    pub rcc_cfgr2: RCC_CFGR2,
    ///0x24 - RCC clock configuration register 3
    pub rcc_cfgr3: RCC_CFGR3,
    ///0x28 - RCC PLL1 configuration register
    pub rcc_pll1cfgr: RCC_PLL1CFGR,
    ///0x2c - RCC PLL2 configuration register
    pub rcc_pll2cfgr: RCC_PLL2CFGR,
    ///0x30 - RCC PLL3 configuration register
    pub rcc_pll3cfgr: RCC_PLL3CFGR,
    ///0x34 - RCC PLL1 dividers register
    pub rcc_pll1divr: RCC_PLL1DIVR,
    ///0x38 - RCC PLL1 fractional divider register
    pub rcc_pll1fracr: RCC_PLL1FRACR,
    ///0x3c - RCC PLL2 dividers configuration register
    pub rcc_pll2divr: RCC_PLL2DIVR,
    ///0x40 - RCC PLL2 fractional divider register
    pub rcc_pll2fracr: RCC_PLL2FRACR,
    ///0x44 - RCC PLL3 dividers configuration register
    pub rcc_pll3divr: RCC_PLL3DIVR,
    ///0x48 - RCC PLL3 fractional divider register
    pub rcc_pll3fracr: RCC_PLL3FRACR,
    _reserved17: [u8; 0x04],
    ///0x50 - RCC clock interrupt enable register
    pub rcc_cier: RCC_CIER,
    ///0x54 - RCC clock interrupt flag register
    pub rcc_cifr: RCC_CIFR,
    ///0x58 - RCC clock interrupt clear register
    pub rcc_cicr: RCC_CICR,
    _reserved20: [u8; 0x04],
    ///0x60 - RCC AHB1 peripheral reset register
    pub rcc_ahb1rstr: RCC_AHB1RSTR,
    ///0x64 - RCC AHB2 peripheral reset register 1
    pub rcc_ahb2rstr1: RCC_AHB2RSTR1,
    ///0x68 - RCC AHB2 peripheral reset register 2
    pub rcc_ahb2rstr2: RCC_AHB2RSTR2,
    ///0x6c - RCC AHB3 peripheral reset register
    pub rcc_ahb3rstr: RCC_AHB3RSTR,
    _reserved24: [u8; 0x04],
    ///0x74 - RCC APB1 peripheral reset register 1
    pub rcc_apb1rstr1: RCC_APB1RSTR1,
    ///0x78 - RCC APB1 peripheral reset register 2
    pub rcc_apb1rstr2: RCC_APB1RSTR2,
    ///0x7c - RCC APB2 peripheral reset register
    pub rcc_apb2rstr: RCC_APB2RSTR,
    ///0x80 - RCC APB3 peripheral reset register
    pub rcc_apb3rstr: RCC_APB3RSTR,
    _reserved28: [u8; 0x04],
    ///0x88 - RCC AHB1 peripheral clock enable register
    pub rcc_ahb1enr: RCC_AHB1ENR,
    ///0x8c - RCC AHB2 peripheral clock enable register 1
    pub rcc_ahb2enr1: RCC_AHB2ENR1,
    ///0x90 - RCC AHB2 peripheral clock enable register 2
    pub rcc_ahb2enr2: RCC_AHB2ENR2,
    ///0x94 - RCC AHB3 peripheral clock enable register
    pub rcc_ahb3enr: RCC_AHB3ENR,
    _reserved32: [u8; 0x04],
    ///0x9c - RCC APB1 peripheral clock enable register 1
    pub rcc_apb1enr1: RCC_APB1ENR1,
    ///0xa0 - RCC APB1 peripheral clock enable register 2
    pub rcc_apb1enr2: RCC_APB1ENR2,
    ///0xa4 - RCC APB2 peripheral clock enable register
    pub rcc_apb2enr: RCC_APB2ENR,
    ///0xa8 - RCC APB3 peripheral clock enable register
    pub rcc_apb3enr: RCC_APB3ENR,
    _reserved36: [u8; 0x04],
    ///0xb0 - RCC AHB1 peripheral clocks enable in Sleep and Stop modes register
    pub rcc_ahb1smenr: RCC_AHB1SMENR,
    ///0xb4 - RCC AHB2 peripheral clocks enable in Sleep and Stop modes register 1
    pub rcc_ahb2smenr1: RCC_AHB2SMENR1,
    ///0xb8 - RCC AHB2 peripheral clocks enable in Sleep and Stop modes register 2
    pub rcc_ahb2smenr2: RCC_AHB2SMENR2,
    ///0xbc - RCC AHB3 peripheral clocks enable in Sleep and Stop modes register
    pub rcc_ahb3smenr: RCC_AHB3SMENR,
    _reserved40: [u8; 0x04],
    ///0xc4 - RCC APB1 peripheral clocks enable in Sleep and Stop modes register 1
    pub rcc_apb1smenr1: RCC_APB1SMENR1,
    ///0xc8 - RCC APB1 peripheral clocks enable in Sleep and Stop modes register 2
    pub rcc_apb1smenr2: RCC_APB1SMENR2,
    ///0xcc - RCC APB2 peripheral clocks enable in Sleep and Stop modes register
    pub rcc_apb2smenr: RCC_APB2SMENR,
    ///0xd0 - RCC APB3 peripheral clock enable in Sleep and Stop modes register
    pub rcc_apb3smenr: RCC_APB3SMENR,
    _reserved44: [u8; 0x04],
    ///0xd8 - RCC SmartRun domain peripheral autonomous mode register
    pub rcc_srdamr: RCC_SRDAMR,
    _reserved45: [u8; 0x04],
    ///0xe0 - RCC peripherals independent clock configuration register 1
    pub rcc_ccipr1: RCC_CCIPR1,
    ///0xe4 - RCC peripherals independent clock configuration register 2
    pub rcc_ccipr2: RCC_CCIPR2,
    ///0xe8 - RCC peripherals independent clock configuration register 3
    pub rcc_ccipr3: RCC_CCIPR3,
    _reserved48: [u8; 0x04],
    ///0xf0 - RCC Backup domain control register
    pub rcc_bdcr: RCC_BDCR,
    ///0xf4 - RCC control/status register
    pub rcc_csr: RCC_CSR,
    _reserved50: [u8; 0x18],
    ///0x110 - RCC secure configuration register
    pub rcc_seccfgr: RCC_SECCFGR,
    ///0x114 - RCC privilege configuration register
    pub rcc_privcfgr: RCC_PRIVCFGR,
}
///RCC_CR (rw) register accessor: an alias for `Reg<RCC_CR_SPEC>`
pub type RCC_CR = crate::Reg<rcc_cr::RCC_CR_SPEC>;
///RCC clock control register
pub mod rcc_cr;
///RCC_ICSCR1 (rw) register accessor: an alias for `Reg<RCC_ICSCR1_SPEC>`
pub type RCC_ICSCR1 = crate::Reg<rcc_icscr1::RCC_ICSCR1_SPEC>;
///RCC internal clock sources calibration register 1
pub mod rcc_icscr1;
///RCC_ICSCR2 (rw) register accessor: an alias for `Reg<RCC_ICSCR2_SPEC>`
pub type RCC_ICSCR2 = crate::Reg<rcc_icscr2::RCC_ICSCR2_SPEC>;
///RCC internal clock sources calibration register 2
pub mod rcc_icscr2;
///RCC_ICSCR3 (rw) register accessor: an alias for `Reg<RCC_ICSCR3_SPEC>`
pub type RCC_ICSCR3 = crate::Reg<rcc_icscr3::RCC_ICSCR3_SPEC>;
///RCC internal clock sources calibration register 3
pub mod rcc_icscr3;
///RCC_CRRCR (r) register accessor: an alias for `Reg<RCC_CRRCR_SPEC>`
pub type RCC_CRRCR = crate::Reg<rcc_crrcr::RCC_CRRCR_SPEC>;
///RCC clock recovery RC register
pub mod rcc_crrcr;
///RCC_CFGR1 (rw) register accessor: an alias for `Reg<RCC_CFGR1_SPEC>`
pub type RCC_CFGR1 = crate::Reg<rcc_cfgr1::RCC_CFGR1_SPEC>;
///RCC clock configuration register 1
pub mod rcc_cfgr1;
///RCC_CFGR2 (rw) register accessor: an alias for `Reg<RCC_CFGR2_SPEC>`
pub type RCC_CFGR2 = crate::Reg<rcc_cfgr2::RCC_CFGR2_SPEC>;
///RCC clock configuration register 2
pub mod rcc_cfgr2;
///RCC_CFGR3 (rw) register accessor: an alias for `Reg<RCC_CFGR3_SPEC>`
pub type RCC_CFGR3 = crate::Reg<rcc_cfgr3::RCC_CFGR3_SPEC>;
///RCC clock configuration register 3
pub mod rcc_cfgr3;
///RCC_PLL1CFGR (rw) register accessor: an alias for `Reg<RCC_PLL1CFGR_SPEC>`
pub type RCC_PLL1CFGR = crate::Reg<rcc_pll1cfgr::RCC_PLL1CFGR_SPEC>;
///RCC PLL1 configuration register
pub mod rcc_pll1cfgr;
///RCC_PLL2CFGR (rw) register accessor: an alias for `Reg<RCC_PLL2CFGR_SPEC>`
pub type RCC_PLL2CFGR = crate::Reg<rcc_pll2cfgr::RCC_PLL2CFGR_SPEC>;
///RCC PLL2 configuration register
pub mod rcc_pll2cfgr;
///RCC_PLL3CFGR (rw) register accessor: an alias for `Reg<RCC_PLL3CFGR_SPEC>`
pub type RCC_PLL3CFGR = crate::Reg<rcc_pll3cfgr::RCC_PLL3CFGR_SPEC>;
///RCC PLL3 configuration register
pub mod rcc_pll3cfgr;
///RCC_PLL1DIVR (rw) register accessor: an alias for `Reg<RCC_PLL1DIVR_SPEC>`
pub type RCC_PLL1DIVR = crate::Reg<rcc_pll1divr::RCC_PLL1DIVR_SPEC>;
///RCC PLL1 dividers register
pub mod rcc_pll1divr;
///RCC_PLL1FRACR (rw) register accessor: an alias for `Reg<RCC_PLL1FRACR_SPEC>`
pub type RCC_PLL1FRACR = crate::Reg<rcc_pll1fracr::RCC_PLL1FRACR_SPEC>;
///RCC PLL1 fractional divider register
pub mod rcc_pll1fracr;
///RCC_PLL2DIVR (rw) register accessor: an alias for `Reg<RCC_PLL2DIVR_SPEC>`
pub type RCC_PLL2DIVR = crate::Reg<rcc_pll2divr::RCC_PLL2DIVR_SPEC>;
///RCC PLL2 dividers configuration register
pub mod rcc_pll2divr;
///RCC_PLL2FRACR (rw) register accessor: an alias for `Reg<RCC_PLL2FRACR_SPEC>`
pub type RCC_PLL2FRACR = crate::Reg<rcc_pll2fracr::RCC_PLL2FRACR_SPEC>;
///RCC PLL2 fractional divider register
pub mod rcc_pll2fracr;
///RCC_PLL3DIVR (rw) register accessor: an alias for `Reg<RCC_PLL3DIVR_SPEC>`
pub type RCC_PLL3DIVR = crate::Reg<rcc_pll3divr::RCC_PLL3DIVR_SPEC>;
///RCC PLL3 dividers configuration register
pub mod rcc_pll3divr;
///RCC_PLL3FRACR (rw) register accessor: an alias for `Reg<RCC_PLL3FRACR_SPEC>`
pub type RCC_PLL3FRACR = crate::Reg<rcc_pll3fracr::RCC_PLL3FRACR_SPEC>;
///RCC PLL3 fractional divider register
pub mod rcc_pll3fracr;
///RCC_CIER (rw) register accessor: an alias for `Reg<RCC_CIER_SPEC>`
pub type RCC_CIER = crate::Reg<rcc_cier::RCC_CIER_SPEC>;
///RCC clock interrupt enable register
pub mod rcc_cier;
///RCC_CIFR (r) register accessor: an alias for `Reg<RCC_CIFR_SPEC>`
pub type RCC_CIFR = crate::Reg<rcc_cifr::RCC_CIFR_SPEC>;
///RCC clock interrupt flag register
pub mod rcc_cifr;
///RCC_CICR (w) register accessor: an alias for `Reg<RCC_CICR_SPEC>`
pub type RCC_CICR = crate::Reg<rcc_cicr::RCC_CICR_SPEC>;
///RCC clock interrupt clear register
pub mod rcc_cicr;
///RCC_AHB1RSTR (rw) register accessor: an alias for `Reg<RCC_AHB1RSTR_SPEC>`
pub type RCC_AHB1RSTR = crate::Reg<rcc_ahb1rstr::RCC_AHB1RSTR_SPEC>;
///RCC AHB1 peripheral reset register
pub mod rcc_ahb1rstr;
///RCC_AHB2RSTR1 (rw) register accessor: an alias for `Reg<RCC_AHB2RSTR1_SPEC>`
pub type RCC_AHB2RSTR1 = crate::Reg<rcc_ahb2rstr1::RCC_AHB2RSTR1_SPEC>;
///RCC AHB2 peripheral reset register 1
pub mod rcc_ahb2rstr1;
///RCC_AHB2RSTR2 (rw) register accessor: an alias for `Reg<RCC_AHB2RSTR2_SPEC>`
pub type RCC_AHB2RSTR2 = crate::Reg<rcc_ahb2rstr2::RCC_AHB2RSTR2_SPEC>;
///RCC AHB2 peripheral reset register 2
pub mod rcc_ahb2rstr2;
///RCC_AHB3RSTR (rw) register accessor: an alias for `Reg<RCC_AHB3RSTR_SPEC>`
pub type RCC_AHB3RSTR = crate::Reg<rcc_ahb3rstr::RCC_AHB3RSTR_SPEC>;
///RCC AHB3 peripheral reset register
pub mod rcc_ahb3rstr;
///RCC_APB1RSTR1 (rw) register accessor: an alias for `Reg<RCC_APB1RSTR1_SPEC>`
pub type RCC_APB1RSTR1 = crate::Reg<rcc_apb1rstr1::RCC_APB1RSTR1_SPEC>;
///RCC APB1 peripheral reset register 1
pub mod rcc_apb1rstr1;
///RCC_APB1RSTR2 (rw) register accessor: an alias for `Reg<RCC_APB1RSTR2_SPEC>`
pub type RCC_APB1RSTR2 = crate::Reg<rcc_apb1rstr2::RCC_APB1RSTR2_SPEC>;
///RCC APB1 peripheral reset register 2
pub mod rcc_apb1rstr2;
///RCC_APB2RSTR (rw) register accessor: an alias for `Reg<RCC_APB2RSTR_SPEC>`
pub type RCC_APB2RSTR = crate::Reg<rcc_apb2rstr::RCC_APB2RSTR_SPEC>;
///RCC APB2 peripheral reset register
pub mod rcc_apb2rstr;
///RCC_APB3RSTR (rw) register accessor: an alias for `Reg<RCC_APB3RSTR_SPEC>`
pub type RCC_APB3RSTR = crate::Reg<rcc_apb3rstr::RCC_APB3RSTR_SPEC>;
///RCC APB3 peripheral reset register
pub mod rcc_apb3rstr;
///RCC_AHB1ENR (rw) register accessor: an alias for `Reg<RCC_AHB1ENR_SPEC>`
pub type RCC_AHB1ENR = crate::Reg<rcc_ahb1enr::RCC_AHB1ENR_SPEC>;
///RCC AHB1 peripheral clock enable register
pub mod rcc_ahb1enr;
///RCC_AHB2ENR1 (rw) register accessor: an alias for `Reg<RCC_AHB2ENR1_SPEC>`
pub type RCC_AHB2ENR1 = crate::Reg<rcc_ahb2enr1::RCC_AHB2ENR1_SPEC>;
///RCC AHB2 peripheral clock enable register 1
pub mod rcc_ahb2enr1;
///RCC_AHB2ENR2 (rw) register accessor: an alias for `Reg<RCC_AHB2ENR2_SPEC>`
pub type RCC_AHB2ENR2 = crate::Reg<rcc_ahb2enr2::RCC_AHB2ENR2_SPEC>;
///RCC AHB2 peripheral clock enable register 2
pub mod rcc_ahb2enr2;
///RCC_AHB3ENR (rw) register accessor: an alias for `Reg<RCC_AHB3ENR_SPEC>`
pub type RCC_AHB3ENR = crate::Reg<rcc_ahb3enr::RCC_AHB3ENR_SPEC>;
///RCC AHB3 peripheral clock enable register
pub mod rcc_ahb3enr;
///RCC_APB1ENR1 (rw) register accessor: an alias for `Reg<RCC_APB1ENR1_SPEC>`
pub type RCC_APB1ENR1 = crate::Reg<rcc_apb1enr1::RCC_APB1ENR1_SPEC>;
///RCC APB1 peripheral clock enable register 1
pub mod rcc_apb1enr1;
///RCC_APB1ENR2 (rw) register accessor: an alias for `Reg<RCC_APB1ENR2_SPEC>`
pub type RCC_APB1ENR2 = crate::Reg<rcc_apb1enr2::RCC_APB1ENR2_SPEC>;
///RCC APB1 peripheral clock enable register 2
pub mod rcc_apb1enr2;
///RCC_APB2ENR (rw) register accessor: an alias for `Reg<RCC_APB2ENR_SPEC>`
pub type RCC_APB2ENR = crate::Reg<rcc_apb2enr::RCC_APB2ENR_SPEC>;
///RCC APB2 peripheral clock enable register
pub mod rcc_apb2enr;
///RCC_APB3ENR (rw) register accessor: an alias for `Reg<RCC_APB3ENR_SPEC>`
pub type RCC_APB3ENR = crate::Reg<rcc_apb3enr::RCC_APB3ENR_SPEC>;
///RCC APB3 peripheral clock enable register
pub mod rcc_apb3enr;
///RCC_AHB1SMENR (rw) register accessor: an alias for `Reg<RCC_AHB1SMENR_SPEC>`
pub type RCC_AHB1SMENR = crate::Reg<rcc_ahb1smenr::RCC_AHB1SMENR_SPEC>;
///RCC AHB1 peripheral clocks enable in Sleep and Stop modes register
pub mod rcc_ahb1smenr;
///RCC_AHB2SMENR1 (rw) register accessor: an alias for `Reg<RCC_AHB2SMENR1_SPEC>`
pub type RCC_AHB2SMENR1 = crate::Reg<rcc_ahb2smenr1::RCC_AHB2SMENR1_SPEC>;
///RCC AHB2 peripheral clocks enable in Sleep and Stop modes register 1
pub mod rcc_ahb2smenr1;
///RCC_AHB2SMENR2 (rw) register accessor: an alias for `Reg<RCC_AHB2SMENR2_SPEC>`
pub type RCC_AHB2SMENR2 = crate::Reg<rcc_ahb2smenr2::RCC_AHB2SMENR2_SPEC>;
///RCC AHB2 peripheral clocks enable in Sleep and Stop modes register 2
pub mod rcc_ahb2smenr2;
///RCC_AHB3SMENR (rw) register accessor: an alias for `Reg<RCC_AHB3SMENR_SPEC>`
pub type RCC_AHB3SMENR = crate::Reg<rcc_ahb3smenr::RCC_AHB3SMENR_SPEC>;
///RCC AHB3 peripheral clocks enable in Sleep and Stop modes register
pub mod rcc_ahb3smenr;
///RCC_APB1SMENR1 (rw) register accessor: an alias for `Reg<RCC_APB1SMENR1_SPEC>`
pub type RCC_APB1SMENR1 = crate::Reg<rcc_apb1smenr1::RCC_APB1SMENR1_SPEC>;
///RCC APB1 peripheral clocks enable in Sleep and Stop modes register 1
pub mod rcc_apb1smenr1;
///RCC_APB1SMENR2 (rw) register accessor: an alias for `Reg<RCC_APB1SMENR2_SPEC>`
pub type RCC_APB1SMENR2 = crate::Reg<rcc_apb1smenr2::RCC_APB1SMENR2_SPEC>;
///RCC APB1 peripheral clocks enable in Sleep and Stop modes register 2
pub mod rcc_apb1smenr2;
///RCC_APB2SMENR (rw) register accessor: an alias for `Reg<RCC_APB2SMENR_SPEC>`
pub type RCC_APB2SMENR = crate::Reg<rcc_apb2smenr::RCC_APB2SMENR_SPEC>;
///RCC APB2 peripheral clocks enable in Sleep and Stop modes register
pub mod rcc_apb2smenr;
///RCC_APB3SMENR (rw) register accessor: an alias for `Reg<RCC_APB3SMENR_SPEC>`
pub type RCC_APB3SMENR = crate::Reg<rcc_apb3smenr::RCC_APB3SMENR_SPEC>;
///RCC APB3 peripheral clock enable in Sleep and Stop modes register
pub mod rcc_apb3smenr;
///RCC_SRDAMR (rw) register accessor: an alias for `Reg<RCC_SRDAMR_SPEC>`
pub type RCC_SRDAMR = crate::Reg<rcc_srdamr::RCC_SRDAMR_SPEC>;
///RCC SmartRun domain peripheral autonomous mode register
pub mod rcc_srdamr;
///RCC_CCIPR1 (rw) register accessor: an alias for `Reg<RCC_CCIPR1_SPEC>`
pub type RCC_CCIPR1 = crate::Reg<rcc_ccipr1::RCC_CCIPR1_SPEC>;
///RCC peripherals independent clock configuration register 1
pub mod rcc_ccipr1;
///RCC_CCIPR2 (rw) register accessor: an alias for `Reg<RCC_CCIPR2_SPEC>`
pub type RCC_CCIPR2 = crate::Reg<rcc_ccipr2::RCC_CCIPR2_SPEC>;
///RCC peripherals independent clock configuration register 2
pub mod rcc_ccipr2;
///RCC_CCIPR3 (rw) register accessor: an alias for `Reg<RCC_CCIPR3_SPEC>`
pub type RCC_CCIPR3 = crate::Reg<rcc_ccipr3::RCC_CCIPR3_SPEC>;
///RCC peripherals independent clock configuration register 3
pub mod rcc_ccipr3;
///RCC_BDCR (rw) register accessor: an alias for `Reg<RCC_BDCR_SPEC>`
pub type RCC_BDCR = crate::Reg<rcc_bdcr::RCC_BDCR_SPEC>;
///RCC Backup domain control register
pub mod rcc_bdcr;
///RCC_CSR (rw) register accessor: an alias for `Reg<RCC_CSR_SPEC>`
pub type RCC_CSR = crate::Reg<rcc_csr::RCC_CSR_SPEC>;
///RCC control/status register
pub mod rcc_csr;
///RCC_SECCFGR (rw) register accessor: an alias for `Reg<RCC_SECCFGR_SPEC>`
pub type RCC_SECCFGR = crate::Reg<rcc_seccfgr::RCC_SECCFGR_SPEC>;
///RCC secure configuration register
pub mod rcc_seccfgr;
///RCC_PRIVCFGR (rw) register accessor: an alias for `Reg<RCC_PRIVCFGR_SPEC>`
pub type RCC_PRIVCFGR = crate::Reg<rcc_privcfgr::RCC_PRIVCFGR_SPEC>;
///RCC privilege configuration register
pub mod rcc_privcfgr;
