///Register `RCC_MC_AHB5LPENCLRR` reader
pub struct R(crate::R<RCC_MC_AHB5LPENCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_AHB5LPENCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_AHB5LPENCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_AHB5LPENCLRR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_MC_AHB5LPENCLRR` writer
pub struct W(crate::W<RCC_MC_AHB5LPENCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_AHB5LPENCLRR_SPEC>;
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
impl From<crate::W<RCC_MC_AHB5LPENCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_AHB5LPENCLRR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GPIOZLPEN` reader - GPIOZLPEN
pub type GPIOZLPEN_R = crate::BitReader<bool>;
///Field `GPIOZLPEN` writer - GPIOZLPEN
pub type GPIOZLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MC_AHB5LPENCLRR_SPEC, bool, O>;
///Field `CRYP1LPEN` reader - CRYP1LPEN
pub type CRYP1LPEN_R = crate::BitReader<bool>;
///Field `CRYP1LPEN` writer - CRYP1LPEN
pub type CRYP1LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MC_AHB5LPENCLRR_SPEC, bool, O>;
///Field `HASH1LPEN` reader - HASH1LPEN
pub type HASH1LPEN_R = crate::BitReader<bool>;
///Field `HASH1LPEN` writer - HASH1LPEN
pub type HASH1LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MC_AHB5LPENCLRR_SPEC, bool, O>;
///Field `RNG1LPEN` reader - RNG1LPEN
pub type RNG1LPEN_R = crate::BitReader<bool>;
///Field `RNG1LPEN` writer - RNG1LPEN
pub type RNG1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB5LPENCLRR_SPEC, bool, O>;
///Field `BKPSRAMLPEN` reader - BKPSRAMLPEN
pub type BKPSRAMLPEN_R = crate::BitReader<bool>;
///Field `BKPSRAMLPEN` writer - BKPSRAMLPEN
pub type BKPSRAMLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MC_AHB5LPENCLRR_SPEC, bool, O>;
impl R {
    ///Bit 0 - GPIOZLPEN
    #[inline(always)]
    pub fn gpiozlpen(&self) -> GPIOZLPEN_R {
        GPIOZLPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - CRYP1LPEN
    #[inline(always)]
    pub fn cryp1lpen(&self) -> CRYP1LPEN_R {
        CRYP1LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HASH1LPEN
    #[inline(always)]
    pub fn hash1lpen(&self) -> HASH1LPEN_R {
        HASH1LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RNG1LPEN
    #[inline(always)]
    pub fn rng1lpen(&self) -> RNG1LPEN_R {
        RNG1LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - BKPSRAMLPEN
    #[inline(always)]
    pub fn bkpsramlpen(&self) -> BKPSRAMLPEN_R {
        BKPSRAMLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - GPIOZLPEN
    #[inline(always)]
    #[must_use]
    pub fn gpiozlpen(&mut self) -> GPIOZLPEN_W<0> {
        GPIOZLPEN_W::new(self)
    }
    ///Bit 4 - CRYP1LPEN
    #[inline(always)]
    #[must_use]
    pub fn cryp1lpen(&mut self) -> CRYP1LPEN_W<4> {
        CRYP1LPEN_W::new(self)
    }
    ///Bit 5 - HASH1LPEN
    #[inline(always)]
    #[must_use]
    pub fn hash1lpen(&mut self) -> HASH1LPEN_W<5> {
        HASH1LPEN_W::new(self)
    }
    ///Bit 6 - RNG1LPEN
    #[inline(always)]
    #[must_use]
    pub fn rng1lpen(&mut self) -> RNG1LPEN_W<6> {
        RNG1LPEN_W::new(self)
    }
    ///Bit 8 - BKPSRAMLPEN
    #[inline(always)]
    #[must_use]
    pub fn bkpsramlpen(&mut self) -> BKPSRAMLPEN_W<8> {
        BKPSRAMLPEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used by the MCU in order to clear the PERxLPEN bit If TZEN = , this register can only be modified in secure mode.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_mc_ahb5lpenclrr](index.html) module
pub struct RCC_MC_AHB5LPENCLRR_SPEC;
impl crate::RegisterSpec for RCC_MC_AHB5LPENCLRR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_mc_ahb5lpenclrr::R](R) reader structure
impl crate::Readable for RCC_MC_AHB5LPENCLRR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_mc_ahb5lpenclrr::W](W) writer structure
impl crate::Writable for RCC_MC_AHB5LPENCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_MC_AHB5LPENCLRR to value 0x0171
impl crate::Resettable for RCC_MC_AHB5LPENCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0171;
}
