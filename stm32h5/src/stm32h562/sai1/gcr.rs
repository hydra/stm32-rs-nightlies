///Register `GCR` reader
pub struct R(crate::R<GCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GCR` writer
pub struct W(crate::W<GCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCR_SPEC>;
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
impl From<crate::W<GCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SYNCIN` reader - Synchronization inputs These bits are set and cleared by software. Refer to for information on how to program this field. These bits must be set when both audio blocks (A and B) are disabled. They are meaningful if one of the two audio blocks is defined to operate in synchronous mode with an external SAI (SYNCEN\[1:0\]
///= 10 in SAI_ACR1 or in SAI_BCR1 registers).
pub type SYNCIN_R = crate::FieldReader<u8, u8>;
///Field `SYNCIN` writer - Synchronization inputs These bits are set and cleared by software. Refer to for information on how to program this field. These bits must be set when both audio blocks (A and B) are disabled. They are meaningful if one of the two audio blocks is defined to operate in synchronous mode with an external SAI (SYNCEN\[1:0\]
///= 10 in SAI_ACR1 or in SAI_BCR1 registers).
pub type SYNCIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCR_SPEC, u8, u8, 2, O>;
///Field `SYNCOUT` reader - Synchronization outputs These bits are set and cleared by software.
pub type SYNCOUT_R = crate::FieldReader<u8, u8>;
///Field `SYNCOUT` writer - Synchronization outputs These bits are set and cleared by software.
pub type SYNCOUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCR_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:1 - Synchronization inputs These bits are set and cleared by software. Refer to for information on how to program this field. These bits must be set when both audio blocks (A and B) are disabled. They are meaningful if one of the two audio blocks is defined to operate in synchronous mode with an external SAI (SYNCEN\[1:0\]
    ///= 10 in SAI_ACR1 or in SAI_BCR1 registers).
    #[inline(always)]
    pub fn syncin(&self) -> SYNCIN_R {
        SYNCIN_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:5 - Synchronization outputs These bits are set and cleared by software.
    #[inline(always)]
    pub fn syncout(&self) -> SYNCOUT_R {
        SYNCOUT_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - Synchronization inputs These bits are set and cleared by software. Refer to for information on how to program this field. These bits must be set when both audio blocks (A and B) are disabled. They are meaningful if one of the two audio blocks is defined to operate in synchronous mode with an external SAI (SYNCEN\[1:0\]
    ///= 10 in SAI_ACR1 or in SAI_BCR1 registers).
    #[inline(always)]
    #[must_use]
    pub fn syncin(&mut self) -> SYNCIN_W<0> {
        SYNCIN_W::new(self)
    }
    ///Bits 4:5 - Synchronization outputs These bits are set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn syncout(&mut self) -> SYNCOUT_W<4> {
        SYNCOUT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SAI global configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gcr](index.html) module
pub struct GCR_SPEC;
impl crate::RegisterSpec for GCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gcr::R](R) reader structure
impl crate::Readable for GCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gcr::W](W) writer structure
impl crate::Writable for GCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GCR to value 0
impl crate::Resettable for GCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
