///Register `EXTICR2` reader
pub struct R(crate::R<EXTICR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EXTICR2` writer
pub struct W(crate::W<EXTICR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR2_SPEC>;
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
impl From<crate::W<EXTICR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EXTI0_7` reader - EXTIm GPIO port selection
pub type EXTI0_7_R = crate::FieldReader<u8, u8>;
///Field `EXTI0_7` writer - EXTIm GPIO port selection
pub type EXTI0_7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR2_SPEC, u8, u8, 8, O>;
///Field `EXTI8_15` reader - EXTIm+1 GPIO port selection
pub type EXTI8_15_R = crate::FieldReader<u8, u8>;
///Field `EXTI8_15` writer - EXTIm+1 GPIO port selection
pub type EXTI8_15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR2_SPEC, u8, u8, 8, O>;
///Field `EXTI16_23` reader - EXTIm+2 GPIO port selection
pub type EXTI16_23_R = crate::FieldReader<u8, u8>;
///Field `EXTI16_23` writer - EXTIm+2 GPIO port selection
pub type EXTI16_23_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR2_SPEC, u8, u8, 8, O>;
///Field `EXTI24_31` reader - EXTIm+3 GPIO port selection
pub type EXTI24_31_R = crate::FieldReader<u8, u8>;
///Field `EXTI24_31` writer - EXTIm+3 GPIO port selection
pub type EXTI24_31_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR2_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - EXTIm GPIO port selection
    #[inline(always)]
    pub fn exti0_7(&self) -> EXTI0_7_R {
        EXTI0_7_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - EXTIm+1 GPIO port selection
    #[inline(always)]
    pub fn exti8_15(&self) -> EXTI8_15_R {
        EXTI8_15_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - EXTIm+2 GPIO port selection
    #[inline(always)]
    pub fn exti16_23(&self) -> EXTI16_23_R {
        EXTI16_23_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - EXTIm+3 GPIO port selection
    #[inline(always)]
    pub fn exti24_31(&self) -> EXTI24_31_R {
        EXTI24_31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - EXTIm GPIO port selection
    #[inline(always)]
    #[must_use]
    pub fn exti0_7(&mut self) -> EXTI0_7_W<0> {
        EXTI0_7_W::new(self)
    }
    ///Bits 8:15 - EXTIm+1 GPIO port selection
    #[inline(always)]
    #[must_use]
    pub fn exti8_15(&mut self) -> EXTI8_15_W<8> {
        EXTI8_15_W::new(self)
    }
    ///Bits 16:23 - EXTIm+2 GPIO port selection
    #[inline(always)]
    #[must_use]
    pub fn exti16_23(&mut self) -> EXTI16_23_W<16> {
        EXTI16_23_W::new(self)
    }
    ///Bits 24:31 - EXTIm+3 GPIO port selection
    #[inline(always)]
    #[must_use]
    pub fn exti24_31(&mut self) -> EXTI24_31_W<24> {
        EXTI24_31_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI external interrupt selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exticr2](index.html) module
pub struct EXTICR2_SPEC;
impl crate::RegisterSpec for EXTICR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [exticr2::R](R) reader structure
impl crate::Readable for EXTICR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [exticr2::W](W) writer structure
impl crate::Writable for EXTICR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EXTICR2 to value 0
impl crate::Resettable for EXTICR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
