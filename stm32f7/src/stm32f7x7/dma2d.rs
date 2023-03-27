///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: CR,
    ///0x04 - Interrupt Status Register
    pub isr: ISR,
    ///0x08 - interrupt flag clear register
    pub ifcr: IFCR,
    ///0x0c - foreground memory address register
    pub fgmar: FGMAR,
    ///0x10 - foreground offset register
    pub fgor: FGOR,
    ///0x14 - background memory address register
    pub bgmar: BGMAR,
    ///0x18 - background offset register
    pub bgor: BGOR,
    ///0x1c - foreground PFC control register
    pub fgpfccr: FGPFCCR,
    ///0x20 - foreground color register
    pub fgcolr: FGCOLR,
    ///0x24 - background PFC control register
    pub bgpfccr: BGPFCCR,
    ///0x28 - background color register
    pub bgcolr: BGCOLR,
    ///0x2c - foreground CLUT memory address register
    pub fgcmar: FGCMAR,
    ///0x30 - background CLUT memory address register
    pub bgcmar: BGCMAR,
    ///0x34 - output PFC control register
    pub opfccr: OPFCCR,
    ///0x38 - output color register
    pub ocolr: OCOLR,
    ///0x3c - output memory address register
    pub omar: OMAR,
    ///0x40 - output offset register
    pub oor: OOR,
    ///0x44 - number of line register
    pub nlr: NLR,
    ///0x48 - line watermark register
    pub lwr: LWR,
    ///0x4c - AHB master timer configuration register
    pub amtcr: AMTCR,
    _reserved20: [u8; 0x03b0],
    ///0x400 - FGCLUT
    pub fgclut: FGCLUT,
    _reserved21: [u8; 0x03fc],
    ///0x800 - BGCLUT
    pub bgclut: BGCLUT,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///control register
pub mod cr;
///ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///Interrupt Status Register
pub mod isr;
///IFCR (rw) register accessor: an alias for `Reg<IFCR_SPEC>`
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
///interrupt flag clear register
pub mod ifcr;
///FGMAR (rw) register accessor: an alias for `Reg<FGMAR_SPEC>`
pub type FGMAR = crate::Reg<fgmar::FGMAR_SPEC>;
///foreground memory address register
pub mod fgmar;
///FGOR (rw) register accessor: an alias for `Reg<FGOR_SPEC>`
pub type FGOR = crate::Reg<fgor::FGOR_SPEC>;
///foreground offset register
pub mod fgor;
///BGMAR (rw) register accessor: an alias for `Reg<BGMAR_SPEC>`
pub type BGMAR = crate::Reg<bgmar::BGMAR_SPEC>;
///background memory address register
pub mod bgmar;
///BGOR (rw) register accessor: an alias for `Reg<BGOR_SPEC>`
pub type BGOR = crate::Reg<bgor::BGOR_SPEC>;
///background offset register
pub mod bgor;
///FGPFCCR (rw) register accessor: an alias for `Reg<FGPFCCR_SPEC>`
pub type FGPFCCR = crate::Reg<fgpfccr::FGPFCCR_SPEC>;
///foreground PFC control register
pub mod fgpfccr;
///FGCOLR (rw) register accessor: an alias for `Reg<FGCOLR_SPEC>`
pub type FGCOLR = crate::Reg<fgcolr::FGCOLR_SPEC>;
///foreground color register
pub mod fgcolr;
///BGPFCCR (rw) register accessor: an alias for `Reg<BGPFCCR_SPEC>`
pub type BGPFCCR = crate::Reg<bgpfccr::BGPFCCR_SPEC>;
///background PFC control register
pub mod bgpfccr;
///BGCOLR (rw) register accessor: an alias for `Reg<BGCOLR_SPEC>`
pub type BGCOLR = crate::Reg<bgcolr::BGCOLR_SPEC>;
///background color register
pub mod bgcolr;
///FGCMAR (rw) register accessor: an alias for `Reg<FGCMAR_SPEC>`
pub type FGCMAR = crate::Reg<fgcmar::FGCMAR_SPEC>;
///foreground CLUT memory address register
pub mod fgcmar;
///BGCMAR (rw) register accessor: an alias for `Reg<BGCMAR_SPEC>`
pub type BGCMAR = crate::Reg<bgcmar::BGCMAR_SPEC>;
///background CLUT memory address register
pub mod bgcmar;
///OPFCCR (rw) register accessor: an alias for `Reg<OPFCCR_SPEC>`
pub type OPFCCR = crate::Reg<opfccr::OPFCCR_SPEC>;
///output PFC control register
pub mod opfccr;
///OCOLR (rw) register accessor: an alias for `Reg<OCOLR_SPEC>`
pub type OCOLR = crate::Reg<ocolr::OCOLR_SPEC>;
///output color register
pub mod ocolr;
///OMAR (rw) register accessor: an alias for `Reg<OMAR_SPEC>`
pub type OMAR = crate::Reg<omar::OMAR_SPEC>;
///output memory address register
pub mod omar;
///OOR (rw) register accessor: an alias for `Reg<OOR_SPEC>`
pub type OOR = crate::Reg<oor::OOR_SPEC>;
///output offset register
pub mod oor;
///NLR (rw) register accessor: an alias for `Reg<NLR_SPEC>`
pub type NLR = crate::Reg<nlr::NLR_SPEC>;
///number of line register
pub mod nlr;
///LWR (rw) register accessor: an alias for `Reg<LWR_SPEC>`
pub type LWR = crate::Reg<lwr::LWR_SPEC>;
///line watermark register
pub mod lwr;
///AMTCR (rw) register accessor: an alias for `Reg<AMTCR_SPEC>`
pub type AMTCR = crate::Reg<amtcr::AMTCR_SPEC>;
///AHB master timer configuration register
pub mod amtcr;
///FGCLUT (rw) register accessor: an alias for `Reg<FGCLUT_SPEC>`
pub type FGCLUT = crate::Reg<fgclut::FGCLUT_SPEC>;
///FGCLUT
pub mod fgclut;
///BGCLUT (rw) register accessor: an alias for `Reg<BGCLUT_SPEC>`
pub type BGCLUT = crate::Reg<bgclut::BGCLUT_SPEC>;
///BGCLUT
pub mod bgclut;
