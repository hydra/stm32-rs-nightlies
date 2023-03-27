///Register `EXTICR3` reader
pub struct R(crate::R<EXTICR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EXTICR3` writer
pub struct W(crate::W<EXTICR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR3_SPEC>;
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
impl From<crate::W<EXTICR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EXTI8` reader - EXTI8 configuration
pub type EXTI8_R = crate::FieldReader<u8, u8>;
///Field `EXTI8` writer - EXTI8 configuration
pub type EXTI8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR3_SPEC, u8, u8, 4, O>;
///Field `EXTI9` reader - EXTI9 configuration
pub type EXTI9_R = crate::FieldReader<u8, u8>;
///Field `EXTI9` writer - EXTI9 configuration
pub type EXTI9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR3_SPEC, u8, u8, 4, O>;
///Field `EXTI10` reader - EXTI10 configuration
pub type EXTI10_R = crate::FieldReader<u8, u8>;
///Field `EXTI10` writer - EXTI10 configuration
pub type EXTI10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR3_SPEC, u8, u8, 4, O>;
///Field `EXTI11` reader - EXTI11 configuration
pub type EXTI11_R = crate::FieldReader<u8, u8>;
///Field `EXTI11` writer - EXTI11 configuration
pub type EXTI11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR3_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:3 - EXTI8 configuration
    #[inline(always)]
    pub fn exti8(&self) -> EXTI8_R {
        EXTI8_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - EXTI9 configuration
    #[inline(always)]
    pub fn exti9(&self) -> EXTI9_R {
        EXTI9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - EXTI10 configuration
    #[inline(always)]
    pub fn exti10(&self) -> EXTI10_R {
        EXTI10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - EXTI11 configuration
    #[inline(always)]
    pub fn exti11(&self) -> EXTI11_R {
        EXTI11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - EXTI8 configuration
    #[inline(always)]
    #[must_use]
    pub fn exti8(&mut self) -> EXTI8_W<0> {
        EXTI8_W::new(self)
    }
    ///Bits 4:7 - EXTI9 configuration
    #[inline(always)]
    #[must_use]
    pub fn exti9(&mut self) -> EXTI9_W<4> {
        EXTI9_W::new(self)
    }
    ///Bits 8:11 - EXTI10 configuration
    #[inline(always)]
    #[must_use]
    pub fn exti10(&mut self) -> EXTI10_W<8> {
        EXTI10_W::new(self)
    }
    ///Bits 12:15 - EXTI11 configuration
    #[inline(always)]
    #[must_use]
    pub fn exti11(&mut self) -> EXTI11_W<12> {
        EXTI11_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///External interrupt configuration register 3 (AFIO_EXTICR3)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exticr3](index.html) module
pub struct EXTICR3_SPEC;
impl crate::RegisterSpec for EXTICR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [exticr3::R](R) reader structure
impl crate::Readable for EXTICR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [exticr3::W](W) writer structure
impl crate::Writable for EXTICR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EXTICR3 to value 0
impl crate::Resettable for EXTICR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
