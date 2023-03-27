///Register `CSR` reader
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSR` writer
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LSION` reader - Internal low-speed oscillator enable
pub type LSION_R = crate::BitReader<LSION_A>;
///Internal low-speed oscillator enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSION_A {
    ///0: Oscillator OFF
    Off = 0,
    ///1: Oscillator ON
    On = 1,
}
impl From<LSION_A> for bool {
    #[inline(always)]
    fn from(variant: LSION_A) -> Self {
        variant as u8 != 0
    }
}
impl LSION_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSION_A {
        match self.bits {
            false => LSION_A::Off,
            true => LSION_A::On,
        }
    }
    ///Checks if the value of the field is `Off`
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LSION_A::Off
    }
    ///Checks if the value of the field is `On`
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LSION_A::On
    }
}
///Field `LSION` writer - Internal low-speed oscillator enable
pub type LSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, LSION_A, O>;
impl<'a, const O: u8> LSION_W<'a, O> {
    ///Oscillator OFF
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(LSION_A::Off)
    }
    ///Oscillator ON
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(LSION_A::On)
    }
}
///Field `LSIRDY` reader - Internal low-speed oscillator ready bit
pub type LSIRDY_R = crate::BitReader<LSIRDY_A>;
///Internal low-speed oscillator ready bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSIRDY_A {
    ///0: Oscillator not ready
    NotReady = 0,
    ///1: Oscillator ready
    Ready = 1,
}
impl From<LSIRDY_A> for bool {
    #[inline(always)]
    fn from(variant: LSIRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl LSIRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSIRDY_A {
        match self.bits {
            false => LSIRDY_A::NotReady,
            true => LSIRDY_A::Ready,
        }
    }
    ///Checks if the value of the field is `NotReady`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSIRDY_A::NotReady
    }
    ///Checks if the value of the field is `Ready`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSIRDY_A::Ready
    }
}
///Field `LSIRDY` writer - Internal low-speed oscillator ready bit
pub type LSIRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, LSIRDY_A, O>;
impl<'a, const O: u8> LSIRDY_W<'a, O> {
    ///Oscillator not ready
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut W {
        self.variant(LSIRDY_A::NotReady)
    }
    ///Oscillator ready
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(LSIRDY_A::Ready)
    }
}
///Field `LSEON` reader - External low-speed oscillator enable bit
pub use LSION_R as LSEON_R;
///Field `LSEON` writer - External low-speed oscillator enable bit
pub use LSION_W as LSEON_W;
///Field `LSERDY` reader - External low-speed oscillator ready bit
pub use LSIRDY_R as LSERDY_R;
///Field `LSEBYP` reader - External low-speed oscillator bypass bit
pub type LSEBYP_R = crate::BitReader<LSEBYP_A>;
///External low-speed oscillator bypass bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEBYP_A {
    ///0: LSE oscillator not bypassed
    NotBypassed = 0,
    ///1: LSE oscillator bypassed
    Bypassed = 1,
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
            false => LSEBYP_A::NotBypassed,
            true => LSEBYP_A::Bypassed,
        }
    }
    ///Checks if the value of the field is `NotBypassed`
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == LSEBYP_A::NotBypassed
    }
    ///Checks if the value of the field is `Bypassed`
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == LSEBYP_A::Bypassed
    }
}
///Field `LSEBYP` writer - External low-speed oscillator bypass bit
pub type LSEBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, LSEBYP_A, O>;
impl<'a, const O: u8> LSEBYP_W<'a, O> {
    ///LSE oscillator not bypassed
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut W {
        self.variant(LSEBYP_A::NotBypassed)
    }
    ///LSE oscillator bypassed
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(LSEBYP_A::Bypassed)
    }
}
///Field `LSEDRV` reader - LSEDRV
pub type LSEDRV_R = crate::FieldReader<u8, LSEDRV_A>;
///LSEDRV
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LSEDRV_A {
    ///0: Lowest drive
    Low = 0,
    ///1: Medium low drive
    MediumLow = 1,
    ///2: Medium high drive
    MediumHigh = 2,
    ///3: Highest drive
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
            1 => LSEDRV_A::MediumLow,
            2 => LSEDRV_A::MediumHigh,
            3 => LSEDRV_A::High,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Low`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == LSEDRV_A::Low
    }
    ///Checks if the value of the field is `MediumLow`
    #[inline(always)]
    pub fn is_medium_low(&self) -> bool {
        *self == LSEDRV_A::MediumLow
    }
    ///Checks if the value of the field is `MediumHigh`
    #[inline(always)]
    pub fn is_medium_high(&self) -> bool {
        *self == LSEDRV_A::MediumHigh
    }
    ///Checks if the value of the field is `High`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == LSEDRV_A::High
    }
}
///Field `LSEDRV` writer - LSEDRV
pub type LSEDRV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CSR_SPEC, u8, LSEDRV_A, 2, O>;
impl<'a, const O: u8> LSEDRV_W<'a, O> {
    ///Lowest drive
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(LSEDRV_A::Low)
    }
    ///Medium low drive
    #[inline(always)]
    pub fn medium_low(self) -> &'a mut W {
        self.variant(LSEDRV_A::MediumLow)
    }
    ///Medium high drive
    #[inline(always)]
    pub fn medium_high(self) -> &'a mut W {
        self.variant(LSEDRV_A::MediumHigh)
    }
    ///Highest drive
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(LSEDRV_A::High)
    }
}
///Field `CSSLSEON` reader - CSSLSEON
pub use LSION_R as CSSLSEON_R;
///Field `CSSLSEON` writer - CSSLSEON
pub use LSION_W as CSSLSEON_W;
///Field `CSSLSED` reader - CSS on LSE failure detection flag
pub type CSSLSED_R = crate::BitReader<CSSLSED_A>;
///CSS on LSE failure detection flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSLSED_A {
    ///0: No failure detected on LSE (32 kHz oscillator)
    NoFailure = 0,
    ///1: Failure detected on LSE (32 kHz oscillator)
    Failure = 1,
}
impl From<CSSLSED_A> for bool {
    #[inline(always)]
    fn from(variant: CSSLSED_A) -> Self {
        variant as u8 != 0
    }
}
impl CSSLSED_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CSSLSED_A {
        match self.bits {
            false => CSSLSED_A::NoFailure,
            true => CSSLSED_A::Failure,
        }
    }
    ///Checks if the value of the field is `NoFailure`
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == CSSLSED_A::NoFailure
    }
    ///Checks if the value of the field is `Failure`
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == CSSLSED_A::Failure
    }
}
///Field `CSSLSED` writer - CSS on LSE failure detection flag
pub type CSSLSED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, CSSLSED_A, O>;
impl<'a, const O: u8> CSSLSED_W<'a, O> {
    ///No failure detected on LSE (32 kHz oscillator)
    #[inline(always)]
    pub fn no_failure(self) -> &'a mut W {
        self.variant(CSSLSED_A::NoFailure)
    }
    ///Failure detected on LSE (32 kHz oscillator)
    #[inline(always)]
    pub fn failure(self) -> &'a mut W {
        self.variant(CSSLSED_A::Failure)
    }
}
///Field `RTCSEL` reader - RTC and LCD clock source selection bits
pub type RTCSEL_R = crate::FieldReader<u8, RTCSEL_A>;
///RTC and LCD clock source selection bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCSEL_A {
    ///0: No clock
    NoClock = 0,
    ///1: LSE oscillator clock used as RTC clock
    Lse = 1,
    ///2: LSI oscillator clock used as RTC clock
    Lsi = 2,
    ///3: HSE oscillator clock divided by a programmable prescaler (selection through the RTCPRE\[1:0\]
    ///bits in the RCC clock control register (RCC_CR)) used as the RTC clock
    Hse = 3,
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
            3 => RTCSEL_A::Hse,
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
    ///Checks if the value of the field is `Hse`
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == RTCSEL_A::Hse
    }
}
///Field `RTCSEL` writer - RTC and LCD clock source selection bits
pub type RTCSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CSR_SPEC, u8, RTCSEL_A, 2, O>;
impl<'a, const O: u8> RTCSEL_W<'a, O> {
    ///No clock
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(RTCSEL_A::NoClock)
    }
    ///LSE oscillator clock used as RTC clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(RTCSEL_A::Lse)
    }
    ///LSI oscillator clock used as RTC clock
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(RTCSEL_A::Lsi)
    }
    ///HSE oscillator clock divided by a programmable prescaler (selection through the RTCPRE\[1:0\]
    ///bits in the RCC clock control register (RCC_CR)) used as the RTC clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(RTCSEL_A::Hse)
    }
}
///Field `RTCEN` reader - RTC clock enable bit
pub type RTCEN_R = crate::BitReader<RTCEN_A>;
///RTC clock enable bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCEN_A {
    ///0: RTC clock disabled
    Disabled = 0,
    ///1: RTC clock enabled
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
///Field `RTCEN` writer - RTC clock enable bit
pub type RTCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, RTCEN_A, O>;
impl<'a, const O: u8> RTCEN_W<'a, O> {
    ///RTC clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTCEN_A::Disabled)
    }
    ///RTC clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTCEN_A::Enabled)
    }
}
///Field `RTCRST` reader - RTC software reset bit
pub type RTCRST_R = crate::BitReader<RTCRSTW_A>;
///RTC software reset bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCRSTW_A {
    ///1: Resets the RTC peripheral
    Reset = 1,
}
impl From<RTCRSTW_A> for bool {
    #[inline(always)]
    fn from(variant: RTCRSTW_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<RTCRSTW_A> {
        match self.bits {
            true => Some(RTCRSTW_A::Reset),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RTCRSTW_A::Reset
    }
}
///Field `RTCRST` writer - RTC software reset bit
pub type RTCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, RTCRSTW_A, O>;
impl<'a, const O: u8> RTCRST_W<'a, O> {
    ///Resets the RTC peripheral
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RTCRSTW_A::Reset)
    }
}
///Field `RMVF` reader - Remove reset flag
pub type RMVF_R = crate::BitReader<RMVFW_A>;
///Remove reset flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMVFW_A {
    ///1: Clears the reset flag
    Clear = 1,
}
impl From<RMVFW_A> for bool {
    #[inline(always)]
    fn from(variant: RMVFW_A) -> Self {
        variant as u8 != 0
    }
}
impl RMVF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<RMVFW_A> {
        match self.bits {
            true => Some(RMVFW_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == RMVFW_A::Clear
    }
}
///Field `RMVF` writer - Remove reset flag
pub type RMVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, RMVFW_A, O>;
impl<'a, const O: u8> RMVF_W<'a, O> {
    ///Clears the reset flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RMVFW_A::Clear)
    }
}
///Field `FWRSTF` reader - Firewall reset flag
pub type FWRSTF_R = crate::BitReader<FWRSTFR_A>;
///Firewall reset flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWRSTFR_A {
    ///0: No reset has occured
    NoReset = 0,
    ///1: A reset has occured
    Reset = 1,
}
impl From<FWRSTFR_A> for bool {
    #[inline(always)]
    fn from(variant: FWRSTFR_A) -> Self {
        variant as u8 != 0
    }
}
impl FWRSTF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FWRSTFR_A {
        match self.bits {
            false => FWRSTFR_A::NoReset,
            true => FWRSTFR_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoReset`
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == FWRSTFR_A::NoReset
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FWRSTFR_A::Reset
    }
}
///Field `FWRSTF` writer - Firewall reset flag
pub type FWRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, FWRSTFR_A, O>;
impl<'a, const O: u8> FWRSTF_W<'a, O> {
    ///No reset has occured
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut W {
        self.variant(FWRSTFR_A::NoReset)
    }
    ///A reset has occured
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FWRSTFR_A::Reset)
    }
}
///Field `OBLRSTF` reader - OBLRSTF
pub use FWRSTF_R as OBLRSTF_R;
///Field `PINRSTF` reader - PIN reset flag
pub use FWRSTF_R as PINRSTF_R;
///Field `PORRSTF` reader - POR/PDR reset flag
pub use FWRSTF_R as PORRSTF_R;
///Field `SFTRSTF` reader - Software reset flag
pub use FWRSTF_R as SFTRSTF_R;
///Field `IWDGRSTF` reader - Independent watchdog reset flag
pub use FWRSTF_R as IWDGRSTF_R;
///Field `WWDGRSTF` reader - Window watchdog reset flag
pub use FWRSTF_R as WWDGRSTF_R;
///Field `LPWRRSTF` reader - Low-power reset flag
pub use FWRSTF_R as LPWRRSTF_R;
///Field `OBLRSTF` writer - OBLRSTF
pub use FWRSTF_W as OBLRSTF_W;
///Field `PINRSTF` writer - PIN reset flag
pub use FWRSTF_W as PINRSTF_W;
///Field `PORRSTF` writer - POR/PDR reset flag
pub use FWRSTF_W as PORRSTF_W;
///Field `SFTRSTF` writer - Software reset flag
pub use FWRSTF_W as SFTRSTF_W;
///Field `IWDGRSTF` writer - Independent watchdog reset flag
pub use FWRSTF_W as IWDGRSTF_W;
///Field `WWDGRSTF` writer - Window watchdog reset flag
pub use FWRSTF_W as WWDGRSTF_W;
///Field `LPWRRSTF` writer - Low-power reset flag
pub use FWRSTF_W as LPWRRSTF_W;
impl R {
    ///Bit 0 - Internal low-speed oscillator enable
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Internal low-speed oscillator ready bit
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - External low-speed oscillator enable bit
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - External low-speed oscillator ready bit
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - External low-speed oscillator bypass bit
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:12 - LSEDRV
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 13 - CSSLSEON
    #[inline(always)]
    pub fn csslseon(&self) -> CSSLSEON_R {
        CSSLSEON_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CSS on LSE failure detection flag
    #[inline(always)]
    pub fn csslsed(&self) -> CSSLSED_R {
        CSSLSED_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:17 - RTC and LCD clock source selection bits
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - RTC clock enable bit
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - RTC software reset bit
    #[inline(always)]
    pub fn rtcrst(&self) -> RTCRST_R {
        RTCRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 23 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Firewall reset flag
    #[inline(always)]
    pub fn fwrstf(&self) -> FWRSTF_R {
        FWRSTF_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - OBLRSTF
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - PIN reset flag
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - POR/PDR reset flag
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Software reset flag
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Independent watchdog reset flag
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Window watchdog reset flag
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low-power reset flag
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Internal low-speed oscillator enable
    #[inline(always)]
    #[must_use]
    pub fn lsion(&mut self) -> LSION_W<0> {
        LSION_W::new(self)
    }
    ///Bit 1 - Internal low-speed oscillator ready bit
    #[inline(always)]
    #[must_use]
    pub fn lsirdy(&mut self) -> LSIRDY_W<1> {
        LSIRDY_W::new(self)
    }
    ///Bit 8 - External low-speed oscillator enable bit
    #[inline(always)]
    #[must_use]
    pub fn lseon(&mut self) -> LSEON_W<8> {
        LSEON_W::new(self)
    }
    ///Bit 10 - External low-speed oscillator bypass bit
    #[inline(always)]
    #[must_use]
    pub fn lsebyp(&mut self) -> LSEBYP_W<10> {
        LSEBYP_W::new(self)
    }
    ///Bits 11:12 - LSEDRV
    #[inline(always)]
    #[must_use]
    pub fn lsedrv(&mut self) -> LSEDRV_W<11> {
        LSEDRV_W::new(self)
    }
    ///Bit 13 - CSSLSEON
    #[inline(always)]
    #[must_use]
    pub fn csslseon(&mut self) -> CSSLSEON_W<13> {
        CSSLSEON_W::new(self)
    }
    ///Bit 14 - CSS on LSE failure detection flag
    #[inline(always)]
    #[must_use]
    pub fn csslsed(&mut self) -> CSSLSED_W<14> {
        CSSLSED_W::new(self)
    }
    ///Bits 16:17 - RTC and LCD clock source selection bits
    #[inline(always)]
    #[must_use]
    pub fn rtcsel(&mut self) -> RTCSEL_W<16> {
        RTCSEL_W::new(self)
    }
    ///Bit 18 - RTC clock enable bit
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RTCEN_W<18> {
        RTCEN_W::new(self)
    }
    ///Bit 19 - RTC software reset bit
    #[inline(always)]
    #[must_use]
    pub fn rtcrst(&mut self) -> RTCRST_W<19> {
        RTCRST_W::new(self)
    }
    ///Bit 23 - Remove reset flag
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<23> {
        RMVF_W::new(self)
    }
    ///Bit 24 - Firewall reset flag
    #[inline(always)]
    #[must_use]
    pub fn fwrstf(&mut self) -> FWRSTF_W<24> {
        FWRSTF_W::new(self)
    }
    ///Bit 25 - OBLRSTF
    #[inline(always)]
    #[must_use]
    pub fn oblrstf(&mut self) -> OBLRSTF_W<25> {
        OBLRSTF_W::new(self)
    }
    ///Bit 26 - PIN reset flag
    #[inline(always)]
    #[must_use]
    pub fn pinrstf(&mut self) -> PINRSTF_W<26> {
        PINRSTF_W::new(self)
    }
    ///Bit 27 - POR/PDR reset flag
    #[inline(always)]
    #[must_use]
    pub fn porrstf(&mut self) -> PORRSTF_W<27> {
        PORRSTF_W::new(self)
    }
    ///Bit 28 - Software reset flag
    #[inline(always)]
    #[must_use]
    pub fn sftrstf(&mut self) -> SFTRSTF_W<28> {
        SFTRSTF_W::new(self)
    }
    ///Bit 29 - Independent watchdog reset flag
    #[inline(always)]
    #[must_use]
    pub fn iwdgrstf(&mut self) -> IWDGRSTF_W<29> {
        IWDGRSTF_W::new(self)
    }
    ///Bit 30 - Window watchdog reset flag
    #[inline(always)]
    #[must_use]
    pub fn wwdgrstf(&mut self) -> WWDGRSTF_W<30> {
        WWDGRSTF_W::new(self)
    }
    ///Bit 31 - Low-power reset flag
    #[inline(always)]
    #[must_use]
    pub fn lpwrrstf(&mut self) -> LPWRRSTF_W<31> {
        LPWRRSTF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Control and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr](index.html) module
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr::R](R) reader structure
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csr::W](W) writer structure
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSR to value 0x0c00_0000
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c00_0000;
}
