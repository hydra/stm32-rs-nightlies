///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - PWR control register 1
    pub cr1: CR1,
    ///0x04 - PWR control status register 1
    pub csr1: CSR1,
    ///0x08 - This register is not reset by wakeup from Standby mode, RESET signal and VDD POR. It is only reset by VSW POR and VSWRST reset. This register shall not be accessed when VSWRST bit in RCC_BDCR register resets the VSW domain.After reset, PWR_CR2 register is write-protected. Prior to modifying its content, the DBP bit in PWR_CR1 register must be set to disable the write protection.
    pub cr2: CR2,
    ///0x0c - Reset only by POR only, not reset by wakeup from Standby mode and RESET pad. The lower byte of this register is written once after POR and shall be written before changing VOS level or ck_sys clock frequency. No limitation applies to the upper bytes.Programming data corresponding to an invalid combination of SDLEVEL, SDEXTHP, SDEN, LDOEN and BYPASS bits (see Table9) will be ignored: data will not be written, the written-once mechanism will lock the register and any further write access will be ignored. The default supply configuration will be kept and the ACTVOSRDY bit in PWR control status register 1 (PWR_CSR1) will go on indicating invalid voltage levels. The system shall be power cycled before writing a new value.
    pub cr3: CR3,
    ///0x10 - This register allows controlling CPU1 power.
    pub cpucr: CPUCR,
    _reserved5: [u8; 0x04],
    ///0x18 - This register allows controlling D3 domain power.Following reset VOSRDY will be read 1 by software
    pub srdcr: SRDCR,
    _reserved6: [u8; 0x04],
    ///0x20 - reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared).
    pub wkupcr: WKUPCR,
    ///0x24 - reset only by system reset, not reset by wakeup from Standby mode
    pub wkupfr: WKUPFR,
    ///0x28 - Reset only by system reset, not reset by wakeup from Standby mode
    pub wkupepr: WKUPEPR,
}
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///PWR control register 1
pub mod cr1;
///CSR1 (r) register accessor: an alias for `Reg<CSR1_SPEC>`
pub type CSR1 = crate::Reg<csr1::CSR1_SPEC>;
///PWR control status register 1
pub mod csr1;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///This register is not reset by wakeup from Standby mode, RESET signal and VDD POR. It is only reset by VSW POR and VSWRST reset. This register shall not be accessed when VSWRST bit in RCC_BDCR register resets the VSW domain.After reset, PWR_CR2 register is write-protected. Prior to modifying its content, the DBP bit in PWR_CR1 register must be set to disable the write protection.
pub mod cr2;
///CR3 (rw) register accessor: an alias for `Reg<CR3_SPEC>`
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
///Reset only by POR only, not reset by wakeup from Standby mode and RESET pad. The lower byte of this register is written once after POR and shall be written before changing VOS level or ck_sys clock frequency. No limitation applies to the upper bytes.Programming data corresponding to an invalid combination of SDLEVEL, SDEXTHP, SDEN, LDOEN and BYPASS bits (see Table9) will be ignored: data will not be written, the written-once mechanism will lock the register and any further write access will be ignored. The default supply configuration will be kept and the ACTVOSRDY bit in PWR control status register 1 (PWR_CSR1) will go on indicating invalid voltage levels. The system shall be power cycled before writing a new value.
pub mod cr3;
///CPUCR (rw) register accessor: an alias for `Reg<CPUCR_SPEC>`
pub type CPUCR = crate::Reg<cpucr::CPUCR_SPEC>;
///This register allows controlling CPU1 power.
pub mod cpucr;
///SRDCR (rw) register accessor: an alias for `Reg<SRDCR_SPEC>`
pub type SRDCR = crate::Reg<srdcr::SRDCR_SPEC>;
///This register allows controlling D3 domain power.Following reset VOSRDY will be read 1 by software
pub mod srdcr;
///WKUPCR (rw) register accessor: an alias for `Reg<WKUPCR_SPEC>`
pub type WKUPCR = crate::Reg<wkupcr::WKUPCR_SPEC>;
///reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared).
pub mod wkupcr;
///WKUPFR (rw) register accessor: an alias for `Reg<WKUPFR_SPEC>`
pub type WKUPFR = crate::Reg<wkupfr::WKUPFR_SPEC>;
///reset only by system reset, not reset by wakeup from Standby mode
pub mod wkupfr;
///WKUPEPR (rw) register accessor: an alias for `Reg<WKUPEPR_SPEC>`
pub type WKUPEPR = crate::Reg<wkupepr::WKUPEPR_SPEC>;
///Reset only by system reset, not reset by wakeup from Standby mode
pub mod wkupepr;
