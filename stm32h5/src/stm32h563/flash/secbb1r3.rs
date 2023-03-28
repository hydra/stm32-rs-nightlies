///Register `SECBB1R3` reader
pub struct R(crate::R<SECBB1R3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECBB1R3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECBB1R3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECBB1R3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SECBB1R3` writer
pub struct W(crate::W<SECBB1R3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECBB1R3_SPEC>;
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
impl From<crate::W<SECBB1R3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECBB1R3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SECBB1` reader - Secure/non-secure 8�Kbytes flash Bank 1 sector attributes
pub type SECBB1_R = crate::FieldReader<u32, u32>;
///Field `SECBB1` writer - Secure/non-secure 8�Kbytes flash Bank 1 sector attributes
pub type SECBB1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SECBB1R3_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Secure/non-secure 8�Kbytes flash Bank 1 sector attributes
    #[inline(always)]
    pub fn secbb1(&self) -> SECBB1_R {
        SECBB1_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Secure/non-secure 8�Kbytes flash Bank 1 sector attributes
    #[inline(always)]
    #[must_use]
    pub fn secbb1(&mut self) -> SECBB1_W<0> {
        SECBB1_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH secure block based register for Bank 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [secbb1r3](index.html) module
pub struct SECBB1R3_SPEC;
impl crate::RegisterSpec for SECBB1R3_SPEC {
    type Ux = u32;
}
///`read()` method returns [secbb1r3::R](R) reader structure
impl crate::Readable for SECBB1R3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [secbb1r3::W](W) writer structure
impl crate::Writable for SECBB1R3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SECBB1R3 to value 0
impl crate::Resettable for SECBB1R3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
