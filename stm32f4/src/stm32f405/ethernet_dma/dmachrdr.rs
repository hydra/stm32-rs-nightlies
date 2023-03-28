///Register `DMACHRDR` reader
pub struct R(crate::R<DMACHRDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACHRDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACHRDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACHRDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `HRDAP` reader - HRDAP
pub type HRDAP_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - HRDAP
    #[inline(always)]
    pub fn hrdap(&self) -> HRDAP_R {
        HRDAP_R::new(self.bits)
    }
}
///Ethernet DMA current host receive descriptor register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmachrdr](index.html) module
pub struct DMACHRDR_SPEC;
impl crate::RegisterSpec for DMACHRDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmachrdr::R](R) reader structure
impl crate::Readable for DMACHRDR_SPEC {
    type Reader = R;
}
///`reset()` method sets DMACHRDR to value 0
impl crate::Resettable for DMACHRDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
