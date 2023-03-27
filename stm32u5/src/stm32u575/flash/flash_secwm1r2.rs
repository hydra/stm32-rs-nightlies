///Register `FLASH_SECWM1R2` reader
pub struct R(crate::R<FLASH_SECWM1R2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_SECWM1R2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_SECWM1R2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_SECWM1R2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLASH_SECWM1R2` writer
pub struct W(crate::W<FLASH_SECWM1R2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_SECWM1R2_SPEC>;
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
impl From<crate::W<FLASH_SECWM1R2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_SECWM1R2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HDP1_PEND` reader - End page of first hide protection area This field contains the last page of the HDP area in bank 1.
pub type HDP1_PEND_R = crate::FieldReader<u8, u8>;
///Field `HDP1_PEND` writer - End page of first hide protection area This field contains the last page of the HDP area in bank 1.
pub type HDP1_PEND_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_SECWM1R2_SPEC, u8, u8, 7, O>;
///Field `HDP1EN` reader - Hide protection first area enable
pub type HDP1EN_R = crate::BitReader<bool>;
///Field `HDP1EN` writer - Hide protection first area enable
pub type HDP1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SECWM1R2_SPEC, bool, O>;
impl R {
    ///Bits 16:22 - End page of first hide protection area This field contains the last page of the HDP area in bank 1.
    #[inline(always)]
    pub fn hdp1_pend(&self) -> HDP1_PEND_R {
        HDP1_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 31 - Hide protection first area enable
    #[inline(always)]
    pub fn hdp1en(&self) -> HDP1EN_R {
        HDP1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 16:22 - End page of first hide protection area This field contains the last page of the HDP area in bank 1.
    #[inline(always)]
    #[must_use]
    pub fn hdp1_pend(&mut self) -> HDP1_PEND_W<16> {
        HDP1_PEND_W::new(self)
    }
    ///Bit 31 - Hide protection first area enable
    #[inline(always)]
    #[must_use]
    pub fn hdp1en(&mut self) -> HDP1EN_W<31> {
        HDP1EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH secure watermark1 register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flash_secwm1r2](index.html) module
pub struct FLASH_SECWM1R2_SPEC;
impl crate::RegisterSpec for FLASH_SECWM1R2_SPEC {
    type Ux = u32;
}
///`read()` method returns [flash_secwm1r2::R](R) reader structure
impl crate::Readable for FLASH_SECWM1R2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [flash_secwm1r2::W](W) writer structure
impl crate::Writable for FLASH_SECWM1R2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLASH_SECWM1R2 to value 0x0f00_ffff
impl crate::Resettable for FLASH_SECWM1R2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f00_ffff;
}
