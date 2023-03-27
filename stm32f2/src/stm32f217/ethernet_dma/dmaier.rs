///Register `DMAIER` reader
pub struct R(crate::R<DMAIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAIER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMAIER` writer
pub struct W(crate::W<DMAIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAIER_SPEC>;
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
impl From<crate::W<DMAIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAIER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIE` reader - Transmit interrupt enable
pub type TIE_R = crate::BitReader<bool>;
///Field `TIE` writer - Transmit interrupt enable
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
///Field `TPSIE` reader - Transmit process stopped interrupt enable
pub type TPSIE_R = crate::BitReader<bool>;
///Field `TPSIE` writer - Transmit process stopped interrupt enable
pub type TPSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
///Field `TBUIE` reader - Transmit buffer unavailable interrupt enable
pub type TBUIE_R = crate::BitReader<bool>;
///Field `TBUIE` writer - Transmit buffer unavailable interrupt enable
pub type TBUIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
///Field `TJTIE` reader - Transmit jabber timeout interrupt enable
pub type TJTIE_R = crate::BitReader<bool>;
///Field `TJTIE` writer - Transmit jabber timeout interrupt enable
pub type TJTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
///Field `ROIE` reader - Overflow interrupt enable
pub type ROIE_R = crate::BitReader<bool>;
///Field `ROIE` writer - Overflow interrupt enable
pub type ROIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
///Field `TUIE` reader - Underflow interrupt enable
pub type TUIE_R = crate::BitReader<bool>;
///Field `TUIE` writer - Underflow interrupt enable
pub type TUIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
///Field `RIE` reader - Receive interrupt enable
pub type RIE_R = crate::BitReader<bool>;
///Field `RIE` writer - Receive interrupt enable
pub type RIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
///Field `RBUIE` reader - Receive buffer unavailable interrupt enable
pub type RBUIE_R = crate::BitReader<bool>;
///Field `RBUIE` writer - Receive buffer unavailable interrupt enable
pub type RBUIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
///Field `RPSIE` reader - Receive process stopped interrupt enable
pub type RPSIE_R = crate::BitReader<bool>;
///Field `RPSIE` writer - Receive process stopped interrupt enable
pub type RPSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
///Field `RWTIE` reader - receive watchdog timeout interrupt enable
pub type RWTIE_R = crate::BitReader<bool>;
///Field `RWTIE` writer - receive watchdog timeout interrupt enable
pub type RWTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
///Field `ETIE` reader - Early transmit interrupt enable
pub type ETIE_R = crate::BitReader<bool>;
///Field `ETIE` writer - Early transmit interrupt enable
pub type ETIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
///Field `FBEIE` reader - Fatal bus error interrupt enable
pub type FBEIE_R = crate::BitReader<bool>;
///Field `FBEIE` writer - Fatal bus error interrupt enable
pub type FBEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
///Field `ERIE` reader - Early receive interrupt enable
pub type ERIE_R = crate::BitReader<bool>;
///Field `ERIE` writer - Early receive interrupt enable
pub type ERIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
///Field `AISE` reader - Abnormal interrupt summary enable
pub type AISE_R = crate::BitReader<bool>;
///Field `AISE` writer - Abnormal interrupt summary enable
pub type AISE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
///Field `NISE` reader - Normal interrupt summary enable
pub type NISE_R = crate::BitReader<bool>;
///Field `NISE` writer - Normal interrupt summary enable
pub type NISE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIER_SPEC, bool, O>;
impl R {
    ///Bit 0 - Transmit interrupt enable
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit process stopped interrupt enable
    #[inline(always)]
    pub fn tpsie(&self) -> TPSIE_R {
        TPSIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Transmit buffer unavailable interrupt enable
    #[inline(always)]
    pub fn tbuie(&self) -> TBUIE_R {
        TBUIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmit jabber timeout interrupt enable
    #[inline(always)]
    pub fn tjtie(&self) -> TJTIE_R {
        TJTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Overflow interrupt enable
    #[inline(always)]
    pub fn roie(&self) -> ROIE_R {
        ROIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Underflow interrupt enable
    #[inline(always)]
    pub fn tuie(&self) -> TUIE_R {
        TUIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Receive interrupt enable
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Receive buffer unavailable interrupt enable
    #[inline(always)]
    pub fn rbuie(&self) -> RBUIE_R {
        RBUIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Receive process stopped interrupt enable
    #[inline(always)]
    pub fn rpsie(&self) -> RPSIE_R {
        RPSIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - receive watchdog timeout interrupt enable
    #[inline(always)]
    pub fn rwtie(&self) -> RWTIE_R {
        RWTIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Early transmit interrupt enable
    #[inline(always)]
    pub fn etie(&self) -> ETIE_R {
        ETIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - Fatal bus error interrupt enable
    #[inline(always)]
    pub fn fbeie(&self) -> FBEIE_R {
        FBEIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Early receive interrupt enable
    #[inline(always)]
    pub fn erie(&self) -> ERIE_R {
        ERIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Abnormal interrupt summary enable
    #[inline(always)]
    pub fn aise(&self) -> AISE_R {
        AISE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Normal interrupt summary enable
    #[inline(always)]
    pub fn nise(&self) -> NISE_R {
        NISE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Transmit interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<0> {
        TIE_W::new(self)
    }
    ///Bit 1 - Transmit process stopped interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tpsie(&mut self) -> TPSIE_W<1> {
        TPSIE_W::new(self)
    }
    ///Bit 2 - Transmit buffer unavailable interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tbuie(&mut self) -> TBUIE_W<2> {
        TBUIE_W::new(self)
    }
    ///Bit 3 - Transmit jabber timeout interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tjtie(&mut self) -> TJTIE_W<3> {
        TJTIE_W::new(self)
    }
    ///Bit 4 - Overflow interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn roie(&mut self) -> ROIE_W<4> {
        ROIE_W::new(self)
    }
    ///Bit 5 - Underflow interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tuie(&mut self) -> TUIE_W<5> {
        TUIE_W::new(self)
    }
    ///Bit 6 - Receive interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<6> {
        RIE_W::new(self)
    }
    ///Bit 7 - Receive buffer unavailable interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rbuie(&mut self) -> RBUIE_W<7> {
        RBUIE_W::new(self)
    }
    ///Bit 8 - Receive process stopped interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rpsie(&mut self) -> RPSIE_W<8> {
        RPSIE_W::new(self)
    }
    ///Bit 9 - receive watchdog timeout interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rwtie(&mut self) -> RWTIE_W<9> {
        RWTIE_W::new(self)
    }
    ///Bit 10 - Early transmit interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn etie(&mut self) -> ETIE_W<10> {
        ETIE_W::new(self)
    }
    ///Bit 13 - Fatal bus error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn fbeie(&mut self) -> FBEIE_W<13> {
        FBEIE_W::new(self)
    }
    ///Bit 14 - Early receive interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn erie(&mut self) -> ERIE_W<14> {
        ERIE_W::new(self)
    }
    ///Bit 15 - Abnormal interrupt summary enable
    #[inline(always)]
    #[must_use]
    pub fn aise(&mut self) -> AISE_W<15> {
        AISE_W::new(self)
    }
    ///Bit 16 - Normal interrupt summary enable
    #[inline(always)]
    #[must_use]
    pub fn nise(&mut self) -> NISE_W<16> {
        NISE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet DMA interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmaier](index.html) module
pub struct DMAIER_SPEC;
impl crate::RegisterSpec for DMAIER_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmaier::R](R) reader structure
impl crate::Readable for DMAIER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dmaier::W](W) writer structure
impl crate::Writable for DMAIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMAIER to value 0
impl crate::Resettable for DMAIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
