///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - OCTOSPI control register
    pub cr: CR,
    _reserved1: [u8; 0x04],
    ///0x08 - OCTOSPI device configuration register 1
    pub dcr1: DCR1,
    ///0x0c - OCTOSPI device configuration register 2
    pub dcr2: DCR2,
    ///0x10 - OCTOSPI device configuration register 3
    pub dcr3: DCR3,
    ///0x14 - OCTOSPI device configuration register 4
    pub dcr4: DCR4,
    _reserved5: [u8; 0x08],
    ///0x20 - OCTOSPI status register
    pub sr: SR,
    ///0x24 - OCTOSPI flag clear register
    pub fcr: FCR,
    _reserved7: [u8; 0x18],
    ///0x40 - OCTOSPI data length register
    pub dlr: DLR,
    _reserved8: [u8; 0x04],
    ///0x48 - OCTOSPI address register
    pub ar: AR,
    _reserved9: [u8; 0x04],
    ///0x50 - OCTOSPI data register
    pub dr: DR,
    _reserved10: [u8; 0x2c],
    ///0x80 - OCTOSPI polling status mask register
    pub psmkr: PSMKR,
    _reserved11: [u8; 0x04],
    ///0x88 - OCTOSPI polling status match register
    pub psmar: PSMAR,
    _reserved12: [u8; 0x04],
    ///0x90 - OCTOSPI polling interval register
    pub pir: PIR,
    _reserved13: [u8; 0x6c],
    ///0x100 - OCTOSPI communication configuration register
    pub ccr: CCR,
    _reserved14: [u8; 0x04],
    ///0x108 - OCTOSPI timing configuration register
    pub tcr: TCR,
    _reserved15: [u8; 0x04],
    ///0x110 - OCTOSPI instruction register
    pub ir: IR,
    _reserved16: [u8; 0x0c],
    ///0x120 - OCTOSPI alternate bytes register
    pub abr: ABR,
    _reserved17: [u8; 0x0c],
    ///0x130 - OCTOSPI low-power timeout register
    pub lptr: LPTR,
    _reserved18: [u8; 0x0c],
    ///0x140 - OCTOSPI wrap communication configuration register
    pub wpccr: WPCCR,
    _reserved19: [u8; 0x04],
    ///0x148 - OCTOSPI wrap timing configuration register
    pub wptcr: WPTCR,
    _reserved20: [u8; 0x04],
    ///0x150 - OCTOSPI wrap instruction register
    pub wpir: WPIR,
    _reserved21: [u8; 0x0c],
    ///0x160 - OCTOSPI wrap alternate bytes register
    pub wpabr: WPABR,
    _reserved22: [u8; 0x1c],
    ///0x180 - OCTOSPI write communication configuration register
    pub wccr: WCCR,
    _reserved23: [u8; 0x04],
    ///0x188 - OCTOSPI write timing configuration register
    pub wtcr: WTCR,
    _reserved24: [u8; 0x04],
    ///0x190 - OCTOSPI write instruction register
    pub wir: WIR,
    _reserved25: [u8; 0x0c],
    ///0x1a0 - OCTOSPI write alternate bytes register
    pub wabr: WABR,
    _reserved26: [u8; 0x5c],
    ///0x200 - OCTOSPI HyperBus latency configuration register
    pub hlcr: HLCR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///OCTOSPI control register
pub mod cr;
///DCR1 (rw) register accessor: an alias for `Reg<DCR1_SPEC>`
pub type DCR1 = crate::Reg<dcr1::DCR1_SPEC>;
///OCTOSPI device configuration register 1
pub mod dcr1;
///DCR2 (rw) register accessor: an alias for `Reg<DCR2_SPEC>`
pub type DCR2 = crate::Reg<dcr2::DCR2_SPEC>;
///OCTOSPI device configuration register 2
pub mod dcr2;
///DCR3 (rw) register accessor: an alias for `Reg<DCR3_SPEC>`
pub type DCR3 = crate::Reg<dcr3::DCR3_SPEC>;
///OCTOSPI device configuration register 3
pub mod dcr3;
///DCR4 (rw) register accessor: an alias for `Reg<DCR4_SPEC>`
pub type DCR4 = crate::Reg<dcr4::DCR4_SPEC>;
///OCTOSPI device configuration register 4
pub mod dcr4;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///OCTOSPI status register
pub mod sr;
///FCR (w) register accessor: an alias for `Reg<FCR_SPEC>`
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
///OCTOSPI flag clear register
pub mod fcr;
///DLR (rw) register accessor: an alias for `Reg<DLR_SPEC>`
pub type DLR = crate::Reg<dlr::DLR_SPEC>;
///OCTOSPI data length register
pub mod dlr;
///AR (rw) register accessor: an alias for `Reg<AR_SPEC>`
pub type AR = crate::Reg<ar::AR_SPEC>;
///OCTOSPI address register
pub mod ar;
///DR (rw) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///OCTOSPI data register
pub mod dr;
///PSMKR (rw) register accessor: an alias for `Reg<PSMKR_SPEC>`
pub type PSMKR = crate::Reg<psmkr::PSMKR_SPEC>;
///OCTOSPI polling status mask register
pub mod psmkr;
///PSMAR (rw) register accessor: an alias for `Reg<PSMAR_SPEC>`
pub type PSMAR = crate::Reg<psmar::PSMAR_SPEC>;
///OCTOSPI polling status match register
pub mod psmar;
///PIR (rw) register accessor: an alias for `Reg<PIR_SPEC>`
pub type PIR = crate::Reg<pir::PIR_SPEC>;
///OCTOSPI polling interval register
pub mod pir;
///CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
///OCTOSPI communication configuration register
pub mod ccr;
///TCR (rw) register accessor: an alias for `Reg<TCR_SPEC>`
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
///OCTOSPI timing configuration register
pub mod tcr;
///IR (rw) register accessor: an alias for `Reg<IR_SPEC>`
pub type IR = crate::Reg<ir::IR_SPEC>;
///OCTOSPI instruction register
pub mod ir;
///ABR (rw) register accessor: an alias for `Reg<ABR_SPEC>`
pub type ABR = crate::Reg<abr::ABR_SPEC>;
///OCTOSPI alternate bytes register
pub mod abr;
///LPTR (rw) register accessor: an alias for `Reg<LPTR_SPEC>`
pub type LPTR = crate::Reg<lptr::LPTR_SPEC>;
///OCTOSPI low-power timeout register
pub mod lptr;
///WPCCR (rw) register accessor: an alias for `Reg<WPCCR_SPEC>`
pub type WPCCR = crate::Reg<wpccr::WPCCR_SPEC>;
///OCTOSPI wrap communication configuration register
pub mod wpccr;
///WPTCR (rw) register accessor: an alias for `Reg<WPTCR_SPEC>`
pub type WPTCR = crate::Reg<wptcr::WPTCR_SPEC>;
///OCTOSPI wrap timing configuration register
pub mod wptcr;
///WPIR (rw) register accessor: an alias for `Reg<WPIR_SPEC>`
pub type WPIR = crate::Reg<wpir::WPIR_SPEC>;
///OCTOSPI wrap instruction register
pub mod wpir;
///WPABR (rw) register accessor: an alias for `Reg<WPABR_SPEC>`
pub type WPABR = crate::Reg<wpabr::WPABR_SPEC>;
///OCTOSPI wrap alternate bytes register
pub mod wpabr;
///WCCR (rw) register accessor: an alias for `Reg<WCCR_SPEC>`
pub type WCCR = crate::Reg<wccr::WCCR_SPEC>;
///OCTOSPI write communication configuration register
pub mod wccr;
///WTCR (rw) register accessor: an alias for `Reg<WTCR_SPEC>`
pub type WTCR = crate::Reg<wtcr::WTCR_SPEC>;
///OCTOSPI write timing configuration register
pub mod wtcr;
///WIR (rw) register accessor: an alias for `Reg<WIR_SPEC>`
pub type WIR = crate::Reg<wir::WIR_SPEC>;
///OCTOSPI write instruction register
pub mod wir;
///WABR (rw) register accessor: an alias for `Reg<WABR_SPEC>`
pub type WABR = crate::Reg<wabr::WABR_SPEC>;
///OCTOSPI write alternate bytes register
pub mod wabr;
///HLCR (rw) register accessor: an alias for `Reg<HLCR_SPEC>`
pub type HLCR = crate::Reg<hlcr::HLCR_SPEC>;
///OCTOSPI HyperBus latency configuration register
pub mod hlcr;
