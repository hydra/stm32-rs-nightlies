///Register `ETH_DMAC0TxRLR` reader
pub struct R(crate::R<ETH_DMAC0TX_RLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC0TX_RLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC0TX_RLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC0TX_RLR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ETH_DMAC0TxRLR` writer
pub struct W(crate::W<ETH_DMAC0TX_RLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMAC0TX_RLR_SPEC>;
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
impl From<crate::W<ETH_DMAC0TX_RLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMAC0TX_RLR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TDRL` reader - Transmit Descriptor Ring Length
pub type TDRL_R = crate::FieldReader<u16, u16>;
///Field `TDRL` writer - Transmit Descriptor Ring Length
pub type TDRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_DMAC0TX_RLR_SPEC, u16, u16, 10, O>;
impl R {
    ///Bits 0:9 - Transmit Descriptor Ring Length
    #[inline(always)]
    pub fn tdrl(&self) -> TDRL_R {
        TDRL_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - Transmit Descriptor Ring Length
    #[inline(always)]
    #[must_use]
    pub fn tdrl(&mut self) -> TDRL_W<0> {
        TDRL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Channel Tx descriptor ring length register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_dmac0tx_rlr](index.html) module
pub struct ETH_DMAC0TX_RLR_SPEC;
impl crate::RegisterSpec for ETH_DMAC0TX_RLR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_dmac0tx_rlr::R](R) reader structure
impl crate::Readable for ETH_DMAC0TX_RLR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [eth_dmac0tx_rlr::W](W) writer structure
impl crate::Writable for ETH_DMAC0TX_RLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ETH_DMAC0TxRLR to value 0
impl crate::Resettable for ETH_DMAC0TX_RLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
