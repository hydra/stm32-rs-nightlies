///Register `QUADSPI_PSMAR` reader
pub struct R(crate::R<QUADSPI_PSMAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUADSPI_PSMAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUADSPI_PSMAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUADSPI_PSMAR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `QUADSPI_PSMAR` writer
pub struct W(crate::W<QUADSPI_PSMAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QUADSPI_PSMAR_SPEC>;
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
impl From<crate::W<QUADSPI_PSMAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QUADSPI_PSMAR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MATCH` reader - MATCH
pub type MATCH_R = crate::FieldReader<u32, u32>;
///Field `MATCH` writer - MATCH
pub type MATCH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QUADSPI_PSMAR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - MATCH
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - MATCH
    #[inline(always)]
    #[must_use]
    pub fn match_(&mut self) -> MATCH_W<0> {
        MATCH_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///QUADSPI polling status match register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [quadspi_psmar](index.html) module
pub struct QUADSPI_PSMAR_SPEC;
impl crate::RegisterSpec for QUADSPI_PSMAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [quadspi_psmar::R](R) reader structure
impl crate::Readable for QUADSPI_PSMAR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [quadspi_psmar::W](W) writer structure
impl crate::Writable for QUADSPI_PSMAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets QUADSPI_PSMAR to value 0
impl crate::Resettable for QUADSPI_PSMAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
