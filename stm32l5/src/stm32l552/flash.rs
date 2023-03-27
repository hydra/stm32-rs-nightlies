///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Access control register
    pub acr: ACR,
    ///0x04 - Power down key register
    pub pdkeyr: PDKEYR,
    ///0x08 - Flash non-secure key register
    pub nskeyr: NSKEYR,
    ///0x0c - Flash secure key register
    pub seckeyr: SECKEYR,
    ///0x10 - Flash option key register
    pub optkeyr: OPTKEYR,
    ///0x14 - Flash low voltage key register
    pub lvekeyr: LVEKEYR,
    _reserved6: [u8; 0x08],
    ///0x20 - Flash status register
    pub nssr: NSSR,
    ///0x24 - Flash status register
    pub secsr: SECSR,
    ///0x28 - Flash non-secure control register
    pub nscr: NSCR,
    ///0x2c - Flash secure control register
    pub seccr: SECCR,
    ///0x30 - Flash ECC register
    pub eccr: ECCR,
    _reserved11: [u8; 0x0c],
    ///0x40 - Flash option register
    pub optr: OPTR,
    ///0x44 - Flash non-secure boot address 0 register
    pub nsbootadd0r: NSBOOTADD0R,
    ///0x48 - Flash non-secure boot address 1 register
    pub nsbootadd1r: NSBOOTADD1R,
    ///0x4c - FFlash secure boot address 0 register
    pub secbootadd0r: SECBOOTADD0R,
    ///0x50 - Flash bank 1 secure watermak1 register
    pub secwm1r1: SECWM1R1,
    ///0x54 - Flash secure watermak1 register 2
    pub secwm1r2: SECWM1R2,
    ///0x58 - Flash Bank 1 WRP area A address register
    pub wrp1ar: WRP1AR,
    ///0x5c - Flash Bank 1 WRP area B address register
    pub wrp1br: WRP1BR,
    ///0x60 - Flash secure watermak2 register
    pub secwm2r1: SECWM2R1,
    ///0x64 - Flash secure watermak2 register2
    pub secwm2r2: SECWM2R2,
    ///0x68 - Flash WPR2 area A address register
    pub wrp2ar: WRP2AR,
    ///0x6c - Flash WPR2 area B address register
    pub wrp2br: WRP2BR,
    _reserved23: [u8; 0x10],
    ///0x80..0x90 - FLASH secure block based bank 1 register
    pub secbb1r: [SECBB1R; 4],
    _reserved24: [u8; 0x10],
    ///0xa0..0xb0 - FLASH secure block based bank 2 register
    pub secbb2r: [SECBB2R; 4],
    _reserved25: [u8; 0x10],
    ///0xc0 - FLASH secure HDP control register
    pub sechdpcr: SECHDPCR,
    ///0xc4 - Power privilege configuration register
    pub privcfgr: PRIVCFGR,
}
impl RegisterBlock {
    ///0x80 - FLASH secure block based bank 1 register
    #[inline(always)]
    pub fn secbb1r1(&self) -> &SECBB1R {
        &self.secbb1r[0]
    }
    ///0x84 - FLASH secure block based bank 1 register
    #[inline(always)]
    pub fn secbb1r2(&self) -> &SECBB1R {
        &self.secbb1r[1]
    }
    ///0x88 - FLASH secure block based bank 1 register
    #[inline(always)]
    pub fn secbb1r3(&self) -> &SECBB1R {
        &self.secbb1r[2]
    }
    ///0x8c - FLASH secure block based bank 1 register
    #[inline(always)]
    pub fn secbb1r4(&self) -> &SECBB1R {
        &self.secbb1r[3]
    }
    ///0xa0 - FLASH secure block based bank 2 register
    #[inline(always)]
    pub fn secbb2r1(&self) -> &SECBB2R {
        &self.secbb2r[0]
    }
    ///0xa4 - FLASH secure block based bank 2 register
    #[inline(always)]
    pub fn secbb2r2(&self) -> &SECBB2R {
        &self.secbb2r[1]
    }
    ///0xa8 - FLASH secure block based bank 2 register
    #[inline(always)]
    pub fn secbb2r3(&self) -> &SECBB2R {
        &self.secbb2r[2]
    }
    ///0xac - FLASH secure block based bank 2 register
    #[inline(always)]
    pub fn secbb2r4(&self) -> &SECBB2R {
        &self.secbb2r[3]
    }
}
///ACR (rw) register accessor: an alias for `Reg<ACR_SPEC>`
pub type ACR = crate::Reg<acr::ACR_SPEC>;
///Access control register
pub mod acr;
///PDKEYR (w) register accessor: an alias for `Reg<PDKEYR_SPEC>`
pub type PDKEYR = crate::Reg<pdkeyr::PDKEYR_SPEC>;
///Power down key register
pub mod pdkeyr;
///NSKEYR (w) register accessor: an alias for `Reg<NSKEYR_SPEC>`
pub type NSKEYR = crate::Reg<nskeyr::NSKEYR_SPEC>;
///Flash non-secure key register
pub mod nskeyr;
///SECKEYR (w) register accessor: an alias for `Reg<SECKEYR_SPEC>`
pub type SECKEYR = crate::Reg<seckeyr::SECKEYR_SPEC>;
///Flash secure key register
pub mod seckeyr;
///OPTKEYR (w) register accessor: an alias for `Reg<OPTKEYR_SPEC>`
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYR_SPEC>;
///Flash option key register
pub mod optkeyr;
///LVEKEYR (w) register accessor: an alias for `Reg<LVEKEYR_SPEC>`
pub type LVEKEYR = crate::Reg<lvekeyr::LVEKEYR_SPEC>;
///Flash low voltage key register
pub mod lvekeyr;
///NSSR (rw) register accessor: an alias for `Reg<NSSR_SPEC>`
pub type NSSR = crate::Reg<nssr::NSSR_SPEC>;
///Flash status register
pub mod nssr;
///SECSR (rw) register accessor: an alias for `Reg<SECSR_SPEC>`
pub type SECSR = crate::Reg<secsr::SECSR_SPEC>;
///Flash status register
pub mod secsr;
///NSCR (rw) register accessor: an alias for `Reg<NSCR_SPEC>`
pub type NSCR = crate::Reg<nscr::NSCR_SPEC>;
///Flash non-secure control register
pub mod nscr;
///SECCR (rw) register accessor: an alias for `Reg<SECCR_SPEC>`
pub type SECCR = crate::Reg<seccr::SECCR_SPEC>;
///Flash secure control register
pub mod seccr;
///ECCR (rw) register accessor: an alias for `Reg<ECCR_SPEC>`
pub type ECCR = crate::Reg<eccr::ECCR_SPEC>;
///Flash ECC register
pub mod eccr;
///OPTR (rw) register accessor: an alias for `Reg<OPTR_SPEC>`
pub type OPTR = crate::Reg<optr::OPTR_SPEC>;
///Flash option register
pub mod optr;
///NSBOOTADD0R (w) register accessor: an alias for `Reg<NSBOOTADD0R_SPEC>`
pub type NSBOOTADD0R = crate::Reg<nsbootadd0r::NSBOOTADD0R_SPEC>;
///Flash non-secure boot address 0 register
pub mod nsbootadd0r;
///NSBOOTADD1R (w) register accessor: an alias for `Reg<NSBOOTADD1R_SPEC>`
pub type NSBOOTADD1R = crate::Reg<nsbootadd1r::NSBOOTADD1R_SPEC>;
///Flash non-secure boot address 1 register
pub mod nsbootadd1r;
///SECBOOTADD0R (rw) register accessor: an alias for `Reg<SECBOOTADD0R_SPEC>`
pub type SECBOOTADD0R = crate::Reg<secbootadd0r::SECBOOTADD0R_SPEC>;
///FFlash secure boot address 0 register
pub mod secbootadd0r;
///SECWM1R1 (rw) register accessor: an alias for `Reg<SECWM1R1_SPEC>`
pub type SECWM1R1 = crate::Reg<secwm1r1::SECWM1R1_SPEC>;
///Flash bank 1 secure watermak1 register
pub mod secwm1r1;
///SECWM1R2 (rw) register accessor: an alias for `Reg<SECWM1R2_SPEC>`
pub type SECWM1R2 = crate::Reg<secwm1r2::SECWM1R2_SPEC>;
///Flash secure watermak1 register 2
pub mod secwm1r2;
///WRP1AR (rw) register accessor: an alias for `Reg<WRP1AR_SPEC>`
pub type WRP1AR = crate::Reg<wrp1ar::WRP1AR_SPEC>;
///Flash Bank 1 WRP area A address register
pub mod wrp1ar;
///WRP1BR (rw) register accessor: an alias for `Reg<WRP1BR_SPEC>`
pub type WRP1BR = crate::Reg<wrp1br::WRP1BR_SPEC>;
///Flash Bank 1 WRP area B address register
pub mod wrp1br;
///SECWM2R1 (rw) register accessor: an alias for `Reg<SECWM2R1_SPEC>`
pub type SECWM2R1 = crate::Reg<secwm2r1::SECWM2R1_SPEC>;
///Flash secure watermak2 register
pub mod secwm2r1;
///SECWM2R2 (rw) register accessor: an alias for `Reg<SECWM2R2_SPEC>`
pub type SECWM2R2 = crate::Reg<secwm2r2::SECWM2R2_SPEC>;
///Flash secure watermak2 register2
pub mod secwm2r2;
///WRP2AR (rw) register accessor: an alias for `Reg<WRP2AR_SPEC>`
pub type WRP2AR = crate::Reg<wrp2ar::WRP2AR_SPEC>;
///Flash WPR2 area A address register
pub mod wrp2ar;
///WRP2BR (rw) register accessor: an alias for `Reg<WRP2BR_SPEC>`
pub type WRP2BR = crate::Reg<wrp2br::WRP2BR_SPEC>;
///Flash WPR2 area B address register
pub mod wrp2br;
///SECBB1R (rw) register accessor: an alias for `Reg<SECBB1R_SPEC>`
pub type SECBB1R = crate::Reg<secbb1r::SECBB1R_SPEC>;
///FLASH secure block based bank 1 register
pub mod secbb1r;
///SECBB2R (rw) register accessor: an alias for `Reg<SECBB2R_SPEC>`
pub type SECBB2R = crate::Reg<secbb2r::SECBB2R_SPEC>;
///FLASH secure block based bank 2 register
pub mod secbb2r;
///SECHDPCR (rw) register accessor: an alias for `Reg<SECHDPCR_SPEC>`
pub type SECHDPCR = crate::Reg<sechdpcr::SECHDPCR_SPEC>;
///FLASH secure HDP control register
pub mod sechdpcr;
///PRIVCFGR (rw) register accessor: an alias for `Reg<PRIVCFGR_SPEC>`
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGR_SPEC>;
///Power privilege configuration register
pub mod privcfgr;
