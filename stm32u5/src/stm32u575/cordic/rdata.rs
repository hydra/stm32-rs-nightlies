///Register `RDATA` reader
pub struct R(crate::R<RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDATA_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RES` reader - Function result
pub type RES_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Function result
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(self.bits)
    }
}
///FMAC Read Data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rdata](index.html) module
pub struct RDATA_SPEC;
impl crate::RegisterSpec for RDATA_SPEC {
    type Ux = u32;
}
///`read()` method returns [rdata::R](R) reader structure
impl crate::Readable for RDATA_SPEC {
    type Reader = R;
}
///`reset()` method sets RDATA to value 0
impl crate::Resettable for RDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
