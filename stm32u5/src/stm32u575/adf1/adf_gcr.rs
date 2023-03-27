///Register `ADF_GCR` reader
pub struct R(crate::R<ADF_GCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADF_GCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADF_GCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADF_GCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADF_GCR` writer
pub struct W(crate::W<ADF_GCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADF_GCR_SPEC>;
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
impl From<crate::W<ADF_GCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADF_GCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TRGO` reader - Trigger output control Set by software and reset by
pub type TRGO_R = crate::BitReader<bool>;
///Field `TRGO` writer - Trigger output control Set by software and reset by
pub type TRGO_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_GCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Trigger output control Set by software and reset by
    #[inline(always)]
    pub fn trgo(&self) -> TRGO_R {
        TRGO_R::new((self.bits & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Trigger output control Set by software and reset by
    #[inline(always)]
    #[must_use]
    pub fn trgo(&mut self) -> TRGO_W<0> {
        TRGO_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADF Global Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adf_gcr](index.html) module
pub struct ADF_GCR_SPEC;
impl crate::RegisterSpec for ADF_GCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [adf_gcr::R](R) reader structure
impl crate::Readable for ADF_GCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adf_gcr::W](W) writer structure
impl crate::Writable for ADF_GCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADF_GCR to value 0
impl crate::Resettable for ADF_GCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
