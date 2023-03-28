///Register `SECOBKKEYR` writer
pub struct W(crate::W<SECOBKKEYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECOBKKEYR_SPEC>;
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
impl From<crate::W<SECOBKKEYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECOBKKEYR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SECOBKKEY` writer - FLASH secure option bytes keys control access unlock key
pub type SECOBKKEY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SECOBKKEYR_SPEC, u32, u32, 32, O>;
impl W {
    ///Bits 0:31 - FLASH secure option bytes keys control access unlock key
    #[inline(always)]
    #[must_use]
    pub fn secobkkey(&mut self) -> SECOBKKEY_W<0> {
        SECOBKKEY_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH secure OBK key register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [secobkkeyr](index.html) module
pub struct SECOBKKEYR_SPEC;
impl crate::RegisterSpec for SECOBKKEYR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [secobkkeyr::W](W) writer structure
impl crate::Writable for SECOBKKEYR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SECOBKKEYR to value 0
impl crate::Resettable for SECOBKKEYR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
