///Register `FLASH_SECWM2R2` reader
pub struct R(crate::R<FLASH_SECWM2R2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_SECWM2R2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_SECWM2R2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_SECWM2R2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLASH_SECWM2R2` writer
pub struct W(crate::W<FLASH_SECWM2R2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_SECWM2R2_SPEC>;
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
impl From<crate::W<FLASH_SECWM2R2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_SECWM2R2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HDP2_PEND` reader - End page of hide protection second area HDP2_PEND contains the last page of the HDP area in bank 2.
pub type HDP2_PEND_R = crate::FieldReader<u8, u8>;
///Field `HDP2_PEND` writer - End page of hide protection second area HDP2_PEND contains the last page of the HDP area in bank 2.
pub type HDP2_PEND_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_SECWM2R2_SPEC, u8, u8, 7, O>;
///Field `HDP2EN` reader - Hide protection second area enable
pub type HDP2EN_R = crate::BitReader<bool>;
///Field `HDP2EN` writer - Hide protection second area enable
pub type HDP2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_SECWM2R2_SPEC, bool, O>;
impl R {
    ///Bits 16:22 - End page of hide protection second area HDP2_PEND contains the last page of the HDP area in bank 2.
    #[inline(always)]
    pub fn hdp2_pend(&self) -> HDP2_PEND_R {
        HDP2_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 31 - Hide protection second area enable
    #[inline(always)]
    pub fn hdp2en(&self) -> HDP2EN_R {
        HDP2EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 16:22 - End page of hide protection second area HDP2_PEND contains the last page of the HDP area in bank 2.
    #[inline(always)]
    #[must_use]
    pub fn hdp2_pend(&mut self) -> HDP2_PEND_W<16> {
        HDP2_PEND_W::new(self)
    }
    ///Bit 31 - Hide protection second area enable
    #[inline(always)]
    #[must_use]
    pub fn hdp2en(&mut self) -> HDP2EN_W<31> {
        HDP2EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH secure watermark2 register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flash_secwm2r2](index.html) module
pub struct FLASH_SECWM2R2_SPEC;
impl crate::RegisterSpec for FLASH_SECWM2R2_SPEC {
    type Ux = u32;
}
///`read()` method returns [flash_secwm2r2::R](R) reader structure
impl crate::Readable for FLASH_SECWM2R2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [flash_secwm2r2::W](W) writer structure
impl crate::Writable for FLASH_SECWM2R2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLASH_SECWM2R2 to value 0x0f00_ffff
impl crate::Resettable for FLASH_SECWM2R2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f00_ffff;
}
