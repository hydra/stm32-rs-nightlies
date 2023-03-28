///Register `UCPD_TX_ORDSETR` reader
pub struct R(crate::R<UCPD_TX_ORDSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCPD_TX_ORDSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCPD_TX_ORDSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCPD_TX_ORDSETR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `UCPD_TX_ORDSETR` writer
pub struct W(crate::W<UCPD_TX_ORDSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCPD_TX_ORDSETR_SPEC>;
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
impl From<crate::W<UCPD_TX_ORDSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCPD_TX_ORDSETR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TXORDSET` reader - Ordered set to transmit The bitfield determines a full 20-bit sequence to transmit, consisting of four K-codes, each of five bits, defining the packet to transmit. The bit 0 (bit 0 of K-code1) is the first, the bit 19 (bit 4 of K‑code4) the last.
pub type TXORDSET_R = crate::FieldReader<u32, u32>;
///Field `TXORDSET` writer - Ordered set to transmit The bitfield determines a full 20-bit sequence to transmit, consisting of four K-codes, each of five bits, defining the packet to transmit. The bit 0 (bit 0 of K-code1) is the first, the bit 19 (bit 4 of K‑code4) the last.
pub type TXORDSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UCPD_TX_ORDSETR_SPEC, u32, u32, 20, O>;
impl R {
    ///Bits 0:19 - Ordered set to transmit The bitfield determines a full 20-bit sequence to transmit, consisting of four K-codes, each of five bits, defining the packet to transmit. The bit 0 (bit 0 of K-code1) is the first, the bit 19 (bit 4 of K‑code4) the last.
    #[inline(always)]
    pub fn txordset(&self) -> TXORDSET_R {
        TXORDSET_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    ///Bits 0:19 - Ordered set to transmit The bitfield determines a full 20-bit sequence to transmit, consisting of four K-codes, each of five bits, defining the packet to transmit. The bit 0 (bit 0 of K-code1) is the first, the bit 19 (bit 4 of K‑code4) the last.
    #[inline(always)]
    #[must_use]
    pub fn txordset(&mut self) -> TXORDSET_W<0> {
        TXORDSET_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///UCPD Tx ordered set type register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ucpd_tx_ordsetr](index.html) module
pub struct UCPD_TX_ORDSETR_SPEC;
impl crate::RegisterSpec for UCPD_TX_ORDSETR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ucpd_tx_ordsetr::R](R) reader structure
impl crate::Readable for UCPD_TX_ORDSETR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ucpd_tx_ordsetr::W](W) writer structure
impl crate::Writable for UCPD_TX_ORDSETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets UCPD_TX_ORDSETR to value 0
impl crate::Resettable for UCPD_TX_ORDSETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
