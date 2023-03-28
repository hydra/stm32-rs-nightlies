///Register `HWCFGR2` reader
pub struct R(crate::R<HWCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PTIONREG_OUT` reader - PTIONREG_OUT
pub type PTIONREG_OUT_R = crate::FieldReader<u8, u8>;
///Field `TRUST_ZONE` reader - TRUST_ZONE
pub type TRUST_ZONE_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - PTIONREG_OUT
    #[inline(always)]
    pub fn ptionreg_out(&self) -> PTIONREG_OUT_R {
        PTIONREG_OUT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - TRUST_ZONE
    #[inline(always)]
    pub fn trust_zone(&self) -> TRUST_ZONE_R {
        TRUST_ZONE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
///TAMP hardware configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hwcfgr2](index.html) module
pub struct HWCFGR2_SPEC;
impl crate::RegisterSpec for HWCFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [hwcfgr2::R](R) reader structure
impl crate::Readable for HWCFGR2_SPEC {
    type Reader = R;
}
///`reset()` method sets HWCFGR2 to value 0
impl crate::Resettable for HWCFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
