///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: CR,
    ///0x04 - software trigger register
    pub swtrigr: SWTRIGR,
    ///0x08 - channel1 12-bit right-aligned data holding register
    pub dhr12r1: DHR12R1,
    ///0x0c - channel1 12-bit left aligned data holding register
    pub dhr12l1: DHR12L1,
    ///0x10 - channel1 8-bit right aligned data holding register
    pub dhr8r1: DHR8R1,
    ///0x14 - DAC channel2 12-bit right-aligned data holding register
    pub dhr12r2: DHR12R2,
    ///0x18 - DAC channel2 12-bit left-aligned data holding register
    pub dhr12l2: DHR12L2,
    ///0x1c - DAC channel2 8-bit right-aligned data holding register
    pub dhr8r2: DHR8R2,
    ///0x20 - DHR12RD
    pub dhr12rd: DHR12RD,
    ///0x24 - Dual DAC 12-bit left-aligned data holding register
    pub dhr12ld: DHR12LD,
    ///0x28 - Dual DAC 8-bit right-aligned data holding register
    pub dhr8rd: DHR8RD,
    ///0x2c - channel1 data output register
    pub dor1: DOR1,
    ///0x30 - DAC channel2 data output register
    pub dor2: DOR2,
    ///0x34 - status register
    pub sr: SR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///control register
pub mod cr;
///SWTRIGR (w) register accessor: an alias for `Reg<SWTRIGR_SPEC>`
pub type SWTRIGR = crate::Reg<swtrigr::SWTRIGR_SPEC>;
///software trigger register
pub mod swtrigr;
///DHR12R1 (rw) register accessor: an alias for `Reg<DHR12R1_SPEC>`
pub type DHR12R1 = crate::Reg<dhr12r1::DHR12R1_SPEC>;
///channel1 12-bit right-aligned data holding register
pub mod dhr12r1;
///DHR12L1 (rw) register accessor: an alias for `Reg<DHR12L1_SPEC>`
pub type DHR12L1 = crate::Reg<dhr12l1::DHR12L1_SPEC>;
///channel1 12-bit left aligned data holding register
pub mod dhr12l1;
///DHR8R1 (rw) register accessor: an alias for `Reg<DHR8R1_SPEC>`
pub type DHR8R1 = crate::Reg<dhr8r1::DHR8R1_SPEC>;
///channel1 8-bit right aligned data holding register
pub mod dhr8r1;
///DOR1 (r) register accessor: an alias for `Reg<DOR1_SPEC>`
pub type DOR1 = crate::Reg<dor1::DOR1_SPEC>;
///channel1 data output register
pub mod dor1;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///status register
pub mod sr;
///DHR12R2 (rw) register accessor: an alias for `Reg<DHR12R2_SPEC>`
pub type DHR12R2 = crate::Reg<dhr12r2::DHR12R2_SPEC>;
///DAC channel2 12-bit right-aligned data holding register
pub mod dhr12r2;
///DHR12L2 (rw) register accessor: an alias for `Reg<DHR12L2_SPEC>`
pub type DHR12L2 = crate::Reg<dhr12l2::DHR12L2_SPEC>;
///DAC channel2 12-bit left-aligned data holding register
pub mod dhr12l2;
///DHR8R2 (rw) register accessor: an alias for `Reg<DHR8R2_SPEC>`
pub type DHR8R2 = crate::Reg<dhr8r2::DHR8R2_SPEC>;
///DAC channel2 8-bit right-aligned data holding register
pub mod dhr8r2;
///DHR12RD (rw) register accessor: an alias for `Reg<DHR12RD_SPEC>`
pub type DHR12RD = crate::Reg<dhr12rd::DHR12RD_SPEC>;
///DHR12RD
pub mod dhr12rd;
///DHR12LD (rw) register accessor: an alias for `Reg<DHR12LD_SPEC>`
pub type DHR12LD = crate::Reg<dhr12ld::DHR12LD_SPEC>;
///Dual DAC 12-bit left-aligned data holding register
pub mod dhr12ld;
///DHR8RD (rw) register accessor: an alias for `Reg<DHR8RD_SPEC>`
pub type DHR8RD = crate::Reg<dhr8rd::DHR8RD_SPEC>;
///Dual DAC 8-bit right-aligned data holding register
pub mod dhr8rd;
///DOR2 (r) register accessor: an alias for `Reg<DOR2_SPEC>`
pub type DOR2 = crate::Reg<dor2::DOR2_SPEC>;
///DAC channel2 data output register
pub mod dor2;
