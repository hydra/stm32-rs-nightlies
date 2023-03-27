///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - BCR1
    pub bcr1: BCR1,
    ///0x04 - BTR1
    pub btr1: BTR,
    ///0x08 - BCR2
    pub bcr2: BCR,
    ///0x0c - BTR1
    pub btr2: BTR,
    ///0x10 - BCR2
    pub bcr3: BCR,
    ///0x14 - BTR1
    pub btr3: BTR,
    ///0x18 - BCR2
    pub bcr4: BCR,
    ///0x1c - BTR1
    pub btr4: BTR,
    _reserved8: [u8; 0xe4],
    ///0x104 - BWTR1
    pub bwtr1: BWTR,
    _reserved9: [u8; 0x04],
    ///0x10c - BWTR1
    pub bwtr2: BWTR,
    _reserved10: [u8; 0x04],
    ///0x114 - BWTR1
    pub bwtr3: BWTR,
    _reserved11: [u8; 0x04],
    ///0x11c - BWTR1
    pub bwtr4: BWTR,
}
pub use bcr as bcr1;
pub use BCR as BCR1;
///BTR (rw) register accessor: an alias for `Reg<BTR_SPEC>`
pub type BTR = crate::Reg<btr::BTR_SPEC>;
///BTR1
pub mod btr;
///BCR (rw) register accessor: an alias for `Reg<BCR_SPEC>`
pub type BCR = crate::Reg<bcr::BCR_SPEC>;
///BCR2
pub mod bcr;
///BWTR (rw) register accessor: an alias for `Reg<BWTR_SPEC>`
pub type BWTR = crate::Reg<bwtr::BWTR_SPEC>;
///BWTR1
pub mod bwtr;
