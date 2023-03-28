///Register `DOUTR0` reader
pub struct R(crate::R<DOUTR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUTR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUTR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUTR0_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DOUTR0` writer
pub struct W(crate::W<DOUTR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUTR0_SPEC>;
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
impl From<crate::W<DOUTR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUTR0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DOUT0` reader - Output data sent to MDIO Master during read frames
pub type DOUT0_R = crate::FieldReader<u16, u16>;
///Field `DOUT0` writer - Output data sent to MDIO Master during read frames
pub type DOUT0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOUTR0_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    pub fn dout0(&self) -> DOUT0_R {
        DOUT0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Output data sent to MDIO Master during read frames
    #[inline(always)]
    #[must_use]
    pub fn dout0(&mut self) -> DOUT0_W<0> {
        DOUT0_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MDIOS output data register 0
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doutr0](index.html) module
pub struct DOUTR0_SPEC;
impl crate::RegisterSpec for DOUTR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [doutr0::R](R) reader structure
impl crate::Readable for DOUTR0_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [doutr0::W](W) writer structure
impl crate::Writable for DOUTR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DOUTR0 to value 0
impl crate::Resettable for DOUTR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
