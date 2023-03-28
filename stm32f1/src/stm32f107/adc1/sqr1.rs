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
///Field `SQ13` reader - 13th conversion in regular sequence
pub type SQ13_R = crate::FieldReader<u8, u8>;
///Field `SQ13` writer - 13th conversion in regular sequence
pub type SQ13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
///Field `SQ14` reader - 14th conversion in regular sequence
pub type SQ14_R = crate::FieldReader<u8, u8>;
///Field `SQ14` writer - 14th conversion in regular sequence
pub type SQ14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
///Field `SQ15` reader - 15th conversion in regular sequence
pub type SQ15_R = crate::FieldReader<u8, u8>;
///Field `SQ15` writer - 15th conversion in regular sequence
pub type SQ15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
///Field `SQ16` reader - 16th conversion in regular sequence
pub type SQ16_R = crate::FieldReader<u8, u8>;
///Field `SQ16` writer - 16th conversion in regular sequence
pub type SQ16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR1_SPEC, u8, u8, 5, O>;
///Field `L` reader - Regular channel sequence length
pub type L_R = crate::FieldReader<u8, u8>;
///Field `L` writer - Regular channel sequence length
pub type L_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SQR1_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:4 - 13th conversion in regular sequence
    #[inline(always)]
    pub fn sq13(&self) -> SQ13_R {
        SQ13_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9 - 14th conversion in regular sequence
    #[inline(always)]
    pub fn sq14(&self) -> SQ14_R {
        SQ14_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14 - 15th conversion in regular sequence
    #[inline(always)]
    pub fn sq15(&self) -> SQ15_R {
        SQ15_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bits 15:19 - 16th conversion in regular sequence
    #[inline(always)]
    pub fn sq16(&self) -> SQ16_R {
        SQ16_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bits 20:23 - Regular channel sequence length
    #[inline(always)]
    pub fn l(&self) -> L_R {
        L_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:4 - 13th conversion in regular sequence
    #[inline(always)]
    #[must_use]
    pub fn sq13(&mut self) -> SQ13_W<0> {
        SQ13_W::new(self)
    }
    ///Bits 5:9 - 14th conversion in regular sequence
    #[inline(always)]
    #[must_use]
    pub fn sq14(&mut self) -> SQ14_W<5> {
        SQ14_W::new(self)
    }
    ///Bits 10:14 - 15th conversion in regular sequence
    #[inline(always)]
    #[must_use]
    pub fn sq15(&mut self) -> SQ15_W<10> {
        SQ15_W::new(self)
    }
    ///Bits 15:19 - 16th conversion in regular sequence
    #[inline(always)]
    #[must_use]
    pub fn sq16(&mut self) -> SQ16_W<15> {
        SQ16_W::new(self)
    }
    ///Bits 20:23 - Regular channel sequence length
    #[inline(always)]
    #[must_use]
    pub fn l(&mut self) -> L_W<20> {
        L_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///regular sequence register 1
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
