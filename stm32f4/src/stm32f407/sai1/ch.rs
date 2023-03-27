///Register block
#[repr(C)]
pub struct CH {
    ///0x00 - SAI AConfiguration register 1
    pub cr1: CR1,
    ///0x04 - SAI AConfiguration register 2
    pub cr2: CR2,
    ///0x08 - SAI AFrame configuration register
    pub frcr: FRCR,
    ///0x0c - SAI ASlot register
    pub slotr: SLOTR,
    ///0x10 - SAI AInterrupt mask register2
    pub im: IM,
    ///0x14 - SAI AStatus register
    pub sr: SR,
    ///0x18 - SAI AClear flag register
    pub clrfr: CLRFR,
    ///0x1c - SAI AData register
    pub dr: DR,
}
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///SAI AConfiguration register 1
pub mod cr1;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///SAI AConfiguration register 2
pub mod cr2;
///FRCR (rw) register accessor: an alias for `Reg<FRCR_SPEC>`
pub type FRCR = crate::Reg<frcr::FRCR_SPEC>;
///SAI AFrame configuration register
pub mod frcr;
///SLOTR (rw) register accessor: an alias for `Reg<SLOTR_SPEC>`
pub type SLOTR = crate::Reg<slotr::SLOTR_SPEC>;
///SAI ASlot register
pub mod slotr;
///IM (rw) register accessor: an alias for `Reg<IM_SPEC>`
pub type IM = crate::Reg<im::IM_SPEC>;
///SAI AInterrupt mask register2
pub mod im;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///SAI AStatus register
pub mod sr;
///CLRFR (rw) register accessor: an alias for `Reg<CLRFR_SPEC>`
pub type CLRFR = crate::Reg<clrfr::CLRFR_SPEC>;
///SAI AClear flag register
pub mod clrfr;
///DR (rw) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///SAI AData register
pub mod dr;
