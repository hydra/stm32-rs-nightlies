///Register `DFSDM_HWCFGR` reader
pub struct R(crate::R<DFSDM_HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `NBT` reader - NBT
pub type NBT_R = crate::FieldReader<u8, u8>;
///Field `NBF` reader - NBF
pub type NBF_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - NBT
    #[inline(always)]
    pub fn nbt(&self) -> NBT_R {
        NBT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - NBF
    #[inline(always)]
    pub fn nbf(&self) -> NBF_R {
        NBF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
///This register specifies the hardware configuration of DFSDM peripheral.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_hwcfgr](index.html) module
pub struct DFSDM_HWCFGR_SPEC;
impl crate::RegisterSpec for DFSDM_HWCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dfsdm_hwcfgr::R](R) reader structure
impl crate::Readable for DFSDM_HWCFGR_SPEC {
    type Reader = R;
}
///`reset()` method sets DFSDM_HWCFGR to value 0x0608
impl crate::Resettable for DFSDM_HWCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0608;
}
