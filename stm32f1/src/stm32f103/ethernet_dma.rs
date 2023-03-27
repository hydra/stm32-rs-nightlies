///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Ethernet DMA bus mode register
    pub dmabmr: DMABMR,
    ///0x04 - Ethernet DMA transmit poll demand register
    pub dmatpdr: DMATPDR,
    ///0x08 - EHERNET DMA receive poll demand register
    pub dmarpdr: DMARPDR,
    ///0x0c - Ethernet DMA receive descriptor list address register
    pub dmardlar: DMARDLAR,
    ///0x10 - Ethernet DMA transmit descriptor list address register
    pub dmatdlar: DMATDLAR,
    ///0x14 - Ethernet DMA status register
    pub dmasr: DMASR,
    ///0x18 - Ethernet DMA operation mode register
    pub dmaomr: DMAOMR,
    ///0x1c - Ethernet DMA interrupt enable register
    pub dmaier: DMAIER,
    ///0x20 - Ethernet DMA missed frame and buffer overflow counter register
    pub dmamfbocr: DMAMFBOCR,
    _reserved9: [u8; 0x24],
    ///0x48 - Ethernet DMA current host transmit descriptor register
    pub dmachtdr: DMACHTDR,
    ///0x4c - Ethernet DMA current host receive descriptor register
    pub dmachrdr: DMACHRDR,
    ///0x50 - Ethernet DMA current host transmit buffer address register
    pub dmachtbar: DMACHTBAR,
    ///0x54 - Ethernet DMA current host receive buffer address register
    pub dmachrbar: DMACHRBAR,
}
///DMABMR (rw) register accessor: an alias for `Reg<DMABMR_SPEC>`
pub type DMABMR = crate::Reg<dmabmr::DMABMR_SPEC>;
///Ethernet DMA bus mode register
pub mod dmabmr;
///DMATPDR (rw) register accessor: an alias for `Reg<DMATPDR_SPEC>`
pub type DMATPDR = crate::Reg<dmatpdr::DMATPDR_SPEC>;
///Ethernet DMA transmit poll demand register
pub mod dmatpdr;
///DMARPDR (rw) register accessor: an alias for `Reg<DMARPDR_SPEC>`
pub type DMARPDR = crate::Reg<dmarpdr::DMARPDR_SPEC>;
///EHERNET DMA receive poll demand register
pub mod dmarpdr;
///DMARDLAR (rw) register accessor: an alias for `Reg<DMARDLAR_SPEC>`
pub type DMARDLAR = crate::Reg<dmardlar::DMARDLAR_SPEC>;
///Ethernet DMA receive descriptor list address register
pub mod dmardlar;
///DMATDLAR (rw) register accessor: an alias for `Reg<DMATDLAR_SPEC>`
pub type DMATDLAR = crate::Reg<dmatdlar::DMATDLAR_SPEC>;
///Ethernet DMA transmit descriptor list address register
pub mod dmatdlar;
///DMASR (rw) register accessor: an alias for `Reg<DMASR_SPEC>`
pub type DMASR = crate::Reg<dmasr::DMASR_SPEC>;
///Ethernet DMA status register
pub mod dmasr;
///DMAOMR (rw) register accessor: an alias for `Reg<DMAOMR_SPEC>`
pub type DMAOMR = crate::Reg<dmaomr::DMAOMR_SPEC>;
///Ethernet DMA operation mode register
pub mod dmaomr;
///DMAIER (rw) register accessor: an alias for `Reg<DMAIER_SPEC>`
pub type DMAIER = crate::Reg<dmaier::DMAIER_SPEC>;
///Ethernet DMA interrupt enable register
pub mod dmaier;
///DMAMFBOCR (r) register accessor: an alias for `Reg<DMAMFBOCR_SPEC>`
pub type DMAMFBOCR = crate::Reg<dmamfbocr::DMAMFBOCR_SPEC>;
///Ethernet DMA missed frame and buffer overflow counter register
pub mod dmamfbocr;
///DMACHTDR (r) register accessor: an alias for `Reg<DMACHTDR_SPEC>`
pub type DMACHTDR = crate::Reg<dmachtdr::DMACHTDR_SPEC>;
///Ethernet DMA current host transmit descriptor register
pub mod dmachtdr;
///DMACHRDR (r) register accessor: an alias for `Reg<DMACHRDR_SPEC>`
pub type DMACHRDR = crate::Reg<dmachrdr::DMACHRDR_SPEC>;
///Ethernet DMA current host receive descriptor register
pub mod dmachrdr;
///DMACHTBAR (r) register accessor: an alias for `Reg<DMACHTBAR_SPEC>`
pub type DMACHTBAR = crate::Reg<dmachtbar::DMACHTBAR_SPEC>;
///Ethernet DMA current host transmit buffer address register
pub mod dmachtbar;
///DMACHRBAR (r) register accessor: an alias for `Reg<DMACHRBAR_SPEC>`
pub type DMACHRBAR = crate::Reg<dmachrbar::DMACHRBAR_SPEC>;
///Ethernet DMA current host receive buffer address register
pub mod dmachrbar;
