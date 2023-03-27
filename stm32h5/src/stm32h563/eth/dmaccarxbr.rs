///Register `DMACCARXBR` reader
pub struct R(crate::R<DMACCARXBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACCARXBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACCARXBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACCARXBR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CURRBUFAPTR` reader - Application Receive Buffer Address Pointer The DMA updates this pointer during Rx operation. This pointer is cleared on reset.
pub type CURRBUFAPTR_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Application Receive Buffer Address Pointer The DMA updates this pointer during Rx operation. This pointer is cleared on reset.
    #[inline(always)]
    pub fn currbufaptr(&self) -> CURRBUFAPTR_R {
        CURRBUFAPTR_R::new(self.bits)
    }
}
///Channel current application receive buffer register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmaccarxbr](index.html) module
pub struct DMACCARXBR_SPEC;
impl crate::RegisterSpec for DMACCARXBR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmaccarxbr::R](R) reader structure
impl crate::Readable for DMACCARXBR_SPEC {
    type Reader = R;
}
///`reset()` method sets DMACCARXBR to value 0
impl crate::Resettable for DMACCARXBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
