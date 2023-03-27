///Register `FLASH_PDKEY1R` writer
pub struct W(crate::W<FLASH_PDKEY1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_PDKEY1R_SPEC>;
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
impl From<crate::W<FLASH_PDKEY1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_PDKEY1R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PDKEY1` writer - Bank 1 power-down key
pub type PDKEY1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_PDKEY1R_SPEC, u32, u32, 32, O>;
impl W {
    ///Bits 0:31 - Bank 1 power-down key
    #[inline(always)]
    #[must_use]
    pub fn pdkey1(&mut self) -> PDKEY1_W<0> {
        PDKEY1_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH bank 1 power-down key register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flash_pdkey1r](index.html) module
pub struct FLASH_PDKEY1R_SPEC;
impl crate::RegisterSpec for FLASH_PDKEY1R_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [flash_pdkey1r::W](W) writer structure
impl crate::Writable for FLASH_PDKEY1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLASH_PDKEY1R to value 0
impl crate::Resettable for FLASH_PDKEY1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
