///Register `DLTRCR` reader
pub struct R(crate::R<DLTRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLTRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLTRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLTRCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DLTRCR` writer
pub struct W(crate::W<DLTRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLTRCR_SPEC>;
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
impl From<crate::W<DLTRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLTRCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MRD_TIME` reader - MRD_TIME
pub type MRD_TIME_R = crate::FieldReader<u16, u16>;
///Field `MRD_TIME` writer - MRD_TIME
pub type MRD_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLTRCR_SPEC, u16, u16, 15, O>;
impl R {
    ///Bits 0:14 - MRD_TIME
    #[inline(always)]
    pub fn mrd_time(&self) -> MRD_TIME_R {
        MRD_TIME_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    ///Bits 0:14 - MRD_TIME
    #[inline(always)]
    #[must_use]
    pub fn mrd_time(&mut self) -> MRD_TIME_W<0> {
        MRD_TIME_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host data lane timer read configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dltrcr](index.html) module
pub struct DLTRCR_SPEC;
impl crate::RegisterSpec for DLTRCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dltrcr::R](R) reader structure
impl crate::Readable for DLTRCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dltrcr::W](W) writer structure
impl crate::Writable for DLTRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DLTRCR to value 0
impl crate::Resettable for DLTRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
