///Register `GICD_PIDR5` reader
pub struct R(crate::R<GICD_PIDR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_PIDR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_PIDR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_PIDR5_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PIDR5` reader - PIDR5
pub type PIDR5_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - PIDR5
    #[inline(always)]
    pub fn pidr5(&self) -> PIDR5_R {
        PIDR5_R::new(self.bits)
    }
}
///GICD peripheral ID5 to ID7 register 5
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gicd_pidr5](index.html) module
pub struct GICD_PIDR5_SPEC;
impl crate::RegisterSpec for GICD_PIDR5_SPEC {
    type Ux = u32;
}
///`read()` method returns [gicd_pidr5::R](R) reader structure
impl crate::Readable for GICD_PIDR5_SPEC {
    type Reader = R;
}
///`reset()` method sets GICD_PIDR5 to value 0
impl crate::Resettable for GICD_PIDR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
