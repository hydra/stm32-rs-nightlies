///Register `VR` reader
pub struct R(crate::R<VR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `VR` writer
pub struct W(crate::W<VR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VR_SPEC>;
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
impl From<crate::W<VR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `VERSION` reader - Version of the DSI Host
pub type VERSION_R = crate::FieldReader<u32, u32>;
///Field `VERSION` writer - Version of the DSI Host
pub type VERSION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Version of the DSI Host
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Version of the DSI Host
    #[inline(always)]
    #[must_use]
    pub fn version(&mut self) -> VERSION_W<0> {
        VERSION_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host Version Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vr](index.html) module
pub struct VR_SPEC;
impl crate::RegisterSpec for VR_SPEC {
    type Ux = u32;
}
///`read()` method returns [vr::R](R) reader structure
impl crate::Readable for VR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [vr::W](W) writer structure
impl crate::Writable for VR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets VR to value 0x3133_302a
impl crate::Resettable for VR_SPEC {
    const RESET_VALUE: Self::Ux = 0x3133_302a;
}
