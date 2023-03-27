///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    ///0x08 - Synchronization Size Configuration Register
    pub sscr: SSCR,
    ///0x0c - Back Porch Configuration Register
    pub bpcr: BPCR,
    ///0x10 - Active Width Configuration Register
    pub awcr: AWCR,
    ///0x14 - Total Width Configuration Register
    pub twcr: TWCR,
    ///0x18 - Global Control Register
    pub gcr: GCR,
    _reserved5: [u8; 0x08],
    ///0x24 - Shadow Reload Configuration Register
    pub srcr: SRCR,
    _reserved6: [u8; 0x04],
    ///0x2c - Background Color Configuration Register
    pub bccr: BCCR,
    _reserved7: [u8; 0x04],
    ///0x34 - Interrupt Enable Register
    pub ier: IER,
    ///0x38 - Interrupt Status Register
    pub isr: ISR,
    ///0x3c - Interrupt Clear Register
    pub icr: ICR,
    ///0x40 - Line Interrupt Position Configuration Register
    pub lipcr: LIPCR,
    ///0x44 - Current Position Status Register
    pub cpsr: CPSR,
    ///0x48 - Current Display Status Register
    pub cdsr: CDSR,
    _reserved13: [u8; 0x38],
    ///0x84..0x184 - Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR
    pub layer: [LAYER; 2],
}
impl RegisterBlock {
    ///0x84..0x104 - Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR
    #[inline(always)]
    pub fn layer1(&self) -> &LAYER {
        &self.layer[0]
    }
    ///0x104..0x184 - Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR
    #[inline(always)]
    pub fn layer2(&self) -> &LAYER {
        &self.layer[1]
    }
}
///SSCR (rw) register accessor: an alias for `Reg<SSCR_SPEC>`
pub type SSCR = crate::Reg<sscr::SSCR_SPEC>;
///Synchronization Size Configuration Register
pub mod sscr;
///BPCR (rw) register accessor: an alias for `Reg<BPCR_SPEC>`
pub type BPCR = crate::Reg<bpcr::BPCR_SPEC>;
///Back Porch Configuration Register
pub mod bpcr;
///AWCR (rw) register accessor: an alias for `Reg<AWCR_SPEC>`
pub type AWCR = crate::Reg<awcr::AWCR_SPEC>;
///Active Width Configuration Register
pub mod awcr;
///TWCR (rw) register accessor: an alias for `Reg<TWCR_SPEC>`
pub type TWCR = crate::Reg<twcr::TWCR_SPEC>;
///Total Width Configuration Register
pub mod twcr;
///GCR (rw) register accessor: an alias for `Reg<GCR_SPEC>`
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
///Global Control Register
pub mod gcr;
///SRCR (rw) register accessor: an alias for `Reg<SRCR_SPEC>`
pub type SRCR = crate::Reg<srcr::SRCR_SPEC>;
///Shadow Reload Configuration Register
pub mod srcr;
///BCCR (rw) register accessor: an alias for `Reg<BCCR_SPEC>`
pub type BCCR = crate::Reg<bccr::BCCR_SPEC>;
///Background Color Configuration Register
pub mod bccr;
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///Interrupt Enable Register
pub mod ier;
///ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///Interrupt Status Register
pub mod isr;
///ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///Interrupt Clear Register
pub mod icr;
///LIPCR (rw) register accessor: an alias for `Reg<LIPCR_SPEC>`
pub type LIPCR = crate::Reg<lipcr::LIPCR_SPEC>;
///Line Interrupt Position Configuration Register
pub mod lipcr;
///CPSR (r) register accessor: an alias for `Reg<CPSR_SPEC>`
pub type CPSR = crate::Reg<cpsr::CPSR_SPEC>;
///Current Position Status Register
pub mod cpsr;
///CDSR (r) register accessor: an alias for `Reg<CDSR_SPEC>`
pub type CDSR = crate::Reg<cdsr::CDSR_SPEC>;
///Current Display Status Register
pub mod cdsr;
///Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR
pub use self::layer::LAYER;
///Cluster
///Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR
pub mod layer;
