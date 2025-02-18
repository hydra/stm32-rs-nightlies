///Register `PECR` reader
pub struct R(crate::R<PECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PECR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PEC` reader - Packet error checking register This field contains the internal PEC when PECEN=1. The PEC is cleared by hardware when PE=0.
pub type PEC_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - Packet error checking register This field contains the internal PEC when PECEN=1. The PEC is cleared by hardware when PE=0.
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new((self.bits & 0xff) as u8)
    }
}
///I2C PEC register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pecr](index.html) module
pub struct PECR_SPEC;
impl crate::RegisterSpec for PECR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pecr::R](R) reader structure
impl crate::Readable for PECR_SPEC {
    type Reader = R;
}
///`reset()` method sets PECR to value 0
impl crate::Resettable for PECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
