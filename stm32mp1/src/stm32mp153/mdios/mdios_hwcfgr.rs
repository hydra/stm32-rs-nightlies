///Register `MDIOS_HWCFGR` reader
pub struct R(crate::R<MDIOS_HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOS_HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOS_HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOS_HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `NBREG` reader - NBREG
pub type NBREG_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - NBREG
    #[inline(always)]
    pub fn nbreg(&self) -> NBREG_R {
        NBREG_R::new((self.bits & 0xff) as u8)
    }
}
///MDIOS HW configuration register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdios_hwcfgr](index.html) module
pub struct MDIOS_HWCFGR_SPEC;
impl crate::RegisterSpec for MDIOS_HWCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdios_hwcfgr::R](R) reader structure
impl crate::Readable for MDIOS_HWCFGR_SPEC {
    type Reader = R;
}
///`reset()` method sets MDIOS_HWCFGR to value 0x20
impl crate::Resettable for MDIOS_HWCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
