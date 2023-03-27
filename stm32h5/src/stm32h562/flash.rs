///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - FLASH access control register
    pub acr: ACR,
    ///0x04 - FLASH non-secure key register
    pub nskeyr: NSKEYR,
    ///0x08 - FLASH secure key register
    pub seckeyr: SECKEYR,
    ///0x0c - FLASH option key register
    pub optkeyr: OPTKEYR,
    ///0x10 - FLASH non-secure OBK key register
    pub nsobkkeyr: NSOBKKEYR,
    ///0x14 - FLASH secure OBK key register
    pub secobkkeyr: SECOBKKEYR,
    ///0x18 - FLASH operation status register
    pub opsr: OPSR,
    ///0x1c - FLASH option control register
    pub optcr: OPTCR,
    ///0x20 - FLASH non-secure status register
    pub nssr: NSSR,
    ///0x24 - FLASH secure status register
    pub secsr: SECSR,
    ///0x28 - FLASH non-secure control register
    pub nscr: NSCR,
    ///0x2c - FLASH secure control register
    pub seccr: SECCR,
    ///0x30 - FLASH non-secure clear control register
    pub nsccr: NSCCR,
    ///0x34 - FLASH secure clear control register
    pub secccr: SECCCR,
    _reserved14: [u8; 0x04],
    ///0x3c - FLASH privilege configuration register
    pub privcfgr: PRIVCFGR,
    ///0x40 - FLASH non-secure OBK configuration register
    pub nsobkcfgr: NSOBKCFGR,
    ///0x44 - FLASH secure OBK configuration register
    pub secobkcfgr: SECOBKCFGR,
    ///0x48 - FLASH HDP extension register
    pub hdpextr: HDPEXTR,
    _reserved18: [u8; 0x04],
    ///0x50 - FLASH option status register
    pub optsr_cur: OPTSR_CUR,
    ///0x54 - FLASH option status register
    pub optsr_prg: OPTSR_PRG,
    _reserved20: [u8; 0x08],
    ///0x60 - FLASH non-secure EPOCH register
    pub nsepochr_cur: NSEPOCHR_CUR,
    _reserved21: [u8; 0x04],
    ///0x68 - FLASH secure EPOCH register
    pub secepochr_cur: SECEPOCHR_CUR,
    _reserved22: [u8; 0x04],
    ///0x70 - FLASH option status register 2
    pub optsr2_cur: OPTSR2_CUR,
    ///0x74 - FLASH option status register 2
    pub optsr2_prg: OPTSR2_PRG,
    _reserved24: [u8; 0x08],
    ///0x80 - FLASH non-secure boot register
    pub nsbootr_cur: NSBOOTR_CUR,
    ///0x84 - FLASH non-secure boot register
    pub nsbootr_prg: NSBOOTR_PRG,
    ///0x88 - FLASH secure boot register
    pub secbootr_cur: SECBOOTR_CUR,
    ///0x8c - FLASH secure boot register
    pub bootr_prg: BOOTR_PRG,
    ///0x90 - FLASH non-secure OTP block lock
    pub otpblr_cur: OTPBLR_CUR,
    ///0x94 - FLASH non-secure OTP block lock
    pub otpblr_prg: OTPBLR_PRG,
    _reserved30: [u8; 0x08],
    ///0xa0 - FLASH secure block based register for Bank 1
    pub secbb1r1: SECBB1R1,
    ///0xa4 - FLASH secure block based register for Bank 1
    pub secbb1r2: SECBB1R2,
    ///0xa8 - FLASH secure block based register for Bank 1
    pub secbb1r3: SECBB1R3,
    ///0xac - FLASH secure block based register for Bank 1
    pub secbb1r4: SECBB1R4,
    _reserved34: [u8; 0x10],
    ///0xc0 - FLASH privilege block based register for Bank 1
    pub privbb1r1: PRIVBB1R1,
    ///0xc4 - FLASH privilege block based register for Bank 1
    pub privbb1r2: PRIVBB1R2,
    ///0xc8 - FLASH privilege block based register for Bank 1
    pub privbb1r3: PRIVBB1R3,
    ///0xcc - FLASH privilege block based register for Bank 1
    pub privbb1r4: PRIVBB1R4,
    _reserved38: [u8; 0x10],
    ///0xe0 - FLASH security watermark for Bank 1
    pub secwm1r_cur: SECWM1R_CUR,
    ///0xe4 - FLASH security watermark for Bank 1
    pub secwm1r_prg: SECWM1R_PRG,
    ///0xe8 - FLASH write sector group protection for Bank 1
    pub wrp1r_cur: WRP1R_CUR,
    ///0xec - FLASH write sector group protection for Bank 1
    pub wrp1r_prg: WRP1R_PRG,
    ///0xf0 - FLASH data sector configuration Bank 1
    pub edata1r_cur: EDATA1R_CUR,
    ///0xf4 - FLASH data sector configuration Bank 1
    pub edata1r_prg: EDATA1R_PRG,
    ///0xf8 - FLASH HDP Bank 1 configuration
    pub hdp1r_cur: HDP1R_CUR,
    ///0xfc - FLASH HDP Bank 1 configuration
    pub hdp1r_prg: HDP1R_PRG,
    ///0x100 - FLASH ECC correction register
    pub ecccorr: ECCCORR,
    ///0x104 - FLASH ECC detection register
    pub eccdetr: ECCDETR,
    ///0x108 - FLASH ECC data
    pub eccdr: ECCDR,
    _reserved49: [u8; 0x94],
    ///0x1a0 - FLASH secure block-based register for Bank 2
    pub secbb2r1: SECBB2R1,
    ///0x1a4 - FLASH secure block-based register for Bank 2
    pub secbb2r2: SECBB2R2,
    ///0x1a8 - FLASH secure block-based register for Bank 2
    pub secbb2r3: SECBB2R3,
    ///0x1ac - FLASH secure block-based register for Bank 2
    pub secbb2r4: SECBB2R4,
    _reserved53: [u8; 0x10],
    ///0x1c0 - FLASH privilege block-based register for Bank 2
    pub privbb2r1: PRIVBB2R1,
    ///0x1c4 - FLASH privilege block-based register for Bank 2
    pub privbb2r2: PRIVBB2R2,
    ///0x1c8 - FLASH privilege block-based register for Bank 2
    pub privbb2r3: PRIVBB2R3,
    ///0x1cc - FLASH privilege block-based register for Bank 2
    pub privbb2r4: PRIVBB2R4,
    _reserved57: [u8; 0x10],
    ///0x1e0 - FLASH security watermark for Bank 2
    pub secwm2r_cur: SECWM2R_CUR,
    ///0x1e4 - FLASH security watermark for Bank 2
    pub secwm2r_prg: SECWM2R_PRG,
    ///0x1e8 - FLASH write sector group protection for Bank 2
    pub wrp2r_cur: WRP2R_CUR,
    ///0x1ec - FLASH write sector group protection for Bank 2
    pub wrp2r_prg: WRP2R_PRG,
    ///0x1f0 - FLASH data sectors configuration Bank 2
    pub edata2r_cur: EDATA2R_CUR,
    ///0x1f4 - FLASH data sector configuration Bank 2
    pub edata2r_prg: EDATA2R_PRG,
    ///0x1f8 - FLASH HDP Bank 2 configuration
    pub hdp2r_cur: HDP2R_CUR,
    ///0x1fc - FLASH HDP Bank 2 configuration
    pub hdp2r_prg: HDP2R_PRG,
}
///ACR (rw) register accessor: an alias for `Reg<ACR_SPEC>`
pub type ACR = crate::Reg<acr::ACR_SPEC>;
///FLASH access control register
pub mod acr;
///NSKEYR (w) register accessor: an alias for `Reg<NSKEYR_SPEC>`
pub type NSKEYR = crate::Reg<nskeyr::NSKEYR_SPEC>;
///FLASH non-secure key register
pub mod nskeyr;
///SECKEYR (w) register accessor: an alias for `Reg<SECKEYR_SPEC>`
pub type SECKEYR = crate::Reg<seckeyr::SECKEYR_SPEC>;
///FLASH secure key register
pub mod seckeyr;
///OPTKEYR (w) register accessor: an alias for `Reg<OPTKEYR_SPEC>`
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYR_SPEC>;
///FLASH option key register
pub mod optkeyr;
///NSOBKKEYR (w) register accessor: an alias for `Reg<NSOBKKEYR_SPEC>`
pub type NSOBKKEYR = crate::Reg<nsobkkeyr::NSOBKKEYR_SPEC>;
///FLASH non-secure OBK key register
pub mod nsobkkeyr;
///SECOBKKEYR (w) register accessor: an alias for `Reg<SECOBKKEYR_SPEC>`
pub type SECOBKKEYR = crate::Reg<secobkkeyr::SECOBKKEYR_SPEC>;
///FLASH secure OBK key register
pub mod secobkkeyr;
///OPSR (r) register accessor: an alias for `Reg<OPSR_SPEC>`
pub type OPSR = crate::Reg<opsr::OPSR_SPEC>;
///FLASH operation status register
pub mod opsr;
///OPTCR (rw) register accessor: an alias for `Reg<OPTCR_SPEC>`
pub type OPTCR = crate::Reg<optcr::OPTCR_SPEC>;
///FLASH option control register
pub mod optcr;
///NSSR (r) register accessor: an alias for `Reg<NSSR_SPEC>`
pub type NSSR = crate::Reg<nssr::NSSR_SPEC>;
///FLASH non-secure status register
pub mod nssr;
///SECSR (r) register accessor: an alias for `Reg<SECSR_SPEC>`
pub type SECSR = crate::Reg<secsr::SECSR_SPEC>;
///FLASH secure status register
pub mod secsr;
///NSCR (rw) register accessor: an alias for `Reg<NSCR_SPEC>`
pub type NSCR = crate::Reg<nscr::NSCR_SPEC>;
///FLASH non-secure control register
pub mod nscr;
///SECCR (rw) register accessor: an alias for `Reg<SECCR_SPEC>`
pub type SECCR = crate::Reg<seccr::SECCR_SPEC>;
///FLASH secure control register
pub mod seccr;
///NSCCR (w) register accessor: an alias for `Reg<NSCCR_SPEC>`
pub type NSCCR = crate::Reg<nsccr::NSCCR_SPEC>;
///FLASH non-secure clear control register
pub mod nsccr;
///SECCCR (w) register accessor: an alias for `Reg<SECCCR_SPEC>`
pub type SECCCR = crate::Reg<secccr::SECCCR_SPEC>;
///FLASH secure clear control register
pub mod secccr;
///PRIVCFGR (rw) register accessor: an alias for `Reg<PRIVCFGR_SPEC>`
pub type PRIVCFGR = crate::Reg<privcfgr::PRIVCFGR_SPEC>;
///FLASH privilege configuration register
pub mod privcfgr;
///NSOBKCFGR (rw) register accessor: an alias for `Reg<NSOBKCFGR_SPEC>`
pub type NSOBKCFGR = crate::Reg<nsobkcfgr::NSOBKCFGR_SPEC>;
///FLASH non-secure OBK configuration register
pub mod nsobkcfgr;
///SECOBKCFGR (rw) register accessor: an alias for `Reg<SECOBKCFGR_SPEC>`
pub type SECOBKCFGR = crate::Reg<secobkcfgr::SECOBKCFGR_SPEC>;
///FLASH secure OBK configuration register
pub mod secobkcfgr;
///HDPEXTR (rw) register accessor: an alias for `Reg<HDPEXTR_SPEC>`
pub type HDPEXTR = crate::Reg<hdpextr::HDPEXTR_SPEC>;
///FLASH HDP extension register
pub mod hdpextr;
///OPTSR_CUR (r) register accessor: an alias for `Reg<OPTSR_CUR_SPEC>`
pub type OPTSR_CUR = crate::Reg<optsr_cur::OPTSR_CUR_SPEC>;
///FLASH option status register
pub mod optsr_cur;
///OPTSR_PRG (rw) register accessor: an alias for `Reg<OPTSR_PRG_SPEC>`
pub type OPTSR_PRG = crate::Reg<optsr_prg::OPTSR_PRG_SPEC>;
///FLASH option status register
pub mod optsr_prg;
///NSEPOCHR_CUR (r) register accessor: an alias for `Reg<NSEPOCHR_CUR_SPEC>`
pub type NSEPOCHR_CUR = crate::Reg<nsepochr_cur::NSEPOCHR_CUR_SPEC>;
///FLASH non-secure EPOCH register
pub mod nsepochr_cur;
///SECEPOCHR_CUR (r) register accessor: an alias for `Reg<SECEPOCHR_CUR_SPEC>`
pub type SECEPOCHR_CUR = crate::Reg<secepochr_cur::SECEPOCHR_CUR_SPEC>;
///FLASH secure EPOCH register
pub mod secepochr_cur;
///OPTSR2_CUR (r) register accessor: an alias for `Reg<OPTSR2_CUR_SPEC>`
pub type OPTSR2_CUR = crate::Reg<optsr2_cur::OPTSR2_CUR_SPEC>;
///FLASH option status register 2
pub mod optsr2_cur;
///OPTSR2_PRG (rw) register accessor: an alias for `Reg<OPTSR2_PRG_SPEC>`
pub type OPTSR2_PRG = crate::Reg<optsr2_prg::OPTSR2_PRG_SPEC>;
///FLASH option status register 2
pub mod optsr2_prg;
///NSBOOTR_CUR (r) register accessor: an alias for `Reg<NSBOOTR_CUR_SPEC>`
pub type NSBOOTR_CUR = crate::Reg<nsbootr_cur::NSBOOTR_CUR_SPEC>;
///FLASH non-secure boot register
pub mod nsbootr_cur;
///NSBOOTR_PRG (rw) register accessor: an alias for `Reg<NSBOOTR_PRG_SPEC>`
pub type NSBOOTR_PRG = crate::Reg<nsbootr_prg::NSBOOTR_PRG_SPEC>;
///FLASH non-secure boot register
pub mod nsbootr_prg;
///SECBOOTR_CUR (r) register accessor: an alias for `Reg<SECBOOTR_CUR_SPEC>`
pub type SECBOOTR_CUR = crate::Reg<secbootr_cur::SECBOOTR_CUR_SPEC>;
///FLASH secure boot register
pub mod secbootr_cur;
///BOOTR_PRG (rw) register accessor: an alias for `Reg<BOOTR_PRG_SPEC>`
pub type BOOTR_PRG = crate::Reg<bootr_prg::BOOTR_PRG_SPEC>;
///FLASH secure boot register
pub mod bootr_prg;
///OTPBLR_CUR (r) register accessor: an alias for `Reg<OTPBLR_CUR_SPEC>`
pub type OTPBLR_CUR = crate::Reg<otpblr_cur::OTPBLR_CUR_SPEC>;
///FLASH non-secure OTP block lock
pub mod otpblr_cur;
///OTPBLR_PRG (rw) register accessor: an alias for `Reg<OTPBLR_PRG_SPEC>`
pub type OTPBLR_PRG = crate::Reg<otpblr_prg::OTPBLR_PRG_SPEC>;
///FLASH non-secure OTP block lock
pub mod otpblr_prg;
///SECBB1R1 (rw) register accessor: an alias for `Reg<SECBB1R1_SPEC>`
pub type SECBB1R1 = crate::Reg<secbb1r1::SECBB1R1_SPEC>;
///FLASH secure block based register for Bank 1
pub mod secbb1r1;
///SECBB1R2 (rw) register accessor: an alias for `Reg<SECBB1R2_SPEC>`
pub type SECBB1R2 = crate::Reg<secbb1r2::SECBB1R2_SPEC>;
///FLASH secure block based register for Bank 1
pub mod secbb1r2;
///SECBB1R3 (rw) register accessor: an alias for `Reg<SECBB1R3_SPEC>`
pub type SECBB1R3 = crate::Reg<secbb1r3::SECBB1R3_SPEC>;
///FLASH secure block based register for Bank 1
pub mod secbb1r3;
///SECBB1R4 (rw) register accessor: an alias for `Reg<SECBB1R4_SPEC>`
pub type SECBB1R4 = crate::Reg<secbb1r4::SECBB1R4_SPEC>;
///FLASH secure block based register for Bank 1
pub mod secbb1r4;
///PRIVBB1R1 (rw) register accessor: an alias for `Reg<PRIVBB1R1_SPEC>`
pub type PRIVBB1R1 = crate::Reg<privbb1r1::PRIVBB1R1_SPEC>;
///FLASH privilege block based register for Bank 1
pub mod privbb1r1;
///PRIVBB1R2 (rw) register accessor: an alias for `Reg<PRIVBB1R2_SPEC>`
pub type PRIVBB1R2 = crate::Reg<privbb1r2::PRIVBB1R2_SPEC>;
///FLASH privilege block based register for Bank 1
pub mod privbb1r2;
///PRIVBB1R3 (rw) register accessor: an alias for `Reg<PRIVBB1R3_SPEC>`
pub type PRIVBB1R3 = crate::Reg<privbb1r3::PRIVBB1R3_SPEC>;
///FLASH privilege block based register for Bank 1
pub mod privbb1r3;
///PRIVBB1R4 (rw) register accessor: an alias for `Reg<PRIVBB1R4_SPEC>`
pub type PRIVBB1R4 = crate::Reg<privbb1r4::PRIVBB1R4_SPEC>;
///FLASH privilege block based register for Bank 1
pub mod privbb1r4;
///SECWM1R_CUR (r) register accessor: an alias for `Reg<SECWM1R_CUR_SPEC>`
pub type SECWM1R_CUR = crate::Reg<secwm1r_cur::SECWM1R_CUR_SPEC>;
///FLASH security watermark for Bank 1
pub mod secwm1r_cur;
///SECWM1R_PRG (rw) register accessor: an alias for `Reg<SECWM1R_PRG_SPEC>`
pub type SECWM1R_PRG = crate::Reg<secwm1r_prg::SECWM1R_PRG_SPEC>;
///FLASH security watermark for Bank 1
pub mod secwm1r_prg;
///WRP1R_CUR (r) register accessor: an alias for `Reg<WRP1R_CUR_SPEC>`
pub type WRP1R_CUR = crate::Reg<wrp1r_cur::WRP1R_CUR_SPEC>;
///FLASH write sector group protection for Bank 1
pub mod wrp1r_cur;
///WRP1R_PRG (rw) register accessor: an alias for `Reg<WRP1R_PRG_SPEC>`
pub type WRP1R_PRG = crate::Reg<wrp1r_prg::WRP1R_PRG_SPEC>;
///FLASH write sector group protection for Bank 1
pub mod wrp1r_prg;
///EDATA1R_CUR (r) register accessor: an alias for `Reg<EDATA1R_CUR_SPEC>`
pub type EDATA1R_CUR = crate::Reg<edata1r_cur::EDATA1R_CUR_SPEC>;
///FLASH data sector configuration Bank 1
pub mod edata1r_cur;
///EDATA1R_PRG (rw) register accessor: an alias for `Reg<EDATA1R_PRG_SPEC>`
pub type EDATA1R_PRG = crate::Reg<edata1r_prg::EDATA1R_PRG_SPEC>;
///FLASH data sector configuration Bank 1
pub mod edata1r_prg;
///HDP1R_CUR (r) register accessor: an alias for `Reg<HDP1R_CUR_SPEC>`
pub type HDP1R_CUR = crate::Reg<hdp1r_cur::HDP1R_CUR_SPEC>;
///FLASH HDP Bank 1 configuration
pub mod hdp1r_cur;
///HDP1R_PRG (rw) register accessor: an alias for `Reg<HDP1R_PRG_SPEC>`
pub type HDP1R_PRG = crate::Reg<hdp1r_prg::HDP1R_PRG_SPEC>;
///FLASH HDP Bank 1 configuration
pub mod hdp1r_prg;
///ECCCORR (rw) register accessor: an alias for `Reg<ECCCORR_SPEC>`
pub type ECCCORR = crate::Reg<ecccorr::ECCCORR_SPEC>;
///FLASH ECC correction register
pub mod ecccorr;
///ECCDETR (rw) register accessor: an alias for `Reg<ECCDETR_SPEC>`
pub type ECCDETR = crate::Reg<eccdetr::ECCDETR_SPEC>;
///FLASH ECC detection register
pub mod eccdetr;
///ECCDR (r) register accessor: an alias for `Reg<ECCDR_SPEC>`
pub type ECCDR = crate::Reg<eccdr::ECCDR_SPEC>;
///FLASH ECC data
pub mod eccdr;
///SECBB2R1 (rw) register accessor: an alias for `Reg<SECBB2R1_SPEC>`
pub type SECBB2R1 = crate::Reg<secbb2r1::SECBB2R1_SPEC>;
///FLASH secure block-based register for Bank 2
pub mod secbb2r1;
///SECBB2R2 (rw) register accessor: an alias for `Reg<SECBB2R2_SPEC>`
pub type SECBB2R2 = crate::Reg<secbb2r2::SECBB2R2_SPEC>;
///FLASH secure block-based register for Bank 2
pub mod secbb2r2;
///SECBB2R3 (rw) register accessor: an alias for `Reg<SECBB2R3_SPEC>`
pub type SECBB2R3 = crate::Reg<secbb2r3::SECBB2R3_SPEC>;
///FLASH secure block-based register for Bank 2
pub mod secbb2r3;
///SECBB2R4 (rw) register accessor: an alias for `Reg<SECBB2R4_SPEC>`
pub type SECBB2R4 = crate::Reg<secbb2r4::SECBB2R4_SPEC>;
///FLASH secure block-based register for Bank 2
pub mod secbb2r4;
///PRIVBB2R1 (rw) register accessor: an alias for `Reg<PRIVBB2R1_SPEC>`
pub type PRIVBB2R1 = crate::Reg<privbb2r1::PRIVBB2R1_SPEC>;
///FLASH privilege block-based register for Bank 2
pub mod privbb2r1;
///PRIVBB2R2 (rw) register accessor: an alias for `Reg<PRIVBB2R2_SPEC>`
pub type PRIVBB2R2 = crate::Reg<privbb2r2::PRIVBB2R2_SPEC>;
///FLASH privilege block-based register for Bank 2
pub mod privbb2r2;
///PRIVBB2R3 (rw) register accessor: an alias for `Reg<PRIVBB2R3_SPEC>`
pub type PRIVBB2R3 = crate::Reg<privbb2r3::PRIVBB2R3_SPEC>;
///FLASH privilege block-based register for Bank 2
pub mod privbb2r3;
///PRIVBB2R4 (rw) register accessor: an alias for `Reg<PRIVBB2R4_SPEC>`
pub type PRIVBB2R4 = crate::Reg<privbb2r4::PRIVBB2R4_SPEC>;
///FLASH privilege block-based register for Bank 2
pub mod privbb2r4;
///SECWM2R_CUR (r) register accessor: an alias for `Reg<SECWM2R_CUR_SPEC>`
pub type SECWM2R_CUR = crate::Reg<secwm2r_cur::SECWM2R_CUR_SPEC>;
///FLASH security watermark for Bank 2
pub mod secwm2r_cur;
///SECWM2R_PRG (rw) register accessor: an alias for `Reg<SECWM2R_PRG_SPEC>`
pub type SECWM2R_PRG = crate::Reg<secwm2r_prg::SECWM2R_PRG_SPEC>;
///FLASH security watermark for Bank 2
pub mod secwm2r_prg;
///WRP2R_CUR (r) register accessor: an alias for `Reg<WRP2R_CUR_SPEC>`
pub type WRP2R_CUR = crate::Reg<wrp2r_cur::WRP2R_CUR_SPEC>;
///FLASH write sector group protection for Bank 2
pub mod wrp2r_cur;
///WRP2R_PRG (rw) register accessor: an alias for `Reg<WRP2R_PRG_SPEC>`
pub type WRP2R_PRG = crate::Reg<wrp2r_prg::WRP2R_PRG_SPEC>;
///FLASH write sector group protection for Bank 2
pub mod wrp2r_prg;
///EDATA2R_CUR (r) register accessor: an alias for `Reg<EDATA2R_CUR_SPEC>`
pub type EDATA2R_CUR = crate::Reg<edata2r_cur::EDATA2R_CUR_SPEC>;
///FLASH data sectors configuration Bank 2
pub mod edata2r_cur;
///EDATA2R_PRG (rw) register accessor: an alias for `Reg<EDATA2R_PRG_SPEC>`
pub type EDATA2R_PRG = crate::Reg<edata2r_prg::EDATA2R_PRG_SPEC>;
///FLASH data sector configuration Bank 2
pub mod edata2r_prg;
///HDP2R_CUR (r) register accessor: an alias for `Reg<HDP2R_CUR_SPEC>`
pub type HDP2R_CUR = crate::Reg<hdp2r_cur::HDP2R_CUR_SPEC>;
///FLASH HDP Bank 2 configuration
pub mod hdp2r_cur;
///HDP2R_PRG (rw) register accessor: an alias for `Reg<HDP2R_PRG_SPEC>`
pub type HDP2R_PRG = crate::Reg<hdp2r_prg::HDP2R_PRG_SPEC>;
///FLASH HDP Bank 2 configuration
pub mod hdp2r_prg;
