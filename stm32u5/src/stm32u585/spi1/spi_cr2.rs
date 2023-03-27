///Register `SPI_CR2` reader
pub struct R(crate::R<SPI_CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SPI_CR2` writer
pub struct W(crate::W<SPI_CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CR2_SPEC>;
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
impl From<crate::W<SPI_CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TSIZE` reader - number of data at current transfer When these bits are changed by software, the SPI has to be disabled. Endless transaction is initialized when CSTART is set while zero value is stored at TSIZE. TSIZE cannot be set to 0xFFFF respective 0x3FFF value when CRC is enabled. Note: TSIZE\[15:10\]
///bits are reserved at limited feature set instances and must be kept at reset value.
pub type TSIZE_R = crate::FieldReader<u16, u16>;
///Field `TSIZE` writer - number of data at current transfer When these bits are changed by software, the SPI has to be disabled. Endless transaction is initialized when CSTART is set while zero value is stored at TSIZE. TSIZE cannot be set to 0xFFFF respective 0x3FFF value when CRC is enabled. Note: TSIZE\[15:10\]
///bits are reserved at limited feature set instances and must be kept at reset value.
pub type TSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_CR2_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - number of data at current transfer When these bits are changed by software, the SPI has to be disabled. Endless transaction is initialized when CSTART is set while zero value is stored at TSIZE. TSIZE cannot be set to 0xFFFF respective 0x3FFF value when CRC is enabled. Note: TSIZE\[15:10\]
    ///bits are reserved at limited feature set instances and must be kept at reset value.
    #[inline(always)]
    pub fn tsize(&self) -> TSIZE_R {
        TSIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - number of data at current transfer When these bits are changed by software, the SPI has to be disabled. Endless transaction is initialized when CSTART is set while zero value is stored at TSIZE. TSIZE cannot be set to 0xFFFF respective 0x3FFF value when CRC is enabled. Note: TSIZE\[15:10\]
    ///bits are reserved at limited feature set instances and must be kept at reset value.
    #[inline(always)]
    #[must_use]
    pub fn tsize(&mut self) -> TSIZE_W<0> {
        TSIZE_W::new(self)
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
///For information about available fields see [spi_cr2](index.html) module
pub struct SPI_CR2_SPEC;
impl crate::RegisterSpec for SPI_CR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [spi_cr2::R](R) reader structure
impl crate::Readable for SPI_CR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [spi_cr2::W](W) writer structure
impl crate::Writable for SPI_CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SPI_CR2 to value 0
impl crate::Resettable for SPI_CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
