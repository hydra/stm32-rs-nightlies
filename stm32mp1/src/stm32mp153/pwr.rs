///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Reset on any system reset. This register provides write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value.
    pub pwr_cr1: PWR_CR1,
    ///0x04 - Reset on any system reset.
    pub pwr_csr1: PWR_CSR1,
    ///0x08 - Not reset by wakeup from Standby mode, Application reset (NRST, IWDG, ...) and VDD POR, but reset only by VSW POR and VSWRST. Access 6 wait states when writing this register. After reset the register is write-protected and the DBP bit in the PWR control register 1 (PWR_CR1) has to be set before it can be written. When DBP is cleared, there is no bus errors generated when writing this register. This register shall not be accessed when the RCC VSWRST register bit in Section10.7.89: RCC Backup Domain Control Register (RCC_BDCR) resets the VSW domain. This register provides Write access security when enabled by TZEN register bit in Section10.7.2: RCC TrustZone Control Register (RCC_TZCR). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.
    pub pwr_cr2: PWR_CR2,
    ///0x0c - Not reset by wakeup from Standby mode and Application reset (such as NRST, IWDG) but only reset by VDD POR. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.
    pub pwr_cr3: PWR_CR3,
    ///0x10 - See individual bits for reset condition. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.
    pub pwr_mpucr: PWR_MPUCR,
    ///0x14 - See individual bits for reset condition. Access 6 wait states when writing this register. This register is always non-secure. When a system reset occurs during the register write cycle the written data is not guaranteed.
    pub pwr_mcucr: PWR_MCUCR,
    _reserved6: [u8; 0x08],
    ///0x20 - Not reset by wakeup from Standby mode, but by any application reset (such as NRST, IWDG). Access 6 wait states when writing this register (when clearing a WKUPF, the AHB write access completes after the WKUPF has cleared). This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access on individual WKUPC\[6:1\], WKUPP\[6:1\]
    ///bits and WKUPPUPD\[6:1\]
    ///bit pairs are discarded when the corresponding WKUPEN\[6:1\]
    ///bit in PWR MPU wakeup enable register (PWR_MPUWKUPENR) is set. No bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.
    pub pwr_wkupcr: PWR_WKUPCR,
    ///0x24 - Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...)
    pub pwr_wkupfr: PWR_WKUPFR,
    ///0x28 - Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...). Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access is discarded and a bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.
    pub pwr_mpuwkupenr: PWR_MPUWKUPENR,
    ///0x2c - Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...) Access 6 wait states when writing this register. When a system reset occurs during the register write cycle the written data is not guaranteed.
    pub pwr_mcuwkupenr: PWR_MCUWKUPENR,
    _reserved10: [u8; 0x03c4],
    ///0x3f4 - PWR IP version register
    pub pwr_ver: PWR_VER,
    ///0x3f8 - PWR IP identification register
    pub pwr_id: PWR_ID,
    ///0x3fc - PWR size ID register
    pub pwr_sid: PWR_SID,
}
///PWR_CR1 (rw) register accessor: an alias for `Reg<PWR_CR1_SPEC>`
pub type PWR_CR1 = crate::Reg<pwr_cr1::PWR_CR1_SPEC>;
///Reset on any system reset. This register provides write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value.
pub mod pwr_cr1;
///PWR_CSR1 (r) register accessor: an alias for `Reg<PWR_CSR1_SPEC>`
pub type PWR_CSR1 = crate::Reg<pwr_csr1::PWR_CSR1_SPEC>;
///Reset on any system reset.
pub mod pwr_csr1;
///PWR_CR2 (rw) register accessor: an alias for `Reg<PWR_CR2_SPEC>`
pub type PWR_CR2 = crate::Reg<pwr_cr2::PWR_CR2_SPEC>;
///Not reset by wakeup from Standby mode, Application reset (NRST, IWDG, ...) and VDD POR, but reset only by VSW POR and VSWRST. Access 6 wait states when writing this register. After reset the register is write-protected and the DBP bit in the PWR control register 1 (PWR_CR1) has to be set before it can be written. When DBP is cleared, there is no bus errors generated when writing this register. This register shall not be accessed when the RCC VSWRST register bit in Section10.7.89: RCC Backup Domain Control Register (RCC_BDCR) resets the VSW domain. This register provides Write access security when enabled by TZEN register bit in Section10.7.2: RCC TrustZone Control Register (RCC_TZCR). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.
pub mod pwr_cr2;
///PWR_CR3 (rw) register accessor: an alias for `Reg<PWR_CR3_SPEC>`
pub type PWR_CR3 = crate::Reg<pwr_cr3::PWR_CR3_SPEC>;
///Not reset by wakeup from Standby mode and Application reset (such as NRST, IWDG) but only reset by VDD POR. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.
pub mod pwr_cr3;
///PWR_MPUCR (rw) register accessor: an alias for `Reg<PWR_MPUCR_SPEC>`
pub type PWR_MPUCR = crate::Reg<pwr_mpucr::PWR_MPUCR_SPEC>;
///See individual bits for reset condition. Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.
pub mod pwr_mpucr;
///PWR_MCUCR (rw) register accessor: an alias for `Reg<PWR_MCUCR_SPEC>`
pub type PWR_MCUCR = crate::Reg<pwr_mcucr::PWR_MCUCR_SPEC>;
///See individual bits for reset condition. Access 6 wait states when writing this register. This register is always non-secure. When a system reset occurs during the register write cycle the written data is not guaranteed.
pub mod pwr_mcucr;
///PWR_WKUPCR (rw) register accessor: an alias for `Reg<PWR_WKUPCR_SPEC>`
pub type PWR_WKUPCR = crate::Reg<pwr_wkupcr::PWR_WKUPCR_SPEC>;
///Not reset by wakeup from Standby mode, but by any application reset (such as NRST, IWDG). Access 6 wait states when writing this register (when clearing a WKUPF, the AHB write access completes after the WKUPF has cleared). This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access on individual WKUPC\[6:1\], WKUPP\[6:1\]
///bits and WKUPPUPD\[6:1\]
///bit pairs are discarded when the corresponding WKUPEN\[6:1\]
///bit in PWR MPU wakeup enable register (PWR_MPUWKUPENR) is set. No bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.
pub mod pwr_wkupcr;
///PWR_WKUPFR (r) register accessor: an alias for `Reg<PWR_WKUPFR_SPEC>`
pub type PWR_WKUPFR = crate::Reg<pwr_wkupfr::PWR_WKUPFR_SPEC>;
///Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...)
pub mod pwr_wkupfr;
///PWR_MPUWKUPENR (rw) register accessor: an alias for `Reg<PWR_MPUWKUPENR_SPEC>`
pub type PWR_MPUWKUPENR = crate::Reg<pwr_mpuwkupenr::PWR_MPUWKUPENR_SPEC>;
///Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...). Access 6 wait states when writing this register. This register provides Write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access is discarded and a bus error is generated. Secure and non-secure read accesses are granted and return the register value. When a system reset occurs during the register write cycle the written data is not guaranteed.
pub mod pwr_mpuwkupenr;
///PWR_MCUWKUPENR (rw) register accessor: an alias for `Reg<PWR_MCUWKUPENR_SPEC>`
pub type PWR_MCUWKUPENR = crate::Reg<pwr_mcuwkupenr::PWR_MCUWKUPENR_SPEC>;
///Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...) Access 6 wait states when writing this register. When a system reset occurs during the register write cycle the written data is not guaranteed.
pub mod pwr_mcuwkupenr;
///PWR_VER (r) register accessor: an alias for `Reg<PWR_VER_SPEC>`
pub type PWR_VER = crate::Reg<pwr_ver::PWR_VER_SPEC>;
///PWR IP version register
pub mod pwr_ver;
///PWR_ID (r) register accessor: an alias for `Reg<PWR_ID_SPEC>`
pub type PWR_ID = crate::Reg<pwr_id::PWR_ID_SPEC>;
///PWR IP identification register
pub mod pwr_id;
///PWR_SID (r) register accessor: an alias for `Reg<PWR_SID_SPEC>`
pub type PWR_SID = crate::Reg<pwr_sid::PWR_SID_SPEC>;
///PWR size ID register
pub mod pwr_sid;
