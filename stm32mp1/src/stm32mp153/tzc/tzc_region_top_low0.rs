///Register `TZC_REGION_TOP_LOW0` reader
pub struct R(crate::R<TZC_REGION_TOP_LOW0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_REGION_TOP_LOW0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_REGION_TOP_LOW0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_REGION_TOP_LOW0_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TOP_ADDRESS_LOW` reader - TOP_ADDRESS_LOW
pub type TOP_ADDRESS_LOW_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 12:31 - TOP_ADDRESS_LOW
    #[inline(always)]
    pub fn top_address_low(&self) -> TOP_ADDRESS_LOW_R {
        TOP_ADDRESS_LOW_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
///Top address bits \[31:12\]
///for region 0.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzc_region_top_low0](index.html) module
pub struct TZC_REGION_TOP_LOW0_SPEC;
impl crate::RegisterSpec for TZC_REGION_TOP_LOW0_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzc_region_top_low0::R](R) reader structure
impl crate::Readable for TZC_REGION_TOP_LOW0_SPEC {
    type Reader = R;
}
///`reset()` method sets TZC_REGION_TOP_LOW0 to value 0xffff_ffff
impl crate::Resettable for TZC_REGION_TOP_LOW0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
