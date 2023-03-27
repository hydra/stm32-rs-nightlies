///Register `DMACCATXDR` reader
pub struct R(crate::R<DMACCATXDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACCATXDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACCATXDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACCATXDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CURTDESAPTR` reader - Application Transmit Descriptor Address Pointer The DMA updates this pointer during Tx operation. This pointer is cleared on reset.
pub type CURTDESAPTR_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Application Transmit Descriptor Address Pointer The DMA updates this pointer during Tx operation. This pointer is cleared on reset.
    #[inline(always)]
    pub fn curtdesaptr(&self) -> CURTDESAPTR_R {
        CURTDESAPTR_R::new(self.bits)
    }
}
///Channel current application transmit descriptor register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmaccatxdr](index.html) module
pub struct DMACCATXDR_SPEC;
impl crate::RegisterSpec for DMACCATXDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmaccatxdr::R](R) reader structure
impl crate::Readable for DMACCATXDR_SPEC {
    type Reader = R;
}
///`reset()` method sets DMACCATXDR to value 0
impl crate::Resettable for DMACCATXDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
