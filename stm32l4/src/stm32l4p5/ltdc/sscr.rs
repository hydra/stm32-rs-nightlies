///Register `SSCR` reader
pub struct R(crate::R<SSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SSCR` writer
pub struct W(crate::W<SSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSCR_SPEC>;
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
impl From<crate::W<SSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `VSH` reader - Vertical Synchronization Height (in units of horizontal scan line)
pub type VSH_R = crate::FieldReader<u16, u16>;
///Field `VSH` writer - Vertical Synchronization Height (in units of horizontal scan line)
pub type VSH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SSCR_SPEC, u16, u16, 11, O>;
///Field `HSW` reader - Horizontal Synchronization Width (in units of pixel clock period)
pub type HSW_R = crate::FieldReader<u16, u16>;
///Field `HSW` writer - Horizontal Synchronization Width (in units of pixel clock period)
pub type HSW_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SSCR_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 0:10 - Vertical Synchronization Height (in units of horizontal scan line)
    #[inline(always)]
    pub fn vsh(&self) -> VSH_R {
        VSH_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:27 - Horizontal Synchronization Width (in units of pixel clock period)
    #[inline(always)]
    pub fn hsw(&self) -> HSW_R {
        HSW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Vertical Synchronization Height (in units of horizontal scan line)
    #[inline(always)]
    #[must_use]
    pub fn vsh(&mut self) -> VSH_W<0> {
        VSH_W::new(self)
    }
    ///Bits 16:27 - Horizontal Synchronization Width (in units of pixel clock period)
    #[inline(always)]
    #[must_use]
    pub fn hsw(&mut self) -> HSW_W<16> {
        HSW_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LTDC Synchronization Size Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sscr](index.html) module
pub struct SSCR_SPEC;
impl crate::RegisterSpec for SSCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sscr::R](R) reader structure
impl crate::Readable for SSCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sscr::W](W) writer structure
impl crate::Writable for SSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SSCR to value 0
impl crate::Resettable for SSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
