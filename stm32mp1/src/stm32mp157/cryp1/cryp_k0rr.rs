///Register `CRYP_K0RR` writer
pub struct W(crate::W<CRYP_K0RR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYP_K0RR_SPEC>;
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
impl From<crate::W<CRYP_K0RR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYP_K0RR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `K` writer - K
pub type K_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRYP_K0RR_SPEC, u32, u32, 32, O>;
impl W {
    ///Bits 0:31 - K
    #[inline(always)]
    #[must_use]
    pub fn k(&mut self) -> K_W<0> {
        K_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Refer to Section39.6.9: CRYP key register 0L (CRYP_K0LR) for details.
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cryp_k0rr](index.html) module
pub struct CRYP_K0RR_SPEC;
impl crate::RegisterSpec for CRYP_K0RR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [cryp_k0rr::W](W) writer structure
impl crate::Writable for CRYP_K0RR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CRYP_K0RR to value 0
impl crate::Resettable for CRYP_K0RR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
