///Register `SPI_AUTOCR` reader
pub struct R(crate::R<SPI_AUTOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_AUTOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_AUTOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_AUTOCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SPI_AUTOCR` writer
pub struct W(crate::W<SPI_AUTOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_AUTOCR_SPEC>;
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
impl From<crate::W<SPI_AUTOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_AUTOCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TRIGSEL` reader - trigger selection (refer ). ... Note: these bits can be written only when SPE = 0.
pub type TRIGSEL_R = crate::FieldReader<u8, u8>;
///Field `TRIGSEL` writer - trigger selection (refer ). ... Note: these bits can be written only when SPE = 0.
pub type TRIGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_AUTOCR_SPEC, u8, u8, 4, O>;
///Field `TRIGPOL` reader - trigger polarity Note: This bit can be written only when SPE = 0.
pub type TRIGPOL_R = crate::BitReader<bool>;
///Field `TRIGPOL` writer - trigger polarity Note: This bit can be written only when SPE = 0.
pub type TRIGPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_AUTOCR_SPEC, bool, O>;
///Field `TRIGEN` reader - trigger of CSTART control enable Note: if user can't prevent trigger event during write, the TRIGEN has to be changed when SPI is disabled
pub type TRIGEN_R = crate::BitReader<bool>;
///Field `TRIGEN` writer - trigger of CSTART control enable Note: if user can't prevent trigger event during write, the TRIGEN has to be changed when SPI is disabled
pub type TRIGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_AUTOCR_SPEC, bool, O>;
impl R {
    ///Bits 16:19 - trigger selection (refer ). ... Note: these bits can be written only when SPE = 0.
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bit 20 - trigger polarity Note: This bit can be written only when SPE = 0.
    #[inline(always)]
    pub fn trigpol(&self) -> TRIGPOL_R {
        TRIGPOL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - trigger of CSTART control enable Note: if user can't prevent trigger event during write, the TRIGEN has to be changed when SPI is disabled
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    ///Bits 16:19 - trigger selection (refer ). ... Note: these bits can be written only when SPE = 0.
    #[inline(always)]
    #[must_use]
    pub fn trigsel(&mut self) -> TRIGSEL_W<16> {
        TRIGSEL_W::new(self)
    }
    ///Bit 20 - trigger polarity Note: This bit can be written only when SPE = 0.
    #[inline(always)]
    #[must_use]
    pub fn trigpol(&mut self) -> TRIGPOL_W<20> {
        TRIGPOL_W::new(self)
    }
    ///Bit 21 - trigger of CSTART control enable Note: if user can't prevent trigger event during write, the TRIGEN has to be changed when SPI is disabled
    #[inline(always)]
    #[must_use]
    pub fn trigen(&mut self) -> TRIGEN_W<21> {
        TRIGEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [spi_autocr](index.html) module
pub struct SPI_AUTOCR_SPEC;
impl crate::RegisterSpec for SPI_AUTOCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [spi_autocr::R](R) reader structure
impl crate::Readable for SPI_AUTOCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [spi_autocr::W](W) writer structure
impl crate::Writable for SPI_AUTOCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SPI_AUTOCR to value 0
impl crate::Resettable for SPI_AUTOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
