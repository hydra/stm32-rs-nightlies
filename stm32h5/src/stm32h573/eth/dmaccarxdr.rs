///Register `DMACCARXDR` reader
pub struct R(crate::R<DMACCARXDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACCARXDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACCARXDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACCARXDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CURRDESAPTR` reader - Application Receive Descriptor Address Pointer The DMA updates this pointer during Rx operation. This pointer is cleared on reset.
pub type CURRDESAPTR_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Application Receive Descriptor Address Pointer The DMA updates this pointer during Rx operation. This pointer is cleared on reset.
    #[inline(always)]
    pub fn currdesaptr(&self) -> CURRDESAPTR_R {
        CURRDESAPTR_R::new(self.bits)
    }
}
///Channel current application receive descriptor register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmaccarxdr](index.html) module
pub struct DMACCARXDR_SPEC;
impl crate::RegisterSpec for DMACCARXDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmaccarxdr::R](R) reader structure
impl crate::Readable for DMACCARXDR_SPEC {
    type Reader = R;
}
///`reset()` method sets DMACCARXDR to value 0
impl crate::Resettable for DMACCARXDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
