///Register `GICD_PIDR6` reader
pub struct R(crate::R<GICD_PIDR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_PIDR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_PIDR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_PIDR6_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PIDR6` reader - PIDR6
pub type PIDR6_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - PIDR6
    #[inline(always)]
    pub fn pidr6(&self) -> PIDR6_R {
        PIDR6_R::new(self.bits)
    }
}
///GICD peripheral ID5 to ID7 register 6
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gicd_pidr6](index.html) module
pub struct GICD_PIDR6_SPEC;
impl crate::RegisterSpec for GICD_PIDR6_SPEC {
    type Ux = u32;
}
///`read()` method returns [gicd_pidr6::R](R) reader structure
impl crate::Readable for GICD_PIDR6_SPEC {
    type Reader = R;
}
///`reset()` method sets GICD_PIDR6 to value 0
impl crate::Resettable for GICD_PIDR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
