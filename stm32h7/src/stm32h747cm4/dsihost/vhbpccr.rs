///Register `VHBPCCR` reader
pub struct R(crate::R<VHBPCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VHBPCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VHBPCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VHBPCCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `VHBPCCR` writer
pub struct W(crate::W<VHBPCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VHBPCCR_SPEC>;
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
impl From<crate::W<VHBPCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VHBPCCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HBP` reader - Horizontal back
pub type HBP_R = crate::FieldReader<u16, u16>;
///Field `HBP` writer - Horizontal back
pub type HBP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VHBPCCR_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 0:11 - Horizontal back
    #[inline(always)]
    pub fn hbp(&self) -> HBP_R {
        HBP_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - Horizontal back
    #[inline(always)]
    #[must_use]
    pub fn hbp(&mut self) -> HBP_W<0> {
        HBP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host video HBP current configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vhbpccr](index.html) module
pub struct VHBPCCR_SPEC;
impl crate::RegisterSpec for VHBPCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [vhbpccr::R](R) reader structure
impl crate::Readable for VHBPCCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [vhbpccr::W](W) writer structure
impl crate::Writable for VHBPCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets VHBPCCR to value 0
impl crate::Resettable for VHBPCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
