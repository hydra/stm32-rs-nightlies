///Register `M2FAR` reader
pub struct R(crate::R<M2FAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M2FAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M2FAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M2FAR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `M2FAR` writer
pub struct W(crate::W<M2FAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M2FAR_SPEC>;
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
impl From<crate::W<M2FAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M2FAR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FADD` reader - ECC failing address
pub type FADD_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - ECC failing address
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new(self.bits)
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
///RAMECC monitor 2 failing address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [m2far](index.html) module
pub struct M2FAR_SPEC;
impl crate::RegisterSpec for M2FAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [m2far::R](R) reader structure
impl crate::Readable for M2FAR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [m2far::W](W) writer structure
impl crate::Writable for M2FAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets M2FAR to value 0
impl crate::Resettable for M2FAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
