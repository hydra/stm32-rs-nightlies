///Register `TISEL` reader
pub struct R(crate::R<TISEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TISEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TISEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TISEL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TISEL` writer
pub struct W(crate::W<TISEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TISEL_SPEC>;
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
impl From<crate::W<TISEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TISEL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TI1SEL` reader - selects TI1\[0\]
///to TI1\[15\]
///input
pub type TI1SEL_R = crate::FieldReader<u8, u8>;
///Field `TI1SEL` writer - selects TI1\[0\]
///to TI1\[15\]
///input
pub type TI1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TISEL_SPEC, u8, u8, 4, O>;
///Field `TI2SEL` reader - selects TI2\[0\]
///to TI2\[15\]
///input
pub type TI2SEL_R = crate::FieldReader<u8, u8>;
///Field `TI2SEL` writer - selects TI2\[0\]
///to TI2\[15\]
///input
pub type TI2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TISEL_SPEC, u8, u8, 4, O>;
///Field `TI3SEL` reader - selects TI3\[0\]
///to TI3\[15\]
///input
pub type TI3SEL_R = crate::FieldReader<u8, u8>;
///Field `TI3SEL` writer - selects TI3\[0\]
///to TI3\[15\]
///input
pub type TI3SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TISEL_SPEC, u8, u8, 4, O>;
///Field `TI4SEL` reader - selects TI4\[0\]
///to TI4\[15\]
///input
pub type TI4SEL_R = crate::FieldReader<u8, u8>;
///Field `TI4SEL` writer - selects TI4\[0\]
///to TI4\[15\]
///input
pub type TI4SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TISEL_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:3 - selects TI1\[0\]
    ///to TI1\[15\]
    ///input
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - selects TI2\[0\]
    ///to TI2\[15\]
    ///input
    #[inline(always)]
    pub fn ti2sel(&self) -> TI2SEL_R {
        TI2SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:19 - selects TI3\[0\]
    ///to TI3\[15\]
    ///input
    #[inline(always)]
    pub fn ti3sel(&self) -> TI3SEL_R {
        TI3SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 24:27 - selects TI4\[0\]
    ///to TI4\[15\]
    ///input
    #[inline(always)]
    pub fn ti4sel(&self) -> TI4SEL_R {
        TI4SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - selects TI1\[0\]
    ///to TI1\[15\]
    ///input
    #[inline(always)]
    #[must_use]
    pub fn ti1sel(&mut self) -> TI1SEL_W<0> {
        TI1SEL_W::new(self)
    }
    ///Bits 8:11 - selects TI2\[0\]
    ///to TI2\[15\]
    ///input
    #[inline(always)]
    #[must_use]
    pub fn ti2sel(&mut self) -> TI2SEL_W<8> {
        TI2SEL_W::new(self)
    }
    ///Bits 16:19 - selects TI3\[0\]
    ///to TI3\[15\]
    ///input
    #[inline(always)]
    #[must_use]
    pub fn ti3sel(&mut self) -> TI3SEL_W<16> {
        TI3SEL_W::new(self)
    }
    ///Bits 24:27 - selects TI4\[0\]
    ///to TI4\[15\]
    ///input
    #[inline(always)]
    #[must_use]
    pub fn ti4sel(&mut self) -> TI4SEL_W<24> {
        TI4SEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM1 timer input selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tisel](index.html) module
pub struct TISEL_SPEC;
impl crate::RegisterSpec for TISEL_SPEC {
    type Ux = u32;
}
///`read()` method returns [tisel::R](R) reader structure
impl crate::Readable for TISEL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tisel::W](W) writer structure
impl crate::Writable for TISEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TISEL to value 0
impl crate::Resettable for TISEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
