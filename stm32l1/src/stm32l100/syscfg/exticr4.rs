///Register `EXTICR4` reader
pub struct R(crate::R<EXTICR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EXTICR4` writer
pub struct W(crate::W<EXTICR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR4_SPEC>;
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
impl From<crate::W<EXTICR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR4_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EXTI12` reader - EXTI12
pub type EXTI12_R = crate::FieldReader<u8, u8>;
///Field `EXTI12` writer - EXTI12
pub type EXTI12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, u8, 4, O>;
///Field `EXTI13` reader - EXTI13
pub type EXTI13_R = crate::FieldReader<u8, u8>;
///Field `EXTI13` writer - EXTI13
pub type EXTI13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, u8, 4, O>;
///Field `EXTI14` reader - EXTI14
pub type EXTI14_R = crate::FieldReader<u8, u8>;
///Field `EXTI14` writer - EXTI14
pub type EXTI14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, u8, 4, O>;
///Field `EXTI15` reader - EXTI x configuration (x = 12 to 15)
pub type EXTI15_R = crate::FieldReader<u8, u8>;
///Field `EXTI15` writer - EXTI x configuration (x = 12 to 15)
pub type EXTI15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:3 - EXTI12
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - EXTI13
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - EXTI14
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - EXTI x configuration (x = 12 to 15)
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - EXTI12
    #[inline(always)]
    #[must_use]
    pub fn exti12(&mut self) -> EXTI12_W<0> {
        EXTI12_W::new(self)
    }
    ///Bits 4:7 - EXTI13
    #[inline(always)]
    #[must_use]
    pub fn exti13(&mut self) -> EXTI13_W<4> {
        EXTI13_W::new(self)
    }
    ///Bits 8:11 - EXTI14
    #[inline(always)]
    #[must_use]
    pub fn exti14(&mut self) -> EXTI14_W<8> {
        EXTI14_W::new(self)
    }
    ///Bits 12:15 - EXTI x configuration (x = 12 to 15)
    #[inline(always)]
    #[must_use]
    pub fn exti15(&mut self) -> EXTI15_W<12> {
        EXTI15_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///external interrupt configuration register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exticr4](index.html) module
pub struct EXTICR4_SPEC;
impl crate::RegisterSpec for EXTICR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [exticr4::R](R) reader structure
impl crate::Readable for EXTICR4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [exticr4::W](W) writer structure
impl crate::Writable for EXTICR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EXTICR4 to value 0
impl crate::Resettable for EXTICR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
