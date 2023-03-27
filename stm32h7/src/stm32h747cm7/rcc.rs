///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - clock control register
    pub cr: CR,
    ///0x04 - RCC HSI configuration register
    pub hsicfgr: HSICFGR,
    ///0x08 - RCC Clock Recovery RC Register
    pub crrcr: CRRCR,
    ///0x0c - RCC CSI configuration register
    pub csicfgr: CSICFGR,
    ///0x10 - RCC Clock Configuration Register
    pub cfgr: CFGR,
    _reserved5: [u8; 0x04],
    ///0x18 - RCC Domain 1 Clock Configuration Register
    pub d1cfgr: D1CFGR,
    ///0x1c - RCC Domain 2 Clock Configuration Register
    pub d2cfgr: D2CFGR,
    ///0x20 - RCC Domain 3 Clock Configuration Register
    pub d3cfgr: D3CFGR,
    _reserved8: [u8; 0x04],
    ///0x28 - RCC PLLs Clock Source Selection Register
    pub pllckselr: PLLCKSELR,
    ///0x2c - RCC PLLs Configuration Register
    pub pllcfgr: PLLCFGR,
    ///0x30 - RCC PLL1 Dividers Configuration Register
    pub pll1divr: PLL1DIVR,
    ///0x34 - RCC PLL1 Fractional Divider Register
    pub pll1fracr: PLL1FRACR,
    ///0x38 - RCC PLL2 Dividers Configuration Register
    pub pll2divr: PLL2DIVR,
    ///0x3c - RCC PLL2 Fractional Divider Register
    pub pll2fracr: PLL2FRACR,
    ///0x40 - RCC PLL3 Dividers Configuration Register
    pub pll3divr: PLL3DIVR,
    ///0x44 - RCC PLL3 Fractional Divider Register
    pub pll3fracr: PLL3FRACR,
    _reserved16: [u8; 0x04],
    ///0x4c - RCC Domain 1 Kernel Clock Configuration Register
    pub d1ccipr: D1CCIPR,
    ///0x50 - RCC Domain 2 Kernel Clock Configuration Register
    pub d2ccip1r: D2CCIP1R,
    ///0x54 - RCC Domain 2 Kernel Clock Configuration Register
    pub d2ccip2r: D2CCIP2R,
    ///0x58 - RCC Domain 3 Kernel Clock Configuration Register
    pub d3ccipr: D3CCIPR,
    _reserved20: [u8; 0x04],
    ///0x60 - RCC Clock Source Interrupt Enable Register
    pub cier: CIER,
    ///0x64 - RCC Clock Source Interrupt Flag Register
    pub cifr: CIFR,
    ///0x68 - RCC Clock Source Interrupt Clear Register
    pub cicr: CICR,
    _reserved23: [u8; 0x04],
    ///0x70 - RCC Backup Domain Control Register
    pub bdcr: BDCR,
    ///0x74 - RCC Clock Control and Status Register
    pub csr: CSR,
    _reserved25: [u8; 0x04],
    ///0x7c - RCC AHB3 Reset Register
    pub ahb3rstr: AHB3RSTR,
    ///0x80 - RCC AHB1 Peripheral Reset Register
    pub ahb1rstr: AHB1RSTR,
    ///0x84 - RCC AHB2 Peripheral Reset Register
    pub ahb2rstr: AHB2RSTR,
    ///0x88 - RCC AHB4 Peripheral Reset Register
    pub ahb4rstr: AHB4RSTR,
    ///0x8c - RCC APB3 Peripheral Reset Register
    pub apb3rstr: APB3RSTR,
    ///0x90 - RCC APB1 Peripheral Reset Register
    pub apb1lrstr: APB1LRSTR,
    ///0x94 - RCC APB1 Peripheral Reset Register
    pub apb1hrstr: APB1HRSTR,
    ///0x98 - RCC APB2 Peripheral Reset Register
    pub apb2rstr: APB2RSTR,
    ///0x9c - RCC APB4 Peripheral Reset Register
    pub apb4rstr: APB4RSTR,
    ///0xa0 - RCC Global Control Register
    pub gcr: GCR,
    _reserved35: [u8; 0x04],
    ///0xa8 - RCC D3 Autonomous mode Register
    pub d3amr: D3AMR,
    _reserved36: [u8; 0x24],
    ///0xd0 - RCC Reset Status Register
    pub rsr: RSR,
    ///0xd4 - RCC AHB3 Clock Register
    pub ahb3enr: AHB3ENR,
    ///0xd8 - RCC AHB1 Clock Register
    pub ahb1enr: AHB1ENR,
    ///0xdc - RCC AHB2 Clock Register
    pub ahb2enr: AHB2ENR,
    ///0xe0 - RCC AHB4 Clock Register
    pub ahb4enr: AHB4ENR,
    ///0xe4 - RCC APB3 Clock Register
    pub apb3enr: APB3ENR,
    ///0xe8 - RCC APB1 Clock Register
    pub apb1lenr: APB1LENR,
    ///0xec - RCC APB1 Clock Register
    pub apb1henr: APB1HENR,
    ///0xf0 - RCC APB2 Clock Register
    pub apb2enr: APB2ENR,
    ///0xf4 - RCC APB4 Clock Register
    pub apb4enr: APB4ENR,
    _reserved46: [u8; 0x04],
    ///0xfc - RCC AHB3 Sleep Clock Register
    pub ahb3lpenr: AHB3LPENR,
    ///0x100 - RCC AHB1 Sleep Clock Register
    pub ahb1lpenr: AHB1LPENR,
    ///0x104 - RCC AHB2 Sleep Clock Register
    pub ahb2lpenr: AHB2LPENR,
    ///0x108 - RCC AHB4 Sleep Clock Register
    pub ahb4lpenr: AHB4LPENR,
    ///0x10c - RCC APB3 Sleep Clock Register
    pub apb3lpenr: APB3LPENR,
    ///0x110 - RCC APB1 Low Sleep Clock Register
    pub apb1llpenr: APB1LLPENR,
    ///0x114 - RCC APB1 High Sleep Clock Register
    pub apb1hlpenr: APB1HLPENR,
    ///0x118 - RCC APB2 Sleep Clock Register
    pub apb2lpenr: APB2LPENR,
    ///0x11c - RCC APB4 Sleep Clock Register
    pub apb4lpenr: APB4LPENR,
    _reserved55: [u8; 0x10],
    ///0x130 - RCC Reset Status Register
    pub c1_rsr: C1_RSR,
    ///0x134 - RCC AHB3 Clock Register
    pub c1_ahb3enr: C1_AHB3ENR,
    ///0x138 - RCC AHB1 Clock Register
    pub c1_ahb1enr: C1_AHB1ENR,
    ///0x13c - RCC AHB2 Clock Register
    pub c1_ahb2enr: C1_AHB2ENR,
    ///0x140 - RCC AHB4 Clock Register
    pub c1_ahb4enr: C1_AHB4ENR,
    ///0x144 - RCC APB3 Clock Register
    pub c1_apb3enr: C1_APB3ENR,
    ///0x148 - RCC APB1 Clock Register
    pub c1_apb1lenr: C1_APB1LENR,
    ///0x14c - RCC APB1 Clock Register
    pub c1_apb1henr: C1_APB1HENR,
    ///0x150 - RCC APB2 Clock Register
    pub c1_apb2enr: C1_APB2ENR,
    ///0x154 - RCC APB4 Clock Register
    pub c1_apb4enr: C1_APB4ENR,
    _reserved65: [u8; 0x04],
    ///0x15c - RCC AHB3 Sleep Clock Register
    pub c1_ahb3lpenr: C1_AHB3LPENR,
    ///0x160 - RCC AHB1 Sleep Clock Register
    pub c1_ahb1lpenr: C1_AHB1LPENR,
    ///0x164 - RCC AHB2 Sleep Clock Register
    pub c1_ahb2lpenr: C1_AHB2LPENR,
    ///0x168 - RCC AHB4 Sleep Clock Register
    pub c1_ahb4lpenr: C1_AHB4LPENR,
    ///0x16c - RCC APB3 Sleep Clock Register
    pub c1_apb3lpenr: C1_APB3LPENR,
    ///0x170 - RCC APB1 Low Sleep Clock Register
    pub c1_apb1llpenr: C1_APB1LLPENR,
    ///0x174 - RCC APB1 High Sleep Clock Register
    pub c1_apb1hlpenr: C1_APB1HLPENR,
    ///0x178 - RCC APB2 Sleep Clock Register
    pub c1_apb2lpenr: C1_APB2LPENR,
    ///0x17c - RCC APB4 Sleep Clock Register
    pub c1_apb4lpenr: C1_APB4LPENR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///clock control register
pub mod cr;
///CRRCR (r) register accessor: an alias for `Reg<CRRCR_SPEC>`
pub type CRRCR = crate::Reg<crrcr::CRRCR_SPEC>;
///RCC Clock Recovery RC Register
pub mod crrcr;
///CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
///RCC Clock Configuration Register
pub mod cfgr;
///D1CFGR (rw) register accessor: an alias for `Reg<D1CFGR_SPEC>`
pub type D1CFGR = crate::Reg<d1cfgr::D1CFGR_SPEC>;
///RCC Domain 1 Clock Configuration Register
pub mod d1cfgr;
///D2CFGR (rw) register accessor: an alias for `Reg<D2CFGR_SPEC>`
pub type D2CFGR = crate::Reg<d2cfgr::D2CFGR_SPEC>;
///RCC Domain 2 Clock Configuration Register
pub mod d2cfgr;
///D3CFGR (rw) register accessor: an alias for `Reg<D3CFGR_SPEC>`
pub type D3CFGR = crate::Reg<d3cfgr::D3CFGR_SPEC>;
///RCC Domain 3 Clock Configuration Register
pub mod d3cfgr;
///PLLCKSELR (rw) register accessor: an alias for `Reg<PLLCKSELR_SPEC>`
pub type PLLCKSELR = crate::Reg<pllckselr::PLLCKSELR_SPEC>;
///RCC PLLs Clock Source Selection Register
pub mod pllckselr;
///PLLCFGR (rw) register accessor: an alias for `Reg<PLLCFGR_SPEC>`
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGR_SPEC>;
///RCC PLLs Configuration Register
pub mod pllcfgr;
///PLL1DIVR (rw) register accessor: an alias for `Reg<PLL1DIVR_SPEC>`
pub type PLL1DIVR = crate::Reg<pll1divr::PLL1DIVR_SPEC>;
///RCC PLL1 Dividers Configuration Register
pub mod pll1divr;
///PLL1FRACR (rw) register accessor: an alias for `Reg<PLL1FRACR_SPEC>`
pub type PLL1FRACR = crate::Reg<pll1fracr::PLL1FRACR_SPEC>;
///RCC PLL1 Fractional Divider Register
pub mod pll1fracr;
///PLL2DIVR (rw) register accessor: an alias for `Reg<PLL2DIVR_SPEC>`
pub type PLL2DIVR = crate::Reg<pll2divr::PLL2DIVR_SPEC>;
///RCC PLL2 Dividers Configuration Register
pub mod pll2divr;
///PLL2FRACR (rw) register accessor: an alias for `Reg<PLL2FRACR_SPEC>`
pub type PLL2FRACR = crate::Reg<pll2fracr::PLL2FRACR_SPEC>;
///RCC PLL2 Fractional Divider Register
pub mod pll2fracr;
///PLL3DIVR (rw) register accessor: an alias for `Reg<PLL3DIVR_SPEC>`
pub type PLL3DIVR = crate::Reg<pll3divr::PLL3DIVR_SPEC>;
///RCC PLL3 Dividers Configuration Register
pub mod pll3divr;
///PLL3FRACR (rw) register accessor: an alias for `Reg<PLL3FRACR_SPEC>`
pub type PLL3FRACR = crate::Reg<pll3fracr::PLL3FRACR_SPEC>;
///RCC PLL3 Fractional Divider Register
pub mod pll3fracr;
///D1CCIPR (rw) register accessor: an alias for `Reg<D1CCIPR_SPEC>`
pub type D1CCIPR = crate::Reg<d1ccipr::D1CCIPR_SPEC>;
///RCC Domain 1 Kernel Clock Configuration Register
pub mod d1ccipr;
///D2CCIP1R (rw) register accessor: an alias for `Reg<D2CCIP1R_SPEC>`
pub type D2CCIP1R = crate::Reg<d2ccip1r::D2CCIP1R_SPEC>;
///RCC Domain 2 Kernel Clock Configuration Register
pub mod d2ccip1r;
///D2CCIP2R (rw) register accessor: an alias for `Reg<D2CCIP2R_SPEC>`
pub type D2CCIP2R = crate::Reg<d2ccip2r::D2CCIP2R_SPEC>;
///RCC Domain 2 Kernel Clock Configuration Register
pub mod d2ccip2r;
///D3CCIPR (rw) register accessor: an alias for `Reg<D3CCIPR_SPEC>`
pub type D3CCIPR = crate::Reg<d3ccipr::D3CCIPR_SPEC>;
///RCC Domain 3 Kernel Clock Configuration Register
pub mod d3ccipr;
///CIER (rw) register accessor: an alias for `Reg<CIER_SPEC>`
pub type CIER = crate::Reg<cier::CIER_SPEC>;
///RCC Clock Source Interrupt Enable Register
pub mod cier;
///CIFR (r) register accessor: an alias for `Reg<CIFR_SPEC>`
pub type CIFR = crate::Reg<cifr::CIFR_SPEC>;
///RCC Clock Source Interrupt Flag Register
pub mod cifr;
///CICR (rw) register accessor: an alias for `Reg<CICR_SPEC>`
pub type CICR = crate::Reg<cicr::CICR_SPEC>;
///RCC Clock Source Interrupt Clear Register
pub mod cicr;
///BDCR (rw) register accessor: an alias for `Reg<BDCR_SPEC>`
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
///RCC Backup Domain Control Register
pub mod bdcr;
///CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///RCC Clock Control and Status Register
pub mod csr;
///AHB3RSTR (rw) register accessor: an alias for `Reg<AHB3RSTR_SPEC>`
pub type AHB3RSTR = crate::Reg<ahb3rstr::AHB3RSTR_SPEC>;
///RCC AHB3 Reset Register
pub mod ahb3rstr;
///AHB1RSTR (rw) register accessor: an alias for `Reg<AHB1RSTR_SPEC>`
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTR_SPEC>;
///RCC AHB1 Peripheral Reset Register
pub mod ahb1rstr;
///AHB2RSTR (rw) register accessor: an alias for `Reg<AHB2RSTR_SPEC>`
pub type AHB2RSTR = crate::Reg<ahb2rstr::AHB2RSTR_SPEC>;
///RCC AHB2 Peripheral Reset Register
pub mod ahb2rstr;
///AHB4RSTR (rw) register accessor: an alias for `Reg<AHB4RSTR_SPEC>`
pub type AHB4RSTR = crate::Reg<ahb4rstr::AHB4RSTR_SPEC>;
///RCC AHB4 Peripheral Reset Register
pub mod ahb4rstr;
///APB3RSTR (rw) register accessor: an alias for `Reg<APB3RSTR_SPEC>`
pub type APB3RSTR = crate::Reg<apb3rstr::APB3RSTR_SPEC>;
///RCC APB3 Peripheral Reset Register
pub mod apb3rstr;
///APB1LRSTR (rw) register accessor: an alias for `Reg<APB1LRSTR_SPEC>`
pub type APB1LRSTR = crate::Reg<apb1lrstr::APB1LRSTR_SPEC>;
///RCC APB1 Peripheral Reset Register
pub mod apb1lrstr;
///APB1HRSTR (rw) register accessor: an alias for `Reg<APB1HRSTR_SPEC>`
pub type APB1HRSTR = crate::Reg<apb1hrstr::APB1HRSTR_SPEC>;
///RCC APB1 Peripheral Reset Register
pub mod apb1hrstr;
///APB2RSTR (rw) register accessor: an alias for `Reg<APB2RSTR_SPEC>`
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTR_SPEC>;
///RCC APB2 Peripheral Reset Register
pub mod apb2rstr;
///APB4RSTR (rw) register accessor: an alias for `Reg<APB4RSTR_SPEC>`
pub type APB4RSTR = crate::Reg<apb4rstr::APB4RSTR_SPEC>;
///RCC APB4 Peripheral Reset Register
pub mod apb4rstr;
///GCR (rw) register accessor: an alias for `Reg<GCR_SPEC>`
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
///RCC Global Control Register
pub mod gcr;
///D3AMR (rw) register accessor: an alias for `Reg<D3AMR_SPEC>`
pub type D3AMR = crate::Reg<d3amr::D3AMR_SPEC>;
///RCC D3 Autonomous mode Register
pub mod d3amr;
///RSR (rw) register accessor: an alias for `Reg<RSR_SPEC>`
pub type RSR = crate::Reg<rsr::RSR_SPEC>;
///RCC Reset Status Register
pub mod rsr;
///C1_RSR (rw) register accessor: an alias for `Reg<C1_RSR_SPEC>`
pub type C1_RSR = crate::Reg<c1_rsr::C1_RSR_SPEC>;
///RCC Reset Status Register
pub mod c1_rsr;
///C1_AHB3ENR (rw) register accessor: an alias for `Reg<C1_AHB3ENR_SPEC>`
pub type C1_AHB3ENR = crate::Reg<c1_ahb3enr::C1_AHB3ENR_SPEC>;
///RCC AHB3 Clock Register
pub mod c1_ahb3enr;
///AHB3ENR (rw) register accessor: an alias for `Reg<AHB3ENR_SPEC>`
pub type AHB3ENR = crate::Reg<ahb3enr::AHB3ENR_SPEC>;
///RCC AHB3 Clock Register
pub mod ahb3enr;
///AHB1ENR (rw) register accessor: an alias for `Reg<AHB1ENR_SPEC>`
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENR_SPEC>;
///RCC AHB1 Clock Register
pub mod ahb1enr;
///C1_AHB1ENR (rw) register accessor: an alias for `Reg<C1_AHB1ENR_SPEC>`
pub type C1_AHB1ENR = crate::Reg<c1_ahb1enr::C1_AHB1ENR_SPEC>;
///RCC AHB1 Clock Register
pub mod c1_ahb1enr;
///C1_AHB2ENR (rw) register accessor: an alias for `Reg<C1_AHB2ENR_SPEC>`
pub type C1_AHB2ENR = crate::Reg<c1_ahb2enr::C1_AHB2ENR_SPEC>;
///RCC AHB2 Clock Register
pub mod c1_ahb2enr;
///AHB2ENR (rw) register accessor: an alias for `Reg<AHB2ENR_SPEC>`
pub type AHB2ENR = crate::Reg<ahb2enr::AHB2ENR_SPEC>;
///RCC AHB2 Clock Register
pub mod ahb2enr;
///AHB4ENR (rw) register accessor: an alias for `Reg<AHB4ENR_SPEC>`
pub type AHB4ENR = crate::Reg<ahb4enr::AHB4ENR_SPEC>;
///RCC AHB4 Clock Register
pub mod ahb4enr;
///C1_AHB4ENR (rw) register accessor: an alias for `Reg<C1_AHB4ENR_SPEC>`
pub type C1_AHB4ENR = crate::Reg<c1_ahb4enr::C1_AHB4ENR_SPEC>;
///RCC AHB4 Clock Register
pub mod c1_ahb4enr;
///C1_APB3ENR (rw) register accessor: an alias for `Reg<C1_APB3ENR_SPEC>`
pub type C1_APB3ENR = crate::Reg<c1_apb3enr::C1_APB3ENR_SPEC>;
///RCC APB3 Clock Register
pub mod c1_apb3enr;
///APB3ENR (rw) register accessor: an alias for `Reg<APB3ENR_SPEC>`
pub type APB3ENR = crate::Reg<apb3enr::APB3ENR_SPEC>;
///RCC APB3 Clock Register
pub mod apb3enr;
///APB1LENR (rw) register accessor: an alias for `Reg<APB1LENR_SPEC>`
pub type APB1LENR = crate::Reg<apb1lenr::APB1LENR_SPEC>;
///RCC APB1 Clock Register
pub mod apb1lenr;
///C1_APB1LENR (rw) register accessor: an alias for `Reg<C1_APB1LENR_SPEC>`
pub type C1_APB1LENR = crate::Reg<c1_apb1lenr::C1_APB1LENR_SPEC>;
///RCC APB1 Clock Register
pub mod c1_apb1lenr;
///APB1HENR (rw) register accessor: an alias for `Reg<APB1HENR_SPEC>`
pub type APB1HENR = crate::Reg<apb1henr::APB1HENR_SPEC>;
///RCC APB1 Clock Register
pub mod apb1henr;
///C1_APB1HENR (rw) register accessor: an alias for `Reg<C1_APB1HENR_SPEC>`
pub type C1_APB1HENR = crate::Reg<c1_apb1henr::C1_APB1HENR_SPEC>;
///RCC APB1 Clock Register
pub mod c1_apb1henr;
///C1_APB2ENR (rw) register accessor: an alias for `Reg<C1_APB2ENR_SPEC>`
pub type C1_APB2ENR = crate::Reg<c1_apb2enr::C1_APB2ENR_SPEC>;
///RCC APB2 Clock Register
pub mod c1_apb2enr;
///APB2ENR (rw) register accessor: an alias for `Reg<APB2ENR_SPEC>`
pub type APB2ENR = crate::Reg<apb2enr::APB2ENR_SPEC>;
///RCC APB2 Clock Register
pub mod apb2enr;
///APB4ENR (rw) register accessor: an alias for `Reg<APB4ENR_SPEC>`
pub type APB4ENR = crate::Reg<apb4enr::APB4ENR_SPEC>;
///RCC APB4 Clock Register
pub mod apb4enr;
///C1_APB4ENR (rw) register accessor: an alias for `Reg<C1_APB4ENR_SPEC>`
pub type C1_APB4ENR = crate::Reg<c1_apb4enr::C1_APB4ENR_SPEC>;
///RCC APB4 Clock Register
pub mod c1_apb4enr;
///C1_AHB3LPENR (rw) register accessor: an alias for `Reg<C1_AHB3LPENR_SPEC>`
pub type C1_AHB3LPENR = crate::Reg<c1_ahb3lpenr::C1_AHB3LPENR_SPEC>;
///RCC AHB3 Sleep Clock Register
pub mod c1_ahb3lpenr;
///AHB3LPENR (rw) register accessor: an alias for `Reg<AHB3LPENR_SPEC>`
pub type AHB3LPENR = crate::Reg<ahb3lpenr::AHB3LPENR_SPEC>;
///RCC AHB3 Sleep Clock Register
pub mod ahb3lpenr;
///AHB1LPENR (rw) register accessor: an alias for `Reg<AHB1LPENR_SPEC>`
pub type AHB1LPENR = crate::Reg<ahb1lpenr::AHB1LPENR_SPEC>;
///RCC AHB1 Sleep Clock Register
pub mod ahb1lpenr;
///C1_AHB1LPENR (rw) register accessor: an alias for `Reg<C1_AHB1LPENR_SPEC>`
pub type C1_AHB1LPENR = crate::Reg<c1_ahb1lpenr::C1_AHB1LPENR_SPEC>;
///RCC AHB1 Sleep Clock Register
pub mod c1_ahb1lpenr;
///C1_AHB2LPENR (rw) register accessor: an alias for `Reg<C1_AHB2LPENR_SPEC>`
pub type C1_AHB2LPENR = crate::Reg<c1_ahb2lpenr::C1_AHB2LPENR_SPEC>;
///RCC AHB2 Sleep Clock Register
pub mod c1_ahb2lpenr;
///AHB2LPENR (rw) register accessor: an alias for `Reg<AHB2LPENR_SPEC>`
pub type AHB2LPENR = crate::Reg<ahb2lpenr::AHB2LPENR_SPEC>;
///RCC AHB2 Sleep Clock Register
pub mod ahb2lpenr;
///AHB4LPENR (rw) register accessor: an alias for `Reg<AHB4LPENR_SPEC>`
pub type AHB4LPENR = crate::Reg<ahb4lpenr::AHB4LPENR_SPEC>;
///RCC AHB4 Sleep Clock Register
pub mod ahb4lpenr;
///C1_AHB4LPENR (rw) register accessor: an alias for `Reg<C1_AHB4LPENR_SPEC>`
pub type C1_AHB4LPENR = crate::Reg<c1_ahb4lpenr::C1_AHB4LPENR_SPEC>;
///RCC AHB4 Sleep Clock Register
pub mod c1_ahb4lpenr;
///C1_APB3LPENR (rw) register accessor: an alias for `Reg<C1_APB3LPENR_SPEC>`
pub type C1_APB3LPENR = crate::Reg<c1_apb3lpenr::C1_APB3LPENR_SPEC>;
///RCC APB3 Sleep Clock Register
pub mod c1_apb3lpenr;
///APB3LPENR (rw) register accessor: an alias for `Reg<APB3LPENR_SPEC>`
pub type APB3LPENR = crate::Reg<apb3lpenr::APB3LPENR_SPEC>;
///RCC APB3 Sleep Clock Register
pub mod apb3lpenr;
///APB1LLPENR (rw) register accessor: an alias for `Reg<APB1LLPENR_SPEC>`
pub type APB1LLPENR = crate::Reg<apb1llpenr::APB1LLPENR_SPEC>;
///RCC APB1 Low Sleep Clock Register
pub mod apb1llpenr;
///C1_APB1LLPENR (rw) register accessor: an alias for `Reg<C1_APB1LLPENR_SPEC>`
pub type C1_APB1LLPENR = crate::Reg<c1_apb1llpenr::C1_APB1LLPENR_SPEC>;
///RCC APB1 Low Sleep Clock Register
pub mod c1_apb1llpenr;
///C1_APB1HLPENR (rw) register accessor: an alias for `Reg<C1_APB1HLPENR_SPEC>`
pub type C1_APB1HLPENR = crate::Reg<c1_apb1hlpenr::C1_APB1HLPENR_SPEC>;
///RCC APB1 High Sleep Clock Register
pub mod c1_apb1hlpenr;
///APB1HLPENR (rw) register accessor: an alias for `Reg<APB1HLPENR_SPEC>`
pub type APB1HLPENR = crate::Reg<apb1hlpenr::APB1HLPENR_SPEC>;
///RCC APB1 High Sleep Clock Register
pub mod apb1hlpenr;
///APB2LPENR (rw) register accessor: an alias for `Reg<APB2LPENR_SPEC>`
pub type APB2LPENR = crate::Reg<apb2lpenr::APB2LPENR_SPEC>;
///RCC APB2 Sleep Clock Register
pub mod apb2lpenr;
///C1_APB2LPENR (rw) register accessor: an alias for `Reg<C1_APB2LPENR_SPEC>`
pub type C1_APB2LPENR = crate::Reg<c1_apb2lpenr::C1_APB2LPENR_SPEC>;
///RCC APB2 Sleep Clock Register
pub mod c1_apb2lpenr;
///C1_APB4LPENR (rw) register accessor: an alias for `Reg<C1_APB4LPENR_SPEC>`
pub type C1_APB4LPENR = crate::Reg<c1_apb4lpenr::C1_APB4LPENR_SPEC>;
///RCC APB4 Sleep Clock Register
pub mod c1_apb4lpenr;
///APB4LPENR (rw) register accessor: an alias for `Reg<APB4LPENR_SPEC>`
pub type APB4LPENR = crate::Reg<apb4lpenr::APB4LPENR_SPEC>;
///RCC APB4 Sleep Clock Register
pub mod apb4lpenr;
///HSICFGR (rw) register accessor: an alias for `Reg<HSICFGR_SPEC>`
pub type HSICFGR = crate::Reg<hsicfgr::HSICFGR_SPEC>;
///RCC HSI configuration register
pub mod hsicfgr;
///CSICFGR (rw) register accessor: an alias for `Reg<CSICFGR_SPEC>`
pub type CSICFGR = crate::Reg<csicfgr::CSICFGR_SPEC>;
///RCC CSI configuration register
pub mod csicfgr;
