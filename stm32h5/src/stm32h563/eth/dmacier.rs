///Register `DMACIER` reader
pub struct R(crate::R<DMACIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACIER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMACIER` writer
pub struct W(crate::W<DMACIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACIER_SPEC>;
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
impl From<crate::W<DMACIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACIER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIE` reader - Transmit Interrupt Enable When this bit is set along with the NIE bit, the Transmit Interrupt is enabled. When this bit is reset, the Transmit Interrupt is disabled.
pub type TIE_R = crate::BitReader<bool>;
///Field `TIE` writer - Transmit Interrupt Enable When this bit is set along with the NIE bit, the Transmit Interrupt is enabled. When this bit is reset, the Transmit Interrupt is disabled.
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACIER_SPEC, bool, O>;
///Field `TXSE` reader - Transmit Stopped Enable When this bit is set along with the AIE bit, the Transmission Stopped interrupt is enabled. When this bit is reset, the Transmission Stopped interrupt is disabled.
pub type TXSE_R = crate::BitReader<bool>;
///Field `TXSE` writer - Transmit Stopped Enable When this bit is set along with the AIE bit, the Transmission Stopped interrupt is enabled. When this bit is reset, the Transmission Stopped interrupt is disabled.
pub type TXSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACIER_SPEC, bool, O>;
///Field `TBUE` reader - Transmit Buffer Unavailable Enable When this bit is set along with the NIE bit, the Transmit Buffer Unavailable interrupt is enabled. When this bit is reset, the Transmit Buffer Unavailable interrupt is disabled.
pub type TBUE_R = crate::BitReader<bool>;
///Field `TBUE` writer - Transmit Buffer Unavailable Enable When this bit is set along with the NIE bit, the Transmit Buffer Unavailable interrupt is enabled. When this bit is reset, the Transmit Buffer Unavailable interrupt is disabled.
pub type TBUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACIER_SPEC, bool, O>;
///Field `RIE` reader - Receive Interrupt Enable When this bit is set along with the NIE bit, the Receive Interrupt is enabled. When this bit is reset, the Receive Interrupt is disabled.
pub type RIE_R = crate::BitReader<bool>;
///Field `RIE` writer - Receive Interrupt Enable When this bit is set along with the NIE bit, the Receive Interrupt is enabled. When this bit is reset, the Receive Interrupt is disabled.
pub type RIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACIER_SPEC, bool, O>;
///Field `RBUE` reader - Receive Buffer Unavailable Enable When this bit is set along with the AIE bit, the Receive Buffer Unavailable interrupt is enabled. When this bit is reset, the Receive Buffer Unavailable interrupt is disabled.
pub type RBUE_R = crate::BitReader<bool>;
///Field `RBUE` writer - Receive Buffer Unavailable Enable When this bit is set along with the AIE bit, the Receive Buffer Unavailable interrupt is enabled. When this bit is reset, the Receive Buffer Unavailable interrupt is disabled.
pub type RBUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACIER_SPEC, bool, O>;
///Field `RSE` reader - Receive Stopped Enable When this bit is set along with the AIE bit, the Receive Stopped Interrupt is enabled. When this bit is reset, the Receive Stopped interrupt is disabled.
pub type RSE_R = crate::BitReader<bool>;
///Field `RSE` writer - Receive Stopped Enable When this bit is set along with the AIE bit, the Receive Stopped Interrupt is enabled. When this bit is reset, the Receive Stopped interrupt is disabled.
pub type RSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACIER_SPEC, bool, O>;
///Field `RWTE` reader - Receive Watchdog Timeout Enable When this bit is set along with the AIE bit, the Receive Watchdog Timeout interrupt is enabled. When this bit is reset, the Receive Watchdog Timeout interrupt is disabled.
pub type RWTE_R = crate::BitReader<bool>;
///Field `RWTE` writer - Receive Watchdog Timeout Enable When this bit is set along with the AIE bit, the Receive Watchdog Timeout interrupt is enabled. When this bit is reset, the Receive Watchdog Timeout interrupt is disabled.
pub type RWTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACIER_SPEC, bool, O>;
///Field `ETIE` reader - Early Transmit Interrupt Enable When this bit is set along with the AIE bit, the Early Transmit interrupt is enabled. When this bit is reset, the Early Transmit interrupt is disabled.
pub type ETIE_R = crate::BitReader<bool>;
///Field `ETIE` writer - Early Transmit Interrupt Enable When this bit is set along with the AIE bit, the Early Transmit interrupt is enabled. When this bit is reset, the Early Transmit interrupt is disabled.
pub type ETIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACIER_SPEC, bool, O>;
///Field `ERIE` reader - Early Receive Interrupt Enable When this bit is set along with the NIE bit, the Early Receive interrupt is enabled. When this bit is reset, the Early Receive interrupt is disabled.
pub type ERIE_R = crate::BitReader<bool>;
///Field `ERIE` writer - Early Receive Interrupt Enable When this bit is set along with the NIE bit, the Early Receive interrupt is enabled. When this bit is reset, the Early Receive interrupt is disabled.
pub type ERIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACIER_SPEC, bool, O>;
///Field `FBEE` reader - Fatal Bus Error Enable When this bit is set along with the AIE bit, the Fatal Bus error interrupt is enabled. When this bit is reset, the Fatal Bus Error error interrupt is disabled.
pub type FBEE_R = crate::BitReader<bool>;
///Field `FBEE` writer - Fatal Bus Error Enable When this bit is set along with the AIE bit, the Fatal Bus error interrupt is enabled. When this bit is reset, the Fatal Bus Error error interrupt is disabled.
pub type FBEE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACIER_SPEC, bool, O>;
///Field `CDEE` reader - Context Descriptor Error Enable When this bit is set along with the AIE bit, the Context Descriptor error interrupt is enabled. When this bit is reset, the Context Descriptor error interrupt is disabled.
pub type CDEE_R = crate::BitReader<bool>;
///Field `CDEE` writer - Context Descriptor Error Enable When this bit is set along with the AIE bit, the Context Descriptor error interrupt is enabled. When this bit is reset, the Context Descriptor error interrupt is disabled.
pub type CDEE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACIER_SPEC, bool, O>;
///Field `AIE` reader - Abnormal Interrupt Summary Enable When this bit is set, the abnormal interrupt summary is enabled. This bit enables the following interrupts in the Channel status register (ETH_DMACSR): Bit 1: Transmit Process Stopped Bit 7: Rx Buffer Unavailable Bit 8: Receive Process Stopped Bit 9: Receive Watchdog Timeout Bit 10: Early Transmit Interrupt Bit 12: Fatal Bus Error When this bit is reset, the abnormal interrupt summary is disabled.
pub type AIE_R = crate::BitReader<bool>;
///Field `AIE` writer - Abnormal Interrupt Summary Enable When this bit is set, the abnormal interrupt summary is enabled. This bit enables the following interrupts in the Channel status register (ETH_DMACSR): Bit 1: Transmit Process Stopped Bit 7: Rx Buffer Unavailable Bit 8: Receive Process Stopped Bit 9: Receive Watchdog Timeout Bit 10: Early Transmit Interrupt Bit 12: Fatal Bus Error When this bit is reset, the abnormal interrupt summary is disabled.
pub type AIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACIER_SPEC, bool, O>;
///Field `NIE` reader - Normal Interrupt Summary Enable When this bit is set, the normal interrupt summary is enabled. This bit enables the following interrupts in the Channel status register (ETH_DMACSR): Bit 0: Transmit Interrupt Bit 2: Transmit Buffer Unavailable Bit 6: Receive Interrupt Bit 11: Early Receive Interrupt When this bit is reset, the normal interrupt summary is disabled.
pub type NIE_R = crate::BitReader<bool>;
///Field `NIE` writer - Normal Interrupt Summary Enable When this bit is set, the normal interrupt summary is enabled. This bit enables the following interrupts in the Channel status register (ETH_DMACSR): Bit 0: Transmit Interrupt Bit 2: Transmit Buffer Unavailable Bit 6: Receive Interrupt Bit 11: Early Receive Interrupt When this bit is reset, the normal interrupt summary is disabled.
pub type NIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACIER_SPEC, bool, O>;
impl R {
    ///Bit 0 - Transmit Interrupt Enable When this bit is set along with the NIE bit, the Transmit Interrupt is enabled. When this bit is reset, the Transmit Interrupt is disabled.
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit Stopped Enable When this bit is set along with the AIE bit, the Transmission Stopped interrupt is enabled. When this bit is reset, the Transmission Stopped interrupt is disabled.
    #[inline(always)]
    pub fn txse(&self) -> TXSE_R {
        TXSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Transmit Buffer Unavailable Enable When this bit is set along with the NIE bit, the Transmit Buffer Unavailable interrupt is enabled. When this bit is reset, the Transmit Buffer Unavailable interrupt is disabled.
    #[inline(always)]
    pub fn tbue(&self) -> TBUE_R {
        TBUE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 6 - Receive Interrupt Enable When this bit is set along with the NIE bit, the Receive Interrupt is enabled. When this bit is reset, the Receive Interrupt is disabled.
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Receive Buffer Unavailable Enable When this bit is set along with the AIE bit, the Receive Buffer Unavailable interrupt is enabled. When this bit is reset, the Receive Buffer Unavailable interrupt is disabled.
    #[inline(always)]
    pub fn rbue(&self) -> RBUE_R {
        RBUE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Receive Stopped Enable When this bit is set along with the AIE bit, the Receive Stopped Interrupt is enabled. When this bit is reset, the Receive Stopped interrupt is disabled.
    #[inline(always)]
    pub fn rse(&self) -> RSE_R {
        RSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Receive Watchdog Timeout Enable When this bit is set along with the AIE bit, the Receive Watchdog Timeout interrupt is enabled. When this bit is reset, the Receive Watchdog Timeout interrupt is disabled.
    #[inline(always)]
    pub fn rwte(&self) -> RWTE_R {
        RWTE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Early Transmit Interrupt Enable When this bit is set along with the AIE bit, the Early Transmit interrupt is enabled. When this bit is reset, the Early Transmit interrupt is disabled.
    #[inline(always)]
    pub fn etie(&self) -> ETIE_R {
        ETIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Early Receive Interrupt Enable When this bit is set along with the NIE bit, the Early Receive interrupt is enabled. When this bit is reset, the Early Receive interrupt is disabled.
    #[inline(always)]
    pub fn erie(&self) -> ERIE_R {
        ERIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Fatal Bus Error Enable When this bit is set along with the AIE bit, the Fatal Bus error interrupt is enabled. When this bit is reset, the Fatal Bus Error error interrupt is disabled.
    #[inline(always)]
    pub fn fbee(&self) -> FBEE_R {
        FBEE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Context Descriptor Error Enable When this bit is set along with the AIE bit, the Context Descriptor error interrupt is enabled. When this bit is reset, the Context Descriptor error interrupt is disabled.
    #[inline(always)]
    pub fn cdee(&self) -> CDEE_R {
        CDEE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Abnormal Interrupt Summary Enable When this bit is set, the abnormal interrupt summary is enabled. This bit enables the following interrupts in the Channel status register (ETH_DMACSR): Bit 1: Transmit Process Stopped Bit 7: Rx Buffer Unavailable Bit 8: Receive Process Stopped Bit 9: Receive Watchdog Timeout Bit 10: Early Transmit Interrupt Bit 12: Fatal Bus Error When this bit is reset, the abnormal interrupt summary is disabled.
    #[inline(always)]
    pub fn aie(&self) -> AIE_R {
        AIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Normal Interrupt Summary Enable When this bit is set, the normal interrupt summary is enabled. This bit enables the following interrupts in the Channel status register (ETH_DMACSR): Bit 0: Transmit Interrupt Bit 2: Transmit Buffer Unavailable Bit 6: Receive Interrupt Bit 11: Early Receive Interrupt When this bit is reset, the normal interrupt summary is disabled.
    #[inline(always)]
    pub fn nie(&self) -> NIE_R {
        NIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Transmit Interrupt Enable When this bit is set along with the NIE bit, the Transmit Interrupt is enabled. When this bit is reset, the Transmit Interrupt is disabled.
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<0> {
        TIE_W::new(self)
    }
    ///Bit 1 - Transmit Stopped Enable When this bit is set along with the AIE bit, the Transmission Stopped interrupt is enabled. When this bit is reset, the Transmission Stopped interrupt is disabled.
    #[inline(always)]
    #[must_use]
    pub fn txse(&mut self) -> TXSE_W<1> {
        TXSE_W::new(self)
    }
    ///Bit 2 - Transmit Buffer Unavailable Enable When this bit is set along with the NIE bit, the Transmit Buffer Unavailable interrupt is enabled. When this bit is reset, the Transmit Buffer Unavailable interrupt is disabled.
    #[inline(always)]
    #[must_use]
    pub fn tbue(&mut self) -> TBUE_W<2> {
        TBUE_W::new(self)
    }
    ///Bit 6 - Receive Interrupt Enable When this bit is set along with the NIE bit, the Receive Interrupt is enabled. When this bit is reset, the Receive Interrupt is disabled.
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<6> {
        RIE_W::new(self)
    }
    ///Bit 7 - Receive Buffer Unavailable Enable When this bit is set along with the AIE bit, the Receive Buffer Unavailable interrupt is enabled. When this bit is reset, the Receive Buffer Unavailable interrupt is disabled.
    #[inline(always)]
    #[must_use]
    pub fn rbue(&mut self) -> RBUE_W<7> {
        RBUE_W::new(self)
    }
    ///Bit 8 - Receive Stopped Enable When this bit is set along with the AIE bit, the Receive Stopped Interrupt is enabled. When this bit is reset, the Receive Stopped interrupt is disabled.
    #[inline(always)]
    #[must_use]
    pub fn rse(&mut self) -> RSE_W<8> {
        RSE_W::new(self)
    }
    ///Bit 9 - Receive Watchdog Timeout Enable When this bit is set along with the AIE bit, the Receive Watchdog Timeout interrupt is enabled. When this bit is reset, the Receive Watchdog Timeout interrupt is disabled.
    #[inline(always)]
    #[must_use]
    pub fn rwte(&mut self) -> RWTE_W<9> {
        RWTE_W::new(self)
    }
    ///Bit 10 - Early Transmit Interrupt Enable When this bit is set along with the AIE bit, the Early Transmit interrupt is enabled. When this bit is reset, the Early Transmit interrupt is disabled.
    #[inline(always)]
    #[must_use]
    pub fn etie(&mut self) -> ETIE_W<10> {
        ETIE_W::new(self)
    }
    ///Bit 11 - Early Receive Interrupt Enable When this bit is set along with the NIE bit, the Early Receive interrupt is enabled. When this bit is reset, the Early Receive interrupt is disabled.
    #[inline(always)]
    #[must_use]
    pub fn erie(&mut self) -> ERIE_W<11> {
        ERIE_W::new(self)
    }
    ///Bit 12 - Fatal Bus Error Enable When this bit is set along with the AIE bit, the Fatal Bus error interrupt is enabled. When this bit is reset, the Fatal Bus Error error interrupt is disabled.
    #[inline(always)]
    #[must_use]
    pub fn fbee(&mut self) -> FBEE_W<12> {
        FBEE_W::new(self)
    }
    ///Bit 13 - Context Descriptor Error Enable When this bit is set along with the AIE bit, the Context Descriptor error interrupt is enabled. When this bit is reset, the Context Descriptor error interrupt is disabled.
    #[inline(always)]
    #[must_use]
    pub fn cdee(&mut self) -> CDEE_W<13> {
        CDEE_W::new(self)
    }
    ///Bit 14 - Abnormal Interrupt Summary Enable When this bit is set, the abnormal interrupt summary is enabled. This bit enables the following interrupts in the Channel status register (ETH_DMACSR): Bit 1: Transmit Process Stopped Bit 7: Rx Buffer Unavailable Bit 8: Receive Process Stopped Bit 9: Receive Watchdog Timeout Bit 10: Early Transmit Interrupt Bit 12: Fatal Bus Error When this bit is reset, the abnormal interrupt summary is disabled.
    #[inline(always)]
    #[must_use]
    pub fn aie(&mut self) -> AIE_W<14> {
        AIE_W::new(self)
    }
    ///Bit 15 - Normal Interrupt Summary Enable When this bit is set, the normal interrupt summary is enabled. This bit enables the following interrupts in the Channel status register (ETH_DMACSR): Bit 0: Transmit Interrupt Bit 2: Transmit Buffer Unavailable Bit 6: Receive Interrupt Bit 11: Early Receive Interrupt When this bit is reset, the normal interrupt summary is disabled.
    #[inline(always)]
    #[must_use]
    pub fn nie(&mut self) -> NIE_W<15> {
        NIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Channel interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmacier](index.html) module
pub struct DMACIER_SPEC;
impl crate::RegisterSpec for DMACIER_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmacier::R](R) reader structure
impl crate::Readable for DMACIER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dmacier::W](W) writer structure
impl crate::Writable for DMACIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMACIER to value 0
impl crate::Resettable for DMACIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
