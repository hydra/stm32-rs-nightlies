///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - FLASH access control register
    pub flash_acr: FLASH_ACR,
    _reserved1: [u8; 0x04],
    ///0x08 - FLASH non-secure key register
    pub flash_nskeyr: FLASH_NSKEYR,
    ///0x0c - FLASH secure key register
    pub flash_seckeyr: FLASH_SECKEYR,
    ///0x10 - FLASH option key register
    pub flash_optkeyr: FLASH_OPTKEYR,
    _reserved4: [u8; 0x04],
    ///0x18 - FLASH bank 1 power-down key register
    pub flash_pdkey1r: FLASH_PDKEY1R,
    ///0x1c - FLASH bank 2 power-down key register
    pub flash_pdkey2r: FLASH_PDKEY2R,
    ///0x20 - FLASH non-secure status register
    pub flash_nssr: FLASH_NSSR,
    ///0x24 - FLASH secure status register
    pub flash_secsr: FLASH_SECSR,
    ///0x28 - FLASH non-secure control register
    pub flash_nscr: FLASH_NSCR,
    ///0x2c - FLASH secure control register
    pub flash_seccr: FLASH_SECCR,
    ///0x30 - FLASH ECC register
    pub flash_eccr: FLASH_ECCR,
    ///0x34 - FLASH operation status register
    pub flash_opsr: FLASH_OPSR,
    _reserved12: [u8; 0x08],
    ///0x40 - FLASH option register
    pub flash_optr: FLASH_OPTR,
    ///0x44 - FLASH non-secure boot address 0 register
    pub flash_nsbootadd0r: FLASH_NSBOOTADD0R,
    ///0x48 - FLASH non-secure boot address 1 register
    pub flash_nsbootadd1r: FLASH_NSBOOTADD1R,
    ///0x4c - FLASH secure boot address 0 register
    pub flash_secbootadd0r: FLASH_SECBOOTADD0R,
    ///0x50 - FLASH secure watermark1 register 1
    pub flash_secwm1r1: FLASH_SECWM1R1,
    ///0x54 - FLASH secure watermark1 register 2
    pub flash_secwm1r2: FLASH_SECWM1R2,
    ///0x58 - FLASH WRP1 area A address register
    pub flash_wrp1ar: FLASH_WRP1AR,
    ///0x5c - FLASH WRP1 area B address register
    pub flash_wrp1br: FLASH_WRP1BR,
    ///0x60 - FLASH secure watermark2 register 1
    pub flash_secwm2r1: FLASH_SECWM2R1,
    ///0x64 - FLASH secure watermark2 register 2
    pub flash_secwm2r2: FLASH_SECWM2R2,
    ///0x68 - FLASH WPR2 area A address register
    pub flash_wrp2ar: FLASH_WRP2AR,
    ///0x6c - FLASH WPR2 area B address register
    pub flash_wrp2br: FLASH_WRP2BR,
    ///0x70 - FLASH OEM1 key register 1
    pub flash_oem1keyr1: FLASH_OEM1KEYR1,
    ///0x74 - FLASH OEM1 key register 2
    pub flash_oem1keyr2: FLASH_OEM1KEYR2,
    ///0x78 - FLASH OEM2 key register 1
    pub flash_oem2keyr1: FLASH_OEM2KEYR1,
    ///0x7c - FLASH OEM2 key register 2
    pub flash_oem2keyr2: FLASH_OEM2KEYR2,
    ///0x80 - FLASH secure block based bank 1 register 1
    pub flash_sec1bbr1: FLASH_SEC1BBR1,
    ///0x84 - FLASH secure block based bank 1 register 2
    pub flash_sec1bbr2: FLASH_SEC1BBR2,
    ///0x88 - FLASH secure block based bank 1 register 3
    pub flash_sec1bbr3: FLASH_SEC1BBR3,
    ///0x8c - FLASH secure block based bank 1 register 4
    pub flash_sec1bbr4: FLASH_SEC1BBR4,
    _reserved32: [u8; 0x10],
    ///0xa0 - FLASH secure block based bank 2 register 1
    pub flash_sec2bbr1: FLASH_SEC2BBR1,
    ///0xa4 - FLASH secure block based bank 2 register 2
    pub flash_sec2bbr2: FLASH_SEC2BBR2,
    ///0xa8 - FLASH secure block based bank 2 register 3
    pub flash_sec2bbr3: FLASH_SEC2BBR3,
    ///0xac - FLASH secure block based bank 2 register 4
    pub flash_sec2bbr4: FLASH_SEC2BBR4,
    _reserved36: [u8; 0x10],
    ///0xc0 - FLASH secure HDP control register
    pub flash_sechdpcr: FLASH_SECHDPCR,
    ///0xc4 - FLASH privilege configuration register
    pub flash_privcfgr: FLASH_PRIVCFGR,
    _reserved38: [u8; 0x08],
    ///0xd0 - FLASH privilege block based bank 1 register 1
    pub flash_priv1bbr1: FLASH_PRIV1BBR1,
    ///0xd4 - FLASH privilege block based bank 1 register 2
    pub flash_priv1bbr2: FLASH_PRIV1BBR2,
    ///0xd8 - FLASH privilege block based bank 1 register 3
    pub flash_priv1bbr3: FLASH_PRIV1BBR3,
    ///0xdc - FLASH privilege block based bank 1 register 4
    pub flash_priv1bbr4: FLASH_PRIV1BBR4,
    _reserved42: [u8; 0x10],
    ///0xf0 - FLASH privilege block based bank 2 register 1
    pub flash_priv2bbr1: FLASH_PRIV2BBR1,
    ///0xf4 - FLASH privilege block based bank 2 register 2
    pub flash_priv2bbr2: FLASH_PRIV2BBR2,
    ///0xf8 - FLASH privilege block based bank 2 register 3
    pub flash_priv2bbr3: FLASH_PRIV2BBR3,
    ///0xfc - FLASH privilege block based bank 2 register 4
    pub flash_priv2bbr4: FLASH_PRIV2BBR4,
}
///FLASH_ACR (rw) register accessor: an alias for `Reg<FLASH_ACR_SPEC>`
pub type FLASH_ACR = crate::Reg<flash_acr::FLASH_ACR_SPEC>;
///FLASH access control register
pub mod flash_acr;
///FLASH_NSKEYR (w) register accessor: an alias for `Reg<FLASH_NSKEYR_SPEC>`
pub type FLASH_NSKEYR = crate::Reg<flash_nskeyr::FLASH_NSKEYR_SPEC>;
///FLASH non-secure key register
pub mod flash_nskeyr;
///FLASH_SECKEYR (w) register accessor: an alias for `Reg<FLASH_SECKEYR_SPEC>`
pub type FLASH_SECKEYR = crate::Reg<flash_seckeyr::FLASH_SECKEYR_SPEC>;
///FLASH secure key register
pub mod flash_seckeyr;
///FLASH_OPTKEYR (w) register accessor: an alias for `Reg<FLASH_OPTKEYR_SPEC>`
pub type FLASH_OPTKEYR = crate::Reg<flash_optkeyr::FLASH_OPTKEYR_SPEC>;
///FLASH option key register
pub mod flash_optkeyr;
///FLASH_PDKEY1R (w) register accessor: an alias for `Reg<FLASH_PDKEY1R_SPEC>`
pub type FLASH_PDKEY1R = crate::Reg<flash_pdkey1r::FLASH_PDKEY1R_SPEC>;
///FLASH bank 1 power-down key register
pub mod flash_pdkey1r;
///FLASH_PDKEY2R (w) register accessor: an alias for `Reg<FLASH_PDKEY2R_SPEC>`
pub type FLASH_PDKEY2R = crate::Reg<flash_pdkey2r::FLASH_PDKEY2R_SPEC>;
///FLASH bank 2 power-down key register
pub mod flash_pdkey2r;
///FLASH_NSSR (rw) register accessor: an alias for `Reg<FLASH_NSSR_SPEC>`
pub type FLASH_NSSR = crate::Reg<flash_nssr::FLASH_NSSR_SPEC>;
///FLASH non-secure status register
pub mod flash_nssr;
///FLASH_SECSR (rw) register accessor: an alias for `Reg<FLASH_SECSR_SPEC>`
pub type FLASH_SECSR = crate::Reg<flash_secsr::FLASH_SECSR_SPEC>;
///FLASH secure status register
pub mod flash_secsr;
///FLASH_NSCR (rw) register accessor: an alias for `Reg<FLASH_NSCR_SPEC>`
pub type FLASH_NSCR = crate::Reg<flash_nscr::FLASH_NSCR_SPEC>;
///FLASH non-secure control register
pub mod flash_nscr;
///FLASH_SECCR (rw) register accessor: an alias for `Reg<FLASH_SECCR_SPEC>`
pub type FLASH_SECCR = crate::Reg<flash_seccr::FLASH_SECCR_SPEC>;
///FLASH secure control register
pub mod flash_seccr;
///FLASH_ECCR (rw) register accessor: an alias for `Reg<FLASH_ECCR_SPEC>`
pub type FLASH_ECCR = crate::Reg<flash_eccr::FLASH_ECCR_SPEC>;
///FLASH ECC register
pub mod flash_eccr;
///FLASH_OPSR (r) register accessor: an alias for `Reg<FLASH_OPSR_SPEC>`
pub type FLASH_OPSR = crate::Reg<flash_opsr::FLASH_OPSR_SPEC>;
///FLASH operation status register
pub mod flash_opsr;
///FLASH_OPTR (rw) register accessor: an alias for `Reg<FLASH_OPTR_SPEC>`
pub type FLASH_OPTR = crate::Reg<flash_optr::FLASH_OPTR_SPEC>;
///FLASH option register
pub mod flash_optr;
///FLASH_NSBOOTADD0R (rw) register accessor: an alias for `Reg<FLASH_NSBOOTADD0R_SPEC>`
pub type FLASH_NSBOOTADD0R = crate::Reg<flash_nsbootadd0r::FLASH_NSBOOTADD0R_SPEC>;
///FLASH non-secure boot address 0 register
pub mod flash_nsbootadd0r;
///FLASH_NSBOOTADD1R (rw) register accessor: an alias for `Reg<FLASH_NSBOOTADD1R_SPEC>`
pub type FLASH_NSBOOTADD1R = crate::Reg<flash_nsbootadd1r::FLASH_NSBOOTADD1R_SPEC>;
///FLASH non-secure boot address 1 register
pub mod flash_nsbootadd1r;
///FLASH_SECBOOTADD0R (rw) register accessor: an alias for `Reg<FLASH_SECBOOTADD0R_SPEC>`
pub type FLASH_SECBOOTADD0R = crate::Reg<flash_secbootadd0r::FLASH_SECBOOTADD0R_SPEC>;
///FLASH secure boot address 0 register
pub mod flash_secbootadd0r;
///FLASH_SECWM1R1 (rw) register accessor: an alias for `Reg<FLASH_SECWM1R1_SPEC>`
pub type FLASH_SECWM1R1 = crate::Reg<flash_secwm1r1::FLASH_SECWM1R1_SPEC>;
///FLASH secure watermark1 register 1
pub mod flash_secwm1r1;
///FLASH_SECWM1R2 (rw) register accessor: an alias for `Reg<FLASH_SECWM1R2_SPEC>`
pub type FLASH_SECWM1R2 = crate::Reg<flash_secwm1r2::FLASH_SECWM1R2_SPEC>;
///FLASH secure watermark1 register 2
pub mod flash_secwm1r2;
///FLASH_WRP1AR (rw) register accessor: an alias for `Reg<FLASH_WRP1AR_SPEC>`
pub type FLASH_WRP1AR = crate::Reg<flash_wrp1ar::FLASH_WRP1AR_SPEC>;
///FLASH WRP1 area A address register
pub mod flash_wrp1ar;
///FLASH_WRP1BR (rw) register accessor: an alias for `Reg<FLASH_WRP1BR_SPEC>`
pub type FLASH_WRP1BR = crate::Reg<flash_wrp1br::FLASH_WRP1BR_SPEC>;
///FLASH WRP1 area B address register
pub mod flash_wrp1br;
///FLASH_SECWM2R1 (rw) register accessor: an alias for `Reg<FLASH_SECWM2R1_SPEC>`
pub type FLASH_SECWM2R1 = crate::Reg<flash_secwm2r1::FLASH_SECWM2R1_SPEC>;
///FLASH secure watermark2 register 1
pub mod flash_secwm2r1;
///FLASH_SECWM2R2 (rw) register accessor: an alias for `Reg<FLASH_SECWM2R2_SPEC>`
pub type FLASH_SECWM2R2 = crate::Reg<flash_secwm2r2::FLASH_SECWM2R2_SPEC>;
///FLASH secure watermark2 register 2
pub mod flash_secwm2r2;
///FLASH_WRP2AR (rw) register accessor: an alias for `Reg<FLASH_WRP2AR_SPEC>`
pub type FLASH_WRP2AR = crate::Reg<flash_wrp2ar::FLASH_WRP2AR_SPEC>;
///FLASH WPR2 area A address register
pub mod flash_wrp2ar;
///FLASH_WRP2BR (rw) register accessor: an alias for `Reg<FLASH_WRP2BR_SPEC>`
pub type FLASH_WRP2BR = crate::Reg<flash_wrp2br::FLASH_WRP2BR_SPEC>;
///FLASH WPR2 area B address register
pub mod flash_wrp2br;
///FLASH_OEM1KEYR1 (w) register accessor: an alias for `Reg<FLASH_OEM1KEYR1_SPEC>`
pub type FLASH_OEM1KEYR1 = crate::Reg<flash_oem1keyr1::FLASH_OEM1KEYR1_SPEC>;
///FLASH OEM1 key register 1
pub mod flash_oem1keyr1;
///FLASH_OEM1KEYR2 (w) register accessor: an alias for `Reg<FLASH_OEM1KEYR2_SPEC>`
pub type FLASH_OEM1KEYR2 = crate::Reg<flash_oem1keyr2::FLASH_OEM1KEYR2_SPEC>;
///FLASH OEM1 key register 2
pub mod flash_oem1keyr2;
///FLASH_OEM2KEYR1 (w) register accessor: an alias for `Reg<FLASH_OEM2KEYR1_SPEC>`
pub type FLASH_OEM2KEYR1 = crate::Reg<flash_oem2keyr1::FLASH_OEM2KEYR1_SPEC>;
///FLASH OEM2 key register 1
pub mod flash_oem2keyr1;
///FLASH_OEM2KEYR2 (w) register accessor: an alias for `Reg<FLASH_OEM2KEYR2_SPEC>`
pub type FLASH_OEM2KEYR2 = crate::Reg<flash_oem2keyr2::FLASH_OEM2KEYR2_SPEC>;
///FLASH OEM2 key register 2
pub mod flash_oem2keyr2;
///FLASH_SEC1BBR1 (rw) register accessor: an alias for `Reg<FLASH_SEC1BBR1_SPEC>`
pub type FLASH_SEC1BBR1 = crate::Reg<flash_sec1bbr1::FLASH_SEC1BBR1_SPEC>;
///FLASH secure block based bank 1 register 1
pub mod flash_sec1bbr1;
///FLASH_SEC1BBR2 (rw) register accessor: an alias for `Reg<FLASH_SEC1BBR2_SPEC>`
pub type FLASH_SEC1BBR2 = crate::Reg<flash_sec1bbr2::FLASH_SEC1BBR2_SPEC>;
///FLASH secure block based bank 1 register 2
pub mod flash_sec1bbr2;
///FLASH_SEC1BBR3 (rw) register accessor: an alias for `Reg<FLASH_SEC1BBR3_SPEC>`
pub type FLASH_SEC1BBR3 = crate::Reg<flash_sec1bbr3::FLASH_SEC1BBR3_SPEC>;
///FLASH secure block based bank 1 register 3
pub mod flash_sec1bbr3;
///FLASH_SEC1BBR4 (rw) register accessor: an alias for `Reg<FLASH_SEC1BBR4_SPEC>`
pub type FLASH_SEC1BBR4 = crate::Reg<flash_sec1bbr4::FLASH_SEC1BBR4_SPEC>;
///FLASH secure block based bank 1 register 4
pub mod flash_sec1bbr4;
///FLASH_SEC2BBR1 (rw) register accessor: an alias for `Reg<FLASH_SEC2BBR1_SPEC>`
pub type FLASH_SEC2BBR1 = crate::Reg<flash_sec2bbr1::FLASH_SEC2BBR1_SPEC>;
///FLASH secure block based bank 2 register 1
pub mod flash_sec2bbr1;
///FLASH_SEC2BBR2 (rw) register accessor: an alias for `Reg<FLASH_SEC2BBR2_SPEC>`
pub type FLASH_SEC2BBR2 = crate::Reg<flash_sec2bbr2::FLASH_SEC2BBR2_SPEC>;
///FLASH secure block based bank 2 register 2
pub mod flash_sec2bbr2;
///FLASH_SEC2BBR3 (rw) register accessor: an alias for `Reg<FLASH_SEC2BBR3_SPEC>`
pub type FLASH_SEC2BBR3 = crate::Reg<flash_sec2bbr3::FLASH_SEC2BBR3_SPEC>;
///FLASH secure block based bank 2 register 3
pub mod flash_sec2bbr3;
///FLASH_SEC2BBR4 (rw) register accessor: an alias for `Reg<FLASH_SEC2BBR4_SPEC>`
pub type FLASH_SEC2BBR4 = crate::Reg<flash_sec2bbr4::FLASH_SEC2BBR4_SPEC>;
///FLASH secure block based bank 2 register 4
pub mod flash_sec2bbr4;
///FLASH_SECHDPCR (rw) register accessor: an alias for `Reg<FLASH_SECHDPCR_SPEC>`
pub type FLASH_SECHDPCR = crate::Reg<flash_sechdpcr::FLASH_SECHDPCR_SPEC>;
///FLASH secure HDP control register
pub mod flash_sechdpcr;
///FLASH_PRIVCFGR (rw) register accessor: an alias for `Reg<FLASH_PRIVCFGR_SPEC>`
pub type FLASH_PRIVCFGR = crate::Reg<flash_privcfgr::FLASH_PRIVCFGR_SPEC>;
///FLASH privilege configuration register
pub mod flash_privcfgr;
///FLASH_PRIV1BBR1 (rw) register accessor: an alias for `Reg<FLASH_PRIV1BBR1_SPEC>`
pub type FLASH_PRIV1BBR1 = crate::Reg<flash_priv1bbr1::FLASH_PRIV1BBR1_SPEC>;
///FLASH privilege block based bank 1 register 1
pub mod flash_priv1bbr1;
///FLASH_PRIV1BBR2 (rw) register accessor: an alias for `Reg<FLASH_PRIV1BBR2_SPEC>`
pub type FLASH_PRIV1BBR2 = crate::Reg<flash_priv1bbr2::FLASH_PRIV1BBR2_SPEC>;
///FLASH privilege block based bank 1 register 2
pub mod flash_priv1bbr2;
///FLASH_PRIV1BBR3 (rw) register accessor: an alias for `Reg<FLASH_PRIV1BBR3_SPEC>`
pub type FLASH_PRIV1BBR3 = crate::Reg<flash_priv1bbr3::FLASH_PRIV1BBR3_SPEC>;
///FLASH privilege block based bank 1 register 3
pub mod flash_priv1bbr3;
///FLASH_PRIV1BBR4 (rw) register accessor: an alias for `Reg<FLASH_PRIV1BBR4_SPEC>`
pub type FLASH_PRIV1BBR4 = crate::Reg<flash_priv1bbr4::FLASH_PRIV1BBR4_SPEC>;
///FLASH privilege block based bank 1 register 4
pub mod flash_priv1bbr4;
///FLASH_PRIV2BBR1 (rw) register accessor: an alias for `Reg<FLASH_PRIV2BBR1_SPEC>`
pub type FLASH_PRIV2BBR1 = crate::Reg<flash_priv2bbr1::FLASH_PRIV2BBR1_SPEC>;
///FLASH privilege block based bank 2 register 1
pub mod flash_priv2bbr1;
///FLASH_PRIV2BBR2 (rw) register accessor: an alias for `Reg<FLASH_PRIV2BBR2_SPEC>`
pub type FLASH_PRIV2BBR2 = crate::Reg<flash_priv2bbr2::FLASH_PRIV2BBR2_SPEC>;
///FLASH privilege block based bank 2 register 2
pub mod flash_priv2bbr2;
///FLASH_PRIV2BBR3 (rw) register accessor: an alias for `Reg<FLASH_PRIV2BBR3_SPEC>`
pub type FLASH_PRIV2BBR3 = crate::Reg<flash_priv2bbr3::FLASH_PRIV2BBR3_SPEC>;
///FLASH privilege block based bank 2 register 3
pub mod flash_priv2bbr3;
///FLASH_PRIV2BBR4 (rw) register accessor: an alias for `Reg<FLASH_PRIV2BBR4_SPEC>`
pub type FLASH_PRIV2BBR4 = crate::Reg<flash_priv2bbr4::FLASH_PRIV2BBR4_SPEC>;
///FLASH privilege block based bank 2 register 4
pub mod flash_priv2bbr4;
