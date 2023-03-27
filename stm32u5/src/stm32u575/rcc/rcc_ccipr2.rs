///Register `RCC_CCIPR2` reader
pub struct R(crate::R<RCC_CCIPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CCIPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CCIPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CCIPR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_CCIPR2` writer
pub struct W(crate::W<RCC_CCIPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CCIPR2_SPEC>;
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
impl From<crate::W<RCC_CCIPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CCIPR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MDF1SEL` reader - MDF1 kernel clock source selection These bits are used to select the MDF1 kernel clock source. others: reserved
pub type MDF1SEL_R = crate::FieldReader<u8, u8>;
///Field `MDF1SEL` writer - MDF1 kernel clock source selection These bits are used to select the MDF1 kernel clock source. others: reserved
pub type MDF1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR2_SPEC, u8, u8, 3, O>;
///Field `SAI1SEL` reader - SAI1 kernel clock source selection These bits are used to select the SAI1 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible.
pub type SAI1SEL_R = crate::FieldReader<u8, u8>;
///Field `SAI1SEL` writer - SAI1 kernel clock source selection These bits are used to select the SAI1 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible.
pub type SAI1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR2_SPEC, u8, u8, 3, O>;
///Field `SAI2SEL` reader - SAI2 kernel clock source selection These bits are used to select the SAI2 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible.
pub type SAI2SEL_R = crate::FieldReader<u8, u8>;
///Field `SAI2SEL` writer - SAI2 kernel clock source selection These bits are used to select the SAI2 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible.
pub type SAI2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR2_SPEC, u8, u8, 3, O>;
///Field `SAESSEL` reader - SAES kernel clock source selection This bit is used to select the SAES kernel clock source.
pub type SAESSEL_R = crate::BitReader<bool>;
///Field `SAESSEL` writer - SAES kernel clock source selection This bit is used to select the SAES kernel clock source.
pub type SAESSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CCIPR2_SPEC, bool, O>;
///Field `RNGSEL` reader - RNGSEL kernel clock source selection These bits are used to select the RNG kernel clock source.
pub type RNGSEL_R = crate::FieldReader<u8, u8>;
///Field `RNGSEL` writer - RNGSEL kernel clock source selection These bits are used to select the RNG kernel clock source.
pub type RNGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR2_SPEC, u8, u8, 2, O>;
///Field `SDMMCSEL` reader - SDMMC1 and SDMMC2 kernel clock source selection This bit is used to select the SDMMC kernel clock source. It is recommended to change this bit only after reset and before enabling the SDMMC.
pub type SDMMCSEL_R = crate::BitReader<bool>;
///Field `SDMMCSEL` writer - SDMMC1 and SDMMC2 kernel clock source selection This bit is used to select the SDMMC kernel clock source. It is recommended to change this bit only after reset and before enabling the SDMMC.
pub type SDMMCSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_CCIPR2_SPEC, bool, O>;
///Field `OCTOSPISEL` reader - OCTOSPI1 and OCTOSPI2 kernel clock source selection These bits are used to select the OCTOSPI1 and OCTOSPI2 kernel clock source.
pub type OCTOSPISEL_R = crate::FieldReader<u8, u8>;
///Field `OCTOSPISEL` writer - OCTOSPI1 and OCTOSPI2 kernel clock source selection These bits are used to select the OCTOSPI1 and OCTOSPI2 kernel clock source.
pub type OCTOSPISEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CCIPR2_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:2 - MDF1 kernel clock source selection These bits are used to select the MDF1 kernel clock source. others: reserved
    #[inline(always)]
    pub fn mdf1sel(&self) -> MDF1SEL_R {
        MDF1SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 5:7 - SAI1 kernel clock source selection These bits are used to select the SAI1 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible.
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bits 8:10 - SAI2 kernel clock source selection These bits are used to select the SAI2 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible.
    #[inline(always)]
    pub fn sai2sel(&self) -> SAI2SEL_R {
        SAI2SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 11 - SAES kernel clock source selection This bit is used to select the SAES kernel clock source.
    #[inline(always)]
    pub fn saessel(&self) -> SAESSEL_R {
        SAESSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - RNGSEL kernel clock source selection These bits are used to select the RNG kernel clock source.
    #[inline(always)]
    pub fn rngsel(&self) -> RNGSEL_R {
        RNGSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - SDMMC1 and SDMMC2 kernel clock source selection This bit is used to select the SDMMC kernel clock source. It is recommended to change this bit only after reset and before enabling the SDMMC.
    #[inline(always)]
    pub fn sdmmcsel(&self) -> SDMMCSEL_R {
        SDMMCSEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 20:21 - OCTOSPI1 and OCTOSPI2 kernel clock source selection These bits are used to select the OCTOSPI1 and OCTOSPI2 kernel clock source.
    #[inline(always)]
    pub fn octospisel(&self) -> OCTOSPISEL_R {
        OCTOSPISEL_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    ///Bits 0:2 - MDF1 kernel clock source selection These bits are used to select the MDF1 kernel clock source. others: reserved
    #[inline(always)]
    #[must_use]
    pub fn mdf1sel(&mut self) -> MDF1SEL_W<0> {
        MDF1SEL_W::new(self)
    }
    ///Bits 5:7 - SAI1 kernel clock source selection These bits are used to select the SAI1 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible.
    #[inline(always)]
    #[must_use]
    pub fn sai1sel(&mut self) -> SAI1SEL_W<5> {
        SAI1SEL_W::new(self)
    }
    ///Bits 8:10 - SAI2 kernel clock source selection These bits are used to select the SAI2 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible.
    #[inline(always)]
    #[must_use]
    pub fn sai2sel(&mut self) -> SAI2SEL_W<8> {
        SAI2SEL_W::new(self)
    }
    ///Bit 11 - SAES kernel clock source selection This bit is used to select the SAES kernel clock source.
    #[inline(always)]
    #[must_use]
    pub fn saessel(&mut self) -> SAESSEL_W<11> {
        SAESSEL_W::new(self)
    }
    ///Bits 12:13 - RNGSEL kernel clock source selection These bits are used to select the RNG kernel clock source.
    #[inline(always)]
    #[must_use]
    pub fn rngsel(&mut self) -> RNGSEL_W<12> {
        RNGSEL_W::new(self)
    }
    ///Bit 14 - SDMMC1 and SDMMC2 kernel clock source selection This bit is used to select the SDMMC kernel clock source. It is recommended to change this bit only after reset and before enabling the SDMMC.
    #[inline(always)]
    #[must_use]
    pub fn sdmmcsel(&mut self) -> SDMMCSEL_W<14> {
        SDMMCSEL_W::new(self)
    }
    ///Bits 20:21 - OCTOSPI1 and OCTOSPI2 kernel clock source selection These bits are used to select the OCTOSPI1 and OCTOSPI2 kernel clock source.
    #[inline(always)]
    #[must_use]
    pub fn octospisel(&mut self) -> OCTOSPISEL_W<20> {
        OCTOSPISEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC peripherals independent clock configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_ccipr2](index.html) module
pub struct RCC_CCIPR2_SPEC;
impl crate::RegisterSpec for RCC_CCIPR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_ccipr2::R](R) reader structure
impl crate::Readable for RCC_CCIPR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_ccipr2::W](W) writer structure
impl crate::Writable for RCC_CCIPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_CCIPR2 to value 0
impl crate::Resettable for RCC_CCIPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
