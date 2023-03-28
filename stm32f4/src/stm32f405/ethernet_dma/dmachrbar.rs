///Register `DMACHRBAR` reader
pub struct R(crate::R<DMACHRBAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACHRBAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACHRBAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACHRBAR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `HRBAP` reader - HRBAP
pub type HRBAP_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - HRBAP
    #[inline(always)]
    pub fn hrbap(&self) -> HRBAP_R {
        HRBAP_R::new(self.bits)
    }
}
///Ethernet DMA current host receive buffer address register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmachrbar](index.html) module
pub struct DMACHRBAR_SPEC;
impl crate::RegisterSpec for DMACHRBAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmachrbar::R](R) reader structure
impl crate::Readable for DMACHRBAR_SPEC {
    type Reader = R;
}
///`reset()` method sets DMACHRBAR to value 0
impl crate::Resettable for DMACHRBAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
