///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - ADF Global Control Register
    pub adf_gcr: ADF_GCR,
    ///0x04 - ADF clock generator control register
    pub adf_ckgcr: ADF_CKGCR,
    _reserved2: [u8; 0x78],
    ///0x80 - ADF serial interface control register 0
    pub adf_sitf0cr: ADF_SITF0CR,
    ///0x84 - ADF bitstream matrix control register 0
    pub adf_bsmx0cr: ADF_BSMX0CR,
    ///0x88 - ADF digital filter control register 0
    pub adf_dflt0cr: ADF_DFLT0CR,
    ///0x8c - ADF digital filer configuration register 0
    pub adf_dflt0cicr: ADF_DFLT0CICR,
    ///0x90 - ADF reshape filter configuration register 0
    pub adf_dflt0rsfr: ADF_DFLT0RSFR,
    _reserved7: [u8; 0x10],
    ///0xa4 - ADF delay control register 0
    pub adf_dly0cr: ADF_DLY0CR,
    _reserved8: [u8; 0x04],
    ///0xac - ADF DFLT0 interrupt enable register
    pub adf_dflt0ier: ADF_DFLT0IER,
    ///0xb0 - ADF DFLT0 interrupt status register 0
    pub adf_dflt0isr: ADF_DFLT0ISR,
    _reserved10: [u8; 0x04],
    ///0xb8 - ADF SAD control register
    pub adf_sadcr: ADF_SADCR,
    ///0xbc - ADF SAD configuration register
    pub adf_sadcfgr: ADF_SADCFGR,
    ///0xc0 - ADF SAD sound level register
    pub adf_sadsdlvr: ADF_SADSDLVR,
    ///0xc4 - ADF SAD ambient noise level register
    pub adf_sadanlvr: ADF_SADANLVR,
    _reserved14: [u8; 0x28],
    ///0xf0 - ADF digital filter data register 0
    pub adf_dflt0dr: ADF_DFLT0DR,
}
///ADF_GCR (rw) register accessor: an alias for `Reg<ADF_GCR_SPEC>`
pub type ADF_GCR = crate::Reg<adf_gcr::ADF_GCR_SPEC>;
///ADF Global Control Register
pub mod adf_gcr;
///ADF_CKGCR (rw) register accessor: an alias for `Reg<ADF_CKGCR_SPEC>`
pub type ADF_CKGCR = crate::Reg<adf_ckgcr::ADF_CKGCR_SPEC>;
///ADF clock generator control register
pub mod adf_ckgcr;
///ADF_SITF0CR (rw) register accessor: an alias for `Reg<ADF_SITF0CR_SPEC>`
pub type ADF_SITF0CR = crate::Reg<adf_sitf0cr::ADF_SITF0CR_SPEC>;
///ADF serial interface control register 0
pub mod adf_sitf0cr;
///ADF_BSMX0CR (rw) register accessor: an alias for `Reg<ADF_BSMX0CR_SPEC>`
pub type ADF_BSMX0CR = crate::Reg<adf_bsmx0cr::ADF_BSMX0CR_SPEC>;
///ADF bitstream matrix control register 0
pub mod adf_bsmx0cr;
///ADF_DFLT0CR (rw) register accessor: an alias for `Reg<ADF_DFLT0CR_SPEC>`
pub type ADF_DFLT0CR = crate::Reg<adf_dflt0cr::ADF_DFLT0CR_SPEC>;
///ADF digital filter control register 0
pub mod adf_dflt0cr;
///ADF_DFLT0CICR (rw) register accessor: an alias for `Reg<ADF_DFLT0CICR_SPEC>`
pub type ADF_DFLT0CICR = crate::Reg<adf_dflt0cicr::ADF_DFLT0CICR_SPEC>;
///ADF digital filer configuration register 0
pub mod adf_dflt0cicr;
///ADF_DFLT0RSFR (rw) register accessor: an alias for `Reg<ADF_DFLT0RSFR_SPEC>`
pub type ADF_DFLT0RSFR = crate::Reg<adf_dflt0rsfr::ADF_DFLT0RSFR_SPEC>;
///ADF reshape filter configuration register 0
pub mod adf_dflt0rsfr;
///ADF_DLY0CR (rw) register accessor: an alias for `Reg<ADF_DLY0CR_SPEC>`
pub type ADF_DLY0CR = crate::Reg<adf_dly0cr::ADF_DLY0CR_SPEC>;
///ADF delay control register 0
pub mod adf_dly0cr;
///ADF_DFLT0IER (rw) register accessor: an alias for `Reg<ADF_DFLT0IER_SPEC>`
pub type ADF_DFLT0IER = crate::Reg<adf_dflt0ier::ADF_DFLT0IER_SPEC>;
///ADF DFLT0 interrupt enable register
pub mod adf_dflt0ier;
///ADF_DFLT0ISR (rw) register accessor: an alias for `Reg<ADF_DFLT0ISR_SPEC>`
pub type ADF_DFLT0ISR = crate::Reg<adf_dflt0isr::ADF_DFLT0ISR_SPEC>;
///ADF DFLT0 interrupt status register 0
pub mod adf_dflt0isr;
///ADF_SADCR (rw) register accessor: an alias for `Reg<ADF_SADCR_SPEC>`
pub type ADF_SADCR = crate::Reg<adf_sadcr::ADF_SADCR_SPEC>;
///ADF SAD control register
pub mod adf_sadcr;
///ADF_SADCFGR (rw) register accessor: an alias for `Reg<ADF_SADCFGR_SPEC>`
pub type ADF_SADCFGR = crate::Reg<adf_sadcfgr::ADF_SADCFGR_SPEC>;
///ADF SAD configuration register
pub mod adf_sadcfgr;
///ADF_SADSDLVR (r) register accessor: an alias for `Reg<ADF_SADSDLVR_SPEC>`
pub type ADF_SADSDLVR = crate::Reg<adf_sadsdlvr::ADF_SADSDLVR_SPEC>;
///ADF SAD sound level register
pub mod adf_sadsdlvr;
///ADF_SADANLVR (r) register accessor: an alias for `Reg<ADF_SADANLVR_SPEC>`
pub type ADF_SADANLVR = crate::Reg<adf_sadanlvr::ADF_SADANLVR_SPEC>;
///ADF SAD ambient noise level register
pub mod adf_sadanlvr;
///ADF_DFLT0DR (r) register accessor: an alias for `Reg<ADF_DFLT0DR_SPEC>`
pub type ADF_DFLT0DR = crate::Reg<adf_dflt0dr::ADF_DFLT0DR_SPEC>;
///ADF digital filter data register 0
pub mod adf_dflt0dr;
