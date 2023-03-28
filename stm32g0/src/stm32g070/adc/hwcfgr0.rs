///Register `HWCFGR0` reader
pub struct R(crate::R<HWCFGR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR0_SPEC>) -> Self {
        R(reader)
    }
}
///Field `NUM_CHAN_24` reader - NUM_CHAN_24
pub type NUM_CHAN_24_R = crate::FieldReader<u8, u8>;
///Field `EXTRA_AWDS` reader - Extra analog watchdog
pub type EXTRA_AWDS_R = crate::FieldReader<u8, u8>;
///Field `OVS` reader - Oversampling
pub type OVS_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - NUM_CHAN_24
    #[inline(always)]
    pub fn num_chan_24(&self) -> NUM_CHAN_24_R {
        NUM_CHAN_24_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Extra analog watchdog
    #[inline(always)]
    pub fn extra_awds(&self) -> EXTRA_AWDS_R {
        EXTRA_AWDS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Oversampling
    #[inline(always)]
    pub fn ovs(&self) -> OVS_R {
        OVS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
///Hardware Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hwcfgr0](index.html) module
pub struct HWCFGR0_SPEC;
impl crate::RegisterSpec for HWCFGR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [hwcfgr0::R](R) reader structure
impl crate::Readable for HWCFGR0_SPEC {
    type Reader = R;
}
///`reset()` method sets HWCFGR0 to value 0x0110
impl crate::Resettable for HWCFGR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0110;
}
