///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DCACHE control register
    pub cr: CR,
    ///0x04 - DCACHE status register
    pub sr: SR,
    ///0x08 - DCACHE interrupt enable register
    pub ier: IER,
    ///0x0c - DCACHE flag clear register
    pub fcr: FCR,
    ///0x10 - DCACHE read-hit monitor register
    pub rhmonr: RHMONR,
    ///0x14 - DCACHE read-miss monitor register
    pub rmmonr: RMMONR,
    _reserved6: [u8; 0x08],
    ///0x20 - DCACHE write-hit monitor register
    pub whmonr: WHMONR,
    ///0x24 - DCACHE write-miss monitor register
    pub wmmonr: WMMONR,
    ///0x28 - DCACHE command range start address register
    pub cmdrsaddrr: CMDRSADDRR,
    ///0x2c - DCACHE command range end address register
    pub cmdreaddrr: CMDREADDRR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///DCACHE control register
pub mod cr;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///DCACHE status register
pub mod sr;
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///DCACHE interrupt enable register
pub mod ier;
///FCR (w) register accessor: an alias for `Reg<FCR_SPEC>`
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
///DCACHE flag clear register
pub mod fcr;
///RHMONR (r) register accessor: an alias for `Reg<RHMONR_SPEC>`
pub type RHMONR = crate::Reg<rhmonr::RHMONR_SPEC>;
///DCACHE read-hit monitor register
pub mod rhmonr;
///RMMONR (r) register accessor: an alias for `Reg<RMMONR_SPEC>`
pub type RMMONR = crate::Reg<rmmonr::RMMONR_SPEC>;
///DCACHE read-miss monitor register
pub mod rmmonr;
///WHMONR (r) register accessor: an alias for `Reg<WHMONR_SPEC>`
pub type WHMONR = crate::Reg<whmonr::WHMONR_SPEC>;
///DCACHE write-hit monitor register
pub mod whmonr;
///WMMONR (r) register accessor: an alias for `Reg<WMMONR_SPEC>`
pub type WMMONR = crate::Reg<wmmonr::WMMONR_SPEC>;
///DCACHE write-miss monitor register
pub mod wmmonr;
///CMDRSADDRR (rw) register accessor: an alias for `Reg<CMDRSADDRR_SPEC>`
pub type CMDRSADDRR = crate::Reg<cmdrsaddrr::CMDRSADDRR_SPEC>;
///DCACHE command range start address register
pub mod cmdrsaddrr;
///CMDREADDRR (rw) register accessor: an alias for `Reg<CMDREADDRR_SPEC>`
pub type CMDREADDRR = crate::Reg<cmdreaddrr::CMDREADDRR_SPEC>;
///DCACHE command range end address register
pub mod cmdreaddrr;
