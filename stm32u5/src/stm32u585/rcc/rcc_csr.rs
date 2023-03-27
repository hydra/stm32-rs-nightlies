///Register `RCC_CSR` reader
pub struct R(crate::R<RCC_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_CSR` writer
pub struct W(crate::W<RCC_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CSR_SPEC>;
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
impl From<crate::W<RCC_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MSIKSRANGE` reader - MSIK range after Standby mode Set by software to chose the MSIK frequency at startup. This range is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4 MHz. MSIKSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing the MSIKSRANGE does not change the current MSIK frequency.
pub type MSIKSRANGE_R = crate::FieldReader<u8, u8>;
///Field `MSIKSRANGE` writer - MSIK range after Standby mode Set by software to chose the MSIK frequency at startup. This range is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4 MHz. MSIKSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing the MSIKSRANGE does not change the current MSIK frequency.
pub type MSIKSRANGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CSR_SPEC, u8, u8, 4, O>;
///Field `MSISSRANGE` reader - MSIS range after Standby mode Set by software to chose the MSIS frequency at startup. This range is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4 MHz. MSISSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing the MSISSRANGE does not change the current MSIS frequency.
pub type MSISSRANGE_R = crate::FieldReader<u8, u8>;
///Field `MSISSRANGE` writer - MSIS range after Standby mode Set by software to chose the MSIS frequency at startup. This range is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4 MHz. MSISSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing the MSISSRANGE does not change the current MSIS frequency.
pub type MSISSRANGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CSR_SPEC, u8, u8, 4, O>;
///Field `RMVF` reader - Remove reset flag Set by software to clear the reset flags.
pub type RMVF_R = crate::BitReader<bool>;
///Field `RMVF` writer - Remove reset flag Set by software to clear the reset flags.
pub type RMVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CSR_SPEC, bool, O>;
///Field `OBLRSTF` reader - Option byte loader reset flag Set by hardware when a reset from the option byte loading occurs. Cleared by writing to the RMVF bit.
pub type OBLRSTF_R = crate::BitReader<bool>;
///Field `PINRSTF` reader - NRST pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by writing to the RMVF bit.
pub type PINRSTF_R = crate::BitReader<bool>;
///Field `BORRSTF` reader - BOR flag Set by hardware when a BOR occurs. Cleared by writing to the RMVF bit.
pub type BORRSTF_R = crate::BitReader<bool>;
///Field `SFTRSTF` reader - Software reset flag Set by hardware when a software reset occurs. Cleared by writing to the RMVF bit.
pub type SFTRSTF_R = crate::BitReader<bool>;
///Field `IWDGRSTF` reader - Independent watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by writing to the RMVF bit.
pub type IWDGRSTF_R = crate::BitReader<bool>;
///Field `WWDGRSTF` reader - Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by writing to the RMVF bit.
pub type WWDGRSTF_R = crate::BitReader<bool>;
///Field `LPWRRSTF` reader - Low-power reset flag Set by hardware when a reset occurs due to Stop, Standby or Shutdown mode entry, whereas the corresponding nRST_STOP, nRST_STBY or nRST_SHDW option bit is cleared. Cleared by writing to the RMVF bit.
pub type LPWRRSTF_R = crate::BitReader<bool>;
impl R {
    ///Bits 8:11 - MSIK range after Standby mode Set by software to chose the MSIK frequency at startup. This range is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4 MHz. MSIKSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing the MSIKSRANGE does not change the current MSIK frequency.
    #[inline(always)]
    pub fn msiksrange(&self) -> MSIKSRANGE_R {
        MSIKSRANGE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - MSIS range after Standby mode Set by software to chose the MSIS frequency at startup. This range is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4 MHz. MSISSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing the MSISSRANGE does not change the current MSIS frequency.
    #[inline(always)]
    pub fn msissrange(&self) -> MSISSRANGE_R {
        MSISSRANGE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bit 23 - Remove reset flag Set by software to clear the reset flags.
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - Option byte loader reset flag Set by hardware when a reset from the option byte loading occurs. Cleared by writing to the RMVF bit.
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - NRST pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by writing to the RMVF bit.
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - BOR flag Set by hardware when a BOR occurs. Cleared by writing to the RMVF bit.
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Software reset flag Set by hardware when a software reset occurs. Cleared by writing to the RMVF bit.
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Independent watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by writing to the RMVF bit.
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by writing to the RMVF bit.
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low-power reset flag Set by hardware when a reset occurs due to Stop, Standby or Shutdown mode entry, whereas the corresponding nRST_STOP, nRST_STBY or nRST_SHDW option bit is cleared. Cleared by writing to the RMVF bit.
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 8:11 - MSIK range after Standby mode Set by software to chose the MSIK frequency at startup. This range is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4 MHz. MSIKSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing the MSIKSRANGE does not change the current MSIK frequency.
    #[inline(always)]
    #[must_use]
    pub fn msiksrange(&mut self) -> MSIKSRANGE_W<8> {
        MSIKSRANGE_W::new(self)
    }
    ///Bits 12:15 - MSIS range after Standby mode Set by software to chose the MSIS frequency at startup. This range is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4 MHz. MSISSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing the MSISSRANGE does not change the current MSIS frequency.
    #[inline(always)]
    #[must_use]
    pub fn msissrange(&mut self) -> MSISSRANGE_W<12> {
        MSISSRANGE_W::new(self)
    }
    ///Bit 23 - Remove reset flag Set by software to clear the reset flags.
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<23> {
        RMVF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_csr](index.html) module
pub struct RCC_CSR_SPEC;
impl crate::RegisterSpec for RCC_CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_csr::R](R) reader structure
impl crate::Readable for RCC_CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_csr::W](W) writer structure
impl crate::Writable for RCC_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_CSR to value 0x0c00_4400
impl crate::Resettable for RCC_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c00_4400;
}
