///Register `DOUTR14` reader
pub struct R(crate::R<DOUTR14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUTR14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUTR14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUTR14_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DOUTR14` writer
pub struct W(crate::W<DOUTR14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUTR14_SPEC>;
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
impl From<crate::W<DOUTR14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUTR14_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DOUT14` reader - Output data sent to MDIO Master during read frames
pub type DOUT14_R = crate::FieldReader<u16, u16>;
///Field `DOUT14` writer - Output data sent to MDIO Master during read frames
pub type DOUT14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUTR14_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout14(&self) -> DOUT14_R {
        DOUT14_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    #[must_use]
    pub fn dout14(&mut self) -> DOUT14_W<0> {
        DOUT14_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MDIOS output data register 14
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doutr14](index.html) module
pub struct DOUTR14_SPEC;
impl crate::RegisterSpec for DOUTR14_SPEC {
    type Ux = u32;
}
///`read()` method returns [doutr14::R](R) reader structure
impl crate::Readable for DOUTR14_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [doutr14::W](W) writer structure
impl crate::Writable for DOUTR14_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DOUTR14 to value 0
impl crate::Resettable for DOUTR14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
