///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Interrupt status register
    pub isr: ISR,
    ///0x04 - Interrupt flag clear register
    pub ifcr: IFCR,
    ///0x08..0xa8 - Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR? and CM1AR registers?
    pub ch: [CH; 8],
}
///ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///Interrupt status register
pub mod isr;
///IFCR (w) register accessor: an alias for `Reg<IFCR_SPEC>`
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
///Interrupt flag clear register
pub mod ifcr;
///Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR? and CM1AR registers?
pub use self::ch::CH;
///Cluster
///Channel cluster: CCR?, CNDTR?, CPAR?, CM0AR? and CM1AR registers?
pub mod ch;
