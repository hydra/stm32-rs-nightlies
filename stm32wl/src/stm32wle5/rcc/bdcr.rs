///Register `BDCR` reader
pub struct R(crate::R<BDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BDCR` writer
pub struct W(crate::W<BDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDCR_SPEC>;
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
impl From<crate::W<BDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LSEON` reader - LSE oscillator enable
pub type LSEON_R = crate::BitReader<LSEON_A>;
///LSE oscillator enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEON_A {
    ///0: LSE oscillator off
    Off = 0,
    ///1: LSE oscillator on
    On = 1,
}
impl From<LSEON_A> for bool {
    #[inline(always)]
    fn from(variant: LSEON_A) -> Self {
        variant as u8 != 0
    }
}
impl LSEON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSEON_A {
        match self.bits {
            false => LSEON_A::Off,
            true => LSEON_A::On,
        }
    }
    ///Checks if the value of the field is `Off`
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LSEON_A::Off
    }
    ///Checks if the value of the field is `On`
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LSEON_A::On
    }
}
///Field `LSEON` writer - LSE oscillator enable
pub type LSEON_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, LSEON_A, O>;
impl<'a, const O: u8> LSEON_W<'a, O> {
    ///LSE oscillator off
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(LSEON_A::Off)
    }
    ///LSE oscillator on
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(LSEON_A::On)
    }
}
///Field `LSERDY` reader - LSE oscillator ready
pub type LSERDY_R = crate::BitReader<LSERDY_A>;
///LSE oscillator ready
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDY_A {
    ///0: LSE oscillator not ready
    NotReady = 0,
    ///1: LSE oscillator ready
    Ready = 1,
}
impl From<LSERDY_A> for bool {
    #[inline(always)]
    fn from(variant: LSERDY_A) -> Self {
        variant as u8 != 0
    }
}
impl LSERDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSERDY_A {
        match self.bits {
            false => LSERDY_A::NotReady,
            true => LSERDY_A::Ready,
        }
    }
    ///Checks if the value of the field is `NotReady`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSERDY_A::NotReady
    }
    ///Checks if the value of the field is `Ready`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSERDY_A::Ready
    }
}
///Field `LSEBYP` reader - LSE oscillator bypass
pub type LSEBYP_R = crate::BitReader<LSEBYP_A>;
///LSE oscillator bypass
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEBYP_A {
    ///0: LSE oscillator not bypassed
    Disabled = 0,
    ///1: LSE oscillator bypassed
    Enabled = 1,
}
impl From<LSEBYP_A> for bool {
    #[inline(always)]
    fn from(variant: LSEBYP_A) -> Self {
        variant as u8 != 0
    }
}
impl LSEBYP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSEBYP_A {
        match self.bits {
            false => LSEBYP_A::Disabled,
            true => LSEBYP_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSEBYP_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSEBYP_A::Enabled
    }
}
///Field `LSEBYP` writer - LSE oscillator bypass
pub type LSEBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, LSEBYP_A, O>;
impl<'a, const O: u8> LSEBYP_W<'a, O> {
    ///LSE oscillator not bypassed
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSEBYP_A::Disabled)
    }
    ///LSE oscillator bypassed
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSEBYP_A::Enabled)
    }
}
///Field `LSEDRV` reader - LSE oscillator drive capability
pub type LSEDRV_R = crate::FieldReader<u8, LSEDRV_A>;
///LSE oscillator drive capability
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LSEDRV_A {
    ///0: Xtal mode lower driving capability
    Low = 0,
    ///1: Xtal mode medium-low driving capability
    MedLow = 1,
    ///2: Xtal mode medium-high driving capability
    MedHigh = 2,
    ///3: Xtal mode higher driving capability
    High = 3,
}
impl From<LSEDRV_A> for u8 {
    #[inline(always)]
    fn from(variant: LSEDRV_A) -> Self {
        variant as _
    }
}
impl LSEDRV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSEDRV_A {
        match self.bits {
            0 => LSEDRV_A::Low,
            1 => LSEDRV_A::MedLow,
            2 => LSEDRV_A::MedHigh,
            3 => LSEDRV_A::High,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Low`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == LSEDRV_A::Low
    }
    ///Checks if the value of the field is `MedLow`
    #[inline(always)]
    pub fn is_med_low(&self) -> bool {
        *self == LSEDRV_A::MedLow
    }
    ///Checks if the value of the field is `MedHigh`
    #[inline(always)]
    pub fn is_med_high(&self) -> bool {
        *self == LSEDRV_A::MedHigh
    }
    ///Checks if the value of the field is `High`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == LSEDRV_A::High
    }
}
///Field `LSEDRV` writer - LSE oscillator drive capability
pub type LSEDRV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BDCR_SPEC, u8, LSEDRV_A, 2, O>;
impl<'a, const O: u8> LSEDRV_W<'a, O> {
    ///Xtal mode lower driving capability
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(LSEDRV_A::Low)
    }
    ///Xtal mode medium-low driving capability
    #[inline(always)]
    pub fn med_low(self) -> &'a mut W {
        self.variant(LSEDRV_A::MedLow)
    }
    ///Xtal mode medium-high driving capability
    #[inline(always)]
    pub fn med_high(self) -> &'a mut W {
        self.variant(LSEDRV_A::MedHigh)
    }
    ///Xtal mode higher driving capability
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(LSEDRV_A::High)
    }
}
///Field `LSECSSON` reader - CSS on LSE enable
pub type LSECSSON_R = crate::BitReader<LSECSSON_A>;
///CSS on LSE enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSON_A {
    ///0: CSS on LSE disabled
    Disabled = 0,
    ///1: CSS on LSE enabled
    Enabled = 1,
}
impl From<LSECSSON_A> for bool {
    #[inline(always)]
    fn from(variant: LSECSSON_A) -> Self {
        variant as u8 != 0
    }
}
impl LSECSSON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSECSSON_A {
        match self.bits {
            false => LSECSSON_A::Disabled,
            true => LSECSSON_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSECSSON_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSECSSON_A::Enabled
    }
}
///Field `LSECSSON` writer - CSS on LSE enable
pub type LSECSSON_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, LSECSSON_A, O>;
impl<'a, const O: u8> LSECSSON_W<'a, O> {
    ///CSS on LSE disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSECSSON_A::Disabled)
    }
    ///CSS on LSE enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSECSSON_A::Enabled)
    }
}
///Field `LSECSSD` reader - CSS on LSE failure Detection
pub type LSECSSD_R = crate::BitReader<LSECSSD_A>;
///CSS on LSE failure Detection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSECSSD_A {
    ///0: No failure detected on LSE
    NoFailure = 0,
    ///1: Failure detected on LSE
    Failure = 1,
}
impl From<LSECSSD_A> for bool {
    #[inline(always)]
    fn from(variant: LSECSSD_A) -> Self {
        variant as u8 != 0
    }
}
impl LSECSSD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSECSSD_A {
        match self.bits {
            false => LSECSSD_A::NoFailure,
            true => LSECSSD_A::Failure,
        }
    }
    ///Checks if the value of the field is `NoFailure`
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == LSECSSD_A::NoFailure
    }
    ///Checks if the value of the field is `Failure`
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == LSECSSD_A::Failure
    }
}
///Field `LSESYSEN` reader - LSE system clock enable
pub type LSESYSEN_R = crate::BitReader<LSESYSEN_A>;
///LSE system clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSESYSEN_A {
    ///0: LSE system clock disabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode
    Disabled = 0,
    ///1: LSE system clock enabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode
    Enabled = 1,
}
impl From<LSESYSEN_A> for bool {
    #[inline(always)]
    fn from(variant: LSESYSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LSESYSEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSESYSEN_A {
        match self.bits {
            false => LSESYSEN_A::Disabled,
            true => LSESYSEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSESYSEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSESYSEN_A::Enabled
    }
}
///Field `LSESYSEN` writer - LSE system clock enable
pub type LSESYSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, LSESYSEN_A, O>;
impl<'a, const O: u8> LSESYSEN_W<'a, O> {
    ///LSE system clock disabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSESYSEN_A::Disabled)
    }
    ///LSE system clock enabled to USARTx, LPUARTx, LPTIMx, TIMx, RNG, system LSCO, MCO, MSI PLL mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSESYSEN_A::Enabled)
    }
}
///Field `RTCSEL` reader - RTC clock source selection
pub type RTCSEL_R = crate::FieldReader<u8, RTCSEL_A>;
///RTC clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCSEL_A {
    ///0: No clock
    NoClock = 0,
    ///1: LSE oscillator clock selected
    Lse = 1,
    ///2: LSI oscillator clock selected
    Lsi = 2,
    ///3: HSE32 oscillator clock divided by 32 selected
    Hse32 = 3,
}
impl From<RTCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSEL_A) -> Self {
        variant as _
    }
}
impl RTCSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RTCSEL_A {
        match self.bits {
            0 => RTCSEL_A::NoClock,
            1 => RTCSEL_A::Lse,
            2 => RTCSEL_A::Lsi,
            3 => RTCSEL_A::Hse32,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NoClock`
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == RTCSEL_A::NoClock
    }
    ///Checks if the value of the field is `Lse`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == RTCSEL_A::Lse
    }
    ///Checks if the value of the field is `Lsi`
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == RTCSEL_A::Lsi
    }
    ///Checks if the value of the field is `Hse32`
    #[inline(always)]
    pub fn is_hse32(&self) -> bool {
        *self == RTCSEL_A::Hse32
    }
}
///Field `RTCSEL` writer - RTC clock source selection
pub type RTCSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BDCR_SPEC, u8, RTCSEL_A, 2, O>;
impl<'a, const O: u8> RTCSEL_W<'a, O> {
    ///No clock
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(RTCSEL_A::NoClock)
    }
    ///LSE oscillator clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(RTCSEL_A::Lse)
    }
    ///LSI oscillator clock selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(RTCSEL_A::Lsi)
    }
    ///HSE32 oscillator clock divided by 32 selected
    #[inline(always)]
    pub fn hse32(self) -> &'a mut W {
        self.variant(RTCSEL_A::Hse32)
    }
}
///Field `LSESYSRDY` reader - LSE system clock ready
pub type LSESYSRDY_R = crate::BitReader<LSESYSRDY_A>;
///LSE system clock ready
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSESYSRDY_A {
    ///0: LSE system clock not ready
    NotReady = 0,
    ///1: LSE system clock ready
    Ready = 1,
}
impl From<LSESYSRDY_A> for bool {
    #[inline(always)]
    fn from(variant: LSESYSRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl LSESYSRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSESYSRDY_A {
        match self.bits {
            false => LSESYSRDY_A::NotReady,
            true => LSESYSRDY_A::Ready,
        }
    }
    ///Checks if the value of the field is `NotReady`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSESYSRDY_A::NotReady
    }
    ///Checks if the value of the field is `Ready`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSESYSRDY_A::Ready
    }
}
///Field `RTCEN` reader - RTC clock enable
pub type RTCEN_R = crate::BitReader<RTCEN_A>;
///RTC clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCEN_A {
    ///0: RTC kernel clock disabled
    Disabled = 0,
    ///1: RTC kernel clock enabled
    Enabled = 1,
}
impl From<RTCEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RTCEN_A {
        match self.bits {
            false => RTCEN_A::Disabled,
            true => RTCEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTCEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTCEN_A::Enabled
    }
}
///Field `RTCEN` writer - RTC clock enable
pub type RTCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, RTCEN_A, O>;
impl<'a, const O: u8> RTCEN_W<'a, O> {
    ///RTC kernel clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTCEN_A::Disabled)
    }
    ///RTC kernel clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTCEN_A::Enabled)
    }
}
///Field `BDRST` reader - Backup domain software reset
pub type BDRST_R = crate::BitReader<BDRST_A>;
///Backup domain software reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDRST_A {
    ///0: Reset not activated
    NotActive = 0,
    ///1: Entire Backup domain reset
    Reset = 1,
}
impl From<BDRST_A> for bool {
    #[inline(always)]
    fn from(variant: BDRST_A) -> Self {
        variant as u8 != 0
    }
}
impl BDRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BDRST_A {
        match self.bits {
            false => BDRST_A::NotActive,
            true => BDRST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NotActive`
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == BDRST_A::NotActive
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BDRST_A::Reset
    }
}
///Field `BDRST` writer - Backup domain software reset
pub type BDRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, BDRST_A, O>;
impl<'a, const O: u8> BDRST_W<'a, O> {
    ///Reset not activated
    #[inline(always)]
    pub fn not_active(self) -> &'a mut W {
        self.variant(BDRST_A::NotActive)
    }
    ///Entire Backup domain reset
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BDRST_A::Reset)
    }
}
///Field `LSCOEN` reader - Low speed clock output enable
pub type LSCOEN_R = crate::BitReader<LSCOEN_A>;
///Low speed clock output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSCOEN_A {
    ///0: LSCO disabled
    Disabled = 0,
    ///1: LSCO enabled
    Enabled = 1,
}
impl From<LSCOEN_A> for bool {
    #[inline(always)]
    fn from(variant: LSCOEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LSCOEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSCOEN_A {
        match self.bits {
            false => LSCOEN_A::Disabled,
            true => LSCOEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LSCOEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LSCOEN_A::Enabled
    }
}
///Field `LSCOEN` writer - Low speed clock output enable
pub type LSCOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, LSCOEN_A, O>;
impl<'a, const O: u8> LSCOEN_W<'a, O> {
    ///LSCO disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LSCOEN_A::Disabled)
    }
    ///LSCO enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LSCOEN_A::Enabled)
    }
}
///Field `LSCOSEL` reader - Low speed clock output selection
pub type LSCOSEL_R = crate::BitReader<LSCOSEL_A>;
///Low speed clock output selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSCOSEL_A {
    ///0: LSI clock selected
    Lsi = 0,
    ///1: LSE clock selected
    Lse = 1,
}
impl From<LSCOSEL_A> for bool {
    #[inline(always)]
    fn from(variant: LSCOSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl LSCOSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSCOSEL_A {
        match self.bits {
            false => LSCOSEL_A::Lsi,
            true => LSCOSEL_A::Lse,
        }
    }
    ///Checks if the value of the field is `Lsi`
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LSCOSEL_A::Lsi
    }
    ///Checks if the value of the field is `Lse`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LSCOSEL_A::Lse
    }
}
///Field `LSCOSEL` writer - Low speed clock output selection
pub type LSCOSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, LSCOSEL_A, O>;
impl<'a, const O: u8> LSCOSEL_W<'a, O> {
    ///LSI clock selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(LSCOSEL_A::Lsi)
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LSCOSEL_A::Lse)
    }
}
impl R {
    ///Bit 0 - LSE oscillator enable
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE oscillator ready
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LSE oscillator bypass
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - LSE oscillator drive capability
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - CSS on LSE enable
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CSS on LSE failure Detection
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - LSE system clock enable
    #[inline(always)]
    pub fn lsesysen(&self) -> LSESYSEN_R {
        LSESYSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - RTC clock source selection
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 11 - LSE system clock ready
    #[inline(always)]
    pub fn lsesysrdy(&self) -> LSESYSRDY_R {
        LSESYSRDY_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - RTC clock enable
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Backup domain software reset
    #[inline(always)]
    pub fn bdrst(&self) -> BDRST_R {
        BDRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Low speed clock output enable
    #[inline(always)]
    pub fn lscoen(&self) -> LSCOEN_R {
        LSCOEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Low speed clock output selection
    #[inline(always)]
    pub fn lscosel(&self) -> LSCOSEL_R {
        LSCOSEL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LSE oscillator enable
    #[inline(always)]
    #[must_use]
    pub fn lseon(&mut self) -> LSEON_W<0> {
        LSEON_W::new(self)
    }
    ///Bit 2 - LSE oscillator bypass
    #[inline(always)]
    #[must_use]
    pub fn lsebyp(&mut self) -> LSEBYP_W<2> {
        LSEBYP_W::new(self)
    }
    ///Bits 3:4 - LSE oscillator drive capability
    #[inline(always)]
    #[must_use]
    pub fn lsedrv(&mut self) -> LSEDRV_W<3> {
        LSEDRV_W::new(self)
    }
    ///Bit 5 - CSS on LSE enable
    #[inline(always)]
    #[must_use]
    pub fn lsecsson(&mut self) -> LSECSSON_W<5> {
        LSECSSON_W::new(self)
    }
    ///Bit 7 - LSE system clock enable
    #[inline(always)]
    #[must_use]
    pub fn lsesysen(&mut self) -> LSESYSEN_W<7> {
        LSESYSEN_W::new(self)
    }
    ///Bits 8:9 - RTC clock source selection
    #[inline(always)]
    #[must_use]
    pub fn rtcsel(&mut self) -> RTCSEL_W<8> {
        RTCSEL_W::new(self)
    }
    ///Bit 15 - RTC clock enable
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RTCEN_W<15> {
        RTCEN_W::new(self)
    }
    ///Bit 16 - Backup domain software reset
    #[inline(always)]
    #[must_use]
    pub fn bdrst(&mut self) -> BDRST_W<16> {
        BDRST_W::new(self)
    }
    ///Bit 24 - Low speed clock output enable
    #[inline(always)]
    #[must_use]
    pub fn lscoen(&mut self) -> LSCOEN_W<24> {
        LSCOEN_W::new(self)
    }
    ///Bit 25 - Low speed clock output selection
    #[inline(always)]
    #[must_use]
    pub fn lscosel(&mut self) -> LSCOSEL_W<25> {
        LSCOSEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Backup domain control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bdcr](index.html) module
pub struct BDCR_SPEC;
impl crate::RegisterSpec for BDCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [bdcr::R](R) reader structure
impl crate::Readable for BDCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bdcr::W](W) writer structure
impl crate::Writable for BDCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BDCR to value 0
impl crate::Resettable for BDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
