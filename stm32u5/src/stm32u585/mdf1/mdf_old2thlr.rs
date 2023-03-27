///Register `MDF_OLD2THLR` reader
pub struct R(crate::R<MDF_OLD2THLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDF_OLD2THLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDF_OLD2THLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDF_OLD2THLR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MDF_OLD2THLR` writer
pub struct W(crate::W<MDF_OLD2THLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDF_OLD2THLR_SPEC>;
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
impl From<crate::W<MDF_OLD2THLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDF_OLD2THLR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OLDTHL` reader - OLD Low Threshold Value Set and cleared by software. OLDTHL represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHL. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type OLDTHL_R = crate::FieldReader<u32, u32>;
///Field `OLDTHL` writer - OLD Low Threshold Value Set and cleared by software. OLDTHL represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHL. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type OLDTHL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MDF_OLD2THLR_SPEC, u32, u32, 26, O>;
impl R {
    ///Bits 0:25 - OLD Low Threshold Value Set and cleared by software. OLDTHL represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHL. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn oldthl(&self) -> OLDTHL_R {
        OLDTHL_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    ///Bits 0:25 - OLD Low Threshold Value Set and cleared by software. OLDTHL represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHL. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    #[must_use]
    pub fn oldthl(&mut self) -> OLDTHL_W<0> {
        OLDTHL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used for the adjustment of the Out-off Limit low threshold.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdf_old2thlr](index.html) module
pub struct MDF_OLD2THLR_SPEC;
impl crate::RegisterSpec for MDF_OLD2THLR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdf_old2thlr::R](R) reader structure
impl crate::Readable for MDF_OLD2THLR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mdf_old2thlr::W](W) writer structure
impl crate::Writable for MDF_OLD2THLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MDF_OLD2THLR to value 0
impl crate::Resettable for MDF_OLD2THLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
