///Register `ADF_DLY0CR` reader
pub struct R(crate::R<ADF_DLY0CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADF_DLY0CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADF_DLY0CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADF_DLY0CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADF_DLY0CR` writer
pub struct W(crate::W<ADF_DLY0CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADF_DLY0CR_SPEC>;
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
impl From<crate::W<ADF_DLY0CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADF_DLY0CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SKPDLY` reader - Delay to apply to a bitstream
pub type SKPDLY_R = crate::FieldReader<u8, u8>;
///Field `SKPDLY` writer - Delay to apply to a bitstream
pub type SKPDLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADF_DLY0CR_SPEC, u8, u8, 7, O>;
///Field `SKPBF` reader - Skip busy flag
pub type SKPBF_R = crate::BitReader<bool>;
///Field `SKPBF` writer - Skip busy flag
pub type SKPBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADF_DLY0CR_SPEC, bool, O>;
impl R {
    ///Bits 0:6 - Delay to apply to a bitstream
    #[inline(always)]
    pub fn skpdly(&self) -> SKPDLY_R {
        SKPDLY_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 31 - Skip busy flag
    #[inline(always)]
    pub fn skpbf(&self) -> SKPBF_R {
        SKPBF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:6 - Delay to apply to a bitstream
    #[inline(always)]
    #[must_use]
    pub fn skpdly(&mut self) -> SKPDLY_W<0> {
        SKPDLY_W::new(self)
    }
    ///Bit 31 - Skip busy flag
    #[inline(always)]
    #[must_use]
    pub fn skpbf(&mut self) -> SKPBF_W<31> {
        SKPBF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADF delay control register 0
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adf_dly0cr](index.html) module
pub struct ADF_DLY0CR_SPEC;
impl crate::RegisterSpec for ADF_DLY0CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [adf_dly0cr::R](R) reader structure
impl crate::Readable for ADF_DLY0CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adf_dly0cr::W](W) writer structure
impl crate::Writable for ADF_DLY0CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADF_DLY0CR to value 0
impl crate::Resettable for ADF_DLY0CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
