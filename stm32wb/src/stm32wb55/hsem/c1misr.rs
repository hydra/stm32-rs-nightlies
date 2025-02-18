///Register `C1MISR` reader
pub struct R(crate::R<C1MISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1MISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1MISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1MISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `MISFm` reader - masked CPU(n) semaphore m status bit after enable (mask).
pub type MISFM_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - masked CPU(n) semaphore m status bit after enable (mask).
    #[inline(always)]
    pub fn misfm(&self) -> MISFM_R {
        MISFM_R::new(self.bits)
    }
}
///HSEM Masked interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c1misr](index.html) module
pub struct C1MISR_SPEC;
impl crate::RegisterSpec for C1MISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c1misr::R](R) reader structure
impl crate::Readable for C1MISR_SPEC {
    type Reader = R;
}
///`reset()` method sets C1MISR to value 0
impl crate::Resettable for C1MISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
