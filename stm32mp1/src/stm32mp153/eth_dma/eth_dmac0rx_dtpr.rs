///Register `ETH_DMAC0RxDTPR` reader
pub struct R(crate::R<ETH_DMAC0RX_DTPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC0RX_DTPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC0RX_DTPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC0RX_DTPR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ETH_DMAC0RxDTPR` writer
pub struct W(crate::W<ETH_DMAC0RX_DTPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMAC0RX_DTPR_SPEC>;
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
impl From<crate::W<ETH_DMAC0RX_DTPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMAC0RX_DTPR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RDT` reader - Receive Descriptor Tail Pointer
pub type RDT_R = crate::FieldReader<u32, u32>;
///Field `RDT` writer - Receive Descriptor Tail Pointer
pub type RDT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_DMAC0RX_DTPR_SPEC, u32, u32, 29, O>;
impl R {
    ///Bits 3:31 - Receive Descriptor Tail Pointer
    #[inline(always)]
    pub fn rdt(&self) -> RDT_R {
        RDT_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    ///Bits 3:31 - Receive Descriptor Tail Pointer
    #[inline(always)]
    #[must_use]
    pub fn rdt(&mut self) -> RDT_W<3> {
        RDT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Channel Rx descriptor tail pointer register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_dmac0rx_dtpr](index.html) module
pub struct ETH_DMAC0RX_DTPR_SPEC;
impl crate::RegisterSpec for ETH_DMAC0RX_DTPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_dmac0rx_dtpr::R](R) reader structure
impl crate::Readable for ETH_DMAC0RX_DTPR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [eth_dmac0rx_dtpr::W](W) writer structure
impl crate::Writable for ETH_DMAC0RX_DTPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ETH_DMAC0RxDTPR to value 0
impl crate::Resettable for ETH_DMAC0RX_DTPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
