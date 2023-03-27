///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - ICACHE control register
    pub cr: CR,
    ///0x04 - ICACHE status register
    pub sr: SR,
    ///0x08 - ICACHE interrupt enable register
    pub ier: IER,
    ///0x0c - ICACHE flag clear register
    pub fcr: FCR,
    ///0x10 - ICACHE hit monitor register
    pub hmonr: HMONR,
    ///0x14 - ICACHE miss monitor register
    pub mmonr: MMONR,
    _reserved6: [u8; 0x08],
    ///0x20 - ICACHE region 0 configuration register
    pub crr0: CRR0,
    ///0x24 - ICACHE region 1 configuration register
    pub crr1: CRR1,
    ///0x28 - ICACHE region 2 configuration register
    pub crr2: CRR2,
    ///0x2c - ICACHE region 3 configuration register
    pub crr3: CRR3,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///ICACHE control register
pub mod cr;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///ICACHE status register
pub mod sr;
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///ICACHE interrupt enable register
pub mod ier;
///FCR (w) register accessor: an alias for `Reg<FCR_SPEC>`
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
///ICACHE flag clear register
pub mod fcr;
///HMONR (r) register accessor: an alias for `Reg<HMONR_SPEC>`
pub type HMONR = crate::Reg<hmonr::HMONR_SPEC>;
///ICACHE hit monitor register
pub mod hmonr;
///MMONR (r) register accessor: an alias for `Reg<MMONR_SPEC>`
pub type MMONR = crate::Reg<mmonr::MMONR_SPEC>;
///ICACHE miss monitor register
pub mod mmonr;
///CRR0 (rw) register accessor: an alias for `Reg<CRR0_SPEC>`
pub type CRR0 = crate::Reg<crr0::CRR0_SPEC>;
///ICACHE region 0 configuration register
pub mod crr0;
///CRR1 (rw) register accessor: an alias for `Reg<CRR1_SPEC>`
pub type CRR1 = crate::Reg<crr1::CRR1_SPEC>;
///ICACHE region 1 configuration register
pub mod crr1;
///CRR2 (rw) register accessor: an alias for `Reg<CRR2_SPEC>`
pub type CRR2 = crate::Reg<crr2::CRR2_SPEC>;
///ICACHE region 2 configuration register
pub mod crr2;
///CRR3 (rw) register accessor: an alias for `Reg<CRR3_SPEC>`
pub type CRR3 = crate::Reg<crr3::CRR3_SPEC>;
///ICACHE region 3 configuration register
pub mod crr3;
