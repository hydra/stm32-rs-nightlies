///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DTS_CFGR1 is the configuration register for temperature sensor 1.
    pub dts_cfgr1: DTS_CFGR1,
    _reserved1: [u8; 0x04],
    ///0x08 - DTS_T0VALR1 contains the value of the factory calibration temperature (T0) for temperature sensor 1. The system reset value is factory trimmed.
    pub dts_t0valr1: DTS_T0VALR1,
    _reserved2: [u8; 0x04],
    ///0x10 - The DTS_RAMPVALR is the ramp coefficient for the temperature sensor. The system reset value is factory trimmed.
    pub dts_rampvalr: DTS_RAMPVALR,
    ///0x14 - DTS_ITR1 contains the threshold values for sensor 1.
    pub dts_itr1: DTS_ITR1,
    _reserved4: [u8; 0x04],
    ///0x1c - The DTS_DR contains the number of REF_CLK cycles used to compute the FM(T) frequency.
    pub dts_dr: DTS_DR,
    ///0x20 - Temperature sensor status register
    pub dts_sr: DTS_SR,
    ///0x24 - Temperature sensor interrupt enable register
    pub dts_itenr: DTS_ITENR,
    ///0x28 - DTS_ICIFR is the control register for the interrupt flags.
    pub dts_icifr: DTS_ICIFR,
    ///0x2c - The DTS_OR contains general-purpose option bits.
    pub dts_or: DTS_OR,
}
///DTS_CFGR1 (rw) register accessor: an alias for `Reg<DTS_CFGR1_SPEC>`
pub type DTS_CFGR1 = crate::Reg<dts_cfgr1::DTS_CFGR1_SPEC>;
///DTS_CFGR1 is the configuration register for temperature sensor 1.
pub mod dts_cfgr1;
///DTS_T0VALR1 (r) register accessor: an alias for `Reg<DTS_T0VALR1_SPEC>`
pub type DTS_T0VALR1 = crate::Reg<dts_t0valr1::DTS_T0VALR1_SPEC>;
///DTS_T0VALR1 contains the value of the factory calibration temperature (T0) for temperature sensor 1. The system reset value is factory trimmed.
pub mod dts_t0valr1;
///DTS_RAMPVALR (r) register accessor: an alias for `Reg<DTS_RAMPVALR_SPEC>`
pub type DTS_RAMPVALR = crate::Reg<dts_rampvalr::DTS_RAMPVALR_SPEC>;
///The DTS_RAMPVALR is the ramp coefficient for the temperature sensor. The system reset value is factory trimmed.
pub mod dts_rampvalr;
///DTS_ITR1 (rw) register accessor: an alias for `Reg<DTS_ITR1_SPEC>`
pub type DTS_ITR1 = crate::Reg<dts_itr1::DTS_ITR1_SPEC>;
///DTS_ITR1 contains the threshold values for sensor 1.
pub mod dts_itr1;
///DTS_DR (rw) register accessor: an alias for `Reg<DTS_DR_SPEC>`
pub type DTS_DR = crate::Reg<dts_dr::DTS_DR_SPEC>;
///The DTS_DR contains the number of REF_CLK cycles used to compute the FM(T) frequency.
pub mod dts_dr;
///DTS_SR (r) register accessor: an alias for `Reg<DTS_SR_SPEC>`
pub type DTS_SR = crate::Reg<dts_sr::DTS_SR_SPEC>;
///Temperature sensor status register
pub mod dts_sr;
///DTS_ITENR (rw) register accessor: an alias for `Reg<DTS_ITENR_SPEC>`
pub type DTS_ITENR = crate::Reg<dts_itenr::DTS_ITENR_SPEC>;
///Temperature sensor interrupt enable register
pub mod dts_itenr;
///DTS_ICIFR (rw) register accessor: an alias for `Reg<DTS_ICIFR_SPEC>`
pub type DTS_ICIFR = crate::Reg<dts_icifr::DTS_ICIFR_SPEC>;
///DTS_ICIFR is the control register for the interrupt flags.
pub mod dts_icifr;
///DTS_OR (rw) register accessor: an alias for `Reg<DTS_OR_SPEC>`
pub type DTS_OR = crate::Reg<dts_or::DTS_OR_SPEC>;
///The DTS_OR contains general-purpose option bits.
pub mod dts_or;
