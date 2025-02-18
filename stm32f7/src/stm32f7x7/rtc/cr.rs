///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WUCKSEL` reader - Wakeup clock selection
pub type WUCKSEL_R = crate::FieldReader<u8, WUCKSEL_A>;
///Wakeup clock selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUCKSEL_A {
    ///0: RTC/16 clock is selected
    Div16 = 0,
    ///1: RTC/8 clock is selected
    Div8 = 1,
    ///2: RTC/4 clock is selected
    Div4 = 2,
    ///3: RTC/2 clock is selected
    Div2 = 3,
    ///4: ck_spre (usually 1 Hz) clock is selected
    ClockSpare = 4,
    ///6: ck_spre (usually 1 Hz) clock is selected and 2^16 is added to the WUT counter value
    ClockSpareWithOffset = 6,
}
impl From<WUCKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WUCKSEL_A) -> Self {
        variant as _
    }
}
impl WUCKSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<WUCKSEL_A> {
        match self.bits {
            0 => Some(WUCKSEL_A::Div16),
            1 => Some(WUCKSEL_A::Div8),
            2 => Some(WUCKSEL_A::Div4),
            3 => Some(WUCKSEL_A::Div2),
            4 => Some(WUCKSEL_A::ClockSpare),
            6 => Some(WUCKSEL_A::ClockSpareWithOffset),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == WUCKSEL_A::Div16
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == WUCKSEL_A::Div8
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == WUCKSEL_A::Div4
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == WUCKSEL_A::Div2
    }
    ///Checks if the value of the field is `ClockSpare`
    #[inline(always)]
    pub fn is_clock_spare(&self) -> bool {
        *self == WUCKSEL_A::ClockSpare
    }
    ///Checks if the value of the field is `ClockSpareWithOffset`
    #[inline(always)]
    pub fn is_clock_spare_with_offset(&self) -> bool {
        *self == WUCKSEL_A::ClockSpareWithOffset
    }
}
///Field `WUCKSEL` writer - Wakeup clock selection
pub type WUCKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, WUCKSEL_A, 3, O>;
impl<'a, const O: u8> WUCKSEL_W<'a, O> {
    ///RTC/16 clock is selected
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(WUCKSEL_A::Div16)
    }
    ///RTC/8 clock is selected
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(WUCKSEL_A::Div8)
    }
    ///RTC/4 clock is selected
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(WUCKSEL_A::Div4)
    }
    ///RTC/2 clock is selected
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(WUCKSEL_A::Div2)
    }
    ///ck_spre (usually 1 Hz) clock is selected
    #[inline(always)]
    pub fn clock_spare(self) -> &'a mut W {
        self.variant(WUCKSEL_A::ClockSpare)
    }
    ///ck_spre (usually 1 Hz) clock is selected and 2^16 is added to the WUT counter value
    #[inline(always)]
    pub fn clock_spare_with_offset(self) -> &'a mut W {
        self.variant(WUCKSEL_A::ClockSpareWithOffset)
    }
}
///Field `TSEDGE` reader - Time-stamp event active edge
pub type TSEDGE_R = crate::BitReader<TSEDGE_A>;
///Time-stamp event active edge
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSEDGE_A {
    ///0: RTC_TS input rising edge generates a time-stamp event
    RisingEdge = 0,
    ///1: RTC_TS input falling edge generates a time-stamp event
    FallingEdge = 1,
}
impl From<TSEDGE_A> for bool {
    #[inline(always)]
    fn from(variant: TSEDGE_A) -> Self {
        variant as u8 != 0
    }
}
impl TSEDGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TSEDGE_A {
        match self.bits {
            false => TSEDGE_A::RisingEdge,
            true => TSEDGE_A::FallingEdge,
        }
    }
    ///Checks if the value of the field is `RisingEdge`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == TSEDGE_A::RisingEdge
    }
    ///Checks if the value of the field is `FallingEdge`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == TSEDGE_A::FallingEdge
    }
}
///Field `TSEDGE` writer - Time-stamp event active edge
pub type TSEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TSEDGE_A, O>;
impl<'a, const O: u8> TSEDGE_W<'a, O> {
    ///RTC_TS input rising edge generates a time-stamp event
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(TSEDGE_A::RisingEdge)
    }
    ///RTC_TS input falling edge generates a time-stamp event
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(TSEDGE_A::FallingEdge)
    }
}
///Field `REFCKON` reader - Reference clock detection enable (50 or 60 Hz)
pub type REFCKON_R = crate::BitReader<REFCKON_A>;
///Reference clock detection enable (50 or 60 Hz)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFCKON_A {
    ///0: RTC_REFIN detection disabled
    Disabled = 0,
    ///1: RTC_REFIN detection enabled
    Enabled = 1,
}
impl From<REFCKON_A> for bool {
    #[inline(always)]
    fn from(variant: REFCKON_A) -> Self {
        variant as u8 != 0
    }
}
impl REFCKON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> REFCKON_A {
        match self.bits {
            false => REFCKON_A::Disabled,
            true => REFCKON_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REFCKON_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REFCKON_A::Enabled
    }
}
///Field `REFCKON` writer - Reference clock detection enable (50 or 60 Hz)
pub type REFCKON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, REFCKON_A, O>;
impl<'a, const O: u8> REFCKON_W<'a, O> {
    ///RTC_REFIN detection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REFCKON_A::Disabled)
    }
    ///RTC_REFIN detection enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REFCKON_A::Enabled)
    }
}
///Field `BYPSHAD` reader - Bypass the shadow registers
pub type BYPSHAD_R = crate::BitReader<BYPSHAD_A>;
///Bypass the shadow registers
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPSHAD_A {
    ///0: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles
    ShadowReg = 0,
    ///1: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters
    BypassShadowReg = 1,
}
impl From<BYPSHAD_A> for bool {
    #[inline(always)]
    fn from(variant: BYPSHAD_A) -> Self {
        variant as u8 != 0
    }
}
impl BYPSHAD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BYPSHAD_A {
        match self.bits {
            false => BYPSHAD_A::ShadowReg,
            true => BYPSHAD_A::BypassShadowReg,
        }
    }
    ///Checks if the value of the field is `ShadowReg`
    #[inline(always)]
    pub fn is_shadow_reg(&self) -> bool {
        *self == BYPSHAD_A::ShadowReg
    }
    ///Checks if the value of the field is `BypassShadowReg`
    #[inline(always)]
    pub fn is_bypass_shadow_reg(&self) -> bool {
        *self == BYPSHAD_A::BypassShadowReg
    }
}
///Field `BYPSHAD` writer - Bypass the shadow registers
pub type BYPSHAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, BYPSHAD_A, O>;
impl<'a, const O: u8> BYPSHAD_W<'a, O> {
    ///Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles
    #[inline(always)]
    pub fn shadow_reg(self) -> &'a mut W {
        self.variant(BYPSHAD_A::ShadowReg)
    }
    ///Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters
    #[inline(always)]
    pub fn bypass_shadow_reg(self) -> &'a mut W {
        self.variant(BYPSHAD_A::BypassShadowReg)
    }
}
///Field `FMT` reader - Hour format
pub type FMT_R = crate::BitReader<FMT_A>;
///Hour format
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMT_A {
    ///0: 24 hour/day format
    TwentyFourHour = 0,
    ///1: AM/PM hour format
    AmPm = 1,
}
impl From<FMT_A> for bool {
    #[inline(always)]
    fn from(variant: FMT_A) -> Self {
        variant as u8 != 0
    }
}
impl FMT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FMT_A {
        match self.bits {
            false => FMT_A::TwentyFourHour,
            true => FMT_A::AmPm,
        }
    }
    ///Checks if the value of the field is `TwentyFourHour`
    #[inline(always)]
    pub fn is_twenty_four_hour(&self) -> bool {
        *self == FMT_A::TwentyFourHour
    }
    ///Checks if the value of the field is `AmPm`
    #[inline(always)]
    pub fn is_am_pm(&self) -> bool {
        *self == FMT_A::AmPm
    }
}
///Field `FMT` writer - Hour format
pub type FMT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, FMT_A, O>;
impl<'a, const O: u8> FMT_W<'a, O> {
    ///24 hour/day format
    #[inline(always)]
    pub fn twenty_four_hour(self) -> &'a mut W {
        self.variant(FMT_A::TwentyFourHour)
    }
    ///AM/PM hour format
    #[inline(always)]
    pub fn am_pm(self) -> &'a mut W {
        self.variant(FMT_A::AmPm)
    }
}
///Field `ALRAE` reader - Alarm A enable
pub type ALRAE_R = crate::BitReader<ALRAE_A>;
///Alarm A enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAE_A {
    ///0: Alarm A disabled
    Disabled = 0,
    ///1: Alarm A enabled
    Enabled = 1,
}
impl From<ALRAE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAE_A) -> Self {
        variant as u8 != 0
    }
}
impl ALRAE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALRAE_A {
        match self.bits {
            false => ALRAE_A::Disabled,
            true => ALRAE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALRAE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALRAE_A::Enabled
    }
}
///Field `ALRAE` writer - Alarm A enable
pub type ALRAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ALRAE_A, O>;
impl<'a, const O: u8> ALRAE_W<'a, O> {
    ///Alarm A disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALRAE_A::Disabled)
    }
    ///Alarm A enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALRAE_A::Enabled)
    }
}
///Field `ALRBE` reader - Alarm B enable
pub type ALRBE_R = crate::BitReader<ALRBE_A>;
///Alarm B enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRBE_A {
    ///0: Alarm B disabled
    Disabled = 0,
    ///1: Alarm B enabled
    Enabled = 1,
}
impl From<ALRBE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRBE_A) -> Self {
        variant as u8 != 0
    }
}
impl ALRBE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALRBE_A {
        match self.bits {
            false => ALRBE_A::Disabled,
            true => ALRBE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALRBE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALRBE_A::Enabled
    }
}
///Field `ALRBE` writer - Alarm B enable
pub type ALRBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ALRBE_A, O>;
impl<'a, const O: u8> ALRBE_W<'a, O> {
    ///Alarm B disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALRBE_A::Disabled)
    }
    ///Alarm B enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALRBE_A::Enabled)
    }
}
///Field `WUTE` reader - Wakeup timer enable
pub type WUTE_R = crate::BitReader<WUTE_A>;
///Wakeup timer enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTE_A {
    ///0: Wakeup timer disabled
    Disabled = 0,
    ///1: Wakeup timer enabled
    Enabled = 1,
}
impl From<WUTE_A> for bool {
    #[inline(always)]
    fn from(variant: WUTE_A) -> Self {
        variant as u8 != 0
    }
}
impl WUTE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUTE_A {
        match self.bits {
            false => WUTE_A::Disabled,
            true => WUTE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUTE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUTE_A::Enabled
    }
}
///Field `WUTE` writer - Wakeup timer enable
pub type WUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, WUTE_A, O>;
impl<'a, const O: u8> WUTE_W<'a, O> {
    ///Wakeup timer disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WUTE_A::Disabled)
    }
    ///Wakeup timer enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WUTE_A::Enabled)
    }
}
///Field `TSE` reader - Time stamp enable
pub type TSE_R = crate::BitReader<TSE_A>;
///Time stamp enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSE_A {
    ///0: Timestamp disabled
    Disabled = 0,
    ///1: Timestamp enabled
    Enabled = 1,
}
impl From<TSE_A> for bool {
    #[inline(always)]
    fn from(variant: TSE_A) -> Self {
        variant as u8 != 0
    }
}
impl TSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TSE_A {
        match self.bits {
            false => TSE_A::Disabled,
            true => TSE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSE_A::Enabled
    }
}
///Field `TSE` writer - Time stamp enable
pub type TSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TSE_A, O>;
impl<'a, const O: u8> TSE_W<'a, O> {
    ///Timestamp disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSE_A::Disabled)
    }
    ///Timestamp enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSE_A::Enabled)
    }
}
///Field `ALRAIE` reader - Alarm A interrupt enable
pub type ALRAIE_R = crate::BitReader<ALRAIE_A>;
///Alarm A interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAIE_A {
    ///0: Alarm A interrupt disabled
    Disabled = 0,
    ///1: Alarm A interrupt enabled
    Enabled = 1,
}
impl From<ALRAIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ALRAIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALRAIE_A {
        match self.bits {
            false => ALRAIE_A::Disabled,
            true => ALRAIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALRAIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALRAIE_A::Enabled
    }
}
///Field `ALRAIE` writer - Alarm A interrupt enable
pub type ALRAIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ALRAIE_A, O>;
impl<'a, const O: u8> ALRAIE_W<'a, O> {
    ///Alarm A interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALRAIE_A::Disabled)
    }
    ///Alarm A interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALRAIE_A::Enabled)
    }
}
///Field `ALRBIE` reader - Alarm B interrupt enable
pub type ALRBIE_R = crate::BitReader<ALRBIE_A>;
///Alarm B interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRBIE_A {
    ///0: Alarm B Interrupt disabled
    Disabled = 0,
    ///1: Alarm B Interrupt enabled
    Enabled = 1,
}
impl From<ALRBIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRBIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ALRBIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALRBIE_A {
        match self.bits {
            false => ALRBIE_A::Disabled,
            true => ALRBIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALRBIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALRBIE_A::Enabled
    }
}
///Field `ALRBIE` writer - Alarm B interrupt enable
pub type ALRBIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ALRBIE_A, O>;
impl<'a, const O: u8> ALRBIE_W<'a, O> {
    ///Alarm B Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALRBIE_A::Disabled)
    }
    ///Alarm B Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALRBIE_A::Enabled)
    }
}
///Field `WUTIE` reader - Wakeup timer interrupt enable
pub type WUTIE_R = crate::BitReader<WUTIE_A>;
///Wakeup timer interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTIE_A {
    ///0: Wakeup timer interrupt disabled
    Disabled = 0,
    ///1: Wakeup timer interrupt enabled
    Enabled = 1,
}
impl From<WUTIE_A> for bool {
    #[inline(always)]
    fn from(variant: WUTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl WUTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WUTIE_A {
        match self.bits {
            false => WUTIE_A::Disabled,
            true => WUTIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUTIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUTIE_A::Enabled
    }
}
///Field `WUTIE` writer - Wakeup timer interrupt enable
pub type WUTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, WUTIE_A, O>;
impl<'a, const O: u8> WUTIE_W<'a, O> {
    ///Wakeup timer interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WUTIE_A::Disabled)
    }
    ///Wakeup timer interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WUTIE_A::Enabled)
    }
}
///Field `TSIE` reader - Time-stamp interrupt enable
pub type TSIE_R = crate::BitReader<TSIE_A>;
///Time-stamp interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSIE_A {
    ///0: Time-stamp Interrupt disabled
    Disabled = 0,
    ///1: Time-stamp Interrupt enabled
    Enabled = 1,
}
impl From<TSIE_A> for bool {
    #[inline(always)]
    fn from(variant: TSIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TSIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TSIE_A {
        match self.bits {
            false => TSIE_A::Disabled,
            true => TSIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSIE_A::Enabled
    }
}
///Field `TSIE` writer - Time-stamp interrupt enable
pub type TSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TSIE_A, O>;
impl<'a, const O: u8> TSIE_W<'a, O> {
    ///Time-stamp Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSIE_A::Disabled)
    }
    ///Time-stamp Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSIE_A::Enabled)
    }
}
///Field `ADD1H` reader - Add 1 hour (summer time change)
pub type ADD1H_R = crate::BitReader<ADD1HW_A>;
///Add 1 hour (summer time change)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADD1HW_A {
    ///1: Adds 1 hour to the current time. This can be used for summer time change outside initialization mode
    Add1 = 1,
}
impl From<ADD1HW_A> for bool {
    #[inline(always)]
    fn from(variant: ADD1HW_A) -> Self {
        variant as u8 != 0
    }
}
impl ADD1H_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<ADD1HW_A> {
        match self.bits {
            true => Some(ADD1HW_A::Add1),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Add1`
    #[inline(always)]
    pub fn is_add1(&self) -> bool {
        *self == ADD1HW_A::Add1
    }
}
///Field `ADD1H` writer - Add 1 hour (summer time change)
pub type ADD1H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, ADD1HW_A, O>;
impl<'a, const O: u8> ADD1H_W<'a, O> {
    ///Adds 1 hour to the current time. This can be used for summer time change outside initialization mode
    #[inline(always)]
    pub fn add1(self) -> &'a mut W {
        self.variant(ADD1HW_A::Add1)
    }
}
///Field `SUB1H` reader - Subtract 1 hour (winter time change)
pub type SUB1H_R = crate::BitReader<SUB1HW_A>;
///Subtract 1 hour (winter time change)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUB1HW_A {
    ///1: Subtracts 1 hour to the current time. This can be used for winter time change outside initialization mode
    Sub1 = 1,
}
impl From<SUB1HW_A> for bool {
    #[inline(always)]
    fn from(variant: SUB1HW_A) -> Self {
        variant as u8 != 0
    }
}
impl SUB1H_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SUB1HW_A> {
        match self.bits {
            true => Some(SUB1HW_A::Sub1),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Sub1`
    #[inline(always)]
    pub fn is_sub1(&self) -> bool {
        *self == SUB1HW_A::Sub1
    }
}
///Field `SUB1H` writer - Subtract 1 hour (winter time change)
pub type SUB1H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, SUB1HW_A, O>;
impl<'a, const O: u8> SUB1H_W<'a, O> {
    ///Subtracts 1 hour to the current time. This can be used for winter time change outside initialization mode
    #[inline(always)]
    pub fn sub1(self) -> &'a mut W {
        self.variant(SUB1HW_A::Sub1)
    }
}
///Field `BKP` reader - Backup
pub type BKP_R = crate::BitReader<BKP_A>;
///Backup
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKP_A {
    ///0: Daylight Saving Time change has not been performed
    DstNotChanged = 0,
    ///1: Daylight Saving Time change has been performed
    DstChanged = 1,
}
impl From<BKP_A> for bool {
    #[inline(always)]
    fn from(variant: BKP_A) -> Self {
        variant as u8 != 0
    }
}
impl BKP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BKP_A {
        match self.bits {
            false => BKP_A::DstNotChanged,
            true => BKP_A::DstChanged,
        }
    }
    ///Checks if the value of the field is `DstNotChanged`
    #[inline(always)]
    pub fn is_dst_not_changed(&self) -> bool {
        *self == BKP_A::DstNotChanged
    }
    ///Checks if the value of the field is `DstChanged`
    #[inline(always)]
    pub fn is_dst_changed(&self) -> bool {
        *self == BKP_A::DstChanged
    }
}
///Field `BKP` writer - Backup
pub type BKP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, BKP_A, O>;
impl<'a, const O: u8> BKP_W<'a, O> {
    ///Daylight Saving Time change has not been performed
    #[inline(always)]
    pub fn dst_not_changed(self) -> &'a mut W {
        self.variant(BKP_A::DstNotChanged)
    }
    ///Daylight Saving Time change has been performed
    #[inline(always)]
    pub fn dst_changed(self) -> &'a mut W {
        self.variant(BKP_A::DstChanged)
    }
}
///Field `COSEL` reader - Calibration output selection
pub type COSEL_R = crate::BitReader<COSEL_A>;
///Calibration output selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COSEL_A {
    ///0: Calibration output is 512 Hz (with default prescaler setting)
    CalFreq512hz = 0,
    ///1: Calibration output is 1 Hz (with default prescaler setting)
    CalFreq1hz = 1,
}
impl From<COSEL_A> for bool {
    #[inline(always)]
    fn from(variant: COSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl COSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> COSEL_A {
        match self.bits {
            false => COSEL_A::CalFreq512hz,
            true => COSEL_A::CalFreq1hz,
        }
    }
    ///Checks if the value of the field is `CalFreq512hz`
    #[inline(always)]
    pub fn is_cal_freq_512hz(&self) -> bool {
        *self == COSEL_A::CalFreq512hz
    }
    ///Checks if the value of the field is `CalFreq1hz`
    #[inline(always)]
    pub fn is_cal_freq_1hz(&self) -> bool {
        *self == COSEL_A::CalFreq1hz
    }
}
///Field `COSEL` writer - Calibration output selection
pub type COSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, COSEL_A, O>;
impl<'a, const O: u8> COSEL_W<'a, O> {
    ///Calibration output is 512 Hz (with default prescaler setting)
    #[inline(always)]
    pub fn cal_freq_512hz(self) -> &'a mut W {
        self.variant(COSEL_A::CalFreq512hz)
    }
    ///Calibration output is 1 Hz (with default prescaler setting)
    #[inline(always)]
    pub fn cal_freq_1hz(self) -> &'a mut W {
        self.variant(COSEL_A::CalFreq1hz)
    }
}
///Field `POL` reader - Output polarity
pub type POL_R = crate::BitReader<POL_A>;
///Output polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_A {
    ///0: The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\])
    High = 0,
    ///1: The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\])
    Low = 1,
}
impl From<POL_A> for bool {
    #[inline(always)]
    fn from(variant: POL_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> POL_A {
        match self.bits {
            false => POL_A::High,
            true => POL_A::Low,
        }
    }
    ///Checks if the value of the field is `High`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_A::High
    }
    ///Checks if the value of the field is `Low`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_A::Low
    }
}
///Field `POL` writer - Output polarity
pub type POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, POL_A, O>;
impl<'a, const O: u8> POL_W<'a, O> {
    ///The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\])
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_A::High)
    }
    ///The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\[1:0\])
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_A::Low)
    }
}
///Field `OSEL` reader - Output selection
pub type OSEL_R = crate::FieldReader<u8, OSEL_A>;
///Output selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSEL_A {
    ///0: Output disabled
    Disabled = 0,
    ///1: Alarm A output enabled
    AlarmA = 1,
    ///2: Alarm B output enabled
    AlarmB = 2,
    ///3: Wakeup output enabled
    Wakeup = 3,
}
impl From<OSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OSEL_A) -> Self {
        variant as _
    }
}
impl OSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OSEL_A {
        match self.bits {
            0 => OSEL_A::Disabled,
            1 => OSEL_A::AlarmA,
            2 => OSEL_A::AlarmB,
            3 => OSEL_A::Wakeup,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSEL_A::Disabled
    }
    ///Checks if the value of the field is `AlarmA`
    #[inline(always)]
    pub fn is_alarm_a(&self) -> bool {
        *self == OSEL_A::AlarmA
    }
    ///Checks if the value of the field is `AlarmB`
    #[inline(always)]
    pub fn is_alarm_b(&self) -> bool {
        *self == OSEL_A::AlarmB
    }
    ///Checks if the value of the field is `Wakeup`
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == OSEL_A::Wakeup
    }
}
///Field `OSEL` writer - Output selection
pub type OSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, OSEL_A, 2, O>;
impl<'a, const O: u8> OSEL_W<'a, O> {
    ///Output disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSEL_A::Disabled)
    }
    ///Alarm A output enabled
    #[inline(always)]
    pub fn alarm_a(self) -> &'a mut W {
        self.variant(OSEL_A::AlarmA)
    }
    ///Alarm B output enabled
    #[inline(always)]
    pub fn alarm_b(self) -> &'a mut W {
        self.variant(OSEL_A::AlarmB)
    }
    ///Wakeup output enabled
    #[inline(always)]
    pub fn wakeup(self) -> &'a mut W {
        self.variant(OSEL_A::Wakeup)
    }
}
///Field `COE` reader - Calibration output enable
pub type COE_R = crate::BitReader<COE_A>;
///Calibration output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COE_A {
    ///0: Calibration output disabled
    Disabled = 0,
    ///1: Calibration output enabled
    Enabled = 1,
}
impl From<COE_A> for bool {
    #[inline(always)]
    fn from(variant: COE_A) -> Self {
        variant as u8 != 0
    }
}
impl COE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> COE_A {
        match self.bits {
            false => COE_A::Disabled,
            true => COE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COE_A::Enabled
    }
}
///Field `COE` writer - Calibration output enable
pub type COE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, COE_A, O>;
impl<'a, const O: u8> COE_W<'a, O> {
    ///Calibration output disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COE_A::Disabled)
    }
    ///Calibration output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COE_A::Enabled)
    }
}
///Field `ITSE` reader - timestamp on internal event enable
pub type ITSE_R = crate::BitReader<bool>;
///Field `ITSE` writer - timestamp on internal event enable
pub type ITSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    ///Bits 0:2 - Wakeup clock selection
    #[inline(always)]
    pub fn wucksel(&self) -> WUCKSEL_R {
        WUCKSEL_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Time-stamp event active edge
    #[inline(always)]
    pub fn tsedge(&self) -> TSEDGE_R {
        TSEDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Reference clock detection enable (50 or 60 Hz)
    #[inline(always)]
    pub fn refckon(&self) -> REFCKON_R {
        REFCKON_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Bypass the shadow registers
    #[inline(always)]
    pub fn bypshad(&self) -> BYPSHAD_R {
        BYPSHAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Hour format
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Alarm A enable
    #[inline(always)]
    pub fn alrae(&self) -> ALRAE_R {
        ALRAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Alarm B enable
    #[inline(always)]
    pub fn alrbe(&self) -> ALRBE_R {
        ALRBE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Wakeup timer enable
    #[inline(always)]
    pub fn wute(&self) -> WUTE_R {
        WUTE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Time stamp enable
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Alarm A interrupt enable
    #[inline(always)]
    pub fn alraie(&self) -> ALRAIE_R {
        ALRAIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Alarm B interrupt enable
    #[inline(always)]
    pub fn alrbie(&self) -> ALRBIE_R {
        ALRBIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Wakeup timer interrupt enable
    #[inline(always)]
    pub fn wutie(&self) -> WUTIE_R {
        WUTIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Time-stamp interrupt enable
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Add 1 hour (summer time change)
    #[inline(always)]
    pub fn add1h(&self) -> ADD1H_R {
        ADD1H_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Subtract 1 hour (winter time change)
    #[inline(always)]
    pub fn sub1h(&self) -> SUB1H_R {
        SUB1H_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Backup
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Calibration output selection
    #[inline(always)]
    pub fn cosel(&self) -> COSEL_R {
        COSEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Output polarity
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - Output selection
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 23 - Calibration output enable
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - timestamp on internal event enable
    #[inline(always)]
    pub fn itse(&self) -> ITSE_R {
        ITSE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - Wakeup clock selection
    #[inline(always)]
    #[must_use]
    pub fn wucksel(&mut self) -> WUCKSEL_W<0> {
        WUCKSEL_W::new(self)
    }
    ///Bit 3 - Time-stamp event active edge
    #[inline(always)]
    #[must_use]
    pub fn tsedge(&mut self) -> TSEDGE_W<3> {
        TSEDGE_W::new(self)
    }
    ///Bit 4 - Reference clock detection enable (50 or 60 Hz)
    #[inline(always)]
    #[must_use]
    pub fn refckon(&mut self) -> REFCKON_W<4> {
        REFCKON_W::new(self)
    }
    ///Bit 5 - Bypass the shadow registers
    #[inline(always)]
    #[must_use]
    pub fn bypshad(&mut self) -> BYPSHAD_W<5> {
        BYPSHAD_W::new(self)
    }
    ///Bit 6 - Hour format
    #[inline(always)]
    #[must_use]
    pub fn fmt(&mut self) -> FMT_W<6> {
        FMT_W::new(self)
    }
    ///Bit 8 - Alarm A enable
    #[inline(always)]
    #[must_use]
    pub fn alrae(&mut self) -> ALRAE_W<8> {
        ALRAE_W::new(self)
    }
    ///Bit 9 - Alarm B enable
    #[inline(always)]
    #[must_use]
    pub fn alrbe(&mut self) -> ALRBE_W<9> {
        ALRBE_W::new(self)
    }
    ///Bit 10 - Wakeup timer enable
    #[inline(always)]
    #[must_use]
    pub fn wute(&mut self) -> WUTE_W<10> {
        WUTE_W::new(self)
    }
    ///Bit 11 - Time stamp enable
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TSE_W<11> {
        TSE_W::new(self)
    }
    ///Bit 12 - Alarm A interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn alraie(&mut self) -> ALRAIE_W<12> {
        ALRAIE_W::new(self)
    }
    ///Bit 13 - Alarm B interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn alrbie(&mut self) -> ALRBIE_W<13> {
        ALRBIE_W::new(self)
    }
    ///Bit 14 - Wakeup timer interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn wutie(&mut self) -> WUTIE_W<14> {
        WUTIE_W::new(self)
    }
    ///Bit 15 - Time-stamp interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tsie(&mut self) -> TSIE_W<15> {
        TSIE_W::new(self)
    }
    ///Bit 16 - Add 1 hour (summer time change)
    #[inline(always)]
    #[must_use]
    pub fn add1h(&mut self) -> ADD1H_W<16> {
        ADD1H_W::new(self)
    }
    ///Bit 17 - Subtract 1 hour (winter time change)
    #[inline(always)]
    #[must_use]
    pub fn sub1h(&mut self) -> SUB1H_W<17> {
        SUB1H_W::new(self)
    }
    ///Bit 18 - Backup
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<18> {
        BKP_W::new(self)
    }
    ///Bit 19 - Calibration output selection
    #[inline(always)]
    #[must_use]
    pub fn cosel(&mut self) -> COSEL_W<19> {
        COSEL_W::new(self)
    }
    ///Bit 20 - Output polarity
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> POL_W<20> {
        POL_W::new(self)
    }
    ///Bits 21:22 - Output selection
    #[inline(always)]
    #[must_use]
    pub fn osel(&mut self) -> OSEL_W<21> {
        OSEL_W::new(self)
    }
    ///Bit 23 - Calibration output enable
    #[inline(always)]
    #[must_use]
    pub fn coe(&mut self) -> COE_W<23> {
        COE_W::new(self)
    }
    ///Bit 24 - timestamp on internal event enable
    #[inline(always)]
    #[must_use]
    pub fn itse(&mut self) -> ITSE_W<24> {
        ITSE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
