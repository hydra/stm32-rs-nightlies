///Register `ARR` reader
pub struct R(crate::R<ARR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ARR` writer
pub struct W(crate::W<ARR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARR_SPEC>;
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
impl From<crate::W<ARR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ARR_L` reader - Low Auto-reload value
pub type ARR_L_R = crate::FieldReader<u16, u16>;
///Field `ARR_L` writer - Low Auto-reload value
pub type ARR_L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ARR_SPEC, u16, u16, 16, O>;
///Field `ARR_H` reader - High Auto-reload value (TIM2 only)
pub type ARR_H_R = crate::FieldReader<u16, u16>;
///Field `ARR_H` writer - High Auto-reload value (TIM2 only)
pub type ARR_H_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ARR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Low Auto-reload value
    #[inline(always)]
    pub fn arr_l(&self) -> ARR_L_R {
        ARR_L_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - High Auto-reload value (TIM2 only)
    #[inline(always)]
    pub fn arr_h(&self) -> ARR_H_R {
        ARR_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Low Auto-reload value
    #[inline(always)]
    #[must_use]
    pub fn arr_l(&mut self) -> ARR_L_W<0> {
        ARR_L_W::new(self)
    }
    ///Bits 16:31 - High Auto-reload value (TIM2 only)
    #[inline(always)]
    #[must_use]
    pub fn arr_h(&mut self) -> ARR_H_W<16> {
        ARR_H_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///auto-reload register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [arr](index.html) module
pub struct ARR_SPEC;
impl crate::RegisterSpec for ARR_SPEC {
    type Ux = u32;
}
///`read()` method returns [arr::R](R) reader structure
impl crate::Readable for ARR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [arr::W](W) writer structure
impl crate::Writable for ARR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ARR to value 0xffff_ffff
impl crate::Resettable for ARR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
