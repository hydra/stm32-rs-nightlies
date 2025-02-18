///Register `DR` reader
pub struct R(crate::R<DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RNDATA` reader - Random data 32-bit random data which are valid when DRDY=1.
pub type RNDATA_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Random data 32-bit random data which are valid when DRDY=1.
    #[inline(always)]
    pub fn rndata(&self) -> RNDATA_R {
        RNDATA_R::new(self.bits)
    }
}
///The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dr](index.html) module
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dr::R](R) reader structure
impl crate::Readable for DR_SPEC {
    type Reader = R;
}
///`reset()` method sets DR to value 0
impl crate::Resettable for DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
