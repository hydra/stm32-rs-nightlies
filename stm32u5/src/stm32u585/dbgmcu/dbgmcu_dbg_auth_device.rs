///Register `DBGMCU_DBG_AUTH_DEVICE` reader
pub struct R(crate::R<DBGMCU_DBG_AUTH_DEVICE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBGMCU_DBG_AUTH_DEVICE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBGMCU_DBG_AUTH_DEVICE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBGMCU_DBG_AUTH_DEVICE_SPEC>) -> Self {
        R(reader)
    }
}
///Field `AUTH_ID` reader - Device specific ID Device specific ID used for RDP regression.
pub type AUTH_ID_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Device specific ID Device specific ID used for RDP regression.
    #[inline(always)]
    pub fn auth_id(&self) -> AUTH_ID_R {
        AUTH_ID_R::new(self.bits)
    }
}
///DBGMCU debug device authentication register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dbgmcu_dbg_auth_device](index.html) module
pub struct DBGMCU_DBG_AUTH_DEVICE_SPEC;
impl crate::RegisterSpec for DBGMCU_DBG_AUTH_DEVICE_SPEC {
    type Ux = u32;
}
///`read()` method returns [dbgmcu_dbg_auth_device::R](R) reader structure
impl crate::Readable for DBGMCU_DBG_AUTH_DEVICE_SPEC {
    type Reader = R;
}
///`reset()` method sets DBGMCU_DBG_AUTH_DEVICE to value 0
impl crate::Resettable for DBGMCU_DBG_AUTH_DEVICE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
