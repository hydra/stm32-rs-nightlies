///Register `STIR` reader
pub struct R(crate::R<STIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STIR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `STIR` writer
pub struct W(crate::W<STIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STIR_SPEC>;
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
impl From<crate::W<STIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STIR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `INTID` reader - Software generated interrupt ID
pub type INTID_R = crate::FieldReader<u16, u16>;
///Field `INTID` writer - Software generated interrupt ID
pub type INTID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STIR_SPEC, u16, u16, 9, O>;
impl R {
    ///Bits 0:8 - Software generated interrupt ID
    #[inline(always)]
    pub fn intid(&self) -> INTID_R {
        INTID_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    ///Bits 0:8 - Software generated interrupt ID
    #[inline(always)]
    #[must_use]
    pub fn intid(&mut self) -> INTID_W<0> {
        INTID_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Software trigger interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [stir](index.html) module
pub struct STIR_SPEC;
impl crate::RegisterSpec for STIR_SPEC {
    type Ux = u32;
}
///`read()` method returns [stir::R](R) reader structure
impl crate::Readable for STIR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [stir::W](W) writer structure
impl crate::Writable for STIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets STIR to value 0
impl crate::Resettable for STIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
