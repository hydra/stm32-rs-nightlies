///Register `JSQR` reader
pub struct R(crate::R<JSQR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JSQR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JSQR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JSQR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `JSQR` writer
pub struct W(crate::W<JSQR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JSQR_SPEC>;
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
impl From<crate::W<JSQR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JSQR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `JSQ1` reader - 1st conversion in injected sequence
pub type JSQ1_R = crate::FieldReader<u8, u8>;
///Field `JSQ1` writer - 1st conversion in injected sequence
pub type JSQ1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JSQR_SPEC, u8, u8, 5, O>;
///Field `JSQ2` reader - 2nd conversion in injected sequence
pub type JSQ2_R = crate::FieldReader<u8, u8>;
///Field `JSQ2` writer - 2nd conversion in injected sequence
pub type JSQ2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JSQR_SPEC, u8, u8, 5, O>;
///Field `JSQ3` reader - 3rd conversion in injected sequence
pub type JSQ3_R = crate::FieldReader<u8, u8>;
///Field `JSQ3` writer - 3rd conversion in injected sequence
pub type JSQ3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JSQR_SPEC, u8, u8, 5, O>;
///Field `JSQ4` reader - 4th conversion in injected sequence
pub type JSQ4_R = crate::FieldReader<u8, u8>;
///Field `JSQ4` writer - 4th conversion in injected sequence
pub type JSQ4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JSQR_SPEC, u8, u8, 5, O>;
///Field `JL` reader - Injected sequence length
pub type JL_R = crate::FieldReader<u8, u8>;
///Field `JL` writer - Injected sequence length
pub type JL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, JSQR_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:4 - 1st conversion in injected sequence
    #[inline(always)]
    pub fn jsq1(&self) -> JSQ1_R {
        JSQ1_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9 - 2nd conversion in injected sequence
    #[inline(always)]
    pub fn jsq2(&self) -> JSQ2_R {
        JSQ2_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14 - 3rd conversion in injected sequence
    #[inline(always)]
    pub fn jsq3(&self) -> JSQ3_R {
        JSQ3_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bits 15:19 - 4th conversion in injected sequence
    #[inline(always)]
    pub fn jsq4(&self) -> JSQ4_R {
        JSQ4_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bits 20:21 - Injected sequence length
    #[inline(always)]
    pub fn jl(&self) -> JL_R {
        JL_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    ///Bits 0:4 - 1st conversion in injected sequence
    #[inline(always)]
    #[must_use]
    pub fn jsq1(&mut self) -> JSQ1_W<0> {
        JSQ1_W::new(self)
    }
    ///Bits 5:9 - 2nd conversion in injected sequence
    #[inline(always)]
    #[must_use]
    pub fn jsq2(&mut self) -> JSQ2_W<5> {
        JSQ2_W::new(self)
    }
    ///Bits 10:14 - 3rd conversion in injected sequence
    #[inline(always)]
    #[must_use]
    pub fn jsq3(&mut self) -> JSQ3_W<10> {
        JSQ3_W::new(self)
    }
    ///Bits 15:19 - 4th conversion in injected sequence
    #[inline(always)]
    #[must_use]
    pub fn jsq4(&mut self) -> JSQ4_W<15> {
        JSQ4_W::new(self)
    }
    ///Bits 20:21 - Injected sequence length
    #[inline(always)]
    #[must_use]
    pub fn jl(&mut self) -> JL_W<20> {
        JL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///injected sequence register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [jsqr](index.html) module
pub struct JSQR_SPEC;
impl crate::RegisterSpec for JSQR_SPEC {
    type Ux = u32;
}
///`read()` method returns [jsqr::R](R) reader structure
impl crate::Readable for JSQR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [jsqr::W](W) writer structure
impl crate::Writable for JSQR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets JSQR to value 0
impl crate::Resettable for JSQR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
