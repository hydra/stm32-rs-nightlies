///Register `ETH_MTLTxQ1OMR` reader
pub struct R(crate::R<ETH_MTLTX_Q1OMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLTX_Q1OMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLTX_Q1OMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLTX_Q1OMR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ETH_MTLTxQ1OMR` writer
pub struct W(crate::W<ETH_MTLTX_Q1OMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MTLTX_Q1OMR_SPEC>;
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
impl From<crate::W<ETH_MTLTX_Q1OMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MTLTX_Q1OMR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FTQ` reader - FTQ
pub type FTQ_R = crate::BitReader<bool>;
///Field `FTQ` writer - FTQ
pub type FTQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLTX_Q1OMR_SPEC, bool, O>;
///Field `TSF` reader - TSF
pub type TSF_R = crate::BitReader<bool>;
///Field `TSF` writer - TSF
pub type TSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLTX_Q1OMR_SPEC, bool, O>;
///Field `TXQEN` reader - TXQEN
pub type TXQEN_R = crate::FieldReader<u8, u8>;
///Field `TXQEN` writer - TXQEN
pub type TXQEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MTLTX_Q1OMR_SPEC, u8, u8, 2, O>;
///Field `TTC` reader - TTC
pub type TTC_R = crate::FieldReader<u8, u8>;
///Field `TTC` writer - TTC
pub type TTC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MTLTX_Q1OMR_SPEC, u8, u8, 2, O>;
///Field `TQS` reader - TQS
pub type TQS_R = crate::FieldReader<u16, u16>;
///Field `TQS` writer - TQS
pub type TQS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MTLTX_Q1OMR_SPEC, u16, u16, 9, O>;
impl R {
    ///Bit 0 - FTQ
    #[inline(always)]
    pub fn ftq(&self) -> FTQ_R {
        FTQ_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TSF
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - TXQEN
    #[inline(always)]
    pub fn txqen(&self) -> TXQEN_R {
        TXQEN_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - TTC
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 16:24 - TQS
    #[inline(always)]
    pub fn tqs(&self) -> TQS_R {
        TQS_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    ///Bit 0 - FTQ
    #[inline(always)]
    #[must_use]
    pub fn ftq(&mut self) -> FTQ_W<0> {
        FTQ_W::new(self)
    }
    ///Bit 1 - TSF
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TSF_W<1> {
        TSF_W::new(self)
    }
    ///Bits 2:3 - TXQEN
    #[inline(always)]
    #[must_use]
    pub fn txqen(&mut self) -> TXQEN_W<2> {
        TXQEN_W::new(self)
    }
    ///Bits 4:5 - TTC
    #[inline(always)]
    #[must_use]
    pub fn ttc(&mut self) -> TTC_W<4> {
        TTC_W::new(self)
    }
    ///Bits 16:24 - TQS
    #[inline(always)]
    #[must_use]
    pub fn tqs(&mut self) -> TQS_W<16> {
        TQS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Tx queue 1 operating mode Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_mtltx_q1omr](index.html) module
pub struct ETH_MTLTX_Q1OMR_SPEC;
impl crate::RegisterSpec for ETH_MTLTX_Q1OMR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_mtltx_q1omr::R](R) reader structure
impl crate::Readable for ETH_MTLTX_Q1OMR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [eth_mtltx_q1omr::W](W) writer structure
impl crate::Writable for ETH_MTLTX_Q1OMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ETH_MTLTxQ1OMR to value 0
impl crate::Resettable for ETH_MTLTX_Q1OMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
