///Register `GICV_IIDR` reader
pub struct R(crate::R<GICV_IIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICV_IIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICV_IIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICV_IIDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `IIDR` reader - IIDR
pub type IIDR_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - IIDR
    #[inline(always)]
    pub fn iidr(&self) -> IIDR_R {
        IIDR_R::new(self.bits)
    }
}
///The GICV_IIDR is an alias of GICC_IIDR.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gicv_iidr](index.html) module
pub struct GICV_IIDR_SPEC;
impl crate::RegisterSpec for GICV_IIDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gicv_iidr::R](R) reader structure
impl crate::Readable for GICV_IIDR_SPEC {
    type Reader = R;
}
///`reset()` method sets GICV_IIDR to value 0x0102_143b
impl crate::Resettable for GICV_IIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0102_143b;
}
