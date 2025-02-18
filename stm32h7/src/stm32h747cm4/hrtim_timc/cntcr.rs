///Register `CNTCR` reader
pub struct R(crate::R<CNTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CNTCR` writer
pub struct W(crate::W<CNTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTCR_SPEC>;
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
impl From<crate::W<CNTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CNTx` reader - Timerx Counter value
pub type CNTX_R = crate::FieldReader<u16, u16>;
///Field `CNTx` writer - Timerx Counter value
pub type CNTX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNTCR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Timerx Counter value
    #[inline(always)]
    pub fn cntx(&self) -> CNTX_R {
        CNTX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Timerx Counter value
    #[inline(always)]
    #[must_use]
    pub fn cntx(&mut self) -> CNTX_W<0> {
        CNTX_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timerx Counter Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cntcr](index.html) module
pub struct CNTCR_SPEC;
impl crate::RegisterSpec for CNTCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cntcr::R](R) reader structure
impl crate::Readable for CNTCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cntcr::W](W) writer structure
impl crate::Writable for CNTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CNTCR to value 0
impl crate::Resettable for CNTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
