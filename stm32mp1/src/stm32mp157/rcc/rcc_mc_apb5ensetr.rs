///Register `RCC_MC_APB5ENSETR` reader
pub struct R(crate::R<RCC_MC_APB5ENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_APB5ENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_APB5ENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_APB5ENSETR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_MC_APB5ENSETR` writer
pub struct W(crate::W<RCC_MC_APB5ENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_APB5ENSETR_SPEC>;
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
impl From<crate::W<RCC_MC_APB5ENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_APB5ENSETR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SPI6EN` reader - SPI6EN
pub type SPI6EN_R = crate::BitReader<bool>;
///Field `SPI6EN` writer - SPI6EN
pub type SPI6EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB5ENSETR_SPEC, bool, O>;
///Field `I2C4EN` reader - I2C4EN
pub type I2C4EN_R = crate::BitReader<bool>;
///Field `I2C4EN` writer - I2C4EN
pub type I2C4EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB5ENSETR_SPEC, bool, O>;
///Field `I2C6EN` reader - I2C6EN
pub type I2C6EN_R = crate::BitReader<bool>;
///Field `I2C6EN` writer - I2C6EN
pub type I2C6EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB5ENSETR_SPEC, bool, O>;
///Field `USART1EN` reader - USART1EN
pub type USART1EN_R = crate::BitReader<bool>;
///Field `USART1EN` writer - USART1EN
pub type USART1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB5ENSETR_SPEC, bool, O>;
///Field `RTCAPBEN` reader - RTCAPBEN
pub type RTCAPBEN_R = crate::BitReader<bool>;
///Field `RTCAPBEN` writer - RTCAPBEN
pub type RTCAPBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB5ENSETR_SPEC, bool, O>;
///Field `TZC1EN` reader - TZC1EN
pub type TZC1EN_R = crate::BitReader<bool>;
///Field `TZC1EN` writer - TZC1EN
pub type TZC1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB5ENSETR_SPEC, bool, O>;
///Field `TZC2EN` reader - TZC2EN
pub type TZC2EN_R = crate::BitReader<bool>;
///Field `TZC2EN` writer - TZC2EN
pub type TZC2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB5ENSETR_SPEC, bool, O>;
///Field `TZPCEN` reader - TZPCEN
pub type TZPCEN_R = crate::BitReader<bool>;
///Field `TZPCEN` writer - TZPCEN
pub type TZPCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB5ENSETR_SPEC, bool, O>;
///Field `BSECEN` reader - BSECEN
pub type BSECEN_R = crate::BitReader<bool>;
///Field `BSECEN` writer - BSECEN
pub type BSECEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB5ENSETR_SPEC, bool, O>;
///Field `STGENEN` reader - STGENEN
pub type STGENEN_R = crate::BitReader<bool>;
///Field `STGENEN` writer - STGENEN
pub type STGENEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_APB5ENSETR_SPEC, bool, O>;
impl R {
    ///Bit 0 - SPI6EN
    #[inline(always)]
    pub fn spi6en(&self) -> SPI6EN_R {
        SPI6EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - I2C4EN
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I2C6EN
    #[inline(always)]
    pub fn i2c6en(&self) -> I2C6EN_R {
        I2C6EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - USART1EN
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - RTCAPBEN
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - TZC1EN
    #[inline(always)]
    pub fn tzc1en(&self) -> TZC1EN_R {
        TZC1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TZC2EN
    #[inline(always)]
    pub fn tzc2en(&self) -> TZC2EN_R {
        TZC2EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - TZPCEN
    #[inline(always)]
    pub fn tzpcen(&self) -> TZPCEN_R {
        TZPCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - BSECEN
    #[inline(always)]
    pub fn bsecen(&self) -> BSECEN_R {
        BSECEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - STGENEN
    #[inline(always)]
    pub fn stgenen(&self) -> STGENEN_R {
        STGENEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SPI6EN
    #[inline(always)]
    #[must_use]
    pub fn spi6en(&mut self) -> SPI6EN_W<0> {
        SPI6EN_W::new(self)
    }
    ///Bit 2 - I2C4EN
    #[inline(always)]
    #[must_use]
    pub fn i2c4en(&mut self) -> I2C4EN_W<2> {
        I2C4EN_W::new(self)
    }
    ///Bit 3 - I2C6EN
    #[inline(always)]
    #[must_use]
    pub fn i2c6en(&mut self) -> I2C6EN_W<3> {
        I2C6EN_W::new(self)
    }
    ///Bit 4 - USART1EN
    #[inline(always)]
    #[must_use]
    pub fn usart1en(&mut self) -> USART1EN_W<4> {
        USART1EN_W::new(self)
    }
    ///Bit 8 - RTCAPBEN
    #[inline(always)]
    #[must_use]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<8> {
        RTCAPBEN_W::new(self)
    }
    ///Bit 11 - TZC1EN
    #[inline(always)]
    #[must_use]
    pub fn tzc1en(&mut self) -> TZC1EN_W<11> {
        TZC1EN_W::new(self)
    }
    ///Bit 12 - TZC2EN
    #[inline(always)]
    #[must_use]
    pub fn tzc2en(&mut self) -> TZC2EN_W<12> {
        TZC2EN_W::new(self)
    }
    ///Bit 13 - TZPCEN
    #[inline(always)]
    #[must_use]
    pub fn tzpcen(&mut self) -> TZPCEN_W<13> {
        TZPCEN_W::new(self)
    }
    ///Bit 16 - BSECEN
    #[inline(always)]
    #[must_use]
    pub fn bsecen(&mut self) -> BSECEN_W<16> {
        BSECEN_W::new(self)
    }
    ///Bit 20 - STGENEN
    #[inline(always)]
    #[must_use]
    pub fn stgenen(&mut self) -> STGENEN_W<20> {
        STGENEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to set the peripheral clock enable bit
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_mc_apb5ensetr](index.html) module
pub struct RCC_MC_APB5ENSETR_SPEC;
impl crate::RegisterSpec for RCC_MC_APB5ENSETR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_mc_apb5ensetr::R](R) reader structure
impl crate::Readable for RCC_MC_APB5ENSETR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_mc_apb5ensetr::W](W) writer structure
impl crate::Writable for RCC_MC_APB5ENSETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_MC_APB5ENSETR to value 0
impl crate::Resettable for RCC_MC_APB5ENSETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
