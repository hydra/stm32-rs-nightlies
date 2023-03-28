///Register `SUSP4R` reader
pub struct R(crate::R<SUSP4R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUSP4R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUSP4R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUSP4R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SUSP4R` writer
pub struct W(crate::W<SUSP4R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUSP4R_SPEC>;
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
impl From<crate::W<SUSP4R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUSP4R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SUSP` reader - AES suspend register 4
pub type SUSP_R = crate::FieldReader<u32, u32>;
///Field `SUSP` writer - AES suspend register 4
pub type SUSP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SUSP4R_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - AES suspend register 4
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - AES suspend register 4
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<0> {
        SUSP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
///AES suspend register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [susp4r](index.html) module
pub struct SUSP4R_SPEC;
impl crate::RegisterSpec for SUSP4R_SPEC {
    type Ux = u32;
}
///`read()` method returns [susp4r::R](R) reader structure
impl crate::Readable for SUSP4R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [susp4r::W](W) writer structure
impl crate::Writable for SUSP4R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SUSP4R to value 0
impl crate::Resettable for SUSP4R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
