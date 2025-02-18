///Register `TZC_FAIL_ADDRESS_LOW0` reader
pub struct R(crate::R<TZC_FAIL_ADDRESS_LOW0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_FAIL_ADDRESS_LOW0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_FAIL_ADDRESS_LOW0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_FAIL_ADDRESS_LOW0_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ADDR_STATUS_LOW` reader - ADDR_STATUS_LOW
pub type ADDR_STATUS_LOW_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - ADDR_STATUS_LOW
    #[inline(always)]
    pub fn addr_status_low(&self) -> ADDR_STATUS_LOW_R {
        ADDR_STATUS_LOW_R::new(self.bits)
    }
}
///Address low bits of the first failed access in the associated filter (0 to 1).
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tzc_fail_address_low0](index.html) module
pub struct TZC_FAIL_ADDRESS_LOW0_SPEC;
impl crate::RegisterSpec for TZC_FAIL_ADDRESS_LOW0_SPEC {
    type Ux = u32;
}
///`read()` method returns [tzc_fail_address_low0::R](R) reader structure
impl crate::Readable for TZC_FAIL_ADDRESS_LOW0_SPEC {
    type Reader = R;
}
///`reset()` method sets TZC_FAIL_ADDRESS_LOW0 to value 0
impl crate::Resettable for TZC_FAIL_ADDRESS_LOW0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
