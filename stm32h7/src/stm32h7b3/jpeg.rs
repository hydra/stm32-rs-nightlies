///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - JPEG codec control register
    pub confr0: CONFR0,
    ///0x04 - JPEG codec configuration register 1
    pub confr1: CONFR1,
    ///0x08 - JPEG codec configuration register 2
    pub confr2: CONFR2,
    ///0x0c - JPEG codec configuration register 3
    pub confr3: CONFR3,
    ///0x10 - JPEG codec configuration register 4-7
    pub confrn1: CONFRN1,
    ///0x14 - JPEG codec configuration register 4-7
    pub confrn2: CONFRN2,
    ///0x18 - JPEG codec configuration register 4-7
    pub confrn3: CONFRN3,
    ///0x1c - JPEG codec configuration register 4-7
    pub confrn4: CONFRN4,
    _reserved8: [u8; 0x10],
    ///0x30 - JPEG control register
    pub cr: CR,
    ///0x34 - JPEG status register
    pub sr: SR,
    ///0x38 - JPEG clear flag register
    pub cfr: CFR,
    _reserved11: [u8; 0x04],
    ///0x40 - JPEG data input register
    pub dir: DIR,
    ///0x44 - JPEG data output register
    pub dor: DOR,
}
///CONFR0 (w) register accessor: an alias for `Reg<CONFR0_SPEC>`
pub type CONFR0 = crate::Reg<confr0::CONFR0_SPEC>;
///JPEG codec control register
pub mod confr0;
///CONFR1 (rw) register accessor: an alias for `Reg<CONFR1_SPEC>`
pub type CONFR1 = crate::Reg<confr1::CONFR1_SPEC>;
///JPEG codec configuration register 1
pub mod confr1;
///CONFR2 (rw) register accessor: an alias for `Reg<CONFR2_SPEC>`
pub type CONFR2 = crate::Reg<confr2::CONFR2_SPEC>;
///JPEG codec configuration register 2
pub mod confr2;
///CONFR3 (rw) register accessor: an alias for `Reg<CONFR3_SPEC>`
pub type CONFR3 = crate::Reg<confr3::CONFR3_SPEC>;
///JPEG codec configuration register 3
pub mod confr3;
///CONFRN1 (rw) register accessor: an alias for `Reg<CONFRN1_SPEC>`
pub type CONFRN1 = crate::Reg<confrn1::CONFRN1_SPEC>;
///JPEG codec configuration register 4-7
pub mod confrn1;
///CONFRN2 (rw) register accessor: an alias for `Reg<CONFRN2_SPEC>`
pub type CONFRN2 = crate::Reg<confrn2::CONFRN2_SPEC>;
///JPEG codec configuration register 4-7
pub mod confrn2;
///CONFRN3 (rw) register accessor: an alias for `Reg<CONFRN3_SPEC>`
pub type CONFRN3 = crate::Reg<confrn3::CONFRN3_SPEC>;
///JPEG codec configuration register 4-7
pub mod confrn3;
///CONFRN4 (rw) register accessor: an alias for `Reg<CONFRN4_SPEC>`
pub type CONFRN4 = crate::Reg<confrn4::CONFRN4_SPEC>;
///JPEG codec configuration register 4-7
pub mod confrn4;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///JPEG control register
pub mod cr;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///JPEG status register
pub mod sr;
///CFR (rw) register accessor: an alias for `Reg<CFR_SPEC>`
pub type CFR = crate::Reg<cfr::CFR_SPEC>;
///JPEG clear flag register
pub mod cfr;
///DIR (w) register accessor: an alias for `Reg<DIR_SPEC>`
pub type DIR = crate::Reg<dir::DIR_SPEC>;
///JPEG data input register
pub mod dir;
///DOR (r) register accessor: an alias for `Reg<DOR_SPEC>`
pub type DOR = crate::Reg<dor::DOR_SPEC>;
///JPEG data output register
pub mod dor;
