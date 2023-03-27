///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    ///0x04 - RI input capture register
    pub icr: ICR,
    ///0x08 - RI analog switches control register 1
    pub ascr1: ASCR1,
    ///0x0c - RI analog switches control register 2
    pub ascr2: ASCR2,
    ///0x10 - RI hysteresis control register 1
    pub hyscr1: HYSCR1,
    ///0x14 - RI hysteresis control register 2
    pub hyscr2: HYSCR2,
    ///0x18 - RI hysteresis control register 3
    pub hyscr3: HYSCR3,
    ///0x1c - Hysteresis control register
    pub hyscr4: HYSCR4,
}
///ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///RI input capture register
pub mod icr;
///ASCR1 (rw) register accessor: an alias for `Reg<ASCR1_SPEC>`
pub type ASCR1 = crate::Reg<ascr1::ASCR1_SPEC>;
///RI analog switches control register 1
pub mod ascr1;
///ASCR2 (rw) register accessor: an alias for `Reg<ASCR2_SPEC>`
pub type ASCR2 = crate::Reg<ascr2::ASCR2_SPEC>;
///RI analog switches control register 2
pub mod ascr2;
///HYSCR1 (rw) register accessor: an alias for `Reg<HYSCR1_SPEC>`
pub type HYSCR1 = crate::Reg<hyscr1::HYSCR1_SPEC>;
///RI hysteresis control register 1
pub mod hyscr1;
///HYSCR2 (rw) register accessor: an alias for `Reg<HYSCR2_SPEC>`
pub type HYSCR2 = crate::Reg<hyscr2::HYSCR2_SPEC>;
///RI hysteresis control register 2
pub mod hyscr2;
///HYSCR3 (rw) register accessor: an alias for `Reg<HYSCR3_SPEC>`
pub type HYSCR3 = crate::Reg<hyscr3::HYSCR3_SPEC>;
///RI hysteresis control register 3
pub mod hyscr3;
///HYSCR4 (rw) register accessor: an alias for `Reg<HYSCR4_SPEC>`
pub type HYSCR4 = crate::Reg<hyscr4::HYSCR4_SPEC>;
///Hysteresis control register
pub mod hyscr4;
