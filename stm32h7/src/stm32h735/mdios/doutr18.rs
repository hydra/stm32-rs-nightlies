///Register `DOUTR18` reader
pub struct R(crate::R<DOUTR18_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUTR18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUTR18_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUTR18_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DOUTR18` writer
pub struct W(crate::W<DOUTR18_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUTR18_SPEC>;
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
impl From<crate::W<DOUTR18_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUTR18_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DOUT18` reader - Output data sent to MDIO Master during read frames
pub type DOUT18_R = crate::FieldReader<u16, u16>;
///Field `DOUT18` writer - Output data sent to MDIO Master during read frames
pub type DOUT18_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUTR18_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout18(&self) -> DOUT18_R {
        DOUT18_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    #[must_use]
    pub fn dout18(&mut self) -> DOUT18_W<0> {
        DOUT18_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MDIOS output data register 18
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doutr18](index.html) module
pub struct DOUTR18_SPEC;
impl crate::RegisterSpec for DOUTR18_SPEC {
    type Ux = u32;
}
///`read()` method returns [doutr18::R](R) reader structure
impl crate::Readable for DOUTR18_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [doutr18::W](W) writer structure
impl crate::Writable for DOUTR18_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DOUTR18 to value 0
impl crate::Resettable for DOUTR18_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
