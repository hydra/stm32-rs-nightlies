///Register `SUSP6R` reader
pub struct R(crate::R<SUSP6R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUSP6R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUSP6R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUSP6R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SUSP6R` writer
pub struct W(crate::W<SUSP6R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUSP6R_SPEC>;
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
impl From<crate::W<SUSP6R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUSP6R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SUSP` reader - SAES suspend Upon suspend operation, this bitfield of the corresponding SAES_SUSPxR register takes the value of one of internal SAES registers.
pub type SUSP_R = crate::FieldReader<u32, u32>;
///Field `SUSP` writer - SAES suspend Upon suspend operation, this bitfield of the corresponding SAES_SUSPxR register takes the value of one of internal SAES registers.
pub type SUSP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SUSP6R_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - SAES suspend Upon suspend operation, this bitfield of the corresponding SAES_SUSPxR register takes the value of one of internal SAES registers.
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - SAES suspend Upon suspend operation, this bitfield of the corresponding SAES_SUSPxR register takes the value of one of internal SAES registers.
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<0> {
        SUSP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SAES suspend registers
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [susp6r](index.html) module
pub struct SUSP6R_SPEC;
impl crate::RegisterSpec for SUSP6R_SPEC {
    type Ux = u32;
}
///`read()` method returns [susp6r::R](R) reader structure
impl crate::Readable for SUSP6R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [susp6r::W](W) writer structure
impl crate::Writable for SUSP6R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SUSP6R to value 0
impl crate::Resettable for SUSP6R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
