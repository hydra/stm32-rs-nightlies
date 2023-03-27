///Register `DBGMCU_DBG_AUTH_HOST` reader
pub struct R(crate::R<DBGMCU_DBG_AUTH_HOST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBGMCU_DBG_AUTH_HOST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBGMCU_DBG_AUTH_HOST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBGMCU_DBG_AUTH_HOST_SPEC>) -> Self {
        R(reader)
    }
}
///Field `AUTH_KEY` reader - Device authentication key The device specific 64-bit authentication key (OEM key) must be written to this register (in two successive 32-bit writes, least significant word first) to permit RDP regression. Writing a wrong key locks access to the device and prevent code execution from the Flash memory.
pub type AUTH_KEY_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Device authentication key The device specific 64-bit authentication key (OEM key) must be written to this register (in two successive 32-bit writes, least significant word first) to permit RDP regression. Writing a wrong key locks access to the device and prevent code execution from the Flash memory.
    #[inline(always)]
    pub fn auth_key(&self) -> AUTH_KEY_R {
        AUTH_KEY_R::new(self.bits)
    }
}
///DBGMCU debug host authentication register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dbgmcu_dbg_auth_host](index.html) module
pub struct DBGMCU_DBG_AUTH_HOST_SPEC;
impl crate::RegisterSpec for DBGMCU_DBG_AUTH_HOST_SPEC {
    type Ux = u32;
}
///`read()` method returns [dbgmcu_dbg_auth_host::R](R) reader structure
impl crate::Readable for DBGMCU_DBG_AUTH_HOST_SPEC {
    type Reader = R;
}
///`reset()` method sets DBGMCU_DBG_AUTH_HOST to value 0
impl crate::Resettable for DBGMCU_DBG_AUTH_HOST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
