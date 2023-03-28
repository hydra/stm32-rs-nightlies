///Register `ALRBBINR` reader
pub struct R(crate::R<ALRBBINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALRBBINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALRBBINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALRBBINR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ALRBBINR` writer
pub struct W(crate::W<ALRBBINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALRBBINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ALRBBINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALRBBINR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SS` reader - Synchronous counter alarm value in Binary mode This value is compared with the contents of the synchronous counter to determine if Alarm Bis to be activated. Only bits 0 up MASKSS-1 are compared. SS\[14:0\]
///is the mirror of SS\[14:0\]
///in the RTC_ALRMBSSRR, and so can also be read or written through RTC_ALRMBSSR.
pub type SS_R = crate::FieldReader<u32, u32>;
///Field `SS` writer - Synchronous counter alarm value in Binary mode This value is compared with the contents of the synchronous counter to determine if Alarm Bis to be activated. Only bits 0 up MASKSS-1 are compared. SS\[14:0\]
///is the mirror of SS\[14:0\]
///in the RTC_ALRMBSSRR, and so can also be read or written through RTC_ALRMBSSR.
pub type SS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALRBBINR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Synchronous counter alarm value in Binary mode This value is compared with the contents of the synchronous counter to determine if Alarm Bis to be activated. Only bits 0 up MASKSS-1 are compared. SS\[14:0\]
    ///is the mirror of SS\[14:0\]
    ///in the RTC_ALRMBSSRR, and so can also be read or written through RTC_ALRMBSSR.
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Synchronous counter alarm value in Binary mode This value is compared with the contents of the synchronous counter to determine if Alarm Bis to be activated. Only bits 0 up MASKSS-1 are compared. SS\[14:0\]
    ///is the mirror of SS\[14:0\]
    ///in the RTC_ALRMBSSRR, and so can also be read or written through RTC_ALRMBSSR.
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SS_W<0> {
        SS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RTC alarm B binary mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [alrbbinr](index.html) module
pub struct ALRBBINR_SPEC;
impl crate::RegisterSpec for ALRBBINR_SPEC {
    type Ux = u32;
}
///`read()` method returns [alrbbinr::R](R) reader structure
impl crate::Readable for ALRBBINR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [alrbbinr::W](W) writer structure
impl crate::Writable for ALRBBINR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ALRBBINR to value 0
impl crate::Resettable for ALRBBINR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
