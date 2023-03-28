///Register `LPTIM_ARR` reader
pub struct R(crate::R<LPTIM_ARR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTIM_ARR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTIM_ARR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTIM_ARR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LPTIM_ARR` writer
pub struct W(crate::W<LPTIM_ARR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPTIM_ARR_SPEC>;
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
impl From<crate::W<LPTIM_ARR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPTIM_ARR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ARR` reader - ARR
pub type ARR_R = crate::FieldReader<u16, u16>;
///Field `ARR` writer - ARR
pub type ARR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTIM_ARR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - ARR
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - ARR
    #[inline(always)]
    #[must_use]
    pub fn arr(&mut self) -> ARR_W<0> {
        ARR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LPTIM autoreload register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lptim_arr](index.html) module
pub struct LPTIM_ARR_SPEC;
impl crate::RegisterSpec for LPTIM_ARR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lptim_arr::R](R) reader structure
impl crate::Readable for LPTIM_ARR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lptim_arr::W](W) writer structure
impl crate::Writable for LPTIM_ARR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LPTIM_ARR to value 0x01
impl crate::Resettable for LPTIM_ARR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
