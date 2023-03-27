///Register `RAM2SEAR` reader
pub struct R(crate::R<RAM2SEAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM2SEAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM2SEAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM2SEAR_SPEC>) -> Self {
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
///For information about available fields see [ram2sear](index.html) module
pub struct RAM2SEAR_SPEC;
impl crate::RegisterSpec for RAM2SEAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ram2sear::R](R) reader structure
impl crate::Readable for RAM2SEAR_SPEC {
    type Reader = R;
}
///`reset()` method sets RAM2SEAR to value 0
impl crate::Resettable for RAM2SEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
