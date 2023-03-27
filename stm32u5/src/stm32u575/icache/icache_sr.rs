///Register `ICACHE_SR` reader
pub struct R(crate::R<ICACHE_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_SR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `BUSYF` reader - BUSYF
pub type BUSYF_R = crate::BitReader<bool>;
///Field `BSYENDF` reader - BSYENDF
pub type BSYENDF_R = crate::BitReader<bool>;
///Field `ERRF` reader - ERRF
pub type ERRF_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - BUSYF
    #[inline(always)]
    pub fn busyf(&self) -> BUSYF_R {
        BUSYF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BSYENDF
    #[inline(always)]
    pub fn bsyendf(&self) -> BSYENDF_R {
        BSYENDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ERRF
    #[inline(always)]
    pub fn errf(&self) -> ERRF_R {
        ERRF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
///ICACHE status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icache_sr](index.html) module
pub struct ICACHE_SR_SPEC;
impl crate::RegisterSpec for ICACHE_SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [icache_sr::R](R) reader structure
impl crate::Readable for ICACHE_SR_SPEC {
    type Reader = R;
}
///`reset()` method sets ICACHE_SR to value 0x01
impl crate::Resettable for ICACHE_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
