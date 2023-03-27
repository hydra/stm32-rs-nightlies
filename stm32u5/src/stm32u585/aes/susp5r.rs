///Register `SUSP5R` writer
pub struct W(crate::W<SUSP5R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUSP5R_SPEC>;
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
impl From<crate::W<SUSP5R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUSP5R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SUSP5` writer - AES suspend
pub type SUSP5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SUSP5R_SPEC, u32, u32, 32, O>;
impl W {
    ///Bits 0:31 - AES suspend
    #[inline(always)]
    #[must_use]
    pub fn susp5(&mut self) -> SUSP5_W<0> {
        SUSP5_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///suspend registers
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [susp5r](index.html) module
pub struct SUSP5R_SPEC;
impl crate::RegisterSpec for SUSP5R_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [susp5r::W](W) writer structure
impl crate::Writable for SUSP5R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SUSP5R to value 0
impl crate::Resettable for SUSP5R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
