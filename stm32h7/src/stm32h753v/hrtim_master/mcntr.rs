///Register `MCNTR` reader
pub struct R(crate::R<MCNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCNTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MCNTR` writer
pub struct W(crate::W<MCNTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCNTR_SPEC>;
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
impl From<crate::W<MCNTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCNTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MCNT` reader - Counter value
pub type MCNT_R = crate::FieldReader<u16, u16>;
///Field `MCNT` writer - Counter value
pub type MCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCNTR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Counter value
    #[inline(always)]
    pub fn mcnt(&self) -> MCNT_R {
        MCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Counter value
    #[inline(always)]
    #[must_use]
    pub fn mcnt(&mut self) -> MCNT_W<0> {
        MCNT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Master Timer Counter Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mcntr](index.html) module
pub struct MCNTR_SPEC;
impl crate::RegisterSpec for MCNTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mcntr::R](R) reader structure
impl crate::Readable for MCNTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mcntr::W](W) writer structure
impl crate::Writable for MCNTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MCNTR to value 0
impl crate::Resettable for MCNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
