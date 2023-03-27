///Register `RCC_APB3SMENR` reader
pub struct R(crate::R<RCC_APB3SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_APB3SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_APB3SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_APB3SMENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_APB3SMENR` writer
pub struct W(crate::W<RCC_APB3SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_APB3SMENR_SPEC>;
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
impl From<crate::W<RCC_APB3SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_APB3SMENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SYSCFGSMEN` reader - SYSCFG clocks enable during Sleep and Stop modes Set and cleared by software.
pub type SYSCFGSMEN_R = crate::BitReader<bool>;
///Field `SYSCFGSMEN` writer - SYSCFG clocks enable during Sleep and Stop modes Set and cleared by software.
pub type SYSCFGSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3SMENR_SPEC, bool, O>;
///Field `SPI3SMEN` reader - SPI3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type SPI3SMEN_R = crate::BitReader<bool>;
///Field `SPI3SMEN` writer - SPI3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type SPI3SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3SMENR_SPEC, bool, O>;
///Field `LPUART1SMEN` reader - LPUART1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPUART1SMEN_R = crate::BitReader<bool>;
///Field `LPUART1SMEN` writer - LPUART1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPUART1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3SMENR_SPEC, bool, O>;
///Field `I2C3SMEN` reader - I2C3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type I2C3SMEN_R = crate::BitReader<bool>;
///Field `I2C3SMEN` writer - I2C3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type I2C3SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3SMENR_SPEC, bool, O>;
///Field `LPTIM1SMEN` reader - LPTIM1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPTIM1SMEN_R = crate::BitReader<bool>;
///Field `LPTIM1SMEN` writer - LPTIM1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPTIM1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3SMENR_SPEC, bool, O>;
///Field `LPTIM3SMEN` reader - LPTIM3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPTIM3SMEN_R = crate::BitReader<bool>;
///Field `LPTIM3SMEN` writer - LPTIM3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPTIM3SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3SMENR_SPEC, bool, O>;
///Field `LPTIM4SMEN` reader - LPTIM4 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPTIM4SMEN_R = crate::BitReader<bool>;
///Field `LPTIM4SMEN` writer - LPTIM4 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPTIM4SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3SMENR_SPEC, bool, O>;
///Field `OPAMPSMEN` reader - OPAMP clocks enable during Sleep and Stop modes Set and cleared by software.
pub type OPAMPSMEN_R = crate::BitReader<bool>;
///Field `OPAMPSMEN` writer - OPAMP clocks enable during Sleep and Stop modes Set and cleared by software.
pub type OPAMPSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3SMENR_SPEC, bool, O>;
///Field `COMPSMEN` reader - COMP clocks enable during Sleep and Stop modes Set and cleared by software.
pub type COMPSMEN_R = crate::BitReader<bool>;
///Field `COMPSMEN` writer - COMP clocks enable during Sleep and Stop modes Set and cleared by software.
pub type COMPSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3SMENR_SPEC, bool, O>;
///Field `VREFSMEN` reader - VREFBUF clocks enable during Sleep and Stop modes Set and cleared by software.
pub type VREFSMEN_R = crate::BitReader<bool>;
///Field `VREFSMEN` writer - VREFBUF clocks enable during Sleep and Stop modes Set and cleared by software.
pub type VREFSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3SMENR_SPEC, bool, O>;
///Field `RTCAPBSMEN` reader - RTC and TAMP APB clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type RTCAPBSMEN_R = crate::BitReader<bool>;
///Field `RTCAPBSMEN` writer - RTC and TAMP APB clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type RTCAPBSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_APB3SMENR_SPEC, bool, O>;
impl R {
    ///Bit 1 - SYSCFG clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SYSCFGSMEN_R {
        SYSCFGSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - SPI3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn spi3smen(&self) -> SPI3SMEN_R {
        SPI3SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - LPUART1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - I2C3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - LPTIM1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lptim1smen(&self) -> LPTIM1SMEN_R {
        LPTIM1SMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lptim3smen(&self) -> LPTIM3SMEN_R {
        LPTIM3SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - LPTIM4 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lptim4smen(&self) -> LPTIM4SMEN_R {
        LPTIM4SMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - OPAMP clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn opampsmen(&self) -> OPAMPSMEN_R {
        OPAMPSMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - COMP clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn compsmen(&self) -> COMPSMEN_R {
        COMPSMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 20 - VREFBUF clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    pub fn vrefsmen(&self) -> VREFSMEN_R {
        VREFSMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - RTC and TAMP APB clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - SYSCFG clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn syscfgsmen(&mut self) -> SYSCFGSMEN_W<1> {
        SYSCFGSMEN_W::new(self)
    }
    ///Bit 5 - SPI3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn spi3smen(&mut self) -> SPI3SMEN_W<5> {
        SPI3SMEN_W::new(self)
    }
    ///Bit 6 - LPUART1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W<6> {
        LPUART1SMEN_W::new(self)
    }
    ///Bit 7 - I2C3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W<7> {
        I2C3SMEN_W::new(self)
    }
    ///Bit 11 - LPTIM1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn lptim1smen(&mut self) -> LPTIM1SMEN_W<11> {
        LPTIM1SMEN_W::new(self)
    }
    ///Bit 12 - LPTIM3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn lptim3smen(&mut self) -> LPTIM3SMEN_W<12> {
        LPTIM3SMEN_W::new(self)
    }
    ///Bit 13 - LPTIM4 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn lptim4smen(&mut self) -> LPTIM4SMEN_W<13> {
        LPTIM4SMEN_W::new(self)
    }
    ///Bit 14 - OPAMP clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn opampsmen(&mut self) -> OPAMPSMEN_W<14> {
        OPAMPSMEN_W::new(self)
    }
    ///Bit 15 - COMP clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn compsmen(&mut self) -> COMPSMEN_W<15> {
        COMPSMEN_W::new(self)
    }
    ///Bit 20 - VREFBUF clocks enable during Sleep and Stop modes Set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn vrefsmen(&mut self) -> VREFSMEN_W<20> {
        VREFSMEN_W::new(self)
    }
    ///Bit 21 - RTC and TAMP APB clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W<21> {
        RTCAPBSMEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC APB3 peripheral clock enable in Sleep and Stop modes register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_apb3smenr](index.html) module
pub struct RCC_APB3SMENR_SPEC;
impl crate::RegisterSpec for RCC_APB3SMENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_apb3smenr::R](R) reader structure
impl crate::Readable for RCC_APB3SMENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_apb3smenr::W](W) writer structure
impl crate::Writable for RCC_APB3SMENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_APB3SMENR to value 0xffff_ffff
impl crate::Resettable for RCC_APB3SMENR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
