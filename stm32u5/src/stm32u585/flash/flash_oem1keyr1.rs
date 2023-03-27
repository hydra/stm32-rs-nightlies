///Register `FLASH_OEM1KEYR1` writer
pub struct W(crate::W<FLASH_OEM1KEYR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_OEM1KEYR1_SPEC>;
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
impl From<crate::W<FLASH_OEM1KEYR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_OEM1KEYR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OEM1KEY` writer - OEM1 least significant bytes key
pub type OEM1KEY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_OEM1KEYR1_SPEC, u32, u32, 32, O>;
impl W {
    ///Bits 0:31 - OEM1 least significant bytes key
    #[inline(always)]
    #[must_use]
    pub fn oem1key(&mut self) -> OEM1KEY_W<0> {
        OEM1KEY_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH OEM1 key register 1
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flash_oem1keyr1](index.html) module
pub struct FLASH_OEM1KEYR1_SPEC;
impl crate::RegisterSpec for FLASH_OEM1KEYR1_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [flash_oem1keyr1::W](W) writer structure
impl crate::Writable for FLASH_OEM1KEYR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLASH_OEM1KEYR1 to value 0
impl crate::Resettable for FLASH_OEM1KEYR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
