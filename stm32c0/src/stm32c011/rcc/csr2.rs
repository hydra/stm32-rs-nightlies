///Register `CSR2` reader
pub struct R(crate::R<CSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSR2` writer
pub struct W(crate::W<CSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR2_SPEC>;
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
impl From<crate::W<CSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LSION` reader - LSI oscillator enable Set and cleared by software to enable/disable the LSI oscillator:
pub type LSION_R = crate::BitReader<bool>;
///Field `LSION` writer - LSI oscillator enable Set and cleared by software to enable/disable the LSI oscillator:
pub type LSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR2_SPEC, bool, O>;
///Field `LSIRDY` reader - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is ready (stable): After the LSION bit is cleared, LSIRDY goes low after 3 LSI oscillator clock cycles. This bit can be set even if LSION = 0 if the LSI is requested by the Clock Security System on LSE, by the Independent Watchdog or by the RTC.
pub type LSIRDY_R = crate::BitReader<bool>;
///Field `RMVF` reader - Remove reset flags Set by software to clear the reset flags.
pub type RMVF_R = crate::BitReader<bool>;
///Field `RMVF` writer - Remove reset flags Set by software to clear the reset flags.
pub type RMVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR2_SPEC, bool, O>;
///Field `OBLRSTF` reader - Option byte loader reset flag Set by hardware when a reset from the Option byte loading occurs. Cleared by setting the RMVF bit.
pub type OBLRSTF_R = crate::BitReader<bool>;
///Field `PINRSTF` reader - Pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by setting the RMVF bit.
pub type PINRSTF_R = crate::BitReader<bool>;
///Field `PWRRSTF` reader - BOR or POR/PDR flag Set by hardware when a BOR or POR/PDR occurs. Cleared by setting the RMVF bit.
pub type PWRRSTF_R = crate::BitReader<bool>;
///Field `SFTRSTF` reader - Software reset flag Set by hardware when a software reset occurs. Cleared by setting the RMVF bit.
pub type SFTRSTF_R = crate::BitReader<bool>;
///Field `IWDGRSTF` reader - Independent window watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by setting the RMVF bit.
pub type IWDGRSTF_R = crate::BitReader<bool>;
///Field `WWDGRSTF` reader - Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by setting the RMVF bit.
pub type WWDGRSTF_R = crate::BitReader<bool>;
///Field `LPWRRSTF` reader - Low-power reset flag Set by hardware when a reset occurs due to illegal Stop, or Standby, or Shutdown mode entry. Cleared by setting the RMVF bit. This operates only if nRST_STOP, or nRST_STDBY or nRST_SHDW option bits are cleared.
pub type LPWRRSTF_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - LSI oscillator enable Set and cleared by software to enable/disable the LSI oscillator:
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is ready (stable): After the LSION bit is cleared, LSIRDY goes low after 3 LSI oscillator clock cycles. This bit can be set even if LSION = 0 if the LSI is requested by the Clock Security System on LSE, by the Independent Watchdog or by the RTC.
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 23 - Remove reset flags Set by software to clear the reset flags.
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - Option byte loader reset flag Set by hardware when a reset from the Option byte loading occurs. Cleared by setting the RMVF bit.
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by setting the RMVF bit.
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - BOR or POR/PDR flag Set by hardware when a BOR or POR/PDR occurs. Cleared by setting the RMVF bit.
    #[inline(always)]
    pub fn pwrrstf(&self) -> PWRRSTF_R {
        PWRRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Software reset flag Set by hardware when a software reset occurs. Cleared by setting the RMVF bit.
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Independent window watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by setting the RMVF bit.
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by setting the RMVF bit.
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low-power reset flag Set by hardware when a reset occurs due to illegal Stop, or Standby, or Shutdown mode entry. Cleared by setting the RMVF bit. This operates only if nRST_STOP, or nRST_STDBY or nRST_SHDW option bits are cleared.
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LSI oscillator enable Set and cleared by software to enable/disable the LSI oscillator:
    #[inline(always)]
    #[must_use]
    pub fn lsion(&mut self) -> LSION_W<0> {
        LSION_W::new(self)
    }
    ///Bit 23 - Remove reset flags Set by software to clear the reset flags.
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
///RCC control/status register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr2](index.html) module
pub struct CSR2_SPEC;
impl crate::RegisterSpec for CSR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr2::R](R) reader structure
impl crate::Readable for CSR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csr2::W](W) writer structure
impl crate::Writable for CSR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSR2 to value 0
impl crate::Resettable for CSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
