///Register `LTR` reader
pub struct R(crate::R<LTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LTR` writer
pub struct W(crate::W<LTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTR_SPEC>;
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
impl From<crate::W<LTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LT` reader - Analog watchdog lower threshold
pub type LT_R = crate::FieldReader<u16, u16>;
///Field `LT` writer - Analog watchdog lower threshold
pub type LT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, LTR_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 0:11 - Analog watchdog lower threshold
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - Analog watchdog lower threshold
    #[inline(always)]
    #[must_use]
    pub fn lt(&mut self) -> LT_W<0> {
        LT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///watchdog lower threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ltr](index.html) module
pub struct LTR_SPEC;
impl crate::RegisterSpec for LTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ltr::R](R) reader structure
impl crate::Readable for LTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ltr::W](W) writer structure
impl crate::Writable for LTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LTR to value 0
impl crate::Resettable for LTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
