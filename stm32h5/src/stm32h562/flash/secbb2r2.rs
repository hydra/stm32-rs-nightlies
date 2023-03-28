///Register `SECBB2R2` reader
pub struct R(crate::R<SECBB2R2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECBB2R2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECBB2R2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECBB2R2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SECBB2R2` writer
pub struct W(crate::W<SECBB2R2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECBB2R2_SPEC>;
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
impl From<crate::W<SECBB2R2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECBB2R2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SECBB2` reader - Secure/non-secure flash Bank 2 sector attribute
pub type SECBB2_R = crate::FieldReader<u32, u32>;
///Field `SECBB2` writer - Secure/non-secure flash Bank 2 sector attribute
pub type SECBB2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SECBB2R2_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Secure/non-secure flash Bank 2 sector attribute
    #[inline(always)]
    pub fn secbb2(&self) -> SECBB2_R {
        SECBB2_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Secure/non-secure flash Bank 2 sector attribute
    #[inline(always)]
    #[must_use]
    pub fn secbb2(&mut self) -> SECBB2_W<0> {
        SECBB2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH secure block-based register for Bank 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [secbb2r2](index.html) module
pub struct SECBB2R2_SPEC;
impl crate::RegisterSpec for SECBB2R2_SPEC {
    type Ux = u32;
}
///`read()` method returns [secbb2r2::R](R) reader structure
impl crate::Readable for SECBB2R2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [secbb2r2::W](W) writer structure
impl crate::Writable for SECBB2R2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SECBB2R2 to value 0
impl crate::Resettable for SECBB2R2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
