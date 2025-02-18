///Register `ABR` reader
pub struct R(crate::R<ABR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ABR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ABR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ABR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ABR` writer
pub struct W(crate::W<ABR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ABR_SPEC>;
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
impl From<crate::W<ABR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ABR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIMEOUT` reader - Timeout period
pub type TIMEOUT_R = crate::FieldReader<u16, u16>;
///Field `TIMEOUT` writer - Timeout period
pub type TIMEOUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ABR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Timeout period
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Timeout period
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<0> {
        TIMEOUT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///alternate bytes register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [abr](index.html) module
pub struct ABR_SPEC;
impl crate::RegisterSpec for ABR_SPEC {
    type Ux = u32;
}
///`read()` method returns [abr::R](R) reader structure
impl crate::Readable for ABR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [abr::W](W) writer structure
impl crate::Writable for ABR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ABR to value 0
impl crate::Resettable for ABR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
