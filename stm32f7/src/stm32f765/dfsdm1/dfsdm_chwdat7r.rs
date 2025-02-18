///Register `DFSDM_CHWDAT7R` reader
pub struct R(crate::R<DFSDM_CHWDAT7R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_CHWDAT7R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_CHWDAT7R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_CHWDAT7R_SPEC>) -> Self {
        R(reader)
    }
}
///Field `WDATA` reader - Input channel y watchdog data
pub type WDATA_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Input channel y watchdog data
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
///DFSDM channel watchdog filter data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_chwdat7r](index.html) module
pub struct DFSDM_CHWDAT7R_SPEC;
impl crate::RegisterSpec for DFSDM_CHWDAT7R_SPEC {
    type Ux = u32;
}
///`read()` method returns [dfsdm_chwdat7r::R](R) reader structure
impl crate::Readable for DFSDM_CHWDAT7R_SPEC {
    type Reader = R;
}
///`reset()` method sets DFSDM_CHWDAT7R to value 0
impl crate::Resettable for DFSDM_CHWDAT7R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
