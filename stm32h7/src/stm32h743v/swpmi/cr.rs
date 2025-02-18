///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RXDMA` reader - Reception DMA enable
pub type RXDMA_R = crate::BitReader<bool>;
///Field `RXDMA` writer - Reception DMA enable
pub type RXDMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `TXDMA` reader - Transmission DMA enable
pub type TXDMA_R = crate::BitReader<bool>;
///Field `TXDMA` writer - Transmission DMA enable
pub type TXDMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `RXMODE` reader - Reception buffering mode
pub type RXMODE_R = crate::BitReader<bool>;
///Field `RXMODE` writer - Reception buffering mode
pub type RXMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `TXMODE` reader - Transmission buffering mode
pub type TXMODE_R = crate::BitReader<bool>;
///Field `TXMODE` writer - Transmission buffering mode
pub type TXMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `LPBK` reader - Loopback mode enable
pub type LPBK_R = crate::BitReader<bool>;
///Field `LPBK` writer - Loopback mode enable
pub type LPBK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `SWPACT` reader - Single wire protocol master interface activate
pub type SWPACT_R = crate::BitReader<bool>;
///Field `SWPACT` writer - Single wire protocol master interface activate
pub type SWPACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `DEACT` reader - Single wire protocol master interface deactivate
pub type DEACT_R = crate::BitReader<bool>;
///Field `DEACT` writer - Single wire protocol master interface deactivate
pub type DEACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `SWPTEN` reader - Single wire protocol master transceiver enable
pub type SWPTEN_R = crate::BitReader<bool>;
///Field `SWPTEN` writer - Single wire protocol master transceiver enable
pub type SWPTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Reception DMA enable
    #[inline(always)]
    pub fn rxdma(&self) -> RXDMA_R {
        RXDMA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmission DMA enable
    #[inline(always)]
    pub fn txdma(&self) -> TXDMA_R {
        TXDMA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Reception buffering mode
    #[inline(always)]
    pub fn rxmode(&self) -> RXMODE_R {
        RXMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmission buffering mode
    #[inline(always)]
    pub fn txmode(&self) -> TXMODE_R {
        TXMODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Loopback mode enable
    #[inline(always)]
    pub fn lpbk(&self) -> LPBK_R {
        LPBK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Single wire protocol master interface activate
    #[inline(always)]
    pub fn swpact(&self) -> SWPACT_R {
        SWPACT_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 10 - Single wire protocol master interface deactivate
    #[inline(always)]
    pub fn deact(&self) -> DEACT_R {
        DEACT_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Single wire protocol master transceiver enable
    #[inline(always)]
    pub fn swpten(&self) -> SWPTEN_R {
        SWPTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Reception DMA enable
    #[inline(always)]
    #[must_use]
    pub fn rxdma(&mut self) -> RXDMA_W<0> {
        RXDMA_W::new(self)
    }
    ///Bit 1 - Transmission DMA enable
    #[inline(always)]
    #[must_use]
    pub fn txdma(&mut self) -> TXDMA_W<1> {
        TXDMA_W::new(self)
    }
    ///Bit 2 - Reception buffering mode
    #[inline(always)]
    #[must_use]
    pub fn rxmode(&mut self) -> RXMODE_W<2> {
        RXMODE_W::new(self)
    }
    ///Bit 3 - Transmission buffering mode
    #[inline(always)]
    #[must_use]
    pub fn txmode(&mut self) -> TXMODE_W<3> {
        TXMODE_W::new(self)
    }
    ///Bit 4 - Loopback mode enable
    #[inline(always)]
    #[must_use]
    pub fn lpbk(&mut self) -> LPBK_W<4> {
        LPBK_W::new(self)
    }
    ///Bit 5 - Single wire protocol master interface activate
    #[inline(always)]
    #[must_use]
    pub fn swpact(&mut self) -> SWPACT_W<5> {
        SWPACT_W::new(self)
    }
    ///Bit 10 - Single wire protocol master interface deactivate
    #[inline(always)]
    #[must_use]
    pub fn deact(&mut self) -> DEACT_W<10> {
        DEACT_W::new(self)
    }
    ///Bit 11 - Single wire protocol master transceiver enable
    #[inline(always)]
    #[must_use]
    pub fn swpten(&mut self) -> SWPTEN_W<11> {
        SWPTEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SWPMI Configuration/Control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
