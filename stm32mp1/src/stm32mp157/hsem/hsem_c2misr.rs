///Register `HSEM_C2MISR` reader
pub struct R(crate::R<HSEM_C2MISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSEM_C2MISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSEM_C2MISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSEM_C2MISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `MISF` reader - MISF
pub type MISF_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - MISF
    #[inline(always)]
    pub fn misf(&self) -> MISF_R {
        MISF_R::new(self.bits)
    }
}
///HSEM i2terrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hsem_c2misr](index.html) module
pub struct HSEM_C2MISR_SPEC;
impl crate::RegisterSpec for HSEM_C2MISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [hsem_c2misr::R](R) reader structure
impl crate::Readable for HSEM_C2MISR_SPEC {
    type Reader = R;
}
///`reset()` method sets HSEM_C2MISR to value 0
impl crate::Resettable for HSEM_C2MISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
