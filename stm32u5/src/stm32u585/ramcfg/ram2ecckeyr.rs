///Register `RAM2ECCKEYR` writer
pub struct W(crate::W<RAM2ECCKEYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM2ECCKEYR_SPEC>;
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
impl From<crate::W<RAM2ECCKEYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM2ECCKEYR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ECCKEY` writer - ECCKEY
pub type ECCKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RAM2ECCKEYR_SPEC, u8, u8, 8, O>;
impl W {
    ///Bits 0:7 - ECCKEY
    #[inline(always)]
    #[must_use]
    pub fn ecckey(&mut self) -> ECCKEY_W<0> {
        ECCKEY_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RAMCFG SRAM x ECC key register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ram2ecckeyr](index.html) module
pub struct RAM2ECCKEYR_SPEC;
impl crate::RegisterSpec for RAM2ECCKEYR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [ram2ecckeyr::W](W) writer structure
impl crate::Writable for RAM2ECCKEYR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RAM2ECCKEYR to value 0
impl crate::Resettable for RAM2ECCKEYR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
