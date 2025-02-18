///Register `CNTL` reader
pub struct R(crate::R<CNTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CNTL` writer
pub struct W(crate::W<CNTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTL_SPEC>;
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
impl From<crate::W<CNTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CNTL` reader - RTC counter register Low
pub type CNTL_R = crate::FieldReader<u16, u16>;
///Field `CNTL` writer - RTC counter register Low
pub type CNTL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNTL_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - RTC counter register Low
    #[inline(always)]
    pub fn cntl(&self) -> CNTL_R {
        CNTL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - RTC counter register Low
    #[inline(always)]
    #[must_use]
    pub fn cntl(&mut self) -> CNTL_W<0> {
        CNTL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RTC Counter Register Low
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cntl](index.html) module
pub struct CNTL_SPEC;
impl crate::RegisterSpec for CNTL_SPEC {
    type Ux = u32;
}
///`read()` method returns [cntl::R](R) reader structure
impl crate::Readable for CNTL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cntl::W](W) writer structure
impl crate::Writable for CNTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CNTL to value 0
impl crate::Resettable for CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
