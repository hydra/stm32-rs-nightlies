///Register `CCER` reader
pub struct R(crate::R<CCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCER` writer
pub struct W(crate::W<CCER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCER_SPEC>;
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
impl From<crate::W<CCER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CC1E` reader - Capture/Compare 1 output enable
pub type CC1E_R = crate::BitReader<bool>;
///Field `CC1E` writer - Capture/Compare 1 output enable
pub type CC1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
///Field `CC1P` reader - Capture/Compare 1 output Polarity
pub type CC1P_R = crate::BitReader<bool>;
///Field `CC1P` writer - Capture/Compare 1 output Polarity
pub type CC1P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
///Field `CC1NE` reader - Capture/Compare 1 complementary output enable
pub type CC1NE_R = crate::BitReader<bool>;
///Field `CC1NE` writer - Capture/Compare 1 complementary output enable
pub type CC1NE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
///Field `CC1NP` reader - Capture/Compare 1 output Polarity
pub type CC1NP_R = crate::BitReader<bool>;
///Field `CC1NP` writer - Capture/Compare 1 output Polarity
pub type CC1NP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
///Field `CC2E` reader - Capture/Compare 2 output enable
pub type CC2E_R = crate::BitReader<bool>;
///Field `CC2E` writer - Capture/Compare 2 output enable
pub type CC2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
///Field `CC2P` reader - Capture/Compare 2 output polarity
pub type CC2P_R = crate::BitReader<bool>;
///Field `CC2P` writer - Capture/Compare 2 output polarity
pub type CC2P_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
///Field `CC2NP` reader - Capture/Compare 2 complementary output polarity
pub type CC2NP_R = crate::BitReader<bool>;
///Field `CC2NP` writer - Capture/Compare 2 complementary output polarity
pub type CC2NP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCER_SPEC, bool, O>;
impl R {
    ///Bit 0 - Capture/Compare 1 output enable
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/Compare 1 output Polarity
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Capture/Compare 1 complementary output enable
    #[inline(always)]
    pub fn cc1ne(&self) -> CC1NE_R {
        CC1NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/Compare 1 output Polarity
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Capture/Compare 2 output enable
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Capture/Compare 2 output polarity
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Capture/Compare 2 complementary output polarity
    #[inline(always)]
    pub fn cc2np(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Capture/Compare 1 output enable
    #[inline(always)]
    #[must_use]
    pub fn cc1e(&mut self) -> CC1E_W<0> {
        CC1E_W::new(self)
    }
    ///Bit 1 - Capture/Compare 1 output Polarity
    #[inline(always)]
    #[must_use]
    pub fn cc1p(&mut self) -> CC1P_W<1> {
        CC1P_W::new(self)
    }
    ///Bit 2 - Capture/Compare 1 complementary output enable
    #[inline(always)]
    #[must_use]
    pub fn cc1ne(&mut self) -> CC1NE_W<2> {
        CC1NE_W::new(self)
    }
    ///Bit 3 - Capture/Compare 1 output Polarity
    #[inline(always)]
    #[must_use]
    pub fn cc1np(&mut self) -> CC1NP_W<3> {
        CC1NP_W::new(self)
    }
    ///Bit 4 - Capture/Compare 2 output enable
    #[inline(always)]
    #[must_use]
    pub fn cc2e(&mut self) -> CC2E_W<4> {
        CC2E_W::new(self)
    }
    ///Bit 5 - Capture/Compare 2 output polarity
    #[inline(always)]
    #[must_use]
    pub fn cc2p(&mut self) -> CC2P_W<5> {
        CC2P_W::new(self)
    }
    ///Bit 7 - Capture/Compare 2 complementary output polarity
    #[inline(always)]
    #[must_use]
    pub fn cc2np(&mut self) -> CC2NP_W<7> {
        CC2NP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///capture/compare enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccer](index.html) module
pub struct CCER_SPEC;
impl crate::RegisterSpec for CCER_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccer::R](R) reader structure
impl crate::Readable for CCER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccer::W](W) writer structure
impl crate::Writable for CCER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCER to value 0
impl crate::Resettable for CCER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
