///Register `FDCAN_TXBC` reader
pub struct R(crate::R<FDCAN_TXBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXBC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXBC_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FDCAN_TXBC` writer
pub struct W(crate::W<FDCAN_TXBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TXBC_SPEC>;
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
impl From<crate::W<FDCAN_TXBC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TXBC_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TBSA` reader - Tx Buffers Start Address
pub type TBSA_R = crate::FieldReader<u16, u16>;
///Field `TBSA` writer - Tx Buffers Start Address
pub type TBSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TXBC_SPEC, u16, u16, 14, O>;
///Field `NDTB` reader - Number of Dedicated Transmit Buffers
pub type NDTB_R = crate::FieldReader<u8, u8>;
///Field `NDTB` writer - Number of Dedicated Transmit Buffers
pub type NDTB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TXBC_SPEC, u8, u8, 6, O>;
///Field `TFQS` reader - Transmit FIFO/Queue Size
pub type TFQS_R = crate::FieldReader<u8, u8>;
///Field `TFQS` writer - Transmit FIFO/Queue Size
pub type TFQS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TXBC_SPEC, u8, u8, 6, O>;
///Field `TFQM` reader - Tx FIFO/Queue Mode
pub type TFQM_R = crate::BitReader<bool>;
///Field `TFQM` writer - Tx FIFO/Queue Mode
pub type TFQM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TXBC_SPEC, bool, O>;
impl R {
    ///Bits 2:15 - Tx Buffers Start Address
    #[inline(always)]
    pub fn tbsa(&self) -> TBSA_R {
        TBSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    ///Bits 16:21 - Number of Dedicated Transmit Buffers
    #[inline(always)]
    pub fn ndtb(&self) -> NDTB_R {
        NDTB_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:29 - Transmit FIFO/Queue Size
    #[inline(always)]
    pub fn tfqs(&self) -> TFQS_R {
        TFQS_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    ///Bit 30 - Tx FIFO/Queue Mode
    #[inline(always)]
    pub fn tfqm(&self) -> TFQM_R {
        TFQM_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bits 2:15 - Tx Buffers Start Address
    #[inline(always)]
    #[must_use]
    pub fn tbsa(&mut self) -> TBSA_W<2> {
        TBSA_W::new(self)
    }
    ///Bits 16:21 - Number of Dedicated Transmit Buffers
    #[inline(always)]
    #[must_use]
    pub fn ndtb(&mut self) -> NDTB_W<16> {
        NDTB_W::new(self)
    }
    ///Bits 24:29 - Transmit FIFO/Queue Size
    #[inline(always)]
    #[must_use]
    pub fn tfqs(&mut self) -> TFQS_W<24> {
        TFQS_W::new(self)
    }
    ///Bit 30 - Tx FIFO/Queue Mode
    #[inline(always)]
    #[must_use]
    pub fn tfqm(&mut self) -> TFQM_W<30> {
        TFQM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Tx Buffer Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_txbc](index.html) module
pub struct FDCAN_TXBC_SPEC;
impl crate::RegisterSpec for FDCAN_TXBC_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_txbc::R](R) reader structure
impl crate::Readable for FDCAN_TXBC_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fdcan_txbc::W](W) writer structure
impl crate::Writable for FDCAN_TXBC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FDCAN_TXBC to value 0
impl crate::Resettable for FDCAN_TXBC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
