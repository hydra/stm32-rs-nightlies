///Register `SMPR2` reader
pub struct R(crate::R<SMPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SMPR2` writer
pub struct W(crate::W<SMPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR2_SPEC>;
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
impl From<crate::W<SMPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SMP` reader - Channel sampling time selection
pub type SMP_R = crate::FieldReader<u32, u32>;
///Field `SMP` writer - Channel sampling time selection
pub type SMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPR2_SPEC, u32, u32, 30, O>;
impl R {
    ///Bits 0:29 - Channel sampling time selection
    #[inline(always)]
    pub fn smp(&self) -> SMP_R {
        SMP_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    ///Bits 0:29 - Channel sampling time selection
    #[inline(always)]
    #[must_use]
    pub fn smp(&mut self) -> SMP_W<0> {
        SMP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///sample time register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [smpr2](index.html) module
pub struct SMPR2_SPEC;
impl crate::RegisterSpec for SMPR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [smpr2::R](R) reader structure
impl crate::Readable for SMPR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [smpr2::W](W) writer structure
impl crate::Writable for SMPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SMPR2 to value 0
impl crate::Resettable for SMPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
