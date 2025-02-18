///Register `LTDC_LIPCR` reader
pub struct R(crate::R<LTDC_LIPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_LIPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_LIPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_LIPCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LTDC_LIPCR` writer
pub struct W(crate::W<LTDC_LIPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_LIPCR_SPEC>;
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
impl From<crate::W<LTDC_LIPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_LIPCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LIPOS` reader - LIPOS
pub type LIPOS_R = crate::FieldReader<u16, u16>;
///Field `LIPOS` writer - LIPOS
pub type LIPOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LTDC_LIPCR_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 0:11 - LIPOS
    #[inline(always)]
    pub fn lipos(&self) -> LIPOS_R {
        LIPOS_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - LIPOS
    #[inline(always)]
    #[must_use]
    pub fn lipos(&mut self) -> LIPOS_W<0> {
        LIPOS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register defines the position of the line interrupt. The line value to be programmed depends on the timings parameters. Refer to Figure274.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ltdc_lipcr](index.html) module
pub struct LTDC_LIPCR_SPEC;
impl crate::RegisterSpec for LTDC_LIPCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ltdc_lipcr::R](R) reader structure
impl crate::Readable for LTDC_LIPCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ltdc_lipcr::W](W) writer structure
impl crate::Writable for LTDC_LIPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LTDC_LIPCR to value 0
impl crate::Resettable for LTDC_LIPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
