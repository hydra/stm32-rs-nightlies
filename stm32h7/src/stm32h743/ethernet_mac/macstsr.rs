///Register `MACSTSR` reader
pub struct R(crate::R<MACSTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACSTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACSTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACSTSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TSS` reader - Timestamp Second
pub type TSS_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Timestamp Second
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new(self.bits)
    }
}
///System time seconds register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macstsr](index.html) module
pub struct MACSTSR_SPEC;
impl crate::RegisterSpec for MACSTSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macstsr::R](R) reader structure
impl crate::Readable for MACSTSR_SPEC {
    type Reader = R;
}
///`reset()` method sets MACSTSR to value 0
impl crate::Resettable for MACSTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
