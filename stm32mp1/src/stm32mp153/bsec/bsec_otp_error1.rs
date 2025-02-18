///Register `BSEC_OTP_ERROR1` reader
pub struct R(crate::R<BSEC_OTP_ERROR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_OTP_ERROR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_OTP_ERROR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_OTP_ERROR1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ERR` reader - ERR
pub type ERR_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - ERR
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(self.bits)
    }
}
///BSEC_OTP_ERROR0 is used to report error state of BSEC_OTP_DATA0 to BSEC_OTP_DATA31 (lower 1Kbits OTP which are protected by 2:1 redundancy). BSEC_OTP_ERROR1 is used to report error state of BSEC_OTP_DATA32 to BSEC_OTP_DATA63 which are protected by 6-bit ECC. BSEC_OTP_ERROR2 is used to report error state of BSEC_OTP_DATA64 to BSEC_OTP_DATA95 which are protected by 6-bit ECC.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bsec_otp_error1](index.html) module
pub struct BSEC_OTP_ERROR1_SPEC;
impl crate::RegisterSpec for BSEC_OTP_ERROR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [bsec_otp_error1::R](R) reader structure
impl crate::Readable for BSEC_OTP_ERROR1_SPEC {
    type Reader = R;
}
///`reset()` method sets BSEC_OTP_ERROR1 to value 0
impl crate::Resettable for BSEC_OTP_ERROR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
