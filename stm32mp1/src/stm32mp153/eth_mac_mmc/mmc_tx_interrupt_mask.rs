///Register `MMC_TX_INTERRUPT_MASK` reader
pub struct R(crate::R<MMC_TX_INTERRUPT_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_TX_INTERRUPT_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_TX_INTERRUPT_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_TX_INTERRUPT_MASK_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MMC_TX_INTERRUPT_MASK` writer
pub struct W(crate::W<MMC_TX_INTERRUPT_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMC_TX_INTERRUPT_MASK_SPEC>;
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
impl From<crate::W<MMC_TX_INTERRUPT_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMC_TX_INTERRUPT_MASK_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TXSCOLGPIM` reader - TXSCOLGPIM
pub type TXSCOLGPIM_R = crate::BitReader<bool>;
///Field `TXSCOLGPIM` writer - TXSCOLGPIM
pub type TXSCOLGPIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_TX_INTERRUPT_MASK_SPEC, bool, O>;
///Field `TXMCOLGPIM` reader - TXMCOLGPIM
pub type TXMCOLGPIM_R = crate::BitReader<bool>;
///Field `TXMCOLGPIM` writer - TXMCOLGPIM
pub type TXMCOLGPIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_TX_INTERRUPT_MASK_SPEC, bool, O>;
///Field `TXGPKTIM` reader - TXGPKTIM
pub type TXGPKTIM_R = crate::BitReader<bool>;
///Field `TXGPKTIM` writer - TXGPKTIM
pub type TXGPKTIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_TX_INTERRUPT_MASK_SPEC, bool, O>;
///Field `TXLPIUSCIM` reader - TXLPIUSCIM
pub type TXLPIUSCIM_R = crate::BitReader<bool>;
///Field `TXLPIUSCIM` writer - TXLPIUSCIM
pub type TXLPIUSCIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_TX_INTERRUPT_MASK_SPEC, bool, O>;
///Field `TXLPITRCIM` reader - TXLPITRCIM
pub type TXLPITRCIM_R = crate::BitReader<bool>;
impl R {
    ///Bit 14 - TXSCOLGPIM
    #[inline(always)]
    pub fn txscolgpim(&self) -> TXSCOLGPIM_R {
        TXSCOLGPIM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TXMCOLGPIM
    #[inline(always)]
    pub fn txmcolgpim(&self) -> TXMCOLGPIM_R {
        TXMCOLGPIM_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 21 - TXGPKTIM
    #[inline(always)]
    pub fn txgpktim(&self) -> TXGPKTIM_R {
        TXGPKTIM_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 26 - TXLPIUSCIM
    #[inline(always)]
    pub fn txlpiuscim(&self) -> TXLPIUSCIM_R {
        TXLPIUSCIM_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - TXLPITRCIM
    #[inline(always)]
    pub fn txlpitrcim(&self) -> TXLPITRCIM_R {
        TXLPITRCIM_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    ///Bit 14 - TXSCOLGPIM
    #[inline(always)]
    #[must_use]
    pub fn txscolgpim(&mut self) -> TXSCOLGPIM_W<14> {
        TXSCOLGPIM_W::new(self)
    }
    ///Bit 15 - TXMCOLGPIM
    #[inline(always)]
    #[must_use]
    pub fn txmcolgpim(&mut self) -> TXMCOLGPIM_W<15> {
        TXMCOLGPIM_W::new(self)
    }
    ///Bit 21 - TXGPKTIM
    #[inline(always)]
    #[must_use]
    pub fn txgpktim(&mut self) -> TXGPKTIM_W<21> {
        TXGPKTIM_W::new(self)
    }
    ///Bit 26 - TXLPIUSCIM
    #[inline(always)]
    #[must_use]
    pub fn txlpiuscim(&mut self) -> TXLPIUSCIM_W<26> {
        TXLPIUSCIM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register maintains the masks for interrupts generated from all Transmit statistics counters. The MMC Transmit Interrupt Mask register maintains the masks for the interrupts generated when the transmit statistic counters reach half of their maximum value or the maximum values. This register is 32 bit wide. This register is present only when any one of the MMC Transmit Counters is selected during core configuration.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mmc_tx_interrupt_mask](index.html) module
pub struct MMC_TX_INTERRUPT_MASK_SPEC;
impl crate::RegisterSpec for MMC_TX_INTERRUPT_MASK_SPEC {
    type Ux = u32;
}
///`read()` method returns [mmc_tx_interrupt_mask::R](R) reader structure
impl crate::Readable for MMC_TX_INTERRUPT_MASK_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mmc_tx_interrupt_mask::W](W) writer structure
impl crate::Writable for MMC_TX_INTERRUPT_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MMC_TX_INTERRUPT_MASK to value 0
impl crate::Resettable for MMC_TX_INTERRUPT_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
