///Register `GPIOG_IPIDR` reader
pub struct R(crate::R<GPIOG_IPIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOG_IPIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOG_IPIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOG_IPIDR_SPEC>) -> Self {
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
///For information about available fields see [gpiog_ipidr](index.html) module
pub struct GPIOG_IPIDR_SPEC;
impl crate::RegisterSpec for GPIOG_IPIDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpiog_ipidr::R](R) reader structure
impl crate::Readable for GPIOG_IPIDR_SPEC {
    type Reader = R;
}
///`reset()` method sets GPIOG_IPIDR to value 0x000f_0002
impl crate::Resettable for GPIOG_IPIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x000f_0002;
}
