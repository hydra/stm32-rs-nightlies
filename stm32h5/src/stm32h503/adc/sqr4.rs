///Register `SQR4` reader
pub struct R(crate::R<SQR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SQR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SQR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SQR4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SQR4` writer
pub struct W(crate::W<SQR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SQR4_SPEC>;
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
impl From<crate::W<SQR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SQR4_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SQ15` reader - 15th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 15th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type SQ15_R = crate::FieldReader<u8, u8>;
///Field `SQ15` writer - 15th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 15th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type SQ15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR4_SPEC, u8, u8, 5, O>;
///Field `SQ16` reader - 16th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 16th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type SQ16_R = crate::FieldReader<u8, u8>;
///Field `SQ16` writer - 16th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 16th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type SQ16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR4_SPEC, u8, u8, 5, O>;
impl R {
    ///Bits 0:4 - 15th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 15th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn sq15(&self) -> SQ15_R {
        SQ15_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 6:10 - 16th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 16th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn sq16(&self) -> SQ16_R {
        SQ16_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:4 - 15th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 15th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn sq15(&mut self) -> SQ15_W<0> {
        SQ15_W::new(self)
    }
    ///Bits 6:10 - 16th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 16th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn sq16(&mut self) -> SQ16_W<6> {
        SQ16_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC regular sequence register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sqr4](index.html) module
pub struct SQR4_SPEC;
impl crate::RegisterSpec for SQR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [sqr4::R](R) reader structure
impl crate::Readable for SQR4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sqr4::W](W) writer structure
impl crate::Writable for SQR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SQR4 to value 0
impl crate::Resettable for SQR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
