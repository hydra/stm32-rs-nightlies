///Register `ITLINE31` reader
pub struct R(crate::R<ITLINE31_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE31_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE31_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE31_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RNG` reader - RNG
pub type RNG_R = crate::BitReader<bool>;
///Field `AES` reader - AES
pub type AES_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - RNG
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - AES
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new(((self.bits >> 1) & 1) != 0)
    }
}
///interrupt line 31 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itline31](index.html) module
pub struct ITLINE31_SPEC;
impl crate::RegisterSpec for ITLINE31_SPEC {
    type Ux = u32;
}
///`read()` method returns [itline31::R](R) reader structure
impl crate::Readable for ITLINE31_SPEC {
    type Reader = R;
}
///`reset()` method sets ITLINE31 to value 0
impl crate::Resettable for ITLINE31_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
