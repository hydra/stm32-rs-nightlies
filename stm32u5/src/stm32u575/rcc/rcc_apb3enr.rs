///Register `RCC_APB3ENR` reader
pub struct R(crate::R<RCC_APB3ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB3ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB3ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB3ENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_APB3ENR` writer
pub struct W(crate::W<RCC_APB3ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB3ENR_SPEC>;
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
impl From<crate::W<RCC_APB3ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB3ENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SYSCFGEN` reader - SYSCFG clock enable Set and cleared by software.
pub type SYSCFGEN_R = crate::BitReader<bool>;
///Field `SYSCFGEN` writer - SYSCFG clock enable Set and cleared by software.
pub type SYSCFGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3ENR_SPEC, bool, O>;
///Field `SPI3EN` reader - SPI3 clock enable Set and cleared by software.
pub type SPI3EN_R = crate::BitReader<bool>;
///Field `SPI3EN` writer - SPI3 clock enable Set and cleared by software.
pub type SPI3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3ENR_SPEC, bool, O>;
///Field `LPUART1EN` reader - LPUART1 clock enable Set and cleared by software.
pub type LPUART1EN_R = crate::BitReader<bool>;
///Field `LPUART1EN` writer - LPUART1 clock enable Set and cleared by software.
pub type LPUART1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3ENR_SPEC, bool, O>;
///Field `I2C3EN` reader - I2C3 clock enable Set and cleared by software.
pub type I2C3EN_R = crate::BitReader<bool>;
///Field `I2C3EN` writer - I2C3 clock enable Set and cleared by software.
pub type I2C3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3ENR_SPEC, bool, O>;
///Field `LPTIM1EN` reader - LPTIM1 clock enable Set and cleared by software.
pub type LPTIM1EN_R = crate::BitReader<bool>;
///Field `LPTIM1EN` writer - LPTIM1 clock enable Set and cleared by software.
pub type LPTIM1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3ENR_SPEC, bool, O>;
///Field `LPTIM3EN` reader - LPTIM3 clock enable Set and cleared by software.
pub type LPTIM3EN_R = crate::BitReader<bool>;
///Field `LPTIM3EN` writer - LPTIM3 clock enable Set and cleared by software.
pub type LPTIM3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3ENR_SPEC, bool, O>;
///Field `LPTIM4EN` reader - LPTIM4 clock enable Set and cleared by software.
pub type LPTIM4EN_R = crate::BitReader<bool>;
///Field `LPTIM4EN` writer - LPTIM4 clock enable Set and cleared by software.
pub type LPTIM4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3ENR_SPEC, bool, O>;
///Field `OPAMPEN` reader - OPAMP clock enable Set and cleared by software.
pub type OPAMPEN_R = crate::BitReader<bool>;
///Field `OPAMPEN` writer - OPAMP clock enable Set and cleared by software.
pub type OPAMPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3ENR_SPEC, bool, O>;
///Field `COMPEN` reader - COMP clock enable Set and cleared by software.
pub type COMPEN_R = crate::BitReader<bool>;
///Field `COMPEN` writer - COMP clock enable Set and cleared by software.
pub type COMPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3ENR_SPEC, bool, O>;
///Field `VREFEN` reader - VREFBUF clock enable Set and cleared by software.
pub type VREFEN_R = crate::BitReader<bool>;
///Field `VREFEN` writer - VREFBUF clock enable Set and cleared by software.
pub type VREFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3ENR_SPEC, bool, O>;
///Field `RTCAPBEN` reader - RTC and TAMP APB clock enable Set and cleared by software.
pub type RTCAPBEN_R = crate::BitReader<bool>;
///Field `RTCAPBEN` writer - RTC and TAMP APB clock enable Set and cleared by software.
pub type RTCAPBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3ENR_SPEC, bool, O>;
impl R {
    ///Bit 1 - SYSCFG clock enable Set and cleared by software.
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - SPI3 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LPUART1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C3 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - LPTIM1 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM3 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn lptim3en(&self) -> LPTIM3EN_R {
        LPTIM3EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - LPTIM4 clock enable Set and cleared by software.
    #[inline(always)]
    pub fn lptim4en(&self) -> LPTIM4EN_R {
        LPTIM4EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - OPAMP clock enable Set and cleared by software.
    #[inline(always)]
    pub fn opampen(&self) -> OPAMPEN_R {
        OPAMPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - COMP clock enable Set and cleared by software.
    #[inline(always)]
    pub fn compen(&self) -> COMPEN_R {
        COMPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 20 - VREFBUF clock enable Set and cleared by software.
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - RTC and TAMP APB clock enable Set and cleared by software.
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - SYSCFG clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<1> {
        SYSCFGEN_W::new(self)
    }
    ///Bit 5 - SPI3 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn spi3en(&mut self) -> SPI3EN_W<5> {
        SPI3EN_W::new(self)
    }
    ///Bit 6 - LPUART1 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<6> {
        LPUART1EN_W::new(self)
    }
    ///Bit 7 - I2C3 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn i2c3en(&mut self) -> I2C3EN_W<7> {
        I2C3EN_W::new(self)
    }
    ///Bit 11 - LPTIM1 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<11> {
        LPTIM1EN_W::new(self)
    }
    ///Bit 12 - LPTIM3 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lptim3en(&mut self) -> LPTIM3EN_W<12> {
        LPTIM3EN_W::new(self)
    }
    ///Bit 13 - LPTIM4 clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lptim4en(&mut self) -> LPTIM4EN_W<13> {
        LPTIM4EN_W::new(self)
    }
    ///Bit 14 - OPAMP clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn opampen(&mut self) -> OPAMPEN_W<14> {
        OPAMPEN_W::new(self)
    }
    ///Bit 15 - COMP clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn compen(&mut self) -> COMPEN_W<15> {
        COMPEN_W::new(self)
    }
    ///Bit 20 - VREFBUF clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<20> {
        VREFEN_W::new(self)
    }
    ///Bit 21 - RTC and TAMP APB clock enable Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<21> {
        RTCAPBEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC APB3 peripheral clock enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_apb3enr](index.html) module
pub struct RCC_APB3ENR_SPEC;
impl crate::RegisterSpec for RCC_APB3ENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_apb3enr::R](R) reader structure
impl crate::Readable for RCC_APB3ENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_apb3enr::W](W) writer structure
impl crate::Writable for RCC_APB3ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_APB3ENR to value 0
impl crate::Resettable for RCC_APB3ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
