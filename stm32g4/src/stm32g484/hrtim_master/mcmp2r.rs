///Register `MCMP2R` reader
pub struct R(crate::R<MCMP2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCMP2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCMP2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCMP2R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MCMP2R` writer
pub struct W(crate::W<MCMP2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCMP2R_SPEC>;
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
impl From<crate::W<MCMP2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCMP2R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MCMP2` reader - Master Timer Compare 2 value
pub type MCMP2_R = crate::FieldReader<u16, u16>;
///Field `MCMP2` writer - Master Timer Compare 2 value
pub type MCMP2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCMP2R_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Master Timer Compare 2 value
    #[inline(always)]
    pub fn mcmp2(&self) -> MCMP2_R {
        MCMP2_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Master Timer Compare 2 value
    #[inline(always)]
    #[must_use]
    pub fn mcmp2(&mut self) -> MCMP2_W<0> {
        MCMP2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Master Timer Compare 2 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mcmp2r](index.html) module
pub struct MCMP2R_SPEC;
impl crate::RegisterSpec for MCMP2R_SPEC {
    type Ux = u32;
}
///`read()` method returns [mcmp2r::R](R) reader structure
impl crate::Readable for MCMP2R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mcmp2r::W](W) writer structure
impl crate::Writable for MCMP2R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MCMP2R to value 0
impl crate::Resettable for MCMP2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
