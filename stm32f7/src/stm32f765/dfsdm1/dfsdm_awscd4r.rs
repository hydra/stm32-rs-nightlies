///Register `DFSDM_AWSCD4R` reader
pub struct R(crate::R<DFSDM_AWSCD4R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_AWSCD4R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_AWSCD4R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_AWSCD4R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DFSDM_AWSCD4R` writer
pub struct W(crate::W<DFSDM_AWSCD4R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSDM_AWSCD4R_SPEC>;
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
impl From<crate::W<DFSDM_AWSCD4R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSDM_AWSCD4R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SCDT` reader - short-circuit detector threshold for channel 4
pub type SCDT_R = crate::FieldReader<u8, u8>;
///Field `SCDT` writer - short-circuit detector threshold for channel 4
pub type SCDT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSDM_AWSCD4R_SPEC, u8, u8, 8, O>;
///Field `BKSCD` reader - Break signal assignment for short-circuit detector on channel 4
pub type BKSCD_R = crate::FieldReader<u8, u8>;
///Field `BKSCD` writer - Break signal assignment for short-circuit detector on channel 4
pub type BKSCD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSDM_AWSCD4R_SPEC, u8, u8, 4, O>;
///Field `AWFOSR` reader - Analog watchdog filter oversampling ratio (decimation rate) on channel 4
pub type AWFOSR_R = crate::FieldReader<u8, u8>;
///Field `AWFOSR` writer - Analog watchdog filter oversampling ratio (decimation rate) on channel 4
pub type AWFOSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSDM_AWSCD4R_SPEC, u8, u8, 5, O>;
///Field `AWFORD` reader - Analog watchdog Sinc filter order on channel 4
pub type AWFORD_R = crate::FieldReader<u8, u8>;
///Field `AWFORD` writer - Analog watchdog Sinc filter order on channel 4
pub type AWFORD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSDM_AWSCD4R_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:7 - short-circuit detector threshold for channel 4
    #[inline(always)]
    pub fn scdt(&self) -> SCDT_R {
        SCDT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 12:15 - Break signal assignment for short-circuit detector on channel 4
    #[inline(always)]
    pub fn bkscd(&self) -> BKSCD_R {
        BKSCD_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:20 - Analog watchdog filter oversampling ratio (decimation rate) on channel 4
    #[inline(always)]
    pub fn awfosr(&self) -> AWFOSR_R {
        AWFOSR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 22:23 - Analog watchdog Sinc filter order on channel 4
    #[inline(always)]
    pub fn awford(&self) -> AWFORD_R {
        AWFORD_R::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    ///Bits 0:7 - short-circuit detector threshold for channel 4
    #[inline(always)]
    #[must_use]
    pub fn scdt(&mut self) -> SCDT_W<0> {
        SCDT_W::new(self)
    }
    ///Bits 12:15 - Break signal assignment for short-circuit detector on channel 4
    #[inline(always)]
    #[must_use]
    pub fn bkscd(&mut self) -> BKSCD_W<12> {
        BKSCD_W::new(self)
    }
    ///Bits 16:20 - Analog watchdog filter oversampling ratio (decimation rate) on channel 4
    #[inline(always)]
    #[must_use]
    pub fn awfosr(&mut self) -> AWFOSR_W<16> {
        AWFOSR_W::new(self)
    }
    ///Bits 22:23 - Analog watchdog Sinc filter order on channel 4
    #[inline(always)]
    #[must_use]
    pub fn awford(&mut self) -> AWFORD_W<22> {
        AWFORD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DFSDM analog watchdog and short-circuit detector register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_awscd4r](index.html) module
pub struct DFSDM_AWSCD4R_SPEC;
impl crate::RegisterSpec for DFSDM_AWSCD4R_SPEC {
    type Ux = u32;
}
///`read()` method returns [dfsdm_awscd4r::R](R) reader structure
impl crate::Readable for DFSDM_AWSCD4R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dfsdm_awscd4r::W](W) writer structure
impl crate::Writable for DFSDM_AWSCD4R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DFSDM_AWSCD4R to value 0
impl crate::Resettable for DFSDM_AWSCD4R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
