///Register `TIMx_SMCR` reader
pub struct R(crate::R<TIMX_SMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_SMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_SMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_SMCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIMx_SMCR` writer
pub struct W(crate::W<TIMX_SMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_SMCR_SPEC>;
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
impl From<crate::W<TIMX_SMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_SMCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SMS` reader - Slave mode selection
pub type SMS_R = crate::FieldReader<u8, u8>;
///Field `SMS` writer - Slave mode selection
pub type SMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMX_SMCR_SPEC, u8, u8, 3, O>;
///Field `TS` reader - Trigger selection
pub type TS_R = crate::FieldReader<u8, u8>;
///Field `TS` writer - Trigger selection
pub type TS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMX_SMCR_SPEC, u8, u8, 3, O>;
///Field `MSM` reader - Master/Slave mode
pub type MSM_R = crate::BitReader<bool>;
///Field `MSM` writer - Master/Slave mode
pub type MSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMX_SMCR_SPEC, bool, O>;
///Field `SMS_3` reader - Slave mode selection - bit 3
pub type SMS_3_R = crate::BitReader<bool>;
///Field `SMS_3` writer - Slave mode selection - bit 3
pub type SMS_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMX_SMCR_SPEC, bool, O>;
///Field `TS_4_3` reader - Trigger selection
pub type TS_4_3_R = crate::FieldReader<u8, u8>;
///Field `TS_4_3` writer - Trigger selection
pub type TS_4_3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMX_SMCR_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:2 - Slave mode selection
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Trigger selection
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Master/Slave mode
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 16 - Slave mode selection - bit 3
    #[inline(always)]
    pub fn sms_3(&self) -> SMS_3_R {
        SMS_3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 20:21 - Trigger selection
    #[inline(always)]
    pub fn ts_4_3(&self) -> TS_4_3_R {
        TS_4_3_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    ///Bits 0:2 - Slave mode selection
    #[inline(always)]
    #[must_use]
    pub fn sms(&mut self) -> SMS_W<0> {
        SMS_W::new(self)
    }
    ///Bits 4:6 - Trigger selection
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<4> {
        TS_W::new(self)
    }
    ///Bit 7 - Master/Slave mode
    #[inline(always)]
    #[must_use]
    pub fn msm(&mut self) -> MSM_W<7> {
        MSM_W::new(self)
    }
    ///Bit 16 - Slave mode selection - bit 3
    #[inline(always)]
    #[must_use]
    pub fn sms_3(&mut self) -> SMS_3_W<16> {
        SMS_3_W::new(self)
    }
    ///Bits 20:21 - Trigger selection
    #[inline(always)]
    #[must_use]
    pub fn ts_4_3(&mut self) -> TS_4_3_W<20> {
        TS_4_3_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///slave mode control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timx_smcr](index.html) module
pub struct TIMX_SMCR_SPEC;
impl crate::RegisterSpec for TIMX_SMCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [timx_smcr::R](R) reader structure
impl crate::Readable for TIMX_SMCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [timx_smcr::W](W) writer structure
impl crate::Writable for TIMX_SMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIMx_SMCR to value 0
impl crate::Resettable for TIMX_SMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
