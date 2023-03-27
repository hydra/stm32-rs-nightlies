///Register `RAM3SEAR` reader
pub struct R(crate::R<RAM3SEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM3SEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM3SEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM3SEAR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ESEA` reader - ESEA
pub type ESEA_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - ESEA
    #[inline(always)]
    pub fn esea(&self) -> ESEA_R {
        ESEA_R::new(self.bits)
    }
}
///RAMCFG RAM x ECC single error address register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ram3sear](index.html) module
pub struct RAM3SEAR_SPEC;
impl crate::RegisterSpec for RAM3SEAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ram3sear::R](R) reader structure
impl crate::Readable for RAM3SEAR_SPEC {
    type Reader = R;
}
///`reset()` method sets RAM3SEAR to value 0
impl crate::Resettable for RAM3SEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
