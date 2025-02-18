///Register `IVR0` reader
pub struct R(crate::R<IVR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IVR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IVR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IVR0_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IVR0` writer
pub struct W(crate::W<IVR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IVR0_SPEC>;
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
impl From<crate::W<IVR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IVR0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IVI` reader - Initialization vector input, bits \[31:0\]
///Refer to for description of the IVI\[127:0\]
///bitfield. The initialization vector is only used in chaining modes other than ECB. The SAES_IVRx registers may be written only when the SAES peripheral is disabled
pub type IVI_R = crate::FieldReader<u32, u32>;
///Field `IVI` writer - Initialization vector input, bits \[31:0\]
///Refer to for description of the IVI\[127:0\]
///bitfield. The initialization vector is only used in chaining modes other than ECB. The SAES_IVRx registers may be written only when the SAES peripheral is disabled
pub type IVI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IVR0_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Initialization vector input, bits \[31:0\]
    ///Refer to for description of the IVI\[127:0\]
    ///bitfield. The initialization vector is only used in chaining modes other than ECB. The SAES_IVRx registers may be written only when the SAES peripheral is disabled
    #[inline(always)]
    pub fn ivi(&self) -> IVI_R {
        IVI_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Initialization vector input, bits \[31:0\]
    ///Refer to for description of the IVI\[127:0\]
    ///bitfield. The initialization vector is only used in chaining modes other than ECB. The SAES_IVRx registers may be written only when the SAES peripheral is disabled
    #[inline(always)]
    #[must_use]
    pub fn ivi(&mut self) -> IVI_W<0> {
        IVI_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SAES initialization vector register 0
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ivr0](index.html) module
pub struct IVR0_SPEC;
impl crate::RegisterSpec for IVR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [ivr0::R](R) reader structure
impl crate::Readable for IVR0_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ivr0::W](W) writer structure
impl crate::Writable for IVR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IVR0 to value 0
impl crate::Resettable for IVR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
