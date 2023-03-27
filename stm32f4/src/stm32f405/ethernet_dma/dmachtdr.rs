///Register `DMACHTDR` reader
pub struct R(crate::R<DMACHTDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACHTDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACHTDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACHTDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `HTDAP` reader - HTDAP
pub type HTDAP_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - HTDAP
    #[inline(always)]
    pub fn htdap(&self) -> HTDAP_R {
        HTDAP_R::new(self.bits)
    }
}
///Ethernet DMA current host transmit descriptor register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmachtdr](index.html) module
pub struct DMACHTDR_SPEC;
impl crate::RegisterSpec for DMACHTDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmachtdr::R](R) reader structure
impl crate::Readable for DMACHTDR_SPEC {
    type Reader = R;
}
///`reset()` method sets DMACHTDR to value 0
impl crate::Resettable for DMACHTDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
