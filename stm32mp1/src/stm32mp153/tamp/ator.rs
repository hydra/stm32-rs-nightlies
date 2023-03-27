///Register `ATOR` reader
pub struct R(crate::R<ATOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATOR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PRNG` reader - PRNG
pub type PRNG_R = crate::FieldReader<u8, u8>;
///Field `SEEDF` reader - SEEDF
pub type SEEDF_R = crate::BitReader<bool>;
///Field `INITS` reader - INITS
pub type INITS_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:7 - PRNG
    #[inline(always)]
    pub fn prng(&self) -> PRNG_R {
        PRNG_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 14 - SEEDF
    #[inline(always)]
    pub fn seedf(&self) -> SEEDF_R {
        SEEDF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - INITS
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
///This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ator](index.html) module
pub struct ATOR_SPEC;
impl crate::RegisterSpec for ATOR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ator::R](R) reader structure
impl crate::Readable for ATOR_SPEC {
    type Reader = R;
}
///`reset()` method sets ATOR to value 0
impl crate::Resettable for ATOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
