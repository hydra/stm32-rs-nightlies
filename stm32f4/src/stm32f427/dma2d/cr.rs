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
///Field `START` reader - Start This bit can be used to launch the DMA2D according to the parameters loaded in the various configuration registers. This bit is automatically reset by the following events: ** At the end of the transfer ** When the data transfer is aborted by the user application by setting the ABORT bit in DMA2D_CR ** When a data transfer error occurs ** When the data transfer has not started due to a configuration error or another transfer operation already ongoing (automatic CLUT loading).
pub type START_R = crate::BitReader<bool>;
///Field `START` writer - Start This bit can be used to launch the DMA2D according to the parameters loaded in the various configuration registers. This bit is automatically reset by the following events: ** At the end of the transfer ** When the data transfer is aborted by the user application by setting the ABORT bit in DMA2D_CR ** When a data transfer error occurs ** When the data transfer has not started due to a configuration error or another transfer operation already ongoing (automatic CLUT loading).
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `SUSP` reader - Suspend This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when the START bit is reset.
pub type SUSP_R = crate::BitReader<bool>;
///Field `SUSP` writer - Suspend This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when the START bit is reset.
pub type SUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `ABORT` reader - Abort This bit can be used to abort the current transfer. This bit is set by software and is automatically reset by hardware when the START bit is reset.
pub type ABORT_R = crate::BitReader<bool>;
///Field `ABORT` writer - Abort This bit can be used to abort the current transfer. This bit is set by software and is automatically reset by hardware when the START bit is reset.
pub type ABORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `TEIE` reader - Transfer error interrupt enable This bit is set and cleared by software.
pub type TEIE_R = crate::BitReader<bool>;
///Field `TEIE` writer - Transfer error interrupt enable This bit is set and cleared by software.
pub type TEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `TCIE` reader - Transfer complete interrupt enable This bit is set and cleared by software.
pub type TCIE_R = crate::BitReader<bool>;
///Field `TCIE` writer - Transfer complete interrupt enable This bit is set and cleared by software.
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `TWIE` reader - Transfer watermark interrupt enable This bit is set and cleared by software.
pub type TWIE_R = crate::BitReader<bool>;
///Field `TWIE` writer - Transfer watermark interrupt enable This bit is set and cleared by software.
pub type TWIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `CAEIE` reader - CLUT access error interrupt enable This bit is set and cleared by software.
pub type CAEIE_R = crate::BitReader<bool>;
///Field `CAEIE` writer - CLUT access error interrupt enable This bit is set and cleared by software.
pub type CAEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `CTCIE` reader - CLUT transfer complete interrupt enable This bit is set and cleared by software.
pub type CTCIE_R = crate::BitReader<bool>;
///Field `CTCIE` writer - CLUT transfer complete interrupt enable This bit is set and cleared by software.
pub type CTCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `CEIE` reader - Configuration Error Interrupt Enable This bit is set and cleared by software.
pub type CEIE_R = crate::BitReader<bool>;
///Field `CEIE` writer - Configuration Error Interrupt Enable This bit is set and cleared by software.
pub type CEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `MODE` reader - DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing.
pub type MODE_R = crate::FieldReader<u8, u8>;
///Field `MODE` writer - DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing.
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
impl R {
    ///Bit 0 - Start This bit can be used to launch the DMA2D according to the parameters loaded in the various configuration registers. This bit is automatically reset by the following events: ** At the end of the transfer ** When the data transfer is aborted by the user application by setting the ABORT bit in DMA2D_CR ** When a data transfer error occurs ** When the data transfer has not started due to a configuration error or another transfer operation already ongoing (automatic CLUT loading).
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Suspend This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when the START bit is reset.
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Abort This bit can be used to abort the current transfer. This bit is set by software and is automatically reset by hardware when the START bit is reset.
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - Transfer error interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Transfer complete interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Transfer watermark interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn twie(&self) -> TWIE_R {
        TWIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CLUT access error interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn caeie(&self) -> CAEIE_R {
        CAEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CLUT transfer complete interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn ctcie(&self) -> CTCIE_R {
        CTCIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Configuration Error Interrupt Enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn ceie(&self) -> CEIE_R {
        CEIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 16:17 - DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing.
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - Start This bit can be used to launch the DMA2D according to the parameters loaded in the various configuration registers. This bit is automatically reset by the following events: ** At the end of the transfer ** When the data transfer is aborted by the user application by setting the ABORT bit in DMA2D_CR ** When a data transfer error occurs ** When the data transfer has not started due to a configuration error or another transfer operation already ongoing (automatic CLUT loading).
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    ///Bit 1 - Suspend This bit can be used to suspend the current transfer. This bit is set and reset by software. It is automatically reset by hardware when the START bit is reset.
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<1> {
        SUSP_W::new(self)
    }
    ///Bit 2 - Abort This bit can be used to abort the current transfer. This bit is set by software and is automatically reset by hardware when the START bit is reset.
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<2> {
        ABORT_W::new(self)
    }
    ///Bit 8 - Transfer error interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<8> {
        TEIE_W::new(self)
    }
    ///Bit 9 - Transfer complete interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<9> {
        TCIE_W::new(self)
    }
    ///Bit 10 - Transfer watermark interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn twie(&mut self) -> TWIE_W<10> {
        TWIE_W::new(self)
    }
    ///Bit 11 - CLUT access error interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn caeie(&mut self) -> CAEIE_W<11> {
        CAEIE_W::new(self)
    }
    ///Bit 12 - CLUT transfer complete interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn ctcie(&mut self) -> CTCIE_W<12> {
        CTCIE_W::new(self)
    }
    ///Bit 13 - Configuration Error Interrupt Enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn ceie(&mut self) -> CEIE_W<13> {
        CEIE_W::new(self)
    }
    ///Bits 16:17 - DMA2D mode This bit is set and cleared by software. It cannot be modified while a transfer is ongoing.
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<16> {
        MODE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DMA2D control register
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
