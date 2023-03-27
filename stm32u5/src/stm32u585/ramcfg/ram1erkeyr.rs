///Register `RAM1ERKEYR` writer
pub struct W(crate::W<RAM1ERKEYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM1ERKEYR_SPEC>;
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
impl From<crate::W<RAM1ERKEYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM1ERKEYR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ERASEKEY` writer - ERASEKEY
pub type ERASEKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RAM1ERKEYR_SPEC, u8, u8, 8, O>;
impl W {
    ///Bits 0:7 - ERASEKEY
    #[inline(always)]
    #[must_use]
    pub fn erasekey(&mut self) -> ERASEKEY_W<0> {
        ERASEKEY_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RAMCFG SRAM x erase key register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ram1erkeyr](index.html) module
pub struct RAM1ERKEYR_SPEC;
impl crate::RegisterSpec for RAM1ERKEYR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [ram1erkeyr::W](W) writer structure
impl crate::Writable for RAM1ERKEYR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RAM1ERKEYR to value 0
impl crate::Resettable for RAM1ERKEYR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
