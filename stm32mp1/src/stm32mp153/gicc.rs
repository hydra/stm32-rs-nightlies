///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - GICC control register
    pub gicc_ctlr: GICC_CTLR,
    ///0x04 - GICC input priority mask register
    pub gicc_pmr: GICC_PMR,
    ///0x08 - GICC binary point register
    pub gicc_bpr: GICC_BPR,
    ///0x0c - GICC interrupt acknowledge register
    pub gicc_iar: GICC_IAR,
    ///0x10 - GICC end of interrupt register
    pub gicc_eoir: GICC_EOIR,
    ///0x14 - GICC running priority register
    pub gicc_rpr: GICC_RPR,
    ///0x18 - GICC highest priority pending interrupt register
    pub gicc_hppir: GICC_HPPIR,
    ///0x1c - GICC_ABPR is an alias of the non-secure GICC_BPR. When GICC_CTLR.CBPR is set to 0, a secure access to this register is equivalent to a non-secure access to GICC_BPR.
    pub gicc_abpr: GICC_ABPR,
    ///0x20 - GICC_AIAR is an alias of the non-secure view of GICC_IAR. A secure access to this register is identical to a non-secure access to GICC_IAR.
    pub gicc_aiar: GICC_AIAR,
    ///0x24 - GICC_AEOIR is an alias of the Non-secure GICC_EOIR. A secure access to this register is similar to a non-secure access to GICC_EOIR, except that the GICC_CTLR.EOImodeS bit is used.
    pub gicc_aeoir: GICC_AEOIR,
    ///0x28 - ICC_AHPPIR is an alias of the non-secure GICC_HPPIR. A secure access to this register is equivalent to a non-secure access to GICC_HPPIR.
    pub gicc_ahppir: GICC_AHPPIR,
    _reserved11: [u8; 0xa4],
    ///0xd0 - GICC active priority register
    pub gicc_apr0: GICC_APR0,
    _reserved12: [u8; 0x0c],
    ///0xe0 - GICC non-secure active priority register
    pub gicc_nsapr0: GICC_NSAPR0,
    _reserved13: [u8; 0x18],
    ///0xfc - GICC interface identification register
    pub gicc_iidr: GICC_IIDR,
    _reserved14: [u8; 0x0f00],
    ///0x1000 - GICC deactivate interrupt register
    pub gicc_dir: GICC_DIR,
}
///GICC_CTLR (rw) register accessor: an alias for `Reg<GICC_CTLR_SPEC>`
pub type GICC_CTLR = crate::Reg<gicc_ctlr::GICC_CTLR_SPEC>;
///GICC control register
pub mod gicc_ctlr;
///GICC_PMR (rw) register accessor: an alias for `Reg<GICC_PMR_SPEC>`
pub type GICC_PMR = crate::Reg<gicc_pmr::GICC_PMR_SPEC>;
///GICC input priority mask register
pub mod gicc_pmr;
///GICC_BPR (rw) register accessor: an alias for `Reg<GICC_BPR_SPEC>`
pub type GICC_BPR = crate::Reg<gicc_bpr::GICC_BPR_SPEC>;
///GICC binary point register
pub mod gicc_bpr;
///GICC_IAR (r) register accessor: an alias for `Reg<GICC_IAR_SPEC>`
pub type GICC_IAR = crate::Reg<gicc_iar::GICC_IAR_SPEC>;
///GICC interrupt acknowledge register
pub mod gicc_iar;
///GICC_EOIR (w) register accessor: an alias for `Reg<GICC_EOIR_SPEC>`
pub type GICC_EOIR = crate::Reg<gicc_eoir::GICC_EOIR_SPEC>;
///GICC end of interrupt register
pub mod gicc_eoir;
///GICC_RPR (r) register accessor: an alias for `Reg<GICC_RPR_SPEC>`
pub type GICC_RPR = crate::Reg<gicc_rpr::GICC_RPR_SPEC>;
///GICC running priority register
pub mod gicc_rpr;
///GICC_HPPIR (r) register accessor: an alias for `Reg<GICC_HPPIR_SPEC>`
pub type GICC_HPPIR = crate::Reg<gicc_hppir::GICC_HPPIR_SPEC>;
///GICC highest priority pending interrupt register
pub mod gicc_hppir;
///GICC_ABPR (rw) register accessor: an alias for `Reg<GICC_ABPR_SPEC>`
pub type GICC_ABPR = crate::Reg<gicc_abpr::GICC_ABPR_SPEC>;
///GICC_ABPR is an alias of the non-secure GICC_BPR. When GICC_CTLR.CBPR is set to 0, a secure access to this register is equivalent to a non-secure access to GICC_BPR.
pub mod gicc_abpr;
///GICC_AIAR (r) register accessor: an alias for `Reg<GICC_AIAR_SPEC>`
pub type GICC_AIAR = crate::Reg<gicc_aiar::GICC_AIAR_SPEC>;
///GICC_AIAR is an alias of the non-secure view of GICC_IAR. A secure access to this register is identical to a non-secure access to GICC_IAR.
pub mod gicc_aiar;
///GICC_AEOIR (w) register accessor: an alias for `Reg<GICC_AEOIR_SPEC>`
pub type GICC_AEOIR = crate::Reg<gicc_aeoir::GICC_AEOIR_SPEC>;
///GICC_AEOIR is an alias of the Non-secure GICC_EOIR. A secure access to this register is similar to a non-secure access to GICC_EOIR, except that the GICC_CTLR.EOImodeS bit is used.
pub mod gicc_aeoir;
///GICC_AHPPIR (r) register accessor: an alias for `Reg<GICC_AHPPIR_SPEC>`
pub type GICC_AHPPIR = crate::Reg<gicc_ahppir::GICC_AHPPIR_SPEC>;
///ICC_AHPPIR is an alias of the non-secure GICC_HPPIR. A secure access to this register is equivalent to a non-secure access to GICC_HPPIR.
pub mod gicc_ahppir;
///GICC_APR0 (rw) register accessor: an alias for `Reg<GICC_APR0_SPEC>`
pub type GICC_APR0 = crate::Reg<gicc_apr0::GICC_APR0_SPEC>;
///GICC active priority register
pub mod gicc_apr0;
///GICC_NSAPR0 (rw) register accessor: an alias for `Reg<GICC_NSAPR0_SPEC>`
pub type GICC_NSAPR0 = crate::Reg<gicc_nsapr0::GICC_NSAPR0_SPEC>;
///GICC non-secure active priority register
pub mod gicc_nsapr0;
///GICC_IIDR (r) register accessor: an alias for `Reg<GICC_IIDR_SPEC>`
pub type GICC_IIDR = crate::Reg<gicc_iidr::GICC_IIDR_SPEC>;
///GICC interface identification register
pub mod gicc_iidr;
///GICC_DIR (w) register accessor: an alias for `Reg<GICC_DIR_SPEC>`
pub type GICC_DIR = crate::Reg<gicc_dir::GICC_DIR_SPEC>;
///GICC deactivate interrupt register
pub mod gicc_dir;
