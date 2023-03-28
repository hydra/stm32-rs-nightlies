///Register `GICD_ICACTIVER4` reader
pub struct R(crate::R<GICD_ICACTIVER4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ICACTIVER4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ICACTIVER4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ICACTIVER4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GICD_ICACTIVER4` writer
pub struct W(crate::W<GICD_ICACTIVER4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ICACTIVER4_SPEC>;
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
impl From<crate::W<GICD_ICACTIVER4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ICACTIVER4_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ICACTIVER4` reader - ICACTIVER4
pub type ICACTIVER4_R = crate::FieldReader<u32, u32>;
///Field `ICACTIVER4` writer - ICACTIVER4
pub type ICACTIVER4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ICACTIVER4_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - ICACTIVER4
    #[inline(always)]
    pub fn icactiver4(&self) -> ICACTIVER4_R {
        ICACTIVER4_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - ICACTIVER4
    #[inline(always)]
    #[must_use]
    pub fn icactiver4(&mut self) -> ICACTIVER4_W<0> {
        ICACTIVER4_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///For interrupts ID
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gicd_icactiver4](index.html) module
pub struct GICD_ICACTIVER4_SPEC;
impl crate::RegisterSpec for GICD_ICACTIVER4_SPEC {
    type Ux = u32;
}
///`read()` method returns [gicd_icactiver4::R](R) reader structure
impl crate::Readable for GICD_ICACTIVER4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gicd_icactiver4::W](W) writer structure
impl crate::Writable for GICD_ICACTIVER4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GICD_ICACTIVER4 to value 0
impl crate::Resettable for GICD_ICACTIVER4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
