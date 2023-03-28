///Register `TAMP_ATOR` reader
pub struct R(crate::R<TAMP_ATOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMP_ATOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMP_ATOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMP_ATOR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PRNG` reader - Pseudo-random generator value This field provides the values of the PRNG output. Because of potential inconsistencies due to synchronization delays, PRNG must be read at least twice. The read value is correct if it is equal to previous read value. This field can only be read when the APB is in secure mode.
pub type PRNG_R = crate::FieldReader<u8, u8>;
///Field `SEEDF` reader - Seed running flag This flag is set by hardware when a new seed is written in the TAMP_ATSEEDR. It is cleared by hardware when the PRNG has absorbed this new seed, and by system reset. The TAMP APB cock must not be switched off as long as SEEDF is set.
pub type SEEDF_R = crate::BitReader<bool>;
///Field `INITS` reader - Active tamper initialization status This flag is set by hardware when the PRNG has absorbed the first 128-bit seed, meaning that the enabled active tampers are functional. This flag is cleared when the active tampers are disabled.
pub type INITS_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:7 - Pseudo-random generator value This field provides the values of the PRNG output. Because of potential inconsistencies due to synchronization delays, PRNG must be read at least twice. The read value is correct if it is equal to previous read value. This field can only be read when the APB is in secure mode.
    #[inline(always)]
    pub fn prng(&self) -> PRNG_R {
        PRNG_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 14 - Seed running flag This flag is set by hardware when a new seed is written in the TAMP_ATSEEDR. It is cleared by hardware when the PRNG has absorbed this new seed, and by system reset. The TAMP APB cock must not be switched off as long as SEEDF is set.
    #[inline(always)]
    pub fn seedf(&self) -> SEEDF_R {
        SEEDF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Active tamper initialization status This flag is set by hardware when the PRNG has absorbed the first 128-bit seed, meaning that the enabled active tampers are functional. This flag is cleared when the active tampers are disabled.
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
///TAMP active tamper output register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tamp_ator](index.html) module
pub struct TAMP_ATOR_SPEC;
impl crate::RegisterSpec for TAMP_ATOR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tamp_ator::R](R) reader structure
impl crate::Readable for TAMP_ATOR_SPEC {
    type Reader = R;
}
///`reset()` method sets TAMP_ATOR to value 0
impl crate::Resettable for TAMP_ATOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
