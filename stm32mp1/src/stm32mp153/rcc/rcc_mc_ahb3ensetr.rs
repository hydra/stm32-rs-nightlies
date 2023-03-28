///Register `RCC_MC_AHB3ENSETR` reader
pub struct R(crate::R<RCC_MC_AHB3ENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MC_AHB3ENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MC_AHB3ENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MC_AHB3ENSETR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_MC_AHB3ENSETR` writer
pub struct W(crate::W<RCC_MC_AHB3ENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MC_AHB3ENSETR_SPEC>;
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
impl From<crate::W<RCC_MC_AHB3ENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MC_AHB3ENSETR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DCMIEN` reader - DCMIEN
pub type DCMIEN_R = crate::BitReader<bool>;
///Field `DCMIEN` writer - DCMIEN
pub type DCMIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB3ENSETR_SPEC, bool, O>;
///Field `CRYP2EN` reader - CRYP2EN
pub type CRYP2EN_R = crate::BitReader<bool>;
///Field `CRYP2EN` writer - CRYP2EN
pub type CRYP2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB3ENSETR_SPEC, bool, O>;
///Field `HASH2EN` reader - HASH2EN
pub type HASH2EN_R = crate::BitReader<bool>;
///Field `HASH2EN` writer - HASH2EN
pub type HASH2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB3ENSETR_SPEC, bool, O>;
///Field `RNG2EN` reader - RNG2EN
pub type RNG2EN_R = crate::BitReader<bool>;
///Field `RNG2EN` writer - RNG2EN
pub type RNG2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB3ENSETR_SPEC, bool, O>;
///Field `CRC2EN` reader - CRC2EN
pub type CRC2EN_R = crate::BitReader<bool>;
///Field `CRC2EN` writer - CRC2EN
pub type CRC2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB3ENSETR_SPEC, bool, O>;
///Field `HSEMEN` reader - HSEMEN
pub type HSEMEN_R = crate::BitReader<bool>;
///Field `HSEMEN` writer - HSEMEN
pub type HSEMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB3ENSETR_SPEC, bool, O>;
///Field `IPCCEN` reader - IPCCEN
pub type IPCCEN_R = crate::BitReader<bool>;
///Field `IPCCEN` writer - IPCCEN
pub type IPCCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MC_AHB3ENSETR_SPEC, bool, O>;
impl R {
    ///Bit 0 - DCMIEN
    #[inline(always)]
    pub fn dcmien(&self) -> DCMIEN_R {
        DCMIEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 4 - CRYP2EN
    #[inline(always)]
    pub fn cryp2en(&self) -> CRYP2EN_R {
        CRYP2EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HASH2EN
    #[inline(always)]
    pub fn hash2en(&self) -> HASH2EN_R {
        HASH2EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RNG2EN
    #[inline(always)]
    pub fn rng2en(&self) -> RNG2EN_R {
        RNG2EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CRC2EN
    #[inline(always)]
    pub fn crc2en(&self) -> CRC2EN_R {
        CRC2EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - HSEMEN
    #[inline(always)]
    pub fn hsemen(&self) -> HSEMEN_R {
        HSEMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - IPCCEN
    #[inline(always)]
    pub fn ipccen(&self) -> IPCCEN_R {
        IPCCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DCMIEN
    #[inline(always)]
    #[must_use]
    pub fn dcmien(&mut self) -> DCMIEN_W<0> {
        DCMIEN_W::new(self)
    }
    ///Bit 4 - CRYP2EN
    #[inline(always)]
    #[must_use]
    pub fn cryp2en(&mut self) -> CRYP2EN_W<4> {
        CRYP2EN_W::new(self)
    }
    ///Bit 5 - HASH2EN
    #[inline(always)]
    #[must_use]
    pub fn hash2en(&mut self) -> HASH2EN_W<5> {
        HASH2EN_W::new(self)
    }
    ///Bit 6 - RNG2EN
    #[inline(always)]
    #[must_use]
    pub fn rng2en(&mut self) -> RNG2EN_W<6> {
        RNG2EN_W::new(self)
    }
    ///Bit 7 - CRC2EN
    #[inline(always)]
    #[must_use]
    pub fn crc2en(&mut self) -> CRC2EN_W<7> {
        CRC2EN_W::new(self)
    }
    ///Bit 11 - HSEMEN
    #[inline(always)]
    #[must_use]
    pub fn hsemen(&mut self) -> HSEMEN_W<11> {
        HSEMEN_W::new(self)
    }
    ///Bit 12 - IPCCEN
    #[inline(always)]
    #[must_use]
    pub fn ipccen(&mut self) -> IPCCEN_W<12> {
        IPCCEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to set the peripheral clock enable bit
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_mc_ahb3ensetr](index.html) module
pub struct RCC_MC_AHB3ENSETR_SPEC;
impl crate::RegisterSpec for RCC_MC_AHB3ENSETR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_mc_ahb3ensetr::R](R) reader structure
impl crate::Readable for RCC_MC_AHB3ENSETR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_mc_ahb3ensetr::W](W) writer structure
impl crate::Writable for RCC_MC_AHB3ENSETR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_MC_AHB3ENSETR to value 0
impl crate::Resettable for RCC_MC_AHB3ENSETR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
