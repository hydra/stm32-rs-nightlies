///Register `CCR2` reader
pub struct R(crate::R<CCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCR2` writer
pub struct W(crate::W<CCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR2_SPEC>;
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
impl From<crate::W<CCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CCR2_L` reader - Low Capture/Compare 2 value
pub type CCR2_L_R = crate::FieldReader<u16, u16>;
///Field `CCR2_L` writer - Low Capture/Compare 2 value
pub type CCR2_L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR2_SPEC, u16, u16, 16, O>;
///Field `CCR2_H` reader - High Capture/Compare 2 value (TIM2 only)
pub type CCR2_H_R = crate::FieldReader<u16, u16>;
///Field `CCR2_H` writer - High Capture/Compare 2 value (TIM2 only)
pub type CCR2_H_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR2_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Low Capture/Compare 2 value
    #[inline(always)]
    pub fn ccr2_l(&self) -> CCR2_L_R {
        CCR2_L_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - High Capture/Compare 2 value (TIM2 only)
    #[inline(always)]
    pub fn ccr2_h(&self) -> CCR2_H_R {
        CCR2_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Low Capture/Compare 2 value
    #[inline(always)]
    #[must_use]
    pub fn ccr2_l(&mut self) -> CCR2_L_W<0> {
        CCR2_L_W::new(self)
    }
    ///Bits 16:31 - High Capture/Compare 2 value (TIM2 only)
    #[inline(always)]
    #[must_use]
    pub fn ccr2_h(&mut self) -> CCR2_H_W<16> {
        CCR2_H_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///capture/compare register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr2](index.html) module
pub struct CCR2_SPEC;
impl crate::RegisterSpec for CCR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccr2::R](R) reader structure
impl crate::Readable for CCR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccr2::W](W) writer structure
impl crate::Writable for CCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCR2 to value 0
impl crate::Resettable for CCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
