///Register `CSLCKR` reader
pub struct R(crate::R<CSLCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSLCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSLCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSLCKR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSLCKR` writer
pub struct W(crate::W<CSLCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSLCKR_SPEC>;
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
impl From<crate::W<CSLCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSLCKR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LOCKSVTAIRCR` reader - VTOR_S and AIRCR register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to VTOR_S register, PRIS and BFHFNMINS bits in the AIRCR register.
pub type LOCKSVTAIRCR_R = crate::BitReader<bool>;
///Field `LOCKSVTAIRCR` writer - VTOR_S and AIRCR register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to VTOR_S register, PRIS and BFHFNMINS bits in the AIRCR register.
pub type LOCKSVTAIRCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSLCKR_SPEC, bool, O>;
///Field `LOCKSMPU` reader - secure MPU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to secure MPU_CTRL, MPU_RNR and MPU_RBAR registers.
pub type LOCKSMPU_R = crate::BitReader<bool>;
///Field `LOCKSMPU` writer - secure MPU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to secure MPU_CTRL, MPU_RNR and MPU_RBAR registers.
pub type LOCKSMPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSLCKR_SPEC, bool, O>;
///Field `LOCKSAU` reader - SAU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers.
pub type LOCKSAU_R = crate::BitReader<bool>;
///Field `LOCKSAU` writer - SAU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers.
pub type LOCKSAU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSLCKR_SPEC, bool, O>;
impl R {
    ///Bit 0 - VTOR_S and AIRCR register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to VTOR_S register, PRIS and BFHFNMINS bits in the AIRCR register.
    #[inline(always)]
    pub fn locksvtaircr(&self) -> LOCKSVTAIRCR_R {
        LOCKSVTAIRCR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - secure MPU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to secure MPU_CTRL, MPU_RNR and MPU_RBAR registers.
    #[inline(always)]
    pub fn locksmpu(&self) -> LOCKSMPU_R {
        LOCKSMPU_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SAU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers.
    #[inline(always)]
    pub fn locksau(&self) -> LOCKSAU_R {
        LOCKSAU_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - VTOR_S and AIRCR register lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to VTOR_S register, PRIS and BFHFNMINS bits in the AIRCR register.
    #[inline(always)]
    #[must_use]
    pub fn locksvtaircr(&mut self) -> LOCKSVTAIRCR_W<0> {
        LOCKSVTAIRCR_W::new(self)
    }
    ///Bit 1 - secure MPU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to secure MPU_CTRL, MPU_RNR and MPU_RBAR registers.
    #[inline(always)]
    #[must_use]
    pub fn locksmpu(&mut self) -> LOCKSMPU_W<1> {
        LOCKSMPU_W::new(self)
    }
    ///Bit 2 - SAU registers lock This bit is set by software and cleared only by a system reset. When set, this bit disables write access to SAU_CTRL, SAU_RNR, SAU_RBAR and SAU_RLAR registers.
    #[inline(always)]
    #[must_use]
    pub fn locksau(&mut self) -> LOCKSAU_W<2> {
        LOCKSAU_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SBS CPU secure lock register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cslckr](index.html) module
pub struct CSLCKR_SPEC;
impl crate::RegisterSpec for CSLCKR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cslckr::R](R) reader structure
impl crate::Readable for CSLCKR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cslckr::W](W) writer structure
impl crate::Writable for CSLCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSLCKR to value 0
impl crate::Resettable for CSLCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
