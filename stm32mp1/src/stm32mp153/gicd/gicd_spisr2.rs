///Register `GICD_SPISR2` reader
pub struct R(crate::R<GICD_SPISR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_SPISR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_SPISR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_SPISR2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SPISR2` reader - SPISR2
pub type SPISR2_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - SPISR2
    #[inline(always)]
    pub fn spisr2(&self) -> SPISR2_R {
        SPISR2_R::new(self.bits)
    }
}
///For interrupts ID
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gicd_spisr2](index.html) module
pub struct GICD_SPISR2_SPEC;
impl crate::RegisterSpec for GICD_SPISR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [gicd_spisr2::R](R) reader structure
impl crate::Readable for GICD_SPISR2_SPEC {
    type Reader = R;
}
///`reset()` method sets GICD_SPISR2 to value 0
impl crate::Resettable for GICD_SPISR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
