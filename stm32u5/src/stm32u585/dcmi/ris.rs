///Register `RIS` reader
pub struct R(crate::R<RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RIS_SPEC>) -> Self {
        R(reader)
    }
}
///Field `FRAME_RIS` reader - Capture complete raw interrupt status
pub type FRAME_RIS_R = crate::BitReader<bool>;
///Field `OVR_RIS` reader - Overrun raw interrupt status
pub type OVR_RIS_R = crate::BitReader<bool>;
///Field `ERR_RIS` reader - Synchronization error raw interrupt status
pub type ERR_RIS_R = crate::BitReader<bool>;
///Field `VSYNC_RIS` reader - DCMI_VSYNC raw interrupt status
pub type VSYNC_RIS_R = crate::BitReader<bool>;
///Field `LINE_RIS` reader - Line raw interrupt status
pub type LINE_RIS_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Capture complete raw interrupt status
    #[inline(always)]
    pub fn frame_ris(&self) -> FRAME_RIS_R {
        FRAME_RIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Overrun raw interrupt status
    #[inline(always)]
    pub fn ovr_ris(&self) -> OVR_RIS_R {
        OVR_RIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Synchronization error raw interrupt status
    #[inline(always)]
    pub fn err_ris(&self) -> ERR_RIS_R {
        ERR_RIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DCMI_VSYNC raw interrupt status
    #[inline(always)]
    pub fn vsync_ris(&self) -> VSYNC_RIS_R {
        VSYNC_RIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Line raw interrupt status
    #[inline(always)]
    pub fn line_ris(&self) -> LINE_RIS_R {
        LINE_RIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
///raw interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ris](index.html) module
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
///`read()` method returns [ris::R](R) reader structure
impl crate::Readable for RIS_SPEC {
    type Reader = R;
}
///`reset()` method sets RIS to value 0
impl crate::Resettable for RIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
