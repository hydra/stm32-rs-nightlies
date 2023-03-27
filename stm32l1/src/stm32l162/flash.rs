///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Access control register
    pub acr: ACR,
    ///0x04 - Program/erase control register
    pub pecr: PECR,
    ///0x08 - Power down key register
    pub pdkeyr: PDKEYR,
    ///0x0c - Program/erase key register
    pub pekeyr: PEKEYR,
    ///0x10 - Program memory key register
    pub prgkeyr: PRGKEYR,
    ///0x14 - Option byte key register
    pub optkeyr: OPTKEYR,
    ///0x18 - Status register
    pub sr: SR,
    ///0x1c - Option byte register
    pub obr: OBR,
    ///0x20 - Write protection register
    pub wrpr1: WRPR1,
    _reserved9: [u8; 0x5c],
    ///0x80 - Write protection register
    pub wrpr2: WRPR2,
    ///0x84 - Write protection register
    pub wrpr3: WRPR3,
}
///ACR (rw) register accessor: an alias for `Reg<ACR_SPEC>`
pub type ACR = crate::Reg<acr::ACR_SPEC>;
///Access control register
pub mod acr;
///PECR (rw) register accessor: an alias for `Reg<PECR_SPEC>`
pub type PECR = crate::Reg<pecr::PECR_SPEC>;
///Program/erase control register
pub mod pecr;
///PDKEYR (w) register accessor: an alias for `Reg<PDKEYR_SPEC>`
pub type PDKEYR = crate::Reg<pdkeyr::PDKEYR_SPEC>;
///Power down key register
pub mod pdkeyr;
///PEKEYR (w) register accessor: an alias for `Reg<PEKEYR_SPEC>`
pub type PEKEYR = crate::Reg<pekeyr::PEKEYR_SPEC>;
///Program/erase key register
pub mod pekeyr;
///PRGKEYR (w) register accessor: an alias for `Reg<PRGKEYR_SPEC>`
pub type PRGKEYR = crate::Reg<prgkeyr::PRGKEYR_SPEC>;
///Program memory key register
pub mod prgkeyr;
///OPTKEYR (w) register accessor: an alias for `Reg<OPTKEYR_SPEC>`
pub type OPTKEYR = crate::Reg<optkeyr::OPTKEYR_SPEC>;
///Option byte key register
pub mod optkeyr;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///Status register
pub mod sr;
///OBR (r) register accessor: an alias for `Reg<OBR_SPEC>`
pub type OBR = crate::Reg<obr::OBR_SPEC>;
///Option byte register
pub mod obr;
///WRPR1 (rw) register accessor: an alias for `Reg<WRPR1_SPEC>`
pub type WRPR1 = crate::Reg<wrpr1::WRPR1_SPEC>;
///Write protection register
pub mod wrpr1;
///WRPR2 (rw) register accessor: an alias for `Reg<WRPR2_SPEC>`
pub type WRPR2 = crate::Reg<wrpr2::WRPR2_SPEC>;
///Write protection register
pub mod wrpr2;
///WRPR3 (rw) register accessor: an alias for `Reg<WRPR3_SPEC>`
pub type WRPR3 = crate::Reg<wrpr3::WRPR3_SPEC>;
///Write protection register
pub mod wrpr3;
