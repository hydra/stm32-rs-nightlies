///Register `VPCR` reader
pub struct R(crate::R<VPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VPCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `VPCR` writer
pub struct W(crate::W<VPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VPCR_SPEC>;
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
impl From<crate::W<VPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VPCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `VPSIZE` reader - Video Packet Size
pub type VPSIZE_R = crate::FieldReader<u16, u16>;
///Field `VPSIZE` writer - Video Packet Size
pub type VPSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VPCR_SPEC, u16, u16, 14, O>;
impl R {
    ///Bits 0:13 - Video Packet Size
    #[inline(always)]
    pub fn vpsize(&self) -> VPSIZE_R {
        VPSIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    ///Bits 0:13 - Video Packet Size
    #[inline(always)]
    #[must_use]
    pub fn vpsize(&mut self) -> VPSIZE_W<0> {
        VPSIZE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host Video Packet Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vpcr](index.html) module
pub struct VPCR_SPEC;
impl crate::RegisterSpec for VPCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [vpcr::R](R) reader structure
impl crate::Readable for VPCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [vpcr::W](W) writer structure
impl crate::Writable for VPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets VPCR to value 0
impl crate::Resettable for VPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
