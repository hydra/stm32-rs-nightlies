///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - MDMA Global Interrupt/Status Register
    pub gisr0: GISR0,
    _reserved1: [u8; 0x3c],
    ///0x40..0x440 - Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers
    pub ch: [CH; 16],
}
///GISR0 (r) register accessor: an alias for `Reg<GISR0_SPEC>`
pub type GISR0 = crate::Reg<gisr0::GISR0_SPEC>;
///MDMA Global Interrupt/Status Register
pub mod gisr0;
///Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers
pub use self::ch::CH;
///Cluster
///Channel cluster: C?ISR, C?IFCR, C?ESR, C?CR, C?TCR, C?BNDTR, C?SAR, C?DAR, C?BRUR, C?LAR, C?TBR, C?MAR and C?MDR registers
pub mod ch;
