///Register `GICD_PIDR2` reader
pub struct R(crate::R<GICD_PIDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_PIDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_PIDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_PIDR2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PIDR2` reader - PIDR2
pub type PIDR2_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - PIDR2
    #[inline(always)]
    pub fn pidr2(&self) -> PIDR2_R {
        PIDR2_R::new(self.bits)
    }
}
///GICD peripheral ID2 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gicd_pidr2](index.html) module
pub struct GICD_PIDR2_SPEC;
impl crate::RegisterSpec for GICD_PIDR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [gicd_pidr2::R](R) reader structure
impl crate::Readable for GICD_PIDR2_SPEC {
    type Reader = R;
}
///`reset()` method sets GICD_PIDR2 to value 0x2b
impl crate::Resettable for GICD_PIDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x2b;
}
