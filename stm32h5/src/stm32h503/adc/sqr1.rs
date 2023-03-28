///Register `SQR1` reader
pub struct R(crate::R<SQR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SQR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SQR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SQR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SQR1` writer
pub struct W(crate::W<SQR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SQR1_SPEC>;
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
impl From<crate::W<SQR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SQR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `L` reader - Regular channel sequence length These bits are written by software to define the total number of conversions in the regular channel conversion sequence. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type L_R = crate::FieldReader<u8, u8>;
///Field `L` writer - Regular channel sequence length These bits are written by software to define the total number of conversions in the regular channel conversion sequence. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 4, O>;
///Field `SQ1` reader - 1st conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 1st in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type SQ1_R = crate::FieldReader<u8, u8>;
///Field `SQ1` writer - 1st conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 1st in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type SQ1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
///Field `SQ2` reader - 2nd conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 2nd in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type SQ2_R = crate::FieldReader<u8, u8>;
///Field `SQ2` writer - 2nd conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 2nd in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type SQ2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
///Field `SQ3` reader - 3rd conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 3rd in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type SQ3_R = crate::FieldReader<u8, u8>;
///Field `SQ3` writer - 3rd conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 3rd in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type SQ3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
///Field `SQ4` reader - 4th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 4th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type SQ4_R = crate::FieldReader<u8, u8>;
///Field `SQ4` writer - 4th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 4th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
pub type SQ4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
impl R {
    ///Bits 0:3 - Regular channel sequence length These bits are written by software to define the total number of conversions in the regular channel conversion sequence. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn l(&self) -> L_R {
        L_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 6:10 - 1st conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 1st in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn sq1(&self) -> SQ1_R {
        SQ1_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 12:16 - 2nd conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 2nd in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn sq2(&self) -> SQ2_R {
        SQ2_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    ///Bits 18:22 - 3rd conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 3rd in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn sq3(&self) -> SQ3_R {
        SQ3_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    ///Bits 24:28 - 4th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 4th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    pub fn sq4(&self) -> SQ4_R {
        SQ4_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:3 - Regular channel sequence length These bits are written by software to define the total number of conversions in the regular channel conversion sequence. ... Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn l(&mut self) -> L_W<0> {
        L_W::new(self)
    }
    ///Bits 6:10 - 1st conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 1st in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn sq1(&mut self) -> SQ1_W<6> {
        SQ1_W::new(self)
    }
    ///Bits 12:16 - 2nd conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 2nd in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn sq2(&mut self) -> SQ2_W<12> {
        SQ2_W::new(self)
    }
    ///Bits 18:22 - 3rd conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 3rd in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn sq3(&mut self) -> SQ3_W<18> {
        SQ3_W::new(self)
    }
    ///Bits 24:28 - 4th conversion in regular sequence These bits are written by software with the channel number (0 to 19) assigned as the 4th in the regular conversion sequence. Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no regular conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn sq4(&mut self) -> SQ4_W<24> {
        SQ4_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC regular sequence register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sqr1](index.html) module
pub struct SQR1_SPEC;
impl crate::RegisterSpec for SQR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [sqr1::R](R) reader structure
impl crate::Readable for SQR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sqr1::W](W) writer structure
impl crate::Writable for SQR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SQR1 to value 0
impl crate::Resettable for SQR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
