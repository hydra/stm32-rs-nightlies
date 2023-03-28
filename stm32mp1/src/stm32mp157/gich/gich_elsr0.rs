///Register `GICH_ELSR0` reader
pub struct R(crate::R<GICH_ELSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICH_ELSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICH_ELSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICH_ELSR0_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ELSR0` reader - ELSR0
pub type ELSR0_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - ELSR0
    #[inline(always)]
    pub fn elsr0(&self) -> ELSR0_R {
        ELSR0_R::new(self.bits)
    }
}
///GICH empty list status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gich_elsr0](index.html) module
pub struct GICH_ELSR0_SPEC;
impl crate::RegisterSpec for GICH_ELSR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [gich_elsr0::R](R) reader structure
impl crate::Readable for GICH_ELSR0_SPEC {
    type Reader = R;
}
///`reset()` method sets GICH_ELSR0 to value 0x0f
impl crate::Resettable for GICH_ELSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
