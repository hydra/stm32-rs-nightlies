///Register `VDSL` reader
pub struct R(crate::R<VDSL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VDSL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VDSL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VDSL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `VDSL` writer
pub struct W(crate::W<VDSL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VDSL_SPEC>;
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
impl From<crate::W<VDSL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VDSL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LENG` reader - Non-volatile data segment length
pub type LENG_R = crate::FieldReader<u16, u16>;
///Field `LENG` writer - Non-volatile data segment length
pub type LENG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VDSL_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 6:17 - Non-volatile data segment length
    #[inline(always)]
    pub fn leng(&self) -> LENG_R {
        LENG_R::new(((self.bits >> 6) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 6:17 - Non-volatile data segment length
    #[inline(always)]
    #[must_use]
    pub fn leng(&mut self) -> LENG_W<6> {
        LENG_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Volatile data segment length
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vdsl](index.html) module
pub struct VDSL_SPEC;
impl crate::RegisterSpec for VDSL_SPEC {
    type Ux = u32;
}
///`read()` method returns [vdsl::R](R) reader structure
impl crate::Readable for VDSL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [vdsl::W](W) writer structure
impl crate::Writable for VDSL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets VDSL to value 0
impl crate::Resettable for VDSL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
