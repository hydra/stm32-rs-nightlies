///Register `DCMI_SR` reader
pub struct R(crate::R<DCMI_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCMI_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCMI_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCMI_SR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `HSYNC` reader - HSYNC
pub type HSYNC_R = crate::BitReader<bool>;
///Field `VSYNC` reader - VSYNC
pub type VSYNC_R = crate::BitReader<bool>;
///Field `FNE` reader - FNE
pub type FNE_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - HSYNC
    #[inline(always)]
    pub fn hsync(&self) -> HSYNC_R {
        HSYNC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - VSYNC
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FNE
    #[inline(always)]
    pub fn fne(&self) -> FNE_R {
        FNE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
///DCMI status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dcmi_sr](index.html) module
pub struct DCMI_SR_SPEC;
impl crate::RegisterSpec for DCMI_SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dcmi_sr::R](R) reader structure
impl crate::Readable for DCMI_SR_SPEC {
    type Reader = R;
}
///`reset()` method sets DCMI_SR to value 0
impl crate::Resettable for DCMI_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
