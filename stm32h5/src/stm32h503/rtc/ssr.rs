///Register `SSR` reader
pub struct R(crate::R<SSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SS` reader - Synchronous binary counter SS\[31:16\]: Synchronous binary counter MSB values When Binary or Mixed mode is selected (BIN = 01 or 10 or 11): SS\[31:16\]
///are the 16 MSB of the SS\[31:0\]
///free-running down-counter. When BCD mode is selected (BIN=00): SS\[31:16\]
///are forced by hardware to 0x0000. SS\[15:0\]: Subsecond value/synchronous binary counter LSB values When Binary mode is selected (BIN = 01 or 10 or 11): SS\[15:0\]
///are the 16 LSB of the SS\[31:0\]
///free-running down-counter. When BCD mode is selected (BIN=00): SS\[15:0\]
///is the value in the synchronous prescaler counter. The fraction of a second is given by the formula below: Second fraction = (PREDIV_S - SS) / (PREDIV_S + 1) SS can be larger than PREDIV_S only after a shift operation. In that case, the correct time/date is one second less than as indicated by RTC_TR/RTC_DR.
pub type SS_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Synchronous binary counter SS\[31:16\]: Synchronous binary counter MSB values When Binary or Mixed mode is selected (BIN = 01 or 10 or 11): SS\[31:16\]
    ///are the 16 MSB of the SS\[31:0\]
    ///free-running down-counter. When BCD mode is selected (BIN=00): SS\[31:16\]
    ///are forced by hardware to 0x0000. SS\[15:0\]: Subsecond value/synchronous binary counter LSB values When Binary mode is selected (BIN = 01 or 10 or 11): SS\[15:0\]
    ///are the 16 LSB of the SS\[31:0\]
    ///free-running down-counter. When BCD mode is selected (BIN=00): SS\[15:0\]
    ///is the value in the synchronous prescaler counter. The fraction of a second is given by the formula below: Second fraction = (PREDIV_S - SS) / (PREDIV_S + 1) SS can be larger than PREDIV_S only after a shift operation. In that case, the correct time/date is one second less than as indicated by RTC_TR/RTC_DR.
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(self.bits)
    }
}
///RTC subsecond register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ssr](index.html) module
pub struct SSR_SPEC;
impl crate::RegisterSpec for SSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ssr::R](R) reader structure
impl crate::Readable for SSR_SPEC {
    type Reader = R;
}
///`reset()` method sets SSR to value 0
impl crate::Resettable for SSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
