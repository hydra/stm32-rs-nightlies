///Register `OPTKEYR` writer
pub struct W(crate::W<OPTKEYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTKEYR_SPEC>;
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
impl From<crate::W<OPTKEYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTKEYR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OPTKEY` writer - Option byte key
pub type OPTKEY_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OPTKEYR_SPEC, u32, u32, 32, O>;
impl W {
    ///Bits 0:31 - Option byte key
    #[inline(always)]
    #[must_use]
    pub fn optkey(&mut self) -> OPTKEY_W<0> {
        OPTKEY_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
///Option byte key register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [optkeyr](index.html) module
pub struct OPTKEYR_SPEC;
impl crate::RegisterSpec for OPTKEYR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [optkeyr::W](W) writer structure
impl crate::Writable for OPTKEYR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OPTKEYR to value 0
impl crate::Resettable for OPTKEYR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
