///Register `BPCR` reader
pub struct R(crate::R<BPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BPCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BPCR` writer
pub struct W(crate::W<BPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BPCR_SPEC>;
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
impl From<crate::W<BPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BPCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AVBP` reader - Accumulated Vertical back porch (in units of horizontal scan line)
pub type AVBP_R = crate::FieldReader<u16, u16>;
///Field `AVBP` writer - Accumulated Vertical back porch (in units of horizontal scan line)
pub type AVBP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BPCR_SPEC, u16, u16, 11, O>;
///Field `AHBP` reader - Accumulated Horizontal back porch (in units of pixel clock period)
pub type AHBP_R = crate::FieldReader<u16, u16>;
///Field `AHBP` writer - Accumulated Horizontal back porch (in units of pixel clock period)
pub type AHBP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BPCR_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 0:10 - Accumulated Vertical back porch (in units of horizontal scan line)
    #[inline(always)]
    pub fn avbp(&self) -> AVBP_R {
        AVBP_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:27 - Accumulated Horizontal back porch (in units of pixel clock period)
    #[inline(always)]
    pub fn ahbp(&self) -> AHBP_R {
        AHBP_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Accumulated Vertical back porch (in units of horizontal scan line)
    #[inline(always)]
    #[must_use]
    pub fn avbp(&mut self) -> AVBP_W<0> {
        AVBP_W::new(self)
    }
    ///Bits 16:27 - Accumulated Horizontal back porch (in units of pixel clock period)
    #[inline(always)]
    #[must_use]
    pub fn ahbp(&mut self) -> AHBP_W<16> {
        AHBP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Back Porch Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bpcr](index.html) module
pub struct BPCR_SPEC;
impl crate::RegisterSpec for BPCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [bpcr::R](R) reader structure
impl crate::Readable for BPCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bpcr::W](W) writer structure
impl crate::Writable for BPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BPCR to value 0
impl crate::Resettable for BPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
