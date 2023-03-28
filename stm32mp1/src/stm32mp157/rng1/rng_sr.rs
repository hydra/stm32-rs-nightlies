///Register `RNG_SR` reader
pub struct R(crate::R<RNG_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNG_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RNG_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RNG_SR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RNG_SR` writer
pub struct W(crate::W<RNG_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RNG_SR_SPEC>;
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
impl From<crate::W<RNG_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RNG_SR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DRDY` reader - DRDY
pub type DRDY_R = crate::BitReader<bool>;
///Field `CECS` reader - CECS
pub type CECS_R = crate::BitReader<bool>;
///Field `SECS` reader - SECS
pub type SECS_R = crate::BitReader<bool>;
///Field `CEIS` reader - CEIS
pub type CEIS_R = crate::BitReader<bool>;
///Field `CEIS` writer - CEIS
pub type CEIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RNG_SR_SPEC, bool, O>;
///Field `SEIS` reader - SEIS
pub type SEIS_R = crate::BitReader<bool>;
///Field `SEIS` writer - SEIS
pub type SEIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RNG_SR_SPEC, bool, O>;
impl R {
    ///Bit 0 - DRDY
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CECS
    #[inline(always)]
    pub fn cecs(&self) -> CECS_R {
        CECS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SECS
    #[inline(always)]
    pub fn secs(&self) -> SECS_R {
        SECS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - CEIS
    #[inline(always)]
    pub fn ceis(&self) -> CEIS_R {
        CEIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SEIS
    #[inline(always)]
    pub fn seis(&self) -> SEIS_R {
        SEIS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 5 - CEIS
    #[inline(always)]
    #[must_use]
    pub fn ceis(&mut self) -> CEIS_W<5> {
        CEIS_W::new(self)
    }
    ///Bit 6 - SEIS
    #[inline(always)]
    #[must_use]
    pub fn seis(&mut self) -> SEIS_W<6> {
        SEIS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RNG status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rng_sr](index.html) module
pub struct RNG_SR_SPEC;
impl crate::RegisterSpec for RNG_SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rng_sr::R](R) reader structure
impl crate::Readable for RNG_SR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rng_sr::W](W) writer structure
impl crate::Writable for RNG_SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RNG_SR to value 0
impl crate::Resettable for RNG_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
