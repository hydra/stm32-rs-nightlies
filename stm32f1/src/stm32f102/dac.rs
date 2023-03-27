///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Control register (DAC_CR)
    pub cr: CR,
    ///0x04 - DAC software trigger register (DAC_SWTRIGR)
    pub swtrigr: SWTRIGR,
    ///0x08 - DAC channel1 12-bit right-aligned data holding register(DAC_DHR12R1)
    pub dhr12r1: DHR12R1,
    ///0x0c - DAC channel1 12-bit left aligned data holding register (DAC_DHR12L1)
    pub dhr12l1: DHR12L1,
    ///0x10 - DAC channel1 8-bit right aligned data holding register (DAC_DHR8R1)
    pub dhr8r1: DHR8R1,
    ///0x14 - DAC channel2 12-bit right aligned data holding register (DAC_DHR12R2)
    pub dhr12r2: DHR12R2,
    ///0x18 - DAC channel2 12-bit left aligned data holding register (DAC_DHR12L2)
    pub dhr12l2: DHR12L2,
    ///0x1c - DAC channel2 8-bit right-aligned data holding register (DAC_DHR8R2)
    pub dhr8r2: DHR8R2,
    ///0x20 - Dual DAC 12-bit right-aligned data holding register (DAC_DHR12RD), Bits 31:28 Reserved, Bits 15:12 Reserved
    pub dhr12rd: DHR12RD,
    ///0x24 - DUAL DAC 12-bit left aligned data holding register (DAC_DHR12LD), Bits 19:16 Reserved, Bits 3:0 Reserved
    pub dhr12ld: DHR12LD,
    ///0x28 - DUAL DAC 8-bit right aligned data holding register (DAC_DHR8RD), Bits 31:16 Reserved
    pub dhr8rd: DHR8RD,
    ///0x2c - DAC channel1 data output register (DAC_DOR1)
    pub dor1: DOR1,
    ///0x30 - DAC channel2 data output register (DAC_DOR2)
    pub dor2: DOR2,
    ///0x34 - DAC status register
    pub sr: SR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Control register (DAC_CR)
pub mod cr;
///SWTRIGR (w) register accessor: an alias for `Reg<SWTRIGR_SPEC>`
pub type SWTRIGR = crate::Reg<swtrigr::SWTRIGR_SPEC>;
///DAC software trigger register (DAC_SWTRIGR)
pub mod swtrigr;
///DHR12R1 (rw) register accessor: an alias for `Reg<DHR12R1_SPEC>`
pub type DHR12R1 = crate::Reg<dhr12r1::DHR12R1_SPEC>;
///DAC channel1 12-bit right-aligned data holding register(DAC_DHR12R1)
pub mod dhr12r1;
///DHR12L1 (rw) register accessor: an alias for `Reg<DHR12L1_SPEC>`
pub type DHR12L1 = crate::Reg<dhr12l1::DHR12L1_SPEC>;
///DAC channel1 12-bit left aligned data holding register (DAC_DHR12L1)
pub mod dhr12l1;
///DHR8R1 (rw) register accessor: an alias for `Reg<DHR8R1_SPEC>`
pub type DHR8R1 = crate::Reg<dhr8r1::DHR8R1_SPEC>;
///DAC channel1 8-bit right aligned data holding register (DAC_DHR8R1)
pub mod dhr8r1;
///DHR12R2 (rw) register accessor: an alias for `Reg<DHR12R2_SPEC>`
pub type DHR12R2 = crate::Reg<dhr12r2::DHR12R2_SPEC>;
///DAC channel2 12-bit right aligned data holding register (DAC_DHR12R2)
pub mod dhr12r2;
///DHR12L2 (rw) register accessor: an alias for `Reg<DHR12L2_SPEC>`
pub type DHR12L2 = crate::Reg<dhr12l2::DHR12L2_SPEC>;
///DAC channel2 12-bit left aligned data holding register (DAC_DHR12L2)
pub mod dhr12l2;
///DHR8R2 (rw) register accessor: an alias for `Reg<DHR8R2_SPEC>`
pub type DHR8R2 = crate::Reg<dhr8r2::DHR8R2_SPEC>;
///DAC channel2 8-bit right-aligned data holding register (DAC_DHR8R2)
pub mod dhr8r2;
///DHR12RD (rw) register accessor: an alias for `Reg<DHR12RD_SPEC>`
pub type DHR12RD = crate::Reg<dhr12rd::DHR12RD_SPEC>;
///Dual DAC 12-bit right-aligned data holding register (DAC_DHR12RD), Bits 31:28 Reserved, Bits 15:12 Reserved
pub mod dhr12rd;
///DHR12LD (rw) register accessor: an alias for `Reg<DHR12LD_SPEC>`
pub type DHR12LD = crate::Reg<dhr12ld::DHR12LD_SPEC>;
///DUAL DAC 12-bit left aligned data holding register (DAC_DHR12LD), Bits 19:16 Reserved, Bits 3:0 Reserved
pub mod dhr12ld;
///DHR8RD (rw) register accessor: an alias for `Reg<DHR8RD_SPEC>`
pub type DHR8RD = crate::Reg<dhr8rd::DHR8RD_SPEC>;
///DUAL DAC 8-bit right aligned data holding register (DAC_DHR8RD), Bits 31:16 Reserved
pub mod dhr8rd;
///DOR1 (r) register accessor: an alias for `Reg<DOR1_SPEC>`
pub type DOR1 = crate::Reg<dor1::DOR1_SPEC>;
///DAC channel1 data output register (DAC_DOR1)
pub mod dor1;
///DOR2 (r) register accessor: an alias for `Reg<DOR2_SPEC>`
pub type DOR2 = crate::Reg<dor2::DOR2_SPEC>;
///DAC channel2 data output register (DAC_DOR2)
pub mod dor2;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///DAC status register
pub mod sr;
