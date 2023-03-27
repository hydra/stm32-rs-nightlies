///Register `ICIFR` reader
pub struct R(crate::R<ICIFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICIFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICIFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICIFR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ICIFR` writer
pub struct W(crate::W<ICIFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICIFR_SPEC>;
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
impl From<crate::W<ICIFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICIFR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TS1_CITEF` reader - Interrupt clear flag for end of measurement on temperature sensor 1 Writing 1 to this bit clears the TS1_ITEF flag in the DTS_SR register.
pub type TS1_CITEF_R = crate::BitReader<bool>;
///Field `TS1_CITEF` writer - Interrupt clear flag for end of measurement on temperature sensor 1 Writing 1 to this bit clears the TS1_ITEF flag in the DTS_SR register.
pub type TS1_CITEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICIFR_SPEC, bool, O>;
///Field `TS1_CITLF` reader - Interrupt clear flag for low threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_ITLF flag in the DTS_SR register.
pub type TS1_CITLF_R = crate::BitReader<bool>;
///Field `TS1_CITLF` writer - Interrupt clear flag for low threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_ITLF flag in the DTS_SR register.
pub type TS1_CITLF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICIFR_SPEC, bool, O>;
///Field `TS1_CITHF` reader - Interrupt clear flag for high threshold on temperature sensor 1 Writing this bit to 1 clears the TS1_ITHF flag in the DTS_SR register.
pub type TS1_CITHF_R = crate::BitReader<bool>;
///Field `TS1_CITHF` writer - Interrupt clear flag for high threshold on temperature sensor 1 Writing this bit to 1 clears the TS1_ITHF flag in the DTS_SR register.
pub type TS1_CITHF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICIFR_SPEC, bool, O>;
///Field `TS1_CAITEF` reader - Write once bit. Clear the asynchronous IT flag for End Of Measure for thermal sensor 1. Writing 1 clears the TS1_AITEF flag of the DTS_SR register.
pub type TS1_CAITEF_R = crate::BitReader<bool>;
///Field `TS1_CAITEF` writer - Write once bit. Clear the asynchronous IT flag for End Of Measure for thermal sensor 1. Writing 1 clears the TS1_AITEF flag of the DTS_SR register.
pub type TS1_CAITEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICIFR_SPEC, bool, O>;
///Field `TS1_CAITLF` reader - Asynchronous interrupt clear flag for low threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_AITLF flag in the DTS_SR register.
pub type TS1_CAITLF_R = crate::BitReader<bool>;
///Field `TS1_CAITLF` writer - Asynchronous interrupt clear flag for low threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_AITLF flag in the DTS_SR register.
pub type TS1_CAITLF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICIFR_SPEC, bool, O>;
///Field `TS1_CAITHF` reader - Asynchronous interrupt clear flag for high threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_AITHF flag in the DTS_SR register.
pub type TS1_CAITHF_R = crate::BitReader<bool>;
///Field `TS1_CAITHF` writer - Asynchronous interrupt clear flag for high threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_AITHF flag in the DTS_SR register.
pub type TS1_CAITHF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICIFR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Interrupt clear flag for end of measurement on temperature sensor 1 Writing 1 to this bit clears the TS1_ITEF flag in the DTS_SR register.
    #[inline(always)]
    pub fn ts1_citef(&self) -> TS1_CITEF_R {
        TS1_CITEF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt clear flag for low threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_ITLF flag in the DTS_SR register.
    #[inline(always)]
    pub fn ts1_citlf(&self) -> TS1_CITLF_R {
        TS1_CITLF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt clear flag for high threshold on temperature sensor 1 Writing this bit to 1 clears the TS1_ITHF flag in the DTS_SR register.
    #[inline(always)]
    pub fn ts1_cithf(&self) -> TS1_CITHF_R {
        TS1_CITHF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Write once bit. Clear the asynchronous IT flag for End Of Measure for thermal sensor 1. Writing 1 clears the TS1_AITEF flag of the DTS_SR register.
    #[inline(always)]
    pub fn ts1_caitef(&self) -> TS1_CAITEF_R {
        TS1_CAITEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Asynchronous interrupt clear flag for low threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_AITLF flag in the DTS_SR register.
    #[inline(always)]
    pub fn ts1_caitlf(&self) -> TS1_CAITLF_R {
        TS1_CAITLF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Asynchronous interrupt clear flag for high threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_AITHF flag in the DTS_SR register.
    #[inline(always)]
    pub fn ts1_caithf(&self) -> TS1_CAITHF_R {
        TS1_CAITHF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Interrupt clear flag for end of measurement on temperature sensor 1 Writing 1 to this bit clears the TS1_ITEF flag in the DTS_SR register.
    #[inline(always)]
    #[must_use]
    pub fn ts1_citef(&mut self) -> TS1_CITEF_W<0> {
        TS1_CITEF_W::new(self)
    }
    ///Bit 1 - Interrupt clear flag for low threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_ITLF flag in the DTS_SR register.
    #[inline(always)]
    #[must_use]
    pub fn ts1_citlf(&mut self) -> TS1_CITLF_W<1> {
        TS1_CITLF_W::new(self)
    }
    ///Bit 2 - Interrupt clear flag for high threshold on temperature sensor 1 Writing this bit to 1 clears the TS1_ITHF flag in the DTS_SR register.
    #[inline(always)]
    #[must_use]
    pub fn ts1_cithf(&mut self) -> TS1_CITHF_W<2> {
        TS1_CITHF_W::new(self)
    }
    ///Bit 4 - Write once bit. Clear the asynchronous IT flag for End Of Measure for thermal sensor 1. Writing 1 clears the TS1_AITEF flag of the DTS_SR register.
    #[inline(always)]
    #[must_use]
    pub fn ts1_caitef(&mut self) -> TS1_CAITEF_W<4> {
        TS1_CAITEF_W::new(self)
    }
    ///Bit 5 - Asynchronous interrupt clear flag for low threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_AITLF flag in the DTS_SR register.
    #[inline(always)]
    #[must_use]
    pub fn ts1_caitlf(&mut self) -> TS1_CAITLF_W<5> {
        TS1_CAITLF_W::new(self)
    }
    ///Bit 6 - Asynchronous interrupt clear flag for high threshold on temperature sensor 1 Writing 1 to this bit clears the TS1_AITHF flag in the DTS_SR register.
    #[inline(always)]
    #[must_use]
    pub fn ts1_caithf(&mut self) -> TS1_CAITHF_W<6> {
        TS1_CAITHF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Temperature sensor clear interrupt flag register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icifr](index.html) module
pub struct ICIFR_SPEC;
impl crate::RegisterSpec for ICIFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [icifr::R](R) reader structure
impl crate::Readable for ICIFR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [icifr::W](W) writer structure
impl crate::Writable for ICIFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ICIFR to value 0
impl crate::Resettable for ICIFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
