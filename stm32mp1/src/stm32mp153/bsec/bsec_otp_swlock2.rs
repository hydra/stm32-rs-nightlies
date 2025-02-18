///Register `BSEC_OTP_SWLOCK2` reader
pub struct R(crate::R<BSEC_OTP_SWLOCK2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_OTP_SWLOCK2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_OTP_SWLOCK2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_OTP_SWLOCK2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BSEC_OTP_SWLOCK2` writer
pub struct W(crate::W<BSEC_OTP_SWLOCK2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSEC_OTP_SWLOCK2_SPEC>;
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
impl From<crate::W<BSEC_OTP_SWLOCK2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSEC_OTP_SWLOCK2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SWLOCK` reader - SWLOCK
pub type SWLOCK_R = crate::FieldReader<u32, u32>;
///Field `SWLOCK` writer - SWLOCK
pub type SWLOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BSEC_OTP_SWLOCK2_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - SWLOCK
    #[inline(always)]
    pub fn swlock(&self) -> SWLOCK_R {
        SWLOCK_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - SWLOCK
    #[inline(always)]
    #[must_use]
    pub fn swlock(&mut self) -> SWLOCK_W<0> {
        SWLOCK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///BSEC_OTP_SWLOCK0 is used to prevent writing to BSEC_OTP_DATA0 to BSEC_OTP_DATA31 until next system-reset. BSEC_OTP_SWLOCK1 is used to prevent writing to BSEC_OTP_DATA32 to BSEC_OTP_DATA63 until next system-reset. BSEC_OTP_SWLOCK2 is used to prevent writing to BSEC_OTP_DATA64 to BSEC_OTP_DATA95 until next system-reset. Write to shadow write locked BSEC_OTP_DATA word are silently ignored. Writing to OTP word 0 shadow is always prevented.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bsec_otp_swlock2](index.html) module
pub struct BSEC_OTP_SWLOCK2_SPEC;
impl crate::RegisterSpec for BSEC_OTP_SWLOCK2_SPEC {
    type Ux = u32;
}
///`read()` method returns [bsec_otp_swlock2::R](R) reader structure
impl crate::Readable for BSEC_OTP_SWLOCK2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bsec_otp_swlock2::W](W) writer structure
impl crate::Writable for BSEC_OTP_SWLOCK2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BSEC_OTP_SWLOCK2 to value 0x01
impl crate::Resettable for BSEC_OTP_SWLOCK2_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
