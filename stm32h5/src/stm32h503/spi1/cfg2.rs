///Register `CFG2` reader
pub struct R(crate::R<CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFG2` writer
pub struct W(crate::W<CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG2_SPEC>;
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
impl From<crate::W<CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MSSI` reader - Master SS Idleness Specifies an extra delay, expressed in number of SPI clock cycle periods, inserted additionally between active edge of SS opening a session and the beginning of the first data frame of the session in Master mode when SSOE is enabled. ... Note: This feature is not supported in TI mode. To include the delay, the SPI must be disabled and re-enabled between sessions.
pub type MSSI_R = crate::FieldReader<u8, u8>;
///Field `MSSI` writer - Master SS Idleness Specifies an extra delay, expressed in number of SPI clock cycle periods, inserted additionally between active edge of SS opening a session and the beginning of the first data frame of the session in Master mode when SSOE is enabled. ... Note: This feature is not supported in TI mode. To include the delay, the SPI must be disabled and re-enabled between sessions.
pub type MSSI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG2_SPEC, u8, u8, 4, O>;
///Field `MIDI` reader - master Inter-Data Idleness Specifies minimum time delay (expressed in SPI clock cycles periods) inserted between two consecutive data frames in Master mode. ... Note: This feature is not supported in TI mode.
pub type MIDI_R = crate::FieldReader<u8, u8>;
///Field `MIDI` writer - master Inter-Data Idleness Specifies minimum time delay (expressed in SPI clock cycles periods) inserted between two consecutive data frames in Master mode. ... Note: This feature is not supported in TI mode.
pub type MIDI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG2_SPEC, u8, u8, 4, O>;
///Field `RDIOM` reader - RDY signal input/output management Note: When DSIZE at the SPI_CFG1 register is configured shorter than 8-bit, the RDIOM bit has to be kept at zero.
pub type RDIOM_R = crate::BitReader<bool>;
///Field `RDIOM` writer - RDY signal input/output management Note: When DSIZE at the SPI_CFG1 register is configured shorter than 8-bit, the RDIOM bit has to be kept at zero.
pub type RDIOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
///Field `RDIOP` reader - RDY signal input/output polarity
pub type RDIOP_R = crate::BitReader<bool>;
///Field `RDIOP` writer - RDY signal input/output polarity
pub type RDIOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
///Field `IOSWP` reader - swap functionality of MISO and MOSI pins When this bit is set, the function of MISO and MOSI pins alternate functions are inverted. Original MISO pin becomes MOSI and original MOSI pin becomes MISO. Note: This bit can be also used in PCM and I2S modes to swap SDO and SDI pins.
pub type IOSWP_R = crate::BitReader<bool>;
///Field `IOSWP` writer - swap functionality of MISO and MOSI pins When this bit is set, the function of MISO and MOSI pins alternate functions are inverted. Original MISO pin becomes MOSI and original MOSI pin becomes MISO. Note: This bit can be also used in PCM and I2S modes to swap SDO and SDI pins.
pub type IOSWP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
///Field `COMM` reader - SPI Communication Mode
pub type COMM_R = crate::FieldReader<u8, u8>;
///Field `COMM` writer - SPI Communication Mode
pub type COMM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG2_SPEC, u8, u8, 2, O>;
///Field `SP` reader - serial protocol others: reserved, must not be used
pub type SP_R = crate::FieldReader<u8, u8>;
///Field `SP` writer - serial protocol others: reserved, must not be used
pub type SP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG2_SPEC, u8, u8, 3, O>;
///Field `MASTER` reader - SPI Master
pub type MASTER_R = crate::BitReader<bool>;
///Field `MASTER` writer - SPI Master
pub type MASTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
///Field `LSBFRST` reader - data frame format Note: This bit can be also used in PCM and I2S modes.
pub type LSBFRST_R = crate::BitReader<bool>;
///Field `LSBFRST` writer - data frame format Note: This bit can be also used in PCM and I2S modes.
pub type LSBFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
///Field `CPHA` reader - clock phase
pub type CPHA_R = crate::BitReader<bool>;
///Field `CPHA` writer - clock phase
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
///Field `CPOL` reader - clock polarity
pub type CPOL_R = crate::BitReader<bool>;
///Field `CPOL` writer - clock polarity
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
///Field `SSM` reader - software management of SS signal input When master uses hardware SS output (SSM=0 and SSOE=1) the SS signal input is forced to not active state internally to prevent master mode fault error.
pub type SSM_R = crate::BitReader<bool>;
///Field `SSM` writer - software management of SS signal input When master uses hardware SS output (SSM=0 and SSOE=1) the SS signal input is forced to not active state internally to prevent master mode fault error.
pub type SSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
///Field `SSIOP` reader - SS input/output polarity
pub type SSIOP_R = crate::BitReader<bool>;
///Field `SSIOP` writer - SS input/output polarity
pub type SSIOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
///Field `SSOE` reader - SS output enable This bit is taken into account in Master mode only
pub type SSOE_R = crate::BitReader<bool>;
///Field `SSOE` writer - SS output enable This bit is taken into account in Master mode only
pub type SSOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
///Field `SSOM` reader - SS output management in Master mode This bit is taken into account in Master mode when SSOE is enabled. It allows the SS output to be configured between two consecutive data transfers.
pub type SSOM_R = crate::BitReader<bool>;
///Field `SSOM` writer - SS output management in Master mode This bit is taken into account in Master mode when SSOE is enabled. It allows the SS output to be configured between two consecutive data transfers.
pub type SSOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
///Field `AFCNTR` reader - alternate function GPIOs control This bit is taken into account when SPE=0 only When SPI has to be disabled temporary for a specific configuration reason (e.g. CRC reset, CPHA or HDDIR change) setting this bit prevents any glitches on the associated outputs configured at alternate function mode by keeping them forced at state corresponding the current SPI configuration. Note: This bit can be also used in PCM and I2S modes. Note: The bit AFCNTR must not be set to 1, when the block is in slave mode.
pub type AFCNTR_R = crate::BitReader<bool>;
///Field `AFCNTR` writer - alternate function GPIOs control This bit is taken into account when SPE=0 only When SPI has to be disabled temporary for a specific configuration reason (e.g. CRC reset, CPHA or HDDIR change) setting this bit prevents any glitches on the associated outputs configured at alternate function mode by keeping them forced at state corresponding the current SPI configuration. Note: This bit can be also used in PCM and I2S modes. Note: The bit AFCNTR must not be set to 1, when the block is in slave mode.
pub type AFCNTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
impl R {
    ///Bits 0:3 - Master SS Idleness Specifies an extra delay, expressed in number of SPI clock cycle periods, inserted additionally between active edge of SS opening a session and the beginning of the first data frame of the session in Master mode when SSOE is enabled. ... Note: This feature is not supported in TI mode. To include the delay, the SPI must be disabled and re-enabled between sessions.
    #[inline(always)]
    pub fn mssi(&self) -> MSSI_R {
        MSSI_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - master Inter-Data Idleness Specifies minimum time delay (expressed in SPI clock cycles periods) inserted between two consecutive data frames in Master mode. ... Note: This feature is not supported in TI mode.
    #[inline(always)]
    pub fn midi(&self) -> MIDI_R {
        MIDI_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 13 - RDY signal input/output management Note: When DSIZE at the SPI_CFG1 register is configured shorter than 8-bit, the RDIOM bit has to be kept at zero.
    #[inline(always)]
    pub fn rdiom(&self) -> RDIOM_R {
        RDIOM_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - RDY signal input/output polarity
    #[inline(always)]
    pub fn rdiop(&self) -> RDIOP_R {
        RDIOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - swap functionality of MISO and MOSI pins When this bit is set, the function of MISO and MOSI pins alternate functions are inverted. Original MISO pin becomes MOSI and original MOSI pin becomes MISO. Note: This bit can be also used in PCM and I2S modes to swap SDO and SDI pins.
    #[inline(always)]
    pub fn ioswp(&self) -> IOSWP_R {
        IOSWP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 17:18 - SPI Communication Mode
    #[inline(always)]
    pub fn comm(&self) -> COMM_R {
        COMM_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:21 - serial protocol others: reserved, must not be used
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 19) & 7) as u8)
    }
    ///Bit 22 - SPI Master
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - data frame format Note: This bit can be also used in PCM and I2S modes.
    #[inline(always)]
    pub fn lsbfrst(&self) -> LSBFRST_R {
        LSBFRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - clock phase
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - clock polarity
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - software management of SS signal input When master uses hardware SS output (SSM=0 and SSOE=1) the SS signal input is forced to not active state internally to prevent master mode fault error.
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - SS input/output polarity
    #[inline(always)]
    pub fn ssiop(&self) -> SSIOP_R {
        SSIOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SS output enable This bit is taken into account in Master mode only
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - SS output management in Master mode This bit is taken into account in Master mode when SSOE is enabled. It allows the SS output to be configured between two consecutive data transfers.
    #[inline(always)]
    pub fn ssom(&self) -> SSOM_R {
        SSOM_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - alternate function GPIOs control This bit is taken into account when SPE=0 only When SPI has to be disabled temporary for a specific configuration reason (e.g. CRC reset, CPHA or HDDIR change) setting this bit prevents any glitches on the associated outputs configured at alternate function mode by keeping them forced at state corresponding the current SPI configuration. Note: This bit can be also used in PCM and I2S modes. Note: The bit AFCNTR must not be set to 1, when the block is in slave mode.
    #[inline(always)]
    pub fn afcntr(&self) -> AFCNTR_R {
        AFCNTR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - Master SS Idleness Specifies an extra delay, expressed in number of SPI clock cycle periods, inserted additionally between active edge of SS opening a session and the beginning of the first data frame of the session in Master mode when SSOE is enabled. ... Note: This feature is not supported in TI mode. To include the delay, the SPI must be disabled and re-enabled between sessions.
    #[inline(always)]
    #[must_use]
    pub fn mssi(&mut self) -> MSSI_W<0> {
        MSSI_W::new(self)
    }
    ///Bits 4:7 - master Inter-Data Idleness Specifies minimum time delay (expressed in SPI clock cycles periods) inserted between two consecutive data frames in Master mode. ... Note: This feature is not supported in TI mode.
    #[inline(always)]
    #[must_use]
    pub fn midi(&mut self) -> MIDI_W<4> {
        MIDI_W::new(self)
    }
    ///Bit 13 - RDY signal input/output management Note: When DSIZE at the SPI_CFG1 register is configured shorter than 8-bit, the RDIOM bit has to be kept at zero.
    #[inline(always)]
    #[must_use]
    pub fn rdiom(&mut self) -> RDIOM_W<13> {
        RDIOM_W::new(self)
    }
    ///Bit 14 - RDY signal input/output polarity
    #[inline(always)]
    #[must_use]
    pub fn rdiop(&mut self) -> RDIOP_W<14> {
        RDIOP_W::new(self)
    }
    ///Bit 15 - swap functionality of MISO and MOSI pins When this bit is set, the function of MISO and MOSI pins alternate functions are inverted. Original MISO pin becomes MOSI and original MOSI pin becomes MISO. Note: This bit can be also used in PCM and I2S modes to swap SDO and SDI pins.
    #[inline(always)]
    #[must_use]
    pub fn ioswp(&mut self) -> IOSWP_W<15> {
        IOSWP_W::new(self)
    }
    ///Bits 17:18 - SPI Communication Mode
    #[inline(always)]
    #[must_use]
    pub fn comm(&mut self) -> COMM_W<17> {
        COMM_W::new(self)
    }
    ///Bits 19:21 - serial protocol others: reserved, must not be used
    #[inline(always)]
    #[must_use]
    pub fn sp(&mut self) -> SP_W<19> {
        SP_W::new(self)
    }
    ///Bit 22 - SPI Master
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MASTER_W<22> {
        MASTER_W::new(self)
    }
    ///Bit 23 - data frame format Note: This bit can be also used in PCM and I2S modes.
    #[inline(always)]
    #[must_use]
    pub fn lsbfrst(&mut self) -> LSBFRST_W<23> {
        LSBFRST_W::new(self)
    }
    ///Bit 24 - clock phase
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<24> {
        CPHA_W::new(self)
    }
    ///Bit 25 - clock polarity
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<25> {
        CPOL_W::new(self)
    }
    ///Bit 26 - software management of SS signal input When master uses hardware SS output (SSM=0 and SSOE=1) the SS signal input is forced to not active state internally to prevent master mode fault error.
    #[inline(always)]
    #[must_use]
    pub fn ssm(&mut self) -> SSM_W<26> {
        SSM_W::new(self)
    }
    ///Bit 28 - SS input/output polarity
    #[inline(always)]
    #[must_use]
    pub fn ssiop(&mut self) -> SSIOP_W<28> {
        SSIOP_W::new(self)
    }
    ///Bit 29 - SS output enable This bit is taken into account in Master mode only
    #[inline(always)]
    #[must_use]
    pub fn ssoe(&mut self) -> SSOE_W<29> {
        SSOE_W::new(self)
    }
    ///Bit 30 - SS output management in Master mode This bit is taken into account in Master mode when SSOE is enabled. It allows the SS output to be configured between two consecutive data transfers.
    #[inline(always)]
    #[must_use]
    pub fn ssom(&mut self) -> SSOM_W<30> {
        SSOM_W::new(self)
    }
    ///Bit 31 - alternate function GPIOs control This bit is taken into account when SPE=0 only When SPI has to be disabled temporary for a specific configuration reason (e.g. CRC reset, CPHA or HDDIR change) setting this bit prevents any glitches on the associated outputs configured at alternate function mode by keeping them forced at state corresponding the current SPI configuration. Note: This bit can be also used in PCM and I2S modes. Note: The bit AFCNTR must not be set to 1, when the block is in slave mode.
    #[inline(always)]
    #[must_use]
    pub fn afcntr(&mut self) -> AFCNTR_W<31> {
        AFCNTR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SPI/I2S configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfg2](index.html) module
pub struct CFG2_SPEC;
impl crate::RegisterSpec for CFG2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfg2::R](R) reader structure
impl crate::Readable for CFG2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfg2::W](W) writer structure
impl crate::Writable for CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CFG2 to value 0
impl crate::Resettable for CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
