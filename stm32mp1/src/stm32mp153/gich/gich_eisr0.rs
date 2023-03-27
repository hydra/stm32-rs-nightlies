///Register `GICH_EISR0` reader
pub struct R(crate::R<GICH_EISR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICH_EISR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICH_EISR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICH_EISR0_SPEC>) -> Self {
        R(reader)
    }
}
///Field `EISR0` reader - EISR0
pub type EISR0_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - EISR0
    #[inline(always)]
    pub fn eisr0(&self) -> EISR0_R {
        EISR0_R::new(self.bits)
    }
}
///GICH end of interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gich_eisr0](index.html) module
pub struct GICH_EISR0_SPEC;
impl crate::RegisterSpec for GICH_EISR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [gich_eisr0::R](R) reader structure
impl crate::Readable for GICH_EISR0_SPEC {
    type Reader = R;
}
///`reset()` method sets GICH_EISR0 to value 0
impl crate::Resettable for GICH_EISR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
