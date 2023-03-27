///Register `MDF_OLD5THHR` reader
pub struct R(crate::R<MDF_OLD5THHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDF_OLD5THHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDF_OLD5THHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDF_OLD5THHR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MDF_OLD5THHR` writer
pub struct W(crate::W<MDF_OLD5THHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDF_OLD5THHR_SPEC>;
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
impl From<crate::W<MDF_OLD5THHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDF_OLD5THHR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OLDTHH` reader - OLD High Threshold Value Set and cleared by software. OLDTHH represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHH. This field can be write-protected, please refer to Section 1.4.15: Register protection for details
pub type OLDTHH_R = crate::FieldReader<u32, u32>;
///Field `OLDTHH` writer - OLD High Threshold Value Set and cleared by software. OLDTHH represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHH. This field can be write-protected, please refer to Section 1.4.15: Register protection for details
pub type OLDTHH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MDF_OLD5THHR_SPEC, u32, u32, 26, O>;
impl R {
    ///Bits 0:25 - OLD High Threshold Value Set and cleared by software. OLDTHH represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHH. This field can be write-protected, please refer to Section 1.4.15: Register protection for details
    #[inline(always)]
    pub fn oldthh(&self) -> OLDTHH_R {
        OLDTHH_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    ///Bits 0:25 - OLD High Threshold Value Set and cleared by software. OLDTHH represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHH. This field can be write-protected, please refer to Section 1.4.15: Register protection for details
    #[inline(always)]
    #[must_use]
    pub fn oldthh(&mut self) -> OLDTHH_W<0> {
        OLDTHH_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used for the adjustment of the Out-off Limit high threshold.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdf_old5thhr](index.html) module
pub struct MDF_OLD5THHR_SPEC;
impl crate::RegisterSpec for MDF_OLD5THHR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdf_old5thhr::R](R) reader structure
impl crate::Readable for MDF_OLD5THHR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mdf_old5thhr::W](W) writer structure
impl crate::Writable for MDF_OLD5THHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MDF_OLD5THHR to value 0
impl crate::Resettable for MDF_OLD5THHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
