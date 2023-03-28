///Register `TIM16_AF2` reader
pub struct R(crate::R<TIM16_AF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM16_AF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM16_AF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM16_AF2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM16_AF2` writer
pub struct W(crate::W<TIM16_AF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM16_AF2_SPEC>;
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
impl From<crate::W<TIM16_AF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM16_AF2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OCRSEL` reader - tim_ocref_clr source selection These bits select the tim_ocref_clr input source. Refer to for product specific implementation. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type OCRSEL_R = crate::FieldReader<u8, u8>;
///Field `OCRSEL` writer - tim_ocref_clr source selection These bits select the tim_ocref_clr input source. Refer to for product specific implementation. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
pub type OCRSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM16_AF2_SPEC, u8, u8, 3, O>;
impl R {
    ///Bits 16:18 - tim_ocref_clr source selection These bits select the tim_ocref_clr input source. Refer to for product specific implementation. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    pub fn ocrsel(&self) -> OCRSEL_R {
        OCRSEL_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    ///Bits 16:18 - tim_ocref_clr source selection These bits select the tim_ocref_clr input source. Refer to for product specific implementation. Note: These bits can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).
    #[inline(always)]
    #[must_use]
    pub fn ocrsel(&mut self) -> OCRSEL_W<16> {
        OCRSEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM16 alternate function register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim16_af2](index.html) module
pub struct TIM16_AF2_SPEC;
impl crate::RegisterSpec for TIM16_AF2_SPEC {
    type Ux = u32;
}
///`read()` method returns [tim16_af2::R](R) reader structure
impl crate::Readable for TIM16_AF2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim16_af2::W](W) writer structure
impl crate::Writable for TIM16_AF2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM16_AF2 to value 0
impl crate::Resettable for TIM16_AF2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
