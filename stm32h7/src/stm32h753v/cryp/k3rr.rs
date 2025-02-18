///Register `K3RR` writer
pub struct W(crate::W<K3RR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<K3RR_SPEC>;
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
impl From<crate::W<K3RR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<K3RR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `K` writer - K0
pub type K_W<'a, const O: u8> = crate::FieldWriter<'a, u32, K3RR_SPEC, u32, u32, 32, O>;
impl W {
    ///Bits 0:31 - K0
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
///key registers
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [k3rr](index.html) module
pub struct K3RR_SPEC;
impl crate::RegisterSpec for K3RR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [k3rr::W](W) writer structure
impl crate::Writable for K3RR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets K3RR to value 0
impl crate::Resettable for K3RR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
