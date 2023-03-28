///Register `DOUTR8` reader
pub struct R(crate::R<DOUTR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUTR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUTR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUTR8_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DOUTR8` writer
pub struct W(crate::W<DOUTR8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUTR8_SPEC>;
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
impl From<crate::W<DOUTR8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUTR8_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DOUT8` reader - Output data sent to MDIO Master during read frames
pub type DOUT8_R = crate::FieldReader<u16, u16>;
///Field `DOUT8` writer - Output data sent to MDIO Master during read frames
pub type DOUT8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUTR8_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout8(&self) -> DOUT8_R {
        DOUT8_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    #[must_use]
    pub fn dout8(&mut self) -> DOUT8_W<0> {
        DOUT8_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MDIOS output data register 8
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doutr8](index.html) module
pub struct DOUTR8_SPEC;
impl crate::RegisterSpec for DOUTR8_SPEC {
    type Ux = u32;
}
///`read()` method returns [doutr8::R](R) reader structure
impl crate::Readable for DOUTR8_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [doutr8::W](W) writer structure
impl crate::Writable for DOUTR8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DOUTR8 to value 0
impl crate::Resettable for DOUTR8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
