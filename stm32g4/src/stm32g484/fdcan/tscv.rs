///Register `TSCV` reader
pub struct R(crate::R<TSCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSCV_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TSCV` writer
pub struct W(crate::W<TSCV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSCV_SPEC>;
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
impl From<crate::W<TSCV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSCV_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TSC` reader - TSC
pub type TSC_R = crate::FieldReader<u16, u16>;
///Field `TSC` writer - TSC
pub type TSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSCV_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - TSC
    #[inline(always)]
    pub fn tsc(&self) -> TSC_R {
        TSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - TSC
    #[inline(always)]
    #[must_use]
    pub fn tsc(&mut self) -> TSC_W<0> {
        TSC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Timestamp Counter Value Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tscv](index.html) module
pub struct TSCV_SPEC;
impl crate::RegisterSpec for TSCV_SPEC {
    type Ux = u32;
}
///`read()` method returns [tscv::R](R) reader structure
impl crate::Readable for TSCV_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tscv::W](W) writer structure
impl crate::Writable for TSCV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TSCV to value 0
impl crate::Resettable for TSCV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
