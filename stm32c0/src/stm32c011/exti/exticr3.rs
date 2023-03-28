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
///Field `EXTI0` reader - EXTIm GPIO port selection (m = 4 * (x - 1)) These bits are written by software to select the source input for EXTIm external interrupt. Other: reserved
pub type EXTI0_R = crate::FieldReader<u8, u8>;
///Field `EXTI0` writer - EXTIm GPIO port selection (m = 4 * (x - 1)) These bits are written by software to select the source input for EXTIm external interrupt. Other: reserved
pub type EXTI0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR3_SPEC, u8, u8, 8, O>;
///Field `EXTI1` reader -
pub type EXTI1_R = crate::FieldReader<u8, u8>;
///Field `EXTI1` writer -
pub type EXTI1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR3_SPEC, u8, u8, 8, O>;
///Field `EXTI2` reader -
pub type EXTI2_R = crate::FieldReader<u8, u8>;
///Field `EXTI2` writer -
pub type EXTI2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR3_SPEC, u8, u8, 8, O>;
///Field `EXTI3` reader -
pub type EXTI3_R = crate::FieldReader<u8, u8>;
///Field `EXTI3` writer -
pub type EXTI3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR3_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - EXTIm GPIO port selection (m = 4 * (x - 1)) These bits are written by software to select the source input for EXTIm external interrupt. Other: reserved
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23
    #[inline(always)]
    pub fn exti2(&self) -> EXTI2_R {
        EXTI2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn exti3(&self) -> EXTI3_R {
        EXTI3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - EXTIm GPIO port selection (m = 4 * (x - 1)) These bits are written by software to select the source input for EXTIm external interrupt. Other: reserved
    #[inline(always)]
    #[must_use]
    pub fn exti0(&mut self) -> EXTI0_W<0> {
        EXTI0_W::new(self)
    }
    ///Bits 8:15
    #[inline(always)]
    #[must_use]
    pub fn exti1(&mut self) -> EXTI1_W<8> {
        EXTI1_W::new(self)
    }
    ///Bits 16:23
    #[inline(always)]
    #[must_use]
    pub fn exti2(&mut self) -> EXTI2_W<16> {
        EXTI2_W::new(self)
    }
    ///Bits 24:31
    #[inline(always)]
    #[must_use]
    pub fn exti3(&mut self) -> EXTI3_W<24> {
        EXTI3_W::new(self)
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
