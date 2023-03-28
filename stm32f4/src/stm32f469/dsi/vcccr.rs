///Register `VCCCR` reader
pub struct R(crate::R<VCCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VCCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VCCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VCCCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `VCCCR` writer
pub struct W(crate::W<VCCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VCCCR_SPEC>;
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
impl From<crate::W<VCCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VCCCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `NUMC` reader - Number of Chunks
pub type NUMC_R = crate::FieldReader<u16, u16>;
///Field `NUMC` writer - Number of Chunks
pub type NUMC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VCCCR_SPEC, u16, u16, 14, O>;
impl R {
    ///Bits 0:13 - Number of Chunks
    #[inline(always)]
    pub fn numc(&self) -> NUMC_R {
        NUMC_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    ///Bits 0:13 - Number of Chunks
    #[inline(always)]
    #[must_use]
    pub fn numc(&mut self) -> NUMC_W<0> {
        NUMC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host Video Chunks Current Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vcccr](index.html) module
pub struct VCCCR_SPEC;
impl crate::RegisterSpec for VCCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [vcccr::R](R) reader structure
impl crate::Readable for VCCCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [vcccr::W](W) writer structure
impl crate::Writable for VCCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets VCCCR to value 0
impl crate::Resettable for VCCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
