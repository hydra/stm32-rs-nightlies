///Register `HLCR` reader
pub struct R(crate::R<HLCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HLCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HLCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HLCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `HLCR` writer
pub struct W(crate::W<HLCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HLCR_SPEC>;
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
impl From<crate::W<HLCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HLCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LM` reader - Latency mode
pub type LM_R = crate::BitReader<bool>;
///Field `LM` writer - Latency mode
pub type LM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HLCR_SPEC, bool, O>;
///Field `WZL` reader - Write zero latency
pub type WZL_R = crate::BitReader<bool>;
///Field `WZL` writer - Write zero latency
pub type WZL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HLCR_SPEC, bool, O>;
///Field `TACC` reader - Access time
pub type TACC_R = crate::FieldReader<u8, u8>;
///Field `TACC` writer - Access time
pub type TACC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HLCR_SPEC, u8, u8, 8, O>;
///Field `TRWR` reader - Read write recovery time
pub type TRWR_R = crate::FieldReader<u8, u8>;
///Field `TRWR` writer - Read write recovery time
pub type TRWR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HLCR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bit 0 - Latency mode
    #[inline(always)]
    pub fn lm(&self) -> LM_R {
        LM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Write zero latency
    #[inline(always)]
    pub fn wzl(&self) -> WZL_R {
        WZL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 8:15 - Access time
    #[inline(always)]
    pub fn tacc(&self) -> TACC_R {
        TACC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Read write recovery time
    #[inline(always)]
    pub fn trwr(&self) -> TRWR_R {
        TRWR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bit 0 - Latency mode
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LM_W<0> {
        LM_W::new(self)
    }
    ///Bit 1 - Write zero latency
    #[inline(always)]
    #[must_use]
    pub fn wzl(&mut self) -> WZL_W<1> {
        WZL_W::new(self)
    }
    ///Bits 8:15 - Access time
    #[inline(always)]
    #[must_use]
    pub fn tacc(&mut self) -> TACC_W<8> {
        TACC_W::new(self)
    }
    ///Bits 16:23 - Read write recovery time
    #[inline(always)]
    #[must_use]
    pub fn trwr(&mut self) -> TRWR_W<16> {
        TRWR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///HyperBus latency configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hlcr](index.html) module
pub struct HLCR_SPEC;
impl crate::RegisterSpec for HLCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [hlcr::R](R) reader structure
impl crate::Readable for HLCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [hlcr::W](W) writer structure
impl crate::Writable for HLCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets HLCR to value 0
impl crate::Resettable for HLCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
