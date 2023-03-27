///Register `DCMI_MIS` reader
pub struct R(crate::R<DCMI_MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCMI_MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCMI_MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCMI_MIS_SPEC>) -> Self {
        R(reader)
    }
}
///Field `FRAME_MIS` reader - FRAME_MIS
pub type FRAME_MIS_R = crate::BitReader<bool>;
///Field `OVR_MIS` reader - OVR_MIS
pub type OVR_MIS_R = crate::BitReader<bool>;
///Field `ERR_MIS` reader - ERR_MIS
pub type ERR_MIS_R = crate::BitReader<bool>;
///Field `VSYNC_MIS` reader - VSYNC_MIS
pub type VSYNC_MIS_R = crate::BitReader<bool>;
///Field `LINE_MIS` reader - LINE_MIS
pub type LINE_MIS_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - FRAME_MIS
    #[inline(always)]
    pub fn frame_mis(&self) -> FRAME_MIS_R {
        FRAME_MIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - OVR_MIS
    #[inline(always)]
    pub fn ovr_mis(&self) -> OVR_MIS_R {
        OVR_MIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ERR_MIS
    #[inline(always)]
    pub fn err_mis(&self) -> ERR_MIS_R {
        ERR_MIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - VSYNC_MIS
    #[inline(always)]
    pub fn vsync_mis(&self) -> VSYNC_MIS_R {
        VSYNC_MIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - LINE_MIS
    #[inline(always)]
    pub fn line_mis(&self) -> LINE_MIS_R {
        LINE_MIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
///This DCMI_MIS register is a read-only register. When read, it returns the current masked status value (depending on the value in DCMI_IER) of the corresponding interrupt. A bit in this register is set if the corresponding enable bit in DCMI_IER is set and the corresponding bit in DCMI_RIS is set.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dcmi_mis](index.html) module
pub struct DCMI_MIS_SPEC;
impl crate::RegisterSpec for DCMI_MIS_SPEC {
    type Ux = u32;
}
///`read()` method returns [dcmi_mis::R](R) reader structure
impl crate::Readable for DCMI_MIS_SPEC {
    type Reader = R;
}
///`reset()` method sets DCMI_MIS to value 0
impl crate::Resettable for DCMI_MIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
