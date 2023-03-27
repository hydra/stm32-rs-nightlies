///Register `T0VALR1` reader
pub struct R(crate::R<T0VALR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0VALR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0VALR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0VALR1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TS1_FMT0` reader - Engineering value of the frequency measured at T0 for temperature sensor 1 This value is expressed in 0.1 kHz.
pub type TS1_FMT0_R = crate::FieldReader<u16, u16>;
///Field `TS1_T0` reader - Engineering value of the T0 temperature for temperature sensor 1. Others: Reserved, must not be used.
pub type TS1_T0_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:15 - Engineering value of the frequency measured at T0 for temperature sensor 1 This value is expressed in 0.1 kHz.
    #[inline(always)]
    pub fn ts1_fmt0(&self) -> TS1_FMT0_R {
        TS1_FMT0_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:17 - Engineering value of the T0 temperature for temperature sensor 1. Others: Reserved, must not be used.
    #[inline(always)]
    pub fn ts1_t0(&self) -> TS1_T0_R {
        TS1_T0_R::new(((self.bits >> 16) & 3) as u8)
    }
}
///Temperature sensor T0 value register 1
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [t0valr1](index.html) module
pub struct T0VALR1_SPEC;
impl crate::RegisterSpec for T0VALR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [t0valr1::R](R) reader structure
impl crate::Readable for T0VALR1_SPEC {
    type Reader = R;
}
///`reset()` method sets T0VALR1 to value 0
impl crate::Resettable for T0VALR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
