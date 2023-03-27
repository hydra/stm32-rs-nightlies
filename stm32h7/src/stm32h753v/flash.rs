///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Access control register
    pub acr: ACR,
    _reserved_1_acr_: [u8; 0x0200],
}
impl RegisterBlock {
    ///0x04..0x204 - Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R
    #[inline(always)]
    pub const fn bank(&self) -> &[BANK; 2] {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    ///0x04..0x104 - Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R
    #[inline(always)]
    pub fn bank1(&self) -> &BANK {
        &self.bank()[0]
    }
    ///0x104..0x204 - Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R
    #[inline(always)]
    pub fn bank2(&self) -> &BANK {
        &self.bank()[1]
    }
    ///0x08 - FLASH option key register
    #[inline(always)]
    pub const fn optkeyr(&self) -> &OPTKEYR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8usize).cast() }
    }
    ///0x18 - FLASH option control register
    #[inline(always)]
    pub const fn optcr(&self) -> &OPTCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    ///0x1c - FLASH option status register
    #[inline(always)]
    pub const fn optsr_cur(&self) -> &OPTSR_CUR {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    ///0x20 - FLASH option status register
    #[inline(always)]
    pub const fn optsr_prg(&self) -> &OPTSR_PRG {
        unsafe { &*(self as *const Self).cast::<u8>().add(32usize).cast() }
    }
    ///0x24 - FLASH option clear control register
    #[inline(always)]
    pub const fn optccr(&self) -> &OPTCCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(36usize).cast() }
    }
    ///0x40 - FLASH register with boot address
    #[inline(always)]
    pub const fn boot_curr(&self) -> &BOOT_CURR {
        unsafe { &*(self as *const Self).cast::<u8>().add(64usize).cast() }
    }
    ///0x44 - FLASH register with boot address
    #[inline(always)]
    pub const fn boot_prgr(&self) -> &BOOT_PRGR {
        unsafe { &*(self as *const Self).cast::<u8>().add(68usize).cast() }
    }
    ///0x5c - FLASH CRC data register
    #[inline(always)]
    pub const fn crcdatar(&self) -> &CRCDATAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(92usize).cast() }
    }
    ///0x100 - Access control register
    #[inline(always)]
    pub const fn acr_(&self) -> &ACR_ {
        unsafe { &*(self as *const Self).cast::<u8>().add(256usize).cast() }
    }
    ///0x108 - FLASH option key register
    #[inline(always)]
    pub const fn optkeyr_(&self) -> &OPTKEYR_ {
        unsafe { &*(self as *const Self).cast::<u8>().add(264usize).cast() }
    }
    ///0x118 - FLASH option control register
    #[inline(always)]
    pub const fn optcr_(&self) -> &OPTCR_ {
        unsafe { &*(self as *const Self).cast::<u8>().add(280usize).cast() }
    }
    ///0x11c - FLASH option status register
    #[inline(always)]
    pub const fn optsr_cur_(&self) -> &OPTSR_CUR_ {
        unsafe { &*(self as *const Self).cast::<u8>().add(284usize).cast() }
    }
    ///0x120 - FLASH option status register
    #[inline(always)]
    pub const fn optsr_prg_(&self) -> &OPTSR_PRG_ {
        unsafe { &*(self as *const Self).cast::<u8>().add(288usize).cast() }
    }
    ///0x124 - FLASH option clear control register
    #[inline(always)]
    pub const fn optccr_(&self) -> &OPTCCR_ {
        unsafe { &*(self as *const Self).cast::<u8>().add(292usize).cast() }
    }
}
///ACR (rw) register accessor: an alias for `Reg<ACR_SPEC>`
pub type ACR = crate::Reg<acr::ACR_SPEC>;
///Access control register
pub mod acr;
///ACR_ (rw) register accessor: an alias for `Reg<ACR__SPEC>`
pub type ACR_ = crate::Reg<acr_::ACR__SPEC>;
///Access control register
pub mod acr_;
///Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R
pub use self::bank::BANK;
///Cluster
///Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R
pub mod bank;
///OPTKEYR (rw) register accessor: an alias for `Reg<OPTKEYR_SPEC>`
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYR_SPEC>;
///FLASH option key register
pub mod optkeyr;
///OPTKEYR_ (rw) register accessor: an alias for `Reg<OPTKEYR__SPEC>`
pub type OPTKEYR_ = crate::Reg<optkeyr_::OPTKEYR__SPEC>;
///FLASH option key register
pub mod optkeyr_;
///OPTCR (rw) register accessor: an alias for `Reg<OPTCR_SPEC>`
pub type OPTCR = crate::Reg<optcr::OPTCR_SPEC>;
///FLASH option control register
pub mod optcr;
///OPTCR_ (rw) register accessor: an alias for `Reg<OPTCR__SPEC>`
pub type OPTCR_ = crate::Reg<optcr_::OPTCR__SPEC>;
///FLASH option control register
pub mod optcr_;
///OPTSR_CUR_ (rw) register accessor: an alias for `Reg<OPTSR_CUR__SPEC>`
pub type OPTSR_CUR_ = crate::Reg<optsr_cur_::OPTSR_CUR__SPEC>;
///FLASH option status register
pub mod optsr_cur_;
///OPTSR_CUR (rw) register accessor: an alias for `Reg<OPTSR_CUR_SPEC>`
pub type OPTSR_CUR = crate::Reg<optsr_cur::OPTSR_CUR_SPEC>;
///FLASH option status register
pub mod optsr_cur;
///OPTSR_PRG (rw) register accessor: an alias for `Reg<OPTSR_PRG_SPEC>`
pub type OPTSR_PRG = crate::Reg<optsr_prg::OPTSR_PRG_SPEC>;
///FLASH option status register
pub mod optsr_prg;
///OPTSR_PRG_ (rw) register accessor: an alias for `Reg<OPTSR_PRG__SPEC>`
pub type OPTSR_PRG_ = crate::Reg<optsr_prg_::OPTSR_PRG__SPEC>;
///FLASH option status register
pub mod optsr_prg_;
///OPTCCR_ (w) register accessor: an alias for `Reg<OPTCCR__SPEC>`
pub type OPTCCR_ = crate::Reg<optccr_::OPTCCR__SPEC>;
///FLASH option clear control register
pub mod optccr_;
///OPTCCR (w) register accessor: an alias for `Reg<OPTCCR_SPEC>`
pub type OPTCCR = crate::Reg<optccr::OPTCCR_SPEC>;
///FLASH option clear control register
pub mod optccr;
///BOOT_CURR (r) register accessor: an alias for `Reg<BOOT_CURR_SPEC>`
pub type BOOT_CURR = crate::Reg<boot_curr::BOOT_CURR_SPEC>;
///FLASH register with boot address
pub mod boot_curr;
///BOOT_PRGR (rw) register accessor: an alias for `Reg<BOOT_PRGR_SPEC>`
pub type BOOT_PRGR = crate::Reg<boot_prgr::BOOT_PRGR_SPEC>;
///FLASH register with boot address
pub mod boot_prgr;
///CRCDATAR (rw) register accessor: an alias for `Reg<CRCDATAR_SPEC>`
pub type CRCDATAR = crate::Reg<crcdatar::CRCDATAR_SPEC>;
///FLASH CRC data register
pub mod crcdatar;
