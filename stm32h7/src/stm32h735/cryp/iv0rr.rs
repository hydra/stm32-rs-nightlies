///Register `IV0RR` reader
pub struct R(crate::R<IV0RR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IV0RR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IV0RR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IV0RR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IV0RR` writer
pub struct W(crate::W<IV0RR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IV0RR_SPEC>;
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
impl From<crate::W<IV0RR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IV0RR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IV` reader - IV63
pub type IV_R = crate::FieldReader<u32, u32>;
///Field `IV` writer - IV63
pub type IV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IV0RR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - IV63
    #[inline(always)]
    pub fn iv(&self) -> IV_R {
        IV_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - IV63
    #[inline(always)]
    #[must_use]
    pub fn iv(&mut self) -> IV_W<0> {
        IV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///initialization vector register 0R
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [iv0rr](index.html) module
pub struct IV0RR_SPEC;
impl crate::RegisterSpec for IV0RR_SPEC {
    type Ux = u32;
}
///`read()` method returns [iv0rr::R](R) reader structure
impl crate::Readable for IV0RR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [iv0rr::W](W) writer structure
impl crate::Writable for IV0RR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IV0RR to value 0
impl crate::Resettable for IV0RR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
