///Register `IVR3` reader
pub struct R(crate::R<IVR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IVR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IVR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IVR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IVR3` writer
pub struct W(crate::W<IVR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IVR3_SPEC>;
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
impl From<crate::W<IVR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IVR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IVR3` reader - Initialization Vector Register
pub type IVR3_R = crate::FieldReader<u32, u32>;
///Field `IVR3` writer - Initialization Vector Register
pub type IVR3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IVR3_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Initialization Vector Register
    #[inline(always)]
    pub fn ivr3(&self) -> IVR3_R {
        IVR3_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Initialization Vector Register
    #[inline(always)]
    #[must_use]
    pub fn ivr3(&mut self) -> IVR3_W<0> {
        IVR3_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Initialization Vector Register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ivr3](index.html) module
pub struct IVR3_SPEC;
impl crate::RegisterSpec for IVR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [ivr3::R](R) reader structure
impl crate::Readable for IVR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ivr3::W](W) writer structure
impl crate::Writable for IVR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IVR3 to value 0
impl crate::Resettable for IVR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
