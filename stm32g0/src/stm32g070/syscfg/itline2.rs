///Register `ITLINE2` reader
pub struct R(crate::R<ITLINE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TAMP` reader - TAMP
pub type TAMP_R = crate::BitReader<bool>;
///Field `RTC` reader - RTC
pub type RTC_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - TAMP
    #[inline(always)]
    pub fn tamp(&self) -> TAMP_R {
        TAMP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RTC
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
///interrupt line 2 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [itline2](index.html) module
pub struct ITLINE2_SPEC;
impl crate::RegisterSpec for ITLINE2_SPEC {
    type Ux = u32;
}
///`read()` method returns [itline2::R](R) reader structure
impl crate::Readable for ITLINE2_SPEC {
    type Reader = R;
}
///`reset()` method sets ITLINE2 to value 0
impl crate::Resettable for ITLINE2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
