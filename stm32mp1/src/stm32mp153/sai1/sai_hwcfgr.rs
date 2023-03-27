///Register `SAI_HWCFGR` reader
pub struct R(crate::R<SAI_HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `FIFO_SIZE` reader - FIFO_SIZE
pub type FIFO_SIZE_R = crate::FieldReader<u8, u8>;
///Field `SPDIF_PDM` reader - SPDIF_PDM
pub type SPDIF_PDM_R = crate::FieldReader<u8, u8>;
///Field `OPTION_REGOUT` reader - OPTION_REGOUT
pub type OPTION_REGOUT_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - FIFO_SIZE
    #[inline(always)]
    pub fn fifo_size(&self) -> FIFO_SIZE_R {
        FIFO_SIZE_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:11 - SPDIF_PDM
    #[inline(always)]
    pub fn spdif_pdm(&self) -> SPDIF_PDM_R {
        SPDIF_PDM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:19 - OPTION_REGOUT
    #[inline(always)]
    pub fn option_regout(&self) -> OPTION_REGOUT_R {
        OPTION_REGOUT_R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
///SAI hardware configuration register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sai_hwcfgr](index.html) module
pub struct SAI_HWCFGR_SPEC;
impl crate::RegisterSpec for SAI_HWCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sai_hwcfgr::R](R) reader structure
impl crate::Readable for SAI_HWCFGR_SPEC {
    type Reader = R;
}
///`reset()` method sets SAI_HWCFGR to value 0x0108
impl crate::Resettable for SAI_HWCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0108;
}
