///Register `ETH_DMAA4TxACR` reader
pub struct R(crate::R<ETH_DMAA4TX_ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAA4TX_ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAA4TX_ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAA4TX_ACR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ETH_DMAA4TxACR` writer
pub struct W(crate::W<ETH_DMAA4TX_ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMAA4TX_ACR_SPEC>;
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
impl From<crate::W<ETH_DMAA4TX_ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMAA4TX_ACR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TDRC` reader - TDRC
pub type TDRC_R = crate::FieldReader<u8, u8>;
///Field `TDRC` writer - TDRC
pub type TDRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAA4TX_ACR_SPEC, u8, u8, 4, O>;
///Field `TEC` reader - TEC
pub type TEC_R = crate::FieldReader<u8, u8>;
///Field `TEC` writer - TEC
pub type TEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAA4TX_ACR_SPEC, u8, u8, 4, O>;
///Field `THC` reader - THC
pub type THC_R = crate::FieldReader<u8, u8>;
///Field `THC` writer - THC
pub type THC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAA4TX_ACR_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:3 - TDRC
    #[inline(always)]
    pub fn tdrc(&self) -> TDRC_R {
        TDRC_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - TEC
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:19 - THC
    #[inline(always)]
    pub fn thc(&self) -> THC_R {
        THC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - TDRC
    #[inline(always)]
    #[must_use]
    pub fn tdrc(&mut self) -> TDRC_W<0> {
        TDRC_W::new(self)
    }
    ///Bits 8:11 - TEC
    #[inline(always)]
    #[must_use]
    pub fn tec(&mut self) -> TEC_W<8> {
        TEC_W::new(self)
    }
    ///Bits 16:19 - THC
    #[inline(always)]
    #[must_use]
    pub fn thc(&mut self) -> THC_W<16> {
        THC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AXI4 transmit channel ACE control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_dmaa4tx_acr](index.html) module
pub struct ETH_DMAA4TX_ACR_SPEC;
impl crate::RegisterSpec for ETH_DMAA4TX_ACR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_dmaa4tx_acr::R](R) reader structure
impl crate::Readable for ETH_DMAA4TX_ACR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [eth_dmaa4tx_acr::W](W) writer structure
impl crate::Writable for ETH_DMAA4TX_ACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ETH_DMAA4TxACR to value 0
impl crate::Resettable for ETH_DMAA4TX_ACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
