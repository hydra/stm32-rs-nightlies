///Register `CALR` reader
pub struct R(crate::R<CALR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CALR` writer
pub struct W(crate::W<CALR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALR_SPEC>;
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
impl From<crate::W<CALR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CALM` reader - Calibration minus
pub type CALM_R = crate::FieldReader<u16, u16>;
///Field `CALM` writer - Calibration minus
pub type CALM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CALR_SPEC, u16, u16, 9, O>;
///Field `LPCAL` reader - Calibration low-power mode
pub type LPCAL_R = crate::BitReader<LPCAL_A>;
///Calibration low-power mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPCAL_A {
    ///0: Calibration window is 220 RTCCLK, which is a high-consumption mode. This mode should be set only when less than 32s calibration window is required
    Rtcclk = 0,
    ///1: Calibration window is 220 ck_apre, which is the required configuration for ultra-low consumption mode
    CkApre = 1,
}
impl From<LPCAL_A> for bool {
    #[inline(always)]
    fn from(variant: LPCAL_A) -> Self {
        variant as u8 != 0
    }
}
impl LPCAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LPCAL_A {
        match self.bits {
            false => LPCAL_A::Rtcclk,
            true => LPCAL_A::CkApre,
        }
    }
    ///Checks if the value of the field is `Rtcclk`
    #[inline(always)]
    pub fn is_rtcclk(&self) -> bool {
        *self == LPCAL_A::Rtcclk
    }
    ///Checks if the value of the field is `CkApre`
    #[inline(always)]
    pub fn is_ck_apre(&self) -> bool {
        *self == LPCAL_A::CkApre
    }
}
///Field `LPCAL` writer - Calibration low-power mode
pub type LPCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALR_SPEC, LPCAL_A, O>;
impl<'a, const O: u8> LPCAL_W<'a, O> {
    ///Calibration window is 220 RTCCLK, which is a high-consumption mode. This mode should be set only when less than 32s calibration window is required
    #[inline(always)]
    pub fn rtcclk(self) -> &'a mut W {
        self.variant(LPCAL_A::Rtcclk)
    }
    ///Calibration window is 220 ck_apre, which is the required configuration for ultra-low consumption mode
    #[inline(always)]
    pub fn ck_apre(self) -> &'a mut W {
        self.variant(LPCAL_A::CkApre)
    }
}
///Field `CALW16` reader - CALW16
pub type CALW16_R = crate::BitReader<CALW16_A>;
///CALW16
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALW16_A {
    ///1: When CALW16 is set to ‘1’, the 16-second calibration cycle period is selected.This bit must not be set to ‘1’ if CALW8=1
    SixteenSeconds = 1,
}
impl From<CALW16_A> for bool {
    #[inline(always)]
    fn from(variant: CALW16_A) -> Self {
        variant as u8 != 0
    }
}
impl CALW16_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CALW16_A> {
        match self.bits {
            true => Some(CALW16_A::SixteenSeconds),
            _ => None,
        }
    }
    ///Checks if the value of the field is `SixteenSeconds`
    #[inline(always)]
    pub fn is_sixteen_seconds(&self) -> bool {
        *self == CALW16_A::SixteenSeconds
    }
}
///Field `CALW16` writer - CALW16
pub type CALW16_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALR_SPEC, CALW16_A, O>;
impl<'a, const O: u8> CALW16_W<'a, O> {
    ///When CALW16 is set to ‘1’, the 16-second calibration cycle period is selected.This bit must not be set to ‘1’ if CALW8=1
    #[inline(always)]
    pub fn sixteen_seconds(self) -> &'a mut W {
        self.variant(CALW16_A::SixteenSeconds)
    }
}
///Field `CALW8` reader - Use a 16-second calibration cycle period
pub type CALW8_R = crate::BitReader<CALW8_A>;
///Use a 16-second calibration cycle period
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALW8_A {
    ///1: When CALW8 is set to ‘1’, the 8-second calibration cycle period is selected
    EightSeconds = 1,
}
impl From<CALW8_A> for bool {
    #[inline(always)]
    fn from(variant: CALW8_A) -> Self {
        variant as u8 != 0
    }
}
impl CALW8_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CALW8_A> {
        match self.bits {
            true => Some(CALW8_A::EightSeconds),
            _ => None,
        }
    }
    ///Checks if the value of the field is `EightSeconds`
    #[inline(always)]
    pub fn is_eight_seconds(&self) -> bool {
        *self == CALW8_A::EightSeconds
    }
}
///Field `CALW8` writer - Use a 16-second calibration cycle period
pub type CALW8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALR_SPEC, CALW8_A, O>;
impl<'a, const O: u8> CALW8_W<'a, O> {
    ///When CALW8 is set to ‘1’, the 8-second calibration cycle period is selected
    #[inline(always)]
    pub fn eight_seconds(self) -> &'a mut W {
        self.variant(CALW8_A::EightSeconds)
    }
}
///Field `CALP` reader - Use an 8-second calibration cycle period
pub type CALP_R = crate::BitReader<CALP_A>;
///Use an 8-second calibration cycle period
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALP_A {
    ///0: No RTCCLK pulses are added
    NoChange = 0,
    ///1: One RTCCLK pulse is effectively inserted every 2^11 pulses (frequency increased by 488.5 ppm)
    IncreaseFreq = 1,
}
impl From<CALP_A> for bool {
    #[inline(always)]
    fn from(variant: CALP_A) -> Self {
        variant as u8 != 0
    }
}
impl CALP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CALP_A {
        match self.bits {
            false => CALP_A::NoChange,
            true => CALP_A::IncreaseFreq,
        }
    }
    ///Checks if the value of the field is `NoChange`
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == CALP_A::NoChange
    }
    ///Checks if the value of the field is `IncreaseFreq`
    #[inline(always)]
    pub fn is_increase_freq(&self) -> bool {
        *self == CALP_A::IncreaseFreq
    }
}
///Field `CALP` writer - Use an 8-second calibration cycle period
pub type CALP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALR_SPEC, CALP_A, O>;
impl<'a, const O: u8> CALP_W<'a, O> {
    ///No RTCCLK pulses are added
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(CALP_A::NoChange)
    }
    ///One RTCCLK pulse is effectively inserted every 2^11 pulses (frequency increased by 488.5 ppm)
    #[inline(always)]
    pub fn increase_freq(self) -> &'a mut W {
        self.variant(CALP_A::IncreaseFreq)
    }
}
impl R {
    ///Bits 0:8 - Calibration minus
    #[inline(always)]
    pub fn calm(&self) -> CALM_R {
        CALM_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 12 - Calibration low-power mode
    #[inline(always)]
    pub fn lpcal(&self) -> LPCAL_R {
        LPCAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CALW16
    #[inline(always)]
    pub fn calw16(&self) -> CALW16_R {
        CALW16_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Use a 16-second calibration cycle period
    #[inline(always)]
    pub fn calw8(&self) -> CALW8_R {
        CALW8_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Use an 8-second calibration cycle period
    #[inline(always)]
    pub fn calp(&self) -> CALP_R {
        CALP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 0:8 - Calibration minus
    #[inline(always)]
    #[must_use]
    pub fn calm(&mut self) -> CALM_W<0> {
        CALM_W::new(self)
    }
    ///Bit 12 - Calibration low-power mode
    #[inline(always)]
    #[must_use]
    pub fn lpcal(&mut self) -> LPCAL_W<12> {
        LPCAL_W::new(self)
    }
    ///Bit 13 - CALW16
    #[inline(always)]
    #[must_use]
    pub fn calw16(&mut self) -> CALW16_W<13> {
        CALW16_W::new(self)
    }
    ///Bit 14 - Use a 16-second calibration cycle period
    #[inline(always)]
    #[must_use]
    pub fn calw8(&mut self) -> CALW8_W<14> {
        CALW8_W::new(self)
    }
    ///Bit 15 - Use an 8-second calibration cycle period
    #[inline(always)]
    #[must_use]
    pub fn calp(&mut self) -> CALP_W<15> {
        CALP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Calibration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [calr](index.html) module
pub struct CALR_SPEC;
impl crate::RegisterSpec for CALR_SPEC {
    type Ux = u32;
}
///`read()` method returns [calr::R](R) reader structure
impl crate::Readable for CALR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [calr::W](W) writer structure
impl crate::Writable for CALR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CALR to value 0
impl crate::Resettable for CALR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
