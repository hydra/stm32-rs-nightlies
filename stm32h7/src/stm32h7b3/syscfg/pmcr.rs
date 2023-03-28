///Register `PMCR` reader
pub struct R(crate::R<PMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PMCR` writer
pub struct W(crate::W<PMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMCR_SPEC>;
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
impl From<crate::W<PMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `I2C1FMP` reader - I2C1 Fm+
pub type I2C1FMP_R = crate::BitReader<bool>;
///Field `I2C1FMP` writer - I2C1 Fm+
pub type I2C1FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCR_SPEC, bool, O>;
///Field `I2C2FMP` reader - I2C2 Fm+
pub type I2C2FMP_R = crate::BitReader<bool>;
///Field `I2C2FMP` writer - I2C2 Fm+
pub type I2C2FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCR_SPEC, bool, O>;
///Field `I2C3FMP` reader - I2C3 Fm+
pub type I2C3FMP_R = crate::BitReader<bool>;
///Field `I2C3FMP` writer - I2C3 Fm+
pub type I2C3FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCR_SPEC, bool, O>;
///Field `I2C4FMP` reader - I2C4 Fm+
pub type I2C4FMP_R = crate::BitReader<bool>;
///Field `I2C4FMP` writer - I2C4 Fm+
pub type I2C4FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCR_SPEC, bool, O>;
///Field `PB6FMP` reader - PB(6) Fm+
pub type PB6FMP_R = crate::BitReader<bool>;
///Field `PB6FMP` writer - PB(6) Fm+
pub type PB6FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCR_SPEC, bool, O>;
///Field `PB7FMP` reader - PB(7) Fast Mode Plus
pub type PB7FMP_R = crate::BitReader<bool>;
///Field `PB7FMP` writer - PB(7) Fast Mode Plus
pub type PB7FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCR_SPEC, bool, O>;
///Field `PB8FMP` reader - PB(8) Fast Mode Plus
pub type PB8FMP_R = crate::BitReader<bool>;
///Field `PB8FMP` writer - PB(8) Fast Mode Plus
pub type PB8FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCR_SPEC, bool, O>;
///Field `PB9FMP` reader - PB(9) Fm+
pub type PB9FMP_R = crate::BitReader<bool>;
///Field `PB9FMP` writer - PB(9) Fm+
pub type PB9FMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCR_SPEC, bool, O>;
///Field `PA0SO` reader - PA0 Switch Open
pub type PA0SO_R = crate::BitReader<bool>;
///Field `PA0SO` writer - PA0 Switch Open
pub type PA0SO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCR_SPEC, bool, O>;
///Field `PA1SO` reader - PA1 Switch Open
pub type PA1SO_R = crate::BitReader<bool>;
///Field `PA1SO` writer - PA1 Switch Open
pub type PA1SO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCR_SPEC, bool, O>;
///Field `PC2SO` reader - PC2 Switch Open
pub type PC2SO_R = crate::BitReader<bool>;
///Field `PC2SO` writer - PC2 Switch Open
pub type PC2SO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCR_SPEC, bool, O>;
///Field `PC3SO` reader - PC3 Switch Open
pub type PC3SO_R = crate::BitReader<bool>;
///Field `PC3SO` writer - PC3 Switch Open
pub type PC3SO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - I2C1 Fm+
    #[inline(always)]
    pub fn i2c1fmp(&self) -> I2C1FMP_R {
        I2C1FMP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - I2C2 Fm+
    #[inline(always)]
    pub fn i2c2fmp(&self) -> I2C2FMP_R {
        I2C2FMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - I2C3 Fm+
    #[inline(always)]
    pub fn i2c3fmp(&self) -> I2C3FMP_R {
        I2C3FMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - I2C4 Fm+
    #[inline(always)]
    pub fn i2c4fmp(&self) -> I2C4FMP_R {
        I2C4FMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PB(6) Fm+
    #[inline(always)]
    pub fn pb6fmp(&self) -> PB6FMP_R {
        PB6FMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PB(7) Fast Mode Plus
    #[inline(always)]
    pub fn pb7fmp(&self) -> PB7FMP_R {
        PB7FMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PB(8) Fast Mode Plus
    #[inline(always)]
    pub fn pb8fmp(&self) -> PB8FMP_R {
        PB8FMP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PB(9) Fm+
    #[inline(always)]
    pub fn pb9fmp(&self) -> PB9FMP_R {
        PB9FMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 24 - PA0 Switch Open
    #[inline(always)]
    pub fn pa0so(&self) -> PA0SO_R {
        PA0SO_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - PA1 Switch Open
    #[inline(always)]
    pub fn pa1so(&self) -> PA1SO_R {
        PA1SO_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - PC2 Switch Open
    #[inline(always)]
    pub fn pc2so(&self) -> PC2SO_R {
        PC2SO_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - PC3 Switch Open
    #[inline(always)]
    pub fn pc3so(&self) -> PC3SO_R {
        PC3SO_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - I2C1 Fm+
    #[inline(always)]
    #[must_use]
    pub fn i2c1fmp(&mut self) -> I2C1FMP_W<0> {
        I2C1FMP_W::new(self)
    }
    ///Bit 1 - I2C2 Fm+
    #[inline(always)]
    #[must_use]
    pub fn i2c2fmp(&mut self) -> I2C2FMP_W<1> {
        I2C2FMP_W::new(self)
    }
    ///Bit 2 - I2C3 Fm+
    #[inline(always)]
    #[must_use]
    pub fn i2c3fmp(&mut self) -> I2C3FMP_W<2> {
        I2C3FMP_W::new(self)
    }
    ///Bit 3 - I2C4 Fm+
    #[inline(always)]
    #[must_use]
    pub fn i2c4fmp(&mut self) -> I2C4FMP_W<3> {
        I2C4FMP_W::new(self)
    }
    ///Bit 4 - PB(6) Fm+
    #[inline(always)]
    #[must_use]
    pub fn pb6fmp(&mut self) -> PB6FMP_W<4> {
        PB6FMP_W::new(self)
    }
    ///Bit 5 - PB(7) Fast Mode Plus
    #[inline(always)]
    #[must_use]
    pub fn pb7fmp(&mut self) -> PB7FMP_W<5> {
        PB7FMP_W::new(self)
    }
    ///Bit 6 - PB(8) Fast Mode Plus
    #[inline(always)]
    #[must_use]
    pub fn pb8fmp(&mut self) -> PB8FMP_W<6> {
        PB8FMP_W::new(self)
    }
    ///Bit 7 - PB(9) Fm+
    #[inline(always)]
    #[must_use]
    pub fn pb9fmp(&mut self) -> PB9FMP_W<7> {
        PB9FMP_W::new(self)
    }
    ///Bit 24 - PA0 Switch Open
    #[inline(always)]
    #[must_use]
    pub fn pa0so(&mut self) -> PA0SO_W<24> {
        PA0SO_W::new(self)
    }
    ///Bit 25 - PA1 Switch Open
    #[inline(always)]
    #[must_use]
    pub fn pa1so(&mut self) -> PA1SO_W<25> {
        PA1SO_W::new(self)
    }
    ///Bit 26 - PC2 Switch Open
    #[inline(always)]
    #[must_use]
    pub fn pc2so(&mut self) -> PC2SO_W<26> {
        PC2SO_W::new(self)
    }
    ///Bit 27 - PC3 Switch Open
    #[inline(always)]
    #[must_use]
    pub fn pc3so(&mut self) -> PC3SO_W<27> {
        PC3SO_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///peripheral mode configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pmcr](index.html) module
pub struct PMCR_SPEC;
impl crate::RegisterSpec for PMCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pmcr::R](R) reader structure
impl crate::Readable for PMCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pmcr::W](W) writer structure
impl crate::Writable for PMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PMCR to value 0
impl crate::Resettable for PMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
