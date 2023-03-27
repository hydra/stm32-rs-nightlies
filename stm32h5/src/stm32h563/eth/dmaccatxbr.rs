///Register `DMACCATXBR` reader
pub struct R(crate::R<DMACCATXBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACCATXBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACCATXBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACCATXBR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CURTBUFAPTR` reader - Application Transmit Buffer Address Pointer The DMA updates this pointer during Tx operation. This pointer is cleared on reset.
pub type CURTBUFAPTR_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Application Transmit Buffer Address Pointer The DMA updates this pointer during Tx operation. This pointer is cleared on reset.
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CURTBUFAPTR_R {
        CURTBUFAPTR_R::new(self.bits)
    }
}
///Channel current application transmit buffer register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmaccatxbr](index.html) module
pub struct DMACCATXBR_SPEC;
impl crate::RegisterSpec for DMACCATXBR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmaccatxbr::R](R) reader structure
impl crate::Readable for DMACCATXBR_SPEC {
    type Reader = R;
}
///`reset()` method sets DMACCATXBR to value 0
impl crate::Resettable for DMACCATXBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
