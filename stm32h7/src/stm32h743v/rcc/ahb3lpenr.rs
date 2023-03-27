///Register `AHB3LPENR` reader
pub struct R(crate::R<AHB3LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB3LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB3LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB3LPENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB3LPENR` writer
pub struct W(crate::W<AHB3LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB3LPENR_SPEC>;
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
impl From<crate::W<AHB3LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB3LPENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MDMALPEN` reader - MDMA Clock Enable During CSleep Mode
pub type MDMALPEN_R = crate::BitReader<MDMALPEN_A>;
///MDMA Clock Enable During CSleep Mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDMALPEN_A {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<MDMALPEN_A> for bool {
    #[inline(always)]
    fn from(variant: MDMALPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MDMALPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MDMALPEN_A {
        match self.bits {
            false => MDMALPEN_A::Disabled,
            true => MDMALPEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MDMALPEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MDMALPEN_A::Enabled
    }
}
///Field `MDMALPEN` writer - MDMA Clock Enable During CSleep Mode
pub type MDMALPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3LPENR_SPEC, MDMALPEN_A, O>;
impl<'a, const O: u8> MDMALPEN_W<'a, O> {
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MDMALPEN_A::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MDMALPEN_A::Enabled)
    }
}
///Field `DMA2DLPEN` reader - DMA2D Clock Enable During CSleep Mode
pub use MDMALPEN_R as DMA2DLPEN_R;
///Field `JPGDECLPEN` reader - JPGDEC Clock Enable During CSleep Mode
pub use MDMALPEN_R as JPGDECLPEN_R;
///Field `FLASHLPEN` reader - FLITF Clock Enable During CSleep Mode
pub use MDMALPEN_R as FLASHLPEN_R;
///Field `FMCLPEN` reader - FMC Peripheral Clocks Enable During CSleep Mode
pub use MDMALPEN_R as FMCLPEN_R;
///Field `QSPILPEN` reader - QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode
pub use MDMALPEN_R as QSPILPEN_R;
///Field `SDMMC1LPEN` reader - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode
pub use MDMALPEN_R as SDMMC1LPEN_R;
///Field `D1DTCM1LPEN` reader - D1DTCM1 Block Clock Enable During CSleep mode
pub use MDMALPEN_R as D1DTCM1LPEN_R;
///Field `DTCM2LPEN` reader - D1 DTCM2 Block Clock Enable During CSleep mode
pub use MDMALPEN_R as DTCM2LPEN_R;
///Field `ITCMLPEN` reader - D1ITCM Block Clock Enable During CSleep mode
pub use MDMALPEN_R as ITCMLPEN_R;
///Field `AXISRAMLPEN` reader - AXISRAM Block Clock Enable During CSleep mode
pub use MDMALPEN_R as AXISRAMLPEN_R;
///Field `DMA2DLPEN` writer - DMA2D Clock Enable During CSleep Mode
pub use MDMALPEN_W as DMA2DLPEN_W;
///Field `JPGDECLPEN` writer - JPGDEC Clock Enable During CSleep Mode
pub use MDMALPEN_W as JPGDECLPEN_W;
///Field `FLASHLPEN` writer - FLITF Clock Enable During CSleep Mode
pub use MDMALPEN_W as FLASHLPEN_W;
///Field `FMCLPEN` writer - FMC Peripheral Clocks Enable During CSleep Mode
pub use MDMALPEN_W as FMCLPEN_W;
///Field `QSPILPEN` writer - QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode
pub use MDMALPEN_W as QSPILPEN_W;
///Field `SDMMC1LPEN` writer - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode
pub use MDMALPEN_W as SDMMC1LPEN_W;
///Field `D1DTCM1LPEN` writer - D1DTCM1 Block Clock Enable During CSleep mode
pub use MDMALPEN_W as D1DTCM1LPEN_W;
///Field `DTCM2LPEN` writer - D1 DTCM2 Block Clock Enable During CSleep mode
pub use MDMALPEN_W as DTCM2LPEN_W;
///Field `ITCMLPEN` writer - D1ITCM Block Clock Enable During CSleep mode
pub use MDMALPEN_W as ITCMLPEN_W;
///Field `AXISRAMLPEN` writer - AXISRAM Block Clock Enable During CSleep mode
pub use MDMALPEN_W as AXISRAMLPEN_W;
impl R {
    ///Bit 0 - MDMA Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn mdmalpen(&self) -> MDMALPEN_R {
        MDMALPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - DMA2D Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn dma2dlpen(&self) -> DMA2DLPEN_R {
        DMA2DLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - JPGDEC Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn jpgdeclpen(&self) -> JPGDECLPEN_R {
        JPGDECLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - FLITF Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn flashlpen(&self) -> FLASHLPEN_R {
        FLASHLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - FMC Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn qspilpen(&self) -> QSPILPEN_R {
        QSPILPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn sdmmc1lpen(&self) -> SDMMC1LPEN_R {
        SDMMC1LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 28 - D1DTCM1 Block Clock Enable During CSleep mode
    #[inline(always)]
    pub fn d1dtcm1lpen(&self) -> D1DTCM1LPEN_R {
        D1DTCM1LPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - D1 DTCM2 Block Clock Enable During CSleep mode
    #[inline(always)]
    pub fn dtcm2lpen(&self) -> DTCM2LPEN_R {
        DTCM2LPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - D1ITCM Block Clock Enable During CSleep mode
    #[inline(always)]
    pub fn itcmlpen(&self) -> ITCMLPEN_R {
        ITCMLPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - AXISRAM Block Clock Enable During CSleep mode
    #[inline(always)]
    pub fn axisramlpen(&self) -> AXISRAMLPEN_R {
        AXISRAMLPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - MDMA Clock Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn mdmalpen(&mut self) -> MDMALPEN_W<0> {
        MDMALPEN_W::new(self)
    }
    ///Bit 4 - DMA2D Clock Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn dma2dlpen(&mut self) -> DMA2DLPEN_W<4> {
        DMA2DLPEN_W::new(self)
    }
    ///Bit 5 - JPGDEC Clock Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn jpgdeclpen(&mut self) -> JPGDECLPEN_W<5> {
        JPGDECLPEN_W::new(self)
    }
    ///Bit 8 - FLITF Clock Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn flashlpen(&mut self) -> FLASHLPEN_W<8> {
        FLASHLPEN_W::new(self)
    }
    ///Bit 12 - FMC Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn fmclpen(&mut self) -> FMCLPEN_W<12> {
        FMCLPEN_W::new(self)
    }
    ///Bit 14 - QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn qspilpen(&mut self) -> QSPILPEN_W<14> {
        QSPILPEN_W::new(self)
    }
    ///Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1lpen(&mut self) -> SDMMC1LPEN_W<16> {
        SDMMC1LPEN_W::new(self)
    }
    ///Bit 28 - D1DTCM1 Block Clock Enable During CSleep mode
    #[inline(always)]
    #[must_use]
    pub fn d1dtcm1lpen(&mut self) -> D1DTCM1LPEN_W<28> {
        D1DTCM1LPEN_W::new(self)
    }
    ///Bit 29 - D1 DTCM2 Block Clock Enable During CSleep mode
    #[inline(always)]
    #[must_use]
    pub fn dtcm2lpen(&mut self) -> DTCM2LPEN_W<29> {
        DTCM2LPEN_W::new(self)
    }
    ///Bit 30 - D1ITCM Block Clock Enable During CSleep mode
    #[inline(always)]
    #[must_use]
    pub fn itcmlpen(&mut self) -> ITCMLPEN_W<30> {
        ITCMLPEN_W::new(self)
    }
    ///Bit 31 - AXISRAM Block Clock Enable During CSleep mode
    #[inline(always)]
    #[must_use]
    pub fn axisramlpen(&mut self) -> AXISRAMLPEN_W<31> {
        AXISRAMLPEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC AHB3 Sleep Clock Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb3lpenr](index.html) module
pub struct AHB3LPENR_SPEC;
impl crate::RegisterSpec for AHB3LPENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb3lpenr::R](R) reader structure
impl crate::Readable for AHB3LPENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb3lpenr::W](W) writer structure
impl crate::Writable for AHB3LPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHB3LPENR to value 0
impl crate::Resettable for AHB3LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
