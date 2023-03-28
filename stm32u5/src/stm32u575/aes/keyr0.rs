///Register `KEYR0` writer
pub struct W(crate::W<KEYR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYR0_SPEC>;
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
impl From<crate::W<KEYR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYR0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `KEY` writer - Cryptographic key, bits \[31:0\]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYR0_SPEC, u32, u32, 32, O>;
impl W {
    ///Bits 0:31 - Cryptographic key, bits \[31:0\]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<0> {
        KEY_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///key register 0
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [keyr0](index.html) module
pub struct KEYR0_SPEC;
impl crate::RegisterSpec for KEYR0_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [keyr0::W](W) writer structure
impl crate::Writable for KEYR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets KEYR0 to value 0
impl crate::Resettable for KEYR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
