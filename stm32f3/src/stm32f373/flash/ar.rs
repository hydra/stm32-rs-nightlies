///Register `AR` writer
pub struct W(crate::W<AR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AR_SPEC>;
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
impl From<crate::W<AR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FAR` writer - Flash address
pub type FAR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, AR_SPEC, u32, u32, 32, O>;
impl W {
    ///Bits 0:31 - Flash address
    #[inline(always)]
    #[must_use]
    pub fn far(&mut self) -> FAR_W<0> {
        FAR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
///Flash address register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ar](index.html) module
pub struct AR_SPEC;
impl crate::RegisterSpec for AR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [ar::W](W) writer structure
impl crate::Writable for AR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AR to value 0
impl crate::Resettable for AR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
