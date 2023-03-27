///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DMA2D control register
    pub cr: CR,
    ///0x04 - DMA2D Interrupt Status Register
    pub isr: ISR,
    ///0x08 - DMA2D interrupt flag clear register
    pub ifcr: IFCR,
    ///0x0c - DMA2D foreground memory address register
    pub fgmar: FGMAR,
    ///0x10 - DMA2D foreground offset register
    pub fgor: FGOR,
    ///0x14 - DMA2D background memory address register
    pub bgmar: BGMAR,
    ///0x18 - DMA2D background offset register
    pub bgor: BGOR,
    ///0x1c - DMA2D foreground PFC control register
    pub fgpfccr: FGPFCCR,
    ///0x20 - DMA2D foreground color register
    pub fgcolr: FGCOLR,
    ///0x24 - DMA2D background PFC control register
    pub bgpfccr: BGPFCCR,
    ///0x28 - DMA2D background color register
    pub bgcolr: BGCOLR,
    ///0x2c - DMA2D foreground CLUT memory address register
    pub fgcmar: FGCMAR,
    ///0x30 - DMA2D background CLUT memory address register
    pub bgcmar: BGCMAR,
    ///0x34 - DMA2D output PFC control register
    pub opfccr: OPFCCR,
    ///0x38 - DMA2D output color register
    pub ocolr: OCOLR,
    ///0x3c - DMA2D output memory address register
    pub omar: OMAR,
    ///0x40 - DMA2D output offset register
    pub oor: OOR,
    ///0x44 - DMA2D number of line register
    pub nlr: NLR,
    ///0x48 - DMA2D line watermark register
    pub lwr: LWR,
    ///0x4c - DMA2D AXI master timer configuration register
    pub amtcr: AMTCR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///DMA2D control register
pub mod cr;
///ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///DMA2D Interrupt Status Register
pub mod isr;
///IFCR (rw) register accessor: an alias for `Reg<IFCR_SPEC>`
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
///DMA2D interrupt flag clear register
pub mod ifcr;
///FGMAR (rw) register accessor: an alias for `Reg<FGMAR_SPEC>`
pub type FGMAR = crate::Reg<fgmar::FGMAR_SPEC>;
///DMA2D foreground memory address register
pub mod fgmar;
///FGOR (rw) register accessor: an alias for `Reg<FGOR_SPEC>`
pub type FGOR = crate::Reg<fgor::FGOR_SPEC>;
///DMA2D foreground offset register
pub mod fgor;
///BGMAR (rw) register accessor: an alias for `Reg<BGMAR_SPEC>`
pub type BGMAR = crate::Reg<bgmar::BGMAR_SPEC>;
///DMA2D background memory address register
pub mod bgmar;
///BGOR (rw) register accessor: an alias for `Reg<BGOR_SPEC>`
pub type BGOR = crate::Reg<bgor::BGOR_SPEC>;
///DMA2D background offset register
pub mod bgor;
///FGPFCCR (rw) register accessor: an alias for `Reg<FGPFCCR_SPEC>`
pub type FGPFCCR = crate::Reg<fgpfccr::FGPFCCR_SPEC>;
///DMA2D foreground PFC control register
pub mod fgpfccr;
///FGCOLR (rw) register accessor: an alias for `Reg<FGCOLR_SPEC>`
pub type FGCOLR = crate::Reg<fgcolr::FGCOLR_SPEC>;
///DMA2D foreground color register
pub mod fgcolr;
///BGPFCCR (rw) register accessor: an alias for `Reg<BGPFCCR_SPEC>`
pub type BGPFCCR = crate::Reg<bgpfccr::BGPFCCR_SPEC>;
///DMA2D background PFC control register
pub mod bgpfccr;
///BGCOLR (rw) register accessor: an alias for `Reg<BGCOLR_SPEC>`
pub type BGCOLR = crate::Reg<bgcolr::BGCOLR_SPEC>;
///DMA2D background color register
pub mod bgcolr;
///FGCMAR (rw) register accessor: an alias for `Reg<FGCMAR_SPEC>`
pub type FGCMAR = crate::Reg<fgcmar::FGCMAR_SPEC>;
///DMA2D foreground CLUT memory address register
pub mod fgcmar;
///BGCMAR (rw) register accessor: an alias for `Reg<BGCMAR_SPEC>`
pub type BGCMAR = crate::Reg<bgcmar::BGCMAR_SPEC>;
///DMA2D background CLUT memory address register
pub mod bgcmar;
///OPFCCR (rw) register accessor: an alias for `Reg<OPFCCR_SPEC>`
pub type OPFCCR = crate::Reg<opfccr::OPFCCR_SPEC>;
///DMA2D output PFC control register
pub mod opfccr;
///OCOLR (rw) register accessor: an alias for `Reg<OCOLR_SPEC>`
pub type OCOLR = crate::Reg<ocolr::OCOLR_SPEC>;
///DMA2D output color register
pub mod ocolr;
///OMAR (rw) register accessor: an alias for `Reg<OMAR_SPEC>`
pub type OMAR = crate::Reg<omar::OMAR_SPEC>;
///DMA2D output memory address register
pub mod omar;
///OOR (rw) register accessor: an alias for `Reg<OOR_SPEC>`
pub type OOR = crate::Reg<oor::OOR_SPEC>;
///DMA2D output offset register
pub mod oor;
///NLR (rw) register accessor: an alias for `Reg<NLR_SPEC>`
pub type NLR = crate::Reg<nlr::NLR_SPEC>;
///DMA2D number of line register
pub mod nlr;
///LWR (rw) register accessor: an alias for `Reg<LWR_SPEC>`
pub type LWR = crate::Reg<lwr::LWR_SPEC>;
///DMA2D line watermark register
pub mod lwr;
///AMTCR (rw) register accessor: an alias for `Reg<AMTCR_SPEC>`
pub type AMTCR = crate::Reg<amtcr::AMTCR_SPEC>;
///DMA2D AXI master timer configuration register
pub mod amtcr;
