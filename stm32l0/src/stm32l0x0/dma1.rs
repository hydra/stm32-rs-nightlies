///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - interrupt status register
    pub isr: ISR,
    ///0x04 - interrupt flag clear register
    pub ifcr: IFCR,
    ///0x08..0x94 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    pub ch: [CH; 7],
    _reserved3: [u8; 0x14],
    ///0xa8 - channel selection register
    pub cselr: CSELR,
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
    ///0x6c..0x80 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    #[inline(always)]
    pub fn ch6(&self) -> &CH {
        &self.ch[5]
    }
    ///0x80..0x94 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
    #[inline(always)]
    pub fn ch7(&self) -> &CH {
        &self.ch[6]
    }
}
///ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///interrupt status register
pub mod isr;
///IFCR (w) register accessor: an alias for `Reg<IFCR_SPEC>`
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
///interrupt flag clear register
pub mod ifcr;
///Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
pub use self::ch::CH;
///Cluster
///Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers
pub mod ch;
///CSELR (rw) register accessor: an alias for `Reg<CSELR_SPEC>`
pub type CSELR = crate::Reg<cselr::CSELR_SPEC>;
///channel selection register
pub mod cselr;
