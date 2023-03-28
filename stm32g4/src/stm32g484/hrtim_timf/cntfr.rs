///Register `CNTFR` reader
pub struct R(crate::R<CNTFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTFR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CNTFR` writer
pub struct W(crate::W<CNTFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTFR_SPEC>;
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
impl From<crate::W<CNTFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTFR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CNTx` reader - Timerx Counter value
pub type CNTX_R = crate::FieldReader<u16, u16>;
///Field `CNTx` writer - Timerx Counter value
pub type CNTX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNTFR_SPEC, u16, u16, 16, O>;
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
///For information about available fields see [cntfr](index.html) module
pub struct CNTFR_SPEC;
impl crate::RegisterSpec for CNTFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cntfr::R](R) reader structure
impl crate::Readable for CNTFR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cntfr::W](W) writer structure
impl crate::Writable for CNTFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CNTFR to value 0
impl crate::Resettable for CNTFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
