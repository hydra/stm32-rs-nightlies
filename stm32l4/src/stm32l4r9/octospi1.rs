///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: CR,
    _reserved1: [u8; 0x04],
    ///0x08 - device configuration register
    pub dcr1: DCR1,
    ///0x0c - device configuration register 2
    pub dcr2: DCR2,
    ///0x10 - device configuration register 3
    pub dcr3: DCR3,
    _reserved4: [u8; 0x0c],
    ///0x20 - status register
    pub sr: SR,
    ///0x24 - flag clear register
    pub fcr: FCR,
    _reserved6: [u8; 0x18],
    ///0x40 - data length register
    pub dlr: DLR,
    _reserved7: [u8; 0x04],
    ///0x48 - address register
    pub ar: AR,
    _reserved8: [u8; 0x04],
    ///0x50 - data register
    pub dr: DR,
    _reserved9: [u8; 0x2c],
    ///0x80 - polling status mask register
    pub psmkr: PSMKR,
    _reserved10: [u8; 0x04],
    ///0x88 - polling status match register
    pub psmar: PSMAR,
    _reserved11: [u8; 0x04],
    ///0x90 - polling interval register
    pub pir: PIR,
    _reserved12: [u8; 0x6c],
    ///0x100 - communication configuration register
    pub ccr: CCR,
    _reserved13: [u8; 0x04],
    ///0x108 - timing configuration register
    pub tcr: TCR,
    _reserved14: [u8; 0x04],
    ///0x110 - instruction register
    pub ir: IR,
    _reserved15: [u8; 0x0c],
    ///0x120 - alternate bytes register
    pub abr: ABR,
    _reserved16: [u8; 0x0c],
    ///0x130 - low-power timeout register
    pub lptr: LPTR,
    _reserved17: [u8; 0x4c],
    ///0x180 - write communication configuration register
    pub wccr: WCCR,
    _reserved18: [u8; 0x04],
    ///0x188 - write timing configuration register
    pub wtcr: WTCR,
    _reserved19: [u8; 0x04],
    ///0x190 - write instruction register
    pub wir: WIR,
    _reserved20: [u8; 0x0c],
    ///0x1a0 - write alternate bytes register
    pub wabr: WABR,
    _reserved21: [u8; 0x5c],
    ///0x200 - HyperBusTM latency configuration register
    pub hlcr: HLCR,
    _reserved22: [u8; 0x01ec],
    ///0x3f0 - HW configuration register
    pub hwcfgr: HWCFGR,
    ///0x3f4 - version register
    pub ver: VER,
    ///0x3f8 - identification
    pub id: ID,
    ///0x3fc - magic ID
    pub mid: MID,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///control register
pub mod cr;
///DCR1 (rw) register accessor: an alias for `Reg<DCR1_SPEC>`
pub type DCR1 = crate::Reg<dcr1::DCR1_SPEC>;
///device configuration register
pub mod dcr1;
///DCR2 (rw) register accessor: an alias for `Reg<DCR2_SPEC>`
pub type DCR2 = crate::Reg<dcr2::DCR2_SPEC>;
///device configuration register 2
pub mod dcr2;
///DCR3 (rw) register accessor: an alias for `Reg<DCR3_SPEC>`
pub type DCR3 = crate::Reg<dcr3::DCR3_SPEC>;
///device configuration register 3
pub mod dcr3;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///status register
pub mod sr;
///FCR (w) register accessor: an alias for `Reg<FCR_SPEC>`
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
///flag clear register
pub mod fcr;
///DLR (rw) register accessor: an alias for `Reg<DLR_SPEC>`
pub type DLR = crate::Reg<dlr::DLR_SPEC>;
///data length register
pub mod dlr;
///AR (rw) register accessor: an alias for `Reg<AR_SPEC>`
pub type AR = crate::Reg<ar::AR_SPEC>;
///address register
pub mod ar;
///DR (rw) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///data register
pub mod dr;
///PSMKR (rw) register accessor: an alias for `Reg<PSMKR_SPEC>`
pub type PSMKR = crate::Reg<psmkr::PSMKR_SPEC>;
///polling status mask register
pub mod psmkr;
///PSMAR (rw) register accessor: an alias for `Reg<PSMAR_SPEC>`
pub type PSMAR = crate::Reg<psmar::PSMAR_SPEC>;
///polling status match register
pub mod psmar;
///PIR (rw) register accessor: an alias for `Reg<PIR_SPEC>`
pub type PIR = crate::Reg<pir::PIR_SPEC>;
///polling interval register
pub mod pir;
///CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
///communication configuration register
pub mod ccr;
///TCR (rw) register accessor: an alias for `Reg<TCR_SPEC>`
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
///timing configuration register
pub mod tcr;
///IR (rw) register accessor: an alias for `Reg<IR_SPEC>`
pub type IR = crate::Reg<ir::IR_SPEC>;
///instruction register
pub mod ir;
///ABR (rw) register accessor: an alias for `Reg<ABR_SPEC>`
pub type ABR = crate::Reg<abr::ABR_SPEC>;
///alternate bytes register
pub mod abr;
///LPTR (rw) register accessor: an alias for `Reg<LPTR_SPEC>`
pub type LPTR = crate::Reg<lptr::LPTR_SPEC>;
///low-power timeout register
pub mod lptr;
///WCCR (rw) register accessor: an alias for `Reg<WCCR_SPEC>`
pub type WCCR = crate::Reg<wccr::WCCR_SPEC>;
///write communication configuration register
pub mod wccr;
///WTCR (rw) register accessor: an alias for `Reg<WTCR_SPEC>`
pub type WTCR = crate::Reg<wtcr::WTCR_SPEC>;
///write timing configuration register
pub mod wtcr;
///WIR (rw) register accessor: an alias for `Reg<WIR_SPEC>`
pub type WIR = crate::Reg<wir::WIR_SPEC>;
///write instruction register
pub mod wir;
///WABR (rw) register accessor: an alias for `Reg<WABR_SPEC>`
pub type WABR = crate::Reg<wabr::WABR_SPEC>;
///write alternate bytes register
pub mod wabr;
///HLCR (rw) register accessor: an alias for `Reg<HLCR_SPEC>`
pub type HLCR = crate::Reg<hlcr::HLCR_SPEC>;
///HyperBusTM latency configuration register
pub mod hlcr;
///HWCFGR (r) register accessor: an alias for `Reg<HWCFGR_SPEC>`
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGR_SPEC>;
///HW configuration register
pub mod hwcfgr;
///VER (r) register accessor: an alias for `Reg<VER_SPEC>`
pub type VER = crate::Reg<ver::VER_SPEC>;
///version register
pub mod ver;
///ID (r) register accessor: an alias for `Reg<ID_SPEC>`
pub type ID = crate::Reg<id::ID_SPEC>;
///identification
pub mod id;
///MID (r) register accessor: an alias for `Reg<MID_SPEC>`
pub type MID = crate::Reg<mid::MID_SPEC>;
///magic ID
pub mod mid;
