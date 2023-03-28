///Register `GICD_ISACTIVER3` reader
pub struct R(crate::R<GICD_ISACTIVER3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ISACTIVER3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ISACTIVER3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ISACTIVER3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GICD_ISACTIVER3` writer
pub struct W(crate::W<GICD_ISACTIVER3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ISACTIVER3_SPEC>;
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
impl From<crate::W<GICD_ISACTIVER3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ISACTIVER3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ISACTIVER3` reader - ISACTIVER3
pub type ISACTIVER3_R = crate::FieldReader<u32, u32>;
///Field `ISACTIVER3` writer - ISACTIVER3
pub type ISACTIVER3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ISACTIVER3_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - ISACTIVER3
    #[inline(always)]
    pub fn isactiver3(&self) -> ISACTIVER3_R {
        ISACTIVER3_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - ISACTIVER3
    #[inline(always)]
    #[must_use]
    pub fn isactiver3(&mut self) -> ISACTIVER3_W<0> {
        ISACTIVER3_W::new(self)
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
///For information about available fields see [gicd_isactiver3](index.html) module
pub struct GICD_ISACTIVER3_SPEC;
impl crate::RegisterSpec for GICD_ISACTIVER3_SPEC {
    type Ux = u32;
}
///`read()` method returns [gicd_isactiver3::R](R) reader structure
impl crate::Readable for GICD_ISACTIVER3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gicd_isactiver3::W](W) writer structure
impl crate::Writable for GICD_ISACTIVER3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GICD_ISACTIVER3 to value 0
impl crate::Resettable for GICD_ISACTIVER3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
