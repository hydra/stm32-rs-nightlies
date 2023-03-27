///Register `RCC_CCIPR1` reader
pub struct R(crate::R<RCC_CCIPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CCIPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CCIPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CCIPR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_CCIPR1` writer
pub struct W(crate::W<RCC_CCIPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CCIPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RCC_CCIPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CCIPR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `USART1SEL` reader - USART1 kernel clock source selection This bits are used to select the USART1 kernel clock source. Note: The USART1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
pub type USART1SEL_R = crate::FieldReader<u8, u8>;
///Field `USART1SEL` writer - USART1 kernel clock source selection This bits are used to select the USART1 kernel clock source. Note: The USART1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
pub type USART1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR1_SPEC, u8, u8, 2, O>;
///Field `USART2SEL` reader - USART2 kernel clock source selection This bits are used to select the USART2 kernel clock source. Note: The USART2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
pub type USART2SEL_R = crate::FieldReader<u8, u8>;
///Field `USART2SEL` writer - USART2 kernel clock source selection This bits are used to select the USART2 kernel clock source. Note: The USART2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
pub type USART2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR1_SPEC, u8, u8, 2, O>;
///Field `USART3SEL` reader - USART3 kernel clock source selection This bits are used to select the USART3 kernel clock source. Note: The USART3 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
pub type USART3SEL_R = crate::FieldReader<u8, u8>;
///Field `USART3SEL` writer - USART3 kernel clock source selection This bits are used to select the USART3 kernel clock source. Note: The USART3 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
pub type USART3SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR1_SPEC, u8, u8, 2, O>;
///Field `UART4SEL` reader - UART4 kernel clock source selection This bits are used to select the UART4 kernel clock source. Note: The UART4 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
pub type UART4SEL_R = crate::FieldReader<u8, u8>;
///Field `UART4SEL` writer - UART4 kernel clock source selection This bits are used to select the UART4 kernel clock source. Note: The UART4 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
pub type UART4SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR1_SPEC, u8, u8, 2, O>;
///Field `UART5SEL` reader - UART5 kernel clock source selection These bits are used to select the UART5 kernel clock source. Note: The UART5 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
pub type UART5SEL_R = crate::FieldReader<u8, u8>;
///Field `UART5SEL` writer - UART5 kernel clock source selection These bits are used to select the UART5 kernel clock source. Note: The UART5 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
pub type UART5SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR1_SPEC, u8, u8, 2, O>;
///Field `I2C1SEL` reader - I2C1 kernel clock source selection These bits are used to select the I2C1 kernel clock source. Note: The I2C1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
pub type I2C1SEL_R = crate::FieldReader<u8, u8>;
///Field `I2C1SEL` writer - I2C1 kernel clock source selection These bits are used to select the I2C1 kernel clock source. Note: The I2C1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
pub type I2C1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR1_SPEC, u8, u8, 2, O>;
///Field `I2C2SEL` reader - I2C2 kernel clock source selection These bits are used to select the I2C2 kernel clock source. Note: The I2C2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
pub type I2C2SEL_R = crate::FieldReader<u8, u8>;
///Field `I2C2SEL` writer - I2C2 kernel clock source selection These bits are used to select the I2C2 kernel clock source. Note: The I2C2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
pub type I2C2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR1_SPEC, u8, u8, 2, O>;
///Field `I2C4SEL` reader - I2C4 kernel clock source selection These bits are used to select the I2C4 kernel clock source. Note: The I2C4 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
pub type I2C4SEL_R = crate::FieldReader<u8, u8>;
///Field `I2C4SEL` writer - I2C4 kernel clock source selection These bits are used to select the I2C4 kernel clock source. Note: The I2C4 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
pub type I2C4SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR1_SPEC, u8, u8, 2, O>;
///Field `SPI2SEL` reader - SPI2 kernel clock source selection These bits are used to select the SPI2 kernel clock source. Note: The SPI2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
pub type SPI2SEL_R = crate::FieldReader<u8, u8>;
///Field `SPI2SEL` writer - SPI2 kernel clock source selection These bits are used to select the SPI2 kernel clock source. Note: The SPI2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
pub type SPI2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR1_SPEC, u8, u8, 2, O>;
///Field `LPTIM2SEL` reader - Low-power timer 2 kernel clock source selection These bits are used to select the LPTIM2 kernel clock source. Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI16 if HSIKERON = 1.
pub type LPTIM2SEL_R = crate::FieldReader<u8, u8>;
///Field `LPTIM2SEL` writer - Low-power timer 2 kernel clock source selection These bits are used to select the LPTIM2 kernel clock source. Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI16 if HSIKERON = 1.
pub type LPTIM2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR1_SPEC, u8, u8, 2, O>;
///Field `SPI1SEL` reader - SPI1 kernel clock source selection These bits are used to select the SPI1 kernel clock source. Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
pub type SPI1SEL_R = crate::FieldReader<u8, u8>;
///Field `SPI1SEL` writer - SPI1 kernel clock source selection These bits are used to select the SPI1 kernel clock source. Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
pub type SPI1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR1_SPEC, u8, u8, 2, O>;
///Field `SYSTICKSEL` reader - SysTick clock source selection These bits are used to select the SysTick clock source. Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one HCLK cycle is introduced, due to the LSE or LSI sampling with HCLK in the SysTick circuitry.
pub type SYSTICKSEL_R = crate::FieldReader<u8, u8>;
///Field `SYSTICKSEL` writer - SysTick clock source selection These bits are used to select the SysTick clock source. Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one HCLK cycle is introduced, due to the LSE or LSI sampling with HCLK in the SysTick circuitry.
pub type SYSTICKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR1_SPEC, u8, u8, 2, O>;
///Field `FDCAN1SEL` reader - FDCAN1 kernel clock source selection These bits are used to select the FDCAN1 kernel clock source.
pub type FDCAN1SEL_R = crate::FieldReader<u8, u8>;
///Field `FDCAN1SEL` writer - FDCAN1 kernel clock source selection These bits are used to select the FDCAN1 kernel clock source.
pub type FDCAN1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR1_SPEC, u8, u8, 2, O>;
///Field `ICLKSEL` reader - intermediate clock source selection These bits are used to select the clock source used by OTG_FS and SDMMC.
pub type ICLKSEL_R = crate::FieldReader<u8, u8>;
///Field `ICLKSEL` writer - intermediate clock source selection These bits are used to select the clock source used by OTG_FS and SDMMC.
pub type ICLKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR1_SPEC, u8, u8, 2, O>;
///Field `TIMICSEL` reader - Clocks sources for TIM16,TIM17 and LPTIM2 internal input capture When the TIMICSEL2 bit is set, the TIM16, TIM17 and LPTIM2 internal input capture can be connected either to HSI/256, MSI/4 or MSI/1024. Depending on TIMICSEL\[1:0\]
///value, MSI is either MSIK or MSIS. When TIMICSEL2 is cleared, the HSI, MSIK and MSIS clock sources cannot be selected as TIM16, TIM17 or LPTIM2 internal input capture. 0xx: HSI, MSIK and MSIS dividers disabled Note: The clock division must be disabled (TIMICSEL configured to 0xx) before selecting or changing a clock sources division.
pub type TIMICSEL_R = crate::FieldReader<u8, u8>;
///Field `TIMICSEL` writer - Clocks sources for TIM16,TIM17 and LPTIM2 internal input capture When the TIMICSEL2 bit is set, the TIM16, TIM17 and LPTIM2 internal input capture can be connected either to HSI/256, MSI/4 or MSI/1024. Depending on TIMICSEL\[1:0\]
///value, MSI is either MSIK or MSIS. When TIMICSEL2 is cleared, the HSI, MSIK and MSIS clock sources cannot be selected as TIM16, TIM17 or LPTIM2 internal input capture. 0xx: HSI, MSIK and MSIS dividers disabled Note: The clock division must be disabled (TIMICSEL configured to 0xx) before selecting or changing a clock sources division.
pub type TIMICSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR1_SPEC, u8, u8, 3, O>;
impl R {
    ///Bits 0:1 - USART1 kernel clock source selection This bits are used to select the USART1 kernel clock source. Note: The USART1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - USART2 kernel clock source selection This bits are used to select the USART2 kernel clock source. Note: The USART2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - USART3 kernel clock source selection This bits are used to select the USART3 kernel clock source. Note: The USART3 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    pub fn usart3sel(&self) -> USART3SEL_R {
        USART3SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - UART4 kernel clock source selection This bits are used to select the UART4 kernel clock source. Note: The UART4 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    pub fn uart4sel(&self) -> UART4SEL_R {
        UART4SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - UART5 kernel clock source selection These bits are used to select the UART5 kernel clock source. Note: The UART5 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    pub fn uart5sel(&self) -> UART5SEL_R {
        UART5SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - I2C1 kernel clock source selection These bits are used to select the I2C1 kernel clock source. Note: The I2C1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - I2C2 kernel clock source selection These bits are used to select the I2C2 kernel clock source. Note: The I2C2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2C2SEL_R {
        I2C2SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - I2C4 kernel clock source selection These bits are used to select the I2C4 kernel clock source. Note: The I2C4 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - SPI2 kernel clock source selection These bits are used to select the SPI2 kernel clock source. Note: The SPI2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    pub fn spi2sel(&self) -> SPI2SEL_R {
        SPI2SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Low-power timer 2 kernel clock source selection These bits are used to select the LPTIM2 kernel clock source. Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI16 if HSIKERON = 1.
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - SPI1 kernel clock source selection These bits are used to select the SPI1 kernel clock source. Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    pub fn spi1sel(&self) -> SPI1SEL_R {
        SPI1SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - SysTick clock source selection These bits are used to select the SysTick clock source. Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one HCLK cycle is introduced, due to the LSE or LSI sampling with HCLK in the SysTick circuitry.
    #[inline(always)]
    pub fn systicksel(&self) -> SYSTICKSEL_R {
        SYSTICKSEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - FDCAN1 kernel clock source selection These bits are used to select the FDCAN1 kernel clock source.
    #[inline(always)]
    pub fn fdcan1sel(&self) -> FDCAN1SEL_R {
        FDCAN1SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - intermediate clock source selection These bits are used to select the clock source used by OTG_FS and SDMMC.
    #[inline(always)]
    pub fn iclksel(&self) -> ICLKSEL_R {
        ICLKSEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 29:31 - Clocks sources for TIM16,TIM17 and LPTIM2 internal input capture When the TIMICSEL2 bit is set, the TIM16, TIM17 and LPTIM2 internal input capture can be connected either to HSI/256, MSI/4 or MSI/1024. Depending on TIMICSEL\[1:0\]
    ///value, MSI is either MSIK or MSIS. When TIMICSEL2 is cleared, the HSI, MSIK and MSIS clock sources cannot be selected as TIM16, TIM17 or LPTIM2 internal input capture. 0xx: HSI, MSIK and MSIS dividers disabled Note: The clock division must be disabled (TIMICSEL configured to 0xx) before selecting or changing a clock sources division.
    #[inline(always)]
    pub fn timicsel(&self) -> TIMICSEL_R {
        TIMICSEL_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    ///Bits 0:1 - USART1 kernel clock source selection This bits are used to select the USART1 kernel clock source. Note: The USART1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    #[must_use]
    pub fn usart1sel(&mut self) -> USART1SEL_W<0> {
        USART1SEL_W::new(self)
    }
    ///Bits 2:3 - USART2 kernel clock source selection This bits are used to select the USART2 kernel clock source. Note: The USART2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    #[must_use]
    pub fn usart2sel(&mut self) -> USART2SEL_W<2> {
        USART2SEL_W::new(self)
    }
    ///Bits 4:5 - USART3 kernel clock source selection This bits are used to select the USART3 kernel clock source. Note: The USART3 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    #[must_use]
    pub fn usart3sel(&mut self) -> USART3SEL_W<4> {
        USART3SEL_W::new(self)
    }
    ///Bits 6:7 - UART4 kernel clock source selection This bits are used to select the UART4 kernel clock source. Note: The UART4 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    #[must_use]
    pub fn uart4sel(&mut self) -> UART4SEL_W<6> {
        UART4SEL_W::new(self)
    }
    ///Bits 8:9 - UART5 kernel clock source selection These bits are used to select the UART5 kernel clock source. Note: The UART5 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or LSE.
    #[inline(always)]
    #[must_use]
    pub fn uart5sel(&mut self) -> UART5SEL_W<8> {
        UART5SEL_W::new(self)
    }
    ///Bits 10:11 - I2C1 kernel clock source selection These bits are used to select the I2C1 kernel clock source. Note: The I2C1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    #[must_use]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<10> {
        I2C1SEL_W::new(self)
    }
    ///Bits 12:13 - I2C2 kernel clock source selection These bits are used to select the I2C2 kernel clock source. Note: The I2C2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    #[must_use]
    pub fn i2c2sel(&mut self) -> I2C2SEL_W<12> {
        I2C2SEL_W::new(self)
    }
    ///Bits 14:15 - I2C4 kernel clock source selection These bits are used to select the I2C4 kernel clock source. Note: The I2C4 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    #[must_use]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W<14> {
        I2C4SEL_W::new(self)
    }
    ///Bits 16:17 - SPI2 kernel clock source selection These bits are used to select the SPI2 kernel clock source. Note: The SPI2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    #[must_use]
    pub fn spi2sel(&mut self) -> SPI2SEL_W<16> {
        SPI2SEL_W::new(self)
    }
    ///Bits 18:19 - Low-power timer 2 kernel clock source selection These bits are used to select the LPTIM2 kernel clock source. Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI16 if HSIKERON = 1.
    #[inline(always)]
    #[must_use]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<18> {
        LPTIM2SEL_W::new(self)
    }
    ///Bits 20:21 - SPI1 kernel clock source selection These bits are used to select the SPI1 kernel clock source. Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI16 or MSIK.
    #[inline(always)]
    #[must_use]
    pub fn spi1sel(&mut self) -> SPI1SEL_W<20> {
        SPI1SEL_W::new(self)
    }
    ///Bits 22:23 - SysTick clock source selection These bits are used to select the SysTick clock source. Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one HCLK cycle is introduced, due to the LSE or LSI sampling with HCLK in the SysTick circuitry.
    #[inline(always)]
    #[must_use]
    pub fn systicksel(&mut self) -> SYSTICKSEL_W<22> {
        SYSTICKSEL_W::new(self)
    }
    ///Bits 24:25 - FDCAN1 kernel clock source selection These bits are used to select the FDCAN1 kernel clock source.
    #[inline(always)]
    #[must_use]
    pub fn fdcan1sel(&mut self) -> FDCAN1SEL_W<24> {
        FDCAN1SEL_W::new(self)
    }
    ///Bits 26:27 - intermediate clock source selection These bits are used to select the clock source used by OTG_FS and SDMMC.
    #[inline(always)]
    #[must_use]
    pub fn iclksel(&mut self) -> ICLKSEL_W<26> {
        ICLKSEL_W::new(self)
    }
    ///Bits 29:31 - Clocks sources for TIM16,TIM17 and LPTIM2 internal input capture When the TIMICSEL2 bit is set, the TIM16, TIM17 and LPTIM2 internal input capture can be connected either to HSI/256, MSI/4 or MSI/1024. Depending on TIMICSEL\[1:0\]
    ///value, MSI is either MSIK or MSIS. When TIMICSEL2 is cleared, the HSI, MSIK and MSIS clock sources cannot be selected as TIM16, TIM17 or LPTIM2 internal input capture. 0xx: HSI, MSIK and MSIS dividers disabled Note: The clock division must be disabled (TIMICSEL configured to 0xx) before selecting or changing a clock sources division.
    #[inline(always)]
    #[must_use]
    pub fn timicsel(&mut self) -> TIMICSEL_W<29> {
        TIMICSEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC peripherals independent clock configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_ccipr1](index.html) module
pub struct RCC_CCIPR1_SPEC;
impl crate::RegisterSpec for RCC_CCIPR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_ccipr1::R](R) reader structure
impl crate::Readable for RCC_CCIPR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_ccipr1::W](W) writer structure
impl crate::Writable for RCC_CCIPR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_CCIPR1 to value 0
impl crate::Resettable for RCC_CCIPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
