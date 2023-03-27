///Register `GTZC1_TZIC_SR2` reader
pub struct R(crate::R<GTZC1_TZIC_SR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTZC1_TZIC_SR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTZC1_TZIC_SR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTZC1_TZIC_SR2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `FDCAN1F` reader - illegal access flag for FDCAN1
pub type FDCAN1F_R = crate::BitReader<bool>;
///Field `FDCAN2F` reader - illegal access flag for FDCAN2
pub type FDCAN2F_R = crate::BitReader<bool>;
///Field `UCPDF` reader - illegal access flag for UCPD
pub type UCPDF_R = crate::BitReader<bool>;
///Field `TIM1F` reader - illegal access flag for TIM1
pub type TIM1F_R = crate::BitReader<bool>;
///Field `SPI1F` reader - illegal access flag for SPI1
pub type SPI1F_R = crate::BitReader<bool>;
///Field `TIM8F` reader - illegal access flag for TIM8
pub type TIM8F_R = crate::BitReader<bool>;
///Field `USART1F` reader - illegal access flag for USART1
pub type USART1F_R = crate::BitReader<bool>;
///Field `TIM15F` reader - illegal access flag for TIM15
pub type TIM15F_R = crate::BitReader<bool>;
///Field `TIM16F` reader - illegal access flag for TIM16
pub type TIM16F_R = crate::BitReader<bool>;
///Field `TIM17F` reader - illegal access flag for TIM17
pub type TIM17F_R = crate::BitReader<bool>;
///Field `SPI4F` reader - illegal access flag for SPI4
pub type SPI4F_R = crate::BitReader<bool>;
///Field `SPI6F` reader - illegal access flag for SPI6
pub type SPI6F_R = crate::BitReader<bool>;
///Field `SAI1F` reader - illegal access flag for SAI1
pub type SAI1F_R = crate::BitReader<bool>;
///Field `SAI2F` reader - illegal access flag for SAI2
pub type SAI2F_R = crate::BitReader<bool>;
///Field `USBF` reader - illegal access flag for USB
pub type USBF_R = crate::BitReader<bool>;
///Field `SPI5F` reader - illegal access flag for SPI5
pub type SPI5F_R = crate::BitReader<bool>;
///Field `LPUART1F` reader - illegal access flag for LPUART
pub type LPUART1F_R = crate::BitReader<bool>;
///Field `I2C3F` reader - illegal access flag for I2C3
pub type I2C3F_R = crate::BitReader<bool>;
///Field `I2C4F` reader - illegal access flag for I2C4
pub type I2C4F_R = crate::BitReader<bool>;
///Field `LPTIM1F` reader - illegal access flag for LPTIM1
pub type LPTIM1F_R = crate::BitReader<bool>;
///Field `LPTIM3F` reader - illegal access flag for LPTIM3
pub type LPTIM3F_R = crate::BitReader<bool>;
///Field `LPTIM4F` reader - illegal access flag for LPTIM4
pub type LPTIM4F_R = crate::BitReader<bool>;
///Field `LPTIM5F` reader - illegal access flag for LPTIM5
pub type LPTIM5F_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - illegal access flag for FDCAN1
    #[inline(always)]
    pub fn fdcan1f(&self) -> FDCAN1F_R {
        FDCAN1F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - illegal access flag for FDCAN2
    #[inline(always)]
    pub fn fdcan2f(&self) -> FDCAN2F_R {
        FDCAN2F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - illegal access flag for UCPD
    #[inline(always)]
    pub fn ucpdf(&self) -> UCPDF_R {
        UCPDF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - illegal access flag for TIM1
    #[inline(always)]
    pub fn tim1f(&self) -> TIM1F_R {
        TIM1F_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - illegal access flag for SPI1
    #[inline(always)]
    pub fn spi1f(&self) -> SPI1F_R {
        SPI1F_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - illegal access flag for TIM8
    #[inline(always)]
    pub fn tim8f(&self) -> TIM8F_R {
        TIM8F_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - illegal access flag for USART1
    #[inline(always)]
    pub fn usart1f(&self) -> USART1F_R {
        USART1F_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - illegal access flag for TIM15
    #[inline(always)]
    pub fn tim15f(&self) -> TIM15F_R {
        TIM15F_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - illegal access flag for TIM16
    #[inline(always)]
    pub fn tim16f(&self) -> TIM16F_R {
        TIM16F_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - illegal access flag for TIM17
    #[inline(always)]
    pub fn tim17f(&self) -> TIM17F_R {
        TIM17F_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - illegal access flag for SPI4
    #[inline(always)]
    pub fn spi4f(&self) -> SPI4F_R {
        SPI4F_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - illegal access flag for SPI6
    #[inline(always)]
    pub fn spi6f(&self) -> SPI6F_R {
        SPI6F_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - illegal access flag for SAI1
    #[inline(always)]
    pub fn sai1f(&self) -> SAI1F_R {
        SAI1F_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - illegal access flag for SAI2
    #[inline(always)]
    pub fn sai2f(&self) -> SAI2F_R {
        SAI2F_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - illegal access flag for USB
    #[inline(always)]
    pub fn usbf(&self) -> USBF_R {
        USBF_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - illegal access flag for SPI5
    #[inline(always)]
    pub fn spi5f(&self) -> SPI5F_R {
        SPI5F_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - illegal access flag for LPUART
    #[inline(always)]
    pub fn lpuart1f(&self) -> LPUART1F_R {
        LPUART1F_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - illegal access flag for I2C3
    #[inline(always)]
    pub fn i2c3f(&self) -> I2C3F_R {
        I2C3F_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - illegal access flag for I2C4
    #[inline(always)]
    pub fn i2c4f(&self) -> I2C4F_R {
        I2C4F_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - illegal access flag for LPTIM1
    #[inline(always)]
    pub fn lptim1f(&self) -> LPTIM1F_R {
        LPTIM1F_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - illegal access flag for LPTIM3
    #[inline(always)]
    pub fn lptim3f(&self) -> LPTIM3F_R {
        LPTIM3F_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - illegal access flag for LPTIM4
    #[inline(always)]
    pub fn lptim4f(&self) -> LPTIM4F_R {
        LPTIM4F_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - illegal access flag for LPTIM5
    #[inline(always)]
    pub fn lptim5f(&self) -> LPTIM5F_R {
        LPTIM5F_R::new(((self.bits >> 31) & 1) != 0)
    }
}
///TZIC status register 2
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gtzc1_tzic_sr2](index.html) module
pub struct GTZC1_TZIC_SR2_SPEC;
impl crate::RegisterSpec for GTZC1_TZIC_SR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [gtzc1_tzic_sr2::R](R) reader structure
impl crate::Readable for GTZC1_TZIC_SR2_SPEC {
    type Reader = R;
}
///`reset()` method sets GTZC1_TZIC_SR2 to value 0
impl crate::Resettable for GTZC1_TZIC_SR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
