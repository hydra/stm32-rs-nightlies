///Register `MMC_RX_INTERRUPT_MASK` reader
pub struct R(crate::R<MMC_RX_INTERRUPT_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_RX_INTERRUPT_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_RX_INTERRUPT_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_RX_INTERRUPT_MASK_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MMC_RX_INTERRUPT_MASK` writer
pub struct W(crate::W<MMC_RX_INTERRUPT_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMC_RX_INTERRUPT_MASK_SPEC>;
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
impl From<crate::W<MMC_RX_INTERRUPT_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMC_RX_INTERRUPT_MASK_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RXCRCERPIM` reader - MMC Receive CRC Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the Rx CRC error packets register (ETH_RX_CRC_ERROR_PACKETS) counter reaches half of the maximum value or the maximum value.
pub type RXCRCERPIM_R = crate::BitReader<bool>;
///Field `RXCRCERPIM` writer - MMC Receive CRC Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the Rx CRC error packets register (ETH_RX_CRC_ERROR_PACKETS) counter reaches half of the maximum value or the maximum value.
pub type RXCRCERPIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_RX_INTERRUPT_MASK_SPEC, bool, O>;
///Field `RXALGNERPIM` reader - MMC Receive Alignment Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the Rx alignment error packets register (ETH_RX_ALIGNMENT_ERROR_PACKETS) counter reaches half of the maximum value or the maximum value.
pub type RXALGNERPIM_R = crate::BitReader<bool>;
///Field `RXALGNERPIM` writer - MMC Receive Alignment Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the Rx alignment error packets register (ETH_RX_ALIGNMENT_ERROR_PACKETS) counter reaches half of the maximum value or the maximum value.
pub type RXALGNERPIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_RX_INTERRUPT_MASK_SPEC, bool, O>;
///Field `RXUCGPIM` reader - MMC Receive Unicast Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the Rx unicast packets good register (ETH_RX_UNICAST_PACKETS_GOOD) counter reaches half of the maximum value or the maximum value.
pub type RXUCGPIM_R = crate::BitReader<bool>;
///Field `RXUCGPIM` writer - MMC Receive Unicast Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the Rx unicast packets good register (ETH_RX_UNICAST_PACKETS_GOOD) counter reaches half of the maximum value or the maximum value.
pub type RXUCGPIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_RX_INTERRUPT_MASK_SPEC, bool, O>;
///Field `RXLPIUSCIM` reader - MMC Receive LPI microsecond counter interrupt Mask Setting this bit masks the interrupt when the Rx LPI microsecond counter register (ETH_RX_LPI_USEC_CNTR) counter reaches half of the maximum value or the maximum value.
pub type RXLPIUSCIM_R = crate::BitReader<bool>;
///Field `RXLPIUSCIM` writer - MMC Receive LPI microsecond counter interrupt Mask Setting this bit masks the interrupt when the Rx LPI microsecond counter register (ETH_RX_LPI_USEC_CNTR) counter reaches half of the maximum value or the maximum value.
pub type RXLPIUSCIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_RX_INTERRUPT_MASK_SPEC, bool, O>;
///Field `RXLPITRCIM` reader - MMC Receive LPI transition counter interrupt Mask Setting this bit masks the interrupt when the Rx LPI transition counter register (ETH_RX_LPI_TRAN_CNTR) counter reaches half of the maximum value or the maximum value.
pub type RXLPITRCIM_R = crate::BitReader<bool>;
///Field `RXLPITRCIM` writer - MMC Receive LPI transition counter interrupt Mask Setting this bit masks the interrupt when the Rx LPI transition counter register (ETH_RX_LPI_TRAN_CNTR) counter reaches half of the maximum value or the maximum value.
pub type RXLPITRCIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_RX_INTERRUPT_MASK_SPEC, bool, O>;
impl R {
    ///Bit 5 - MMC Receive CRC Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the Rx CRC error packets register (ETH_RX_CRC_ERROR_PACKETS) counter reaches half of the maximum value or the maximum value.
    #[inline(always)]
    pub fn rxcrcerpim(&self) -> RXCRCERPIM_R {
        RXCRCERPIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MMC Receive Alignment Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the Rx alignment error packets register (ETH_RX_ALIGNMENT_ERROR_PACKETS) counter reaches half of the maximum value or the maximum value.
    #[inline(always)]
    pub fn rxalgnerpim(&self) -> RXALGNERPIM_R {
        RXALGNERPIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 17 - MMC Receive Unicast Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the Rx unicast packets good register (ETH_RX_UNICAST_PACKETS_GOOD) counter reaches half of the maximum value or the maximum value.
    #[inline(always)]
    pub fn rxucgpim(&self) -> RXUCGPIM_R {
        RXUCGPIM_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 26 - MMC Receive LPI microsecond counter interrupt Mask Setting this bit masks the interrupt when the Rx LPI microsecond counter register (ETH_RX_LPI_USEC_CNTR) counter reaches half of the maximum value or the maximum value.
    #[inline(always)]
    pub fn rxlpiuscim(&self) -> RXLPIUSCIM_R {
        RXLPIUSCIM_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - MMC Receive LPI transition counter interrupt Mask Setting this bit masks the interrupt when the Rx LPI transition counter register (ETH_RX_LPI_TRAN_CNTR) counter reaches half of the maximum value or the maximum value.
    #[inline(always)]
    pub fn rxlpitrcim(&self) -> RXLPITRCIM_R {
        RXLPITRCIM_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    ///Bit 5 - MMC Receive CRC Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the Rx CRC error packets register (ETH_RX_CRC_ERROR_PACKETS) counter reaches half of the maximum value or the maximum value.
    #[inline(always)]
    #[must_use]
    pub fn rxcrcerpim(&mut self) -> RXCRCERPIM_W<5> {
        RXCRCERPIM_W::new(self)
    }
    ///Bit 6 - MMC Receive Alignment Error Packet Counter Interrupt Mask Setting this bit masks the interrupt when the Rx alignment error packets register (ETH_RX_ALIGNMENT_ERROR_PACKETS) counter reaches half of the maximum value or the maximum value.
    #[inline(always)]
    #[must_use]
    pub fn rxalgnerpim(&mut self) -> RXALGNERPIM_W<6> {
        RXALGNERPIM_W::new(self)
    }
    ///Bit 17 - MMC Receive Unicast Good Packet Counter Interrupt Mask Setting this bit masks the interrupt when the Rx unicast packets good register (ETH_RX_UNICAST_PACKETS_GOOD) counter reaches half of the maximum value or the maximum value.
    #[inline(always)]
    #[must_use]
    pub fn rxucgpim(&mut self) -> RXUCGPIM_W<17> {
        RXUCGPIM_W::new(self)
    }
    ///Bit 26 - MMC Receive LPI microsecond counter interrupt Mask Setting this bit masks the interrupt when the Rx LPI microsecond counter register (ETH_RX_LPI_USEC_CNTR) counter reaches half of the maximum value or the maximum value.
    #[inline(always)]
    #[must_use]
    pub fn rxlpiuscim(&mut self) -> RXLPIUSCIM_W<26> {
        RXLPIUSCIM_W::new(self)
    }
    ///Bit 27 - MMC Receive LPI transition counter interrupt Mask Setting this bit masks the interrupt when the Rx LPI transition counter register (ETH_RX_LPI_TRAN_CNTR) counter reaches half of the maximum value or the maximum value.
    #[inline(always)]
    #[must_use]
    pub fn rxlpitrcim(&mut self) -> RXLPITRCIM_W<27> {
        RXLPITRCIM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MMC Rx interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mmc_rx_interrupt_mask](index.html) module
pub struct MMC_RX_INTERRUPT_MASK_SPEC;
impl crate::RegisterSpec for MMC_RX_INTERRUPT_MASK_SPEC {
    type Ux = u32;
}
///`read()` method returns [mmc_rx_interrupt_mask::R](R) reader structure
impl crate::Readable for MMC_RX_INTERRUPT_MASK_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mmc_rx_interrupt_mask::W](W) writer structure
impl crate::Writable for MMC_RX_INTERRUPT_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MMC_RX_INTERRUPT_MASK to value 0
impl crate::Resettable for MMC_RX_INTERRUPT_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
