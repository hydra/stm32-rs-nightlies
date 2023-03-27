///Register `IDMABAR` reader
pub struct R(crate::R<IDMABAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDMABAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDMABAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDMABAR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IDMABAR` writer
pub struct W(crate::W<IDMABAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDMABAR_SPEC>;
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
impl From<crate::W<IDMABAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDMABAR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IDMABA` reader - Word aligned Linked list memory base address Linked list memory base pointer. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type IDMABA_R = crate::FieldReader<u32, u32>;
///Field `IDMABA` writer - Word aligned Linked list memory base address Linked list memory base pointer. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0).
pub type IDMABA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDMABAR_SPEC, u32, u32, 30, O>;
impl R {
    ///Bits 2:31 - Word aligned Linked list memory base address Linked list memory base pointer. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    pub fn idmaba(&self) -> IDMABA_R {
        IDMABA_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    ///Bits 2:31 - Word aligned Linked list memory base address Linked list memory base pointer. These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0).
    #[inline(always)]
    #[must_use]
    pub fn idmaba(&mut self) -> IDMABA_W<2> {
        IDMABA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SDMMC_IDMABAR
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [idmabar](index.html) module
pub struct IDMABAR_SPEC;
impl crate::RegisterSpec for IDMABAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [idmabar::R](R) reader structure
impl crate::Readable for IDMABAR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [idmabar::W](W) writer structure
impl crate::Writable for IDMABAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IDMABAR to value 0
impl crate::Resettable for IDMABAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
