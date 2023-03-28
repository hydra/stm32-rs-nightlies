///Register `SQR2` reader
pub struct R(crate::R<SQR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SQR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SQR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SQR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SQR2` writer
pub struct W(crate::W<SQR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SQR2_SPEC>;
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
impl From<crate::W<SQR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SQR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SQ7` reader - 7th conversion in regular sequence
pub type SQ7_R = crate::FieldReader<u8, u8>;
///Field `SQ7` writer - 7th conversion in regular sequence
pub type SQ7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
///Field `SQ8` reader - 8th conversion in regular sequence
pub type SQ8_R = crate::FieldReader<u8, u8>;
///Field `SQ8` writer - 8th conversion in regular sequence
pub type SQ8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
///Field `SQ9` reader - 9th conversion in regular sequence
pub type SQ9_R = crate::FieldReader<u8, u8>;
///Field `SQ9` writer - 9th conversion in regular sequence
pub type SQ9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
///Field `SQ10` reader - 10th conversion in regular sequence
pub type SQ10_R = crate::FieldReader<u8, u8>;
///Field `SQ10` writer - 10th conversion in regular sequence
pub type SQ10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
///Field `SQ11` reader - 11th conversion in regular sequence
pub type SQ11_R = crate::FieldReader<u8, u8>;
///Field `SQ11` writer - 11th conversion in regular sequence
pub type SQ11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
///Field `SQ12` reader - 12th conversion in regular sequence
pub type SQ12_R = crate::FieldReader<u8, u8>;
///Field `SQ12` writer - 12th conversion in regular sequence
pub type SQ12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
impl R {
    ///Bits 0:4 - 7th conversion in regular sequence
    #[inline(always)]
    pub fn sq7(&self) -> SQ7_R {
        SQ7_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9 - 8th conversion in regular sequence
    #[inline(always)]
    pub fn sq8(&self) -> SQ8_R {
        SQ8_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14 - 9th conversion in regular sequence
    #[inline(always)]
    pub fn sq9(&self) -> SQ9_R {
        SQ9_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bits 15:19 - 10th conversion in regular sequence
    #[inline(always)]
    pub fn sq10(&self) -> SQ10_R {
        SQ10_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bits 20:24 - 11th conversion in regular sequence
    #[inline(always)]
    pub fn sq11(&self) -> SQ11_R {
        SQ11_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    ///Bits 25:29 - 12th conversion in regular sequence
    #[inline(always)]
    pub fn sq12(&self) -> SQ12_R {
        SQ12_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:4 - 7th conversion in regular sequence
    #[inline(always)]
    #[must_use]
    pub fn sq7(&mut self) -> SQ7_W<0> {
        SQ7_W::new(self)
    }
    ///Bits 5:9 - 8th conversion in regular sequence
    #[inline(always)]
    #[must_use]
    pub fn sq8(&mut self) -> SQ8_W<5> {
        SQ8_W::new(self)
    }
    ///Bits 10:14 - 9th conversion in regular sequence
    #[inline(always)]
    #[must_use]
    pub fn sq9(&mut self) -> SQ9_W<10> {
        SQ9_W::new(self)
    }
    ///Bits 15:19 - 10th conversion in regular sequence
    #[inline(always)]
    #[must_use]
    pub fn sq10(&mut self) -> SQ10_W<15> {
        SQ10_W::new(self)
    }
    ///Bits 20:24 - 11th conversion in regular sequence
    #[inline(always)]
    #[must_use]
    pub fn sq11(&mut self) -> SQ11_W<20> {
        SQ11_W::new(self)
    }
    ///Bits 25:29 - 12th conversion in regular sequence
    #[inline(always)]
    #[must_use]
    pub fn sq12(&mut self) -> SQ12_W<25> {
        SQ12_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///regular sequence register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sqr2](index.html) module
pub struct SQR2_SPEC;
impl crate::RegisterSpec for SQR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [sqr2::R](R) reader structure
impl crate::Readable for SQR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sqr2::W](W) writer structure
impl crate::Writable for SQR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SQR2 to value 0
impl crate::Resettable for SQR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
