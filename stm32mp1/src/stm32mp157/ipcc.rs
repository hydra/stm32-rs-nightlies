///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - IPCC Processor 1 control register
    pub ipcc_c1cr: IPCC_C1CR,
    ///0x04 - IPCC Processor 1 mask register
    pub ipcc_c1mr: IPCC_C1MR,
    ///0x08 - Reading this register will always return 0x0000 0000.
    pub ipcc_c1scr: IPCC_C1SCR,
    ///0x0c - IPCC processor 1 to processor 2 status register
    pub ipcc_c1toc2sr: IPCC_C1TOC2SR,
    ///0x10 - IPCC Processor 2 control register
    pub ipcc_c2cr: IPCC_C2CR,
    ///0x14 - IPCC Processor 2 mask register
    pub ipcc_c2mr: IPCC_C2MR,
    ///0x18 - Reading this register will always return 0x0000 0000.
    pub ipcc_c2scr: IPCC_C2SCR,
    ///0x1c - IPCC processor 2 to processor 1 status register
    pub ipcc_c2toc1sr: IPCC_C2TOC1SR,
    _reserved8: [u8; 0x03d0],
    ///0x3f0 - IPCC Hardware configuration register
    pub ipcc_hwcfgr: IPCC_HWCFGR,
    ///0x3f4 - IPCC IP Version register
    pub ipcc_ver: IPCC_VER,
    ///0x3f8 - IPCC IP Identification register
    pub ipcc_id: IPCC_ID,
    ///0x3fc - IPCC Size ID register
    pub ipcc_sid: IPCC_SID,
}
///IPCC_C1CR (rw) register accessor: an alias for `Reg<IPCC_C1CR_SPEC>`
pub type IPCC_C1CR = crate::Reg<ipcc_c1cr::IPCC_C1CR_SPEC>;
///IPCC Processor 1 control register
pub mod ipcc_c1cr;
///IPCC_C1MR (rw) register accessor: an alias for `Reg<IPCC_C1MR_SPEC>`
pub type IPCC_C1MR = crate::Reg<ipcc_c1mr::IPCC_C1MR_SPEC>;
///IPCC Processor 1 mask register
pub mod ipcc_c1mr;
///IPCC_C1SCR (rw) register accessor: an alias for `Reg<IPCC_C1SCR_SPEC>`
pub type IPCC_C1SCR = crate::Reg<ipcc_c1scr::IPCC_C1SCR_SPEC>;
///Reading this register will always return 0x0000 0000.
pub mod ipcc_c1scr;
///IPCC_C1TOC2SR (r) register accessor: an alias for `Reg<IPCC_C1TOC2SR_SPEC>`
pub type IPCC_C1TOC2SR = crate::Reg<ipcc_c1toc2sr::IPCC_C1TOC2SR_SPEC>;
///IPCC processor 1 to processor 2 status register
pub mod ipcc_c1toc2sr;
///IPCC_C2CR (rw) register accessor: an alias for `Reg<IPCC_C2CR_SPEC>`
pub type IPCC_C2CR = crate::Reg<ipcc_c2cr::IPCC_C2CR_SPEC>;
///IPCC Processor 2 control register
pub mod ipcc_c2cr;
///IPCC_C2MR (rw) register accessor: an alias for `Reg<IPCC_C2MR_SPEC>`
pub type IPCC_C2MR = crate::Reg<ipcc_c2mr::IPCC_C2MR_SPEC>;
///IPCC Processor 2 mask register
pub mod ipcc_c2mr;
///IPCC_C2SCR (rw) register accessor: an alias for `Reg<IPCC_C2SCR_SPEC>`
pub type IPCC_C2SCR = crate::Reg<ipcc_c2scr::IPCC_C2SCR_SPEC>;
///Reading this register will always return 0x0000 0000.
pub mod ipcc_c2scr;
///IPCC_C2TOC1SR (r) register accessor: an alias for `Reg<IPCC_C2TOC1SR_SPEC>`
pub type IPCC_C2TOC1SR = crate::Reg<ipcc_c2toc1sr::IPCC_C2TOC1SR_SPEC>;
///IPCC processor 2 to processor 1 status register
pub mod ipcc_c2toc1sr;
///IPCC_HWCFGR (r) register accessor: an alias for `Reg<IPCC_HWCFGR_SPEC>`
pub type IPCC_HWCFGR = crate::Reg<ipcc_hwcfgr::IPCC_HWCFGR_SPEC>;
///IPCC Hardware configuration register
pub mod ipcc_hwcfgr;
///IPCC_VER (r) register accessor: an alias for `Reg<IPCC_VER_SPEC>`
pub type IPCC_VER = crate::Reg<ipcc_ver::IPCC_VER_SPEC>;
///IPCC IP Version register
pub mod ipcc_ver;
///IPCC_ID (r) register accessor: an alias for `Reg<IPCC_ID_SPEC>`
pub type IPCC_ID = crate::Reg<ipcc_id::IPCC_ID_SPEC>;
///IPCC IP Identification register
pub mod ipcc_id;
///IPCC_SID (r) register accessor: an alias for `Reg<IPCC_SID_SPEC>`
pub type IPCC_SID = crate::Reg<ipcc_sid::IPCC_SID_SPEC>;
///IPCC Size ID register
pub mod ipcc_sid;
