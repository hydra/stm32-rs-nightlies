///Register `CCR6` reader
pub struct R(crate::R<CCR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR6_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCR6` writer
pub struct W(crate::W<CCR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR6_SPEC>;
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
impl From<crate::W<CCR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR6_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CCR` reader - Capture/Compare value
pub type CCR_R = crate::FieldReader<u32, u32>;
///Field `CCR` writer - Capture/Compare value
pub type CCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR6_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Capture/Compare value
    #[inline(always)]
    pub fn ccr(&self) -> CCR_R {
        CCR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Capture/Compare value
    #[inline(always)]
    #[must_use]
    pub fn ccr(&mut self) -> CCR_W<0> {
        CCR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///capture/compare register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr6](index.html) module
pub struct CCR6_SPEC;
impl crate::RegisterSpec for CCR6_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccr6::R](R) reader structure
impl crate::Readable for CCR6_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccr6::W](W) writer structure
impl crate::Writable for CCR6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCR6 to value 0
impl crate::Resettable for CCR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
