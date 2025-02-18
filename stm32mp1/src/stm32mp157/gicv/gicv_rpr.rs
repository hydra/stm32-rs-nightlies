///Register `GICV_RPR` reader
pub struct R(crate::R<GICV_RPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICV_RPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICV_RPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICV_RPR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PRIORITY` reader - PRIORITY
pub type PRIORITY_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 3:7 - PRIORITY
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
///GICV VM running priority register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gicv_rpr](index.html) module
pub struct GICV_RPR_SPEC;
impl crate::RegisterSpec for GICV_RPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gicv_rpr::R](R) reader structure
impl crate::Readable for GICV_RPR_SPEC {
    type Reader = R;
}
///`reset()` method sets GICV_RPR to value 0xff
impl crate::Resettable for GICV_RPR_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
