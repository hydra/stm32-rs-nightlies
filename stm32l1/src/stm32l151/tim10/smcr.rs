///Register `SMCR` reader
pub struct R(crate::R<SMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SMCR` writer
pub struct W(crate::W<SMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMCR_SPEC>;
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
impl From<crate::W<SMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ETF` reader - External trigger filter
pub type ETF_R = crate::FieldReader<u8, u8>;
///Field `ETF` writer - External trigger filter
pub type ETF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMCR_SPEC, u8, u8, 4, O>;
///Field `ETPS` reader - External trigger prescaler
pub type ETPS_R = crate::FieldReader<u8, u8>;
///Field `ETPS` writer - External trigger prescaler
pub type ETPS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMCR_SPEC, u8, u8, 2, O>;
///Field `ECE` reader - External clock enable
pub type ECE_R = crate::BitReader<bool>;
///Field `ECE` writer - External clock enable
pub type ECE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMCR_SPEC, bool, O>;
///Field `ETP` reader - External trigger polarity
pub type ETP_R = crate::BitReader<bool>;
///Field `ETP` writer - External trigger polarity
pub type ETP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMCR_SPEC, bool, O>;
impl R {
    ///Bits 8:11 - External trigger filter
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:13 - External trigger prescaler
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - External clock enable
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - External trigger polarity
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 8:11 - External trigger filter
    #[inline(always)]
    #[must_use]
    pub fn etf(&mut self) -> ETF_W<8> {
        ETF_W::new(self)
    }
    ///Bits 12:13 - External trigger prescaler
    #[inline(always)]
    #[must_use]
    pub fn etps(&mut self) -> ETPS_W<12> {
        ETPS_W::new(self)
    }
    ///Bit 14 - External clock enable
    #[inline(always)]
    #[must_use]
    pub fn ece(&mut self) -> ECE_W<14> {
        ECE_W::new(self)
    }
    ///Bit 15 - External trigger polarity
    #[inline(always)]
    #[must_use]
    pub fn etp(&mut self) -> ETP_W<15> {
        ETP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM10 slave mode control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [smcr](index.html) module
pub struct SMCR_SPEC;
impl crate::RegisterSpec for SMCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [smcr::R](R) reader structure
impl crate::Readable for SMCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [smcr::W](W) writer structure
impl crate::Writable for SMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SMCR to value 0
impl crate::Resettable for SMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
