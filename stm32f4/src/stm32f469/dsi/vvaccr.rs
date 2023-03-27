///Register `VVACCR` reader
pub struct R(crate::R<VVACCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VVACCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VVACCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VVACCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `VVACCR` writer
pub struct W(crate::W<VVACCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VVACCR_SPEC>;
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
impl From<crate::W<VVACCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VVACCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `VA` reader - Vertical Active duration
pub type VA_R = crate::FieldReader<u16, u16>;
///Field `VA` writer - Vertical Active duration
pub type VA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VVACCR_SPEC, u16, u16, 14, O>;
impl R {
    ///Bits 0:13 - Vertical Active duration
    #[inline(always)]
    pub fn va(&self) -> VA_R {
        VA_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    ///Bits 0:13 - Vertical Active duration
    #[inline(always)]
    #[must_use]
    pub fn va(&mut self) -> VA_W<0> {
        VA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host Video VA Current Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vvaccr](index.html) module
pub struct VVACCR_SPEC;
impl crate::RegisterSpec for VVACCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [vvaccr::R](R) reader structure
impl crate::Readable for VVACCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [vvaccr::W](W) writer structure
impl crate::Writable for VVACCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets VVACCR to value 0
impl crate::Resettable for VVACCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
