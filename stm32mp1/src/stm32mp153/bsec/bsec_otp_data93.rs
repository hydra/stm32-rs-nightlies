///Register `BSEC_OTP_DATA93` reader
pub struct R(crate::R<BSEC_OTP_DATA93_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSEC_OTP_DATA93_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSEC_OTP_DATA93_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSEC_OTP_DATA93_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BSEC_OTP_DATA93` writer
pub struct W(crate::W<BSEC_OTP_DATA93_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSEC_OTP_DATA93_SPEC>;
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
impl From<crate::W<BSEC_OTP_DATA93_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSEC_OTP_DATA93_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DATA` reader - DATA
pub type DATA_R = crate::FieldReader<u32, u32>;
///Field `DATA` writer - DATA
pub type DATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BSEC_OTP_DATA93_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - DATA
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - DATA
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Several OTP directly impact BSEC behavior, such as: BSEC_OTP_DATA0\[6:0\]
///(see Table15: OTP modes definition on page175) BSEC_OTP_DATA1, 16 lsb used for SoC features control BSEC_OTP_DATA2, 2 lsb used to control the RAM handling The reset value depends on the actual OTP programmed value and the OTP mode.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bsec_otp_data93](index.html) module
pub struct BSEC_OTP_DATA93_SPEC;
impl crate::RegisterSpec for BSEC_OTP_DATA93_SPEC {
    type Ux = u32;
}
///`read()` method returns [bsec_otp_data93::R](R) reader structure
impl crate::Readable for BSEC_OTP_DATA93_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bsec_otp_data93::W](W) writer structure
impl crate::Writable for BSEC_OTP_DATA93_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BSEC_OTP_DATA93 to value 0
impl crate::Resettable for BSEC_OTP_DATA93_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
