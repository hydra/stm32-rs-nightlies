///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - DAC control register
    pub dac_cr: DAC_CR,
    ///0x04 - DAC software trigger register
    pub dac_swtrgr: DAC_SWTRGR,
    ///0x08 - DAC channel1 12-bit right-aligned data holding register
    pub dac_dhr12r1: DAC_DHR12R1,
    ///0x0c - DAC channel1 12-bit left aligned data holding register
    pub dac_dhr12l1: DAC_DHR12L1,
    ///0x10 - DAC channel1 8-bit right aligned data holding register
    pub dac_dhr8r1: DAC_DHR8R1,
    ///0x14 - DAC channel2 12-bit right aligned data holding register
    pub dac_dhr12r2: DAC_DHR12R2,
    ///0x18 - DAC channel2 12-bit left aligned data holding register
    pub dac_dhr12l2: DAC_DHR12L2,
    ///0x1c - DAC channel2 8-bit right-aligned data holding register
    pub dac_dhr8r2: DAC_DHR8R2,
    ///0x20 - Dual DAC 12-bit right-aligned data holding register
    pub dac_dhr12rd: DAC_DHR12RD,
    ///0x24 - DUAL DAC 12-bit left aligned data holding register
    pub dac_dhr12ld: DAC_DHR12LD,
    ///0x28 - DUAL DAC 8-bit right aligned data holding register
    pub dac_dhr8rd: DAC_DHR8RD,
    ///0x2c - DAC channel1 data output register
    pub dac_dor1: DAC_DOR1,
    ///0x30 - DAC channel2 data output register
    pub dac_dor2: DAC_DOR2,
    ///0x34 - DAC status register
    pub dac_sr: DAC_SR,
    ///0x38 - DAC calibration control register
    pub dac_ccr: DAC_CCR,
    ///0x3c - DAC mode control register
    pub dac_mcr: DAC_MCR,
    ///0x40 - DAC Sample and Hold sample time register 1
    pub dac_shsr1: DAC_SHSR1,
    ///0x44 - DAC Sample and Hold sample time register 2
    pub dac_shsr2: DAC_SHSR2,
    ///0x48 - DAC Sample and Hold hold time register
    pub dac_shhr: DAC_SHHR,
    ///0x4c - DAC Sample and Hold refresh time register
    pub dac_shrr: DAC_SHRR,
    _reserved20: [u8; 0x08],
    ///0x58 - Sawtooth register
    pub dac_str1: DAC_STR1,
    ///0x5c - Sawtooth register
    pub dac_str2: DAC_STR2,
    ///0x60 - Sawtooth Mode register
    pub dac_stmodr: DAC_STMODR,
}
///DAC_CR (rw) register accessor: an alias for `Reg<DAC_CR_SPEC>`
pub type DAC_CR = crate::Reg<dac_cr::DAC_CR_SPEC>;
///DAC control register
pub mod dac_cr;
///DAC_SWTRGR (w) register accessor: an alias for `Reg<DAC_SWTRGR_SPEC>`
pub type DAC_SWTRGR = crate::Reg<dac_swtrgr::DAC_SWTRGR_SPEC>;
///DAC software trigger register
pub mod dac_swtrgr;
///DAC_DHR12R1 (rw) register accessor: an alias for `Reg<DAC_DHR12R1_SPEC>`
pub type DAC_DHR12R1 = crate::Reg<dac_dhr12r1::DAC_DHR12R1_SPEC>;
///DAC channel1 12-bit right-aligned data holding register
pub mod dac_dhr12r1;
///DAC_DHR12L1 (rw) register accessor: an alias for `Reg<DAC_DHR12L1_SPEC>`
pub type DAC_DHR12L1 = crate::Reg<dac_dhr12l1::DAC_DHR12L1_SPEC>;
///DAC channel1 12-bit left aligned data holding register
pub mod dac_dhr12l1;
///DAC_DHR8R1 (rw) register accessor: an alias for `Reg<DAC_DHR8R1_SPEC>`
pub type DAC_DHR8R1 = crate::Reg<dac_dhr8r1::DAC_DHR8R1_SPEC>;
///DAC channel1 8-bit right aligned data holding register
pub mod dac_dhr8r1;
///DAC_DHR12R2 (rw) register accessor: an alias for `Reg<DAC_DHR12R2_SPEC>`
pub type DAC_DHR12R2 = crate::Reg<dac_dhr12r2::DAC_DHR12R2_SPEC>;
///DAC channel2 12-bit right aligned data holding register
pub mod dac_dhr12r2;
///DAC_DHR12L2 (rw) register accessor: an alias for `Reg<DAC_DHR12L2_SPEC>`
pub type DAC_DHR12L2 = crate::Reg<dac_dhr12l2::DAC_DHR12L2_SPEC>;
///DAC channel2 12-bit left aligned data holding register
pub mod dac_dhr12l2;
///DAC_DHR8R2 (rw) register accessor: an alias for `Reg<DAC_DHR8R2_SPEC>`
pub type DAC_DHR8R2 = crate::Reg<dac_dhr8r2::DAC_DHR8R2_SPEC>;
///DAC channel2 8-bit right-aligned data holding register
pub mod dac_dhr8r2;
///DAC_DHR12RD (rw) register accessor: an alias for `Reg<DAC_DHR12RD_SPEC>`
pub type DAC_DHR12RD = crate::Reg<dac_dhr12rd::DAC_DHR12RD_SPEC>;
///Dual DAC 12-bit right-aligned data holding register
pub mod dac_dhr12rd;
///DAC_DHR12LD (rw) register accessor: an alias for `Reg<DAC_DHR12LD_SPEC>`
pub type DAC_DHR12LD = crate::Reg<dac_dhr12ld::DAC_DHR12LD_SPEC>;
///DUAL DAC 12-bit left aligned data holding register
pub mod dac_dhr12ld;
///DAC_DHR8RD (rw) register accessor: an alias for `Reg<DAC_DHR8RD_SPEC>`
pub type DAC_DHR8RD = crate::Reg<dac_dhr8rd::DAC_DHR8RD_SPEC>;
///DUAL DAC 8-bit right aligned data holding register
pub mod dac_dhr8rd;
///DAC_DOR1 (r) register accessor: an alias for `Reg<DAC_DOR1_SPEC>`
pub type DAC_DOR1 = crate::Reg<dac_dor1::DAC_DOR1_SPEC>;
///DAC channel1 data output register
pub mod dac_dor1;
///DAC_DOR2 (r) register accessor: an alias for `Reg<DAC_DOR2_SPEC>`
pub type DAC_DOR2 = crate::Reg<dac_dor2::DAC_DOR2_SPEC>;
///DAC channel2 data output register
pub mod dac_dor2;
///DAC_SR (rw) register accessor: an alias for `Reg<DAC_SR_SPEC>`
pub type DAC_SR = crate::Reg<dac_sr::DAC_SR_SPEC>;
///DAC status register
pub mod dac_sr;
///DAC_CCR (rw) register accessor: an alias for `Reg<DAC_CCR_SPEC>`
pub type DAC_CCR = crate::Reg<dac_ccr::DAC_CCR_SPEC>;
///DAC calibration control register
pub mod dac_ccr;
///DAC_MCR (rw) register accessor: an alias for `Reg<DAC_MCR_SPEC>`
pub type DAC_MCR = crate::Reg<dac_mcr::DAC_MCR_SPEC>;
///DAC mode control register
pub mod dac_mcr;
///DAC_SHSR1 (rw) register accessor: an alias for `Reg<DAC_SHSR1_SPEC>`
pub type DAC_SHSR1 = crate::Reg<dac_shsr1::DAC_SHSR1_SPEC>;
///DAC Sample and Hold sample time register 1
pub mod dac_shsr1;
///DAC_SHSR2 (rw) register accessor: an alias for `Reg<DAC_SHSR2_SPEC>`
pub type DAC_SHSR2 = crate::Reg<dac_shsr2::DAC_SHSR2_SPEC>;
///DAC Sample and Hold sample time register 2
pub mod dac_shsr2;
///DAC_SHHR (rw) register accessor: an alias for `Reg<DAC_SHHR_SPEC>`
pub type DAC_SHHR = crate::Reg<dac_shhr::DAC_SHHR_SPEC>;
///DAC Sample and Hold hold time register
pub mod dac_shhr;
///DAC_SHRR (rw) register accessor: an alias for `Reg<DAC_SHRR_SPEC>`
pub type DAC_SHRR = crate::Reg<dac_shrr::DAC_SHRR_SPEC>;
///DAC Sample and Hold refresh time register
pub mod dac_shrr;
///DAC_STR1 (rw) register accessor: an alias for `Reg<DAC_STR1_SPEC>`
pub type DAC_STR1 = crate::Reg<dac_str1::DAC_STR1_SPEC>;
///Sawtooth register
pub mod dac_str1;
///DAC_STR2 (rw) register accessor: an alias for `Reg<DAC_STR2_SPEC>`
pub type DAC_STR2 = crate::Reg<dac_str2::DAC_STR2_SPEC>;
///Sawtooth register
pub mod dac_str2;
///DAC_STMODR (rw) register accessor: an alias for `Reg<DAC_STMODR_SPEC>`
pub type DAC_STMODR = crate::Reg<dac_stmodr::DAC_STMODR_SPEC>;
///Sawtooth Mode register
pub mod dac_stmodr;
