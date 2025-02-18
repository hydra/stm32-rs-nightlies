///Register `M3SR` reader
pub struct R(crate::R<M3SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M3SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M3SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M3SR_SPEC>) -> Self {
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
///RAMECC monitor x status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [m3sr](index.html) module
pub struct M3SR_SPEC;
impl crate::RegisterSpec for M3SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [m3sr::R](R) reader structure
impl crate::Readable for M3SR_SPEC {
    type Reader = R;
}
///`reset()` method sets M3SR to value 0
impl crate::Resettable for M3SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
