///Register `M3SEAR` reader
pub struct R(crate::R<M3SEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M3SEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M3SEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M3SEAR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ESEA` reader - ECC single error address When the ALE bit is set in the RAMCFG_MxCR register, this field is updated with the address corresponding to the ECC single error.
pub type ESEA_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - ECC single error address When the ALE bit is set in the RAMCFG_MxCR register, this field is updated with the address corresponding to the ECC single error.
    #[inline(always)]
    pub fn esea(&self) -> ESEA_R {
        ESEA_R::new(self.bits)
    }
}
///RAMCFG memory 3 ECC single error address register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [m3sear](index.html) module
pub struct M3SEAR_SPEC;
impl crate::RegisterSpec for M3SEAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [m3sear::R](R) reader structure
impl crate::Readable for M3SEAR_SPEC {
    type Reader = R;
}
///`reset()` method sets M3SEAR to value 0
impl crate::Resettable for M3SEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
