///Register `CALFACT` reader
pub struct R(crate::R<CALFACT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALFACT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALFACT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALFACT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CALFACT` writer
pub struct W(crate::W<CALFACT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALFACT_SPEC>;
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
impl From<crate::W<CALFACT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALFACT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CALFACT` reader - CALFACT
pub type CALFACT_R = crate::FieldReader<u8, u8>;
///Field `CALFACT` writer - CALFACT
pub type CALFACT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CALFACT_SPEC, u8, u8, 7, O>;
impl R {
    ///Bits 0:6 - CALFACT
    #[inline(always)]
    pub fn calfact(&self) -> CALFACT_R {
        CALFACT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:6 - CALFACT
    #[inline(always)]
    #[must_use]
    pub fn calfact(&mut self) -> CALFACT_W<0> {
        CALFACT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC Calibration factor
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [calfact](index.html) module
pub struct CALFACT_SPEC;
impl crate::RegisterSpec for CALFACT_SPEC {
    type Ux = u32;
}
///`read()` method returns [calfact::R](R) reader structure
impl crate::Readable for CALFACT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [calfact::W](W) writer structure
impl crate::Writable for CALFACT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CALFACT to value 0
impl crate::Resettable for CALFACT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
