///Register `ADF_SADSDLVR` reader
pub struct R(crate::R<ADF_SADSDLVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADF_SADSDLVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADF_SADSDLVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADF_SADSDLVR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SDLVL` reader - SDLVL
pub type SDLVL_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:14 - SDLVL
    #[inline(always)]
    pub fn sdlvl(&self) -> SDLVL_R {
        SDLVL_R::new((self.bits & 0x7fff) as u16)
    }
}
///ADF SAD sound level register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adf_sadsdlvr](index.html) module
pub struct ADF_SADSDLVR_SPEC;
impl crate::RegisterSpec for ADF_SADSDLVR_SPEC {
    type Ux = u32;
}
///`read()` method returns [adf_sadsdlvr::R](R) reader structure
impl crate::Readable for ADF_SADSDLVR_SPEC {
    type Reader = R;
}
///`reset()` method sets ADF_SADSDLVR to value 0
impl crate::Resettable for ADF_SADSDLVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
