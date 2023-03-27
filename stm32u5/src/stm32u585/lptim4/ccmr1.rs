///Register `CCMR1` reader
pub struct R(crate::R<CCMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCMR1` writer
pub struct W(crate::W<CCMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMR1_SPEC>;
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
impl From<crate::W<CCMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CC1SEL` reader - Capture/compare 1 selection
pub type CC1SEL_R = crate::BitReader<bool>;
///Field `CC1SEL` writer - Capture/compare 1 selection
pub type CC1SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_SPEC, bool, O>;
///Field `CC1E` reader - Capture/compare 1 output enable
pub type CC1E_R = crate::BitReader<bool>;
///Field `CC1E` writer - Capture/compare 1 output enable
pub type CC1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_SPEC, bool, O>;
///Field `CC1P` reader - Capture/compare 1 output polarity
pub type CC1P_R = crate::FieldReader<u8, u8>;
///Field `CC1P` writer - Capture/compare 1 output polarity
pub type CC1P_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR1_SPEC, u8, u8, 2, O>;
///Field `IC1PSC` reader - Input capture 1 prescaler
pub type IC1PSC_R = crate::FieldReader<u8, u8>;
///Field `IC1PSC` writer - Input capture 1 prescaler
pub type IC1PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR1_SPEC, u8, u8, 2, O>;
///Field `IC1F` reader - Input capture 1 filter
pub type IC1F_R = crate::FieldReader<u8, u8>;
///Field `IC1F` writer - Input capture 1 filter
pub type IC1F_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR1_SPEC, u8, u8, 2, O>;
///Field `CC2SEL` reader - Capture/compare 2 selection
pub type CC2SEL_R = crate::BitReader<bool>;
///Field `CC2SEL` writer - Capture/compare 2 selection
pub type CC2SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_SPEC, bool, O>;
///Field `CC2E` reader - Capture/compare 2 output enable
pub type CC2E_R = crate::BitReader<bool>;
///Field `CC2E` writer - Capture/compare 2 output enable
pub type CC2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR1_SPEC, bool, O>;
///Field `CC2P` reader - Capture/compare 2 output polarity
pub type CC2P_R = crate::FieldReader<u8, u8>;
///Field `CC2P` writer - Capture/compare 2 output polarity
pub type CC2P_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR1_SPEC, u8, u8, 2, O>;
///Field `IC2PSC` reader - Input capture 2 prescaler
pub type IC2PSC_R = crate::FieldReader<u8, u8>;
///Field `IC2PSC` writer - Input capture 2 prescaler
pub type IC2PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR1_SPEC, u8, u8, 2, O>;
///Field `IC2F` reader - Input capture 2 filter
pub type IC2F_R = crate::FieldReader<u8, u8>;
///Field `IC2F` writer - Input capture 2 filter
pub type IC2F_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR1_SPEC, u8, u8, 2, O>;
impl R {
    ///Bit 0 - Capture/compare 1 selection
    #[inline(always)]
    pub fn cc1sel(&self) -> CC1SEL_R {
        CC1SEL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/compare 1 output enable
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Capture/compare 1 output polarity
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 8:9 - Input capture 1 prescaler
    #[inline(always)]
    pub fn ic1psc(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 12:13 - Input capture 1 filter
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 16 - Capture/compare 2 selection
    #[inline(always)]
    pub fn cc2sel(&self) -> CC2SEL_R {
        CC2SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Capture/compare 2 output enable
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - Capture/compare 2 output polarity
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 24:25 - Input capture 2 prescaler
    #[inline(always)]
    pub fn ic2psc(&self) -> IC2PSC_R {
        IC2PSC_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 28:29 - Input capture 2 filter
    #[inline(always)]
    pub fn ic2f(&self) -> IC2F_R {
        IC2F_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - Capture/compare 1 selection
    #[inline(always)]
    #[must_use]
    pub fn cc1sel(&mut self) -> CC1SEL_W<0> {
        CC1SEL_W::new(self)
    }
    ///Bit 1 - Capture/compare 1 output enable
    #[inline(always)]
    #[must_use]
    pub fn cc1e(&mut self) -> CC1E_W<1> {
        CC1E_W::new(self)
    }
    ///Bits 2:3 - Capture/compare 1 output polarity
    #[inline(always)]
    #[must_use]
    pub fn cc1p(&mut self) -> CC1P_W<2> {
        CC1P_W::new(self)
    }
    ///Bits 8:9 - Input capture 1 prescaler
    #[inline(always)]
    #[must_use]
    pub fn ic1psc(&mut self) -> IC1PSC_W<8> {
        IC1PSC_W::new(self)
    }
    ///Bits 12:13 - Input capture 1 filter
    #[inline(always)]
    #[must_use]
    pub fn ic1f(&mut self) -> IC1F_W<12> {
        IC1F_W::new(self)
    }
    ///Bit 16 - Capture/compare 2 selection
    #[inline(always)]
    #[must_use]
    pub fn cc2sel(&mut self) -> CC2SEL_W<16> {
        CC2SEL_W::new(self)
    }
    ///Bit 17 - Capture/compare 2 output enable
    #[inline(always)]
    #[must_use]
    pub fn cc2e(&mut self) -> CC2E_W<17> {
        CC2E_W::new(self)
    }
    ///Bits 18:19 - Capture/compare 2 output polarity
    #[inline(always)]
    #[must_use]
    pub fn cc2p(&mut self) -> CC2P_W<18> {
        CC2P_W::new(self)
    }
    ///Bits 24:25 - Input capture 2 prescaler
    #[inline(always)]
    #[must_use]
    pub fn ic2psc(&mut self) -> IC2PSC_W<24> {
        IC2PSC_W::new(self)
    }
    ///Bits 28:29 - Input capture 2 filter
    #[inline(always)]
    #[must_use]
    pub fn ic2f(&mut self) -> IC2F_W<28> {
        IC2F_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LPTIM capture/compare mode register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccmr1](index.html) module
pub struct CCMR1_SPEC;
impl crate::RegisterSpec for CCMR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccmr1::R](R) reader structure
impl crate::Readable for CCMR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccmr1::W](W) writer structure
impl crate::Writable for CCMR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCMR1 to value 0
impl crate::Resettable for CCMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
