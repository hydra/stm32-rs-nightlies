///Register `FLT1CNVTIMR` reader
pub struct R(crate::R<FLT1CNVTIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLT1CNVTIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLT1CNVTIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLT1CNVTIMR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CNVCNT` reader - 28-bit timer counting conversion time t = CNVCNT\[27:0\]
pub type CNVCNT_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 4:31 - 28-bit timer counting conversion time t = CNVCNT\[27:0\]
    #[inline(always)]
    pub fn cnvcnt(&self) -> CNVCNT_R {
        CNVCNT_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
///conversion timer register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flt1cnvtimr](index.html) module
pub struct FLT1CNVTIMR_SPEC;
impl crate::RegisterSpec for FLT1CNVTIMR_SPEC {
    type Ux = u32;
}
///`read()` method returns [flt1cnvtimr::R](R) reader structure
impl crate::Readable for FLT1CNVTIMR_SPEC {
    type Reader = R;
}
///`reset()` method sets FLT1CNVTIMR to value 0
impl crate::Resettable for FLT1CNVTIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
