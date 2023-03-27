///Register `ADF_SADANLVR` reader
pub struct R(crate::R<ADF_SADANLVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADF_SADANLVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADF_SADANLVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADF_SADANLVR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ANLVL` reader - ANLVL
pub type ANLVL_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:14 - ANLVL
    #[inline(always)]
    pub fn anlvl(&self) -> ANLVL_R {
        ANLVL_R::new((self.bits & 0x7fff) as u16)
    }
}
///ADF SAD ambient noise level register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adf_sadanlvr](index.html) module
pub struct ADF_SADANLVR_SPEC;
impl crate::RegisterSpec for ADF_SADANLVR_SPEC {
    type Ux = u32;
}
///`read()` method returns [adf_sadanlvr::R](R) reader structure
impl crate::Readable for ADF_SADANLVR_SPEC {
    type Reader = R;
}
///`reset()` method sets ADF_SADANLVR to value 0
impl crate::Resettable for ADF_SADANLVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
