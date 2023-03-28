///Register `MPCWM3_UPWMR` reader
pub struct R(crate::R<MPCWM3_UPWMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPCWM3_UPWMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPCWM3_UPWMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPCWM3_UPWMR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MPCWM3_UPWMR` writer
pub struct W(crate::W<MPCWM3_UPWMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPCWM3_UPWMR_SPEC>;
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
impl From<crate::W<MPCWM3_UPWMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPCWM3_UPWMR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LGTH` reader - LGTH
pub type LGTH_R = crate::FieldReader<u16, u16>;
///Field `LGTH` writer - LGTH
pub type LGTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPCWM3_UPWMR_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 16:27 - LGTH
    #[inline(always)]
    pub fn lgth(&self) -> LGTH_R {
        LGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 16:27 - LGTH
    #[inline(always)]
    #[must_use]
    pub fn lgth(&mut self) -> LGTH_W<16> {
        LGTH_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Unprivileged Water Mark 3 register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mpcwm3_upwmr](index.html) module
pub struct MPCWM3_UPWMR_SPEC;
impl crate::RegisterSpec for MPCWM3_UPWMR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mpcwm3_upwmr::R](R) reader structure
impl crate::Readable for MPCWM3_UPWMR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mpcwm3_upwmr::W](W) writer structure
impl crate::Writable for MPCWM3_UPWMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MPCWM3_UPWMR to value 0x0fff_0000
impl crate::Resettable for MPCWM3_UPWMR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_0000;
}
