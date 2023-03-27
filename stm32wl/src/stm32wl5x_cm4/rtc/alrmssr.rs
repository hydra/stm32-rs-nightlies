///Register `ALRM%sSSR` reader
pub struct R(crate::R<ALRMSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALRMSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALRMSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALRMSSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ALRM%sSSR` writer
pub struct W(crate::W<ALRMSSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALRMSSR_SPEC>;
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
impl From<crate::W<ALRMSSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALRMSSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SS` reader - Sub seconds value
pub type SS_R = crate::FieldReader<u16, u16>;
///Field `SS` writer - Sub seconds value
pub type SS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ALRMSSR_SPEC, u16, u16, 15, O>;
///Field `MASKSS` reader - Mask the most-significant bits starting at this bit
pub type MASKSS_R = crate::FieldReader<u8, u8>;
///Field `MASKSS` writer - Mask the most-significant bits starting at this bit
pub type MASKSS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ALRMSSR_SPEC, u8, u8, 6, O>;
///Field `SSCLR` reader - Clear synchronous counter on alarm (Binary mode only)
pub type SSCLR_R = crate::BitReader<SSCLR_A>;
///Clear synchronous counter on alarm (Binary mode only)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSCLR_A {
    ///0: The synchronous binary counter (SS\[31:0\]
    ///in RTC_SSR) is free-running
    FreeRunning = 0,
    ///1: The synchronous binary counter (SS\[31:0\]
    ///in RTC_SSR) is running from 0xFFFF FFFF to RTC_ALRMABINR → SS\[31:0\]
    ///value and is automatically reloaded with 0xFFFF FFFF when reaching RTC_ALRMABINR → SS\[31:0\]
    Alrmbinr = 1,
}
impl From<SSCLR_A> for bool {
    #[inline(always)]
    fn from(variant: SSCLR_A) -> Self {
        variant as u8 != 0
    }
}
impl SSCLR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SSCLR_A {
        match self.bits {
            false => SSCLR_A::FreeRunning,
            true => SSCLR_A::Alrmbinr,
        }
    }
    ///Checks if the value of the field is `FreeRunning`
    #[inline(always)]
    pub fn is_free_running(&self) -> bool {
        *self == SSCLR_A::FreeRunning
    }
    ///Checks if the value of the field is `Alrmbinr`
    #[inline(always)]
    pub fn is_alrmbinr(&self) -> bool {
        *self == SSCLR_A::Alrmbinr
    }
}
///Field `SSCLR` writer - Clear synchronous counter on alarm (Binary mode only)
pub type SSCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALRMSSR_SPEC, SSCLR_A, O>;
impl<'a, const O: u8> SSCLR_W<'a, O> {
    ///The synchronous binary counter (SS\[31:0\]
    ///in RTC_SSR) is free-running
    #[inline(always)]
    pub fn free_running(self) -> &'a mut W {
        self.variant(SSCLR_A::FreeRunning)
    }
    ///The synchronous binary counter (SS\[31:0\]
    ///in RTC_SSR) is running from 0xFFFF FFFF to RTC_ALRMABINR → SS\[31:0\]
    ///value and is automatically reloaded with 0xFFFF FFFF when reaching RTC_ALRMABINR → SS\[31:0\]
    #[inline(always)]
    pub fn alrmbinr(self) -> &'a mut W {
        self.variant(SSCLR_A::Alrmbinr)
    }
}
impl R {
    ///Bits 0:14 - Sub seconds value
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0x7fff) as u16)
    }
    ///Bits 24:29 - Mask the most-significant bits starting at this bit
    #[inline(always)]
    pub fn maskss(&self) -> MASKSS_R {
        MASKSS_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    ///Bit 31 - Clear synchronous counter on alarm (Binary mode only)
    #[inline(always)]
    pub fn ssclr(&self) -> SSCLR_R {
        SSCLR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:14 - Sub seconds value
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SS_W<0> {
        SS_W::new(self)
    }
    ///Bits 24:29 - Mask the most-significant bits starting at this bit
    #[inline(always)]
    #[must_use]
    pub fn maskss(&mut self) -> MASKSS_W<24> {
        MASKSS_W::new(self)
    }
    ///Bit 31 - Clear synchronous counter on alarm (Binary mode only)
    #[inline(always)]
    #[must_use]
    pub fn ssclr(&mut self) -> SSCLR_W<31> {
        SSCLR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Alarm sub-second register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [alrmssr](index.html) module
pub struct ALRMSSR_SPEC;
impl crate::RegisterSpec for ALRMSSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [alrmssr::R](R) reader structure
impl crate::Readable for ALRMSSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [alrmssr::W](W) writer structure
impl crate::Writable for ALRMSSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ALRM%sSSR to value 0
impl crate::Resettable for ALRMSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
