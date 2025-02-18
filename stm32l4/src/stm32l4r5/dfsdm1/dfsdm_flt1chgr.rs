///Register `DFSDM_FLT1CHGR` reader
pub struct R(crate::R<DFSDM_FLT1CHGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_FLT1CHGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_FLT1CHGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_FLT1CHGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DFSDM_FLT1CHGR` writer
pub struct W(crate::W<DFSDM_FLT1CHGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSDM_FLT1CHGR_SPEC>;
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
impl From<crate::W<DFSDM_FLT1CHGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSDM_FLT1CHGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `JCHG` reader - Injected channel group selection
pub type JCHG_R = crate::FieldReader<u8, u8>;
///Field `JCHG` writer - Injected channel group selection
pub type JCHG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSDM_FLT1CHGR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - Injected channel group selection
    #[inline(always)]
    pub fn jchg(&self) -> JCHG_R {
        JCHG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Injected channel group selection
    #[inline(always)]
    #[must_use]
    pub fn jchg(&mut self) -> JCHG_W<0> {
        JCHG_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///injected channel group selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt1chgr](index.html) module
pub struct DFSDM_FLT1CHGR_SPEC;
impl crate::RegisterSpec for DFSDM_FLT1CHGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dfsdm_flt1chgr::R](R) reader structure
impl crate::Readable for DFSDM_FLT1CHGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dfsdm_flt1chgr::W](W) writer structure
impl crate::Writable for DFSDM_FLT1CHGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DFSDM_FLT1CHGR to value 0x01
impl crate::Resettable for DFSDM_FLT1CHGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
