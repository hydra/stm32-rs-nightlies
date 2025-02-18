///Register `CEC_IER` reader
pub struct R(crate::R<CEC_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEC_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEC_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEC_IER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CEC_IER` writer
pub struct W(crate::W<CEC_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEC_IER_SPEC>;
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
impl From<crate::W<CEC_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEC_IER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RXBRIE` reader - Rx-Byte Received Interrupt Enable The RXBRIE bit is set and cleared by software.
pub type RXBRIE_R = crate::BitReader<bool>;
///Field `RXBRIE` writer - Rx-Byte Received Interrupt Enable The RXBRIE bit is set and cleared by software.
pub type RXBRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
///Field `RXENDIE` reader - End Of Reception Interrupt Enable The RXENDIE bit is set and cleared by software.
pub type RXENDIE_R = crate::BitReader<bool>;
///Field `RXENDIE` writer - End Of Reception Interrupt Enable The RXENDIE bit is set and cleared by software.
pub type RXENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
///Field `RXOVRIE` reader - Rx-Buffer Overrun Interrupt Enable The RXOVRIE bit is set and cleared by software.
pub type RXOVRIE_R = crate::BitReader<bool>;
///Field `RXOVRIE` writer - Rx-Buffer Overrun Interrupt Enable The RXOVRIE bit is set and cleared by software.
pub type RXOVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
///Field `BREIE` reader - Bit Rising Error Interrupt Enable The BREIE bit is set and cleared by software.
pub type BREIE_R = crate::BitReader<bool>;
///Field `BREIE` writer - Bit Rising Error Interrupt Enable The BREIE bit is set and cleared by software.
pub type BREIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
///Field `SBPEIE` reader - Short Bit Period Error Interrupt Enable The SBPEIE bit is set and cleared by software.
pub type SBPEIE_R = crate::BitReader<bool>;
///Field `SBPEIE` writer - Short Bit Period Error Interrupt Enable The SBPEIE bit is set and cleared by software.
pub type SBPEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
///Field `LBPEIE` reader - Long Bit Period Error Interrupt Enable The LBPEIE bit is set and cleared by software.
pub type LBPEIE_R = crate::BitReader<bool>;
///Field `LBPEIE` writer - Long Bit Period Error Interrupt Enable The LBPEIE bit is set and cleared by software.
pub type LBPEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
///Field `RXACKIE` reader - Rx-Missing Acknowledge Error Interrupt Enable The RXACKIE bit is set and cleared by software.
pub type RXACKIE_R = crate::BitReader<bool>;
///Field `RXACKIE` writer - Rx-Missing Acknowledge Error Interrupt Enable The RXACKIE bit is set and cleared by software.
pub type RXACKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
///Field `ARBLSTIE` reader - Arbitration Lost Interrupt Enable The ARBLSTIE bit is set and cleared by software.
pub type ARBLSTIE_R = crate::BitReader<bool>;
///Field `ARBLSTIE` writer - Arbitration Lost Interrupt Enable The ARBLSTIE bit is set and cleared by software.
pub type ARBLSTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
///Field `TXBRIE` reader - Tx-Byte Request Interrupt Enable The TXBRIE bit is set and cleared by software.
pub type TXBRIE_R = crate::BitReader<bool>;
///Field `TXBRIE` writer - Tx-Byte Request Interrupt Enable The TXBRIE bit is set and cleared by software.
pub type TXBRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
///Field `TXENDIE` reader - Tx-End Of Message Interrupt Enable The TXENDIE bit is set and cleared by software.
pub type TXENDIE_R = crate::BitReader<bool>;
///Field `TXENDIE` writer - Tx-End Of Message Interrupt Enable The TXENDIE bit is set and cleared by software.
pub type TXENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
///Field `TXUDRIE` reader - Tx-Underrun Interrupt Enable The TXUDRIE bit is set and cleared by software.
pub type TXUDRIE_R = crate::BitReader<bool>;
///Field `TXUDRIE` writer - Tx-Underrun Interrupt Enable The TXUDRIE bit is set and cleared by software.
pub type TXUDRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
///Field `TXERRIE` reader - Tx-Error Interrupt Enable The TXERRIE bit is set and cleared by software.
pub type TXERRIE_R = crate::BitReader<bool>;
///Field `TXERRIE` writer - Tx-Error Interrupt Enable The TXERRIE bit is set and cleared by software.
pub type TXERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
///Field `TXACKIE` reader - Tx-Missing Acknowledge Error Interrupt Enable The TXACKEIE bit is set and cleared by software.
pub type TXACKIE_R = crate::BitReader<bool>;
///Field `TXACKIE` writer - Tx-Missing Acknowledge Error Interrupt Enable The TXACKEIE bit is set and cleared by software.
pub type TXACKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_IER_SPEC, bool, O>;
impl R {
    ///Bit 0 - Rx-Byte Received Interrupt Enable The RXBRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn rxbrie(&self) -> RXBRIE_R {
        RXBRIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End Of Reception Interrupt Enable The RXENDIE bit is set and cleared by software.
    #[inline(always)]
    pub fn rxendie(&self) -> RXENDIE_R {
        RXENDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rx-Buffer Overrun Interrupt Enable The RXOVRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn rxovrie(&self) -> RXOVRIE_R {
        RXOVRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Bit Rising Error Interrupt Enable The BREIE bit is set and cleared by software.
    #[inline(always)]
    pub fn breie(&self) -> BREIE_R {
        BREIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Short Bit Period Error Interrupt Enable The SBPEIE bit is set and cleared by software.
    #[inline(always)]
    pub fn sbpeie(&self) -> SBPEIE_R {
        SBPEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Long Bit Period Error Interrupt Enable The LBPEIE bit is set and cleared by software.
    #[inline(always)]
    pub fn lbpeie(&self) -> LBPEIE_R {
        LBPEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rx-Missing Acknowledge Error Interrupt Enable The RXACKIE bit is set and cleared by software.
    #[inline(always)]
    pub fn rxackie(&self) -> RXACKIE_R {
        RXACKIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Arbitration Lost Interrupt Enable The ARBLSTIE bit is set and cleared by software.
    #[inline(always)]
    pub fn arblstie(&self) -> ARBLSTIE_R {
        ARBLSTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Tx-Byte Request Interrupt Enable The TXBRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txbrie(&self) -> TXBRIE_R {
        TXBRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Tx-End Of Message Interrupt Enable The TXENDIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txendie(&self) -> TXENDIE_R {
        TXENDIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Tx-Underrun Interrupt Enable The TXUDRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txudrie(&self) -> TXUDRIE_R {
        TXUDRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Tx-Error Interrupt Enable The TXERRIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txerrie(&self) -> TXERRIE_R {
        TXERRIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Tx-Missing Acknowledge Error Interrupt Enable The TXACKEIE bit is set and cleared by software.
    #[inline(always)]
    pub fn txackie(&self) -> TXACKIE_R {
        TXACKIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Rx-Byte Received Interrupt Enable The RXBRIE bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn rxbrie(&mut self) -> RXBRIE_W<0> {
        RXBRIE_W::new(self)
    }
    ///Bit 1 - End Of Reception Interrupt Enable The RXENDIE bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn rxendie(&mut self) -> RXENDIE_W<1> {
        RXENDIE_W::new(self)
    }
    ///Bit 2 - Rx-Buffer Overrun Interrupt Enable The RXOVRIE bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn rxovrie(&mut self) -> RXOVRIE_W<2> {
        RXOVRIE_W::new(self)
    }
    ///Bit 3 - Bit Rising Error Interrupt Enable The BREIE bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn breie(&mut self) -> BREIE_W<3> {
        BREIE_W::new(self)
    }
    ///Bit 4 - Short Bit Period Error Interrupt Enable The SBPEIE bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn sbpeie(&mut self) -> SBPEIE_W<4> {
        SBPEIE_W::new(self)
    }
    ///Bit 5 - Long Bit Period Error Interrupt Enable The LBPEIE bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lbpeie(&mut self) -> LBPEIE_W<5> {
        LBPEIE_W::new(self)
    }
    ///Bit 6 - Rx-Missing Acknowledge Error Interrupt Enable The RXACKIE bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn rxackie(&mut self) -> RXACKIE_W<6> {
        RXACKIE_W::new(self)
    }
    ///Bit 7 - Arbitration Lost Interrupt Enable The ARBLSTIE bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn arblstie(&mut self) -> ARBLSTIE_W<7> {
        ARBLSTIE_W::new(self)
    }
    ///Bit 8 - Tx-Byte Request Interrupt Enable The TXBRIE bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn txbrie(&mut self) -> TXBRIE_W<8> {
        TXBRIE_W::new(self)
    }
    ///Bit 9 - Tx-End Of Message Interrupt Enable The TXENDIE bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn txendie(&mut self) -> TXENDIE_W<9> {
        TXENDIE_W::new(self)
    }
    ///Bit 10 - Tx-Underrun Interrupt Enable The TXUDRIE bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn txudrie(&mut self) -> TXUDRIE_W<10> {
        TXUDRIE_W::new(self)
    }
    ///Bit 11 - Tx-Error Interrupt Enable The TXERRIE bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn txerrie(&mut self) -> TXERRIE_W<11> {
        TXERRIE_W::new(self)
    }
    ///Bit 12 - Tx-Missing Acknowledge Error Interrupt Enable The TXACKEIE bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn txackie(&mut self) -> TXACKIE_W<12> {
        TXACKIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CEC interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cec_ier](index.html) module
pub struct CEC_IER_SPEC;
impl crate::RegisterSpec for CEC_IER_SPEC {
    type Ux = u32;
}
///`read()` method returns [cec_ier::R](R) reader structure
impl crate::Readable for CEC_IER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cec_ier::W](W) writer structure
impl crate::Writable for CEC_IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CEC_IER to value 0
impl crate::Resettable for CEC_IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
