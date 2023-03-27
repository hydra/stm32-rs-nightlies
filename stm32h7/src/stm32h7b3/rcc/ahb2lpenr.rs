///Register `AHB2LPENR` reader
pub struct R(crate::R<AHB2LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2LPENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB2LPENR` writer
pub struct W(crate::W<AHB2LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2LPENR_SPEC>;
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
impl From<crate::W<AHB2LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2LPENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DCMI_PSSILPEN` reader - digital camera interface peripheral clock enable during CSleep mode (DCMI or PSSI depending which IP is active) Set and reset by software.
pub type DCMI_PSSILPEN_R = crate::BitReader<DCMI_PSSILPEN_A>;
///digital camera interface peripheral clock enable during CSleep mode (DCMI or PSSI depending which IP is active) Set and reset by software.
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMI_PSSILPEN_A {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<DCMI_PSSILPEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCMI_PSSILPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DCMI_PSSILPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DCMI_PSSILPEN_A {
        match self.bits {
            false => DCMI_PSSILPEN_A::Disabled,
            true => DCMI_PSSILPEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCMI_PSSILPEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCMI_PSSILPEN_A::Enabled
    }
}
///Field `DCMI_PSSILPEN` writer - digital camera interface peripheral clock enable during CSleep mode (DCMI or PSSI depending which IP is active) Set and reset by software.
pub type DCMI_PSSILPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AHB2LPENR_SPEC, DCMI_PSSILPEN_A, O>;
impl<'a, const O: u8> DCMI_PSSILPEN_W<'a, O> {
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCMI_PSSILPEN_A::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCMI_PSSILPEN_A::Enabled)
    }
}
///Field `CRYPTLPEN` reader - CRYPT peripheral clock enable during CSleep mode Set and reset by software.
pub use DCMI_PSSILPEN_R as CRYPTLPEN_R;
///Field `HASHLPEN` reader - HASH peripheral clock enable during CSleep mode Set and reset by software.
pub use DCMI_PSSILPEN_R as HASHLPEN_R;
///Field `RNGLPEN` reader - RNG peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock.
pub use DCMI_PSSILPEN_R as RNGLPEN_R;
///Field `SDMMC2LPEN` reader - SDMMC2 and SDMMC2 delay clock enable during CSleep mode Set and reset by software.
pub use DCMI_PSSILPEN_R as SDMMC2LPEN_R;
///Field `DFSDMDMALPEN` reader - DFSDMDMA clock enable during CSleep mode Set and reset by software.
pub use DCMI_PSSILPEN_R as DFSDMDMALPEN_R;
///Field `AHBSRAM1LPEN` reader - AHBSRAM1 clock enable during CSleep mode Set and reset by software.
pub use DCMI_PSSILPEN_R as AHBSRAM1LPEN_R;
///Field `AHBSRAM2LPEN` reader - AHBSRAM2 clock enable during CSleep mode Set and reset by software.
pub use DCMI_PSSILPEN_R as AHBSRAM2LPEN_R;
///Field `CRYPTLPEN` writer - CRYPT peripheral clock enable during CSleep mode Set and reset by software.
pub use DCMI_PSSILPEN_W as CRYPTLPEN_W;
///Field `HASHLPEN` writer - HASH peripheral clock enable during CSleep mode Set and reset by software.
pub use DCMI_PSSILPEN_W as HASHLPEN_W;
///Field `RNGLPEN` writer - RNG peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock.
pub use DCMI_PSSILPEN_W as RNGLPEN_W;
///Field `SDMMC2LPEN` writer - SDMMC2 and SDMMC2 delay clock enable during CSleep mode Set and reset by software.
pub use DCMI_PSSILPEN_W as SDMMC2LPEN_W;
///Field `DFSDMDMALPEN` writer - DFSDMDMA clock enable during CSleep mode Set and reset by software.
pub use DCMI_PSSILPEN_W as DFSDMDMALPEN_W;
///Field `AHBSRAM1LPEN` writer - AHBSRAM1 clock enable during CSleep mode Set and reset by software.
pub use DCMI_PSSILPEN_W as AHBSRAM1LPEN_W;
///Field `AHBSRAM2LPEN` writer - AHBSRAM2 clock enable during CSleep mode Set and reset by software.
pub use DCMI_PSSILPEN_W as AHBSRAM2LPEN_W;
impl R {
    ///Bit 0 - digital camera interface peripheral clock enable during CSleep mode (DCMI or PSSI depending which IP is active) Set and reset by software.
    #[inline(always)]
    pub fn dcmi_pssilpen(&self) -> DCMI_PSSILPEN_R {
        DCMI_PSSILPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - CRYPT peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn cryptlpen(&self) -> CRYPTLPEN_R {
        CRYPTLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HASH peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn hashlpen(&self) -> HASHLPEN_R {
        HASHLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RNG peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock.
    #[inline(always)]
    pub fn rnglpen(&self) -> RNGLPEN_R {
        RNGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - SDMMC2 and SDMMC2 delay clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn sdmmc2lpen(&self) -> SDMMC2LPEN_R {
        SDMMC2LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - DFSDMDMA clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn dfsdmdmalpen(&self) -> DFSDMDMALPEN_R {
        DFSDMDMALPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 29 - AHBSRAM1 clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn ahbsram1lpen(&self) -> AHBSRAM1LPEN_R {
        AHBSRAM1LPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - AHBSRAM2 clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn ahbsram2lpen(&self) -> AHBSRAM2LPEN_R {
        AHBSRAM2LPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - digital camera interface peripheral clock enable during CSleep mode (DCMI or PSSI depending which IP is active) Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn dcmi_pssilpen(&mut self) -> DCMI_PSSILPEN_W<0> {
        DCMI_PSSILPEN_W::new(self)
    }
    ///Bit 4 - CRYPT peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn cryptlpen(&mut self) -> CRYPTLPEN_W<4> {
        CRYPTLPEN_W::new(self)
    }
    ///Bit 5 - HASH peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn hashlpen(&mut self) -> HASHLPEN_W<5> {
        HASHLPEN_W::new(self)
    }
    ///Bit 6 - RNG peripheral clock enable during CSleep mode Set and reset by software. The peripheral clocks of the RNG are the kernel clock selected by RNGSEL and provided to rng_clk input, and the rcc_hclk2 bus interface clock.
    #[inline(always)]
    #[must_use]
    pub fn rnglpen(&mut self) -> RNGLPEN_W<6> {
        RNGLPEN_W::new(self)
    }
    ///Bit 9 - SDMMC2 and SDMMC2 delay clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2lpen(&mut self) -> SDMMC2LPEN_W<9> {
        SDMMC2LPEN_W::new(self)
    }
    ///Bit 11 - DFSDMDMA clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn dfsdmdmalpen(&mut self) -> DFSDMDMALPEN_W<11> {
        DFSDMDMALPEN_W::new(self)
    }
    ///Bit 29 - AHBSRAM1 clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn ahbsram1lpen(&mut self) -> AHBSRAM1LPEN_W<29> {
        AHBSRAM1LPEN_W::new(self)
    }
    ///Bit 30 - AHBSRAM2 clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn ahbsram2lpen(&mut self) -> AHBSRAM2LPEN_W<30> {
        AHBSRAM2LPEN_W::new(self)
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
///For information about available fields see [ahb2lpenr](index.html) module
pub struct AHB2LPENR_SPEC;
impl crate::RegisterSpec for AHB2LPENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb2lpenr::R](R) reader structure
impl crate::Readable for AHB2LPENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb2lpenr::W](W) writer structure
impl crate::Writable for AHB2LPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHB2LPENR to value 0x6000_0a71
impl crate::Resettable for AHB2LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x6000_0a71;
}
