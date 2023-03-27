///Register `M1FECR` reader
pub struct R(crate::R<M1FECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M1FECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M1FECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M1FECR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `M1FECR` writer
pub struct W(crate::W<M1FECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M1FECR_SPEC>;
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
impl From<crate::W<M1FECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M1FECR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FEC` reader - ECC failing code
pub type FEC_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - ECC failing code
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new(self.bits)
    }
}
impl W {
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RAMECC monitor 1 failing error code register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [m1fecr](index.html) module
pub struct M1FECR_SPEC;
impl crate::RegisterSpec for M1FECR_SPEC {
    type Ux = u32;
}
///`read()` method returns [m1fecr::R](R) reader structure
impl crate::Readable for M1FECR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [m1fecr::W](W) writer structure
impl crate::Writable for M1FECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets M1FECR to value 0
impl crate::Resettable for M1FECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
