///Register `AWCR` reader
pub struct R(crate::R<AWCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AWCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AWCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AWCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AWCR` writer
pub struct W(crate::W<AWCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AWCR_SPEC>;
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
impl From<crate::W<AWCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AWCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AAH` reader - Accumulated Active Height (in units of horizontal scan line)
pub type AAH_R = crate::FieldReader<u16, u16>;
///Field `AAH` writer - Accumulated Active Height (in units of horizontal scan line)
pub type AAH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, AWCR_SPEC, u16, u16, 11, O>;
///Field `AAW` reader - Accumulated Active Width (in units of pixel clock period)
pub type AAW_R = crate::FieldReader<u16, u16>;
///Field `AAW` writer - Accumulated Active Width (in units of pixel clock period)
pub type AAW_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, AWCR_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 0:10 - Accumulated Active Height (in units of horizontal scan line)
    #[inline(always)]
    pub fn aah(&self) -> AAH_R {
        AAH_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:27 - Accumulated Active Width (in units of pixel clock period)
    #[inline(always)]
    pub fn aaw(&self) -> AAW_R {
        AAW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:10 - Accumulated Active Height (in units of horizontal scan line)
    #[inline(always)]
    #[must_use]
    pub fn aah(&mut self) -> AAH_W<0> {
        AAH_W::new(self)
    }
    ///Bits 16:27 - Accumulated Active Width (in units of pixel clock period)
    #[inline(always)]
    #[must_use]
    pub fn aaw(&mut self) -> AAW_W<16> {
        AAW_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Active Width Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [awcr](index.html) module
pub struct AWCR_SPEC;
impl crate::RegisterSpec for AWCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [awcr::R](R) reader structure
impl crate::Readable for AWCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [awcr::W](W) writer structure
impl crate::Writable for AWCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AWCR to value 0
impl crate::Resettable for AWCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
