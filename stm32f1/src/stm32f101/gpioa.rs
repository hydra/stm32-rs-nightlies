///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Port configuration register low (GPIOn_CRL)
    pub crl: CRL,
    ///0x04 - Port configuration register high (GPIOn_CRL)
    pub crh: CRH,
    ///0x08 - Port input data register (GPIOn_IDR)
    pub idr: IDR,
    ///0x0c - Port output data register (GPIOn_ODR)
    pub odr: ODR,
    ///0x10 - Port bit set/reset register (GPIOn_BSRR)
    pub bsrr: BSRR,
    ///0x14 - Port bit reset register (GPIOn_BRR)
    pub brr: BRR,
    ///0x18 - Port configuration lock register
    pub lckr: LCKR,
}
///CRL (rw) register accessor: an alias for `Reg<CRL_SPEC>`
pub type CRL = crate::Reg<crl::CRL_SPEC>;
///Port configuration register low (GPIOn_CRL)
pub mod crl;
///CRH (rw) register accessor: an alias for `Reg<CRH_SPEC>`
pub type CRH = crate::Reg<crh::CRH_SPEC>;
///Port configuration register high (GPIOn_CRL)
pub mod crh;
///IDR (r) register accessor: an alias for `Reg<IDR_SPEC>`
pub type IDR = crate::Reg<idr::IDR_SPEC>;
///Port input data register (GPIOn_IDR)
pub mod idr;
///ODR (rw) register accessor: an alias for `Reg<ODR_SPEC>`
pub type ODR = crate::Reg<odr::ODR_SPEC>;
///Port output data register (GPIOn_ODR)
pub mod odr;
///BSRR (w) register accessor: an alias for `Reg<BSRR_SPEC>`
pub type BSRR = crate::Reg<bsrr::BSRR_SPEC>;
///Port bit set/reset register (GPIOn_BSRR)
pub mod bsrr;
///BRR (w) register accessor: an alias for `Reg<BRR_SPEC>`
pub type BRR = crate::Reg<brr::BRR_SPEC>;
///Port bit reset register (GPIOn_BRR)
pub mod brr;
///LCKR (rw) register accessor: an alias for `Reg<LCKR_SPEC>`
pub type LCKR = crate::Reg<lckr::LCKR_SPEC>;
///Port configuration lock register
pub mod lckr;
