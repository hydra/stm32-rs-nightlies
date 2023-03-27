///Register `RCC_ICSCR1` reader
pub struct R(crate::R<RCC_ICSCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_ICSCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_ICSCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_ICSCR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_ICSCR1` writer
pub struct W(crate::W<RCC_ICSCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_ICSCR1_SPEC>;
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
impl From<crate::W<RCC_ICSCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_ICSCR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MSICAL3` reader - MSIRC3 clock calibration for MSI ranges 12 to 15 These bits are initialized at startup with the factory-programmed MSIRC3 calibration trim value for ranges 12 to 15. When MSITRIM3 is written, MSICAL3 is updated with the sum of MSITRIM3\[4:0\]
///and the factory calibration trim value MSIRC2\[4:0\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.
pub type MSICAL3_R = crate::FieldReader<u8, u8>;
///Field `MSICAL2` reader - MSIRC2 clock calibration for MSI ranges 8 to 11 These bits are initialized at startup with the factory-programmed MSIRC2 calibration trim value for ranges 8 to 11. When MSITRIM2 is written, MSICAL2 is updated with the sum of MSITRIM2\[4:0\]
///and the factory calibration trim value MSIRC2\[4:0\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.
pub type MSICAL2_R = crate::FieldReader<u8, u8>;
///Field `MSICAL1` reader - MSIRC1 clock calibration for MSI ranges 4 to 7 These bits are initialized at startup with the factory-programmed MSIRC1 calibration trim value for ranges 4 to 7. When MSITRIM1 is written, MSICAL1 is updated with the sum of MSITRIM1\[4:0\]
///and the factory calibration trim value MSIRC1\[4:0\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.
pub type MSICAL1_R = crate::FieldReader<u8, u8>;
///Field `MSICAL0` reader - MSIRC0 clock calibration for MSI ranges 0 to 3 These bits are initialized at startup with the factory-programmed MSIRC0 calibration trim value for ranges 0 to 3. When MSITRIM0 is written, MSICAL0 is updated with the sum of MSITRIM0\[4:0\]
///and the factory-programmed calibration trim value MSIRC0\[4:0\].
pub type MSICAL0_R = crate::FieldReader<u8, u8>;
///Field `MSIBIAS` reader - MSI bias mode selection Set by software to select the MSI bias mode. By default, the MSI bias is in continuous mode in order to maintain the output clocks accuracy. Setting this bit reduces the MSI consumption under range 4 but decrease its accuracy.
pub type MSIBIAS_R = crate::BitReader<bool>;
///Field `MSIBIAS` writer - MSI bias mode selection Set by software to select the MSI bias mode. By default, the MSI bias is in continuous mode in order to maintain the output clocks accuracy. Setting this bit reduces the MSI consumption under range 4 but decrease its accuracy.
pub type MSIBIAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_ICSCR1_SPEC, bool, O>;
///Field `MSIRGSEL` reader - MSI clock range selection Set by software to select the MSIS and MSIK clocks range with MSISRANGE\[3:0\]
///and MSIKRANGE\[3:0\]. Write 0 has no effect. After exiting Standby or Shutdown mode, or after a reset, this bit is at 0 and the MSIS and MSIK ranges are provided by MSISSRANGE\[3:0\]
///and MSIKSRANGE\[3:0\]
///in RCC_CSR.
pub type MSIRGSEL_R = crate::BitReader<bool>;
///Field `MSIRGSEL` writer - MSI clock range selection Set by software to select the MSIS and MSIK clocks range with MSISRANGE\[3:0\]
///and MSIKRANGE\[3:0\]. Write 0 has no effect. After exiting Standby or Shutdown mode, or after a reset, this bit is at 0 and the MSIS and MSIK ranges are provided by MSISSRANGE\[3:0\]
///and MSIKSRANGE\[3:0\]
///in RCC_CSR.
pub type MSIRGSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_ICSCR1_SPEC, bool, O>;
///Field `MSIKRANGE` reader - MSIK clock ranges These bits are configured by software to choose the frequency range of MSIK oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSIKRANGE can be modified when MSIK is OFF (MSISON = 0) or when MSIK is ready (MSIKRDY = 1). MSIKRANGE must NOT be modified when MSIK is ON and NOT ready (MSIKON = 1 and MSIKRDY = 0) MSIKRANGE is kept when the device wakes up from Stop mode, except when the MSIK range is above 24 MHz. In this case MSIKRANGE is changed by hardware into Range 2 (24 MHz).
pub type MSIKRANGE_R = crate::FieldReader<u8, u8>;
///Field `MSIKRANGE` writer - MSIK clock ranges These bits are configured by software to choose the frequency range of MSIK oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSIKRANGE can be modified when MSIK is OFF (MSISON = 0) or when MSIK is ready (MSIKRDY = 1). MSIKRANGE must NOT be modified when MSIK is ON and NOT ready (MSIKON = 1 and MSIKRDY = 0) MSIKRANGE is kept when the device wakes up from Stop mode, except when the MSIK range is above 24 MHz. In this case MSIKRANGE is changed by hardware into Range 2 (24 MHz).
pub type MSIKRANGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_ICSCR1_SPEC, u8, u8, 4, O>;
///Field `MSISRANGE` reader - MSIS clock ranges These bits are configured by software to choose the frequency range of MSIS oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSISRANGE can be modified when MSIS is OFF (MSISON = 0) or when MSIS is ready (MSISRDY = 1). MSISRANGE must NOT be modified when MSIS is ON and NOT ready (MSISON = 1 and MSISRDY = 0) MSISRANGE is kept when the device wakes up from Stop mode, except when the MSIS range is above 24 MHz. In this case MSISRANGE is changed by hardware into Range 2 (24 MHz).
pub type MSISRANGE_R = crate::FieldReader<u8, u8>;
///Field `MSISRANGE` writer - MSIS clock ranges These bits are configured by software to choose the frequency range of MSIS oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSISRANGE can be modified when MSIS is OFF (MSISON = 0) or when MSIS is ready (MSISRDY = 1). MSISRANGE must NOT be modified when MSIS is ON and NOT ready (MSISON = 1 and MSISRDY = 0) MSISRANGE is kept when the device wakes up from Stop mode, except when the MSIS range is above 24 MHz. In this case MSISRANGE is changed by hardware into Range 2 (24 MHz).
pub type MSISRANGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_ICSCR1_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:4 - MSIRC3 clock calibration for MSI ranges 12 to 15 These bits are initialized at startup with the factory-programmed MSIRC3 calibration trim value for ranges 12 to 15. When MSITRIM3 is written, MSICAL3 is updated with the sum of MSITRIM3\[4:0\]
    ///and the factory calibration trim value MSIRC2\[4:0\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.
    #[inline(always)]
    pub fn msical3(&self) -> MSICAL3_R {
        MSICAL3_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9 - MSIRC2 clock calibration for MSI ranges 8 to 11 These bits are initialized at startup with the factory-programmed MSIRC2 calibration trim value for ranges 8 to 11. When MSITRIM2 is written, MSICAL2 is updated with the sum of MSITRIM2\[4:0\]
    ///and the factory calibration trim value MSIRC2\[4:0\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.
    #[inline(always)]
    pub fn msical2(&self) -> MSICAL2_R {
        MSICAL2_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14 - MSIRC1 clock calibration for MSI ranges 4 to 7 These bits are initialized at startup with the factory-programmed MSIRC1 calibration trim value for ranges 4 to 7. When MSITRIM1 is written, MSICAL1 is updated with the sum of MSITRIM1\[4:0\]
    ///and the factory calibration trim value MSIRC1\[4:0\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level.
    #[inline(always)]
    pub fn msical1(&self) -> MSICAL1_R {
        MSICAL1_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bits 15:19 - MSIRC0 clock calibration for MSI ranges 0 to 3 These bits are initialized at startup with the factory-programmed MSIRC0 calibration trim value for ranges 0 to 3. When MSITRIM0 is written, MSICAL0 is updated with the sum of MSITRIM0\[4:0\]
    ///and the factory-programmed calibration trim value MSIRC0\[4:0\].
    #[inline(always)]
    pub fn msical0(&self) -> MSICAL0_R {
        MSICAL0_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bit 22 - MSI bias mode selection Set by software to select the MSI bias mode. By default, the MSI bias is in continuous mode in order to maintain the output clocks accuracy. Setting this bit reduces the MSI consumption under range 4 but decrease its accuracy.
    #[inline(always)]
    pub fn msibias(&self) -> MSIBIAS_R {
        MSIBIAS_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - MSI clock range selection Set by software to select the MSIS and MSIK clocks range with MSISRANGE\[3:0\]
    ///and MSIKRANGE\[3:0\]. Write 0 has no effect. After exiting Standby or Shutdown mode, or after a reset, this bit is at 0 and the MSIS and MSIK ranges are provided by MSISSRANGE\[3:0\]
    ///and MSIKSRANGE\[3:0\]
    ///in RCC_CSR.
    #[inline(always)]
    pub fn msirgsel(&self) -> MSIRGSEL_R {
        MSIRGSEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:27 - MSIK clock ranges These bits are configured by software to choose the frequency range of MSIK oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSIKRANGE can be modified when MSIK is OFF (MSISON = 0) or when MSIK is ready (MSIKRDY = 1). MSIKRANGE must NOT be modified when MSIK is ON and NOT ready (MSIKON = 1 and MSIKRDY = 0) MSIKRANGE is kept when the device wakes up from Stop mode, except when the MSIK range is above 24 MHz. In this case MSIKRANGE is changed by hardware into Range 2 (24 MHz).
    #[inline(always)]
    pub fn msikrange(&self) -> MSIKRANGE_R {
        MSIKRANGE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - MSIS clock ranges These bits are configured by software to choose the frequency range of MSIS oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSISRANGE can be modified when MSIS is OFF (MSISON = 0) or when MSIS is ready (MSISRDY = 1). MSISRANGE must NOT be modified when MSIS is ON and NOT ready (MSISON = 1 and MSISRDY = 0) MSISRANGE is kept when the device wakes up from Stop mode, except when the MSIS range is above 24 MHz. In this case MSISRANGE is changed by hardware into Range 2 (24 MHz).
    #[inline(always)]
    pub fn msisrange(&self) -> MSISRANGE_R {
        MSISRANGE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 22 - MSI bias mode selection Set by software to select the MSI bias mode. By default, the MSI bias is in continuous mode in order to maintain the output clocks accuracy. Setting this bit reduces the MSI consumption under range 4 but decrease its accuracy.
    #[inline(always)]
    #[must_use]
    pub fn msibias(&mut self) -> MSIBIAS_W<22> {
        MSIBIAS_W::new(self)
    }
    ///Bit 23 - MSI clock range selection Set by software to select the MSIS and MSIK clocks range with MSISRANGE\[3:0\]
    ///and MSIKRANGE\[3:0\]. Write 0 has no effect. After exiting Standby or Shutdown mode, or after a reset, this bit is at 0 and the MSIS and MSIK ranges are provided by MSISSRANGE\[3:0\]
    ///and MSIKSRANGE\[3:0\]
    ///in RCC_CSR.
    #[inline(always)]
    #[must_use]
    pub fn msirgsel(&mut self) -> MSIRGSEL_W<23> {
        MSIRGSEL_W::new(self)
    }
    ///Bits 24:27 - MSIK clock ranges These bits are configured by software to choose the frequency range of MSIK oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSIKRANGE can be modified when MSIK is OFF (MSISON = 0) or when MSIK is ready (MSIKRDY = 1). MSIKRANGE must NOT be modified when MSIK is ON and NOT ready (MSIKON = 1 and MSIKRDY = 0) MSIKRANGE is kept when the device wakes up from Stop mode, except when the MSIK range is above 24 MHz. In this case MSIKRANGE is changed by hardware into Range 2 (24 MHz).
    #[inline(always)]
    #[must_use]
    pub fn msikrange(&mut self) -> MSIKRANGE_W<24> {
        MSIKRANGE_W::new(self)
    }
    ///Bits 28:31 - MSIS clock ranges These bits are configured by software to choose the frequency range of MSIS oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSISRANGE can be modified when MSIS is OFF (MSISON = 0) or when MSIS is ready (MSISRDY = 1). MSISRANGE must NOT be modified when MSIS is ON and NOT ready (MSISON = 1 and MSISRDY = 0) MSISRANGE is kept when the device wakes up from Stop mode, except when the MSIS range is above 24 MHz. In this case MSISRANGE is changed by hardware into Range 2 (24 MHz).
    #[inline(always)]
    #[must_use]
    pub fn msisrange(&mut self) -> MSISRANGE_W<28> {
        MSISRANGE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC internal clock sources calibration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_icscr1](index.html) module
pub struct RCC_ICSCR1_SPEC;
impl crate::RegisterSpec for RCC_ICSCR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_icscr1::R](R) reader structure
impl crate::Readable for RCC_ICSCR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_icscr1::W](W) writer structure
impl crate::Writable for RCC_ICSCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_ICSCR1 to value 0x4400_0000
impl crate::Resettable for RCC_ICSCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x4400_0000;
}
