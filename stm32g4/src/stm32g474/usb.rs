///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - USB endpoint n register
    pub ep0r: EP0R,
    ///0x04 - USB endpoint n register
    pub ep1r: EP1R,
    ///0x08 - USB endpoint n register
    pub ep2r: EP2R,
    ///0x0c - USB endpoint n register
    pub ep3r: EP3R,
    ///0x10 - USB endpoint n register
    pub ep4r: EP4R,
    ///0x14 - USB endpoint n register
    pub ep5r: EP5R,
    ///0x18 - USB endpoint n register
    pub ep6r: EP6R,
    ///0x1c - USB endpoint n register
    pub ep7r: EP7R,
    _reserved8: [u8; 0x20],
    ///0x40 - USB control register
    pub cntr: CNTR,
    ///0x44 - USB interrupt status register
    pub istr: ISTR,
    ///0x48 - USB frame number register
    pub fnr: FNR,
    ///0x4c - USB device address
    pub daddr: DADDR,
    ///0x50 - Buffer table address
    pub btable: BTABLE,
    _reserved13: [u8; 0x04],
    ///0x58 - Battery Charging Detector
    pub bcdr: BCDR,
}
///EP0R (rw) register accessor: an alias for `Reg<EP0R_SPEC>`
pub type EP0R = crate::Reg<ep0r::EP0R_SPEC>;
///USB endpoint n register
pub mod ep0r;
///EP1R (rw) register accessor: an alias for `Reg<EP1R_SPEC>`
pub type EP1R = crate::Reg<ep1r::EP1R_SPEC>;
///USB endpoint n register
pub mod ep1r;
///EP2R (rw) register accessor: an alias for `Reg<EP2R_SPEC>`
pub type EP2R = crate::Reg<ep2r::EP2R_SPEC>;
///USB endpoint n register
pub mod ep2r;
///EP3R (rw) register accessor: an alias for `Reg<EP3R_SPEC>`
pub type EP3R = crate::Reg<ep3r::EP3R_SPEC>;
///USB endpoint n register
pub mod ep3r;
///EP4R (rw) register accessor: an alias for `Reg<EP4R_SPEC>`
pub type EP4R = crate::Reg<ep4r::EP4R_SPEC>;
///USB endpoint n register
pub mod ep4r;
///EP5R (rw) register accessor: an alias for `Reg<EP5R_SPEC>`
pub type EP5R = crate::Reg<ep5r::EP5R_SPEC>;
///USB endpoint n register
pub mod ep5r;
///EP6R (rw) register accessor: an alias for `Reg<EP6R_SPEC>`
pub type EP6R = crate::Reg<ep6r::EP6R_SPEC>;
///USB endpoint n register
pub mod ep6r;
///EP7R (rw) register accessor: an alias for `Reg<EP7R_SPEC>`
pub type EP7R = crate::Reg<ep7r::EP7R_SPEC>;
///USB endpoint n register
pub mod ep7r;
///CNTR (rw) register accessor: an alias for `Reg<CNTR_SPEC>`
pub type CNTR = crate::Reg<cntr::CNTR_SPEC>;
///USB control register
pub mod cntr;
///ISTR (rw) register accessor: an alias for `Reg<ISTR_SPEC>`
pub type ISTR = crate::Reg<istr::ISTR_SPEC>;
///USB interrupt status register
pub mod istr;
///FNR (r) register accessor: an alias for `Reg<FNR_SPEC>`
pub type FNR = crate::Reg<fnr::FNR_SPEC>;
///USB frame number register
pub mod fnr;
///DADDR (rw) register accessor: an alias for `Reg<DADDR_SPEC>`
pub type DADDR = crate::Reg<daddr::DADDR_SPEC>;
///USB device address
pub mod daddr;
///BTABLE (rw) register accessor: an alias for `Reg<BTABLE_SPEC>`
pub type BTABLE = crate::Reg<btable::BTABLE_SPEC>;
///Buffer table address
pub mod btable;
///BCDR (rw) register accessor: an alias for `Reg<BCDR_SPEC>`
pub type BCDR = crate::Reg<bcdr::BCDR_SPEC>;
///Battery Charging Detector
pub mod bcdr;
