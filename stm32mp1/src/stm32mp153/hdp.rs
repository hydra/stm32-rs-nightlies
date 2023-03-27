///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - HDP Control
    pub hdp_ctrl: HDP_CTRL,
    ///0x04 - HDP multiplexing
    pub hdp_mux: HDP_MUX,
    _reserved2: [u8; 0x08],
    ///0x10 - HDP value
    pub hdp_val: HDP_VAL,
    ///0x14 - HDP GPO set
    pub hdp_gposet: HDP_GPOSET,
    ///0x18 - HDP GPO clear
    pub hdp_gpoclr: HDP_GPOCLR,
    ///0x1c - HDP GPO value
    pub hdp_gpoval: HDP_GPOVAL,
    _reserved6: [u8; 0x03d4],
    ///0x3f4 - HDP version register
    pub hdp_verr: HDP_VERR,
    ///0x3f8 - HDP IP identification register
    pub hdp_ipidr: HDP_IPIDR,
    ///0x3fc - HDP size identification register
    pub hdp_sidr: HDP_SIDR,
}
///HDP_CTRL (rw) register accessor: an alias for `Reg<HDP_CTRL_SPEC>`
pub type HDP_CTRL = crate::Reg<hdp_ctrl::HDP_CTRL_SPEC>;
///HDP Control
pub mod hdp_ctrl;
///HDP_MUX (rw) register accessor: an alias for `Reg<HDP_MUX_SPEC>`
pub type HDP_MUX = crate::Reg<hdp_mux::HDP_MUX_SPEC>;
///HDP multiplexing
pub mod hdp_mux;
///HDP_VAL (r) register accessor: an alias for `Reg<HDP_VAL_SPEC>`
pub type HDP_VAL = crate::Reg<hdp_val::HDP_VAL_SPEC>;
///HDP value
pub mod hdp_val;
///HDP_GPOSET (w) register accessor: an alias for `Reg<HDP_GPOSET_SPEC>`
pub type HDP_GPOSET = crate::Reg<hdp_gposet::HDP_GPOSET_SPEC>;
///HDP GPO set
pub mod hdp_gposet;
///HDP_GPOCLR (w) register accessor: an alias for `Reg<HDP_GPOCLR_SPEC>`
pub type HDP_GPOCLR = crate::Reg<hdp_gpoclr::HDP_GPOCLR_SPEC>;
///HDP GPO clear
pub mod hdp_gpoclr;
///HDP_GPOVAL (rw) register accessor: an alias for `Reg<HDP_GPOVAL_SPEC>`
pub type HDP_GPOVAL = crate::Reg<hdp_gpoval::HDP_GPOVAL_SPEC>;
///HDP GPO value
pub mod hdp_gpoval;
///HDP_VERR (r) register accessor: an alias for `Reg<HDP_VERR_SPEC>`
pub type HDP_VERR = crate::Reg<hdp_verr::HDP_VERR_SPEC>;
///HDP version register
pub mod hdp_verr;
///HDP_IPIDR (r) register accessor: an alias for `Reg<HDP_IPIDR_SPEC>`
pub type HDP_IPIDR = crate::Reg<hdp_ipidr::HDP_IPIDR_SPEC>;
///HDP IP identification register
pub mod hdp_ipidr;
///HDP_SIDR (r) register accessor: an alias for `Reg<HDP_SIDR_SPEC>`
pub type HDP_SIDR = crate::Reg<hdp_sidr::HDP_SIDR_SPEC>;
///HDP size identification register
pub mod hdp_sidr;
