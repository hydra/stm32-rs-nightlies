///Register `TIMINGR` reader
pub struct R(crate::R<TIMINGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMINGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMINGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMINGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIMINGR` writer
pub struct W(crate::W<TIMINGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMINGR_SPEC>;
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
impl From<crate::W<TIMINGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMINGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SCLL` reader - SCL low period (master mode)
pub type SCLL_R = crate::FieldReader<u8, u8>;
///Field `SCLL` writer - SCL low period (master mode)
pub type SCLL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMINGR_SPEC, u8, u8, 8, O>;
///Field `SCLH` reader - SCL high period (master mode)
pub type SCLH_R = crate::FieldReader<u8, u8>;
///Field `SCLH` writer - SCL high period (master mode)
pub type SCLH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMINGR_SPEC, u8, u8, 8, O>;
///Field `SDADEL` reader - Data hold time
pub type SDADEL_R = crate::FieldReader<u8, u8>;
///Field `SDADEL` writer - Data hold time
pub type SDADEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMINGR_SPEC, u8, u8, 4, O>;
///Field `SCLDEL` reader - Data setup time
pub type SCLDEL_R = crate::FieldReader<u8, u8>;
///Field `SCLDEL` writer - Data setup time
pub type SCLDEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMINGR_SPEC, u8, u8, 4, O>;
///Field `PRESC` reader - Timing prescaler
pub type PRESC_R = crate::FieldReader<u8, u8>;
///Field `PRESC` writer - Timing prescaler
pub type PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMINGR_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:7 - SCL low period (master mode)
    #[inline(always)]
    pub fn scll(&self) -> SCLL_R {
        SCLL_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - SCL high period (master mode)
    #[inline(always)]
    pub fn sclh(&self) -> SCLH_R {
        SCLH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - Data hold time
    #[inline(always)]
    pub fn sdadel(&self) -> SDADEL_R {
        SDADEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Data setup time
    #[inline(always)]
    pub fn scldel(&self) -> SCLDEL_R {
        SCLDEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 28:31 - Timing prescaler
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:7 - SCL low period (master mode)
    #[inline(always)]
    #[must_use]
    pub fn scll(&mut self) -> SCLL_W<0> {
        SCLL_W::new(self)
    }
    ///Bits 8:15 - SCL high period (master mode)
    #[inline(always)]
    #[must_use]
    pub fn sclh(&mut self) -> SCLH_W<8> {
        SCLH_W::new(self)
    }
    ///Bits 16:19 - Data hold time
    #[inline(always)]
    #[must_use]
    pub fn sdadel(&mut self) -> SDADEL_W<16> {
        SDADEL_W::new(self)
    }
    ///Bits 20:23 - Data setup time
    #[inline(always)]
    #[must_use]
    pub fn scldel(&mut self) -> SCLDEL_W<20> {
        SCLDEL_W::new(self)
    }
    ///Bits 28:31 - Timing prescaler
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<28> {
        PRESC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timing register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timingr](index.html) module
pub struct TIMINGR_SPEC;
impl crate::RegisterSpec for TIMINGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [timingr::R](R) reader structure
impl crate::Readable for TIMINGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [timingr::W](W) writer structure
impl crate::Writable for TIMINGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIMINGR to value 0
impl crate::Resettable for TIMINGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
