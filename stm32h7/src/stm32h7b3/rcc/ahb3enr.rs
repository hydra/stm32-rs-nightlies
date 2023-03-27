///Register `AHB3ENR` reader
pub struct R(crate::R<AHB3ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB3ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB3ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB3ENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB3ENR` writer
pub struct W(crate::W<AHB3ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB3ENR_SPEC>;
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
impl From<crate::W<AHB3ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB3ENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MDMAEN` reader - MDMA peripheral clock enable Set and reset by software.
pub type MDMAEN_R = crate::BitReader<MDMAEN_A>;
///MDMA peripheral clock enable Set and reset by software.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDMAEN_A {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<MDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: MDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MDMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MDMAEN_A {
        match self.bits {
            false => MDMAEN_A::Disabled,
            true => MDMAEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MDMAEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MDMAEN_A::Enabled
    }
}
///Field `MDMAEN` writer - MDMA peripheral clock enable Set and reset by software.
pub type MDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB3ENR_SPEC, MDMAEN_A, O>;
impl<'a, const O: u8> MDMAEN_W<'a, O> {
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MDMAEN_A::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MDMAEN_A::Enabled)
    }
}
///Field `DMA2DEN` reader - DMA2D peripheral clock enable Set and reset by software.
pub use MDMAEN_R as DMA2DEN_R;
///Field `JPGDECEN` reader - JPGDEC peripheral clock enable Set and reset by software.
pub use MDMAEN_R as JPGDECEN_R;
///Field `FMCEN` reader - FMC peripheral clocks enable Set and reset by software. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL and provided to fmc_ker_ck input, and the rcc_hclk3 bus interface clock.
pub use MDMAEN_R as FMCEN_R;
///Field `OCTOSPI1EN` reader - OCTOSPI1 and OCTOSPI1 delay clock enable Set and reset by software.
pub use MDMAEN_R as OCTOSPI1EN_R;
///Field `SDMMC1EN` reader - SDMMC1 and SDMMC1 delay clock enable Set and reset by software.
pub use MDMAEN_R as SDMMC1EN_R;
///Field `OCTOSPI2EN` reader - OCTOSPI2 clock enable Set and reset by software.
pub use MDMAEN_R as OCTOSPI2EN_R;
///Field `OCTOSPIMEN` reader - OCTOSPIM clock enable Set and reset by software.
pub use MDMAEN_R as OCTOSPIMEN_R;
///Field `OTFD1EN` reader - OTFD1 clock enable Set and reset by software.
pub use MDMAEN_R as OTFD1EN_R;
///Field `OTFD2EN` reader - OTFD2 clock enable Set and reset by software.
pub use MDMAEN_R as OTFD2EN_R;
///Field `GFXMMUEN` reader - GFXMMU clock enable Set and reset by software.
pub use MDMAEN_R as GFXMMUEN_R;
///Field `DTCM1EN` reader - D1 DTCM1 block enable
pub use MDMAEN_R as DTCM1EN_R;
///Field `DTCM2EN` reader - D1 DTCM2 block enable
pub use MDMAEN_R as DTCM2EN_R;
///Field `ITCM1EN` reader - D1 ITCM block enable
pub use MDMAEN_R as ITCM1EN_R;
///Field `AXISRAMEN` reader - AXISRAM block enable
pub use MDMAEN_R as AXISRAMEN_R;
///Field `DMA2DEN` writer - DMA2D peripheral clock enable Set and reset by software.
pub use MDMAEN_W as DMA2DEN_W;
///Field `JPGDECEN` writer - JPGDEC peripheral clock enable Set and reset by software.
pub use MDMAEN_W as JPGDECEN_W;
///Field `FMCEN` writer - FMC peripheral clocks enable Set and reset by software. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL and provided to fmc_ker_ck input, and the rcc_hclk3 bus interface clock.
pub use MDMAEN_W as FMCEN_W;
///Field `OCTOSPI1EN` writer - OCTOSPI1 and OCTOSPI1 delay clock enable Set and reset by software.
pub use MDMAEN_W as OCTOSPI1EN_W;
///Field `SDMMC1EN` writer - SDMMC1 and SDMMC1 delay clock enable Set and reset by software.
pub use MDMAEN_W as SDMMC1EN_W;
///Field `OCTOSPI2EN` writer - OCTOSPI2 clock enable Set and reset by software.
pub use MDMAEN_W as OCTOSPI2EN_W;
///Field `OCTOSPIMEN` writer - OCTOSPIM clock enable Set and reset by software.
pub use MDMAEN_W as OCTOSPIMEN_W;
///Field `OTFD1EN` writer - OTFD1 clock enable Set and reset by software.
pub use MDMAEN_W as OTFD1EN_W;
///Field `OTFD2EN` writer - OTFD2 clock enable Set and reset by software.
pub use MDMAEN_W as OTFD2EN_W;
///Field `GFXMMUEN` writer - GFXMMU clock enable Set and reset by software.
pub use MDMAEN_W as GFXMMUEN_W;
///Field `DTCM1EN` writer - D1 DTCM1 block enable
pub use MDMAEN_W as DTCM1EN_W;
///Field `DTCM2EN` writer - D1 DTCM2 block enable
pub use MDMAEN_W as DTCM2EN_W;
///Field `ITCM1EN` writer - D1 ITCM block enable
pub use MDMAEN_W as ITCM1EN_W;
///Field `AXISRAMEN` writer - AXISRAM block enable
pub use MDMAEN_W as AXISRAMEN_W;
impl R {
    ///Bit 0 - MDMA peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn mdmaen(&self) -> MDMAEN_R {
        MDMAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - DMA2D peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn dma2den(&self) -> DMA2DEN_R {
        DMA2DEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - JPGDEC peripheral clock enable Set and reset by software.
    #[inline(always)]
    pub fn jpgdecen(&self) -> JPGDECEN_R {
        JPGDECEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 12 - FMC peripheral clocks enable Set and reset by software. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL and provided to fmc_ker_ck input, and the rcc_hclk3 bus interface clock.
    #[inline(always)]
    pub fn fmcen(&self) -> FMCEN_R {
        FMCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - OCTOSPI1 and OCTOSPI1 delay clock enable Set and reset by software.
    #[inline(always)]
    pub fn octospi1en(&self) -> OCTOSPI1EN_R {
        OCTOSPI1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - SDMMC1 and SDMMC1 delay clock enable Set and reset by software.
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 19 - OCTOSPI2 clock enable Set and reset by software.
    #[inline(always)]
    pub fn octospi2en(&self) -> OCTOSPI2EN_R {
        OCTOSPI2EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - OCTOSPIM clock enable Set and reset by software.
    #[inline(always)]
    pub fn octospimen(&self) -> OCTOSPIMEN_R {
        OCTOSPIMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - OTFD1 clock enable Set and reset by software.
    #[inline(always)]
    pub fn otfd1en(&self) -> OTFD1EN_R {
        OTFD1EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - OTFD2 clock enable Set and reset by software.
    #[inline(always)]
    pub fn otfd2en(&self) -> OTFD2EN_R {
        OTFD2EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - GFXMMU clock enable Set and reset by software.
    #[inline(always)]
    pub fn gfxmmuen(&self) -> GFXMMUEN_R {
        GFXMMUEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - D1 DTCM1 block enable
    #[inline(always)]
    pub fn dtcm1en(&self) -> DTCM1EN_R {
        DTCM1EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - D1 DTCM2 block enable
    #[inline(always)]
    pub fn dtcm2en(&self) -> DTCM2EN_R {
        DTCM2EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - D1 ITCM block enable
    #[inline(always)]
    pub fn itcm1en(&self) -> ITCM1EN_R {
        ITCM1EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - AXISRAM block enable
    #[inline(always)]
    pub fn axisramen(&self) -> AXISRAMEN_R {
        AXISRAMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - MDMA peripheral clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn mdmaen(&mut self) -> MDMAEN_W<0> {
        MDMAEN_W::new(self)
    }
    ///Bit 4 - DMA2D peripheral clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn dma2den(&mut self) -> DMA2DEN_W<4> {
        DMA2DEN_W::new(self)
    }
    ///Bit 5 - JPGDEC peripheral clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn jpgdecen(&mut self) -> JPGDECEN_W<5> {
        JPGDECEN_W::new(self)
    }
    ///Bit 12 - FMC peripheral clocks enable Set and reset by software. The peripheral clocks of the FMC are the kernel clock selected by FMCSEL and provided to fmc_ker_ck input, and the rcc_hclk3 bus interface clock.
    #[inline(always)]
    #[must_use]
    pub fn fmcen(&mut self) -> FMCEN_W<12> {
        FMCEN_W::new(self)
    }
    ///Bit 14 - OCTOSPI1 and OCTOSPI1 delay clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn octospi1en(&mut self) -> OCTOSPI1EN_W<14> {
        OCTOSPI1EN_W::new(self)
    }
    ///Bit 16 - SDMMC1 and SDMMC1 delay clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<16> {
        SDMMC1EN_W::new(self)
    }
    ///Bit 19 - OCTOSPI2 clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn octospi2en(&mut self) -> OCTOSPI2EN_W<19> {
        OCTOSPI2EN_W::new(self)
    }
    ///Bit 21 - OCTOSPIM clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn octospimen(&mut self) -> OCTOSPIMEN_W<21> {
        OCTOSPIMEN_W::new(self)
    }
    ///Bit 22 - OTFD1 clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn otfd1en(&mut self) -> OTFD1EN_W<22> {
        OTFD1EN_W::new(self)
    }
    ///Bit 23 - OTFD2 clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn otfd2en(&mut self) -> OTFD2EN_W<23> {
        OTFD2EN_W::new(self)
    }
    ///Bit 24 - GFXMMU clock enable Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn gfxmmuen(&mut self) -> GFXMMUEN_W<24> {
        GFXMMUEN_W::new(self)
    }
    ///Bit 28 - D1 DTCM1 block enable
    #[inline(always)]
    #[must_use]
    pub fn dtcm1en(&mut self) -> DTCM1EN_W<28> {
        DTCM1EN_W::new(self)
    }
    ///Bit 29 - D1 DTCM2 block enable
    #[inline(always)]
    #[must_use]
    pub fn dtcm2en(&mut self) -> DTCM2EN_W<29> {
        DTCM2EN_W::new(self)
    }
    ///Bit 30 - D1 ITCM block enable
    #[inline(always)]
    #[must_use]
    pub fn itcm1en(&mut self) -> ITCM1EN_W<30> {
        ITCM1EN_W::new(self)
    }
    ///Bit 31 - AXISRAM block enable
    #[inline(always)]
    #[must_use]
    pub fn axisramen(&mut self) -> AXISRAMEN_W<31> {
        AXISRAMEN_W::new(self)
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
///For information about available fields see [ahb3enr](index.html) module
pub struct AHB3ENR_SPEC;
impl crate::RegisterSpec for AHB3ENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb3enr::R](R) reader structure
impl crate::Readable for AHB3ENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb3enr::W](W) writer structure
impl crate::Writable for AHB3ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHB3ENR to value 0
impl crate::Resettable for AHB3ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
