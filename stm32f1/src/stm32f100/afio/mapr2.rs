///Register `MAPR2` reader
pub struct R(crate::R<MAPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAPR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MAPR2` writer
pub struct W(crate::W<MAPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAPR2_SPEC>;
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
impl From<crate::W<MAPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAPR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIM15_REMAP` reader - TIM15 remapping
pub type TIM15_REMAP_R = crate::BitReader<bool>;
///Field `TIM15_REMAP` writer - TIM15 remapping
pub type TIM15_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR2_SPEC, bool, O>;
///Field `TIM16_REMAP` reader - TIM16 remapping
pub type TIM16_REMAP_R = crate::BitReader<bool>;
///Field `TIM16_REMAP` writer - TIM16 remapping
pub type TIM16_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR2_SPEC, bool, O>;
///Field `TIM17_REMAP` reader - TIM17 remapping
pub type TIM17_REMAP_R = crate::BitReader<bool>;
///Field `TIM17_REMAP` writer - TIM17 remapping
pub type TIM17_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR2_SPEC, bool, O>;
///Field `CEC_REMAP` reader - CEC remapping
pub type CEC_REMAP_R = crate::BitReader<bool>;
///Field `CEC_REMAP` writer - CEC remapping
pub type CEC_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR2_SPEC, bool, O>;
///Field `TIM1_DMA_REMAP` reader - TIM1 DMA remapping
pub type TIM1_DMA_REMAP_R = crate::BitReader<bool>;
///Field `TIM1_DMA_REMAP` writer - TIM1 DMA remapping
pub type TIM1_DMA_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR2_SPEC, bool, O>;
///Field `TIM13_REMAP` reader - TIM13 remapping
pub type TIM13_REMAP_R = crate::BitReader<bool>;
///Field `TIM13_REMAP` writer - TIM13 remapping
pub type TIM13_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR2_SPEC, bool, O>;
///Field `TIM14_REMAP` reader - TIM14 remapping
pub type TIM14_REMAP_R = crate::BitReader<bool>;
///Field `TIM14_REMAP` writer - TIM14 remapping
pub type TIM14_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR2_SPEC, bool, O>;
///Field `FSMC_NADV` reader - NADV connect/disconnect
pub type FSMC_NADV_R = crate::BitReader<bool>;
///Field `FSMC_NADV` writer - NADV connect/disconnect
pub type FSMC_NADV_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR2_SPEC, bool, O>;
///Field `TIM67_DAC_DMA_REMAP` reader - TIM67_DAC DMA remapping
pub type TIM67_DAC_DMA_REMAP_R = crate::BitReader<bool>;
///Field `TIM67_DAC_DMA_REMAP` writer - TIM67_DAC DMA remapping
pub type TIM67_DAC_DMA_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR2_SPEC, bool, O>;
///Field `TIM12_REMAP` reader - TIM12 remapping
pub type TIM12_REMAP_R = crate::BitReader<bool>;
///Field `TIM12_REMAP` writer - TIM12 remapping
pub type TIM12_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR2_SPEC, bool, O>;
///Field `MISC_REMAP` reader - Miscellaneous features remapping
pub type MISC_REMAP_R = crate::BitReader<bool>;
///Field `MISC_REMAP` writer - Miscellaneous features remapping
pub type MISC_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAPR2_SPEC, bool, O>;
impl R {
    ///Bit 0 - TIM15 remapping
    #[inline(always)]
    pub fn tim15_remap(&self) -> TIM15_REMAP_R {
        TIM15_REMAP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM16 remapping
    #[inline(always)]
    pub fn tim16_remap(&self) -> TIM16_REMAP_R {
        TIM16_REMAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM17 remapping
    #[inline(always)]
    pub fn tim17_remap(&self) -> TIM17_REMAP_R {
        TIM17_REMAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CEC remapping
    #[inline(always)]
    pub fn cec_remap(&self) -> CEC_REMAP_R {
        CEC_REMAP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM1 DMA remapping
    #[inline(always)]
    pub fn tim1_dma_remap(&self) -> TIM1_DMA_REMAP_R {
        TIM1_DMA_REMAP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - TIM13 remapping
    #[inline(always)]
    pub fn tim13_remap(&self) -> TIM13_REMAP_R {
        TIM13_REMAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - TIM14 remapping
    #[inline(always)]
    pub fn tim14_remap(&self) -> TIM14_REMAP_R {
        TIM14_REMAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - NADV connect/disconnect
    #[inline(always)]
    pub fn fsmc_nadv(&self) -> FSMC_NADV_R {
        FSMC_NADV_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - TIM67_DAC DMA remapping
    #[inline(always)]
    pub fn tim67_dac_dma_remap(&self) -> TIM67_DAC_DMA_REMAP_R {
        TIM67_DAC_DMA_REMAP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TIM12 remapping
    #[inline(always)]
    pub fn tim12_remap(&self) -> TIM12_REMAP_R {
        TIM12_REMAP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Miscellaneous features remapping
    #[inline(always)]
    pub fn misc_remap(&self) -> MISC_REMAP_R {
        MISC_REMAP_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TIM15 remapping
    #[inline(always)]
    #[must_use]
    pub fn tim15_remap(&mut self) -> TIM15_REMAP_W<0> {
        TIM15_REMAP_W::new(self)
    }
    ///Bit 1 - TIM16 remapping
    #[inline(always)]
    #[must_use]
    pub fn tim16_remap(&mut self) -> TIM16_REMAP_W<1> {
        TIM16_REMAP_W::new(self)
    }
    ///Bit 2 - TIM17 remapping
    #[inline(always)]
    #[must_use]
    pub fn tim17_remap(&mut self) -> TIM17_REMAP_W<2> {
        TIM17_REMAP_W::new(self)
    }
    ///Bit 3 - CEC remapping
    #[inline(always)]
    #[must_use]
    pub fn cec_remap(&mut self) -> CEC_REMAP_W<3> {
        CEC_REMAP_W::new(self)
    }
    ///Bit 4 - TIM1 DMA remapping
    #[inline(always)]
    #[must_use]
    pub fn tim1_dma_remap(&mut self) -> TIM1_DMA_REMAP_W<4> {
        TIM1_DMA_REMAP_W::new(self)
    }
    ///Bit 8 - TIM13 remapping
    #[inline(always)]
    #[must_use]
    pub fn tim13_remap(&mut self) -> TIM13_REMAP_W<8> {
        TIM13_REMAP_W::new(self)
    }
    ///Bit 9 - TIM14 remapping
    #[inline(always)]
    #[must_use]
    pub fn tim14_remap(&mut self) -> TIM14_REMAP_W<9> {
        TIM14_REMAP_W::new(self)
    }
    ///Bit 10 - NADV connect/disconnect
    #[inline(always)]
    #[must_use]
    pub fn fsmc_nadv(&mut self) -> FSMC_NADV_W<10> {
        FSMC_NADV_W::new(self)
    }
    ///Bit 11 - TIM67_DAC DMA remapping
    #[inline(always)]
    #[must_use]
    pub fn tim67_dac_dma_remap(&mut self) -> TIM67_DAC_DMA_REMAP_W<11> {
        TIM67_DAC_DMA_REMAP_W::new(self)
    }
    ///Bit 12 - TIM12 remapping
    #[inline(always)]
    #[must_use]
    pub fn tim12_remap(&mut self) -> TIM12_REMAP_W<12> {
        TIM12_REMAP_W::new(self)
    }
    ///Bit 13 - Miscellaneous features remapping
    #[inline(always)]
    #[must_use]
    pub fn misc_remap(&mut self) -> MISC_REMAP_W<13> {
        MISC_REMAP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AF remap and debug I/O configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mapr2](index.html) module
pub struct MAPR2_SPEC;
impl crate::RegisterSpec for MAPR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [mapr2::R](R) reader structure
impl crate::Readable for MAPR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mapr2::W](W) writer structure
impl crate::Writable for MAPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MAPR2 to value 0
impl crate::Resettable for MAPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
