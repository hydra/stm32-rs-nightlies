///Register `FLT3FCR` reader
pub struct R(crate::R<FLT3FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLT3FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLT3FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLT3FCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLT3FCR` writer
pub struct W(crate::W<FLT3FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLT3FCR_SPEC>;
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
impl From<crate::W<FLT3FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLT3FCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IOSR` reader - Integrator oversampling ratio (averaging length)
pub type IOSR_R = crate::FieldReader<u8, u8>;
///Field `IOSR` writer - Integrator oversampling ratio (averaging length)
pub type IOSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLT3FCR_SPEC, u8, u8, 8, O>;
///Field `FOSR` reader - Sinc filter oversampling ratio (decimation rate)
pub type FOSR_R = crate::FieldReader<u16, u16>;
///Field `FOSR` writer - Sinc filter oversampling ratio (decimation rate)
pub type FOSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLT3FCR_SPEC, u16, u16, 10, O>;
///Field `FORD` reader - Sinc filter order
pub type FORD_R = crate::FieldReader<u8, u8>;
///Field `FORD` writer - Sinc filter order
pub type FORD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLT3FCR_SPEC, u8, u8, 3, O>;
impl R {
    ///Bits 0:7 - Integrator oversampling ratio (averaging length)
    #[inline(always)]
    pub fn iosr(&self) -> IOSR_R {
        IOSR_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:25 - Sinc filter oversampling ratio (decimation rate)
    #[inline(always)]
    pub fn fosr(&self) -> FOSR_R {
        FOSR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bits 29:31 - Sinc filter order
    #[inline(always)]
    pub fn ford(&self) -> FORD_R {
        FORD_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    ///Bits 0:7 - Integrator oversampling ratio (averaging length)
    #[inline(always)]
    #[must_use]
    pub fn iosr(&mut self) -> IOSR_W<0> {
        IOSR_W::new(self)
    }
    ///Bits 16:25 - Sinc filter oversampling ratio (decimation rate)
    #[inline(always)]
    #[must_use]
    pub fn fosr(&mut self) -> FOSR_W<16> {
        FOSR_W::new(self)
    }
    ///Bits 29:31 - Sinc filter order
    #[inline(always)]
    #[must_use]
    pub fn ford(&mut self) -> FORD_W<29> {
        FORD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///filter control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flt3fcr](index.html) module
pub struct FLT3FCR_SPEC;
impl crate::RegisterSpec for FLT3FCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [flt3fcr::R](R) reader structure
impl crate::Readable for FLT3FCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [flt3fcr::W](W) writer structure
impl crate::Writable for FLT3FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLT3FCR to value 0
impl crate::Resettable for FLT3FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
