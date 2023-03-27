///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DMA interrupt status register
    pub isr: ISR,
    ///0x04 - DMA interrupt flag clear register
    pub ifcr: IFCR,
    ///0x08 - DMA channel 1 configuration register
    pub ccr1: CCR1,
    ///0x0c - DMA channel 1 number of data to transfer register
    pub cndtr1: CNDTR1,
    ///0x10 - DMA channel 1 peripheral address register
    pub cpar1: CPAR1,
    ///0x14 - DMA channel 1 memory address register
    pub cmar1: CMAR1,
    _reserved6: [u8; 0x04],
    ///0x1c - DMA channel 2 configuration register
    pub ccr2: CCR2,
    ///0x20 - DMA channel 2 number of data to transfer register
    pub cndtr2: CNDTR2,
    ///0x24 - DMA channel 2 peripheral address register
    pub cpar2: CPAR2,
    ///0x28 - DMA channel 2 memory address register
    pub cmar2: CMAR2,
    _reserved10: [u8; 0x04],
    ///0x30 - DMA channel 3 configuration register
    pub ccr3: CCR3,
    ///0x34 - DMA channel 3 number of data to transfer register
    pub cndtr3: CNDTR3,
    ///0x38 - DMA channel 3 peripheral address register
    pub cpar3: CPAR3,
    ///0x3c - DMA channel 3 memory address register
    pub cmar3: CMAR3,
}
///ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///DMA interrupt status register
pub mod isr;
///IFCR (w) register accessor: an alias for `Reg<IFCR_SPEC>`
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
///DMA interrupt flag clear register
pub mod ifcr;
///CCR1 (rw) register accessor: an alias for `Reg<CCR1_SPEC>`
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
///DMA channel 1 configuration register
pub mod ccr1;
///CNDTR1 (rw) register accessor: an alias for `Reg<CNDTR1_SPEC>`
pub type CNDTR1 = crate::Reg<cndtr1::CNDTR1_SPEC>;
///DMA channel 1 number of data to transfer register
pub mod cndtr1;
///CPAR1 (rw) register accessor: an alias for `Reg<CPAR1_SPEC>`
pub type CPAR1 = crate::Reg<cpar1::CPAR1_SPEC>;
///DMA channel 1 peripheral address register
pub mod cpar1;
///CMAR1 (rw) register accessor: an alias for `Reg<CMAR1_SPEC>`
pub type CMAR1 = crate::Reg<cmar1::CMAR1_SPEC>;
///DMA channel 1 memory address register
pub mod cmar1;
///CCR2 (rw) register accessor: an alias for `Reg<CCR2_SPEC>`
pub type CCR2 = crate::Reg<ccr2::CCR2_SPEC>;
///DMA channel 2 configuration register
pub mod ccr2;
///CNDTR2 (rw) register accessor: an alias for `Reg<CNDTR2_SPEC>`
pub type CNDTR2 = crate::Reg<cndtr2::CNDTR2_SPEC>;
///DMA channel 2 number of data to transfer register
pub mod cndtr2;
///CPAR2 (rw) register accessor: an alias for `Reg<CPAR2_SPEC>`
pub type CPAR2 = crate::Reg<cpar2::CPAR2_SPEC>;
///DMA channel 2 peripheral address register
pub mod cpar2;
///CMAR2 (rw) register accessor: an alias for `Reg<CMAR2_SPEC>`
pub type CMAR2 = crate::Reg<cmar2::CMAR2_SPEC>;
///DMA channel 2 memory address register
pub mod cmar2;
///CCR3 (rw) register accessor: an alias for `Reg<CCR3_SPEC>`
pub type CCR3 = crate::Reg<ccr3::CCR3_SPEC>;
///DMA channel 3 configuration register
pub mod ccr3;
///CNDTR3 (rw) register accessor: an alias for `Reg<CNDTR3_SPEC>`
pub type CNDTR3 = crate::Reg<cndtr3::CNDTR3_SPEC>;
///DMA channel 3 number of data to transfer register
pub mod cndtr3;
///CPAR3 (rw) register accessor: an alias for `Reg<CPAR3_SPEC>`
pub type CPAR3 = crate::Reg<cpar3::CPAR3_SPEC>;
///DMA channel 3 peripheral address register
pub mod cpar3;
///CMAR3 (rw) register accessor: an alias for `Reg<CMAR3_SPEC>`
pub type CMAR3 = crate::Reg<cmar3::CMAR3_SPEC>;
///DMA channel 3 memory address register
pub mod cmar3;
