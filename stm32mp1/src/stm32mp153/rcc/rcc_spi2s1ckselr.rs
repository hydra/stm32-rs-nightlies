///Register `RCC_SPI2S1CKSELR` reader
pub struct R(crate::R<RCC_SPI2S1CKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_SPI2S1CKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_SPI2S1CKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_SPI2S1CKSELR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_SPI2S1CKSELR` writer
pub struct W(crate::W<RCC_SPI2S1CKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_SPI2S1CKSELR_SPEC>;
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
impl From<crate::W<RCC_SPI2S1CKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_SPI2S1CKSELR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SPI1SRC` reader - SPI1SRC
pub type SPI1SRC_R = crate::FieldReader<u8, u8>;
///Field `SPI1SRC` writer - SPI1SRC
pub type SPI1SRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RCC_SPI2S1CKSELR_SPEC, u8, u8, 3, O>;
impl R {
    ///Bits 0:2 - SPI1SRC
    #[inline(always)]
    pub fn spi1src(&self) -> SPI1SRC_R {
        SPI1SRC_R::new((self.bits & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - SPI1SRC
    #[inline(always)]
    #[must_use]
    pub fn spi1src(&mut self) -> SPI1SRC_W<0> {
        SPI1SRC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to control the selection of the kernel clock for the SPI/I2S1. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_spi2s1ckselr](index.html) module
pub struct RCC_SPI2S1CKSELR_SPEC;
impl crate::RegisterSpec for RCC_SPI2S1CKSELR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_spi2s1ckselr::R](R) reader structure
impl crate::Readable for RCC_SPI2S1CKSELR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_spi2s1ckselr::W](W) writer structure
impl crate::Writable for RCC_SPI2S1CKSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_SPI2S1CKSELR to value 0
impl crate::Resettable for RCC_SPI2S1CKSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
