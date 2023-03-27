///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - QUADSPI control register
    pub cr: CR,
    ///0x04 - QUADSPI device configuration register
    pub dcr: DCR,
    ///0x08 - QUADSPI status register
    pub sr: SR,
    ///0x0c - QUADSPI flag clear register
    pub fcr: FCR,
    ///0x10 - QUADSPI data length register
    pub dlr: DLR,
    ///0x14 - QUADSPI communication configuration register
    pub ccr: CCR,
    ///0x18 - QUADSPI address register
    pub ar: AR,
    ///0x1c - QUADSPI alternate bytes registers
    pub abr: ABR,
    ///0x20 - QUADSPI data register
    pub dr: DR,
    ///0x24 - QUADSPI polling status mask register
    pub psmkr: PSMKR,
    ///0x28 - QUADSPI polling status match register
    pub psmar: PSMAR,
    ///0x2c - QUADSPI polling interval register
    pub pir: PIR,
    ///0x30 - QUADSPI low-power timeout register
    pub lptr: LPTR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///QUADSPI control register
pub mod cr;
///DCR (rw) register accessor: an alias for `Reg<DCR_SPEC>`
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
///QUADSPI device configuration register
pub mod dcr;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///QUADSPI status register
pub mod sr;
///FCR (rw) register accessor: an alias for `Reg<FCR_SPEC>`
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
///QUADSPI flag clear register
pub mod fcr;
///DLR (rw) register accessor: an alias for `Reg<DLR_SPEC>`
pub type DLR = crate::Reg<dlr::DLR_SPEC>;
///QUADSPI data length register
pub mod dlr;
///CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
///QUADSPI communication configuration register
pub mod ccr;
///AR (rw) register accessor: an alias for `Reg<AR_SPEC>`
pub type AR = crate::Reg<ar::AR_SPEC>;
///QUADSPI address register
pub mod ar;
///ABR (rw) register accessor: an alias for `Reg<ABR_SPEC>`
pub type ABR = crate::Reg<abr::ABR_SPEC>;
///QUADSPI alternate bytes registers
pub mod abr;
///DR (rw) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///QUADSPI data register
pub mod dr;
///PSMKR (rw) register accessor: an alias for `Reg<PSMKR_SPEC>`
pub type PSMKR = crate::Reg<psmkr::PSMKR_SPEC>;
///QUADSPI polling status mask register
pub mod psmkr;
///PSMAR (rw) register accessor: an alias for `Reg<PSMAR_SPEC>`
pub type PSMAR = crate::Reg<psmar::PSMAR_SPEC>;
///QUADSPI polling status match register
pub mod psmar;
///PIR (rw) register accessor: an alias for `Reg<PIR_SPEC>`
pub type PIR = crate::Reg<pir::PIR_SPEC>;
///QUADSPI polling interval register
pub mod pir;
///LPTR (rw) register accessor: an alias for `Reg<LPTR_SPEC>`
pub type LPTR = crate::Reg<lptr::LPTR_SPEC>;
///QUADSPI low-power timeout register
pub mod lptr;
