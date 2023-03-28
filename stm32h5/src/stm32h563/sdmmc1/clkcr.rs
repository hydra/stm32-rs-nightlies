///Register `CLKCR` reader
pub struct R(crate::R<CLKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CLKCR` writer
pub struct W(crate::W<CLKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKCR_SPEC>;
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
impl From<crate::W<CLKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CLKDIV` reader - Clock divide factor This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). This field defines the divide factor between the input clock (sdmmc_ker_ck) and the output clock (SDMMC_CK): SDMMC_CK frequency = sdmmc_ker_ck / \[2 * CLKDIV\]. 0x0XX: etc.. 0xXXX: etc..
pub type CLKDIV_R = crate::FieldReader<u16, u16>;
///Field `CLKDIV` writer - Clock divide factor This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). This field defines the divide factor between the input clock (sdmmc_ker_ck) and the output clock (SDMMC_CK): SDMMC_CK frequency = sdmmc_ker_ck / \[2 * CLKDIV\]. 0x0XX: etc.. 0xXXX: etc..
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKCR_SPEC, u16, u16, 10, O>;
///Field `PWRSAV` reader - Power saving configuration bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) For power saving, the SDMMC_CK clock output can be disabled when the bus is idle by setting PWRSAV:
pub type PWRSAV_R = crate::BitReader<bool>;
///Field `PWRSAV` writer - Power saving configuration bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) For power saving, the SDMMC_CK clock output can be disabled when the bus is idle by setting PWRSAV:
pub type PWRSAV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, bool, O>;
///Field `WIDBUS` reader - Wide bus mode enable bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)
pub type WIDBUS_R = crate::FieldReader<u8, u8>;
///Field `WIDBUS` writer - Wide bus mode enable bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)
pub type WIDBUS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKCR_SPEC, u8, u8, 2, O>;
///Field `NEGEDGE` reader - SDMMC_CK dephasing selection bit for data and command This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). When clock division = 1 (CLKDIV = 0), this bit has no effect. Data and Command change on SDMMC_CK falling edge. Command and data changed on the sdmmc_ker_ck falling edge succeeding the rising edge of SDMMC_CK. SDMMC_CK edge occurs on sdmmc_ker_ck rising edge. When clock division >1 (CLKDIV > 0) &amp; DDR = 1: Command changed on the sdmmc_ker_ck falling edge succeeding the rising edge of SDMMC_CK. Data changed on the sdmmc_ker_ck falling edge succeeding a SDMMC_CK edge. SDMMC_CK edge occurs on sdmmc_ker_ck rising edge. Command and data changed on the same sdmmc_ker_ck rising edge generating the SDMMC_CK falling edge. When clock division >1 (CLKDIV > 0) &amp; DDR = 1: Command changed on the same sdmmc_ker_ck rising edge generating the SDMMC_CK falling edge. Data changed on the SDMMC_CK falling edge succeeding a SDMMC_CK edge. SDMMC_CK edge occurs on sdmmc_ker_ck rising edge.
pub type NEGEDGE_R = crate::BitReader<bool>;
///Field `NEGEDGE` writer - SDMMC_CK dephasing selection bit for data and command This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). When clock division = 1 (CLKDIV = 0), this bit has no effect. Data and Command change on SDMMC_CK falling edge. Command and data changed on the sdmmc_ker_ck falling edge succeeding the rising edge of SDMMC_CK. SDMMC_CK edge occurs on sdmmc_ker_ck rising edge. When clock division >1 (CLKDIV > 0) &amp; DDR = 1: Command changed on the sdmmc_ker_ck falling edge succeeding the rising edge of SDMMC_CK. Data changed on the sdmmc_ker_ck falling edge succeeding a SDMMC_CK edge. SDMMC_CK edge occurs on sdmmc_ker_ck rising edge. Command and data changed on the same sdmmc_ker_ck rising edge generating the SDMMC_CK falling edge. When clock division >1 (CLKDIV > 0) &amp; DDR = 1: Command changed on the same sdmmc_ker_ck rising edge generating the SDMMC_CK falling edge. Data changed on the SDMMC_CK falling edge succeeding a SDMMC_CK edge. SDMMC_CK edge occurs on sdmmc_ker_ck rising edge.
pub type NEGEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, bool, O>;
///Field `HWFC_EN` reader - Hardware flow control enable This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) When Hardware flow control is enabled, the meaning of the TXFIFOE and RXFIFOF flags change, please see SDMMC status register definition in .
pub type HWFC_EN_R = crate::BitReader<bool>;
///Field `HWFC_EN` writer - Hardware flow control enable This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) When Hardware flow control is enabled, the meaning of the TXFIFOE and RXFIFOF flags change, please see SDMMC status register definition in .
pub type HWFC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, bool, O>;
///Field `DDR` reader - Data rate signaling selection This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) DDR rate must only be selected with 4-bit or 8-bit wide bus mode. (WIDBUS > 00). DDR = 1 has no effect when WIDBUS = 00 (1-bit wide bus). DDR rate must only be selected with clock division >1. (CLKDIV > 0)
pub type DDR_R = crate::BitReader<bool>;
///Field `DDR` writer - Data rate signaling selection This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) DDR rate must only be selected with 4-bit or 8-bit wide bus mode. (WIDBUS > 00). DDR = 1 has no effect when WIDBUS = 00 (1-bit wide bus). DDR rate must only be selected with clock division >1. (CLKDIV > 0)
pub type DDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, bool, O>;
///Field `BUSSPEED` reader - Bus speed for selection of SDMMC operating modes This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)
pub type BUSSPEED_R = crate::BitReader<bool>;
///Field `BUSSPEED` writer - Bus speed for selection of SDMMC operating modes This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)
pub type BUSSPEED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, bool, O>;
///Field `SELCLKRX` reader - Receive clock selection These bits can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)
pub type SELCLKRX_R = crate::FieldReader<u8, u8>;
///Field `SELCLKRX` writer - Receive clock selection These bits can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)
pub type SELCLKRX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKCR_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:9 - Clock divide factor This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). This field defines the divide factor between the input clock (sdmmc_ker_ck) and the output clock (SDMMC_CK): SDMMC_CK frequency = sdmmc_ker_ck / \[2 * CLKDIV\]. 0x0XX: etc.. 0xXXX: etc..
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 12 - Power saving configuration bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) For power saving, the SDMMC_CK clock output can be disabled when the bus is idle by setting PWRSAV:
    #[inline(always)]
    pub fn pwrsav(&self) -> PWRSAV_R {
        PWRSAV_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 14:15 - Wide bus mode enable bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)
    #[inline(always)]
    pub fn widbus(&self) -> WIDBUS_R {
        WIDBUS_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - SDMMC_CK dephasing selection bit for data and command This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). When clock division = 1 (CLKDIV = 0), this bit has no effect. Data and Command change on SDMMC_CK falling edge. Command and data changed on the sdmmc_ker_ck falling edge succeeding the rising edge of SDMMC_CK. SDMMC_CK edge occurs on sdmmc_ker_ck rising edge. When clock division >1 (CLKDIV > 0) &amp; DDR = 1: Command changed on the sdmmc_ker_ck falling edge succeeding the rising edge of SDMMC_CK. Data changed on the sdmmc_ker_ck falling edge succeeding a SDMMC_CK edge. SDMMC_CK edge occurs on sdmmc_ker_ck rising edge. Command and data changed on the same sdmmc_ker_ck rising edge generating the SDMMC_CK falling edge. When clock division >1 (CLKDIV > 0) &amp; DDR = 1: Command changed on the same sdmmc_ker_ck rising edge generating the SDMMC_CK falling edge. Data changed on the SDMMC_CK falling edge succeeding a SDMMC_CK edge. SDMMC_CK edge occurs on sdmmc_ker_ck rising edge.
    #[inline(always)]
    pub fn negedge(&self) -> NEGEDGE_R {
        NEGEDGE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Hardware flow control enable This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) When Hardware flow control is enabled, the meaning of the TXFIFOE and RXFIFOF flags change, please see SDMMC status register definition in .
    #[inline(always)]
    pub fn hwfc_en(&self) -> HWFC_EN_R {
        HWFC_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Data rate signaling selection This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) DDR rate must only be selected with 4-bit or 8-bit wide bus mode. (WIDBUS > 00). DDR = 1 has no effect when WIDBUS = 00 (1-bit wide bus). DDR rate must only be selected with clock division >1. (CLKDIV > 0)
    #[inline(always)]
    pub fn ddr(&self) -> DDR_R {
        DDR_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Bus speed for selection of SDMMC operating modes This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)
    #[inline(always)]
    pub fn busspeed(&self) -> BUSSPEED_R {
        BUSSPEED_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:21 - Receive clock selection These bits can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)
    #[inline(always)]
    pub fn selclkrx(&self) -> SELCLKRX_R {
        SELCLKRX_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    ///Bits 0:9 - Clock divide factor This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). This field defines the divide factor between the input clock (sdmmc_ker_ck) and the output clock (SDMMC_CK): SDMMC_CK frequency = sdmmc_ker_ck / \[2 * CLKDIV\]. 0x0XX: etc.. 0xXXX: etc..
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<0> {
        CLKDIV_W::new(self)
    }
    ///Bit 12 - Power saving configuration bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) For power saving, the SDMMC_CK clock output can be disabled when the bus is idle by setting PWRSAV:
    #[inline(always)]
    #[must_use]
    pub fn pwrsav(&mut self) -> PWRSAV_W<12> {
        PWRSAV_W::new(self)
    }
    ///Bits 14:15 - Wide bus mode enable bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)
    #[inline(always)]
    #[must_use]
    pub fn widbus(&mut self) -> WIDBUS_W<14> {
        WIDBUS_W::new(self)
    }
    ///Bit 16 - SDMMC_CK dephasing selection bit for data and command This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). When clock division = 1 (CLKDIV = 0), this bit has no effect. Data and Command change on SDMMC_CK falling edge. Command and data changed on the sdmmc_ker_ck falling edge succeeding the rising edge of SDMMC_CK. SDMMC_CK edge occurs on sdmmc_ker_ck rising edge. When clock division >1 (CLKDIV > 0) &amp; DDR = 1: Command changed on the sdmmc_ker_ck falling edge succeeding the rising edge of SDMMC_CK. Data changed on the sdmmc_ker_ck falling edge succeeding a SDMMC_CK edge. SDMMC_CK edge occurs on sdmmc_ker_ck rising edge. Command and data changed on the same sdmmc_ker_ck rising edge generating the SDMMC_CK falling edge. When clock division >1 (CLKDIV > 0) &amp; DDR = 1: Command changed on the same sdmmc_ker_ck rising edge generating the SDMMC_CK falling edge. Data changed on the SDMMC_CK falling edge succeeding a SDMMC_CK edge. SDMMC_CK edge occurs on sdmmc_ker_ck rising edge.
    #[inline(always)]
    #[must_use]
    pub fn negedge(&mut self) -> NEGEDGE_W<16> {
        NEGEDGE_W::new(self)
    }
    ///Bit 17 - Hardware flow control enable This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) When Hardware flow control is enabled, the meaning of the TXFIFOE and RXFIFOF flags change, please see SDMMC status register definition in .
    #[inline(always)]
    #[must_use]
    pub fn hwfc_en(&mut self) -> HWFC_EN_W<17> {
        HWFC_EN_W::new(self)
    }
    ///Bit 18 - Data rate signaling selection This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0) DDR rate must only be selected with 4-bit or 8-bit wide bus mode. (WIDBUS > 00). DDR = 1 has no effect when WIDBUS = 00 (1-bit wide bus). DDR rate must only be selected with clock division >1. (CLKDIV > 0)
    #[inline(always)]
    #[must_use]
    pub fn ddr(&mut self) -> DDR_W<18> {
        DDR_W::new(self)
    }
    ///Bit 19 - Bus speed for selection of SDMMC operating modes This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)
    #[inline(always)]
    #[must_use]
    pub fn busspeed(&mut self) -> BUSSPEED_W<19> {
        BUSSPEED_W::new(self)
    }
    ///Bits 20:21 - Receive clock selection These bits can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)
    #[inline(always)]
    #[must_use]
    pub fn selclkrx(&mut self) -> SELCLKRX_W<20> {
        SELCLKRX_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SDMMC clock control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [clkcr](index.html) module
pub struct CLKCR_SPEC;
impl crate::RegisterSpec for CLKCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [clkcr::R](R) reader structure
impl crate::Readable for CLKCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [clkcr::W](W) writer structure
impl crate::Writable for CLKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CLKCR to value 0
impl crate::Resettable for CLKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
