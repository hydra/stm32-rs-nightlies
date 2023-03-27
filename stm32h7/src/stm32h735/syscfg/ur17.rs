///Register `UR17` reader
pub struct R(crate::R<UR17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR17_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR17_SPEC>) -> Self {
        R(reader)
    }
}
///Field `IO_HSLV` reader - I/O high speed / low voltage
pub type IO_HSLV_R = crate::BitReader<bool>;
///Field `TCM_AXI_SHARED_CFG` reader - ITCM-RAM/AXI-SRAM size
pub type TCM_AXI_SHARED_CFG_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bit 0 - I/O high speed / low voltage
    #[inline(always)]
    pub fn io_hslv(&self) -> IO_HSLV_R {
        IO_HSLV_R::new((self.bits & 1) != 0)
    }
    ///Bits 16:17 - ITCM-RAM/AXI-SRAM size
    #[inline(always)]
    pub fn tcm_axi_shared_cfg(&self) -> TCM_AXI_SHARED_CFG_R {
        TCM_AXI_SHARED_CFG_R::new(((self.bits >> 16) & 3) as u8)
    }
}
///SYSCFG user register 17
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ur17](index.html) module
pub struct UR17_SPEC;
impl crate::RegisterSpec for UR17_SPEC {
    type Ux = u32;
}
///`read()` method returns [ur17::R](R) reader structure
impl crate::Readable for UR17_SPEC {
    type Reader = R;
}
///`reset()` method sets UR17 to value 0
impl crate::Resettable for UR17_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
