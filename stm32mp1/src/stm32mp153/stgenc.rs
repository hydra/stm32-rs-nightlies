///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - STGENC control register
    pub stgenc_cntcr: STGENC_CNTCR,
    ///0x04 - STGENC status register
    pub stgenc_cntsr: STGENC_CNTSR,
    ///0x08 - the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.
    pub stgenc_cntcvl: STGENC_CNTCVL,
    ///0x0c - the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.
    pub stgenc_cntcvu: STGENC_CNTCVU,
    _reserved4: [u8; 0x10],
    ///0x20 - the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.
    pub stgenc_cntfid0: STGENC_CNTFID0,
    _reserved5: [u8; 0x0fac],
    ///0xfd0 - STGENC peripheral ID4 register
    pub stgenc_pidr4: STGENC_PIDR4,
    ///0xfd4 - STGENC peripheral ID5 register
    pub stgenc_pidr5: STGENC_PIDR5,
    ///0xfd8 - STGENC peripheral ID6 register
    pub stgenc_pidr6: STGENC_PIDR6,
    ///0xfdc - STGENC peripheral ID7 register
    pub stgenc_pidr7: STGENC_PIDR7,
    ///0xfe0 - STGENC peripheral ID0 register
    pub stgenc_pidr0: STGENC_PIDR0,
    ///0xfe4 - STGENC peripheral ID1 register
    pub stgenc_pidr1: STGENC_PIDR1,
    ///0xfe8 - STGENC peripheral ID2 register
    pub stgenc_pidr2: STGENC_PIDR2,
    ///0xfec - STGENC peripheral ID3 register
    pub stgenc_pidr3: STGENC_PIDR3,
    ///0xff0 - STGENC component ID0 register
    pub stgenc_cidr0: STGENC_CIDR0,
    ///0xff4 - STGENC component ID1 register
    pub stgenc_cidr1: STGENC_CIDR1,
    ///0xff8 - STGENC component ID2 register
    pub stgenc_cidr2: STGENC_CIDR2,
    ///0xffc - STGENC component ID3 register
    pub stgenc_cidr3: STGENC_CIDR3,
}
///STGENC_CNTCR (rw) register accessor: an alias for `Reg<STGENC_CNTCR_SPEC>`
pub type STGENC_CNTCR = crate::Reg<stgenc_cntcr::STGENC_CNTCR_SPEC>;
///STGENC control register
pub mod stgenc_cntcr;
///STGENC_CNTSR (r) register accessor: an alias for `Reg<STGENC_CNTSR_SPEC>`
pub type STGENC_CNTSR = crate::Reg<stgenc_cntsr::STGENC_CNTSR_SPEC>;
///STGENC status register
pub mod stgenc_cntsr;
///STGENC_CNTCVL (rw) register accessor: an alias for `Reg<STGENC_CNTCVL_SPEC>`
pub type STGENC_CNTCVL = crate::Reg<stgenc_cntcvl::STGENC_CNTCVL_SPEC>;
///the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.
pub mod stgenc_cntcvl;
///STGENC_CNTCVU (rw) register accessor: an alias for `Reg<STGENC_CNTCVU_SPEC>`
pub type STGENC_CNTCVU = crate::Reg<stgenc_cntcvu::STGENC_CNTCVU_SPEC>;
///the control interface must clear the STGENC_CNTCR.EN bit before writing to this register.
pub mod stgenc_cntcvu;
///STGENC_CNTFID0 (rw) register accessor: an alias for `Reg<STGENC_CNTFID0_SPEC>`
pub type STGENC_CNTFID0 = crate::Reg<stgenc_cntfid0::STGENC_CNTFID0_SPEC>;
///the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.
pub mod stgenc_cntfid0;
///STGENC_PIDR4 (r) register accessor: an alias for `Reg<STGENC_PIDR4_SPEC>`
pub type STGENC_PIDR4 = crate::Reg<stgenc_pidr4::STGENC_PIDR4_SPEC>;
///STGENC peripheral ID4 register
pub mod stgenc_pidr4;
///STGENC_PIDR5 (r) register accessor: an alias for `Reg<STGENC_PIDR5_SPEC>`
pub type STGENC_PIDR5 = crate::Reg<stgenc_pidr5::STGENC_PIDR5_SPEC>;
///STGENC peripheral ID5 register
pub mod stgenc_pidr5;
///STGENC_PIDR6 (r) register accessor: an alias for `Reg<STGENC_PIDR6_SPEC>`
pub type STGENC_PIDR6 = crate::Reg<stgenc_pidr6::STGENC_PIDR6_SPEC>;
///STGENC peripheral ID6 register
pub mod stgenc_pidr6;
///STGENC_PIDR7 (r) register accessor: an alias for `Reg<STGENC_PIDR7_SPEC>`
pub type STGENC_PIDR7 = crate::Reg<stgenc_pidr7::STGENC_PIDR7_SPEC>;
///STGENC peripheral ID7 register
pub mod stgenc_pidr7;
///STGENC_PIDR0 (r) register accessor: an alias for `Reg<STGENC_PIDR0_SPEC>`
pub type STGENC_PIDR0 = crate::Reg<stgenc_pidr0::STGENC_PIDR0_SPEC>;
///STGENC peripheral ID0 register
pub mod stgenc_pidr0;
///STGENC_PIDR1 (r) register accessor: an alias for `Reg<STGENC_PIDR1_SPEC>`
pub type STGENC_PIDR1 = crate::Reg<stgenc_pidr1::STGENC_PIDR1_SPEC>;
///STGENC peripheral ID1 register
pub mod stgenc_pidr1;
///STGENC_PIDR2 (r) register accessor: an alias for `Reg<STGENC_PIDR2_SPEC>`
pub type STGENC_PIDR2 = crate::Reg<stgenc_pidr2::STGENC_PIDR2_SPEC>;
///STGENC peripheral ID2 register
pub mod stgenc_pidr2;
///STGENC_PIDR3 (r) register accessor: an alias for `Reg<STGENC_PIDR3_SPEC>`
pub type STGENC_PIDR3 = crate::Reg<stgenc_pidr3::STGENC_PIDR3_SPEC>;
///STGENC peripheral ID3 register
pub mod stgenc_pidr3;
///STGENC_CIDR0 (r) register accessor: an alias for `Reg<STGENC_CIDR0_SPEC>`
pub type STGENC_CIDR0 = crate::Reg<stgenc_cidr0::STGENC_CIDR0_SPEC>;
///STGENC component ID0 register
pub mod stgenc_cidr0;
///STGENC_CIDR1 (r) register accessor: an alias for `Reg<STGENC_CIDR1_SPEC>`
pub type STGENC_CIDR1 = crate::Reg<stgenc_cidr1::STGENC_CIDR1_SPEC>;
///STGENC component ID1 register
pub mod stgenc_cidr1;
///STGENC_CIDR2 (r) register accessor: an alias for `Reg<STGENC_CIDR2_SPEC>`
pub type STGENC_CIDR2 = crate::Reg<stgenc_cidr2::STGENC_CIDR2_SPEC>;
///STGENC component ID2 register
pub mod stgenc_cidr2;
///STGENC_CIDR3 (r) register accessor: an alias for `Reg<STGENC_CIDR3_SPEC>`
pub type STGENC_CIDR3 = crate::Reg<stgenc_cidr3::STGENC_CIDR3_SPEC>;
///STGENC component ID3 register
pub mod stgenc_cidr3;
