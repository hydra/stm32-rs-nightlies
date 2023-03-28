///Register `GTZC1_TZSC_MPCWM4BR` reader
pub struct R(crate::R<GTZC1_TZSC_MPCWM4BR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTZC1_TZSC_MPCWM4BR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTZC1_TZSC_MPCWM4BR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTZC1_TZSC_MPCWM4BR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GTZC1_TZSC_MPCWM4BR` writer
pub struct W(crate::W<GTZC1_TZSC_MPCWM4BR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTZC1_TZSC_MPCWM4BR_SPEC>;
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
impl From<crate::W<GTZC1_TZSC_MPCWM4BR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTZC1_TZSC_MPCWM4BR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SUBB_START` reader - Start of sub-region B This field defines the address offset of the sub-region B, to be multiplied by the granularity defined in Table 16.
pub type SUBB_START_R = crate::FieldReader<u16, u16>;
///Field `SUBB_START` writer - Start of sub-region B This field defines the address offset of the sub-region B, to be multiplied by the granularity defined in Table 16.
pub type SUBB_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GTZC1_TZSC_MPCWM4BR_SPEC, u16, u16, 11, O>;
///Field `SUBB_LENGTH` reader - Length of sub-region B This field defines the length of the sub-region B, to be multiplied by the granularity defined in Table 16. When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBB_LENGTH is applied automatically. If SUBB_LENGTH = 0, the sub-region B is disabled (SREN bit in GTZC1_TZSC_MPCMWBCFGR is cleared).
pub type SUBB_LENGTH_R = crate::FieldReader<u16, u16>;
///Field `SUBB_LENGTH` writer - Length of sub-region B This field defines the length of the sub-region B, to be multiplied by the granularity defined in Table 16. When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBB_LENGTH is applied automatically. If SUBB_LENGTH = 0, the sub-region B is disabled (SREN bit in GTZC1_TZSC_MPCMWBCFGR is cleared).
pub type SUBB_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GTZC1_TZSC_MPCWM4BR_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 0:10 - Start of sub-region B This field defines the address offset of the sub-region B, to be multiplied by the granularity defined in Table 16.
    #[inline(always)]
    pub fn subb_start(&self) -> SUBB_START_R {
        SUBB_START_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:27 - Length of sub-region B This field defines the length of the sub-region B, to be multiplied by the granularity defined in Table 16. When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBB_LENGTH is applied automatically. If SUBB_LENGTH = 0, the sub-region B is disabled (SREN bit in GTZC1_TZSC_MPCMWBCFGR is cleared).
    #[inline(always)]
    pub fn subb_length(&self) -> SUBB_LENGTH_R {
        SUBB_LENGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Start of sub-region B This field defines the address offset of the sub-region B, to be multiplied by the granularity defined in Table 16.
    #[inline(always)]
    #[must_use]
    pub fn subb_start(&mut self) -> SUBB_START_W<0> {
        SUBB_START_W::new(self)
    }
    ///Bits 16:27 - Length of sub-region B This field defines the length of the sub-region B, to be multiplied by the granularity defined in Table 16. When SUBB_START + SUBB_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBB_LENGTH is applied automatically. If SUBB_LENGTH = 0, the sub-region B is disabled (SREN bit in GTZC1_TZSC_MPCMWBCFGR is cleared).
    #[inline(always)]
    #[must_use]
    pub fn subb_length(&mut self) -> SUBB_LENGTH_W<16> {
        SUBB_LENGTH_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GTZC1 TZSC BKPSRAM sub-region B watermark register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gtzc1_tzsc_mpcwm4br](index.html) module
pub struct GTZC1_TZSC_MPCWM4BR_SPEC;
impl crate::RegisterSpec for GTZC1_TZSC_MPCWM4BR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gtzc1_tzsc_mpcwm4br::R](R) reader structure
impl crate::Readable for GTZC1_TZSC_MPCWM4BR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gtzc1_tzsc_mpcwm4br::W](W) writer structure
impl crate::Writable for GTZC1_TZSC_MPCWM4BR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GTZC1_TZSC_MPCWM4BR to value 0x0800_0000
impl crate::Resettable for GTZC1_TZSC_MPCWM4BR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800_0000;
}
