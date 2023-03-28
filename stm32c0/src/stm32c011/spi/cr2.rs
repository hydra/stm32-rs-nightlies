///Register `CR2` reader
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR2` writer
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RXDMAEN` reader - Rx buffer DMA enable When this bit is set, a DMA request is generated whenever the RXNE flag is set.
pub type RXDMAEN_R = crate::BitReader<bool>;
///Field `RXDMAEN` writer - Rx buffer DMA enable When this bit is set, a DMA request is generated whenever the RXNE flag is set.
pub type RXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CR2_SPEC, bool, O>;
///Field `TXDMAEN` reader - Tx buffer DMA enable When this bit is set, a DMA request is generated whenever the TXE flag is set.
pub type TXDMAEN_R = crate::BitReader<bool>;
///Field `TXDMAEN` writer - Tx buffer DMA enable When this bit is set, a DMA request is generated whenever the TXE flag is set.
pub type TXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CR2_SPEC, bool, O>;
///Field `SSOE` reader - SS output enable Note: This bit is not used in I2S mode and SPI TI mode.
pub type SSOE_R = crate::BitReader<bool>;
///Field `SSOE` writer - SS output enable Note: This bit is not used in I2S mode and SPI TI mode.
pub type SSOE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CR2_SPEC, bool, O>;
///Field `NSSP` reader - NSS pulse management This bit is used in master mode only. it allows the SPI to generate an NSS pulse between two consecutive data when doing continuous transfers. In the case of a single data transfer, it forces the NSS pin high level after the transfer. It has no meaning if CPHA = ’1’, or FRF = ’1’. Note: 1. This bit must be written only when the SPI is disabled (SPE=0). 2. This bit is not used in I2S mode and SPI TI mode.
pub type NSSP_R = crate::BitReader<bool>;
///Field `NSSP` writer - NSS pulse management This bit is used in master mode only. it allows the SPI to generate an NSS pulse between two consecutive data when doing continuous transfers. In the case of a single data transfer, it forces the NSS pin high level after the transfer. It has no meaning if CPHA = ’1’, or FRF = ’1’. Note: 1. This bit must be written only when the SPI is disabled (SPE=0). 2. This bit is not used in I2S mode and SPI TI mode.
pub type NSSP_W<'a, const O: u8> = crate::BitWriter<'a, u16, CR2_SPEC, bool, O>;
///Field `FRF` reader - Frame format 1 SPI TI mode Note: This bit must be written only when the SPI is disabled (SPE=0). This bit is not used in I2S mode.
pub type FRF_R = crate::BitReader<bool>;
///Field `FRF` writer - Frame format 1 SPI TI mode Note: This bit must be written only when the SPI is disabled (SPE=0). This bit is not used in I2S mode.
pub type FRF_W<'a, const O: u8> = crate::BitWriter<'a, u16, CR2_SPEC, bool, O>;
///Field `ERRIE` reader - Error interrupt enable This bit controls the generation of an interrupt when an error condition occurs (CRCERR, OVR, MODF in SPI mode, FRE at TI mode and UDR, OVR, and FRE in I2S mode).
pub type ERRIE_R = crate::BitReader<bool>;
///Field `ERRIE` writer - Error interrupt enable This bit controls the generation of an interrupt when an error condition occurs (CRCERR, OVR, MODF in SPI mode, FRE at TI mode and UDR, OVR, and FRE in I2S mode).
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CR2_SPEC, bool, O>;
///Field `RXNEIE` reader - RX buffer not empty interrupt enable
pub type RXNEIE_R = crate::BitReader<bool>;
///Field `RXNEIE` writer - RX buffer not empty interrupt enable
pub type RXNEIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CR2_SPEC, bool, O>;
///Field `TXEIE` reader - Tx buffer empty interrupt enable
pub type TXEIE_R = crate::BitReader<bool>;
///Field `TXEIE` writer - Tx buffer empty interrupt enable
pub type TXEIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CR2_SPEC, bool, O>;
///Field `DS` reader - Data size These bits configure the data length for SPI transfers. If software attempts to write one of the “Not used” values, they are forced to the value “0111” (8-bit) Note: These bits are not used in I2S mode.
pub type DS_R = crate::FieldReader<u8, u8>;
///Field `DS` writer - Data size These bits configure the data length for SPI transfers. If software attempts to write one of the “Not used” values, they are forced to the value “0111” (8-bit) Note: These bits are not used in I2S mode.
pub type DS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CR2_SPEC, u8, u8, 4, O>;
///Field `FRXTH` reader - FIFO reception threshold This bit is used to set the threshold of the RXFIFO that triggers an RXNE event Note: This bit is not used in I2S mode.
pub type FRXTH_R = crate::BitReader<bool>;
///Field `FRXTH` writer - FIFO reception threshold This bit is used to set the threshold of the RXFIFO that triggers an RXNE event Note: This bit is not used in I2S mode.
pub type FRXTH_W<'a, const O: u8> = crate::BitWriter<'a, u16, CR2_SPEC, bool, O>;
///Field `LDMA_RX` reader - Last DMA transfer for reception This bit is used in data packing mode, to define if the total number of data to receive by DMA is odd or even. It has significance only if the RXDMAEN bit in the SPI_CR2 register is set and if packing mode is used (data length =&lt; 8-bit and write access to SPI_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPI_CR1 register). Note: Refer to if the CRCEN bit is set. This bit is not used in I�S mode.
pub type LDMA_RX_R = crate::BitReader<bool>;
///Field `LDMA_RX` writer - Last DMA transfer for reception This bit is used in data packing mode, to define if the total number of data to receive by DMA is odd or even. It has significance only if the RXDMAEN bit in the SPI_CR2 register is set and if packing mode is used (data length =&lt; 8-bit and write access to SPI_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPI_CR1 register). Note: Refer to if the CRCEN bit is set. This bit is not used in I�S mode.
pub type LDMA_RX_W<'a, const O: u8> = crate::BitWriter<'a, u16, CR2_SPEC, bool, O>;
///Field `LDMA_TX` reader - Last DMA transfer for transmission This bit is used in data packing mode, to define if the total number of data to transmit by DMA is odd or even. It has significance only if the TXDMAEN bit in the SPI_CR2 register is set and if packing mode is used (data length =&lt; 8-bit and write access to SPI_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPI_CR1 register). Note: Refer to if the CRCEN bit is set. This bit is not used in I�S mode.
pub type LDMA_TX_R = crate::BitReader<bool>;
///Field `LDMA_TX` writer - Last DMA transfer for transmission This bit is used in data packing mode, to define if the total number of data to transmit by DMA is odd or even. It has significance only if the TXDMAEN bit in the SPI_CR2 register is set and if packing mode is used (data length =&lt; 8-bit and write access to SPI_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPI_CR1 register). Note: Refer to if the CRCEN bit is set. This bit is not used in I�S mode.
pub type LDMA_TX_W<'a, const O: u8> = crate::BitWriter<'a, u16, CR2_SPEC, bool, O>;
impl R {
    ///Bit 0 - Rx buffer DMA enable When this bit is set, a DMA request is generated whenever the RXNE flag is set.
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Tx buffer DMA enable When this bit is set, a DMA request is generated whenever the TXE flag is set.
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SS output enable Note: This bit is not used in I2S mode and SPI TI mode.
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - NSS pulse management This bit is used in master mode only. it allows the SPI to generate an NSS pulse between two consecutive data when doing continuous transfers. In the case of a single data transfer, it forces the NSS pin high level after the transfer. It has no meaning if CPHA = ’1’, or FRF = ’1’. Note: 1. This bit must be written only when the SPI is disabled (SPE=0). 2. This bit is not used in I2S mode and SPI TI mode.
    #[inline(always)]
    pub fn nssp(&self) -> NSSP_R {
        NSSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Frame format 1 SPI TI mode Note: This bit must be written only when the SPI is disabled (SPE=0). This bit is not used in I2S mode.
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Error interrupt enable This bit controls the generation of an interrupt when an error condition occurs (CRCERR, OVR, MODF in SPI mode, FRE at TI mode and UDR, OVR, and FRE in I2S mode).
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RX buffer not empty interrupt enable
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Tx buffer empty interrupt enable
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - Data size These bits configure the data length for SPI transfers. If software attempts to write one of the “Not used” values, they are forced to the value “0111” (8-bit) Note: These bits are not used in I2S mode.
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - FIFO reception threshold This bit is used to set the threshold of the RXFIFO that triggers an RXNE event Note: This bit is not used in I2S mode.
    #[inline(always)]
    pub fn frxth(&self) -> FRXTH_R {
        FRXTH_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Last DMA transfer for reception This bit is used in data packing mode, to define if the total number of data to receive by DMA is odd or even. It has significance only if the RXDMAEN bit in the SPI_CR2 register is set and if packing mode is used (data length =&lt; 8-bit and write access to SPI_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPI_CR1 register). Note: Refer to if the CRCEN bit is set. This bit is not used in I�S mode.
    #[inline(always)]
    pub fn ldma_rx(&self) -> LDMA_RX_R {
        LDMA_RX_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Last DMA transfer for transmission This bit is used in data packing mode, to define if the total number of data to transmit by DMA is odd or even. It has significance only if the TXDMAEN bit in the SPI_CR2 register is set and if packing mode is used (data length =&lt; 8-bit and write access to SPI_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPI_CR1 register). Note: Refer to if the CRCEN bit is set. This bit is not used in I�S mode.
    #[inline(always)]
    pub fn ldma_tx(&self) -> LDMA_TX_R {
        LDMA_TX_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Rx buffer DMA enable When this bit is set, a DMA request is generated whenever the RXNE flag is set.
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<0> {
        RXDMAEN_W::new(self)
    }
    ///Bit 1 - Tx buffer DMA enable When this bit is set, a DMA request is generated whenever the TXE flag is set.
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<1> {
        TXDMAEN_W::new(self)
    }
    ///Bit 2 - SS output enable Note: This bit is not used in I2S mode and SPI TI mode.
    #[inline(always)]
    #[must_use]
    pub fn ssoe(&mut self) -> SSOE_W<2> {
        SSOE_W::new(self)
    }
    ///Bit 3 - NSS pulse management This bit is used in master mode only. it allows the SPI to generate an NSS pulse between two consecutive data when doing continuous transfers. In the case of a single data transfer, it forces the NSS pin high level after the transfer. It has no meaning if CPHA = ’1’, or FRF = ’1’. Note: 1. This bit must be written only when the SPI is disabled (SPE=0). 2. This bit is not used in I2S mode and SPI TI mode.
    #[inline(always)]
    #[must_use]
    pub fn nssp(&mut self) -> NSSP_W<3> {
        NSSP_W::new(self)
    }
    ///Bit 4 - Frame format 1 SPI TI mode Note: This bit must be written only when the SPI is disabled (SPE=0). This bit is not used in I2S mode.
    #[inline(always)]
    #[must_use]
    pub fn frf(&mut self) -> FRF_W<4> {
        FRF_W::new(self)
    }
    ///Bit 5 - Error interrupt enable This bit controls the generation of an interrupt when an error condition occurs (CRCERR, OVR, MODF in SPI mode, FRE at TI mode and UDR, OVR, and FRE in I2S mode).
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<5> {
        ERRIE_W::new(self)
    }
    ///Bit 6 - RX buffer not empty interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rxneie(&mut self) -> RXNEIE_W<6> {
        RXNEIE_W::new(self)
    }
    ///Bit 7 - Tx buffer empty interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn txeie(&mut self) -> TXEIE_W<7> {
        TXEIE_W::new(self)
    }
    ///Bits 8:11 - Data size These bits configure the data length for SPI transfers. If software attempts to write one of the “Not used” values, they are forced to the value “0111” (8-bit) Note: These bits are not used in I2S mode.
    #[inline(always)]
    #[must_use]
    pub fn ds(&mut self) -> DS_W<8> {
        DS_W::new(self)
    }
    ///Bit 12 - FIFO reception threshold This bit is used to set the threshold of the RXFIFO that triggers an RXNE event Note: This bit is not used in I2S mode.
    #[inline(always)]
    #[must_use]
    pub fn frxth(&mut self) -> FRXTH_W<12> {
        FRXTH_W::new(self)
    }
    ///Bit 13 - Last DMA transfer for reception This bit is used in data packing mode, to define if the total number of data to receive by DMA is odd or even. It has significance only if the RXDMAEN bit in the SPI_CR2 register is set and if packing mode is used (data length =&lt; 8-bit and write access to SPI_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPI_CR1 register). Note: Refer to if the CRCEN bit is set. This bit is not used in I�S mode.
    #[inline(always)]
    #[must_use]
    pub fn ldma_rx(&mut self) -> LDMA_RX_W<13> {
        LDMA_RX_W::new(self)
    }
    ///Bit 14 - Last DMA transfer for transmission This bit is used in data packing mode, to define if the total number of data to transmit by DMA is odd or even. It has significance only if the TXDMAEN bit in the SPI_CR2 register is set and if packing mode is used (data length =&lt; 8-bit and write access to SPI_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPI_CR1 register). Note: Refer to if the CRCEN bit is set. This bit is not used in I�S mode.
    #[inline(always)]
    #[must_use]
    pub fn ldma_tx(&mut self) -> LDMA_TX_W<14> {
        LDMA_TX_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SPI control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr2](index.html) module
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u16;
}
///`read()` method returns [cr2::R](R) reader structure
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr2::W](W) writer structure
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR2 to value 0x0700
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0700;
}
