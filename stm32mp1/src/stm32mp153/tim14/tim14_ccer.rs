///Register `TIM14_CCER` reader
pub struct R(crate::R<TIM14_CCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM14_CCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM14_CCER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM14_CCER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM14_CCER` writer
pub struct W(crate::W<TIM14_CCER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM14_CCER_SPEC>;
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
impl From<crate::W<TIM14_CCER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM14_CCER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CC1E` reader - CC1E
pub type CC1E_R = crate::BitReader<bool>;
///Field `CC1E` writer - CC1E
pub type CC1E_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM14_CCER_SPEC, bool, O>;
///Field `CC1P` reader - CC1P
pub type CC1P_R = crate::BitReader<bool>;
///Field `CC1P` writer - CC1P
pub type CC1P_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM14_CCER_SPEC, bool, O>;
///Field `CC1NP` reader - CC1NP
pub type CC1NP_R = crate::BitReader<bool>;
///Field `CC1NP` writer - CC1NP
pub type CC1NP_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM14_CCER_SPEC, bool, O>;
impl R {
    ///Bit 0 - CC1E
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CC1P
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - CC1NP
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CC1E
    #[inline(always)]
    #[must_use]
    pub fn cc1e(&mut self) -> CC1E_W<0> {
        CC1E_W::new(self)
    }
    ///Bit 1 - CC1P
    #[inline(always)]
    #[must_use]
    pub fn cc1p(&mut self) -> CC1P_W<1> {
        CC1P_W::new(self)
    }
    ///Bit 3 - CC1NP
    #[inline(always)]
    #[must_use]
    pub fn cc1np(&mut self) -> CC1NP_W<3> {
        CC1NP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM14 capture/compare enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim14_ccer](index.html) module
pub struct TIM14_CCER_SPEC;
impl crate::RegisterSpec for TIM14_CCER_SPEC {
    type Ux = u16;
}
///`read()` method returns [tim14_ccer::R](R) reader structure
impl crate::Readable for TIM14_CCER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim14_ccer::W](W) writer structure
impl crate::Writable for TIM14_CCER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM14_CCER to value 0
impl crate::Resettable for TIM14_CCER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
