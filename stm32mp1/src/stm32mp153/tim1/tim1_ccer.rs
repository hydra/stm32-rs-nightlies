///Register `TIM1_CCER` reader
pub struct R(crate::R<TIM1_CCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM1_CCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM1_CCER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM1_CCER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM1_CCER` writer
pub struct W(crate::W<TIM1_CCER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM1_CCER_SPEC>;
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
impl From<crate::W<TIM1_CCER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM1_CCER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CC1E` reader - CC1E
pub type CC1E_R = crate::BitReader<bool>;
///Field `CC1E` writer - CC1E
pub type CC1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCER_SPEC, bool, O>;
///Field `CC1P` reader - CC1P
pub type CC1P_R = crate::BitReader<bool>;
///Field `CC1P` writer - CC1P
pub type CC1P_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCER_SPEC, bool, O>;
///Field `CC1NE` reader - CC1NE
pub type CC1NE_R = crate::BitReader<bool>;
///Field `CC1NE` writer - CC1NE
pub type CC1NE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCER_SPEC, bool, O>;
///Field `CC1NP` reader - CC1NP
pub type CC1NP_R = crate::BitReader<bool>;
///Field `CC1NP` writer - CC1NP
pub type CC1NP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCER_SPEC, bool, O>;
///Field `CC2E` reader - CC2E
pub type CC2E_R = crate::BitReader<bool>;
///Field `CC2E` writer - CC2E
pub type CC2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCER_SPEC, bool, O>;
///Field `CC2P` reader - CC2P
pub type CC2P_R = crate::BitReader<bool>;
///Field `CC2P` writer - CC2P
pub type CC2P_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCER_SPEC, bool, O>;
///Field `CC2NE` reader - CC2NE
pub type CC2NE_R = crate::BitReader<bool>;
///Field `CC2NE` writer - CC2NE
pub type CC2NE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCER_SPEC, bool, O>;
///Field `CC2NP` reader - CC2NP
pub type CC2NP_R = crate::BitReader<bool>;
///Field `CC2NP` writer - CC2NP
pub type CC2NP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCER_SPEC, bool, O>;
///Field `CC3E` reader - CC3E
pub type CC3E_R = crate::BitReader<bool>;
///Field `CC3E` writer - CC3E
pub type CC3E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCER_SPEC, bool, O>;
///Field `CC3P` reader - CC3P
pub type CC3P_R = crate::BitReader<bool>;
///Field `CC3P` writer - CC3P
pub type CC3P_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCER_SPEC, bool, O>;
///Field `CC3NE` reader - CC3NE
pub type CC3NE_R = crate::BitReader<bool>;
///Field `CC3NE` writer - CC3NE
pub type CC3NE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCER_SPEC, bool, O>;
///Field `CC3NP` reader - CC3NP
pub type CC3NP_R = crate::BitReader<bool>;
///Field `CC3NP` writer - CC3NP
pub type CC3NP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCER_SPEC, bool, O>;
///Field `CC4E` reader - CC4E
pub type CC4E_R = crate::BitReader<bool>;
///Field `CC4E` writer - CC4E
pub type CC4E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCER_SPEC, bool, O>;
///Field `CC4P` reader - CC4P
pub type CC4P_R = crate::BitReader<bool>;
///Field `CC4P` writer - CC4P
pub type CC4P_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCER_SPEC, bool, O>;
///Field `CC4NP` reader - CC4NP
pub type CC4NP_R = crate::BitReader<bool>;
///Field `CC4NP` writer - CC4NP
pub type CC4NP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCER_SPEC, bool, O>;
///Field `CC5E` reader - CC5E
pub type CC5E_R = crate::BitReader<bool>;
///Field `CC5E` writer - CC5E
pub type CC5E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCER_SPEC, bool, O>;
///Field `CC5P` reader - CC5P
pub type CC5P_R = crate::BitReader<bool>;
///Field `CC5P` writer - CC5P
pub type CC5P_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCER_SPEC, bool, O>;
///Field `CC6E` reader - CC6E
pub type CC6E_R = crate::BitReader<bool>;
///Field `CC6E` writer - CC6E
pub type CC6E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCER_SPEC, bool, O>;
///Field `CC6P` reader - CC6P
pub type CC6P_R = crate::BitReader<bool>;
///Field `CC6P` writer - CC6P
pub type CC6P_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM1_CCER_SPEC, bool, O>;
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
    ///Bit 2 - CC1NE
    #[inline(always)]
    pub fn cc1ne(&self) -> CC1NE_R {
        CC1NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CC1NP
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CC2E
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CC2P
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CC2NE
    #[inline(always)]
    pub fn cc2ne(&self) -> CC2NE_R {
        CC2NE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CC2NP
    #[inline(always)]
    pub fn cc2np(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CC3E
    #[inline(always)]
    pub fn cc3e(&self) -> CC3E_R {
        CC3E_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CC3P
    #[inline(always)]
    pub fn cc3p(&self) -> CC3P_R {
        CC3P_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CC3NE
    #[inline(always)]
    pub fn cc3ne(&self) -> CC3NE_R {
        CC3NE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CC3NP
    #[inline(always)]
    pub fn cc3np(&self) -> CC3NP_R {
        CC3NP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CC4E
    #[inline(always)]
    pub fn cc4e(&self) -> CC4E_R {
        CC4E_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CC4P
    #[inline(always)]
    pub fn cc4p(&self) -> CC4P_R {
        CC4P_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - CC4NP
    #[inline(always)]
    pub fn cc4np(&self) -> CC4NP_R {
        CC4NP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - CC5E
    #[inline(always)]
    pub fn cc5e(&self) -> CC5E_R {
        CC5E_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CC5P
    #[inline(always)]
    pub fn cc5p(&self) -> CC5P_R {
        CC5P_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - CC6E
    #[inline(always)]
    pub fn cc6e(&self) -> CC6E_R {
        CC6E_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - CC6P
    #[inline(always)]
    pub fn cc6p(&self) -> CC6P_R {
        CC6P_R::new(((self.bits >> 21) & 1) != 0)
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
    ///Bit 2 - CC1NE
    #[inline(always)]
    #[must_use]
    pub fn cc1ne(&mut self) -> CC1NE_W<2> {
        CC1NE_W::new(self)
    }
    ///Bit 3 - CC1NP
    #[inline(always)]
    #[must_use]
    pub fn cc1np(&mut self) -> CC1NP_W<3> {
        CC1NP_W::new(self)
    }
    ///Bit 4 - CC2E
    #[inline(always)]
    #[must_use]
    pub fn cc2e(&mut self) -> CC2E_W<4> {
        CC2E_W::new(self)
    }
    ///Bit 5 - CC2P
    #[inline(always)]
    #[must_use]
    pub fn cc2p(&mut self) -> CC2P_W<5> {
        CC2P_W::new(self)
    }
    ///Bit 6 - CC2NE
    #[inline(always)]
    #[must_use]
    pub fn cc2ne(&mut self) -> CC2NE_W<6> {
        CC2NE_W::new(self)
    }
    ///Bit 7 - CC2NP
    #[inline(always)]
    #[must_use]
    pub fn cc2np(&mut self) -> CC2NP_W<7> {
        CC2NP_W::new(self)
    }
    ///Bit 8 - CC3E
    #[inline(always)]
    #[must_use]
    pub fn cc3e(&mut self) -> CC3E_W<8> {
        CC3E_W::new(self)
    }
    ///Bit 9 - CC3P
    #[inline(always)]
    #[must_use]
    pub fn cc3p(&mut self) -> CC3P_W<9> {
        CC3P_W::new(self)
    }
    ///Bit 10 - CC3NE
    #[inline(always)]
    #[must_use]
    pub fn cc3ne(&mut self) -> CC3NE_W<10> {
        CC3NE_W::new(self)
    }
    ///Bit 11 - CC3NP
    #[inline(always)]
    #[must_use]
    pub fn cc3np(&mut self) -> CC3NP_W<11> {
        CC3NP_W::new(self)
    }
    ///Bit 12 - CC4E
    #[inline(always)]
    #[must_use]
    pub fn cc4e(&mut self) -> CC4E_W<12> {
        CC4E_W::new(self)
    }
    ///Bit 13 - CC4P
    #[inline(always)]
    #[must_use]
    pub fn cc4p(&mut self) -> CC4P_W<13> {
        CC4P_W::new(self)
    }
    ///Bit 15 - CC4NP
    #[inline(always)]
    #[must_use]
    pub fn cc4np(&mut self) -> CC4NP_W<15> {
        CC4NP_W::new(self)
    }
    ///Bit 16 - CC5E
    #[inline(always)]
    #[must_use]
    pub fn cc5e(&mut self) -> CC5E_W<16> {
        CC5E_W::new(self)
    }
    ///Bit 17 - CC5P
    #[inline(always)]
    #[must_use]
    pub fn cc5p(&mut self) -> CC5P_W<17> {
        CC5P_W::new(self)
    }
    ///Bit 20 - CC6E
    #[inline(always)]
    #[must_use]
    pub fn cc6e(&mut self) -> CC6E_W<20> {
        CC6E_W::new(self)
    }
    ///Bit 21 - CC6P
    #[inline(always)]
    #[must_use]
    pub fn cc6p(&mut self) -> CC6P_W<21> {
        CC6P_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM1 capture/compare enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim1_ccer](index.html) module
pub struct TIM1_CCER_SPEC;
impl crate::RegisterSpec for TIM1_CCER_SPEC {
    type Ux = u32;
}
///`read()` method returns [tim1_ccer::R](R) reader structure
impl crate::Readable for TIM1_CCER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim1_ccer::W](W) writer structure
impl crate::Writable for TIM1_CCER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM1_CCER to value 0
impl crate::Resettable for TIM1_CCER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
