///Register `EXTI_C2EMR3` reader
pub struct R(crate::R<EXTI_C2EMR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_C2EMR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_C2EMR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_C2EMR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EXTI_C2EMR3` writer
pub struct W(crate::W<EXTI_C2EMR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_C2EMR3_SPEC>;
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
impl From<crate::W<EXTI_C2EMR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_C2EMR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EM66` reader - EM66
pub type EM66_R = crate::BitReader<bool>;
///Field `EM66` writer - EM66
pub type EM66_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_C2EMR3_SPEC, bool, O>;
impl R {
    ///Bit 2 - EM66
    #[inline(always)]
    pub fn em66(&self) -> EM66_R {
        EM66_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - EM66
    #[inline(always)]
    #[must_use]
    pub fn em66(&mut self) -> EM66_W<2> {
        EM66_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI CPU2 wakeup with event mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exti_c2emr3](index.html) module
pub struct EXTI_C2EMR3_SPEC;
impl crate::RegisterSpec for EXTI_C2EMR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [exti_c2emr3::R](R) reader structure
impl crate::Readable for EXTI_C2EMR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [exti_c2emr3::W](W) writer structure
impl crate::Writable for EXTI_C2EMR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EXTI_C2EMR3 to value 0
impl crate::Resettable for EXTI_C2EMR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
