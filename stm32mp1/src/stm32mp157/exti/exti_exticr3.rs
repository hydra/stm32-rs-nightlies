///Register `EXTI_EXTICR3` reader
pub struct R(crate::R<EXTI_EXTICR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_EXTICR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_EXTICR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_EXTICR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EXTI_EXTICR3` writer
pub struct W(crate::W<EXTI_EXTICR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_EXTICR3_SPEC>;
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
impl From<crate::W<EXTI_EXTICR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_EXTICR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EXTI8` reader - EXTI8
pub type EXTI8_R = crate::FieldReader<u8, u8>;
///Field `EXTI8` writer - EXTI8
pub type EXTI8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTI_EXTICR3_SPEC, u8, u8, 8, O>;
///Field `EXTI9` reader - EXTI9
pub type EXTI9_R = crate::FieldReader<u8, u8>;
///Field `EXTI9` writer - EXTI9
pub type EXTI9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTI_EXTICR3_SPEC, u8, u8, 8, O>;
///Field `EXTI10` reader - EXTI10
pub type EXTI10_R = crate::FieldReader<u8, u8>;
///Field `EXTI10` writer - EXTI10
pub type EXTI10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTI_EXTICR3_SPEC, u8, u8, 8, O>;
///Field `EXTI11` reader - EXTI11
pub type EXTI11_R = crate::FieldReader<u8, u8>;
///Field `EXTI11` writer - EXTI11
pub type EXTI11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTI_EXTICR3_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - EXTI8
    #[inline(always)]
    pub fn exti8(&self) -> EXTI8_R {
        EXTI8_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - EXTI9
    #[inline(always)]
    pub fn exti9(&self) -> EXTI9_R {
        EXTI9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - EXTI10
    #[inline(always)]
    pub fn exti10(&self) -> EXTI10_R {
        EXTI10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - EXTI11
    #[inline(always)]
    pub fn exti11(&self) -> EXTI11_R {
        EXTI11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - EXTI8
    #[inline(always)]
    #[must_use]
    pub fn exti8(&mut self) -> EXTI8_W<0> {
        EXTI8_W::new(self)
    }
    ///Bits 8:15 - EXTI9
    #[inline(always)]
    #[must_use]
    pub fn exti9(&mut self) -> EXTI9_W<8> {
        EXTI9_W::new(self)
    }
    ///Bits 16:23 - EXTI10
    #[inline(always)]
    #[must_use]
    pub fn exti10(&mut self) -> EXTI10_W<16> {
        EXTI10_W::new(self)
    }
    ///Bits 24:31 - EXTI11
    #[inline(always)]
    #[must_use]
    pub fn exti11(&mut self) -> EXTI11_W<24> {
        EXTI11_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTIm fields contain only the number of bits in line with the nb_ioport configuration.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [exti_exticr3](index.html) module
pub struct EXTI_EXTICR3_SPEC;
impl crate::RegisterSpec for EXTI_EXTICR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [exti_exticr3::R](R) reader structure
impl crate::Readable for EXTI_EXTICR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [exti_exticr3::W](W) writer structure
impl crate::Writable for EXTI_EXTICR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EXTI_EXTICR3 to value 0
impl crate::Resettable for EXTI_EXTICR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
