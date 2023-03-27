///Register `DBGMCU_SR` reader
pub struct R(crate::R<DBGMCU_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBGMCU_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBGMCU_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBGMCU_SR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `AP_PRESENT` reader - Bit n identifies whether access port AP n is present in device Bit n = 0: APn absent Bit n = 1: APn present
pub type AP_PRESENT_R = crate::FieldReader<u8, u8>;
///Field `AP_LOCKED` reader - DECLARATION TO BE CONFIRMED by PRODUCT OWNER! Bit n identifies whether access port AP n is open (can be accessed via the debug port) or locked (debug access to the AP is blocked) Bit n = 0: APn locked Bit n = 1: APn enabled
pub type AP_LOCKED_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - Bit n identifies whether access port AP n is present in device Bit n = 0: APn absent Bit n = 1: APn present
    #[inline(always)]
    pub fn ap_present(&self) -> AP_PRESENT_R {
        AP_PRESENT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - DECLARATION TO BE CONFIRMED by PRODUCT OWNER! Bit n identifies whether access port AP n is open (can be accessed via the debug port) or locked (debug access to the AP is blocked) Bit n = 0: APn locked Bit n = 1: APn enabled
    #[inline(always)]
    pub fn ap_locked(&self) -> AP_LOCKED_R {
        AP_LOCKED_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
///DBGMCU status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dbgmcu_sr](index.html) module
pub struct DBGMCU_SR_SPEC;
impl crate::RegisterSpec for DBGMCU_SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dbgmcu_sr::R](R) reader structure
impl crate::Readable for DBGMCU_SR_SPEC {
    type Reader = R;
}
///`reset()` method sets DBGMCU_SR to value 0x01
impl crate::Resettable for DBGMCU_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
