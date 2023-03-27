///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.
    pub stgenr_cntcvl: STGENR_CNTCVL,
    ///0x04 - the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.
    pub stgenr_cntcvu: STGENR_CNTCVU,
    _reserved2: [u8; 0x0fc8],
    ///0xfd0 - STGENR peripheral ID4 register
    pub stgenr_pidr4: STGENR_PIDR4,
    ///0xfd4 - STGENR peripheral ID5 register
    pub stgenr_pidr5: STGENR_PIDR5,
    ///0xfd8 - STGENR peripheral ID6 register
    pub stgenr_pidr6: STGENR_PIDR6,
    ///0xfdc - STGENR peripheral ID7 register
    pub stgenr_pidr7: STGENR_PIDR7,
    ///0xfe0 - STGENR peripheral ID0 register
    pub stgenr_pidr0: STGENR_PIDR0,
    ///0xfe4 - STGENR peripheral ID1 register
    pub stgenr_pidr1: STGENR_PIDR1,
    ///0xfe8 - STGENR peripheral ID2 register
    pub stgenr_pidr2: STGENR_PIDR2,
    ///0xfec - STGENR peripheral ID3 register
    pub stgenr_pidr3: STGENR_PIDR3,
    ///0xff0 - STGENR component ID0 register
    pub stgenr_cidr0: STGENR_CIDR0,
    ///0xff4 - STGENR component ID1 register
    pub stgenr_cidr1: STGENR_CIDR1,
    ///0xff8 - STGENR component ID2 register
    pub stgenr_cidr2: STGENR_CIDR2,
    ///0xffc - STGENR component ID3 register
    pub stgenr_cidr3: STGENR_CIDR3,
}
///STGENR_CNTCVL (r) register accessor: an alias for `Reg<STGENR_CNTCVL_SPEC>`
pub type STGENR_CNTCVL = crate::Reg<stgenr_cntcvl::STGENR_CNTCVL_SPEC>;
///the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.
pub mod stgenr_cntcvl;
///STGENR_CNTCVU (r) register accessor: an alias for `Reg<STGENR_CNTCVU_SPEC>`
pub type STGENR_CNTCVU = crate::Reg<stgenr_cntcvu::STGENR_CNTCVU_SPEC>;
///the control interface must clear the STGEN_CNTCR.EN bit before writing to this register.
pub mod stgenr_cntcvu;
///STGENR_PIDR4 (r) register accessor: an alias for `Reg<STGENR_PIDR4_SPEC>`
pub type STGENR_PIDR4 = crate::Reg<stgenr_pidr4::STGENR_PIDR4_SPEC>;
///STGENR peripheral ID4 register
pub mod stgenr_pidr4;
///STGENR_PIDR5 (r) register accessor: an alias for `Reg<STGENR_PIDR5_SPEC>`
pub type STGENR_PIDR5 = crate::Reg<stgenr_pidr5::STGENR_PIDR5_SPEC>;
///STGENR peripheral ID5 register
pub mod stgenr_pidr5;
///STGENR_PIDR6 (r) register accessor: an alias for `Reg<STGENR_PIDR6_SPEC>`
pub type STGENR_PIDR6 = crate::Reg<stgenr_pidr6::STGENR_PIDR6_SPEC>;
///STGENR peripheral ID6 register
pub mod stgenr_pidr6;
///STGENR_PIDR7 (r) register accessor: an alias for `Reg<STGENR_PIDR7_SPEC>`
pub type STGENR_PIDR7 = crate::Reg<stgenr_pidr7::STGENR_PIDR7_SPEC>;
///STGENR peripheral ID7 register
pub mod stgenr_pidr7;
///STGENR_PIDR0 (r) register accessor: an alias for `Reg<STGENR_PIDR0_SPEC>`
pub type STGENR_PIDR0 = crate::Reg<stgenr_pidr0::STGENR_PIDR0_SPEC>;
///STGENR peripheral ID0 register
pub mod stgenr_pidr0;
///STGENR_PIDR1 (r) register accessor: an alias for `Reg<STGENR_PIDR1_SPEC>`
pub type STGENR_PIDR1 = crate::Reg<stgenr_pidr1::STGENR_PIDR1_SPEC>;
///STGENR peripheral ID1 register
pub mod stgenr_pidr1;
///STGENR_PIDR2 (r) register accessor: an alias for `Reg<STGENR_PIDR2_SPEC>`
pub type STGENR_PIDR2 = crate::Reg<stgenr_pidr2::STGENR_PIDR2_SPEC>;
///STGENR peripheral ID2 register
pub mod stgenr_pidr2;
///STGENR_PIDR3 (r) register accessor: an alias for `Reg<STGENR_PIDR3_SPEC>`
pub type STGENR_PIDR3 = crate::Reg<stgenr_pidr3::STGENR_PIDR3_SPEC>;
///STGENR peripheral ID3 register
pub mod stgenr_pidr3;
///STGENR_CIDR0 (r) register accessor: an alias for `Reg<STGENR_CIDR0_SPEC>`
pub type STGENR_CIDR0 = crate::Reg<stgenr_cidr0::STGENR_CIDR0_SPEC>;
///STGENR component ID0 register
pub mod stgenr_cidr0;
///STGENR_CIDR1 (r) register accessor: an alias for `Reg<STGENR_CIDR1_SPEC>`
pub type STGENR_CIDR1 = crate::Reg<stgenr_cidr1::STGENR_CIDR1_SPEC>;
///STGENR component ID1 register
pub mod stgenr_cidr1;
///STGENR_CIDR2 (r) register accessor: an alias for `Reg<STGENR_CIDR2_SPEC>`
pub type STGENR_CIDR2 = crate::Reg<stgenr_cidr2::STGENR_CIDR2_SPEC>;
///STGENR component ID2 register
pub mod stgenr_cidr2;
///STGENR_CIDR3 (r) register accessor: an alias for `Reg<STGENR_CIDR3_SPEC>`
pub type STGENR_CIDR3 = crate::Reg<stgenr_cidr3::STGENR_CIDR3_SPEC>;
///STGENR component ID3 register
pub mod stgenr_cidr3;
