///Register `ETH_DMAC0RxCR` reader
pub struct R(crate::R<ETH_DMAC0RX_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC0RX_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC0RX_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC0RX_CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ETH_DMAC0RxCR` writer
pub struct W(crate::W<ETH_DMAC0RX_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMAC0RX_CR_SPEC>;
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
impl From<crate::W<ETH_DMAC0RX_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMAC0RX_CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SR` reader - Start or Stop Receive Command
pub type SR_R = crate::BitReader<bool>;
///Field `SR` writer - Start or Stop Receive Command
pub type SR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC0RX_CR_SPEC, bool, O>;
///Field `RBSZ` reader - Receive Buffer size
pub type RBSZ_R = crate::FieldReader<u16, u16>;
///Field `RBSZ` writer - Receive Buffer size
pub type RBSZ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_DMAC0RX_CR_SPEC, u16, u16, 14, O>;
///Field `RXPBL` reader - RXPBL
pub type RXPBL_R = crate::FieldReader<u8, u8>;
///Field `RXPBL` writer - RXPBL
pub type RXPBL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAC0RX_CR_SPEC, u8, u8, 6, O>;
///Field `RQOS` reader - RQOS
pub type RQOS_R = crate::FieldReader<u8, u8>;
///Field `RQOS` writer - RQOS
pub type RQOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAC0RX_CR_SPEC, u8, u8, 4, O>;
///Field `RPF` reader - DMA Rx Channel Packet Flush
pub type RPF_R = crate::BitReader<bool>;
///Field `RPF` writer - DMA Rx Channel Packet Flush
pub type RPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC0RX_CR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Start or Stop Receive Command
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:14 - Receive Buffer size
    #[inline(always)]
    pub fn rbsz(&self) -> RBSZ_R {
        RBSZ_R::new(((self.bits >> 1) & 0x3fff) as u16)
    }
    ///Bits 16:21 - RXPBL
    #[inline(always)]
    pub fn rxpbl(&self) -> RXPBL_R {
        RXPBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:27 - RQOS
    #[inline(always)]
    pub fn rqos(&self) -> RQOS_R {
        RQOS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bit 31 - DMA Rx Channel Packet Flush
    #[inline(always)]
    pub fn rpf(&self) -> RPF_R {
        RPF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Start or Stop Receive Command
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SR_W<0> {
        SR_W::new(self)
    }
    ///Bits 1:14 - Receive Buffer size
    #[inline(always)]
    #[must_use]
    pub fn rbsz(&mut self) -> RBSZ_W<1> {
        RBSZ_W::new(self)
    }
    ///Bits 16:21 - RXPBL
    #[inline(always)]
    #[must_use]
    pub fn rxpbl(&mut self) -> RXPBL_W<16> {
        RXPBL_W::new(self)
    }
    ///Bits 24:27 - RQOS
    #[inline(always)]
    #[must_use]
    pub fn rqos(&mut self) -> RQOS_W<24> {
        RQOS_W::new(self)
    }
    ///Bit 31 - DMA Rx Channel Packet Flush
    #[inline(always)]
    #[must_use]
    pub fn rpf(&mut self) -> RPF_W<31> {
        RPF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Channel receive control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_dmac0rx_cr](index.html) module
pub struct ETH_DMAC0RX_CR_SPEC;
impl crate::RegisterSpec for ETH_DMAC0RX_CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_dmac0rx_cr::R](R) reader structure
impl crate::Readable for ETH_DMAC0RX_CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [eth_dmac0rx_cr::W](W) writer structure
impl crate::Writable for ETH_DMAC0RX_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ETH_DMAC0RxCR to value 0x8000
impl crate::Resettable for ETH_DMAC0RX_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
