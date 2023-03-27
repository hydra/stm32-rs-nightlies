///Register `TIM8_RCR` reader
pub struct R(crate::R<TIM8_RCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM8_RCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM8_RCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM8_RCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM8_RCR` writer
pub struct W(crate::W<TIM8_RCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM8_RCR_SPEC>;
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
impl From<crate::W<TIM8_RCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM8_RCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `REP` reader - REP
pub type REP_R = crate::FieldReader<u16, u16>;
///Field `REP` writer - REP
pub type REP_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TIM8_RCR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - REP
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:15 - REP
    #[inline(always)]
    #[must_use]
    pub fn rep(&mut self) -> REP_W<0> {
        REP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM8 repetition counter register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim8_rcr](index.html) module
pub struct TIM8_RCR_SPEC;
impl crate::RegisterSpec for TIM8_RCR_SPEC {
    type Ux = u16;
}
///`read()` method returns [tim8_rcr::R](R) reader structure
impl crate::Readable for TIM8_RCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim8_rcr::W](W) writer structure
impl crate::Writable for TIM8_RCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM8_RCR to value 0
impl crate::Resettable for TIM8_RCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
