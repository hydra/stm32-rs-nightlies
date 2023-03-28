///Register `GPIOC_IPIDR` reader
pub struct R(crate::R<GPIOC_IPIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOC_IPIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOC_IPIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOC_IPIDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `IPIDR` reader - IPIDR
pub type IPIDR_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - IPIDR
    #[inline(always)]
    pub fn ipidr(&self) -> IPIDR_R {
        IPIDR_R::new(self.bits)
    }
}
///GPIO identification register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gpioc_ipidr](index.html) module
pub struct GPIOC_IPIDR_SPEC;
impl crate::RegisterSpec for GPIOC_IPIDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpioc_ipidr::R](R) reader structure
impl crate::Readable for GPIOC_IPIDR_SPEC {
    type Reader = R;
}
///`reset()` method sets GPIOC_IPIDR to value 0x000f_0002
impl crate::Resettable for GPIOC_IPIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x000f_0002;
}
