///Register `GPIOA_IPIDR` reader
pub struct R(crate::R<GPIOA_IPIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOA_IPIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOA_IPIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOA_IPIDR_SPEC>) -> Self {
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
///For information about available fields see [gpioa_ipidr](index.html) module
pub struct GPIOA_IPIDR_SPEC;
impl crate::RegisterSpec for GPIOA_IPIDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpioa_ipidr::R](R) reader structure
impl crate::Readable for GPIOA_IPIDR_SPEC {
    type Reader = R;
}
///`reset()` method sets GPIOA_IPIDR to value 0x000f_0002
impl crate::Resettable for GPIOA_IPIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x000f_0002;
}
