///Register `CSELR` reader
pub struct R(crate::R<CSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSELR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSELR` writer
pub struct W(crate::W<CSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSELR_SPEC>;
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
impl From<crate::W<CSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSELR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MA` reader - peripheral address
pub type MA_R = crate::FieldReader<u32, u32>;
///Field `MA` writer - peripheral address
pub type MA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSELR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - peripheral address
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - peripheral address
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MA_W<0> {
        MA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///channel selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cselr](index.html) module
pub struct CSELR_SPEC;
impl crate::RegisterSpec for CSELR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cselr::R](R) reader structure
impl crate::Readable for CSELR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cselr::W](W) writer structure
impl crate::Writable for CSELR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSELR to value 0
impl crate::Resettable for CSELR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
