///Register `SPI_CRCPOLY` reader
pub struct R(crate::R<SPI_CRCPOLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CRCPOLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CRCPOLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CRCPOLY_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SPI_CRCPOLY` writer
pub struct W(crate::W<SPI_CRCPOLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CRCPOLY_SPEC>;
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
impl From<crate::W<SPI_CRCPOLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CRCPOLY_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CRCPOLY` reader - CRCPOLY
pub type CRCPOLY_R = crate::FieldReader<u32, u32>;
///Field `CRCPOLY` writer - CRCPOLY
pub type CRCPOLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_CRCPOLY_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - CRCPOLY
    #[inline(always)]
    pub fn crcpoly(&self) -> CRCPOLY_R {
        CRCPOLY_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - CRCPOLY
    #[inline(always)]
    #[must_use]
    pub fn crcpoly(&mut self) -> CRCPOLY_W<0> {
        CRCPOLY_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SPI polynomial register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [spi_crcpoly](index.html) module
pub struct SPI_CRCPOLY_SPEC;
impl crate::RegisterSpec for SPI_CRCPOLY_SPEC {
    type Ux = u32;
}
///`read()` method returns [spi_crcpoly::R](R) reader structure
impl crate::Readable for SPI_CRCPOLY_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [spi_crcpoly::W](W) writer structure
impl crate::Writable for SPI_CRCPOLY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SPI_CRCPOLY to value 0x0107
impl crate::Resettable for SPI_CRCPOLY_SPEC {
    const RESET_VALUE: Self::Ux = 0x0107;
}
