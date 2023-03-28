///Register `DOUTR10` reader
pub struct R(crate::R<DOUTR10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUTR10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUTR10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUTR10_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DOUTR10` writer
pub struct W(crate::W<DOUTR10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUTR10_SPEC>;
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
impl From<crate::W<DOUTR10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUTR10_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DOUT10` reader - Output data sent to MDIO Master during read frames
pub type DOUT10_R = crate::FieldReader<u16, u16>;
///Field `DOUT10` writer - Output data sent to MDIO Master during read frames
pub type DOUT10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUTR10_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout10(&self) -> DOUT10_R {
        DOUT10_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    #[must_use]
    pub fn dout10(&mut self) -> DOUT10_W<0> {
        DOUT10_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MDIOS output data register 10
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doutr10](index.html) module
pub struct DOUTR10_SPEC;
impl crate::RegisterSpec for DOUTR10_SPEC {
    type Ux = u32;
}
///`read()` method returns [doutr10::R](R) reader structure
impl crate::Readable for DOUTR10_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [doutr10::W](W) writer structure
impl crate::Writable for DOUTR10_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DOUTR10 to value 0
impl crate::Resettable for DOUTR10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
