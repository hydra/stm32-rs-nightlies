///Register `MREP` reader
pub struct R(crate::R<MREP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MREP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MREP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MREP_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MREP` writer
pub struct W(crate::W<MREP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MREP_SPEC>;
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
impl From<crate::W<MREP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MREP_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MREP` reader - Master Timer Repetition counter value
pub type MREP_R = crate::FieldReader<u8, u8>;
///Field `MREP` writer - Master Timer Repetition counter value
pub type MREP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MREP_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - Master Timer Repetition counter value
    #[inline(always)]
    pub fn mrep(&self) -> MREP_R {
        MREP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Master Timer Repetition counter value
    #[inline(always)]
    #[must_use]
    pub fn mrep(&mut self) -> MREP_W<0> {
        MREP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Master Timer Repetition Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mrep](index.html) module
pub struct MREP_SPEC;
impl crate::RegisterSpec for MREP_SPEC {
    type Ux = u32;
}
///`read()` method returns [mrep::R](R) reader structure
impl crate::Readable for MREP_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mrep::W](W) writer structure
impl crate::Writable for MREP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MREP to value 0
impl crate::Resettable for MREP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
