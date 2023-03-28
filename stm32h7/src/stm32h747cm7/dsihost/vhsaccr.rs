///Register `VHSACCR` reader
pub struct R(crate::R<VHSACCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VHSACCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VHSACCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VHSACCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `VHSACCR` writer
pub struct W(crate::W<VHSACCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VHSACCR_SPEC>;
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
impl From<crate::W<VHSACCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VHSACCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HSA` reader - Horizontal synchronism active duration
pub type HSA_R = crate::FieldReader<u16, u16>;
///Field `HSA` writer - Horizontal synchronism active duration
pub type HSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VHSACCR_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 0:11 - Horizontal synchronism active duration
    #[inline(always)]
    pub fn hsa(&self) -> HSA_R {
        HSA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - Horizontal synchronism active duration
    #[inline(always)]
    #[must_use]
    pub fn hsa(&mut self) -> HSA_W<0> {
        HSA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host video HSA current configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vhsaccr](index.html) module
pub struct VHSACCR_SPEC;
impl crate::RegisterSpec for VHSACCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [vhsaccr::R](R) reader structure
impl crate::Readable for VHSACCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [vhsaccr::W](W) writer structure
impl crate::Writable for VHSACCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets VHSACCR to value 0
impl crate::Resettable for VHSACCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
