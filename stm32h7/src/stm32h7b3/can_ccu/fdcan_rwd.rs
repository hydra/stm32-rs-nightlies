///Register `FDCAN_RWD` reader
pub struct R(crate::R<FDCAN_RWD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_RWD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_RWD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_RWD_SPEC>) -> Self {
        R(reader)
    }
}
///Field `WDC` reader - Watchdog configuration
pub type WDC_R = crate::FieldReader<u8, u8>;
///Field `WDV` reader - Watchdog value
pub type WDV_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - Watchdog configuration
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Watchdog value
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
///FDCAN RAM Watchdog Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_rwd](index.html) module
pub struct FDCAN_RWD_SPEC;
impl crate::RegisterSpec for FDCAN_RWD_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_rwd::R](R) reader structure
impl crate::Readable for FDCAN_RWD_SPEC {
    type Reader = R;
}
///`reset()` method sets FDCAN_RWD to value 0
impl crate::Resettable for FDCAN_RWD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
