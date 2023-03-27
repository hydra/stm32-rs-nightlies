///Register `ETH_MACVR` reader
pub struct R(crate::R<ETH_MACVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACVR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SNPSVER` reader - SNPSVER
pub type SNPSVER_R = crate::FieldReader<u8, u8>;
///Field `USERVER` reader - USERVER
pub type USERVER_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - SNPSVER
    #[inline(always)]
    pub fn snpsver(&self) -> SNPSVER_R {
        SNPSVER_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - USERVER
    #[inline(always)]
    pub fn userver(&self) -> USERVER_R {
        USERVER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
///The version register identifies the version of the Ethernet peripheral.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_macvr](index.html) module
pub struct ETH_MACVR_SPEC;
impl crate::RegisterSpec for ETH_MACVR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_macvr::R](R) reader structure
impl crate::Readable for ETH_MACVR_SPEC {
    type Reader = R;
}
///`reset()` method sets ETH_MACVR to value 0x4042
impl crate::Resettable for ETH_MACVR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4042;
}
