///Register `VVFPCR` reader
pub struct R(crate::R<VVFPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VVFPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VVFPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VVFPCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `VVFPCR` writer
pub struct W(crate::W<VVFPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VVFPCR_SPEC>;
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
impl From<crate::W<VVFPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VVFPCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `VFP` reader - Vertical front-porch duration
pub type VFP_R = crate::FieldReader<u16, u16>;
///Field `VFP` writer - Vertical front-porch duration
pub type VFP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VVFPCR_SPEC, u16, u16, 10, O>;
impl R {
    ///Bits 0:9 - Vertical front-porch duration
    #[inline(always)]
    pub fn vfp(&self) -> VFP_R {
        VFP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - Vertical front-porch duration
    #[inline(always)]
    #[must_use]
    pub fn vfp(&mut self) -> VFP_W<0> {
        VFP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host video VFP configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vvfpcr](index.html) module
pub struct VVFPCR_SPEC;
impl crate::RegisterSpec for VVFPCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [vvfpcr::R](R) reader structure
impl crate::Readable for VVFPCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [vvfpcr::W](W) writer structure
impl crate::Writable for VVFPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets VVFPCR to value 0
impl crate::Resettable for VVFPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
