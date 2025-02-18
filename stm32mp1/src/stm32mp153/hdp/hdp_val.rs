///Register `HDP_VAL` reader
pub struct R(crate::R<HDP_VAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HDP_VAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HDP_VAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HDP_VAL_SPEC>) -> Self {
        R(reader)
    }
}
///Field `HDPVAL` reader - HDPVAL
pub type HDPVAL_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - HDPVAL
    #[inline(always)]
    pub fn hdpval(&self) -> HDPVAL_R {
        HDPVAL_R::new((self.bits & 0xff) as u8)
    }
}
///HDP value
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hdp_val](index.html) module
pub struct HDP_VAL_SPEC;
impl crate::RegisterSpec for HDP_VAL_SPEC {
    type Ux = u32;
}
///`read()` method returns [hdp_val::R](R) reader structure
impl crate::Readable for HDP_VAL_SPEC {
    type Reader = R;
}
///`reset()` method sets HDP_VAL to value 0
impl crate::Resettable for HDP_VAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
