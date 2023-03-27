///Register `HWCFGR` reader
pub struct R(crate::R<HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `HWCFGR` writer
pub struct W(crate::W<HWCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWCFGR_SPEC>;
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
impl From<crate::W<HWCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ALARMB` reader - ALARMB
pub type ALARMB_R = crate::FieldReader<u8, u8>;
///Field `ALARMB` writer - ALARMB
pub type ALARMB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR_SPEC, u8, u8, 4, O>;
///Field `WAKEUP` reader - WAKEUP
pub type WAKEUP_R = crate::FieldReader<u8, u8>;
///Field `WAKEUP` writer - WAKEUP
pub type WAKEUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR_SPEC, u8, u8, 4, O>;
///Field `SMOOTH_CALIB` reader - SMOOTH_CALIB
pub type SMOOTH_CALIB_R = crate::FieldReader<u8, u8>;
///Field `SMOOTH_CALIB` writer - SMOOTH_CALIB
pub type SMOOTH_CALIB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR_SPEC, u8, u8, 4, O>;
///Field `TIMESTAMP` reader - TIMESTAMP
pub type TIMESTAMP_R = crate::FieldReader<u8, u8>;
///Field `TIMESTAMP` writer - TIMESTAMP
pub type TIMESTAMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR_SPEC, u8, u8, 4, O>;
///Field `OPTIONREG_OUT` reader - OPTIONREG_OUT
pub type OPTIONREG_OUT_R = crate::FieldReader<u8, u8>;
///Field `OPTIONREG_OUT` writer - OPTIONREG_OUT
pub type OPTIONREG_OUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR_SPEC, u8, u8, 8, O>;
///Field `TRUST_ZONE` reader - TRUST_ZONE
pub type TRUST_ZONE_R = crate::FieldReader<u8, u8>;
///Field `TRUST_ZONE` writer - TRUST_ZONE
pub type TRUST_ZONE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWCFGR_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:3 - ALARMB
    #[inline(always)]
    pub fn alarmb(&self) -> ALARMB_R {
        ALARMB_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - WAKEUP
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - SMOOTH_CALIB
    #[inline(always)]
    pub fn smooth_calib(&self) -> SMOOTH_CALIB_R {
        SMOOTH_CALIB_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - TIMESTAMP
    #[inline(always)]
    pub fn timestamp(&self) -> TIMESTAMP_R {
        TIMESTAMP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:23 - OPTIONREG_OUT
    #[inline(always)]
    pub fn optionreg_out(&self) -> OPTIONREG_OUT_R {
        OPTIONREG_OUT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:27 - TRUST_ZONE
    #[inline(always)]
    pub fn trust_zone(&self) -> TRUST_ZONE_R {
        TRUST_ZONE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - ALARMB
    #[inline(always)]
    #[must_use]
    pub fn alarmb(&mut self) -> ALARMB_W<0> {
        ALARMB_W::new(self)
    }
    ///Bits 4:7 - WAKEUP
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WAKEUP_W<4> {
        WAKEUP_W::new(self)
    }
    ///Bits 8:11 - SMOOTH_CALIB
    #[inline(always)]
    #[must_use]
    pub fn smooth_calib(&mut self) -> SMOOTH_CALIB_W<8> {
        SMOOTH_CALIB_W::new(self)
    }
    ///Bits 12:15 - TIMESTAMP
    #[inline(always)]
    #[must_use]
    pub fn timestamp(&mut self) -> TIMESTAMP_W<12> {
        TIMESTAMP_W::new(self)
    }
    ///Bits 16:23 - OPTIONREG_OUT
    #[inline(always)]
    #[must_use]
    pub fn optionreg_out(&mut self) -> OPTIONREG_OUT_W<16> {
        OPTIONREG_OUT_W::new(self)
    }
    ///Bits 24:27 - TRUST_ZONE
    #[inline(always)]
    #[must_use]
    pub fn trust_zone(&mut self) -> TRUST_ZONE_W<24> {
        TRUST_ZONE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///hardware configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hwcfgr](index.html) module
pub struct HWCFGR_SPEC;
impl crate::RegisterSpec for HWCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [hwcfgr::R](R) reader structure
impl crate::Readable for HWCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [hwcfgr::W](W) writer structure
impl crate::Writable for HWCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets HWCFGR to value 0
impl crate::Resettable for HWCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
