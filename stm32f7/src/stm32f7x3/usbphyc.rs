///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - USBPHYC PLL1 control register
    pub pll1: PLL1,
    _reserved1: [u8; 0x08],
    ///0x0c - USBPHYC tuning control register
    pub tune: TUNE,
    _reserved2: [u8; 0x08],
    ///0x18 - USBPHYC LDO control and status register
    pub ldo: LDO,
}
///PLL1 (rw) register accessor: an alias for `Reg<PLL1_SPEC>`
pub type PLL1 = crate::Reg<pll1::PLL1_SPEC>;
///USBPHYC PLL1 control register
pub mod pll1;
///TUNE (rw) register accessor: an alias for `Reg<TUNE_SPEC>`
pub type TUNE = crate::Reg<tune::TUNE_SPEC>;
///USBPHYC tuning control register
pub mod tune;
///LDO (rw) register accessor: an alias for `Reg<LDO_SPEC>`
pub type LDO = crate::Reg<ldo::LDO_SPEC>;
///USBPHYC LDO control and status register
pub mod ldo;
