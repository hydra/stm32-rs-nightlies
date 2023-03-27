///Register `M3DEAR` reader
pub struct R(crate::R<M3DEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M3DEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M3DEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M3DEAR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `EDEA` reader - ECC double error address When the ALE bit is set in the RAMCFG_MxCR register, this field is updated with the address corresponding to the ECC double error.
pub type EDEA_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - ECC double error address When the ALE bit is set in the RAMCFG_MxCR register, this field is updated with the address corresponding to the ECC double error.
    #[inline(always)]
    pub fn edea(&self) -> EDEA_R {
        EDEA_R::new(self.bits)
    }
}
///RAMCFG memory 3 ECC double error address register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [m3dear](index.html) module
pub struct M3DEAR_SPEC;
impl crate::RegisterSpec for M3DEAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [m3dear::R](R) reader structure
impl crate::Readable for M3DEAR_SPEC {
    type Reader = R;
}
///`reset()` method sets M3DEAR to value 0
impl crate::Resettable for M3DEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
