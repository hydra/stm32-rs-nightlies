///Register `DVBUSPULSE` reader
pub struct R(crate::R<DVBUSPULSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVBUSPULSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVBUSPULSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVBUSPULSE_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DVBUSPULSE` writer
pub struct W(crate::W<DVBUSPULSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DVBUSPULSE_SPEC>;
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
impl From<crate::W<DVBUSPULSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DVBUSPULSE_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DVBUSP` reader - Device VBUS pulsing time
pub type DVBUSP_R = crate::FieldReader<u16, u16>;
///Field `DVBUSP` writer - Device VBUS pulsing time
pub type DVBUSP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DVBUSPULSE_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 0:11 - Device VBUS pulsing time
    #[inline(always)]
    pub fn dvbusp(&self) -> DVBUSP_R {
        DVBUSP_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - Device VBUS pulsing time
    #[inline(always)]
    #[must_use]
    pub fn dvbusp(&mut self) -> DVBUSP_W<0> {
        DVBUSP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTG_HS device VBUS pulsing time register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dvbuspulse](index.html) module
pub struct DVBUSPULSE_SPEC;
impl crate::RegisterSpec for DVBUSPULSE_SPEC {
    type Ux = u32;
}
///`read()` method returns [dvbuspulse::R](R) reader structure
impl crate::Readable for DVBUSPULSE_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dvbuspulse::W](W) writer structure
impl crate::Writable for DVBUSPULSE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DVBUSPULSE to value 0x05b8
impl crate::Resettable for DVBUSPULSE_SPEC {
    const RESET_VALUE: Self::Ux = 0x05b8;
}
