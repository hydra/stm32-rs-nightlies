///Register `GICD_ICACTIVER5` reader
pub struct R(crate::R<GICD_ICACTIVER5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ICACTIVER5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ICACTIVER5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ICACTIVER5_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GICD_ICACTIVER5` writer
pub struct W(crate::W<GICD_ICACTIVER5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ICACTIVER5_SPEC>;
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
impl From<crate::W<GICD_ICACTIVER5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ICACTIVER5_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ICACTIVER5` reader - ICACTIVER5
pub type ICACTIVER5_R = crate::FieldReader<u32, u32>;
///Field `ICACTIVER5` writer - ICACTIVER5
pub type ICACTIVER5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ICACTIVER5_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - ICACTIVER5
    #[inline(always)]
    pub fn icactiver5(&self) -> ICACTIVER5_R {
        ICACTIVER5_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - ICACTIVER5
    #[inline(always)]
    #[must_use]
    pub fn icactiver5(&mut self) -> ICACTIVER5_W<0> {
        ICACTIVER5_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///For interrupts ID
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gicd_icactiver5](index.html) module
pub struct GICD_ICACTIVER5_SPEC;
impl crate::RegisterSpec for GICD_ICACTIVER5_SPEC {
    type Ux = u32;
}
///`read()` method returns [gicd_icactiver5::R](R) reader structure
impl crate::Readable for GICD_ICACTIVER5_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gicd_icactiver5::W](W) writer structure
impl crate::Writable for GICD_ICACTIVER5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GICD_ICACTIVER5 to value 0
impl crate::Resettable for GICD_ICACTIVER5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
