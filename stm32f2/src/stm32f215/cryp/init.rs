///Register block
#[repr(C)]
pub struct INIT {
    ///0x00 - initialization vector registers
    pub ivlr: IVLR,
    ///0x04 - initialization vector registers
    pub ivrr: IVRR,
}
///IVLR (rw) register accessor: an alias for `Reg<IVLR_SPEC>`
pub type IVLR = crate::Reg<ivlr::IVLR_SPEC>;
///initialization vector registers
pub mod ivlr;
///IVRR (rw) register accessor: an alias for `Reg<IVRR_SPEC>`
pub type IVRR = crate::Reg<ivrr::IVRR_SPEC>;
///initialization vector registers
pub mod ivrr;
