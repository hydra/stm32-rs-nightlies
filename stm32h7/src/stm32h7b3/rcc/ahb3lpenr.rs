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
///Field `MDMALPEN` reader - MDMA clock enable during CSleep mode Set and reset by software.
pub type MDMALPEN_R = crate::BitReader<MDMALPEN_A>;
///MDMA clock enable during CSleep mode Set and reset by software.
///
///Value on reset: 1
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
///Field `MDMALPEN` writer - MDMA clock enable during CSleep mode Set and reset by software.
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
///Field `DMA2DLPEN` reader - DMA2D clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as DMA2DLPEN_R;
///Field `JPGDECLPEN` reader - JPGDEC clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as JPGDECLPEN_R;
///Field `DMA2DLPEN` writer - DMA2D clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as DMA2DLPEN_W;
///Field `JPGDECLPEN` writer - JPGDEC clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as JPGDECLPEN_W;
///Field `FLASHPREN` reader - Flash interface clock enable during csleep mode
pub type FLASHPREN_R = crate::BitReader<bool>;
///Field `FLASHPREN` writer - Flash interface clock enable during csleep mode
pub type FLASHPREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3LPENR_SPEC, bool, O>;
///Field `FMCLPEN` reader - FMC peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL and provided to fmc_ker_ck input, and the rcc_hclk3 bus interface clock.
pub use MDMALPEN_R as FMCLPEN_R;
///Field `OCTOSPI1LPEN` reader - OCTOSPI1 and OCTOSPI1 delay clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as OCTOSPI1LPEN_R;
///Field `SDMMC1LPEN` reader - SDMMC1 and SDMMC1 delay clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as SDMMC1LPEN_R;
///Field `OCTOSPI2LPEN` reader - OCTOSPI2 and OCTOSPI2 delay clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as OCTOSPI2LPEN_R;
///Field `OCTOSPIMLPEN` reader - OCTOSPIM block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as OCTOSPIMLPEN_R;
///Field `OTFD1LPEN` reader - OTFD1 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as OTFD1LPEN_R;
///Field `OTFD2LPEN` reader - OTFD2 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as OTFD2LPEN_R;
///Field `GFXMMULPEN` reader - GFXMMU block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as GFXMMULPEN_R;
///Field `AXISRAM2LPEN` reader - AXISRAM2 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as AXISRAM2LPEN_R;
///Field `AXISRAM3LPEN` reader - AXISRAM3 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as AXISRAM3LPEN_R;
///Field `DTCM1LPEN` reader - DTCM1 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as DTCM1LPEN_R;
///Field `DTCM2LPEN` reader - DTCM2 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as DTCM2LPEN_R;
///Field `ITCMLPEN` reader - ITCM block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as ITCMLPEN_R;
///Field `AXISRAM1LPEN` reader - AXISRAM1 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_R as AXISRAM1LPEN_R;
///Field `FMCLPEN` writer - FMC peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL and provided to fmc_ker_ck input, and the rcc_hclk3 bus interface clock.
pub use MDMALPEN_W as FMCLPEN_W;
///Field `OCTOSPI1LPEN` writer - OCTOSPI1 and OCTOSPI1 delay clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as OCTOSPI1LPEN_W;
///Field `SDMMC1LPEN` writer - SDMMC1 and SDMMC1 delay clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as SDMMC1LPEN_W;
///Field `OCTOSPI2LPEN` writer - OCTOSPI2 and OCTOSPI2 delay clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as OCTOSPI2LPEN_W;
///Field `OCTOSPIMLPEN` writer - OCTOSPIM block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as OCTOSPIMLPEN_W;
///Field `OTFD1LPEN` writer - OTFD1 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as OTFD1LPEN_W;
///Field `OTFD2LPEN` writer - OTFD2 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as OTFD2LPEN_W;
///Field `GFXMMULPEN` writer - GFXMMU block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as GFXMMULPEN_W;
///Field `AXISRAM2LPEN` writer - AXISRAM2 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as AXISRAM2LPEN_W;
///Field `AXISRAM3LPEN` writer - AXISRAM3 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as AXISRAM3LPEN_W;
///Field `DTCM1LPEN` writer - DTCM1 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as DTCM1LPEN_W;
///Field `DTCM2LPEN` writer - DTCM2 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as DTCM2LPEN_W;
///Field `ITCMLPEN` writer - ITCM block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as ITCMLPEN_W;
///Field `AXISRAM1LPEN` writer - AXISRAM1 block clock enable during CSleep mode Set and reset by software.
pub use MDMALPEN_W as AXISRAM1LPEN_W;
impl R {
    ///Bit 0 - MDMA clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn mdmalpen(&self) -> MDMALPEN_R {
        MDMALPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - DMA2D clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn dma2dlpen(&self) -> DMA2DLPEN_R {
        DMA2DLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - JPGDEC clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn jpgdeclpen(&self) -> JPGDECLPEN_R {
        JPGDECLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Flash interface clock enable during csleep mode
    #[inline(always)]
    pub fn flashpren(&self) -> FLASHPREN_R {
        FLASHPREN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - FMC peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL and provided to fmc_ker_ck input, and the rcc_hclk3 bus interface clock.
    #[inline(always)]
    pub fn fmclpen(&self) -> FMCLPEN_R {
        FMCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - OCTOSPI1 and OCTOSPI1 delay clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn octospi1lpen(&self) -> OCTOSPI1LPEN_R {
        OCTOSPI1LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - SDMMC1 and SDMMC1 delay clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn sdmmc1lpen(&self) -> SDMMC1LPEN_R {
        SDMMC1LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 19 - OCTOSPI2 and OCTOSPI2 delay clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn octospi2lpen(&self) -> OCTOSPI2LPEN_R {
        OCTOSPI2LPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - OCTOSPIM block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn octospimlpen(&self) -> OCTOSPIMLPEN_R {
        OCTOSPIMLPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - OTFD1 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn otfd1lpen(&self) -> OTFD1LPEN_R {
        OTFD1LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - OTFD2 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn otfd2lpen(&self) -> OTFD2LPEN_R {
        OTFD2LPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - GFXMMU block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn gfxmmulpen(&self) -> GFXMMULPEN_R {
        GFXMMULPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 26 - AXISRAM2 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn axisram2lpen(&self) -> AXISRAM2LPEN_R {
        AXISRAM2LPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - AXISRAM3 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn axisram3lpen(&self) -> AXISRAM3LPEN_R {
        AXISRAM3LPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - DTCM1 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn dtcm1lpen(&self) -> DTCM1LPEN_R {
        DTCM1LPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DTCM2 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn dtcm2lpen(&self) -> DTCM2LPEN_R {
        DTCM2LPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - ITCM block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn itcmlpen(&self) -> ITCMLPEN_R {
        ITCMLPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - AXISRAM1 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn axisram1lpen(&self) -> AXISRAM1LPEN_R {
        AXISRAM1LPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - MDMA clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn mdmalpen(&mut self) -> MDMALPEN_W<0> {
        MDMALPEN_W::new(self)
    }
    ///Bit 4 - DMA2D clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn dma2dlpen(&mut self) -> DMA2DLPEN_W<4> {
        DMA2DLPEN_W::new(self)
    }
    ///Bit 5 - JPGDEC clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn jpgdeclpen(&mut self) -> JPGDECLPEN_W<5> {
        JPGDECLPEN_W::new(self)
    }
    ///Bit 8 - Flash interface clock enable during csleep mode
    #[inline(always)]
    #[must_use]
    pub fn flashpren(&mut self) -> FLASHPREN_W<8> {
        FLASHPREN_W::new(self)
    }
    ///Bit 12 - FMC peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL and provided to fmc_ker_ck input, and the rcc_hclk3 bus interface clock.
    #[inline(always)]
    #[must_use]
    pub fn fmclpen(&mut self) -> FMCLPEN_W<12> {
        FMCLPEN_W::new(self)
    }
    ///Bit 14 - OCTOSPI1 and OCTOSPI1 delay clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn octospi1lpen(&mut self) -> OCTOSPI1LPEN_W<14> {
        OCTOSPI1LPEN_W::new(self)
    }
    ///Bit 16 - SDMMC1 and SDMMC1 delay clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1lpen(&mut self) -> SDMMC1LPEN_W<16> {
        SDMMC1LPEN_W::new(self)
    }
    ///Bit 19 - OCTOSPI2 and OCTOSPI2 delay clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn octospi2lpen(&mut self) -> OCTOSPI2LPEN_W<19> {
        OCTOSPI2LPEN_W::new(self)
    }
    ///Bit 21 - OCTOSPIM block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn octospimlpen(&mut self) -> OCTOSPIMLPEN_W<21> {
        OCTOSPIMLPEN_W::new(self)
    }
    ///Bit 22 - OTFD1 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn otfd1lpen(&mut self) -> OTFD1LPEN_W<22> {
        OTFD1LPEN_W::new(self)
    }
    ///Bit 23 - OTFD2 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn otfd2lpen(&mut self) -> OTFD2LPEN_W<23> {
        OTFD2LPEN_W::new(self)
    }
    ///Bit 24 - GFXMMU block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gfxmmulpen(&mut self) -> GFXMMULPEN_W<24> {
        GFXMMULPEN_W::new(self)
    }
    ///Bit 26 - AXISRAM2 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn axisram2lpen(&mut self) -> AXISRAM2LPEN_W<26> {
        AXISRAM2LPEN_W::new(self)
    }
    ///Bit 27 - AXISRAM3 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn axisram3lpen(&mut self) -> AXISRAM3LPEN_W<27> {
        AXISRAM3LPEN_W::new(self)
    }
    ///Bit 28 - DTCM1 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn dtcm1lpen(&mut self) -> DTCM1LPEN_W<28> {
        DTCM1LPEN_W::new(self)
    }
    ///Bit 29 - DTCM2 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn dtcm2lpen(&mut self) -> DTCM2LPEN_W<29> {
        DTCM2LPEN_W::new(self)
    }
    ///Bit 30 - ITCM block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn itcmlpen(&mut self) -> ITCMLPEN_W<30> {
        ITCMLPEN_W::new(self)
    }
    ///Bit 31 - AXISRAM1 block clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn axisram1lpen(&mut self) -> AXISRAM1LPEN_W<31> {
        AXISRAM1LPEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///
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
///`reset()` method sets AHB3LPENR to value 0xfde9_5131
impl crate::Resettable for AHB3LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0xfde9_5131;
}
