///Register `TIM8_SMCR` reader
pub struct R(crate::R<TIM8_SMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM8_SMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM8_SMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM8_SMCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIM8_SMCR` writer
pub struct W(crate::W<TIM8_SMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM8_SMCR_SPEC>;
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
impl From<crate::W<TIM8_SMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM8_SMCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SMS` reader - SMS
pub type SMS_R = crate::FieldReader<u8, u8>;
///Field `SMS` writer - SMS
pub type SMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM8_SMCR_SPEC, u8, u8, 3, O>;
///Field `TS` reader - TS
pub type TS_R = crate::FieldReader<u8, u8>;
///Field `TS` writer - TS
pub type TS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM8_SMCR_SPEC, u8, u8, 3, O>;
///Field `MSM` reader - MSM
pub type MSM_R = crate::BitReader<bool>;
///Field `MSM` writer - MSM
pub type MSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_SMCR_SPEC, bool, O>;
///Field `ETF` reader - ETF
pub type ETF_R = crate::FieldReader<u8, u8>;
///Field `ETF` writer - ETF
pub type ETF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM8_SMCR_SPEC, u8, u8, 4, O>;
///Field `ETPS` reader - ETPS
pub type ETPS_R = crate::FieldReader<u8, u8>;
///Field `ETPS` writer - ETPS
pub type ETPS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM8_SMCR_SPEC, u8, u8, 2, O>;
///Field `ECE` reader - ECE
pub type ECE_R = crate::BitReader<bool>;
///Field `ECE` writer - ECE
pub type ECE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_SMCR_SPEC, bool, O>;
///Field `ETP` reader - ETP
pub type ETP_R = crate::BitReader<bool>;
///Field `ETP` writer - ETP
pub type ETP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_SMCR_SPEC, bool, O>;
///Field `SMS3` reader - SMS3
pub type SMS3_R = crate::BitReader<bool>;
///Field `SMS3` writer - SMS3
pub type SMS3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_SMCR_SPEC, bool, O>;
///Field `TS3` reader - TS3
pub type TS3_R = crate::BitReader<bool>;
///Field `TS3` writer - TS3
pub type TS3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_SMCR_SPEC, bool, O>;
///Field `TS4` reader - TS4
pub type TS4_R = crate::BitReader<bool>;
///Field `TS4` writer - TS4
pub type TS4_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM8_SMCR_SPEC, bool, O>;
impl R {
    ///Bits 0:2 - SMS
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - TS
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - MSM
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - ETF
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:13 - ETPS
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - ECE
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - ETP
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SMS3
    #[inline(always)]
    pub fn sms3(&self) -> SMS3_R {
        SMS3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - TS3
    #[inline(always)]
    pub fn ts3(&self) -> TS3_R {
        TS3_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - TS4
    #[inline(always)]
    pub fn ts4(&self) -> TS4_R {
        TS4_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - SMS
    #[inline(always)]
    #[must_use]
    pub fn sms(&mut self) -> SMS_W<0> {
        SMS_W::new(self)
    }
    ///Bits 4:6 - TS
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<4> {
        TS_W::new(self)
    }
    ///Bit 7 - MSM
    #[inline(always)]
    #[must_use]
    pub fn msm(&mut self) -> MSM_W<7> {
        MSM_W::new(self)
    }
    ///Bits 8:11 - ETF
    #[inline(always)]
    #[must_use]
    pub fn etf(&mut self) -> ETF_W<8> {
        ETF_W::new(self)
    }
    ///Bits 12:13 - ETPS
    #[inline(always)]
    #[must_use]
    pub fn etps(&mut self) -> ETPS_W<12> {
        ETPS_W::new(self)
    }
    ///Bit 14 - ECE
    #[inline(always)]
    #[must_use]
    pub fn ece(&mut self) -> ECE_W<14> {
        ECE_W::new(self)
    }
    ///Bit 15 - ETP
    #[inline(always)]
    #[must_use]
    pub fn etp(&mut self) -> ETP_W<15> {
        ETP_W::new(self)
    }
    ///Bit 16 - SMS3
    #[inline(always)]
    #[must_use]
    pub fn sms3(&mut self) -> SMS3_W<16> {
        SMS3_W::new(self)
    }
    ///Bit 20 - TS3
    #[inline(always)]
    #[must_use]
    pub fn ts3(&mut self) -> TS3_W<20> {
        TS3_W::new(self)
    }
    ///Bit 21 - TS4
    #[inline(always)]
    #[must_use]
    pub fn ts4(&mut self) -> TS4_W<21> {
        TS4_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM8 slave mode control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tim8_smcr](index.html) module
pub struct TIM8_SMCR_SPEC;
impl crate::RegisterSpec for TIM8_SMCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tim8_smcr::R](R) reader structure
impl crate::Readable for TIM8_SMCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tim8_smcr::W](W) writer structure
impl crate::Writable for TIM8_SMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIM8_SMCR to value 0
impl crate::Resettable for TIM8_SMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
