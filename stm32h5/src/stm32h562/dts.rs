///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Temperature sensor configuration register 1
    pub cfgr1: CFGR1,
    _reserved1: [u8; 0x04],
    ///0x08 - Temperature sensor T0 value register 1
    pub t0valr1: T0VALR1,
    _reserved2: [u8; 0x04],
    ///0x10 - Temperature sensor ramp value register
    pub rampvalr: RAMPVALR,
    ///0x14 - Temperature sensor interrupt threshold register 1
    pub itr1: ITR1,
    _reserved4: [u8; 0x04],
    ///0x1c - Temperature sensor data register
    pub dr: DR,
    ///0x20 - Temperature sensor status register
    pub sr: SR,
    ///0x24 - Temperature sensor interrupt enable register
    pub itenr: ITENR,
    ///0x28 - Temperature sensor clear interrupt flag register
    pub icifr: ICIFR,
    ///0x2c - Temperature sensor option register
    pub or: OR,
}
///CFGR1 (rw) register accessor: an alias for `Reg<CFGR1_SPEC>`
pub type CFGR1 = crate::Reg<cfgr1::CFGR1_SPEC>;
///Temperature sensor configuration register 1
pub mod cfgr1;
///T0VALR1 (r) register accessor: an alias for `Reg<T0VALR1_SPEC>`
pub type T0VALR1 = crate::Reg<t0valr1::T0VALR1_SPEC>;
///Temperature sensor T0 value register 1
pub mod t0valr1;
///RAMPVALR (r) register accessor: an alias for `Reg<RAMPVALR_SPEC>`
pub type RAMPVALR = crate::Reg<rampvalr::RAMPVALR_SPEC>;
///Temperature sensor ramp value register
pub mod rampvalr;
///ITR1 (rw) register accessor: an alias for `Reg<ITR1_SPEC>`
pub type ITR1 = crate::Reg<itr1::ITR1_SPEC>;
///Temperature sensor interrupt threshold register 1
pub mod itr1;
///DR (rw) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///Temperature sensor data register
pub mod dr;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///Temperature sensor status register
pub mod sr;
///ITENR (rw) register accessor: an alias for `Reg<ITENR_SPEC>`
pub type ITENR = crate::Reg<itenr::ITENR_SPEC>;
///Temperature sensor interrupt enable register
pub mod itenr;
///ICIFR (rw) register accessor: an alias for `Reg<ICIFR_SPEC>`
pub type ICIFR = crate::Reg<icifr::ICIFR_SPEC>;
///Temperature sensor clear interrupt flag register
pub mod icifr;
///OR (rw) register accessor: an alias for `Reg<OR_SPEC>`
pub type OR = crate::Reg<or::OR_SPEC>;
///Temperature sensor option register
pub mod or;
