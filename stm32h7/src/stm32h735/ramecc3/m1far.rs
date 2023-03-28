///Register `M1FAR` reader
pub struct R(crate::R<M1FAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M1FAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M1FAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M1FAR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `FADD` reader - ECC error failing address
pub type FADD_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - ECC error failing address
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new(self.bits)
    }
}
///RAMECC monitor x failing address register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [m1far](index.html) module
pub struct M1FAR_SPEC;
impl crate::RegisterSpec for M1FAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [m1far::R](R) reader structure
impl crate::Readable for M1FAR_SPEC {
    type Reader = R;
}
///`reset()` method sets M1FAR to value 0
impl crate::Resettable for M1FAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
