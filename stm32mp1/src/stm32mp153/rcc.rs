///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - This register is used to switch the RCC into secure mode. This register can only be accessed in secure mode.
    pub rcc_tzcr: RCC_TZCR,
    _reserved1: [u8; 0x08],
    ///0x0c - This register is used to control the oscillators.Writing to this register has no effect, writing will set the corresponding bits. Reading will give the effective values of each bit.If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub rcc_ocensetr: RCC_OCENSETR,
    ///0x10 - This register is used to control the oscillators.Writing to this register has no effect, writing will clear the corresponding bits. Reading will give the effective values of the enable bits.If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub rcc_ocenclrr: RCC_OCENCLRR,
    _reserved3: [u8; 0x04],
    ///0x18 - This register is used to configure the HSI. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub rcc_hsicfgr: RCC_HSICFGR,
    ///0x1c - This register is used to fine-tune the CSI frequency. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See The clock restore sequence description for details.
    pub rcc_csicfgr: RCC_CSICFGR,
    ///0x20 - This register is used to select the clock source for the MPU. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub rcc_mpckselr: RCC_MPCKSELR,
    ///0x24 - This register is used to select the clock source for the AXI sub-system. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub rcc_assckselr: RCC_ASSCKSELR,
    ///0x28 - This register is used to select the reference clock for PLL1 and PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub rcc_rck12selr: RCC_RCK12SELR,
    ///0x2c - This register is used to control the MPU clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
    pub rcc_mpckdivr: RCC_MPCKDIVR,
    ///0x30 - This register is used to control the AXI Matrix clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
    pub rcc_axidivr: RCC_AXIDIVR,
    _reserved10: [u8; 0x08],
    ///0x3c - This register is used to control the APB4 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
    pub rcc_apb4divr: RCC_APB4DIVR,
    ///0x40 - This register is used to control the APB5 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
    pub rcc_apb5divr: RCC_APB5DIVR,
    ///0x44 - This register is used to divide the HSE clock for RTC. If TZEN = , this register can only be modified in secure mode.
    pub rcc_rtcdivr: RCC_RTCDIVR,
    ///0x48 - This register is used to select the clock source for the MCU sub-system, including the MCU itself. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub rcc_mssckselr: RCC_MSSCKSELR,
    _reserved14: [u8; 0x34],
    ///0x80 - This register is used to control the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub rcc_pll1cr: RCC_PLL1CR,
    ///0x84 - This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub rcc_pll1cfgr1: RCC_PLL1CFGR1,
    ///0x88 - This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub rcc_pll1cfgr2: RCC_PLL1CFGR2,
    ///0x8c - This register is used to fine-tune the frequency of the PLL1 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub rcc_pll1fracr: RCC_PLL1FRACR,
    ///0x90 - This register is used to configure the PLL1.It is not recommended to change the content of this register when the PLL1 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub rcc_pll1csgr: RCC_PLL1CSGR,
    ///0x94 - This register is used to control the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub rcc_pll2cr: RCC_PLL2CR,
    ///0x98 - This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub rcc_pll2cfgr1: RCC_PLL2CFGR1,
    ///0x9c - This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub rcc_pll2cfgr2: RCC_PLL2CFGR2,
    ///0xa0 - This register is used to fine-tune the frequency of the PLL2 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub rcc_pll2fracr: RCC_PLL2FRACR,
    ///0xa4 - This register is used to configure the PLL2. It is not recommended to change the content of this register when the PLL2 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
    pub rcc_pll2csgr: RCC_PLL2CSGR,
    _reserved24: [u8; 0x18],
    ///0xc0 - This register is used to control the selection of the kernel clock for the I2C4 and I2C6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
    pub rcc_i2c46ckselr: RCC_I2C46CKSELR,
    ///0xc4 - This register is used to control the selection of the kernel clock for the SPI6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
    pub rcc_spi6ckselr: RCC_SPI6CKSELR,
    ///0xc8 - This register is used to control the selection of the kernel clock for the USART1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
    pub rcc_uart1ckselr: RCC_UART1CKSELR,
    ///0xcc - This register is used to control the selection of the kernel clock for the RNG1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
    pub rcc_rng1ckselr: RCC_RNG1CKSELR,
    ///0xd0 - This register is used to select an oscillator source as kernel clock for the per_ck clock. The per_ck clock is distributed to several peripherals. Refer to Section: Clock enabling delays.
    pub rcc_cperckselr: RCC_CPERCKSELR,
    ///0xd4 - This register is used to select the peripheral clock for the STGEN block. Note that this clock is used to provide a time reference for the application. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
    pub rcc_stgenckselr: RCC_STGENCKSELR,
    ///0xd8 - This register is used to control the DDR interface, including the DDRC and DDRPHYC. If TZEN = , this register can only be modified in secure mode.
    pub rcc_ddritfcr: RCC_DDRITFCR,
    _reserved31: [u8; 0x24],
    ///0x100 - This register is used to control the HOLD boot function when the system exits from Standby. Refer to Section: MCU HOLD_BOOT after processor reset. This register is reset when a system reset occurs, but not when the circuit exits from Standby (app_rst reset).If TZEN = , this register can only be modified in secure mode. This register can only be accessed by the MPU.
    pub rcc_mp_bootcr: RCC_MP_BOOTCR,
    ///0x104 - Writing has no effect, reading will return the values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode.
    pub rcc_mp_sreqsetr: RCC_MP_SREQSETR,
    ///0x108 - Writing has no effect, reading will return the effective values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode.
    pub rcc_mp_sreqclrr: RCC_MP_SREQCLRR,
    ///0x10c - The register contains global control bits. If TZEN = , this register can only be modified in secure mode.
    pub rcc_mp_gcr: RCC_MP_GCR,
    ///0x110 - This register is used to control the behavior of the warm reset. If TZEN = , this register can only be modified in secure mode.
    pub rcc_mp_aprstcr: RCC_MP_APRSTCR,
    ///0x114 - This register provides a status of the RDCTL. If TZEN = , this register can only be modified in secure mode.
    pub rcc_mp_aprstsr: RCC_MP_APRSTSR,
    _reserved37: [u8; 0x28],
    ///0x140 - This register is used to control the LSE function. Wait states are inserted in case of successive write accesses to this register. The number of wait states may be up to 7 cycles of AHB4 clock.After a system reset, the register RCC_BDCR is write-protected. In order to modify this register, the DBP bit in the PWR control register 1 (PWR_CR1) has to be set to . Bits of RCC_BDCR register are only reset after a backup domain reset: nreset_vsw (see Section10.3.6: Backup domain reset). Any other internal or external reset will not have any effect on these bits.This register is located into the VSW domain. If TZEN = , this register can only be modified in secure mode.
    pub rcc_bdcr: RCC_BDCR,
    ///0x144 - This register is used to control the minimum NRST active duration and LSI function.0 to 7 wait states are inserted for word, half-word and byte accesses. Wait states are inserted in case of successive accesses to this register.This register is reset by the por_rst reset, and it is located into the VDD domain. If TZEN = , this register can only be modified in secure mode.
    pub rcc_rdlsicr: RCC_RDLSICR,
    _reserved39: [u8; 0x38],
    ///0x180 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral.
    pub rcc_apb4rstsetr: RCC_APB4RSTSETR,
    ///0x184 - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral.
    pub rcc_apb4rstclrr: RCC_APB4RSTCLRR,
    ///0x188 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
    pub rcc_apb5rstsetr: RCC_APB5RSTSETR,
    ///0x18c - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
    pub rcc_apb5rstclrr: RCC_APB5RSTCLRR,
    ///0x190 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
    pub rcc_ahb5rstsetr: RCC_AHB5RSTSETR,
    ///0x194 - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
    pub rcc_ahb5rstclrr: RCC_AHB5RSTCLRR,
    ///0x198 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral.
    pub rcc_ahb6rstsetr: RCC_AHB6RSTSETR,
    ///0x19c - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral.
    pub rcc_ahb6rstclrr: RCC_AHB6RSTCLRR,
    ///0x1a0 - This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
    pub rcc_tzahb6rstsetr: RCC_TZAHB6RSTSETR,
    ///0x1a4 - This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
    pub rcc_tzahb6rstclrr: RCC_TZAHB6RSTCLRR,
    _reserved49: [u8; 0x58],
    ///0x200 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
    pub rcc_mp_apb4ensetr: RCC_MP_APB4ENSETR,
    ///0x204 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
    pub rcc_mp_apb4enclrr: RCC_MP_APB4ENCLRR,
    ///0x208 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
    pub rcc_mp_apb5ensetr: RCC_MP_APB5ENSETR,
    ///0x20c - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
    pub rcc_mp_apb5enclrr: RCC_MP_APB5ENCLRR,
    ///0x210 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
    pub rcc_mp_ahb5ensetr: RCC_MP_AHB5ENSETR,
    ///0x214 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
    pub rcc_mp_ahb5enclrr: RCC_MP_AHB5ENCLRR,
    ///0x218 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
    pub rcc_mp_ahb6ensetr: RCC_MP_AHB6ENSETR,
    ///0x21c - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
    pub rcc_mp_ahb6enclrr: RCC_MP_AHB6ENCLRR,
    ///0x220 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
    pub rcc_mp_tzahb6ensetr: RCC_MP_TZAHB6ENSETR,
    ///0x224 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
    pub rcc_mp_tzahb6enclrr: RCC_MP_TZAHB6ENCLRR,
    _reserved59: [u8; 0x58],
    ///0x280 - This register is used to set the peripheral clock enable bit
    pub rcc_mc_apb4ensetr: RCC_MC_APB4ENSETR,
    ///0x284 - This register is used to clear the peripheral clock enable bit
    pub rcc_mc_apb4enclrr: RCC_MC_APB4ENCLRR,
    ///0x288 - This register is used to set the peripheral clock enable bit
    pub rcc_mc_apb5ensetr: RCC_MC_APB5ENSETR,
    ///0x28c - This register is used to clear the peripheral clock enable bit
    pub rcc_mc_apb5enclrr: RCC_MC_APB5ENCLRR,
    ///0x290 - This register is used to set the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode.
    pub rcc_mc_ahb5ensetr: RCC_MC_AHB5ENSETR,
    ///0x294 - This register is used to clear the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode.
    pub rcc_mc_ahb5enclrr: RCC_MC_AHB5ENCLRR,
    ///0x298 - This register is used to set the peripheral clock enable bit
    pub rcc_mc_ahb6ensetr: RCC_MC_AHB6ENSETR,
    ///0x29c - This register is used to clear the peripheral clock enable bit
    pub rcc_mc_ahb6enclrr: RCC_MC_AHB6ENCLRR,
    _reserved67: [u8; 0x60],
    ///0x300 - This register is used by the MCU in order to clear the PERxLPEN bits
    pub rcc_mp_apb4lpensetr: RCC_MP_APB4LPENSETR,
    ///0x304 - This register is used by the MCU
    pub rcc_mp_apb4lpenclrr: RCC_MP_APB4LPENCLRR,
    ///0x308 - This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
    pub rcc_mp_apb5lpensetr: RCC_MP_APB5LPENSETR,
    ///0x30c - This register is used by the Mpu.
    pub rcc_mp_apb5lpenclrr: RCC_MP_APB5LPENCLRR,
    ///0x310 - This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
    pub rcc_mp_ahb5lpensetr: RCC_MP_AHB5LPENSETR,
    ///0x314 - This register is used by the MCU
    pub rcc_mp_ahb5lpenclrr: RCC_MP_AHB5LPENCLRR,
    ///0x318 - This register is used by the MCU in order to clear the PERxLPEN bits
    pub rcc_mp_ahb6lpensetr: RCC_MP_AHB6LPENSETR,
    ///0x31c - This register is used by the MCU in order to clear the PERxLPEN bits
    pub rcc_mp_ahb6lpenclrr: RCC_MP_AHB6LPENCLRR,
    ///0x320 - This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
    pub rcc_mp_tzahb6lpensetr: RCC_MP_TZAHB6LPENSETR,
    ///0x324 - This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
    pub rcc_mp_tzahb6lpenclrr: RCC_MP_TZAHB6LPENCLRR,
    _reserved77: [u8; 0x58],
    ///0x380 - This register is used by the MCU in order to set the PERxLPEN bit.
    pub rcc_mc_apb4lpensetr: RCC_MC_APB4LPENSETR,
    ///0x384 - This register is used by the MCU in order to clear the PERxLPEN bit
    pub rcc_mc_apb4lpenclrr: RCC_MC_APB4LPENCLRR,
    ///0x388 - This register is used by the MCU in order to set the PERxLPEN bit.
    pub rcc_mc_apb5lpensetr: RCC_MC_APB5LPENSETR,
    ///0x38c - This register is used by the MCU in order to clear the PERxLPEN bit
    pub rcc_mc_apb5lpenclrr: RCC_MC_APB5LPENCLRR,
    ///0x390 - This register is used by the MCU in order to set the PERxLPEN bit. If TZEN = , this register can only be modified in secure mode.
    pub rcc_mc_ahb5lpensetr: RCC_MC_AHB5LPENSETR,
    ///0x394 - This register is used by the MCU in order to clear the PERxLPEN bit If TZEN = , this register can only be modified in secure mode.
    pub rcc_mc_ahb5lpenclrr: RCC_MC_AHB5LPENCLRR,
    ///0x398 - This register is used by the MCU in order to set the PERxLPEN bit.
    pub rcc_mc_ahb6lpensetr: RCC_MC_AHB6LPENSETR,
    ///0x39c - This register is used by the MCU in order to clear the PERxLPEN bit
    pub rcc_mc_ahb6lpenclrr: RCC_MC_AHB6LPENCLRR,
    _reserved85: [u8; 0x60],
    ///0x400 - This register is used by the BOOTROM to check the reset source. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR). Refer to Section10.3.13: Reset source identification for details.This register except MPUP\[1:0\]RSTF flags is located into VDD domain, and is reset by por_rst reset. The MPUP\[1:0\]RSTF flags are located into VDDCORE and are reset by nreset. If TZEN = , this register can only be modified in secure mode.
    pub rcc_br_rstsclrr: RCC_BR_RSTSCLRR,
    ///0x404 - This register is used by the MPU in order to generate either a MCU reset or a system reset or a reset of one of the two MPU processors. Writing has no effect, reading returns the effective values of the corresponding bits. Writing a activates the reset.
    pub rcc_mp_grstcsetr: RCC_MP_GRSTCSETR,
    ///0x408 - This register is used by the MPU to check the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode.
    pub rcc_mp_rstsclrr: RCC_MP_RSTSCLRR,
    ///0x40c - This register is used by the BOOTROM in order to freeze the IWDGs clocks. After a system reset or Standby reset (nreset), or a CStandby reset (cstby_rst) the MPU is allowed to write it once.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
    pub rcc_mp_iwdgfzsetr: RCC_MP_IWDGFZSETR,
    ///0x410 - This register is used by the BOOTROM in order to unfreeze the IWDGs clocks. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
    pub rcc_mp_iwdgfzclrr: RCC_MP_IWDGFZCLRR,
    ///0x414 - This register shall be used by the MPU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode.
    pub rcc_mp_cier: RCC_MP_CIER,
    ///0x418 - This register shall be used by the MPU in order to read and clear the interrupt flags.Writing has no effect, writing will clear the corresponding flag.Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode.
    pub rcc_mp_cifr: RCC_MP_CIFR,
    ///0x41c - This register is used to program the delay between the moment where the system exits from one of the Stop modes, and the moment where it is allowed to enable the PLLs and provide a clock to bridges and processors. If TZEN = , this register can only be modified in secure mode.
    pub rcc_pwrlpdlycr: RCC_PWRLPDLYCR,
    ///0x420 - This register is dedicated to the BOOTROM code in order to update the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby. The application software shall not use this register. In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR).Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode.
    pub rcc_mp_rstssetr: RCC_MP_RSTSSETR,
    _reserved94: [u8; 0x03dc],
    ///0x800 - This register is used to select the clock generated on MCO1 output.
    pub rcc_mco1cfgr: RCC_MCO1CFGR,
    ///0x804 - This register is used to select the clock generated on MCO2 output.
    pub rcc_mco2cfgr: RCC_MCO2CFGR,
    ///0x808 - This is a read-only access register, It contains the status flags of oscillators. Writing has no effect.
    pub rcc_ocrdyr: RCC_OCRDYR,
    ///0x80c - This is register contains the enable control of the debug and trace function, and the clock divider for the trace function.
    pub rcc_dbgcfgr: RCC_DBGCFGR,
    _reserved98: [u8; 0x10],
    ///0x820 - This register is used to select the reference clock for PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
    pub rcc_rck3selr: RCC_RCK3SELR,
    ///0x824 - This register is used to select the reference clock for PLL4.
    pub rcc_rck4selr: RCC_RCK4SELR,
    ///0x828 - This register is used to control the prescaler value of timers located into APB1 domain. It concerns TIM2, TIM3, TIM4, TIM5, TIM6, TIM7, TIM12, TIM13 and TIM14. Refer to Section: Sub-system clock generation for additional information.
    pub rcc_timg1prer: RCC_TIMG1PRER,
    ///0x82c - This register is used to control the prescaler value of timers located into APB2 domain. It concerns TIM1, TIM8, TIM15, TIM16, and TIM17. Refer to Section: Sub-system clock generation for additional information.
    pub rcc_timg2prer: RCC_TIMG2PRER,
    ///0x830 - This register is used to control the MCU sub-system clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
    pub rcc_mcudivr: RCC_MCUDIVR,
    ///0x834 - This register is used to control the APB1 clock prescaler. Refer to section Section1.4.6.3: Sub-System Clock Generation for additional information.
    pub rcc_apb1divr: RCC_APB1DIVR,
    ///0x838 - This register is used to control the APB2 clock prescaler. Refer to Section: Sub-system clock generation for additional information.
    pub rcc_apb2divr: RCC_APB2DIVR,
    ///0x83c - This register is used to control the APB3 clock prescaler. Refer to Section: Sub-system clock generation for additional information.
    pub rcc_apb3divr: RCC_APB3DIVR,
    _reserved106: [u8; 0x40],
    ///0x880 - This register is used to control the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
    pub rcc_pll3cr: RCC_PLL3CR,
    ///0x884 - This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
    pub rcc_pll3cfgr1: RCC_PLL3CFGR1,
    ///0x888 - This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
    pub rcc_pll3cfgr2: RCC_PLL3CFGR2,
    ///0x88c - This register is used to fine-tune the frequency of the PLL3 VCO. If TZEN = MCKPROT = , this register can only be modified in secure mode.
    pub rcc_pll3fracr: RCC_PLL3FRACR,
    ///0x890 - This register is used to configure the PLL3.It is not recommended to change the content of this register when the PLL3 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode.
    pub rcc_pll3csgr: RCC_PLL3CSGR,
    ///0x894 - This register is used to control the PLL4.
    pub rcc_pll4cr: RCC_PLL4CR,
    ///0x898 - This register is used to configure the PLL4.
    pub rcc_pll4cfgr1: RCC_PLL4CFGR1,
    ///0x89c - This register is used to configure the PLL4.
    pub rcc_pll4cfgr2: RCC_PLL4CFGR2,
    ///0x8a0 - This register is used to fine-tune the frequency of the PLL4 VCO.
    pub rcc_pll4fracr: RCC_PLL4FRACR,
    ///0x8a4 - This register is used to configure the PLL4.It is not recommended to change the content of this register when the PLL4 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode.
    pub rcc_pll4csgr: RCC_PLL4CSGR,
    _reserved116: [u8; 0x18],
    ///0x8c0 - This register is used to control the selection of the kernel clock for the I2C1 and I2C2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub rcc_i2c12ckselr: RCC_I2C12CKSELR,
    ///0x8c4 - This register is used to control the selection of the kernel clock for the I2C3 and I2C5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub rcc_i2c35ckselr: RCC_I2C35CKSELR,
    ///0x8c8 - This register is used to control the selection of the kernel clock for the SAI1 and DFSDM audio clock. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub rcc_sai1ckselr: RCC_SAI1CKSELR,
    ///0x8cc - This register is used to control the selection of the kernel clock for the SAI2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub rcc_sai2ckselr: RCC_SAI2CKSELR,
    ///0x8d0 - This register is used to control the selection of the kernel clock for the SAI3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub rcc_sai3ckselr: RCC_SAI3CKSELR,
    ///0x8d4 - This register is used to control the selection of the kernel clock for the SAI4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub rcc_sai4ckselr: RCC_SAI4CKSELR,
    ///0x8d8 - This register is used to control the selection of the kernel clock for the SPI/I2S1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub rcc_spi2s1ckselr: RCC_SPI2S1CKSELR,
    ///0x8dc - This register is used to control the selection of the kernel clock for the SPI/I2S2,3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub rcc_spi2s23ckselr: RCC_SPI2S23CKSELR,
    ///0x8e0 - This register is used to control the selection of the kernel clock for the SPI4,5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub rcc_spi45ckselr: RCC_SPI45CKSELR,
    ///0x8e4 - This register is used to control the selection of the kernel clock for the USART6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub rcc_uart6ckselr: RCC_UART6CKSELR,
    ///0x8e8 - This register is used to control the selection of the kernel clock for the USART2 and UART4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub rcc_uart24ckselr: RCC_UART24CKSELR,
    ///0x8ec - This register is used to control the selection of the kernel clock for the USART3 and UART5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub rcc_uart35ckselr: RCC_UART35CKSELR,
    ///0x8f0 - This register is used to control the selection of the kernel clock for the UART7 and UART8. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub rcc_uart78ckselr: RCC_UART78CKSELR,
    ///0x8f4 - This register is used to control the selection of the kernel clock for the SDMMC1 and SDMMC2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub rcc_sdmmc12ckselr: RCC_SDMMC12CKSELR,
    ///0x8f8 - This register is used to control the selection of the kernel clock for the SDMMC3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub rcc_sdmmc3ckselr: RCC_SDMMC3CKSELR,
    ///0x8fc - This register is used to control the selection of the kernel clock for the ETH block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub rcc_ethckselr: RCC_ETHCKSELR,
    ///0x900 - This register is used to control the selection of the kernel clock for the QUADSPI. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub rcc_qspickselr: RCC_QSPICKSELR,
    ///0x904 - This register is used to control the selection of the kernel clock for the FMC block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub rcc_fmcckselr: RCC_FMCCKSELR,
    _reserved134: [u8; 0x04],
    ///0x90c - This register is used to control the selection of the kernel clock for the FDCAN block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub rcc_fdcanckselr: RCC_FDCANCKSELR,
    _reserved135: [u8; 0x04],
    ///0x914 - This register is used to control the selection of the kernel clock for the SPDIFRX. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
    pub rcc_spdifckselr: RCC_SPDIFCKSELR,
    ///0x918 - This register is used to control the selection of the kernel clock for the CEC-HDMI.
    pub rcc_cecckselr: RCC_CECCKSELR,
    ///0x91c - This register is used to control the selection of the kernel clock for the USBPHY PLL of the USB HOST and USB OTG
    pub rcc_usbckselr: RCC_USBCKSELR,
    ///0x920 - This register is used to control the selection of the kernel clock for the RNG2.
    pub rcc_rng2ckselr: RCC_RNG2CKSELR,
    ///0x924 - This register is used to control the selection of the kernel clock for the DSI block.
    pub rcc_dsickselr: RCC_DSICKSELR,
    ///0x928 - This register is used to control the selection of the kernel clock for the ADC block.
    pub rcc_adcckselr: RCC_ADCCKSELR,
    ///0x92c - This register is used to control the selection of the kernel clock for the LPTIM4 and LPTIM5 blocks.
    pub rcc_lptim45ckselr: RCC_LPTIM45CKSELR,
    ///0x930 - This register is used to control the selection of the kernel clock for the LPTIM2 and LPTIM3 blocks.
    pub rcc_lptim23ckselr: RCC_LPTIM23CKSELR,
    ///0x934 - This register is used to control the selection of the kernel clock for the LPTIM1 block.
    pub rcc_lptim1ckselr: RCC_LPTIM1CKSELR,
    _reserved144: [u8; 0x48],
    ///0x980 - This register is used to activate the reset of the corresponding peripheral.
    pub rcc_apb1rstsetr: RCC_APB1RSTSETR,
    ///0x984 - This register is used to release the reset of the corresponding peripheral.
    pub rcc_apb1rstclrr: RCC_APB1RSTCLRR,
    ///0x988 - This register is used to activate the reset of the corresponding peripheral.
    pub rcc_apb2rstsetr: RCC_APB2RSTSETR,
    ///0x98c - This register is used to release the reset of the corresponding peripheral.
    pub rcc_apb2rstclrr: RCC_APB2RSTCLRR,
    ///0x990 - This register is used to activate the reset of the corresponding peripheral.
    pub rcc_apb3rstsetr: RCC_APB3RSTSETR,
    ///0x994 - This register is used to release the reset of the corresponding peripheral.
    pub rcc_apb3rstclrr: RCC_APB3RSTCLRR,
    ///0x998 - This register is used to activate the reset of the corresponding peripheral.
    pub rcc_ahb2rstsetr: RCC_AHB2RSTSETR,
    ///0x99c - This register is used to release the reset of the corresponding peripheral.
    pub rcc_ahb2rstclrr: RCC_AHB2RSTCLRR,
    ///0x9a0 - This register is used to activate the reset of the corresponding peripheral.
    pub rcc_ahb3rstsetr: RCC_AHB3RSTSETR,
    ///0x9a4 - This register is used to release the reset of the corresponding peripheral.
    pub rcc_ahb3rstclrr: RCC_AHB3RSTCLRR,
    ///0x9a8 - This register is used to activate the reset of the corresponding peripheral
    pub rcc_ahb4rstsetr: RCC_AHB4RSTSETR,
    ///0x9ac - This register is used to release the reset of the corresponding peripheral.
    pub rcc_ahb4rstclrr: RCC_AHB4RSTCLRR,
    _reserved156: [u8; 0x50],
    ///0xa00 - This register is used to set the peripheral clock enable bit
    pub rcc_mp_apb1ensetr: RCC_MP_APB1ENSETR,
    ///0xa04 - This register is used to clear the peripheral clock enable bit
    pub rcc_mp_apb1enclrr: RCC_MP_APB1ENCLRR,
    ///0xa08 - This register is used to set the peripheral clock enable bit
    pub rcc_mp_apb2ensetr: RCC_MP_APB2ENSETR,
    ///0xa0c - This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
    pub rcc_mp_apb2enclrr: RCC_MP_APB2ENCLRR,
    ///0xa10 - This register is used to set the peripheral clock enable bit
    pub rcc_mp_apb3ensetr: RCC_MP_APB3ENSETR,
    ///0xa14 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
    pub rcc_mp_apb3enclrr: RCC_MP_APB3ENCLRR,
    ///0xa18 - This register is used to set the peripheral clock enable bit of the corresponding peripheral
    pub rcc_mp_ahb2ensetr: RCC_MP_AHB2ENSETR,
    ///0xa1c - This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
    pub rcc_mp_ahb2enclrr: RCC_MP_AHB2ENCLRR,
    ///0xa20 - This register is used to set the peripheral clock enable bit of the corresponding peripheral
    pub rcc_mp_ahb3ensetr: RCC_MP_AHB3ENSETR,
    ///0xa24 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
    pub rcc_mp_ahb3enclrr: RCC_MP_AHB3ENCLRR,
    ///0xa28 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU.
    pub rcc_mp_ahb4ensetr: RCC_MP_AHB4ENSETR,
    ///0xa2c - This register is used to clear the peripheral clock enable bit
    pub rcc_mp_ahb4enclrr: RCC_MP_AHB4ENCLRR,
    _reserved168: [u8; 0x08],
    ///0xa38 - This register is used to set the peripheral clock enable bit
    pub rcc_mp_mlahbensetr: RCC_MP_MLAHBENSETR,
    ///0xa3c - This register is used to clear the peripheral clock enable bit.
    pub rcc_mp_mlahbenclrr: RCC_MP_MLAHBENCLRR,
    _reserved170: [u8; 0x40],
    ///0xa80 - This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MCU. Writing has no effect, reading will return . Writing a sets the corresponding bit to .
    pub rcc_mc_apb1ensetr: RCC_MC_APB1ENSETR,
    ///0xa84 - This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
    pub rcc_mc_apb1enclrr: RCC_MC_APB1ENCLRR,
    ///0xa88 - This register is used to set the peripheral clock enable bit
    pub rcc_mc_apb2ensetr: RCC_MC_APB2ENSETR,
    ///0xa8c - This register is used to clear the peripheral clock enable bit
    pub rcc_mc_apb2enclrr: RCC_MC_APB2ENCLRR,
    ///0xa90 - This register is used to set the peripheral clock enable bit
    pub rcc_mc_apb3ensetr: RCC_MC_APB3ENSETR,
    ///0xa94 - This register is used to clear the peripheral clock enable bit
    pub rcc_mc_apb3enclrr: RCC_MC_APB3ENCLRR,
    ///0xa98 - This register is used to set the peripheral clock enable bit
    pub rcc_mc_ahb2ensetr: RCC_MC_AHB2ENSETR,
    ///0xa9c - This register is used to clear the peripheral clock enable bit
    pub rcc_mc_ahb2enclrr: RCC_MC_AHB2ENCLRR,
    ///0xaa0 - This register is used to set the peripheral clock enable bit
    pub rcc_mc_ahb3ensetr: RCC_MC_AHB3ENSETR,
    ///0xaa4 - This register is used to clear the peripheral clock enable bit
    pub rcc_mc_ahb3enclrr: RCC_MC_AHB3ENCLRR,
    ///0xaa8 - This register is used to set the peripheral clock enable bit
    pub rcc_mc_ahb4ensetr: RCC_MC_AHB4ENSETR,
    ///0xaac - This register is used to clear the peripheral clock enable bit
    pub rcc_mc_ahb4enclrr: RCC_MC_AHB4ENCLRR,
    ///0xab0 - This register is used to set the peripheral clock enable bit
    pub rcc_mc_aximensetr: RCC_MC_AXIMENSETR,
    ///0xab4 - This register is used to clear the peripheral clock enable bit
    pub rcc_mc_aximenclrr: RCC_MC_AXIMENCLRR,
    ///0xab8 - This register is used to set the peripheral clock enable bit
    pub rcc_mc_mlahbensetr: RCC_MC_MLAHBENSETR,
    ///0xabc - This register is used to clear the peripheral clock enable bit
    pub rcc_mc_mlahbenclrr: RCC_MC_MLAHBENCLRR,
    _reserved186: [u8; 0x40],
    ///0xb00 - This register is used by the MCU in order to clear the PERxLPEN bits
    pub rcc_mp_apb1lpensetr: RCC_MP_APB1LPENSETR,
    ///0xb04 - This register is used by the MPU in order to clear the PERxLPEN bits .
    pub rcc_mp_apb1lpenclrr: RCC_MP_APB1LPENCLRR,
    ///0xb08 - This register is used by the MCU in order to clear the PERxLPEN bits
    pub rcc_mp_apb2lpensetr: RCC_MP_APB2LPENSETR,
    ///0xb0c - This register is used by the MCU in order to clear the PERxLPEN bits
    pub rcc_mp_apb2lpenclrr: RCC_MP_APB2LPENCLRR,
    ///0xb10 - This register is used by the MCU in order to clear the PERxLPEN bits
    pub rcc_mp_apb3lpensetr: RCC_MP_APB3LPENSETR,
    ///0xb14 - This register is used by the MCU in order to clear the PERxLPEN bits
    pub rcc_mp_apb3lpenclrr: RCC_MP_APB3LPENCLRR,
    ///0xb18 - This register is used by the MPU in order to set the PERxLPEN bit.
    pub rcc_mp_ahb2lpensetr: RCC_MP_AHB2LPENSETR,
    ///0xb1c - This register is used by the MCU in order to clear the PERxLPEN bits
    pub rcc_mp_ahb2lpenclrr: RCC_MP_AHB2LPENCLRR,
    ///0xb20 - This register is used by the MPU
    pub rcc_mp_ahb3lpensetr: RCC_MP_AHB3LPENSETR,
    ///0xb24 - This register is used by the MPU in order to clear the PERxLPEN bit
    pub rcc_mp_ahb3lpenclrr: RCC_MP_AHB3LPENCLRR,
    ///0xb28 - This register is used by the MPU
    pub rcc_mp_ahb4lpensetr: RCC_MP_AHB4LPENSETR,
    ///0xb2c - This register is used by the MPU
    pub rcc_mp_ahb4lpenclrr: RCC_MP_AHB4LPENCLRR,
    ///0xb30 - This register is used by the MPU
    pub rcc_mp_aximlpensetr: RCC_MP_AXIMLPENSETR,
    ///0xb34 - This register is used by the MPU
    pub rcc_mp_aximlpenclrr: RCC_MP_AXIMLPENCLRR,
    ///0xb38 - This register is used by the MPU in order to set the PERxLPEN bit
    pub rcc_mp_mlahblpensetr: RCC_MP_MLAHBLPENSETR,
    ///0xb3c - This register is used by the MPU in order to clear the PERxLPEN bit
    pub rcc_mp_mlahblpenclrr: RCC_MP_MLAHBLPENCLRR,
    _reserved202: [u8; 0x40],
    ///0xb80 - This register is used by the MCU in order to set the PERxLPEN bit.
    pub rcc_mc_apb1lpensetr: RCC_MC_APB1LPENSETR,
    ///0xb84 - This register is used by the MCU in order to clear the PERxLPEN bits
    pub rcc_mc_apb1lpenclrr: RCC_MC_APB1LPENCLRR,
    ///0xb88 - This register is used by the MCU in order to set the PERxLPEN bit.
    pub rcc_mc_apb2lpensetr: RCC_MC_APB2LPENSETR,
    ///0xb8c - This register is used by the MCU in order to clear the PERxLPEN bit
    pub rcc_mc_apb2lpenclrr: RCC_MC_APB2LPENCLRR,
    ///0xb90 - This register is used by the MCU in order to set the PERxLPEN bit.
    pub rcc_mc_apb3lpensetr: RCC_MC_APB3LPENSETR,
    ///0xb94 - This register is used by the MCU in order to clear the PERxLPEN bit
    pub rcc_mc_apb3lpenclrr: RCC_MC_APB3LPENCLRR,
    ///0xb98 - This register is used by the MCU in order to set the PERxLPEN bit.
    pub rcc_mc_ahb2lpensetr: RCC_MC_AHB2LPENSETR,
    ///0xb9c - This register is used by the MCU in order to clear the PERxLPEN bit
    pub rcc_mc_ahb2lpenclrr: RCC_MC_AHB2LPENCLRR,
    ///0xba0 - This register is used by the MCU in order to set the PERxLPEN bit.
    pub rcc_mc_ahb3lpensetr: RCC_MC_AHB3LPENSETR,
    ///0xba4 - This register is used by the MCU in order to clear the PERxLPEN bit
    pub rcc_mc_ahb3lpenclrr: RCC_MC_AHB3LPENCLRR,
    ///0xba8 - This register is used by the MCU in order to set the PERxLPEN bit.
    pub rcc_mc_ahb4lpensetr: RCC_MC_AHB4LPENSETR,
    ///0xbac - This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.
    pub rcc_mc_ahb4lpenclrr: RCC_MC_AHB4LPENCLRR,
    ///0xbb0 - This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral.
    pub rcc_mc_aximlpensetr: RCC_MC_AXIMLPENSETR,
    ///0xbb4 - This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.
    pub rcc_mc_aximlpenclrr: RCC_MC_AXIMLPENCLRR,
    ///0xbb8 - This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral.
    pub rcc_mc_mlahblpensetr: RCC_MC_MLAHBLPENSETR,
    ///0xbbc - This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.
    pub rcc_mc_mlahblpenclrr: RCC_MC_MLAHBLPENCLRR,
    _reserved218: [u8; 0x40],
    ///0xc00 - This register is used by the MCU to check the reset source.
    pub rcc_mc_rstsclrr: RCC_MC_RSTSCLRR,
    _reserved219: [u8; 0x10],
    ///0xc14 - This register shall be used by the MCU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details.
    pub rcc_mc_cier: RCC_MC_CIER,
    ///0xc18 - This register shall be used by the MCU in order to read and clear the interrupt flags.
    pub rcc_mc_cifr: RCC_MC_CIFR,
    _reserved221: [u8; 0x03d8],
    ///0xff4 - This register gives the IP version
    pub rcc_verr: RCC_VERR,
    ///0xff8 - This register gives the unique identifier of the RCC
    pub rcc_idr: RCC_IDR,
    ///0xffc - This register gives the decoding space, which is for the RCC of 4 kB.
    pub rcc_sidr: RCC_SIDR,
}
///RCC_TZCR (rw) register accessor: an alias for `Reg<RCC_TZCR_SPEC>`
pub type RCC_TZCR = crate::Reg<rcc_tzcr::RCC_TZCR_SPEC>;
///This register is used to switch the RCC into secure mode. This register can only be accessed in secure mode.
pub mod rcc_tzcr;
///RCC_OCENSETR (rw) register accessor: an alias for `Reg<RCC_OCENSETR_SPEC>`
pub type RCC_OCENSETR = crate::Reg<rcc_ocensetr::RCC_OCENSETR_SPEC>;
///This register is used to control the oscillators.Writing to this register has no effect, writing will set the corresponding bits. Reading will give the effective values of each bit.If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod rcc_ocensetr;
///RCC_OCENCLRR (rw) register accessor: an alias for `Reg<RCC_OCENCLRR_SPEC>`
pub type RCC_OCENCLRR = crate::Reg<rcc_ocenclrr::RCC_OCENCLRR_SPEC>;
///This register is used to control the oscillators.Writing to this register has no effect, writing will clear the corresponding bits. Reading will give the effective values of the enable bits.If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod rcc_ocenclrr;
///RCC_HSICFGR (rw) register accessor: an alias for `Reg<RCC_HSICFGR_SPEC>`
pub type RCC_HSICFGR = crate::Reg<rcc_hsicfgr::RCC_HSICFGR_SPEC>;
///This register is used to configure the HSI. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod rcc_hsicfgr;
///RCC_CSICFGR (rw) register accessor: an alias for `Reg<RCC_CSICFGR_SPEC>`
pub type RCC_CSICFGR = crate::Reg<rcc_csicfgr::RCC_CSICFGR_SPEC>;
///This register is used to fine-tune the CSI frequency. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See The clock restore sequence description for details.
pub mod rcc_csicfgr;
///RCC_MPCKSELR (rw) register accessor: an alias for `Reg<RCC_MPCKSELR_SPEC>`
pub type RCC_MPCKSELR = crate::Reg<rcc_mpckselr::RCC_MPCKSELR_SPEC>;
///This register is used to select the clock source for the MPU. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod rcc_mpckselr;
///RCC_ASSCKSELR (rw) register accessor: an alias for `Reg<RCC_ASSCKSELR_SPEC>`
pub type RCC_ASSCKSELR = crate::Reg<rcc_assckselr::RCC_ASSCKSELR_SPEC>;
///This register is used to select the clock source for the AXI sub-system. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod rcc_assckselr;
///RCC_RCK12SELR (rw) register accessor: an alias for `Reg<RCC_RCK12SELR_SPEC>`
pub type RCC_RCK12SELR = crate::Reg<rcc_rck12selr::RCC_RCK12SELR_SPEC>;
///This register is used to select the reference clock for PLL1 and PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod rcc_rck12selr;
///RCC_MPCKDIVR (rw) register accessor: an alias for `Reg<RCC_MPCKDIVR_SPEC>`
pub type RCC_MPCKDIVR = crate::Reg<rcc_mpckdivr::RCC_MPCKDIVR_SPEC>;
///This register is used to control the MPU clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mpckdivr;
///RCC_AXIDIVR (rw) register accessor: an alias for `Reg<RCC_AXIDIVR_SPEC>`
pub type RCC_AXIDIVR = crate::Reg<rcc_axidivr::RCC_AXIDIVR_SPEC>;
///This register is used to control the AXI Matrix clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_axidivr;
///RCC_APB4DIVR (rw) register accessor: an alias for `Reg<RCC_APB4DIVR_SPEC>`
pub type RCC_APB4DIVR = crate::Reg<rcc_apb4divr::RCC_APB4DIVR_SPEC>;
///This register is used to control the APB4 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_apb4divr;
///RCC_APB5DIVR (rw) register accessor: an alias for `Reg<RCC_APB5DIVR_SPEC>`
pub type RCC_APB5DIVR = crate::Reg<rcc_apb5divr::RCC_APB5DIVR_SPEC>;
///This register is used to control the APB5 clock divider. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_apb5divr;
///RCC_RTCDIVR (rw) register accessor: an alias for `Reg<RCC_RTCDIVR_SPEC>`
pub type RCC_RTCDIVR = crate::Reg<rcc_rtcdivr::RCC_RTCDIVR_SPEC>;
///This register is used to divide the HSE clock for RTC. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_rtcdivr;
///RCC_MSSCKSELR (rw) register accessor: an alias for `Reg<RCC_MSSCKSELR_SPEC>`
pub type RCC_MSSCKSELR = crate::Reg<rcc_mssckselr::RCC_MSSCKSELR_SPEC>;
///This register is used to select the clock source for the MCU sub-system, including the MCU itself. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod rcc_mssckselr;
///RCC_PLL1CR (rw) register accessor: an alias for `Reg<RCC_PLL1CR_SPEC>`
pub type RCC_PLL1CR = crate::Reg<rcc_pll1cr::RCC_PLL1CR_SPEC>;
///This register is used to control the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod rcc_pll1cr;
///RCC_PLL1CFGR1 (rw) register accessor: an alias for `Reg<RCC_PLL1CFGR1_SPEC>`
pub type RCC_PLL1CFGR1 = crate::Reg<rcc_pll1cfgr1::RCC_PLL1CFGR1_SPEC>;
///This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod rcc_pll1cfgr1;
///RCC_PLL1CFGR2 (rw) register accessor: an alias for `Reg<RCC_PLL1CFGR2_SPEC>`
pub type RCC_PLL1CFGR2 = crate::Reg<rcc_pll1cfgr2::RCC_PLL1CFGR2_SPEC>;
///This register is used to configure the PLL1. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod rcc_pll1cfgr2;
///RCC_PLL1FRACR (rw) register accessor: an alias for `Reg<RCC_PLL1FRACR_SPEC>`
pub type RCC_PLL1FRACR = crate::Reg<rcc_pll1fracr::RCC_PLL1FRACR_SPEC>;
///This register is used to fine-tune the frequency of the PLL1 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod rcc_pll1fracr;
///RCC_PLL1CSGR (rw) register accessor: an alias for `Reg<RCC_PLL1CSGR_SPEC>`
pub type RCC_PLL1CSGR = crate::Reg<rcc_pll1csgr::RCC_PLL1CSGR_SPEC>;
///This register is used to configure the PLL1.It is not recommended to change the content of this register when the PLL1 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod rcc_pll1csgr;
///RCC_PLL2CR (rw) register accessor: an alias for `Reg<RCC_PLL2CR_SPEC>`
pub type RCC_PLL2CR = crate::Reg<rcc_pll2cr::RCC_PLL2CR_SPEC>;
///This register is used to control the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod rcc_pll2cr;
///RCC_PLL2CFGR1 (rw) register accessor: an alias for `Reg<RCC_PLL2CFGR1_SPEC>`
pub type RCC_PLL2CFGR1 = crate::Reg<rcc_pll2cfgr1::RCC_PLL2CFGR1_SPEC>;
///This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod rcc_pll2cfgr1;
///RCC_PLL2CFGR2 (rw) register accessor: an alias for `Reg<RCC_PLL2CFGR2_SPEC>`
pub type RCC_PLL2CFGR2 = crate::Reg<rcc_pll2cfgr2::RCC_PLL2CFGR2_SPEC>;
///This register is used to configure the PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod rcc_pll2cfgr2;
///RCC_PLL2FRACR (rw) register accessor: an alias for `Reg<RCC_PLL2FRACR_SPEC>`
pub type RCC_PLL2FRACR = crate::Reg<rcc_pll2fracr::RCC_PLL2FRACR_SPEC>;
///This register is used to fine-tune the frequency of the PLL2 VCO. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod rcc_pll2fracr;
///RCC_PLL2CSGR (rw) register accessor: an alias for `Reg<RCC_PLL2CSGR_SPEC>`
pub type RCC_PLL2CSGR = crate::Reg<rcc_pll2csgr::RCC_PLL2CSGR_SPEC>;
///This register is used to configure the PLL2. It is not recommended to change the content of this register when the PLL2 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.
pub mod rcc_pll2csgr;
///RCC_I2C46CKSELR (rw) register accessor: an alias for `Reg<RCC_I2C46CKSELR_SPEC>`
pub type RCC_I2C46CKSELR = crate::Reg<rcc_i2c46ckselr::RCC_I2C46CKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the I2C4 and I2C6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_i2c46ckselr;
///RCC_SPI6CKSELR (rw) register accessor: an alias for `Reg<RCC_SPI6CKSELR_SPEC>`
pub type RCC_SPI6CKSELR = crate::Reg<rcc_spi6ckselr::RCC_SPI6CKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the SPI6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_spi6ckselr;
///RCC_UART1CKSELR (rw) register accessor: an alias for `Reg<RCC_UART1CKSELR_SPEC>`
pub type RCC_UART1CKSELR = crate::Reg<rcc_uart1ckselr::RCC_UART1CKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the USART1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_uart1ckselr;
///RCC_RNG1CKSELR (rw) register accessor: an alias for `Reg<RCC_RNG1CKSELR_SPEC>`
pub type RCC_RNG1CKSELR = crate::Reg<rcc_rng1ckselr::RCC_RNG1CKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the RNG1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_rng1ckselr;
///RCC_CPERCKSELR (rw) register accessor: an alias for `Reg<RCC_CPERCKSELR_SPEC>`
pub type RCC_CPERCKSELR = crate::Reg<rcc_cperckselr::RCC_CPERCKSELR_SPEC>;
///This register is used to select an oscillator source as kernel clock for the per_ck clock. The per_ck clock is distributed to several peripherals. Refer to Section: Clock enabling delays.
pub mod rcc_cperckselr;
///RCC_STGENCKSELR (rw) register accessor: an alias for `Reg<RCC_STGENCKSELR_SPEC>`
pub type RCC_STGENCKSELR = crate::Reg<rcc_stgenckselr::RCC_STGENCKSELR_SPEC>;
///This register is used to select the peripheral clock for the STGEN block. Note that this clock is used to provide a time reference for the application. Refer to Section: Clock enabling delays. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_stgenckselr;
///RCC_DDRITFCR (rw) register accessor: an alias for `Reg<RCC_DDRITFCR_SPEC>`
pub type RCC_DDRITFCR = crate::Reg<rcc_ddritfcr::RCC_DDRITFCR_SPEC>;
///This register is used to control the DDR interface, including the DDRC and DDRPHYC. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_ddritfcr;
///RCC_MP_BOOTCR (rw) register accessor: an alias for `Reg<RCC_MP_BOOTCR_SPEC>`
pub type RCC_MP_BOOTCR = crate::Reg<rcc_mp_bootcr::RCC_MP_BOOTCR_SPEC>;
///This register is used to control the HOLD boot function when the system exits from Standby. Refer to Section: MCU HOLD_BOOT after processor reset. This register is reset when a system reset occurs, but not when the circuit exits from Standby (app_rst reset).If TZEN = , this register can only be modified in secure mode. This register can only be accessed by the MPU.
pub mod rcc_mp_bootcr;
///RCC_MP_SREQSETR (rw) register accessor: an alias for `Reg<RCC_MP_SREQSETR_SPEC>`
pub type RCC_MP_SREQSETR = crate::Reg<rcc_mp_sreqsetr::RCC_MP_SREQSETR_SPEC>;
///Writing has no effect, reading will return the values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mp_sreqsetr;
///RCC_MP_SREQCLRR (rw) register accessor: an alias for `Reg<RCC_MP_SREQCLRR_SPEC>`
pub type RCC_MP_SREQCLRR = crate::Reg<rcc_mp_sreqclrr::RCC_MP_SREQCLRR_SPEC>;
///Writing has no effect, reading will return the effective values of the bits. Writing a sets the corresponding bit to . The MCU cannot access to this register. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mp_sreqclrr;
///RCC_MP_GCR (rw) register accessor: an alias for `Reg<RCC_MP_GCR_SPEC>`
pub type RCC_MP_GCR = crate::Reg<rcc_mp_gcr::RCC_MP_GCR_SPEC>;
///The register contains global control bits. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mp_gcr;
///RCC_MP_APRSTCR (rw) register accessor: an alias for `Reg<RCC_MP_APRSTCR_SPEC>`
pub type RCC_MP_APRSTCR = crate::Reg<rcc_mp_aprstcr::RCC_MP_APRSTCR_SPEC>;
///This register is used to control the behavior of the warm reset. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mp_aprstcr;
///RCC_MP_APRSTSR (r) register accessor: an alias for `Reg<RCC_MP_APRSTSR_SPEC>`
pub type RCC_MP_APRSTSR = crate::Reg<rcc_mp_aprstsr::RCC_MP_APRSTSR_SPEC>;
///This register provides a status of the RDCTL. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mp_aprstsr;
///RCC_BDCR (rw) register accessor: an alias for `Reg<RCC_BDCR_SPEC>`
pub type RCC_BDCR = crate::Reg<rcc_bdcr::RCC_BDCR_SPEC>;
///This register is used to control the LSE function. Wait states are inserted in case of successive write accesses to this register. The number of wait states may be up to 7 cycles of AHB4 clock.After a system reset, the register RCC_BDCR is write-protected. In order to modify this register, the DBP bit in the PWR control register 1 (PWR_CR1) has to be set to . Bits of RCC_BDCR register are only reset after a backup domain reset: nreset_vsw (see Section10.3.6: Backup domain reset). Any other internal or external reset will not have any effect on these bits.This register is located into the VSW domain. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_bdcr;
///RCC_RDLSICR (rw) register accessor: an alias for `Reg<RCC_RDLSICR_SPEC>`
pub type RCC_RDLSICR = crate::Reg<rcc_rdlsicr::RCC_RDLSICR_SPEC>;
///This register is used to control the minimum NRST active duration and LSI function.0 to 7 wait states are inserted for word, half-word and byte accesses. Wait states are inserted in case of successive accesses to this register.This register is reset by the por_rst reset, and it is located into the VDD domain. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_rdlsicr;
///RCC_APB4RSTSETR (rw) register accessor: an alias for `Reg<RCC_APB4RSTSETR_SPEC>`
pub type RCC_APB4RSTSETR = crate::Reg<rcc_apb4rstsetr::RCC_APB4RSTSETR_SPEC>;
///This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral.
pub mod rcc_apb4rstsetr;
///RCC_APB4RSTCLRR (rw) register accessor: an alias for `Reg<RCC_APB4RSTCLRR_SPEC>`
pub type RCC_APB4RSTCLRR = crate::Reg<rcc_apb4rstclrr::RCC_APB4RSTCLRR_SPEC>;
///This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral.
pub mod rcc_apb4rstclrr;
///RCC_APB5RSTSETR (rw) register accessor: an alias for `Reg<RCC_APB5RSTSETR_SPEC>`
pub type RCC_APB5RSTSETR = crate::Reg<rcc_apb5rstsetr::RCC_APB5RSTSETR_SPEC>;
///This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_apb5rstsetr;
///RCC_APB5RSTCLRR (rw) register accessor: an alias for `Reg<RCC_APB5RSTCLRR_SPEC>`
pub type RCC_APB5RSTCLRR = crate::Reg<rcc_apb5rstclrr::RCC_APB5RSTCLRR_SPEC>;
///This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_apb5rstclrr;
///RCC_AHB5RSTSETR (rw) register accessor: an alias for `Reg<RCC_AHB5RSTSETR_SPEC>`
pub type RCC_AHB5RSTSETR = crate::Reg<rcc_ahb5rstsetr::RCC_AHB5RSTSETR_SPEC>;
///This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_ahb5rstsetr;
///RCC_AHB5RSTCLRR (rw) register accessor: an alias for `Reg<RCC_AHB5RSTCLRR_SPEC>`
pub type RCC_AHB5RSTCLRR = crate::Reg<rcc_ahb5rstclrr::RCC_AHB5RSTCLRR_SPEC>;
///This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_ahb5rstclrr;
///RCC_AHB6RSTSETR (rw) register accessor: an alias for `Reg<RCC_AHB6RSTSETR_SPEC>`
pub type RCC_AHB6RSTSETR = crate::Reg<rcc_ahb6rstsetr::RCC_AHB6RSTSETR_SPEC>;
///This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral.
pub mod rcc_ahb6rstsetr;
///RCC_AHB6RSTCLRR (rw) register accessor: an alias for `Reg<RCC_AHB6RSTCLRR_SPEC>`
pub type RCC_AHB6RSTCLRR = crate::Reg<rcc_ahb6rstclrr::RCC_AHB6RSTCLRR_SPEC>;
///This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral.
pub mod rcc_ahb6rstclrr;
///RCC_TZAHB6RSTSETR (rw) register accessor: an alias for `Reg<RCC_TZAHB6RSTSETR_SPEC>`
pub type RCC_TZAHB6RSTSETR = crate::Reg<rcc_tzahb6rstsetr::RCC_TZAHB6RSTSETR_SPEC>;
///This register is used to activate the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a activates the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_tzahb6rstsetr;
///RCC_TZAHB6RSTCLRR (rw) register accessor: an alias for `Reg<RCC_TZAHB6RSTCLRR_SPEC>`
pub type RCC_TZAHB6RSTCLRR = crate::Reg<rcc_tzahb6rstclrr::RCC_TZAHB6RSTCLRR_SPEC>;
///This register is used to release the reset of the corresponding peripheral. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a releases the reset of the corresponding peripheral. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_tzahb6rstclrr;
///RCC_MP_APB4ENSETR (rw) register accessor: an alias for `Reg<RCC_MP_APB4ENSETR_SPEC>`
pub type RCC_MP_APB4ENSETR = crate::Reg<rcc_mp_apb4ensetr::RCC_MP_APB4ENSETR_SPEC>;
///This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
pub mod rcc_mp_apb4ensetr;
///RCC_MP_APB4ENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_APB4ENCLRR_SPEC>`
pub type RCC_MP_APB4ENCLRR = crate::Reg<rcc_mp_apb4enclrr::RCC_MP_APB4ENCLRR_SPEC>;
///This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
pub mod rcc_mp_apb4enclrr;
///RCC_MP_APB5ENSETR (rw) register accessor: an alias for `Reg<RCC_MP_APB5ENSETR_SPEC>`
pub type RCC_MP_APB5ENSETR = crate::Reg<rcc_mp_apb5ensetr::RCC_MP_APB5ENSETR_SPEC>;
///This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
pub mod rcc_mp_apb5ensetr;
///RCC_MP_APB5ENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_APB5ENCLRR_SPEC>`
pub type RCC_MP_APB5ENCLRR = crate::Reg<rcc_mp_apb5enclrr::RCC_MP_APB5ENCLRR_SPEC>;
///This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
pub mod rcc_mp_apb5enclrr;
///RCC_MP_AHB5ENSETR (rw) register accessor: an alias for `Reg<RCC_MP_AHB5ENSETR_SPEC>`
pub type RCC_MP_AHB5ENSETR = crate::Reg<rcc_mp_ahb5ensetr::RCC_MP_AHB5ENSETR_SPEC>;
///This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mp_ahb5ensetr;
///RCC_MP_AHB5ENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_AHB5ENCLRR_SPEC>`
pub type RCC_MP_AHB5ENCLRR = crate::Reg<rcc_mp_ahb5enclrr::RCC_MP_AHB5ENCLRR_SPEC>;
///This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mp_ahb5enclrr;
///RCC_MP_AHB6ENSETR (rw) register accessor: an alias for `Reg<RCC_MP_AHB6ENSETR_SPEC>`
pub type RCC_MP_AHB6ENSETR = crate::Reg<rcc_mp_ahb6ensetr::RCC_MP_AHB6ENSETR_SPEC>;
///This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
pub mod rcc_mp_ahb6ensetr;
///RCC_MP_AHB6ENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_AHB6ENCLRR_SPEC>`
pub type RCC_MP_AHB6ENCLRR = crate::Reg<rcc_mp_ahb6enclrr::RCC_MP_AHB6ENCLRR_SPEC>;
///This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .
pub mod rcc_mp_ahb6enclrr;
///RCC_MP_TZAHB6ENSETR (rw) register accessor: an alias for `Reg<RCC_MP_TZAHB6ENSETR_SPEC>`
pub type RCC_MP_TZAHB6ENSETR = crate::Reg<rcc_mp_tzahb6ensetr::RCC_MP_TZAHB6ENSETR_SPEC>;
///This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mp_tzahb6ensetr;
///RCC_MP_TZAHB6ENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_TZAHB6ENCLRR_SPEC>`
pub type RCC_MP_TZAHB6ENCLRR = crate::Reg<rcc_mp_tzahb6enclrr::RCC_MP_TZAHB6ENCLRR_SPEC>;
///This register is used to clear the peripheral clock enable bit of the corresponding peripheral. It shall be used to deallocate a peripheral from MPU. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mp_tzahb6enclrr;
///RCC_MC_APB4ENSETR (rw) register accessor: an alias for `Reg<RCC_MC_APB4ENSETR_SPEC>`
pub type RCC_MC_APB4ENSETR = crate::Reg<rcc_mc_apb4ensetr::RCC_MC_APB4ENSETR_SPEC>;
///This register is used to set the peripheral clock enable bit
pub mod rcc_mc_apb4ensetr;
///RCC_MC_APB4ENCLRR (rw) register accessor: an alias for `Reg<RCC_MC_APB4ENCLRR_SPEC>`
pub type RCC_MC_APB4ENCLRR = crate::Reg<rcc_mc_apb4enclrr::RCC_MC_APB4ENCLRR_SPEC>;
///This register is used to clear the peripheral clock enable bit
pub mod rcc_mc_apb4enclrr;
///RCC_MC_APB5ENSETR (rw) register accessor: an alias for `Reg<RCC_MC_APB5ENSETR_SPEC>`
pub type RCC_MC_APB5ENSETR = crate::Reg<rcc_mc_apb5ensetr::RCC_MC_APB5ENSETR_SPEC>;
///This register is used to set the peripheral clock enable bit
pub mod rcc_mc_apb5ensetr;
///RCC_MC_APB5ENCLRR (rw) register accessor: an alias for `Reg<RCC_MC_APB5ENCLRR_SPEC>`
pub type RCC_MC_APB5ENCLRR = crate::Reg<rcc_mc_apb5enclrr::RCC_MC_APB5ENCLRR_SPEC>;
///This register is used to clear the peripheral clock enable bit
pub mod rcc_mc_apb5enclrr;
///RCC_MC_AHB5ENSETR (rw) register accessor: an alias for `Reg<RCC_MC_AHB5ENSETR_SPEC>`
pub type RCC_MC_AHB5ENSETR = crate::Reg<rcc_mc_ahb5ensetr::RCC_MC_AHB5ENSETR_SPEC>;
///This register is used to set the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mc_ahb5ensetr;
///RCC_MC_AHB5ENCLRR (rw) register accessor: an alias for `Reg<RCC_MC_AHB5ENCLRR_SPEC>`
pub type RCC_MC_AHB5ENCLRR = crate::Reg<rcc_mc_ahb5enclrr::RCC_MC_AHB5ENCLRR_SPEC>;
///This register is used to clear the peripheral clock enable bit If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mc_ahb5enclrr;
///RCC_MC_AHB6ENSETR (rw) register accessor: an alias for `Reg<RCC_MC_AHB6ENSETR_SPEC>`
pub type RCC_MC_AHB6ENSETR = crate::Reg<rcc_mc_ahb6ensetr::RCC_MC_AHB6ENSETR_SPEC>;
///This register is used to set the peripheral clock enable bit
pub mod rcc_mc_ahb6ensetr;
///RCC_MC_AHB6ENCLRR (rw) register accessor: an alias for `Reg<RCC_MC_AHB6ENCLRR_SPEC>`
pub type RCC_MC_AHB6ENCLRR = crate::Reg<rcc_mc_ahb6enclrr::RCC_MC_AHB6ENCLRR_SPEC>;
///This register is used to clear the peripheral clock enable bit
pub mod rcc_mc_ahb6enclrr;
///RCC_MP_APB4LPENSETR (rw) register accessor: an alias for `Reg<RCC_MP_APB4LPENSETR_SPEC>`
pub type RCC_MP_APB4LPENSETR = crate::Reg<rcc_mp_apb4lpensetr::RCC_MP_APB4LPENSETR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bits
pub mod rcc_mp_apb4lpensetr;
///RCC_MP_APB4LPENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_APB4LPENCLRR_SPEC>`
pub type RCC_MP_APB4LPENCLRR = crate::Reg<rcc_mp_apb4lpenclrr::RCC_MP_APB4LPENCLRR_SPEC>;
///This register is used by the MCU
pub mod rcc_mp_apb4lpenclrr;
///RCC_MP_APB5LPENSETR (rw) register accessor: an alias for `Reg<RCC_MP_APB5LPENSETR_SPEC>`
pub type RCC_MP_APB5LPENSETR = crate::Reg<rcc_mp_apb5lpensetr::RCC_MP_APB5LPENSETR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mp_apb5lpensetr;
///RCC_MP_APB5LPENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_APB5LPENCLRR_SPEC>`
pub type RCC_MP_APB5LPENCLRR = crate::Reg<rcc_mp_apb5lpenclrr::RCC_MP_APB5LPENCLRR_SPEC>;
///This register is used by the Mpu.
pub mod rcc_mp_apb5lpenclrr;
///RCC_MP_AHB5LPENSETR (rw) register accessor: an alias for `Reg<RCC_MP_AHB5LPENSETR_SPEC>`
pub type RCC_MP_AHB5LPENSETR = crate::Reg<rcc_mp_ahb5lpensetr::RCC_MP_AHB5LPENSETR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mp_ahb5lpensetr;
///RCC_MP_AHB5LPENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_AHB5LPENCLRR_SPEC>`
pub type RCC_MP_AHB5LPENCLRR = crate::Reg<rcc_mp_ahb5lpenclrr::RCC_MP_AHB5LPENCLRR_SPEC>;
///This register is used by the MCU
pub mod rcc_mp_ahb5lpenclrr;
///RCC_MP_AHB6LPENSETR (rw) register accessor: an alias for `Reg<RCC_MP_AHB6LPENSETR_SPEC>`
pub type RCC_MP_AHB6LPENSETR = crate::Reg<rcc_mp_ahb6lpensetr::RCC_MP_AHB6LPENSETR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bits
pub mod rcc_mp_ahb6lpensetr;
///RCC_MP_AHB6LPENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_AHB6LPENCLRR_SPEC>`
pub type RCC_MP_AHB6LPENCLRR = crate::Reg<rcc_mp_ahb6lpenclrr::RCC_MP_AHB6LPENCLRR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bits
pub mod rcc_mp_ahb6lpenclrr;
///RCC_MP_TZAHB6LPENSETR (rw) register accessor: an alias for `Reg<RCC_MP_TZAHB6LPENSETR_SPEC>`
pub type RCC_MP_TZAHB6LPENSETR = crate::Reg<rcc_mp_tzahb6lpensetr::RCC_MP_TZAHB6LPENSETR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mp_tzahb6lpensetr;
///RCC_MP_TZAHB6LPENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_TZAHB6LPENCLRR_SPEC>`
pub type RCC_MP_TZAHB6LPENCLRR = crate::Reg<rcc_mp_tzahb6lpenclrr::RCC_MP_TZAHB6LPENCLRR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mp_tzahb6lpenclrr;
///RCC_MC_APB4LPENSETR (rw) register accessor: an alias for `Reg<RCC_MC_APB4LPENSETR_SPEC>`
pub type RCC_MC_APB4LPENSETR = crate::Reg<rcc_mc_apb4lpensetr::RCC_MC_APB4LPENSETR_SPEC>;
///This register is used by the MCU in order to set the PERxLPEN bit.
pub mod rcc_mc_apb4lpensetr;
///RCC_MC_APB4LPENCLRR (rw) register accessor: an alias for `Reg<RCC_MC_APB4LPENCLRR_SPEC>`
pub type RCC_MC_APB4LPENCLRR = crate::Reg<rcc_mc_apb4lpenclrr::RCC_MC_APB4LPENCLRR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bit
pub mod rcc_mc_apb4lpenclrr;
///RCC_MC_APB5LPENSETR (rw) register accessor: an alias for `Reg<RCC_MC_APB5LPENSETR_SPEC>`
pub type RCC_MC_APB5LPENSETR = crate::Reg<rcc_mc_apb5lpensetr::RCC_MC_APB5LPENSETR_SPEC>;
///This register is used by the MCU in order to set the PERxLPEN bit.
pub mod rcc_mc_apb5lpensetr;
///RCC_MC_APB5LPENCLRR (rw) register accessor: an alias for `Reg<RCC_MC_APB5LPENCLRR_SPEC>`
pub type RCC_MC_APB5LPENCLRR = crate::Reg<rcc_mc_apb5lpenclrr::RCC_MC_APB5LPENCLRR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bit
pub mod rcc_mc_apb5lpenclrr;
///RCC_MC_AHB5LPENSETR (rw) register accessor: an alias for `Reg<RCC_MC_AHB5LPENSETR_SPEC>`
pub type RCC_MC_AHB5LPENSETR = crate::Reg<rcc_mc_ahb5lpensetr::RCC_MC_AHB5LPENSETR_SPEC>;
///This register is used by the MCU in order to set the PERxLPEN bit. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mc_ahb5lpensetr;
///RCC_MC_AHB5LPENCLRR (rw) register accessor: an alias for `Reg<RCC_MC_AHB5LPENCLRR_SPEC>`
pub type RCC_MC_AHB5LPENCLRR = crate::Reg<rcc_mc_ahb5lpenclrr::RCC_MC_AHB5LPENCLRR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bit If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mc_ahb5lpenclrr;
///RCC_MC_AHB6LPENSETR (rw) register accessor: an alias for `Reg<RCC_MC_AHB6LPENSETR_SPEC>`
pub type RCC_MC_AHB6LPENSETR = crate::Reg<rcc_mc_ahb6lpensetr::RCC_MC_AHB6LPENSETR_SPEC>;
///This register is used by the MCU in order to set the PERxLPEN bit.
pub mod rcc_mc_ahb6lpensetr;
///RCC_MC_AHB6LPENCLRR (rw) register accessor: an alias for `Reg<RCC_MC_AHB6LPENCLRR_SPEC>`
pub type RCC_MC_AHB6LPENCLRR = crate::Reg<rcc_mc_ahb6lpenclrr::RCC_MC_AHB6LPENCLRR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bit
pub mod rcc_mc_ahb6lpenclrr;
///RCC_BR_RSTSCLRR (rw) register accessor: an alias for `Reg<RCC_BR_RSTSCLRR_SPEC>`
pub type RCC_BR_RSTSCLRR = crate::Reg<rcc_br_rstsclrr::RCC_BR_RSTSCLRR_SPEC>;
///This register is used by the BOOTROM to check the reset source. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR). Refer to Section10.3.13: Reset source identification for details.This register except MPUP\[1:0\]RSTF flags is located into VDD domain, and is reset by por_rst reset. The MPUP\[1:0\]RSTF flags are located into VDDCORE and are reset by nreset. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_br_rstsclrr;
///RCC_MP_GRSTCSETR (rw) register accessor: an alias for `Reg<RCC_MP_GRSTCSETR_SPEC>`
pub type RCC_MP_GRSTCSETR = crate::Reg<rcc_mp_grstcsetr::RCC_MP_GRSTCSETR_SPEC>;
///This register is used by the MPU in order to generate either a MCU reset or a system reset or a reset of one of the two MPU processors. Writing has no effect, reading returns the effective values of the corresponding bits. Writing a activates the reset.
pub mod rcc_mp_grstcsetr;
///RCC_MP_RSTSCLRR (rw) register accessor: an alias for `Reg<RCC_MP_RSTSCLRR_SPEC>`
pub type RCC_MP_RSTSCLRR = crate::Reg<rcc_mp_rstsclrr::RCC_MP_RSTSCLRR_SPEC>;
///This register is used by the MPU to check the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mp_rstsclrr;
///RCC_MP_IWDGFZSETR (rw) register accessor: an alias for `Reg<RCC_MP_IWDGFZSETR_SPEC>`
pub type RCC_MP_IWDGFZSETR = crate::Reg<rcc_mp_iwdgfzsetr::RCC_MP_IWDGFZSETR_SPEC>;
///This register is used by the BOOTROM in order to freeze the IWDGs clocks. After a system reset or Standby reset (nreset), or a CStandby reset (cstby_rst) the MPU is allowed to write it once.Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mp_iwdgfzsetr;
///RCC_MP_IWDGFZCLRR (rw) register accessor: an alias for `Reg<RCC_MP_IWDGFZCLRR_SPEC>`
pub type RCC_MP_IWDGFZCLRR = crate::Reg<rcc_mp_iwdgfzclrr::RCC_MP_IWDGFZCLRR_SPEC>;
///This register is used by the BOOTROM in order to unfreeze the IWDGs clocks. Writing has no effect, reading will return the effective values of the corresponding bits. Writing a clears the corresponding bit to . If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mp_iwdgfzclrr;
///RCC_MP_CIER (rw) register accessor: an alias for `Reg<RCC_MP_CIER_SPEC>`
pub type RCC_MP_CIER = crate::Reg<rcc_mp_cier::RCC_MP_CIER_SPEC>;
///This register shall be used by the MPU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mp_cier;
///RCC_MP_CIFR (rw) register accessor: an alias for `Reg<RCC_MP_CIFR_SPEC>`
pub type RCC_MP_CIFR = crate::Reg<rcc_mp_cifr::RCC_MP_CIFR_SPEC>;
///This register shall be used by the MPU in order to read and clear the interrupt flags.Writing has no effect, writing will clear the corresponding flag.Refer to Section10.5: RCC interrupts for more details. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mp_cifr;
///RCC_PWRLPDLYCR (rw) register accessor: an alias for `Reg<RCC_PWRLPDLYCR_SPEC>`
pub type RCC_PWRLPDLYCR = crate::Reg<rcc_pwrlpdlycr::RCC_PWRLPDLYCR_SPEC>;
///This register is used to program the delay between the moment where the system exits from one of the Stop modes, and the moment where it is allowed to enable the PLLs and provide a clock to bridges and processors. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_pwrlpdlycr;
///RCC_MP_RSTSSETR (rw) register accessor: an alias for `Reg<RCC_MP_RSTSSETR_SPEC>`
pub type RCC_MP_RSTSSETR = crate::Reg<rcc_mp_rstssetr::RCC_MP_RSTSSETR_SPEC>;
///This register is dedicated to the BOOTROM code in order to update the reset source. This register is updated by the BOOTROM code, after a power-on reset (por_rst), a system reset (nreset), or an exit from Standby or CStandby. The application software shall not use this register. In order to identify the reset source, the MPU application must use RCC MPU Reset Status Clear Register (RCC_MP_RSTSCLRR), and the MCU application must use the RCC MCU Reset Status Clear Register (RCC_MC_RSTSCLRR).Writing has no effect, reading will return the effective values of the corresponding bits. Writing a sets the corresponding bit to .Refer to Section10.3.13: Reset source identification for details.The register is located in VDDCORE.If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mp_rstssetr;
///RCC_MCO1CFGR (rw) register accessor: an alias for `Reg<RCC_MCO1CFGR_SPEC>`
pub type RCC_MCO1CFGR = crate::Reg<rcc_mco1cfgr::RCC_MCO1CFGR_SPEC>;
///This register is used to select the clock generated on MCO1 output.
pub mod rcc_mco1cfgr;
///RCC_MCO2CFGR (rw) register accessor: an alias for `Reg<RCC_MCO2CFGR_SPEC>`
pub type RCC_MCO2CFGR = crate::Reg<rcc_mco2cfgr::RCC_MCO2CFGR_SPEC>;
///This register is used to select the clock generated on MCO2 output.
pub mod rcc_mco2cfgr;
///RCC_OCRDYR (r) register accessor: an alias for `Reg<RCC_OCRDYR_SPEC>`
pub type RCC_OCRDYR = crate::Reg<rcc_ocrdyr::RCC_OCRDYR_SPEC>;
///This is a read-only access register, It contains the status flags of oscillators. Writing has no effect.
pub mod rcc_ocrdyr;
///RCC_DBGCFGR (rw) register accessor: an alias for `Reg<RCC_DBGCFGR_SPEC>`
pub type RCC_DBGCFGR = crate::Reg<rcc_dbgcfgr::RCC_DBGCFGR_SPEC>;
///This is register contains the enable control of the debug and trace function, and the clock divider for the trace function.
pub mod rcc_dbgcfgr;
///RCC_RCK3SELR (rw) register accessor: an alias for `Reg<RCC_RCK3SELR_SPEC>`
pub type RCC_RCK3SELR = crate::Reg<rcc_rck3selr::RCC_RCK3SELR_SPEC>;
///This register is used to select the reference clock for PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
pub mod rcc_rck3selr;
///RCC_RCK4SELR (rw) register accessor: an alias for `Reg<RCC_RCK4SELR_SPEC>`
pub type RCC_RCK4SELR = crate::Reg<rcc_rck4selr::RCC_RCK4SELR_SPEC>;
///This register is used to select the reference clock for PLL4.
pub mod rcc_rck4selr;
///RCC_TIMG1PRER (rw) register accessor: an alias for `Reg<RCC_TIMG1PRER_SPEC>`
pub type RCC_TIMG1PRER = crate::Reg<rcc_timg1prer::RCC_TIMG1PRER_SPEC>;
///This register is used to control the prescaler value of timers located into APB1 domain. It concerns TIM2, TIM3, TIM4, TIM5, TIM6, TIM7, TIM12, TIM13 and TIM14. Refer to Section: Sub-system clock generation for additional information.
pub mod rcc_timg1prer;
///RCC_TIMG2PRER (rw) register accessor: an alias for `Reg<RCC_TIMG2PRER_SPEC>`
pub type RCC_TIMG2PRER = crate::Reg<rcc_timg2prer::RCC_TIMG2PRER_SPEC>;
///This register is used to control the prescaler value of timers located into APB2 domain. It concerns TIM1, TIM8, TIM15, TIM16, and TIM17. Refer to Section: Sub-system clock generation for additional information.
pub mod rcc_timg2prer;
///RCC_MCUDIVR (rw) register accessor: an alias for `Reg<RCC_MCUDIVR_SPEC>`
pub type RCC_MCUDIVR = crate::Reg<rcc_mcudivr::RCC_MCUDIVR_SPEC>;
///This register is used to control the MCU sub-system clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.
pub mod rcc_mcudivr;
///RCC_APB1DIVR (rw) register accessor: an alias for `Reg<RCC_APB1DIVR_SPEC>`
pub type RCC_APB1DIVR = crate::Reg<rcc_apb1divr::RCC_APB1DIVR_SPEC>;
///This register is used to control the APB1 clock prescaler. Refer to section Section1.4.6.3: Sub-System Clock Generation for additional information.
pub mod rcc_apb1divr;
///RCC_APB2DIVR (rw) register accessor: an alias for `Reg<RCC_APB2DIVR_SPEC>`
pub type RCC_APB2DIVR = crate::Reg<rcc_apb2divr::RCC_APB2DIVR_SPEC>;
///This register is used to control the APB2 clock prescaler. Refer to Section: Sub-system clock generation for additional information.
pub mod rcc_apb2divr;
///RCC_APB3DIVR (rw) register accessor: an alias for `Reg<RCC_APB3DIVR_SPEC>`
pub type RCC_APB3DIVR = crate::Reg<rcc_apb3divr::RCC_APB3DIVR_SPEC>;
///This register is used to control the APB3 clock prescaler. Refer to Section: Sub-system clock generation for additional information.
pub mod rcc_apb3divr;
///RCC_PLL3CR (rw) register accessor: an alias for `Reg<RCC_PLL3CR_SPEC>`
pub type RCC_PLL3CR = crate::Reg<rcc_pll3cr::RCC_PLL3CR_SPEC>;
///This register is used to control the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
pub mod rcc_pll3cr;
///RCC_PLL3CFGR1 (rw) register accessor: an alias for `Reg<RCC_PLL3CFGR1_SPEC>`
pub type RCC_PLL3CFGR1 = crate::Reg<rcc_pll3cfgr1::RCC_PLL3CFGR1_SPEC>;
///This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
pub mod rcc_pll3cfgr1;
///RCC_PLL3CFGR2 (rw) register accessor: an alias for `Reg<RCC_PLL3CFGR2_SPEC>`
pub type RCC_PLL3CFGR2 = crate::Reg<rcc_pll3cfgr2::RCC_PLL3CFGR2_SPEC>;
///This register is used to configure the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
pub mod rcc_pll3cfgr2;
///RCC_PLL3FRACR (rw) register accessor: an alias for `Reg<RCC_PLL3FRACR_SPEC>`
pub type RCC_PLL3FRACR = crate::Reg<rcc_pll3fracr::RCC_PLL3FRACR_SPEC>;
///This register is used to fine-tune the frequency of the PLL3 VCO. If TZEN = MCKPROT = , this register can only be modified in secure mode.
pub mod rcc_pll3fracr;
///RCC_PLL3CSGR (rw) register accessor: an alias for `Reg<RCC_PLL3CSGR_SPEC>`
pub type RCC_PLL3CSGR = crate::Reg<rcc_pll3csgr::RCC_PLL3CSGR_SPEC>;
///This register is used to configure the PLL3.It is not recommended to change the content of this register when the PLL3 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode.
pub mod rcc_pll3csgr;
///RCC_PLL4CR (rw) register accessor: an alias for `Reg<RCC_PLL4CR_SPEC>`
pub type RCC_PLL4CR = crate::Reg<rcc_pll4cr::RCC_PLL4CR_SPEC>;
///This register is used to control the PLL4.
pub mod rcc_pll4cr;
///RCC_PLL4CFGR1 (rw) register accessor: an alias for `Reg<RCC_PLL4CFGR1_SPEC>`
pub type RCC_PLL4CFGR1 = crate::Reg<rcc_pll4cfgr1::RCC_PLL4CFGR1_SPEC>;
///This register is used to configure the PLL4.
pub mod rcc_pll4cfgr1;
///RCC_PLL4CFGR2 (rw) register accessor: an alias for `Reg<RCC_PLL4CFGR2_SPEC>`
pub type RCC_PLL4CFGR2 = crate::Reg<rcc_pll4cfgr2::RCC_PLL4CFGR2_SPEC>;
///This register is used to configure the PLL4.
pub mod rcc_pll4cfgr2;
///RCC_PLL4FRACR (rw) register accessor: an alias for `Reg<RCC_PLL4FRACR_SPEC>`
pub type RCC_PLL4FRACR = crate::Reg<rcc_pll4fracr::RCC_PLL4FRACR_SPEC>;
///This register is used to fine-tune the frequency of the PLL4 VCO.
pub mod rcc_pll4fracr;
///RCC_PLL4CSGR (rw) register accessor: an alias for `Reg<RCC_PLL4CSGR_SPEC>`
pub type RCC_PLL4CSGR = crate::Reg<rcc_pll4csgr::RCC_PLL4CSGR_SPEC>;
///This register is used to configure the PLL4.It is not recommended to change the content of this register when the PLL4 is enabled (PLLON = ). Refer to Section: Using the PLLs in spread spectrum mode for details. If TZEN = MCKPROT = , this register can only be modified in secure mode.
pub mod rcc_pll4csgr;
///RCC_I2C12CKSELR (rw) register accessor: an alias for `Reg<RCC_I2C12CKSELR_SPEC>`
pub type RCC_I2C12CKSELR = crate::Reg<rcc_i2c12ckselr::RCC_I2C12CKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the I2C1 and I2C2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod rcc_i2c12ckselr;
///RCC_I2C35CKSELR (rw) register accessor: an alias for `Reg<RCC_I2C35CKSELR_SPEC>`
pub type RCC_I2C35CKSELR = crate::Reg<rcc_i2c35ckselr::RCC_I2C35CKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the I2C3 and I2C5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod rcc_i2c35ckselr;
///RCC_SAI1CKSELR (rw) register accessor: an alias for `Reg<RCC_SAI1CKSELR_SPEC>`
pub type RCC_SAI1CKSELR = crate::Reg<rcc_sai1ckselr::RCC_SAI1CKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the SAI1 and DFSDM audio clock. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod rcc_sai1ckselr;
///RCC_SAI2CKSELR (rw) register accessor: an alias for `Reg<RCC_SAI2CKSELR_SPEC>`
pub type RCC_SAI2CKSELR = crate::Reg<rcc_sai2ckselr::RCC_SAI2CKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the SAI2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod rcc_sai2ckselr;
///RCC_SAI3CKSELR (rw) register accessor: an alias for `Reg<RCC_SAI3CKSELR_SPEC>`
pub type RCC_SAI3CKSELR = crate::Reg<rcc_sai3ckselr::RCC_SAI3CKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the SAI3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod rcc_sai3ckselr;
///RCC_SAI4CKSELR (rw) register accessor: an alias for `Reg<RCC_SAI4CKSELR_SPEC>`
pub type RCC_SAI4CKSELR = crate::Reg<rcc_sai4ckselr::RCC_SAI4CKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the SAI4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod rcc_sai4ckselr;
///RCC_SPI2S1CKSELR (rw) register accessor: an alias for `Reg<RCC_SPI2S1CKSELR_SPEC>`
pub type RCC_SPI2S1CKSELR = crate::Reg<rcc_spi2s1ckselr::RCC_SPI2S1CKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the SPI/I2S1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod rcc_spi2s1ckselr;
///RCC_SPI2S23CKSELR (rw) register accessor: an alias for `Reg<RCC_SPI2S23CKSELR_SPEC>`
pub type RCC_SPI2S23CKSELR = crate::Reg<rcc_spi2s23ckselr::RCC_SPI2S23CKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the SPI/I2S2,3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod rcc_spi2s23ckselr;
///RCC_SPI45CKSELR (rw) register accessor: an alias for `Reg<RCC_SPI45CKSELR_SPEC>`
pub type RCC_SPI45CKSELR = crate::Reg<rcc_spi45ckselr::RCC_SPI45CKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the SPI4,5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod rcc_spi45ckselr;
///RCC_UART6CKSELR (rw) register accessor: an alias for `Reg<RCC_UART6CKSELR_SPEC>`
pub type RCC_UART6CKSELR = crate::Reg<rcc_uart6ckselr::RCC_UART6CKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the USART6. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod rcc_uart6ckselr;
///RCC_UART24CKSELR (rw) register accessor: an alias for `Reg<RCC_UART24CKSELR_SPEC>`
pub type RCC_UART24CKSELR = crate::Reg<rcc_uart24ckselr::RCC_UART24CKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the USART2 and UART4. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod rcc_uart24ckselr;
///RCC_UART35CKSELR (rw) register accessor: an alias for `Reg<RCC_UART35CKSELR_SPEC>`
pub type RCC_UART35CKSELR = crate::Reg<rcc_uart35ckselr::RCC_UART35CKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the USART3 and UART5. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod rcc_uart35ckselr;
///RCC_UART78CKSELR (rw) register accessor: an alias for `Reg<RCC_UART78CKSELR_SPEC>`
pub type RCC_UART78CKSELR = crate::Reg<rcc_uart78ckselr::RCC_UART78CKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the UART7 and UART8. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod rcc_uart78ckselr;
///RCC_SDMMC12CKSELR (rw) register accessor: an alias for `Reg<RCC_SDMMC12CKSELR_SPEC>`
pub type RCC_SDMMC12CKSELR = crate::Reg<rcc_sdmmc12ckselr::RCC_SDMMC12CKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the SDMMC1 and SDMMC2. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod rcc_sdmmc12ckselr;
///RCC_SDMMC3CKSELR (rw) register accessor: an alias for `Reg<RCC_SDMMC3CKSELR_SPEC>`
pub type RCC_SDMMC3CKSELR = crate::Reg<rcc_sdmmc3ckselr::RCC_SDMMC3CKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the SDMMC3. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod rcc_sdmmc3ckselr;
///RCC_ETHCKSELR (rw) register accessor: an alias for `Reg<RCC_ETHCKSELR_SPEC>`
pub type RCC_ETHCKSELR = crate::Reg<rcc_ethckselr::RCC_ETHCKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the ETH block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod rcc_ethckselr;
///RCC_QSPICKSELR (rw) register accessor: an alias for `Reg<RCC_QSPICKSELR_SPEC>`
pub type RCC_QSPICKSELR = crate::Reg<rcc_qspickselr::RCC_QSPICKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the QUADSPI. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod rcc_qspickselr;
///RCC_FMCCKSELR (rw) register accessor: an alias for `Reg<RCC_FMCCKSELR_SPEC>`
pub type RCC_FMCCKSELR = crate::Reg<rcc_fmcckselr::RCC_FMCCKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the FMC block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod rcc_fmcckselr;
///RCC_FDCANCKSELR (rw) register accessor: an alias for `Reg<RCC_FDCANCKSELR_SPEC>`
pub type RCC_FDCANCKSELR = crate::Reg<rcc_fdcanckselr::RCC_FDCANCKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the FDCAN block. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod rcc_fdcanckselr;
///RCC_SPDIFCKSELR (rw) register accessor: an alias for `Reg<RCC_SPDIFCKSELR_SPEC>`
pub type RCC_SPDIFCKSELR = crate::Reg<rcc_spdifckselr::RCC_SPDIFCKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the SPDIFRX. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
pub mod rcc_spdifckselr;
///RCC_CECCKSELR (rw) register accessor: an alias for `Reg<RCC_CECCKSELR_SPEC>`
pub type RCC_CECCKSELR = crate::Reg<rcc_cecckselr::RCC_CECCKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the CEC-HDMI.
pub mod rcc_cecckselr;
///RCC_USBCKSELR (rw) register accessor: an alias for `Reg<RCC_USBCKSELR_SPEC>`
pub type RCC_USBCKSELR = crate::Reg<rcc_usbckselr::RCC_USBCKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the USBPHY PLL of the USB HOST and USB OTG
pub mod rcc_usbckselr;
///RCC_RNG2CKSELR (rw) register accessor: an alias for `Reg<RCC_RNG2CKSELR_SPEC>`
pub type RCC_RNG2CKSELR = crate::Reg<rcc_rng2ckselr::RCC_RNG2CKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the RNG2.
pub mod rcc_rng2ckselr;
///RCC_DSICKSELR (rw) register accessor: an alias for `Reg<RCC_DSICKSELR_SPEC>`
pub type RCC_DSICKSELR = crate::Reg<rcc_dsickselr::RCC_DSICKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the DSI block.
pub mod rcc_dsickselr;
///RCC_ADCCKSELR (rw) register accessor: an alias for `Reg<RCC_ADCCKSELR_SPEC>`
pub type RCC_ADCCKSELR = crate::Reg<rcc_adcckselr::RCC_ADCCKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the ADC block.
pub mod rcc_adcckselr;
///RCC_LPTIM45CKSELR (rw) register accessor: an alias for `Reg<RCC_LPTIM45CKSELR_SPEC>`
pub type RCC_LPTIM45CKSELR = crate::Reg<rcc_lptim45ckselr::RCC_LPTIM45CKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the LPTIM4 and LPTIM5 blocks.
pub mod rcc_lptim45ckselr;
///RCC_LPTIM23CKSELR (rw) register accessor: an alias for `Reg<RCC_LPTIM23CKSELR_SPEC>`
pub type RCC_LPTIM23CKSELR = crate::Reg<rcc_lptim23ckselr::RCC_LPTIM23CKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the LPTIM2 and LPTIM3 blocks.
pub mod rcc_lptim23ckselr;
///RCC_LPTIM1CKSELR (rw) register accessor: an alias for `Reg<RCC_LPTIM1CKSELR_SPEC>`
pub type RCC_LPTIM1CKSELR = crate::Reg<rcc_lptim1ckselr::RCC_LPTIM1CKSELR_SPEC>;
///This register is used to control the selection of the kernel clock for the LPTIM1 block.
pub mod rcc_lptim1ckselr;
///RCC_APB1RSTSETR (rw) register accessor: an alias for `Reg<RCC_APB1RSTSETR_SPEC>`
pub type RCC_APB1RSTSETR = crate::Reg<rcc_apb1rstsetr::RCC_APB1RSTSETR_SPEC>;
///This register is used to activate the reset of the corresponding peripheral.
pub mod rcc_apb1rstsetr;
///RCC_APB1RSTCLRR (rw) register accessor: an alias for `Reg<RCC_APB1RSTCLRR_SPEC>`
pub type RCC_APB1RSTCLRR = crate::Reg<rcc_apb1rstclrr::RCC_APB1RSTCLRR_SPEC>;
///This register is used to release the reset of the corresponding peripheral.
pub mod rcc_apb1rstclrr;
///RCC_APB2RSTSETR (rw) register accessor: an alias for `Reg<RCC_APB2RSTSETR_SPEC>`
pub type RCC_APB2RSTSETR = crate::Reg<rcc_apb2rstsetr::RCC_APB2RSTSETR_SPEC>;
///This register is used to activate the reset of the corresponding peripheral.
pub mod rcc_apb2rstsetr;
///RCC_APB2RSTCLRR (rw) register accessor: an alias for `Reg<RCC_APB2RSTCLRR_SPEC>`
pub type RCC_APB2RSTCLRR = crate::Reg<rcc_apb2rstclrr::RCC_APB2RSTCLRR_SPEC>;
///This register is used to release the reset of the corresponding peripheral.
pub mod rcc_apb2rstclrr;
///RCC_APB3RSTSETR (rw) register accessor: an alias for `Reg<RCC_APB3RSTSETR_SPEC>`
pub type RCC_APB3RSTSETR = crate::Reg<rcc_apb3rstsetr::RCC_APB3RSTSETR_SPEC>;
///This register is used to activate the reset of the corresponding peripheral.
pub mod rcc_apb3rstsetr;
///RCC_APB3RSTCLRR (rw) register accessor: an alias for `Reg<RCC_APB3RSTCLRR_SPEC>`
pub type RCC_APB3RSTCLRR = crate::Reg<rcc_apb3rstclrr::RCC_APB3RSTCLRR_SPEC>;
///This register is used to release the reset of the corresponding peripheral.
pub mod rcc_apb3rstclrr;
///RCC_AHB2RSTSETR (rw) register accessor: an alias for `Reg<RCC_AHB2RSTSETR_SPEC>`
pub type RCC_AHB2RSTSETR = crate::Reg<rcc_ahb2rstsetr::RCC_AHB2RSTSETR_SPEC>;
///This register is used to activate the reset of the corresponding peripheral.
pub mod rcc_ahb2rstsetr;
///RCC_AHB2RSTCLRR (rw) register accessor: an alias for `Reg<RCC_AHB2RSTCLRR_SPEC>`
pub type RCC_AHB2RSTCLRR = crate::Reg<rcc_ahb2rstclrr::RCC_AHB2RSTCLRR_SPEC>;
///This register is used to release the reset of the corresponding peripheral.
pub mod rcc_ahb2rstclrr;
///RCC_AHB3RSTSETR (rw) register accessor: an alias for `Reg<RCC_AHB3RSTSETR_SPEC>`
pub type RCC_AHB3RSTSETR = crate::Reg<rcc_ahb3rstsetr::RCC_AHB3RSTSETR_SPEC>;
///This register is used to activate the reset of the corresponding peripheral.
pub mod rcc_ahb3rstsetr;
///RCC_AHB3RSTCLRR (rw) register accessor: an alias for `Reg<RCC_AHB3RSTCLRR_SPEC>`
pub type RCC_AHB3RSTCLRR = crate::Reg<rcc_ahb3rstclrr::RCC_AHB3RSTCLRR_SPEC>;
///This register is used to release the reset of the corresponding peripheral.
pub mod rcc_ahb3rstclrr;
///RCC_AHB4RSTSETR (rw) register accessor: an alias for `Reg<RCC_AHB4RSTSETR_SPEC>`
pub type RCC_AHB4RSTSETR = crate::Reg<rcc_ahb4rstsetr::RCC_AHB4RSTSETR_SPEC>;
///This register is used to activate the reset of the corresponding peripheral
pub mod rcc_ahb4rstsetr;
///RCC_AHB4RSTCLRR (rw) register accessor: an alias for `Reg<RCC_AHB4RSTCLRR_SPEC>`
pub type RCC_AHB4RSTCLRR = crate::Reg<rcc_ahb4rstclrr::RCC_AHB4RSTCLRR_SPEC>;
///This register is used to release the reset of the corresponding peripheral.
pub mod rcc_ahb4rstclrr;
///RCC_MP_APB1ENSETR (rw) register accessor: an alias for `Reg<RCC_MP_APB1ENSETR_SPEC>`
pub type RCC_MP_APB1ENSETR = crate::Reg<rcc_mp_apb1ensetr::RCC_MP_APB1ENSETR_SPEC>;
///This register is used to set the peripheral clock enable bit
pub mod rcc_mp_apb1ensetr;
///RCC_MP_APB1ENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_APB1ENCLRR_SPEC>`
pub type RCC_MP_APB1ENCLRR = crate::Reg<rcc_mp_apb1enclrr::RCC_MP_APB1ENCLRR_SPEC>;
///This register is used to clear the peripheral clock enable bit
pub mod rcc_mp_apb1enclrr;
///RCC_MP_APB2ENSETR (rw) register accessor: an alias for `Reg<RCC_MP_APB2ENSETR_SPEC>`
pub type RCC_MP_APB2ENSETR = crate::Reg<rcc_mp_apb2ensetr::RCC_MP_APB2ENSETR_SPEC>;
///This register is used to set the peripheral clock enable bit
pub mod rcc_mp_apb2ensetr;
///RCC_MP_APB2ENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_APB2ENCLRR_SPEC>`
pub type RCC_MP_APB2ENCLRR = crate::Reg<rcc_mp_apb2enclrr::RCC_MP_APB2ENCLRR_SPEC>;
///This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
pub mod rcc_mp_apb2enclrr;
///RCC_MP_APB3ENSETR (rw) register accessor: an alias for `Reg<RCC_MP_APB3ENSETR_SPEC>`
pub type RCC_MP_APB3ENSETR = crate::Reg<rcc_mp_apb3ensetr::RCC_MP_APB3ENSETR_SPEC>;
///This register is used to set the peripheral clock enable bit
pub mod rcc_mp_apb3ensetr;
///RCC_MP_APB3ENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_APB3ENCLRR_SPEC>`
pub type RCC_MP_APB3ENCLRR = crate::Reg<rcc_mp_apb3enclrr::RCC_MP_APB3ENCLRR_SPEC>;
///This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
pub mod rcc_mp_apb3enclrr;
///RCC_MP_AHB2ENSETR (rw) register accessor: an alias for `Reg<RCC_MP_AHB2ENSETR_SPEC>`
pub type RCC_MP_AHB2ENSETR = crate::Reg<rcc_mp_ahb2ensetr::RCC_MP_AHB2ENSETR_SPEC>;
///This register is used to set the peripheral clock enable bit of the corresponding peripheral
pub mod rcc_mp_ahb2ensetr;
///RCC_MP_AHB2ENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_AHB2ENCLRR_SPEC>`
pub type RCC_MP_AHB2ENCLRR = crate::Reg<rcc_mp_ahb2enclrr::RCC_MP_AHB2ENCLRR_SPEC>;
///This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
pub mod rcc_mp_ahb2enclrr;
///RCC_MP_AHB3ENSETR (rw) register accessor: an alias for `Reg<RCC_MP_AHB3ENSETR_SPEC>`
pub type RCC_MP_AHB3ENSETR = crate::Reg<rcc_mp_ahb3ensetr::RCC_MP_AHB3ENSETR_SPEC>;
///This register is used to set the peripheral clock enable bit of the corresponding peripheral
pub mod rcc_mp_ahb3ensetr;
///RCC_MP_AHB3ENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_AHB3ENCLRR_SPEC>`
pub type RCC_MP_AHB3ENCLRR = crate::Reg<rcc_mp_ahb3enclrr::RCC_MP_AHB3ENCLRR_SPEC>;
///This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
pub mod rcc_mp_ahb3enclrr;
///RCC_MP_AHB4ENSETR (rw) register accessor: an alias for `Reg<RCC_MP_AHB4ENSETR_SPEC>`
pub type RCC_MP_AHB4ENSETR = crate::Reg<rcc_mp_ahb4ensetr::RCC_MP_AHB4ENSETR_SPEC>;
///This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MPU.
pub mod rcc_mp_ahb4ensetr;
///RCC_MP_AHB4ENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_AHB4ENCLRR_SPEC>`
pub type RCC_MP_AHB4ENCLRR = crate::Reg<rcc_mp_ahb4enclrr::RCC_MP_AHB4ENCLRR_SPEC>;
///This register is used to clear the peripheral clock enable bit
pub mod rcc_mp_ahb4enclrr;
///RCC_MP_MLAHBENSETR (rw) register accessor: an alias for `Reg<RCC_MP_MLAHBENSETR_SPEC>`
pub type RCC_MP_MLAHBENSETR = crate::Reg<rcc_mp_mlahbensetr::RCC_MP_MLAHBENSETR_SPEC>;
///This register is used to set the peripheral clock enable bit
pub mod rcc_mp_mlahbensetr;
///RCC_MP_MLAHBENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_MLAHBENCLRR_SPEC>`
pub type RCC_MP_MLAHBENCLRR = crate::Reg<rcc_mp_mlahbenclrr::RCC_MP_MLAHBENCLRR_SPEC>;
///This register is used to clear the peripheral clock enable bit.
pub mod rcc_mp_mlahbenclrr;
///RCC_MC_APB1ENSETR (rw) register accessor: an alias for `Reg<RCC_MC_APB1ENSETR_SPEC>`
pub type RCC_MC_APB1ENSETR = crate::Reg<rcc_mc_apb1ensetr::RCC_MC_APB1ENSETR_SPEC>;
///This register is used to set the peripheral clock enable bit of the corresponding peripheral to . It shall be used to allocate a peripheral to the MCU. Writing has no effect, reading will return . Writing a sets the corresponding bit to .
pub mod rcc_mc_apb1ensetr;
///RCC_MC_APB1ENCLRR (rw) register accessor: an alias for `Reg<RCC_MC_APB1ENCLRR_SPEC>`
pub type RCC_MC_APB1ENCLRR = crate::Reg<rcc_mc_apb1enclrr::RCC_MC_APB1ENCLRR_SPEC>;
///This register is used to clear the peripheral clock enable bit of the corresponding peripheral.
pub mod rcc_mc_apb1enclrr;
///RCC_MC_APB2ENSETR (rw) register accessor: an alias for `Reg<RCC_MC_APB2ENSETR_SPEC>`
pub type RCC_MC_APB2ENSETR = crate::Reg<rcc_mc_apb2ensetr::RCC_MC_APB2ENSETR_SPEC>;
///This register is used to set the peripheral clock enable bit
pub mod rcc_mc_apb2ensetr;
///RCC_MC_APB2ENCLRR (rw) register accessor: an alias for `Reg<RCC_MC_APB2ENCLRR_SPEC>`
pub type RCC_MC_APB2ENCLRR = crate::Reg<rcc_mc_apb2enclrr::RCC_MC_APB2ENCLRR_SPEC>;
///This register is used to clear the peripheral clock enable bit
pub mod rcc_mc_apb2enclrr;
///RCC_MC_APB3ENSETR (rw) register accessor: an alias for `Reg<RCC_MC_APB3ENSETR_SPEC>`
pub type RCC_MC_APB3ENSETR = crate::Reg<rcc_mc_apb3ensetr::RCC_MC_APB3ENSETR_SPEC>;
///This register is used to set the peripheral clock enable bit
pub mod rcc_mc_apb3ensetr;
///RCC_MC_APB3ENCLRR (rw) register accessor: an alias for `Reg<RCC_MC_APB3ENCLRR_SPEC>`
pub type RCC_MC_APB3ENCLRR = crate::Reg<rcc_mc_apb3enclrr::RCC_MC_APB3ENCLRR_SPEC>;
///This register is used to clear the peripheral clock enable bit
pub mod rcc_mc_apb3enclrr;
///RCC_MC_AHB2ENSETR (rw) register accessor: an alias for `Reg<RCC_MC_AHB2ENSETR_SPEC>`
pub type RCC_MC_AHB2ENSETR = crate::Reg<rcc_mc_ahb2ensetr::RCC_MC_AHB2ENSETR_SPEC>;
///This register is used to set the peripheral clock enable bit
pub mod rcc_mc_ahb2ensetr;
///RCC_MC_AHB2ENCLRR (rw) register accessor: an alias for `Reg<RCC_MC_AHB2ENCLRR_SPEC>`
pub type RCC_MC_AHB2ENCLRR = crate::Reg<rcc_mc_ahb2enclrr::RCC_MC_AHB2ENCLRR_SPEC>;
///This register is used to clear the peripheral clock enable bit
pub mod rcc_mc_ahb2enclrr;
///RCC_MC_AHB3ENSETR (rw) register accessor: an alias for `Reg<RCC_MC_AHB3ENSETR_SPEC>`
pub type RCC_MC_AHB3ENSETR = crate::Reg<rcc_mc_ahb3ensetr::RCC_MC_AHB3ENSETR_SPEC>;
///This register is used to set the peripheral clock enable bit
pub mod rcc_mc_ahb3ensetr;
///RCC_MC_AHB3ENCLRR (rw) register accessor: an alias for `Reg<RCC_MC_AHB3ENCLRR_SPEC>`
pub type RCC_MC_AHB3ENCLRR = crate::Reg<rcc_mc_ahb3enclrr::RCC_MC_AHB3ENCLRR_SPEC>;
///This register is used to clear the peripheral clock enable bit
pub mod rcc_mc_ahb3enclrr;
///RCC_MC_AHB4ENSETR (rw) register accessor: an alias for `Reg<RCC_MC_AHB4ENSETR_SPEC>`
pub type RCC_MC_AHB4ENSETR = crate::Reg<rcc_mc_ahb4ensetr::RCC_MC_AHB4ENSETR_SPEC>;
///This register is used to set the peripheral clock enable bit
pub mod rcc_mc_ahb4ensetr;
///RCC_MC_AHB4ENCLRR (rw) register accessor: an alias for `Reg<RCC_MC_AHB4ENCLRR_SPEC>`
pub type RCC_MC_AHB4ENCLRR = crate::Reg<rcc_mc_ahb4enclrr::RCC_MC_AHB4ENCLRR_SPEC>;
///This register is used to clear the peripheral clock enable bit
pub mod rcc_mc_ahb4enclrr;
///RCC_MC_AXIMENSETR (rw) register accessor: an alias for `Reg<RCC_MC_AXIMENSETR_SPEC>`
pub type RCC_MC_AXIMENSETR = crate::Reg<rcc_mc_aximensetr::RCC_MC_AXIMENSETR_SPEC>;
///This register is used to set the peripheral clock enable bit
pub mod rcc_mc_aximensetr;
///RCC_MC_AXIMENCLRR (rw) register accessor: an alias for `Reg<RCC_MC_AXIMENCLRR_SPEC>`
pub type RCC_MC_AXIMENCLRR = crate::Reg<rcc_mc_aximenclrr::RCC_MC_AXIMENCLRR_SPEC>;
///This register is used to clear the peripheral clock enable bit
pub mod rcc_mc_aximenclrr;
///RCC_MC_MLAHBENSETR (rw) register accessor: an alias for `Reg<RCC_MC_MLAHBENSETR_SPEC>`
pub type RCC_MC_MLAHBENSETR = crate::Reg<rcc_mc_mlahbensetr::RCC_MC_MLAHBENSETR_SPEC>;
///This register is used to set the peripheral clock enable bit
pub mod rcc_mc_mlahbensetr;
///RCC_MC_MLAHBENCLRR (rw) register accessor: an alias for `Reg<RCC_MC_MLAHBENCLRR_SPEC>`
pub type RCC_MC_MLAHBENCLRR = crate::Reg<rcc_mc_mlahbenclrr::RCC_MC_MLAHBENCLRR_SPEC>;
///This register is used to clear the peripheral clock enable bit
pub mod rcc_mc_mlahbenclrr;
///RCC_MP_APB1LPENSETR (rw) register accessor: an alias for `Reg<RCC_MP_APB1LPENSETR_SPEC>`
pub type RCC_MP_APB1LPENSETR = crate::Reg<rcc_mp_apb1lpensetr::RCC_MP_APB1LPENSETR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bits
pub mod rcc_mp_apb1lpensetr;
///RCC_MP_APB1LPENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_APB1LPENCLRR_SPEC>`
pub type RCC_MP_APB1LPENCLRR = crate::Reg<rcc_mp_apb1lpenclrr::RCC_MP_APB1LPENCLRR_SPEC>;
///This register is used by the MPU in order to clear the PERxLPEN bits .
pub mod rcc_mp_apb1lpenclrr;
///RCC_MP_APB2LPENSETR (rw) register accessor: an alias for `Reg<RCC_MP_APB2LPENSETR_SPEC>`
pub type RCC_MP_APB2LPENSETR = crate::Reg<rcc_mp_apb2lpensetr::RCC_MP_APB2LPENSETR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bits
pub mod rcc_mp_apb2lpensetr;
///RCC_MP_APB2LPENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_APB2LPENCLRR_SPEC>`
pub type RCC_MP_APB2LPENCLRR = crate::Reg<rcc_mp_apb2lpenclrr::RCC_MP_APB2LPENCLRR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bits
pub mod rcc_mp_apb2lpenclrr;
///RCC_MP_APB3LPENSETR (rw) register accessor: an alias for `Reg<RCC_MP_APB3LPENSETR_SPEC>`
pub type RCC_MP_APB3LPENSETR = crate::Reg<rcc_mp_apb3lpensetr::RCC_MP_APB3LPENSETR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bits
pub mod rcc_mp_apb3lpensetr;
///RCC_MP_APB3LPENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_APB3LPENCLRR_SPEC>`
pub type RCC_MP_APB3LPENCLRR = crate::Reg<rcc_mp_apb3lpenclrr::RCC_MP_APB3LPENCLRR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bits
pub mod rcc_mp_apb3lpenclrr;
///RCC_MP_AHB2LPENSETR (rw) register accessor: an alias for `Reg<RCC_MP_AHB2LPENSETR_SPEC>`
pub type RCC_MP_AHB2LPENSETR = crate::Reg<rcc_mp_ahb2lpensetr::RCC_MP_AHB2LPENSETR_SPEC>;
///This register is used by the MPU in order to set the PERxLPEN bit.
pub mod rcc_mp_ahb2lpensetr;
///RCC_MP_AHB2LPENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_AHB2LPENCLRR_SPEC>`
pub type RCC_MP_AHB2LPENCLRR = crate::Reg<rcc_mp_ahb2lpenclrr::RCC_MP_AHB2LPENCLRR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bits
pub mod rcc_mp_ahb2lpenclrr;
///RCC_MP_AHB3LPENSETR (rw) register accessor: an alias for `Reg<RCC_MP_AHB3LPENSETR_SPEC>`
pub type RCC_MP_AHB3LPENSETR = crate::Reg<rcc_mp_ahb3lpensetr::RCC_MP_AHB3LPENSETR_SPEC>;
///This register is used by the MPU
pub mod rcc_mp_ahb3lpensetr;
///RCC_MP_AHB3LPENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_AHB3LPENCLRR_SPEC>`
pub type RCC_MP_AHB3LPENCLRR = crate::Reg<rcc_mp_ahb3lpenclrr::RCC_MP_AHB3LPENCLRR_SPEC>;
///This register is used by the MPU in order to clear the PERxLPEN bit
pub mod rcc_mp_ahb3lpenclrr;
///RCC_MP_AHB4LPENSETR (rw) register accessor: an alias for `Reg<RCC_MP_AHB4LPENSETR_SPEC>`
pub type RCC_MP_AHB4LPENSETR = crate::Reg<rcc_mp_ahb4lpensetr::RCC_MP_AHB4LPENSETR_SPEC>;
///This register is used by the MPU
pub mod rcc_mp_ahb4lpensetr;
///RCC_MP_AHB4LPENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_AHB4LPENCLRR_SPEC>`
pub type RCC_MP_AHB4LPENCLRR = crate::Reg<rcc_mp_ahb4lpenclrr::RCC_MP_AHB4LPENCLRR_SPEC>;
///This register is used by the MPU
pub mod rcc_mp_ahb4lpenclrr;
///RCC_MP_AXIMLPENSETR (rw) register accessor: an alias for `Reg<RCC_MP_AXIMLPENSETR_SPEC>`
pub type RCC_MP_AXIMLPENSETR = crate::Reg<rcc_mp_aximlpensetr::RCC_MP_AXIMLPENSETR_SPEC>;
///This register is used by the MPU
pub mod rcc_mp_aximlpensetr;
///RCC_MP_AXIMLPENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_AXIMLPENCLRR_SPEC>`
pub type RCC_MP_AXIMLPENCLRR = crate::Reg<rcc_mp_aximlpenclrr::RCC_MP_AXIMLPENCLRR_SPEC>;
///This register is used by the MPU
pub mod rcc_mp_aximlpenclrr;
///RCC_MP_MLAHBLPENSETR (rw) register accessor: an alias for `Reg<RCC_MP_MLAHBLPENSETR_SPEC>`
pub type RCC_MP_MLAHBLPENSETR = crate::Reg<rcc_mp_mlahblpensetr::RCC_MP_MLAHBLPENSETR_SPEC>;
///This register is used by the MPU in order to set the PERxLPEN bit
pub mod rcc_mp_mlahblpensetr;
///RCC_MP_MLAHBLPENCLRR (rw) register accessor: an alias for `Reg<RCC_MP_MLAHBLPENCLRR_SPEC>`
pub type RCC_MP_MLAHBLPENCLRR = crate::Reg<rcc_mp_mlahblpenclrr::RCC_MP_MLAHBLPENCLRR_SPEC>;
///This register is used by the MPU in order to clear the PERxLPEN bit
pub mod rcc_mp_mlahblpenclrr;
///RCC_MC_APB1LPENSETR (rw) register accessor: an alias for `Reg<RCC_MC_APB1LPENSETR_SPEC>`
pub type RCC_MC_APB1LPENSETR = crate::Reg<rcc_mc_apb1lpensetr::RCC_MC_APB1LPENSETR_SPEC>;
///This register is used by the MCU in order to set the PERxLPEN bit.
pub mod rcc_mc_apb1lpensetr;
///RCC_MC_APB1LPENCLRR (rw) register accessor: an alias for `Reg<RCC_MC_APB1LPENCLRR_SPEC>`
pub type RCC_MC_APB1LPENCLRR = crate::Reg<rcc_mc_apb1lpenclrr::RCC_MC_APB1LPENCLRR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bits
pub mod rcc_mc_apb1lpenclrr;
///RCC_MC_APB2LPENSETR (rw) register accessor: an alias for `Reg<RCC_MC_APB2LPENSETR_SPEC>`
pub type RCC_MC_APB2LPENSETR = crate::Reg<rcc_mc_apb2lpensetr::RCC_MC_APB2LPENSETR_SPEC>;
///This register is used by the MCU in order to set the PERxLPEN bit.
pub mod rcc_mc_apb2lpensetr;
///RCC_MC_APB2LPENCLRR (rw) register accessor: an alias for `Reg<RCC_MC_APB2LPENCLRR_SPEC>`
pub type RCC_MC_APB2LPENCLRR = crate::Reg<rcc_mc_apb2lpenclrr::RCC_MC_APB2LPENCLRR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bit
pub mod rcc_mc_apb2lpenclrr;
///RCC_MC_APB3LPENSETR (rw) register accessor: an alias for `Reg<RCC_MC_APB3LPENSETR_SPEC>`
pub type RCC_MC_APB3LPENSETR = crate::Reg<rcc_mc_apb3lpensetr::RCC_MC_APB3LPENSETR_SPEC>;
///This register is used by the MCU in order to set the PERxLPEN bit.
pub mod rcc_mc_apb3lpensetr;
///RCC_MC_APB3LPENCLRR (rw) register accessor: an alias for `Reg<RCC_MC_APB3LPENCLRR_SPEC>`
pub type RCC_MC_APB3LPENCLRR = crate::Reg<rcc_mc_apb3lpenclrr::RCC_MC_APB3LPENCLRR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bit
pub mod rcc_mc_apb3lpenclrr;
///RCC_MC_AHB2LPENSETR (rw) register accessor: an alias for `Reg<RCC_MC_AHB2LPENSETR_SPEC>`
pub type RCC_MC_AHB2LPENSETR = crate::Reg<rcc_mc_ahb2lpensetr::RCC_MC_AHB2LPENSETR_SPEC>;
///This register is used by the MCU in order to set the PERxLPEN bit.
pub mod rcc_mc_ahb2lpensetr;
///RCC_MC_AHB2LPENCLRR (rw) register accessor: an alias for `Reg<RCC_MC_AHB2LPENCLRR_SPEC>`
pub type RCC_MC_AHB2LPENCLRR = crate::Reg<rcc_mc_ahb2lpenclrr::RCC_MC_AHB2LPENCLRR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bit
pub mod rcc_mc_ahb2lpenclrr;
///RCC_MC_AHB3LPENSETR (rw) register accessor: an alias for `Reg<RCC_MC_AHB3LPENSETR_SPEC>`
pub type RCC_MC_AHB3LPENSETR = crate::Reg<rcc_mc_ahb3lpensetr::RCC_MC_AHB3LPENSETR_SPEC>;
///This register is used by the MCU in order to set the PERxLPEN bit.
pub mod rcc_mc_ahb3lpensetr;
///RCC_MC_AHB3LPENCLRR (rw) register accessor: an alias for `Reg<RCC_MC_AHB3LPENCLRR_SPEC>`
pub type RCC_MC_AHB3LPENCLRR = crate::Reg<rcc_mc_ahb3lpenclrr::RCC_MC_AHB3LPENCLRR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bit
pub mod rcc_mc_ahb3lpenclrr;
///RCC_MC_AHB4LPENSETR (rw) register accessor: an alias for `Reg<RCC_MC_AHB4LPENSETR_SPEC>`
pub type RCC_MC_AHB4LPENSETR = crate::Reg<rcc_mc_ahb4lpensetr::RCC_MC_AHB4LPENSETR_SPEC>;
///This register is used by the MCU in order to set the PERxLPEN bit.
pub mod rcc_mc_ahb4lpensetr;
///RCC_MC_AHB4LPENCLRR (rw) register accessor: an alias for `Reg<RCC_MC_AHB4LPENCLRR_SPEC>`
pub type RCC_MC_AHB4LPENCLRR = crate::Reg<rcc_mc_ahb4lpenclrr::RCC_MC_AHB4LPENCLRR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.
pub mod rcc_mc_ahb4lpenclrr;
///RCC_MC_AXIMLPENSETR (rw) register accessor: an alias for `Reg<RCC_MC_AXIMLPENSETR_SPEC>`
pub type RCC_MC_AXIMLPENSETR = crate::Reg<rcc_mc_aximlpensetr::RCC_MC_AXIMLPENSETR_SPEC>;
///This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral.
pub mod rcc_mc_aximlpensetr;
///RCC_MC_AXIMLPENCLRR (rw) register accessor: an alias for `Reg<RCC_MC_AXIMLPENCLRR_SPEC>`
pub type RCC_MC_AXIMLPENCLRR = crate::Reg<rcc_mc_aximlpenclrr::RCC_MC_AXIMLPENCLRR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.
pub mod rcc_mc_aximlpenclrr;
///RCC_MC_MLAHBLPENSETR (rw) register accessor: an alias for `Reg<RCC_MC_MLAHBLPENSETR_SPEC>`
pub type RCC_MC_MLAHBLPENSETR = crate::Reg<rcc_mc_mlahblpensetr::RCC_MC_MLAHBLPENSETR_SPEC>;
///This register is used by the MCU in order to set the PERxLPEN bit of the corresponding peripheral.
pub mod rcc_mc_mlahblpensetr;
///RCC_MC_MLAHBLPENCLRR (rw) register accessor: an alias for `Reg<RCC_MC_MLAHBLPENCLRR_SPEC>`
pub type RCC_MC_MLAHBLPENCLRR = crate::Reg<rcc_mc_mlahblpenclrr::RCC_MC_MLAHBLPENCLRR_SPEC>;
///This register is used by the MCU in order to clear the PERxLPEN bit of the corresponding peripheral.
pub mod rcc_mc_mlahblpenclrr;
///RCC_MC_RSTSCLRR (rw) register accessor: an alias for `Reg<RCC_MC_RSTSCLRR_SPEC>`
pub type RCC_MC_RSTSCLRR = crate::Reg<rcc_mc_rstsclrr::RCC_MC_RSTSCLRR_SPEC>;
///This register is used by the MCU to check the reset source.
pub mod rcc_mc_rstsclrr;
///RCC_MC_CIER (rw) register accessor: an alias for `Reg<RCC_MC_CIER_SPEC>`
pub type RCC_MC_CIER = crate::Reg<rcc_mc_cier::RCC_MC_CIER_SPEC>;
///This register shall be used by the MCU to control the interrupt source enable. Refer to Section10.5: RCC interrupts for more details.
pub mod rcc_mc_cier;
///RCC_MC_CIFR (rw) register accessor: an alias for `Reg<RCC_MC_CIFR_SPEC>`
pub type RCC_MC_CIFR = crate::Reg<rcc_mc_cifr::RCC_MC_CIFR_SPEC>;
///This register shall be used by the MCU in order to read and clear the interrupt flags.
pub mod rcc_mc_cifr;
///RCC_VERR (r) register accessor: an alias for `Reg<RCC_VERR_SPEC>`
pub type RCC_VERR = crate::Reg<rcc_verr::RCC_VERR_SPEC>;
///This register gives the IP version
pub mod rcc_verr;
///RCC_IDR (r) register accessor: an alias for `Reg<RCC_IDR_SPEC>`
pub type RCC_IDR = crate::Reg<rcc_idr::RCC_IDR_SPEC>;
///This register gives the unique identifier of the RCC
pub mod rcc_idr;
///RCC_SIDR (r) register accessor: an alias for `Reg<RCC_SIDR_SPEC>`
pub type RCC_SIDR = crate::Reg<rcc_sidr::RCC_SIDR_SPEC>;
///This register gives the decoding space, which is for the RCC of 4 kB.
pub mod rcc_sidr;
