///Register `TIM2_OR` reader
pub struct R(crate::R<TIM2_OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM2_OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM2_OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM2_OR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM2_OR` writer
pub struct W(crate::W<TIM2_OR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM2_OR_SPEC>;
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
impl From<crate::W<TIM2_OR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM2_OR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IT4_RMP` reader - Timer Input 4 remap
pub type IT4_RMP_R = crate::FieldReader<u8, u8>;
///Field `IT4_RMP` writer - Timer Input 4 remap
pub type IT4_RMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM2_OR_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 6:7 - Timer Input 4 remap
    #[inline(always)]
    pub fn it4_rmp(&self) -> IT4_RMP_R {
        IT4_RMP_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    ///Bits 6:7 - Timer Input 4 remap
    #[inline(always)]
    #[must_use]
    pub fn it4_rmp(&mut self) -> IT4_RMP_W<6> {
        IT4_RMP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM2 option register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim2_or](index.html) module
pub struct TIM2_OR_SPEC;
impl crate::RegisterSpec for TIM2_OR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tim2_or::R](R) reader structure
impl crate::Readable for TIM2_OR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim2_or::W](W) writer structure
impl crate::Writable for TIM2_OR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM2_OR to value 0
impl crate::Resettable for TIM2_OR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
