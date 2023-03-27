///Register `CCR` reader
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCR` writer
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TRIM` reader - Trimming code The TRIM code is a 6-bit unsigned data (minimum 000000, maximum 111111) that is set and updated according the mechanism described below. Reset: TRIM\[5:0\]
///is automatically initialized with the VRS = 0 trimming value stored in the Flash memory during the production test. VRS change: TRIM\[5:0\]
///is automatically initialized with the trimming value (corresponding to VRS setting) stored in the Flash memory during the production test. Write in TRIM\[5:0\]: User can modify the TRIM\[5:0\]
///with an arbitrary value. This is permanently disabling the control of the trimming value with VRS (until the device is reset). Note: If the user application performs the trimming, the trimming code must start from 000000 to 111111 in ascending order.
pub type TRIM_R = crate::FieldReader<u8, u8>;
///Field `TRIM` writer - Trimming code The TRIM code is a 6-bit unsigned data (minimum 000000, maximum 111111) that is set and updated according the mechanism described below. Reset: TRIM\[5:0\]
///is automatically initialized with the VRS = 0 trimming value stored in the Flash memory during the production test. VRS change: TRIM\[5:0\]
///is automatically initialized with the trimming value (corresponding to VRS setting) stored in the Flash memory during the production test. Write in TRIM\[5:0\]: User can modify the TRIM\[5:0\]
///with an arbitrary value. This is permanently disabling the control of the trimming value with VRS (until the device is reset). Note: If the user application performs the trimming, the trimming code must start from 000000 to 111111 in ascending order.
pub type TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, u8, 6, O>;
impl R {
    ///Bits 0:5 - Trimming code The TRIM code is a 6-bit unsigned data (minimum 000000, maximum 111111) that is set and updated according the mechanism described below. Reset: TRIM\[5:0\]
    ///is automatically initialized with the VRS = 0 trimming value stored in the Flash memory during the production test. VRS change: TRIM\[5:0\]
    ///is automatically initialized with the trimming value (corresponding to VRS setting) stored in the Flash memory during the production test. Write in TRIM\[5:0\]: User can modify the TRIM\[5:0\]
    ///with an arbitrary value. This is permanently disabling the control of the trimming value with VRS (until the device is reset). Note: If the user application performs the trimming, the trimming code must start from 000000 to 111111 in ascending order.
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    ///Bits 0:5 - Trimming code The TRIM code is a 6-bit unsigned data (minimum 000000, maximum 111111) that is set and updated according the mechanism described below. Reset: TRIM\[5:0\]
    ///is automatically initialized with the VRS = 0 trimming value stored in the Flash memory during the production test. VRS change: TRIM\[5:0\]
    ///is automatically initialized with the trimming value (corresponding to VRS setting) stored in the Flash memory during the production test. Write in TRIM\[5:0\]: User can modify the TRIM\[5:0\]
    ///with an arbitrary value. This is permanently disabling the control of the trimming value with VRS (until the device is reset). Note: If the user application performs the trimming, the trimming code must start from 000000 to 111111 in ascending order.
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TRIM_W<0> {
        TRIM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///VREFBUF calibration control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr](index.html) module
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccr::R](R) reader structure
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccr::W](W) writer structure
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
