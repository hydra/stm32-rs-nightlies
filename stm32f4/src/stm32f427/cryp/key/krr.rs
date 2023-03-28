///Register `KRR` writer
pub struct W(crate::W<KRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KRR_SPEC>;
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
impl From<crate::W<KRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KRR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `b` writer - b192
pub type B_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KRR_SPEC, u32, u32, 32, O>;
impl W {
    ///Bits 0:31 - b192
    #[inline(always)]
    #[must_use]
    pub fn b(&mut self) -> B_W<0> {
        B_W::new(self)
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
///For information about available fields see [krr](index.html) module
pub struct KRR_SPEC;
impl crate::RegisterSpec for KRR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [krr::W](W) writer structure
impl crate::Writable for KRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets KRR to value 0
impl crate::Resettable for KRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
