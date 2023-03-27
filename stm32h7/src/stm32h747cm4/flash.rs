///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - FLASH access control register
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
    ///0x40 - FLASH register boot address for Arm Cortex-M7 core
    #[inline(always)]
    pub const fn boot7_curr(&self) -> &BOOT7_CURR {
        unsafe { &*(self as *const Self).cast::<u8>().add(64usize).cast() }
    }
    ///0x44 - FLASH register boot address for Arm Cortex-M7 core
    #[inline(always)]
    pub const fn boot7_prgr(&self) -> &BOOT7_PRGR {
        unsafe { &*(self as *const Self).cast::<u8>().add(68usize).cast() }
    }
    ///0x48 - FLASH register boot address for Arm Cortex-M4 core
    #[inline(always)]
    pub const fn boot4_curr(&self) -> &BOOT4_CURR {
        unsafe { &*(self as *const Self).cast::<u8>().add(72usize).cast() }
    }
    ///0x4c - FLASH register boot address for Arm Cortex-M4 core
    #[inline(always)]
    pub const fn boot4_prgr(&self) -> &BOOT4_PRGR {
        unsafe { &*(self as *const Self).cast::<u8>().add(76usize).cast() }
    }
    ///0x5c - FLASH CRC data register
    #[inline(always)]
    pub const fn crcdatar(&self) -> &CRCDATAR {
        unsafe { &*(self as *const Self).cast::<u8>().add(92usize).cast() }
    }
    ///0x100 - FLASH access control register
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
    ///0x140 - FLASH register boot address for Arm Cortex-M7 core
    #[inline(always)]
    pub const fn boot7_curr_(&self) -> &BOOT7_CURR_ {
        unsafe { &*(self as *const Self).cast::<u8>().add(320usize).cast() }
    }
    ///0x144 - FLASH register boot address for Arm Cortex-M7 core
    #[inline(always)]
    pub const fn boot7_prgr_(&self) -> &BOOT7_PRGR_ {
        unsafe { &*(self as *const Self).cast::<u8>().add(324usize).cast() }
    }
    ///0x148 - FLASH register boot address for Arm Cortex-M4 core
    #[inline(always)]
    pub const fn boot4_curr_(&self) -> &BOOT4_CURR_ {
        unsafe { &*(self as *const Self).cast::<u8>().add(328usize).cast() }
    }
    ///0x14c - FLASH register boot address for Arm Cortex-M4 core
    #[inline(always)]
    pub const fn boot4_prgr_(&self) -> &BOOT4_PRGR_ {
        unsafe { &*(self as *const Self).cast::<u8>().add(332usize).cast() }
    }
}
///ACR (rw) register accessor: an alias for `Reg<ACR_SPEC>`
pub type ACR = crate::Reg<acr::ACR_SPEC>;
///FLASH access control register
pub mod acr;
///Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R
pub use self::bank::BANK;
///Cluster
///Cluster BANK%s, containing KEYR?, CR?, SR?, CCR?, PRAR_CUR?, PRAR_PRG?, SCAR_CUR?, SCAR_PRG?, WPSN_CUR?R, WPSN_PRG?R, CRCCR?, CRCSADD?R, CRCEADD?R, ECC_FA?R
pub mod bank;
///OPTKEYR (w) register accessor: an alias for `Reg<OPTKEYR_SPEC>`
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYR_SPEC>;
///FLASH option key register
pub mod optkeyr;
///OPTCR (rw) register accessor: an alias for `Reg<OPTCR_SPEC>`
pub type OPTCR = crate::Reg<optcr::OPTCR_SPEC>;
///FLASH option control register
pub mod optcr;
///OPTSR_CUR (rw) register accessor: an alias for `Reg<OPTSR_CUR_SPEC>`
pub type OPTSR_CUR = crate::Reg<optsr_cur::OPTSR_CUR_SPEC>;
///FLASH option status register
pub mod optsr_cur;
///OPTSR_PRG (rw) register accessor: an alias for `Reg<OPTSR_PRG_SPEC>`
pub type OPTSR_PRG = crate::Reg<optsr_prg::OPTSR_PRG_SPEC>;
///FLASH option status register
pub mod optsr_prg;
///OPTCCR (rw) register accessor: an alias for `Reg<OPTCCR_SPEC>`
pub type OPTCCR = crate::Reg<optccr::OPTCCR_SPEC>;
///FLASH option clear control register
pub mod optccr;
///BOOT7_CURR (rw) register accessor: an alias for `Reg<BOOT7_CURR_SPEC>`
pub type BOOT7_CURR = crate::Reg<boot7_curr::BOOT7_CURR_SPEC>;
///FLASH register boot address for Arm Cortex-M7 core
pub mod boot7_curr;
///BOOT7_PRGR (rw) register accessor: an alias for `Reg<BOOT7_PRGR_SPEC>`
pub type BOOT7_PRGR = crate::Reg<boot7_prgr::BOOT7_PRGR_SPEC>;
///FLASH register boot address for Arm Cortex-M7 core
pub mod boot7_prgr;
///BOOT4_CURR (rw) register accessor: an alias for `Reg<BOOT4_CURR_SPEC>`
pub type BOOT4_CURR = crate::Reg<boot4_curr::BOOT4_CURR_SPEC>;
///FLASH register boot address for Arm Cortex-M4 core
pub mod boot4_curr;
///BOOT4_PRGR (rw) register accessor: an alias for `Reg<BOOT4_PRGR_SPEC>`
pub type BOOT4_PRGR = crate::Reg<boot4_prgr::BOOT4_PRGR_SPEC>;
///FLASH register boot address for Arm Cortex-M4 core
pub mod boot4_prgr;
///CRCDATAR (rw) register accessor: an alias for `Reg<CRCDATAR_SPEC>`
pub type CRCDATAR = crate::Reg<crcdatar::CRCDATAR_SPEC>;
///FLASH CRC data register
pub mod crcdatar;
///ACR_ (rw) register accessor: an alias for `Reg<ACR__SPEC>`
pub type ACR_ = crate::Reg<acr_::ACR__SPEC>;
///FLASH access control register
pub mod acr_;
///OPTKEYR_ (w) register accessor: an alias for `Reg<OPTKEYR__SPEC>`
pub type OPTKEYR_ = crate::Reg<optkeyr_::OPTKEYR__SPEC>;
///FLASH option key register
pub mod optkeyr_;
///OPTCR_ (rw) register accessor: an alias for `Reg<OPTCR__SPEC>`
pub type OPTCR_ = crate::Reg<optcr_::OPTCR__SPEC>;
///FLASH option control register
pub mod optcr_;
///OPTSR_CUR_ (rw) register accessor: an alias for `Reg<OPTSR_CUR__SPEC>`
pub type OPTSR_CUR_ = crate::Reg<optsr_cur_::OPTSR_CUR__SPEC>;
///FLASH option status register
pub mod optsr_cur_;
///OPTSR_PRG_ (rw) register accessor: an alias for `Reg<OPTSR_PRG__SPEC>`
pub type OPTSR_PRG_ = crate::Reg<optsr_prg_::OPTSR_PRG__SPEC>;
///FLASH option status register
pub mod optsr_prg_;
///OPTCCR_ (rw) register accessor: an alias for `Reg<OPTCCR__SPEC>`
pub type OPTCCR_ = crate::Reg<optccr_::OPTCCR__SPEC>;
///FLASH option clear control register
pub mod optccr_;
///BOOT7_CURR_ (rw) register accessor: an alias for `Reg<BOOT7_CURR__SPEC>`
pub type BOOT7_CURR_ = crate::Reg<boot7_curr_::BOOT7_CURR__SPEC>;
///FLASH register boot address for Arm Cortex-M7 core
pub mod boot7_curr_;
///BOOT7_PRGR_ (rw) register accessor: an alias for `Reg<BOOT7_PRGR__SPEC>`
pub type BOOT7_PRGR_ = crate::Reg<boot7_prgr_::BOOT7_PRGR__SPEC>;
///FLASH register boot address for Arm Cortex-M7 core
pub mod boot7_prgr_;
///BOOT4_CURR_ (rw) register accessor: an alias for `Reg<BOOT4_CURR__SPEC>`
pub type BOOT4_CURR_ = crate::Reg<boot4_curr_::BOOT4_CURR__SPEC>;
///FLASH register boot address for Arm Cortex-M4 core
pub mod boot4_curr_;
///BOOT4_PRGR_ (rw) register accessor: an alias for `Reg<BOOT4_PRGR__SPEC>`
pub type BOOT4_PRGR_ = crate::Reg<boot4_prgr_::BOOT4_PRGR__SPEC>;
///FLASH register boot address for Arm Cortex-M4 core
pub mod boot4_prgr_;
