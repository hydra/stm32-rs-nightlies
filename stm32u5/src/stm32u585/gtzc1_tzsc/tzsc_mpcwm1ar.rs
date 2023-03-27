///Register `TZSC_MPCWM1AR` reader
pub struct R(crate::R<TZSC_MPCWM1AR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZSC_MPCWM1AR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZSC_MPCWM1AR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZSC_MPCWM1AR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TZSC_MPCWM1AR` writer
pub struct W(crate::W<TZSC_MPCWM1AR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZSC_MPCWM1AR_SPEC>;
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
impl From<crate::W<TZSC_MPCWM1AR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZSC_MPCWM1AR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SUBA_START` reader - Start of sub-region A
pub type SUBA_START_R = crate::FieldReader<u16, u16>;
///Field `SUBA_START` writer - Start of sub-region A
pub type SUBA_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZSC_MPCWM1AR_SPEC, u16, u16, 11, O>;
///Field `SUBA_LENGTH` reader - Length of sub-region A
pub type SUBA_LENGTH_R = crate::FieldReader<u16, u16>;
///Field `SUBA_LENGTH` writer - Length of sub-region A
pub type SUBA_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZSC_MPCWM1AR_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 0:10 - Start of sub-region A
    #[inline(always)]
    pub fn suba_start(&self) -> SUBA_START_R {
        SUBA_START_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:27 - Length of sub-region A
    #[inline(always)]
    pub fn suba_length(&self) -> SUBA_LENGTH_R {
        SUBA_LENGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Start of sub-region A
    #[inline(always)]
    #[must_use]
    pub fn suba_start(&mut self) -> SUBA_START_W<0> {
        SUBA_START_W::new(self)
    }
    ///Bits 16:27 - Length of sub-region A
    #[inline(always)]
    #[must_use]
    pub fn suba_length(&mut self) -> SUBA_LENGTH_W<16> {
        SUBA_LENGTH_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZSC memory 1 sub-region A watermark register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzsc_mpcwm1ar](index.html) module
pub struct TZSC_MPCWM1AR_SPEC;
impl crate::RegisterSpec for TZSC_MPCWM1AR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzsc_mpcwm1ar::R](R) reader structure
impl crate::Readable for TZSC_MPCWM1AR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tzsc_mpcwm1ar::W](W) writer structure
impl crate::Writable for TZSC_MPCWM1AR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TZSC_MPCWM1AR to value 0
impl crate::Resettable for TZSC_MPCWM1AR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
