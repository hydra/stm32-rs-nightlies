///Register `CWSTRT` reader
pub struct R(crate::R<CWSTRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CWSTRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CWSTRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CWSTRT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CWSTRT` writer
pub struct W(crate::W<CWSTRT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CWSTRT_SPEC>;
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
impl From<crate::W<CWSTRT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CWSTRT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HOFFCNT` reader - Horizontal offset count
pub type HOFFCNT_R = crate::FieldReader<u16, u16>;
///Field `HOFFCNT` writer - Horizontal offset count
pub type HOFFCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CWSTRT_SPEC, u16, u16, 14, O>;
///Field `VST` reader - Vertical start line count
pub type VST_R = crate::FieldReader<u16, u16>;
///Field `VST` writer - Vertical start line count
pub type VST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CWSTRT_SPEC, u16, u16, 13, O>;
impl R {
    ///Bits 0:13 - Horizontal offset count
    #[inline(always)]
    pub fn hoffcnt(&self) -> HOFFCNT_R {
        HOFFCNT_R::new((self.bits & 0x3fff) as u16)
    }
    ///Bits 16:28 - Vertical start line count
    #[inline(always)]
    pub fn vst(&self) -> VST_R {
        VST_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    ///Bits 0:13 - Horizontal offset count
    #[inline(always)]
    #[must_use]
    pub fn hoffcnt(&mut self) -> HOFFCNT_W<0> {
        HOFFCNT_W::new(self)
    }
    ///Bits 16:28 - Vertical start line count
    #[inline(always)]
    #[must_use]
    pub fn vst(&mut self) -> VST_W<16> {
        VST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///crop window start
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cwstrt](index.html) module
pub struct CWSTRT_SPEC;
impl crate::RegisterSpec for CWSTRT_SPEC {
    type Ux = u32;
}
///`read()` method returns [cwstrt::R](R) reader structure
impl crate::Readable for CWSTRT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cwstrt::W](W) writer structure
impl crate::Writable for CWSTRT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CWSTRT to value 0
impl crate::Resettable for CWSTRT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
