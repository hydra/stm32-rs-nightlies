///Register `VVSACCR` reader
pub struct R(crate::R<VVSACCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VVSACCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VVSACCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VVSACCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `VVSACCR` writer
pub struct W(crate::W<VVSACCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VVSACCR_SPEC>;
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
impl From<crate::W<VVSACCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VVSACCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `VSA` reader - Vertical Synchronism Active duration
pub type VSA_R = crate::FieldReader<u16, u16>;
///Field `VSA` writer - Vertical Synchronism Active duration
pub type VSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VVSACCR_SPEC, u16, u16, 10, O>;
impl R {
    ///Bits 0:9 - Vertical Synchronism Active duration
    #[inline(always)]
    pub fn vsa(&self) -> VSA_R {
        VSA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - Vertical Synchronism Active duration
    #[inline(always)]
    #[must_use]
    pub fn vsa(&mut self) -> VSA_W<0> {
        VSA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host Video VSA Current Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vvsaccr](index.html) module
pub struct VVSACCR_SPEC;
impl crate::RegisterSpec for VVSACCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [vvsaccr::R](R) reader structure
impl crate::Readable for VVSACCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [vvsaccr::W](W) writer structure
impl crate::Writable for VVSACCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets VVSACCR to value 0
impl crate::Resettable for VVSACCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
