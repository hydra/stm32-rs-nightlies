///Register `TZC_BUILD_CONFIG` reader
pub struct R(crate::R<TZC_BUILD_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_BUILD_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_BUILD_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_BUILD_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
///Field `NO_OF_REGIONS` reader - NO_OF_REGIONS
pub type NO_OF_REGIONS_R = crate::FieldReader<u8, u8>;
///Field `ADDRESS_WIDTH` reader - ADDRESS_WIDTH
pub type ADDRESS_WIDTH_R = crate::FieldReader<u8, u8>;
///Field `NO_OF_FILTERS` reader - NO_OF_FILTERS
pub type NO_OF_FILTERS_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:4 - NO_OF_REGIONS
    #[inline(always)]
    pub fn no_of_regions(&self) -> NO_OF_REGIONS_R {
        NO_OF_REGIONS_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:13 - ADDRESS_WIDTH
    #[inline(always)]
    pub fn address_width(&self) -> ADDRESS_WIDTH_R {
        ADDRESS_WIDTH_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 24:25 - NO_OF_FILTERS
    #[inline(always)]
    pub fn no_of_filters(&self) -> NO_OF_FILTERS_R {
        NO_OF_FILTERS_R::new(((self.bits >> 24) & 3) as u8)
    }
}
///Provides information about TZC configuration.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzc_build_config](index.html) module
pub struct TZC_BUILD_CONFIG_SPEC;
impl crate::RegisterSpec for TZC_BUILD_CONFIG_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzc_build_config::R](R) reader structure
impl crate::Readable for TZC_BUILD_CONFIG_SPEC {
    type Reader = R;
}
///`reset()` method sets TZC_BUILD_CONFIG to value 0x0100_1f08
impl crate::Resettable for TZC_BUILD_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_1f08;
}
