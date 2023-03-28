///Register `SMISR` reader
pub struct R(crate::R<SMISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ALRAMF` reader - Alarm A interrupt secure masked flag This flag is set by hardware when the alarm A secure interrupt occurs.
pub type ALRAMF_R = crate::BitReader<bool>;
///Field `ALRBMF` reader - Alarm B interrupt secure masked flag This flag is set by hardware when the alarm B secure interrupt occurs.
pub type ALRBMF_R = crate::BitReader<bool>;
///Field `WUTMF` reader - Wakeup timer interrupt secure masked flag This flag is set by hardware when the wakeup timer secure interrupt occurs. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again.
pub type WUTMF_R = crate::BitReader<bool>;
///Field `TSMF` reader - Timestamp interrupt secure masked flag This flag is set by hardware when a timestamp secure interrupt occurs. If ITSF flag is set, TSF must be cleared together with ITSF.
pub type TSMF_R = crate::BitReader<bool>;
///Field `TSOVMF` reader - Timestamp overflow interrupt secure masked flag This flag is set by hardware when a timestamp secure interrupt occurs while TSMF is already set. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared.
pub type TSOVMF_R = crate::BitReader<bool>;
///Field `ITSMF` reader - Internal timestamp interrupt secure masked flag This flag is set by hardware when a timestamp on the internal event occurs and timestamp secure interrupt is raised.
pub type ITSMF_R = crate::BitReader<bool>;
///Field `SSRUMF` reader - SSR underflow secure masked flag This flag is set by hardware when the SSR underflow secure interrupt occurs.
pub type SSRUMF_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Alarm A interrupt secure masked flag This flag is set by hardware when the alarm A secure interrupt occurs.
    #[inline(always)]
    pub fn alramf(&self) -> ALRAMF_R {
        ALRAMF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Alarm B interrupt secure masked flag This flag is set by hardware when the alarm B secure interrupt occurs.
    #[inline(always)]
    pub fn alrbmf(&self) -> ALRBMF_R {
        ALRBMF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup timer interrupt secure masked flag This flag is set by hardware when the wakeup timer secure interrupt occurs. This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1 again.
    #[inline(always)]
    pub fn wutmf(&self) -> WUTMF_R {
        WUTMF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timestamp interrupt secure masked flag This flag is set by hardware when a timestamp secure interrupt occurs. If ITSF flag is set, TSF must be cleared together with ITSF.
    #[inline(always)]
    pub fn tsmf(&self) -> TSMF_R {
        TSMF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timestamp overflow interrupt secure masked flag This flag is set by hardware when a timestamp secure interrupt occurs while TSMF is already set. It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise, an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit is cleared.
    #[inline(always)]
    pub fn tsovmf(&self) -> TSOVMF_R {
        TSOVMF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Internal timestamp interrupt secure masked flag This flag is set by hardware when a timestamp on the internal event occurs and timestamp secure interrupt is raised.
    #[inline(always)]
    pub fn itsmf(&self) -> ITSMF_R {
        ITSMF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SSR underflow secure masked flag This flag is set by hardware when the SSR underflow secure interrupt occurs.
    #[inline(always)]
    pub fn ssrumf(&self) -> SSRUMF_R {
        SSRUMF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
///RTC secure masked interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [smisr](index.html) module
pub struct SMISR_SPEC;
impl crate::RegisterSpec for SMISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [smisr::R](R) reader structure
impl crate::Readable for SMISR_SPEC {
    type Reader = R;
}
///`reset()` method sets SMISR to value 0
impl crate::Resettable for SMISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
