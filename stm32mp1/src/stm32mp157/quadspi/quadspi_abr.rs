///Register `QUADSPI_ABR` reader
pub struct R(crate::R<QUADSPI_ABR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUADSPI_ABR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUADSPI_ABR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUADSPI_ABR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `QUADSPI_ABR` writer
pub struct W(crate::W<QUADSPI_ABR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QUADSPI_ABR_SPEC>;
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
impl From<crate::W<QUADSPI_ABR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QUADSPI_ABR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ALTERNATE` reader - ALTERNATE
pub type ALTERNATE_R = crate::FieldReader<u32, u32>;
///Field `ALTERNATE` writer - ALTERNATE
pub type ALTERNATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QUADSPI_ABR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - ALTERNATE
    #[inline(always)]
    pub fn alternate(&self) -> ALTERNATE_R {
        ALTERNATE_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - ALTERNATE
    #[inline(always)]
    #[must_use]
    pub fn alternate(&mut self) -> ALTERNATE_W<0> {
        ALTERNATE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///QUADSPI alternate bytes registers
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [quadspi_abr](index.html) module
pub struct QUADSPI_ABR_SPEC;
impl crate::RegisterSpec for QUADSPI_ABR_SPEC {
    type Ux = u32;
}
///`read()` method returns [quadspi_abr::R](R) reader structure
impl crate::Readable for QUADSPI_ABR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [quadspi_abr::W](W) writer structure
impl crate::Writable for QUADSPI_ABR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets QUADSPI_ABR to value 0
impl crate::Resettable for QUADSPI_ABR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
