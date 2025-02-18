///Register `GICD_SPISR4` reader
pub struct R(crate::R<GICD_SPISR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_SPISR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_SPISR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_SPISR4_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SPISR4` reader - SPISR4
pub type SPISR4_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - SPISR4
    #[inline(always)]
    pub fn spisr4(&self) -> SPISR4_R {
        SPISR4_R::new(self.bits)
    }
}
///For interrupts ID
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gicd_spisr4](index.html) module
pub struct GICD_SPISR4_SPEC;
impl crate::RegisterSpec for GICD_SPISR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [gicd_spisr4::R](R) reader structure
impl crate::Readable for GICD_SPISR4_SPEC {
    type Reader = R;
}
///`reset()` method sets GICD_SPISR4 to value 0
impl crate::Resettable for GICD_SPISR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
