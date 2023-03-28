///Register `GICD_ICENABLER0` reader
pub struct R(crate::R<GICD_ICENABLER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ICENABLER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ICENABLER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ICENABLER0_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GICD_ICENABLER0` writer
pub struct W(crate::W<GICD_ICENABLER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ICENABLER0_SPEC>;
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
impl From<crate::W<GICD_ICENABLER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ICENABLER0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ICENABLER0` reader - ICENABLER0
pub type ICENABLER0_R = crate::FieldReader<u32, u32>;
///Field `ICENABLER0` writer - ICENABLER0
pub type ICENABLER0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ICENABLER0_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - ICENABLER0
    #[inline(always)]
    pub fn icenabler0(&self) -> ICENABLER0_R {
        ICENABLER0_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - ICENABLER0
    #[inline(always)]
    #[must_use]
    pub fn icenabler0(&mut self) -> ICENABLER0_W<0> {
        ICENABLER0_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///For interrupts ID = 0 to ID = 31
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gicd_icenabler0](index.html) module
pub struct GICD_ICENABLER0_SPEC;
impl crate::RegisterSpec for GICD_ICENABLER0_SPEC {
    type Ux = u32;
}
///`read()` method returns [gicd_icenabler0::R](R) reader structure
impl crate::Readable for GICD_ICENABLER0_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gicd_icenabler0::W](W) writer structure
impl crate::Writable for GICD_ICENABLER0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GICD_ICENABLER0 to value 0xffff
impl crate::Resettable for GICD_ICENABLER0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
