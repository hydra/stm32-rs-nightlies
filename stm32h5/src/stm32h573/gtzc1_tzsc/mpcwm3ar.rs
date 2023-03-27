///Register `MPCWM3AR` reader
pub struct R(crate::R<MPCWM3AR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCWM3AR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCWM3AR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCWM3AR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MPCWM3AR` writer
pub struct W(crate::W<MPCWM3AR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCWM3AR_SPEC>;
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
impl From<crate::W<MPCWM3AR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCWM3AR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SUBA_START` reader - Start of sub-region A in region x This field defines the address offset of the sub-region A, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value).
pub type SUBA_START_R = crate::FieldReader<u16, u16>;
///Field `SUBA_START` writer - Start of sub-region A in region x This field defines the address offset of the sub-region A, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value).
pub type SUBA_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MPCWM3AR_SPEC, u16, u16, 11, O>;
///Field `SUBA_LENGTH` reader - Length of sub-region A in region x This field defines the length of the sub-region A, to be multiplied by the granularity defined in Table�30. When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBA_LENGTH is applied automatically. If SUBA_LENGTH = 0, the sub-region A is disabled.(SREN bit in GTZC1_TZSC_MPCMWxACFGR is cleared).
pub type SUBA_LENGTH_R = crate::FieldReader<u16, u16>;
///Field `SUBA_LENGTH` writer - Length of sub-region A in region x This field defines the length of the sub-region A, to be multiplied by the granularity defined in Table�30. When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBA_LENGTH is applied automatically. If SUBA_LENGTH = 0, the sub-region A is disabled.(SREN bit in GTZC1_TZSC_MPCMWxACFGR is cleared).
pub type SUBA_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MPCWM3AR_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 0:10 - Start of sub-region A in region x This field defines the address offset of the sub-region A, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value).
    #[inline(always)]
    pub fn suba_start(&self) -> SUBA_START_R {
        SUBA_START_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:27 - Length of sub-region A in region x This field defines the length of the sub-region A, to be multiplied by the granularity defined in Table�30. When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBA_LENGTH is applied automatically. If SUBA_LENGTH = 0, the sub-region A is disabled.(SREN bit in GTZC1_TZSC_MPCMWxACFGR is cleared).
    #[inline(always)]
    pub fn suba_length(&self) -> SUBA_LENGTH_R {
        SUBA_LENGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Start of sub-region A in region x This field defines the address offset of the sub-region A, to be multiplied by the granularity defined in Table�30, versus the start of the region x. External memories that are watermark controlled, start fully non-secure at reset when TZEN�=�0xC3. When TZEN�=�0xB4, external memories start fully secure (inverted reset value).
    #[inline(always)]
    #[must_use]
    pub fn suba_start(&mut self) -> SUBA_START_W<0> {
        SUBA_START_W::new(self)
    }
    ///Bits 16:27 - Length of sub-region A in region x This field defines the length of the sub-region A, to be multiplied by the granularity defined in Table�30. When SUBA_START + SUBA_LENGTH is higher than the maximum size allowed for the memory, a saturation of SUBA_LENGTH is applied automatically. If SUBA_LENGTH = 0, the sub-region A is disabled.(SREN bit in GTZC1_TZSC_MPCMWxACFGR is cleared).
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
///GTZC1 TZSC memory 3 sub-region A watermark register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mpcwm3ar](index.html) module
pub struct MPCWM3AR_SPEC;
impl crate::RegisterSpec for MPCWM3AR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mpcwm3ar::R](R) reader structure
impl crate::Readable for MPCWM3AR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mpcwm3ar::W](W) writer structure
impl crate::Writable for MPCWM3AR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MPCWM3AR to value 0
impl crate::Resettable for MPCWM3AR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
