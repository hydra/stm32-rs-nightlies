///Register `TIM12_DIER` reader
pub struct R(crate::R<TIM12_DIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM12_DIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM12_DIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM12_DIER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM12_DIER` writer
pub struct W(crate::W<TIM12_DIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM12_DIER_SPEC>;
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
impl From<crate::W<TIM12_DIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM12_DIER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `UIE` reader - UIE
pub type UIE_R = crate::BitReader<bool>;
///Field `UIE` writer - UIE
pub type UIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM12_DIER_SPEC, bool, O>;
///Field `CC1IE` reader - CC1IE
pub type CC1IE_R = crate::BitReader<bool>;
///Field `CC1IE` writer - CC1IE
pub type CC1IE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM12_DIER_SPEC, bool, O>;
///Field `CC2IE` reader - CC2IE
pub type CC2IE_R = crate::BitReader<bool>;
///Field `CC2IE` writer - CC2IE
pub type CC2IE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM12_DIER_SPEC, bool, O>;
///Field `TIE` reader - TIE
pub type TIE_R = crate::BitReader<bool>;
///Field `TIE` writer - TIE
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM12_DIER_SPEC, bool, O>;
impl R {
    ///Bit 0 - UIE
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CC1IE
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CC2IE
    #[inline(always)]
    pub fn cc2ie(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 6 - TIE
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - UIE
    #[inline(always)]
    #[must_use]
    pub fn uie(&mut self) -> UIE_W<0> {
        UIE_W::new(self)
    }
    ///Bit 1 - CC1IE
    #[inline(always)]
    #[must_use]
    pub fn cc1ie(&mut self) -> CC1IE_W<1> {
        CC1IE_W::new(self)
    }
    ///Bit 2 - CC2IE
    #[inline(always)]
    #[must_use]
    pub fn cc2ie(&mut self) -> CC2IE_W<2> {
        CC2IE_W::new(self)
    }
    ///Bit 6 - TIE
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<6> {
        TIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM12 interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim12_dier](index.html) module
pub struct TIM12_DIER_SPEC;
impl crate::RegisterSpec for TIM12_DIER_SPEC {
    type Ux = u16;
}
///`read()` method returns [tim12_dier::R](R) reader structure
impl crate::Readable for TIM12_DIER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim12_dier::W](W) writer structure
impl crate::Writable for TIM12_DIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM12_DIER to value 0
impl crate::Resettable for TIM12_DIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
