///Register `OPAMP_OR` reader
pub struct R(crate::R<OPAMP_OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAMP_OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAMP_OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAMP_OR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OPAMP_OR` writer
pub struct W(crate::W<OPAMP_OR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAMP_OR_SPEC>;
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
impl From<crate::W<OPAMP_OR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAMP_OR_SPEC>) -> Self {
        W(writer)
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
///OPAMP option register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [opamp_or](index.html) module
pub struct OPAMP_OR_SPEC;
impl crate::RegisterSpec for OPAMP_OR_SPEC {
    type Ux = u32;
}
///`read()` method returns [opamp_or::R](R) reader structure
impl crate::Readable for OPAMP_OR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [opamp_or::W](W) writer structure
impl crate::Writable for OPAMP_OR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OPAMP_OR to value 0
impl crate::Resettable for OPAMP_OR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
