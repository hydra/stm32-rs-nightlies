///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - low interrupt status register
    pub isr: ISR,
    ///0x04 - DMA interrupt flag clear register
    pub ifcr: IFCR,
    ///0x08..0x6c - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    pub ch: [CH; 5],
}
impl RegisterBlock {
    ///0x08..0x1c - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    #[inline(always)]
    pub fn ch1(&self) -> &CH {
        &self.ch[0]
    }
    ///0x1c..0x30 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    #[inline(always)]
    pub fn ch2(&self) -> &CH {
        &self.ch[1]
    }
    ///0x30..0x44 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    #[inline(always)]
    pub fn ch3(&self) -> &CH {
        &self.ch[2]
    }
    ///0x44..0x58 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    #[inline(always)]
    pub fn ch4(&self) -> &CH {
        &self.ch[3]
    }
    ///0x58..0x6c - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    #[inline(always)]
    pub fn ch5(&self) -> &CH {
        &self.ch[4]
    }
}
///ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///low interrupt status register
pub mod isr;
///IFCR (w) register accessor: an alias for `Reg<IFCR_SPEC>`
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
///DMA interrupt flag clear register
pub mod ifcr;
///Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
pub use self::ch::CH;
///Cluster
///Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
pub mod ch;
