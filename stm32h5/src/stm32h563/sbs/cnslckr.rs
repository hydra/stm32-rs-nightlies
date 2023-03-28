///Register `CNSLCKR` reader
pub struct R(crate::R<CNSLCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNSLCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNSLCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNSLCKR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CNSLCKR` writer
pub struct W(crate::W<CNSLCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNSLCKR_SPEC>;
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
impl From<crate::W<CNSLCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNSLCKR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LOCKNSVTOR` reader - VTOR_NS register lock This bit is set by software and cleared only by a system reset.
pub type LOCKNSVTOR_R = crate::BitReader<bool>;
///Field `LOCKNSVTOR` writer - VTOR_NS register lock This bit is set by software and cleared only by a system reset.
pub type LOCKNSVTOR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNSLCKR_SPEC, bool, O>;
///Field `LOCKNSMPU` reader - non-secure MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to non-secure MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers.
pub type LOCKNSMPU_R = crate::BitReader<bool>;
///Field `LOCKNSMPU` writer - non-secure MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to non-secure MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers.
pub type LOCKNSMPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNSLCKR_SPEC, bool, O>;
impl R {
    ///Bit 0 - VTOR_NS register lock This bit is set by software and cleared only by a system reset.
    #[inline(always)]
    pub fn locknsvtor(&self) -> LOCKNSVTOR_R {
        LOCKNSVTOR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - non-secure MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to non-secure MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers.
    #[inline(always)]
    pub fn locknsmpu(&self) -> LOCKNSMPU_R {
        LOCKNSMPU_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - VTOR_NS register lock This bit is set by software and cleared only by a system reset.
    #[inline(always)]
    #[must_use]
    pub fn locknsvtor(&mut self) -> LOCKNSVTOR_W<0> {
        LOCKNSVTOR_W::new(self)
    }
    ///Bit 1 - non-secure MPU register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to non-secure MPU_CTRL_NS, MPU_RNR_NS and MPU_RBAR_NS registers.
    #[inline(always)]
    #[must_use]
    pub fn locknsmpu(&mut self) -> LOCKNSMPU_W<1> {
        LOCKNSMPU_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SBS CPU non-secure lock register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cnslckr](index.html) module
pub struct CNSLCKR_SPEC;
impl crate::RegisterSpec for CNSLCKR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cnslckr::R](R) reader structure
impl crate::Readable for CNSLCKR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cnslckr::W](W) writer structure
impl crate::Writable for CNSLCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CNSLCKR to value 0
impl crate::Resettable for CNSLCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
