///Register `GICD_ISENABLER0` reader
pub struct R(crate::R<GICD_ISENABLER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ISENABLER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ISENABLER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ISENABLER0_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GICD_ISENABLER0` writer
pub struct W(crate::W<GICD_ISENABLER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ISENABLER0_SPEC>;
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
impl From<crate::W<GICD_ISENABLER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ISENABLER0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ISENABLER0` reader - ISENABLER0
pub type ISENABLER0_R = crate::FieldReader<u32, u32>;
///Field `ISENABLER0` writer - ISENABLER0
pub type ISENABLER0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ISENABLER0_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - ISENABLER0
    #[inline(always)]
    pub fn isenabler0(&self) -> ISENABLER0_R {
        ISENABLER0_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - ISENABLER0
    #[inline(always)]
    #[must_use]
    pub fn isenabler0(&mut self) -> ISENABLER0_W<0> {
        ISENABLER0_W::new(self)
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
///For information about available fields see [gicd_isenabler0](index.html) module
pub struct GICD_ISENABLER0_SPEC;
impl crate::RegisterSpec for GICD_ISENABLER0_SPEC {
    type Ux = u32;
}
///`read()` method returns [gicd_isenabler0::R](R) reader structure
impl crate::Readable for GICD_ISENABLER0_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gicd_isenabler0::W](W) writer structure
impl crate::Writable for GICD_ISENABLER0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GICD_ISENABLER0 to value 0xffff
impl crate::Resettable for GICD_ISENABLER0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
