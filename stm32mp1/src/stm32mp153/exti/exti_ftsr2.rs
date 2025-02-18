///Register `EXTI_FTSR2` reader
pub struct R(crate::R<EXTI_FTSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_FTSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_FTSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_FTSR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EXTI_FTSR2` writer
pub struct W(crate::W<EXTI_FTSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_FTSR2_SPEC>;
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
impl From<crate::W<EXTI_FTSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_FTSR2_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Contains only register bits for configurable events.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exti_ftsr2](index.html) module
pub struct EXTI_FTSR2_SPEC;
impl crate::RegisterSpec for EXTI_FTSR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [exti_ftsr2::R](R) reader structure
impl crate::Readable for EXTI_FTSR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [exti_ftsr2::W](W) writer structure
impl crate::Writable for EXTI_FTSR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EXTI_FTSR2 to value 0
impl crate::Resettable for EXTI_FTSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
