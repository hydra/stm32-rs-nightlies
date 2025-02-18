///Register `RCC_MC_AHB3LPENCLRR` reader
pub struct R(crate::R<RCC_MC_AHB3LPENCLRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_AHB3LPENCLRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_AHB3LPENCLRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_AHB3LPENCLRR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_MC_AHB3LPENCLRR` writer
pub struct W(crate::W<RCC_MC_AHB3LPENCLRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_AHB3LPENCLRR_SPEC>;
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
impl From<crate::W<RCC_MC_AHB3LPENCLRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_AHB3LPENCLRR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DCMILPEN` reader - DCMILPEN
pub type DCMILPEN_R = crate::BitReader<bool>;
///Field `DCMILPEN` writer - DCMILPEN
pub type DCMILPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB3LPENCLRR_SPEC, bool, O>;
///Field `CRYP2LPEN` reader - CRYP2LPEN
pub type CRYP2LPEN_R = crate::BitReader<bool>;
///Field `CRYP2LPEN` writer - CRYP2LPEN
pub type CRYP2LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MC_AHB3LPENCLRR_SPEC, bool, O>;
///Field `HASH2LPEN` reader - HASH2LPEN
pub type HASH2LPEN_R = crate::BitReader<bool>;
///Field `HASH2LPEN` writer - HASH2LPEN
pub type HASH2LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MC_AHB3LPENCLRR_SPEC, bool, O>;
///Field `RNG2LPEN` reader - RNG2LPEN
pub type RNG2LPEN_R = crate::BitReader<bool>;
///Field `RNG2LPEN` writer - RNG2LPEN
pub type RNG2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB3LPENCLRR_SPEC, bool, O>;
///Field `CRC2LPEN` reader - CRC2LPEN
pub type CRC2LPEN_R = crate::BitReader<bool>;
///Field `CRC2LPEN` writer - CRC2LPEN
pub type CRC2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB3LPENCLRR_SPEC, bool, O>;
///Field `HSEMLPEN` reader - HSEMLPEN
pub type HSEMLPEN_R = crate::BitReader<bool>;
///Field `HSEMLPEN` writer - HSEMLPEN
pub type HSEMLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB3LPENCLRR_SPEC, bool, O>;
///Field `IPCCLPEN` reader - IPCCLPEN
pub type IPCCLPEN_R = crate::BitReader<bool>;
///Field `IPCCLPEN` writer - IPCCLPEN
pub type IPCCLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB3LPENCLRR_SPEC, bool, O>;
impl R {
    ///Bit 0 - DCMILPEN
    #[inline(always)]
    pub fn dcmilpen(&self) -> DCMILPEN_R {
        DCMILPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - CRYP2LPEN
    #[inline(always)]
    pub fn cryp2lpen(&self) -> CRYP2LPEN_R {
        CRYP2LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HASH2LPEN
    #[inline(always)]
    pub fn hash2lpen(&self) -> HASH2LPEN_R {
        HASH2LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RNG2LPEN
    #[inline(always)]
    pub fn rng2lpen(&self) -> RNG2LPEN_R {
        RNG2LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CRC2LPEN
    #[inline(always)]
    pub fn crc2lpen(&self) -> CRC2LPEN_R {
        CRC2LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - HSEMLPEN
    #[inline(always)]
    pub fn hsemlpen(&self) -> HSEMLPEN_R {
        HSEMLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - IPCCLPEN
    #[inline(always)]
    pub fn ipcclpen(&self) -> IPCCLPEN_R {
        IPCCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DCMILPEN
    #[inline(always)]
    #[must_use]
    pub fn dcmilpen(&mut self) -> DCMILPEN_W<0> {
        DCMILPEN_W::new(self)
    }
    ///Bit 4 - CRYP2LPEN
    #[inline(always)]
    #[must_use]
    pub fn cryp2lpen(&mut self) -> CRYP2LPEN_W<4> {
        CRYP2LPEN_W::new(self)
    }
    ///Bit 5 - HASH2LPEN
    #[inline(always)]
    #[must_use]
    pub fn hash2lpen(&mut self) -> HASH2LPEN_W<5> {
        HASH2LPEN_W::new(self)
    }
    ///Bit 6 - RNG2LPEN
    #[inline(always)]
    #[must_use]
    pub fn rng2lpen(&mut self) -> RNG2LPEN_W<6> {
        RNG2LPEN_W::new(self)
    }
    ///Bit 7 - CRC2LPEN
    #[inline(always)]
    #[must_use]
    pub fn crc2lpen(&mut self) -> CRC2LPEN_W<7> {
        CRC2LPEN_W::new(self)
    }
    ///Bit 11 - HSEMLPEN
    #[inline(always)]
    #[must_use]
    pub fn hsemlpen(&mut self) -> HSEMLPEN_W<11> {
        HSEMLPEN_W::new(self)
    }
    ///Bit 12 - IPCCLPEN
    #[inline(always)]
    #[must_use]
    pub fn ipcclpen(&mut self) -> IPCCLPEN_W<12> {
        IPCCLPEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used by the MCU in order to clear the PERxLPEN bit
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_mc_ahb3lpenclrr](index.html) module
pub struct RCC_MC_AHB3LPENCLRR_SPEC;
impl crate::RegisterSpec for RCC_MC_AHB3LPENCLRR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_mc_ahb3lpenclrr::R](R) reader structure
impl crate::Readable for RCC_MC_AHB3LPENCLRR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_mc_ahb3lpenclrr::W](W) writer structure
impl crate::Writable for RCC_MC_AHB3LPENCLRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_MC_AHB3LPENCLRR to value 0x18f1
impl crate::Resettable for RCC_MC_AHB3LPENCLRR_SPEC {
    const RESET_VALUE: Self::Ux = 0x18f1;
}
