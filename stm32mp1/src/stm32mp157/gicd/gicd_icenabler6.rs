///Register `GICD_ICENABLER6` reader
pub struct R(crate::R<GICD_ICENABLER6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ICENABLER6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ICENABLER6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ICENABLER6_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GICD_ICENABLER6` writer
pub struct W(crate::W<GICD_ICENABLER6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ICENABLER6_SPEC>;
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
impl From<crate::W<GICD_ICENABLER6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ICENABLER6_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ICENABLER6` reader - ICENABLER6
pub type ICENABLER6_R = crate::FieldReader<u32, u32>;
///Field `ICENABLER6` writer - ICENABLER6
pub type ICENABLER6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ICENABLER6_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - ICENABLER6
    #[inline(always)]
    pub fn icenabler6(&self) -> ICENABLER6_R {
        ICENABLER6_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - ICENABLER6
    #[inline(always)]
    #[must_use]
    pub fn icenabler6(&mut self) -> ICENABLER6_W<0> {
        ICENABLER6_W::new(self)
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
///For information about available fields see [gicd_icenabler6](index.html) module
pub struct GICD_ICENABLER6_SPEC;
impl crate::RegisterSpec for GICD_ICENABLER6_SPEC {
    type Ux = u32;
}
///`read()` method returns [gicd_icenabler6::R](R) reader structure
impl crate::Readable for GICD_ICENABLER6_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gicd_icenabler6::W](W) writer structure
impl crate::Writable for GICD_ICENABLER6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GICD_ICENABLER6 to value 0
impl crate::Resettable for GICD_ICENABLER6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
