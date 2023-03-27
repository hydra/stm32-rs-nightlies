///Register `DIN` writer
pub struct W(crate::W<DIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIN_SPEC>;
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
impl From<crate::W<DIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIN_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DATAIN` writer - Data input
pub type DATAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIN_SPEC, u32, u32, 32, O>;
impl W {
    ///Bits 0:31 - Data input
    #[inline(always)]
    #[must_use]
    pub fn datain(&mut self) -> DATAIN_W<0> {
        DATAIN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///data input register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [din](index.html) module
pub struct DIN_SPEC;
impl crate::RegisterSpec for DIN_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [din::W](W) writer structure
impl crate::Writable for DIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DIN to value 0
impl crate::Resettable for DIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
