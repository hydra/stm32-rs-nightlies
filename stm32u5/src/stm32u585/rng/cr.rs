///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RNGEN` reader - True random number generator enable
pub type RNGEN_R = crate::BitReader<bool>;
///Field `RNGEN` writer - True random number generator enable
pub type RNGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `IE` reader - Interrupt Enable
pub type IE_R = crate::BitReader<bool>;
///Field `IE` writer - Interrupt Enable
pub type IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `CED` reader - Clock error detection
pub type CED_R = crate::BitReader<bool>;
///Field `CED` writer - Clock error detection
pub type CED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `ARDIS` reader - Auto reset disable
pub type ARDIS_R = crate::BitReader<bool>;
///Field `ARDIS` writer - Auto reset disable
pub type ARDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `RNG_CONFIG3` reader - RNG configuration 3
pub type RNG_CONFIG3_R = crate::FieldReader<u8, u8>;
///Field `RNG_CONFIG3` writer - RNG configuration 3
pub type RNG_CONFIG3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 4, O>;
///Field `NISTC` reader - Non NIST compliant
pub type NISTC_R = crate::BitReader<bool>;
///Field `NISTC` writer - Non NIST compliant
pub type NISTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `RNG_CONFIG2` reader - RNG configuration 2
pub type RNG_CONFIG2_R = crate::FieldReader<u8, u8>;
///Field `RNG_CONFIG2` writer - RNG configuration 2
pub type RNG_CONFIG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
///Field `CLKDIV` reader - Clock divider factor
pub type CLKDIV_R = crate::FieldReader<u8, u8>;
///Field `CLKDIV` writer - Clock divider factor
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 4, O>;
///Field `RNG_CONFIG1` reader - RNG configuration 1
pub type RNG_CONFIG1_R = crate::FieldReader<u8, u8>;
///Field `RNG_CONFIG1` writer - RNG configuration 1
pub type RNG_CONFIG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 6, O>;
///Field `CONDRST` reader - Conditioning soft reset
pub type CONDRST_R = crate::BitReader<bool>;
///Field `CONDRST` writer - Conditioning soft reset
pub type CONDRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `CONFIGLOCK` reader - RNG Config Lock
pub type CONFIGLOCK_R = crate::BitReader<bool>;
///Field `CONFIGLOCK` writer - RNG Config Lock
pub type CONFIGLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    ///Bit 2 - True random number generator enable
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt Enable
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - Clock error detection
    #[inline(always)]
    pub fn ced(&self) -> CED_R {
        CED_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Auto reset disable
    #[inline(always)]
    pub fn ardis(&self) -> ARDIS_R {
        ARDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - RNG configuration 3
    #[inline(always)]
    pub fn rng_config3(&self) -> RNG_CONFIG3_R {
        RNG_CONFIG3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - Non NIST compliant
    #[inline(always)]
    pub fn nistc(&self) -> NISTC_R {
        NISTC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:15 - RNG configuration 2
    #[inline(always)]
    pub fn rng_config2(&self) -> RNG_CONFIG2_R {
        RNG_CONFIG2_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 16:19 - Clock divider factor
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:25 - RNG configuration 1
    #[inline(always)]
    pub fn rng_config1(&self) -> RNG_CONFIG1_R {
        RNG_CONFIG1_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    ///Bit 30 - Conditioning soft reset
    #[inline(always)]
    pub fn condrst(&self) -> CONDRST_R {
        CONDRST_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - RNG Config Lock
    #[inline(always)]
    pub fn configlock(&self) -> CONFIGLOCK_R {
        CONFIGLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - True random number generator enable
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<2> {
        RNGEN_W::new(self)
    }
    ///Bit 3 - Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<3> {
        IE_W::new(self)
    }
    ///Bit 5 - Clock error detection
    #[inline(always)]
    #[must_use]
    pub fn ced(&mut self) -> CED_W<5> {
        CED_W::new(self)
    }
    ///Bit 7 - Auto reset disable
    #[inline(always)]
    #[must_use]
    pub fn ardis(&mut self) -> ARDIS_W<7> {
        ARDIS_W::new(self)
    }
    ///Bits 8:11 - RNG configuration 3
    #[inline(always)]
    #[must_use]
    pub fn rng_config3(&mut self) -> RNG_CONFIG3_W<8> {
        RNG_CONFIG3_W::new(self)
    }
    ///Bit 12 - Non NIST compliant
    #[inline(always)]
    #[must_use]
    pub fn nistc(&mut self) -> NISTC_W<12> {
        NISTC_W::new(self)
    }
    ///Bits 13:15 - RNG configuration 2
    #[inline(always)]
    #[must_use]
    pub fn rng_config2(&mut self) -> RNG_CONFIG2_W<13> {
        RNG_CONFIG2_W::new(self)
    }
    ///Bits 16:19 - Clock divider factor
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<16> {
        CLKDIV_W::new(self)
    }
    ///Bits 20:25 - RNG configuration 1
    #[inline(always)]
    #[must_use]
    pub fn rng_config1(&mut self) -> RNG_CONFIG1_W<20> {
        RNG_CONFIG1_W::new(self)
    }
    ///Bit 30 - Conditioning soft reset
    #[inline(always)]
    #[must_use]
    pub fn condrst(&mut self) -> CONDRST_W<30> {
        CONDRST_W::new(self)
    }
    ///Bit 31 - RNG Config Lock
    #[inline(always)]
    #[must_use]
    pub fn configlock(&mut self) -> CONFIGLOCK_W<31> {
        CONFIGLOCK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
