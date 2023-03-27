///Register `WDATA` writer
pub struct W(crate::W<WDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDATA_SPEC>;
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
impl From<crate::W<WDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDATA_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ARG` writer - Function input arguments
pub type ARG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDATA_SPEC, u32, u32, 32, O>;
impl W {
    ///Bits 0:31 - Function input arguments
    #[inline(always)]
    #[must_use]
    pub fn arg(&mut self) -> ARG_W<0> {
        ARG_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FMAC Write Data register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wdata](index.html) module
pub struct WDATA_SPEC;
impl crate::RegisterSpec for WDATA_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [wdata::W](W) writer structure
impl crate::Writable for WDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets WDATA to value 0
impl crate::Resettable for WDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
