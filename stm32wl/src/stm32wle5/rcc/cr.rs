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
///Field `MSION` reader - MSI clock enable
pub type MSION_R = crate::BitReader<MSION_A>;
///MSI clock enable
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSION_A {
    ///0: MSI oscillator off
    Disabled = 0,
    ///1: MSI oscillator on
    Enabled = 1,
}
impl From<MSION_A> for bool {
    #[inline(always)]
    fn from(variant: MSION_A) -> Self {
        variant as u8 != 0
    }
}
impl MSION_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MSION_A {
        match self.bits {
            false => MSION_A::Disabled,
            true => MSION_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSION_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSION_A::Enabled
    }
}
///Field `MSION` writer - MSI clock enable
pub type MSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, MSION_A, O>;
impl<'a, const O: u8> MSION_W<'a, O> {
    ///MSI oscillator off
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSION_A::Disabled)
    }
    ///MSI oscillator on
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSION_A::Enabled)
    }
}
///Field `MSIRDY` reader - MSI clock ready flag (After reset this bit will be read 1 once the MSI is ready)
pub type MSIRDY_R = crate::BitReader<MSIRDY_A>;
///MSI clock ready flag (After reset this bit will be read 1 once the MSI is ready)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRDY_A {
    ///0: MSI oscillator not ready
    NotReady = 0,
    ///1: MSI oscillator ready
    Ready = 1,
}
impl From<MSIRDY_A> for bool {
    #[inline(always)]
    fn from(variant: MSIRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl MSIRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MSIRDY_A {
        match self.bits {
            false => MSIRDY_A::NotReady,
            true => MSIRDY_A::Ready,
        }
    }
    ///Checks if the value of the field is `NotReady`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == MSIRDY_A::NotReady
    }
    ///Checks if the value of the field is `Ready`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == MSIRDY_A::Ready
    }
}
///Field `MSIPLLEN` reader - MSI clock PLL enable
pub type MSIPLLEN_R = crate::BitReader<MSIPLLEN_A>;
///MSI clock PLL enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIPLLEN_A {
    ///0: MSI PLL Off
    Off = 0,
    ///1: MSI PLL On
    On = 1,
}
impl From<MSIPLLEN_A> for bool {
    #[inline(always)]
    fn from(variant: MSIPLLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MSIPLLEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MSIPLLEN_A {
        match self.bits {
            false => MSIPLLEN_A::Off,
            true => MSIPLLEN_A::On,
        }
    }
    ///Checks if the value of the field is `Off`
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == MSIPLLEN_A::Off
    }
    ///Checks if the value of the field is `On`
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == MSIPLLEN_A::On
    }
}
///Field `MSIPLLEN` writer - MSI clock PLL enable
pub type MSIPLLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, MSIPLLEN_A, O>;
impl<'a, const O: u8> MSIPLLEN_W<'a, O> {
    ///MSI PLL Off
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(MSIPLLEN_A::Off)
    }
    ///MSI PLL On
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(MSIPLLEN_A::On)
    }
}
///Field `MSIRGSEL` reader - MSI range control selection
pub type MSIRGSEL_R = crate::BitReader<MSIRGSEL_A>;
///MSI range control selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRGSEL_A {
    ///0: MSI frequency range defined by MSISRANGE\[3:0\]
    ///in the RCC_CSR register
    Csr = 0,
    ///1: MSI frequency range defined by MSIRANGE\[3:0\]
    ///in the RCC_CR register
    Cr = 1,
}
impl From<MSIRGSEL_A> for bool {
    #[inline(always)]
    fn from(variant: MSIRGSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl MSIRGSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MSIRGSEL_A {
        match self.bits {
            false => MSIRGSEL_A::Csr,
            true => MSIRGSEL_A::Cr,
        }
    }
    ///Checks if the value of the field is `Csr`
    #[inline(always)]
    pub fn is_csr(&self) -> bool {
        *self == MSIRGSEL_A::Csr
    }
    ///Checks if the value of the field is `Cr`
    #[inline(always)]
    pub fn is_cr(&self) -> bool {
        *self == MSIRGSEL_A::Cr
    }
}
///Field `MSIRGSEL` writer - MSI range control selection
pub type MSIRGSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, MSIRGSEL_A, O>;
impl<'a, const O: u8> MSIRGSEL_W<'a, O> {
    ///MSI frequency range defined by MSISRANGE\[3:0\]
    ///in the RCC_CSR register
    #[inline(always)]
    pub fn csr(self) -> &'a mut W {
        self.variant(MSIRGSEL_A::Csr)
    }
    ///MSI frequency range defined by MSIRANGE\[3:0\]
    ///in the RCC_CR register
    #[inline(always)]
    pub fn cr(self) -> &'a mut W {
        self.variant(MSIRGSEL_A::Cr)
    }
}
///Field `MSIRANGE` reader - MSI clock ranges
pub type MSIRANGE_R = crate::FieldReader<u8, MSIRANGE_A>;
///MSI clock ranges
///
///Value on reset: 6
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSIRANGE_A {
    ///0: range 0 around 100 kHz
    Range100k = 0,
    ///1: range 1 around 200 kHz
    Range200k = 1,
    ///2: range 2 around 400 kHz
    Range400k = 2,
    ///3: range 3 around 800 kHz
    Range800k = 3,
    ///4: range 4 around 1 MHz
    Range1m = 4,
    ///5: range 5 around 2 MHz
    Range2m = 5,
    ///6: range 6 around 4 MHz (reset value)
    Range4m = 6,
    ///7: range 7 around 8 MHz
    Range8m = 7,
    ///8: range 8 around 16 MHz
    Range16m = 8,
    ///9: range 9 around 24 MHz
    Range24m = 9,
    ///10: range 10 around 32 MHz
    Range32m = 10,
    ///11: range 11 around 48 MHz
    Range48m = 11,
}
impl From<MSIRANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: MSIRANGE_A) -> Self {
        variant as _
    }
}
impl MSIRANGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<MSIRANGE_A> {
        match self.bits {
            0 => Some(MSIRANGE_A::Range100k),
            1 => Some(MSIRANGE_A::Range200k),
            2 => Some(MSIRANGE_A::Range400k),
            3 => Some(MSIRANGE_A::Range800k),
            4 => Some(MSIRANGE_A::Range1m),
            5 => Some(MSIRANGE_A::Range2m),
            6 => Some(MSIRANGE_A::Range4m),
            7 => Some(MSIRANGE_A::Range8m),
            8 => Some(MSIRANGE_A::Range16m),
            9 => Some(MSIRANGE_A::Range24m),
            10 => Some(MSIRANGE_A::Range32m),
            11 => Some(MSIRANGE_A::Range48m),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Range100k`
    #[inline(always)]
    pub fn is_range100k(&self) -> bool {
        *self == MSIRANGE_A::Range100k
    }
    ///Checks if the value of the field is `Range200k`
    #[inline(always)]
    pub fn is_range200k(&self) -> bool {
        *self == MSIRANGE_A::Range200k
    }
    ///Checks if the value of the field is `Range400k`
    #[inline(always)]
    pub fn is_range400k(&self) -> bool {
        *self == MSIRANGE_A::Range400k
    }
    ///Checks if the value of the field is `Range800k`
    #[inline(always)]
    pub fn is_range800k(&self) -> bool {
        *self == MSIRANGE_A::Range800k
    }
    ///Checks if the value of the field is `Range1m`
    #[inline(always)]
    pub fn is_range1m(&self) -> bool {
        *self == MSIRANGE_A::Range1m
    }
    ///Checks if the value of the field is `Range2m`
    #[inline(always)]
    pub fn is_range2m(&self) -> bool {
        *self == MSIRANGE_A::Range2m
    }
    ///Checks if the value of the field is `Range4m`
    #[inline(always)]
    pub fn is_range4m(&self) -> bool {
        *self == MSIRANGE_A::Range4m
    }
    ///Checks if the value of the field is `Range8m`
    #[inline(always)]
    pub fn is_range8m(&self) -> bool {
        *self == MSIRANGE_A::Range8m
    }
    ///Checks if the value of the field is `Range16m`
    #[inline(always)]
    pub fn is_range16m(&self) -> bool {
        *self == MSIRANGE_A::Range16m
    }
    ///Checks if the value of the field is `Range24m`
    #[inline(always)]
    pub fn is_range24m(&self) -> bool {
        *self == MSIRANGE_A::Range24m
    }
    ///Checks if the value of the field is `Range32m`
    #[inline(always)]
    pub fn is_range32m(&self) -> bool {
        *self == MSIRANGE_A::Range32m
    }
    ///Checks if the value of the field is `Range48m`
    #[inline(always)]
    pub fn is_range48m(&self) -> bool {
        *self == MSIRANGE_A::Range48m
    }
}
///Field `MSIRANGE` writer - MSI clock ranges
pub type MSIRANGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, MSIRANGE_A, 4, O>;
impl<'a, const O: u8> MSIRANGE_W<'a, O> {
    ///range 0 around 100 kHz
    #[inline(always)]
    pub fn range100k(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range100k)
    }
    ///range 1 around 200 kHz
    #[inline(always)]
    pub fn range200k(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range200k)
    }
    ///range 2 around 400 kHz
    #[inline(always)]
    pub fn range400k(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range400k)
    }
    ///range 3 around 800 kHz
    #[inline(always)]
    pub fn range800k(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range800k)
    }
    ///range 4 around 1 MHz
    #[inline(always)]
    pub fn range1m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range1m)
    }
    ///range 5 around 2 MHz
    #[inline(always)]
    pub fn range2m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range2m)
    }
    ///range 6 around 4 MHz (reset value)
    #[inline(always)]
    pub fn range4m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range4m)
    }
    ///range 7 around 8 MHz
    #[inline(always)]
    pub fn range8m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range8m)
    }
    ///range 8 around 16 MHz
    #[inline(always)]
    pub fn range16m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range16m)
    }
    ///range 9 around 24 MHz
    #[inline(always)]
    pub fn range24m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range24m)
    }
    ///range 10 around 32 MHz
    #[inline(always)]
    pub fn range32m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range32m)
    }
    ///range 11 around 48 MHz
    #[inline(always)]
    pub fn range48m(self) -> &'a mut W {
        self.variant(MSIRANGE_A::Range48m)
    }
}
///Field `HSION` reader - HSI16 clock enable
pub type HSION_R = crate::BitReader<HSION_A>;
///HSI16 clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSION_A {
    ///0: HSI16 oscillator off
    Disabled = 0,
    ///1: HSI16 oscillator on
    Enabled = 1,
}
impl From<HSION_A> for bool {
    #[inline(always)]
    fn from(variant: HSION_A) -> Self {
        variant as u8 != 0
    }
}
impl HSION_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSION_A {
        match self.bits {
            false => HSION_A::Disabled,
            true => HSION_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSION_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSION_A::Enabled
    }
}
///Field `HSION` writer - HSI16 clock enable
pub type HSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSION_A, O>;
impl<'a, const O: u8> HSION_W<'a, O> {
    ///HSI16 oscillator off
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSION_A::Disabled)
    }
    ///HSI16 oscillator on
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSION_A::Enabled)
    }
}
///Field `HSIKERON` reader - HSI16 always enable for peripheral kernel clocks.
pub type HSIKERON_R = crate::BitReader<HSIKERON_A>;
///HSI16 always enable for peripheral kernel clocks.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIKERON_A {
    ///0: No effect on HSI16 oscillator
    NotForced = 0,
    ///1: HSI16 oscillator forced on even in Stop modes
    Forced = 1,
}
impl From<HSIKERON_A> for bool {
    #[inline(always)]
    fn from(variant: HSIKERON_A) -> Self {
        variant as u8 != 0
    }
}
impl HSIKERON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSIKERON_A {
        match self.bits {
            false => HSIKERON_A::NotForced,
            true => HSIKERON_A::Forced,
        }
    }
    ///Checks if the value of the field is `NotForced`
    #[inline(always)]
    pub fn is_not_forced(&self) -> bool {
        *self == HSIKERON_A::NotForced
    }
    ///Checks if the value of the field is `Forced`
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == HSIKERON_A::Forced
    }
}
///Field `HSIKERON` writer - HSI16 always enable for peripheral kernel clocks.
pub type HSIKERON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSIKERON_A, O>;
impl<'a, const O: u8> HSIKERON_W<'a, O> {
    ///No effect on HSI16 oscillator
    #[inline(always)]
    pub fn not_forced(self) -> &'a mut W {
        self.variant(HSIKERON_A::NotForced)
    }
    ///HSI16 oscillator forced on even in Stop modes
    #[inline(always)]
    pub fn forced(self) -> &'a mut W {
        self.variant(HSIKERON_A::Forced)
    }
}
///Field `HSIRDY` reader - HSI16 clock ready flag. (After wakeup from Stop this bit will be read 1 once the HSI16 is ready)
pub type HSIRDY_R = crate::BitReader<HSIRDY_A>;
///HSI16 clock ready flag. (After wakeup from Stop this bit will be read 1 once the HSI16 is ready)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIRDY_A {
    ///0: HSI16 oscillator not ready
    NotReady = 0,
    ///1: HSI16 oscillator ready
    Ready = 1,
}
impl From<HSIRDY_A> for bool {
    #[inline(always)]
    fn from(variant: HSIRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl HSIRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSIRDY_A {
        match self.bits {
            false => HSIRDY_A::NotReady,
            true => HSIRDY_A::Ready,
        }
    }
    ///Checks if the value of the field is `NotReady`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSIRDY_A::NotReady
    }
    ///Checks if the value of the field is `Ready`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSIRDY_A::Ready
    }
}
///Field `HSIASFS` reader - HSI16 automatic start from Stop
pub type HSIASFS_R = crate::BitReader<HSIASFS_A>;
///HSI16 automatic start from Stop
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIASFS_A {
    ///0: HSI16 not enabled by hardware when exiting Stop modes with MSI as wakeup clock
    Disabled = 0,
    ///1: HSI16 enabled by hardware when exiting Stop mode with MSI as wakeup clock
    Enabled = 1,
}
impl From<HSIASFS_A> for bool {
    #[inline(always)]
    fn from(variant: HSIASFS_A) -> Self {
        variant as u8 != 0
    }
}
impl HSIASFS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSIASFS_A {
        match self.bits {
            false => HSIASFS_A::Disabled,
            true => HSIASFS_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSIASFS_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSIASFS_A::Enabled
    }
}
///Field `HSIASFS` writer - HSI16 automatic start from Stop
pub type HSIASFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSIASFS_A, O>;
impl<'a, const O: u8> HSIASFS_W<'a, O> {
    ///HSI16 not enabled by hardware when exiting Stop modes with MSI as wakeup clock
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSIASFS_A::Disabled)
    }
    ///HSI16 enabled by hardware when exiting Stop mode with MSI as wakeup clock
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSIASFS_A::Enabled)
    }
}
///Field `HSIKERDY` reader - HSI16 kernel clock ready flag for peripherals requests.
pub type HSIKERDY_R = crate::BitReader<HSIKERDY_A>;
///HSI16 kernel clock ready flag for peripherals requests.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIKERDY_A {
    ///0: HSI16 oscillator not ready
    NotReady = 0,
    ///1: HSI16 oscillator ready
    Ready = 1,
}
impl From<HSIKERDY_A> for bool {
    #[inline(always)]
    fn from(variant: HSIKERDY_A) -> Self {
        variant as u8 != 0
    }
}
impl HSIKERDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSIKERDY_A {
        match self.bits {
            false => HSIKERDY_A::NotReady,
            true => HSIKERDY_A::Ready,
        }
    }
    ///Checks if the value of the field is `NotReady`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSIKERDY_A::NotReady
    }
    ///Checks if the value of the field is `Ready`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSIKERDY_A::Ready
    }
}
///Field `HSEON` reader - HSE32 clock enable
pub type HSEON_R = crate::BitReader<HSEON_A>;
///HSE32 clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEON_A {
    ///0: HSE32 oscillator for CPU disabled
    Disabled = 0,
    ///1: HSE32 oscillator for CPU enabled
    Enabled = 1,
}
impl From<HSEON_A> for bool {
    #[inline(always)]
    fn from(variant: HSEON_A) -> Self {
        variant as u8 != 0
    }
}
impl HSEON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSEON_A {
        match self.bits {
            false => HSEON_A::Disabled,
            true => HSEON_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSEON_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSEON_A::Enabled
    }
}
///Field `HSEON` writer - HSE32 clock enable
pub type HSEON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSEON_A, O>;
impl<'a, const O: u8> HSEON_W<'a, O> {
    ///HSE32 oscillator for CPU disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSEON_A::Disabled)
    }
    ///HSE32 oscillator for CPU enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSEON_A::Enabled)
    }
}
///Field `HSERDY` reader - HSE32 clock ready flag
pub type HSERDY_R = crate::BitReader<HSERDY_A>;
///HSE32 clock ready flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDY_A {
    ///0: HSE32 oscillator not ready
    NotReady = 0,
    ///1: HSE32 oscillator ready
    Ready = 1,
}
impl From<HSERDY_A> for bool {
    #[inline(always)]
    fn from(variant: HSERDY_A) -> Self {
        variant as u8 != 0
    }
}
impl HSERDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSERDY_A {
        match self.bits {
            false => HSERDY_A::NotReady,
            true => HSERDY_A::Ready,
        }
    }
    ///Checks if the value of the field is `NotReady`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HSERDY_A::NotReady
    }
    ///Checks if the value of the field is `Ready`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HSERDY_A::Ready
    }
}
///Field `CSSON` reader - HSE32 Clock security system enable
pub type CSSON_R = crate::BitReader<CSSON_A>;
///HSE32 Clock security system enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSON_A {
    ///0: HSE32 CSS off
    Disabled = 0,
    ///1: HSE32 CSS on if the HSE32 oscillator is stable and off if not
    Enabled = 1,
}
impl From<CSSON_A> for bool {
    #[inline(always)]
    fn from(variant: CSSON_A) -> Self {
        variant as u8 != 0
    }
}
impl CSSON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CSSON_A {
        match self.bits {
            false => CSSON_A::Disabled,
            true => CSSON_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CSSON_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CSSON_A::Enabled
    }
}
///Field `CSSON` writer - HSE32 Clock security system enable
pub type CSSON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CSSON_A, O>;
impl<'a, const O: u8> CSSON_W<'a, O> {
    ///HSE32 CSS off
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CSSON_A::Disabled)
    }
    ///HSE32 CSS on if the HSE32 oscillator is stable and off if not
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CSSON_A::Enabled)
    }
}
///Field `HSEPRE` reader - HSE32 sysclk prescaler
pub type HSEPRE_R = crate::BitReader<HSEPRE_A>;
///HSE32 sysclk prescaler
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEPRE_A {
    ///0: SYSCLK not divided (HSE32)
    Div1 = 0,
    ///1: SYSCLK divided by two (HSE32/2)
    Div2 = 1,
}
impl From<HSEPRE_A> for bool {
    #[inline(always)]
    fn from(variant: HSEPRE_A) -> Self {
        variant as u8 != 0
    }
}
impl HSEPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSEPRE_A {
        match self.bits {
            false => HSEPRE_A::Div1,
            true => HSEPRE_A::Div2,
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HSEPRE_A::Div1
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HSEPRE_A::Div2
    }
}
///Field `HSEPRE` writer - HSE32 sysclk prescaler
pub type HSEPRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSEPRE_A, O>;
impl<'a, const O: u8> HSEPRE_W<'a, O> {
    ///SYSCLK not divided (HSE32)
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(HSEPRE_A::Div1)
    }
    ///SYSCLK divided by two (HSE32/2)
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HSEPRE_A::Div2)
    }
}
///Field `HSEBYPPWR` reader - Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO.
pub type HSEBYPPWR_R = crate::BitReader<HSEBYPPWR_A>;
///Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEBYPPWR_A {
    ///0: PB0 selected
    Pb0 = 0,
    ///1: VDDTCXO selected
    Vddtcxo = 1,
}
impl From<HSEBYPPWR_A> for bool {
    #[inline(always)]
    fn from(variant: HSEBYPPWR_A) -> Self {
        variant as u8 != 0
    }
}
impl HSEBYPPWR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSEBYPPWR_A {
        match self.bits {
            false => HSEBYPPWR_A::Pb0,
            true => HSEBYPPWR_A::Vddtcxo,
        }
    }
    ///Checks if the value of the field is `Pb0`
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == HSEBYPPWR_A::Pb0
    }
    ///Checks if the value of the field is `Vddtcxo`
    #[inline(always)]
    pub fn is_vddtcxo(&self) -> bool {
        *self == HSEBYPPWR_A::Vddtcxo
    }
}
///Field `HSEBYPPWR` writer - Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO.
pub type HSEBYPPWR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSEBYPPWR_A, O>;
impl<'a, const O: u8> HSEBYPPWR_W<'a, O> {
    ///PB0 selected
    #[inline(always)]
    pub fn pb0(self) -> &'a mut W {
        self.variant(HSEBYPPWR_A::Pb0)
    }
    ///VDDTCXO selected
    #[inline(always)]
    pub fn vddtcxo(self) -> &'a mut W {
        self.variant(HSEBYPPWR_A::Vddtcxo)
    }
}
///Field `PLLON` reader - Main PLL enable
pub type PLLON_R = crate::BitReader<PLLON_A>;
///Main PLL enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLON_A {
    ///0: Main PLL Off
    Off = 0,
    ///1: Main PLL On
    On = 1,
}
impl From<PLLON_A> for bool {
    #[inline(always)]
    fn from(variant: PLLON_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLON_A {
        match self.bits {
            false => PLLON_A::Off,
            true => PLLON_A::On,
        }
    }
    ///Checks if the value of the field is `Off`
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PLLON_A::Off
    }
    ///Checks if the value of the field is `On`
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == PLLON_A::On
    }
}
///Field `PLLON` writer - Main PLL enable
pub type PLLON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, PLLON_A, O>;
impl<'a, const O: u8> PLLON_W<'a, O> {
    ///Main PLL Off
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PLLON_A::Off)
    }
    ///Main PLL On
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(PLLON_A::On)
    }
}
///Field `PLLRDY` reader - Main PLL clock ready flag
pub type PLLRDY_R = crate::BitReader<PLLRDY_A>;
///Main PLL clock ready flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLRDY_A {
    ///0: PLL unlocked
    Unlocked = 0,
    ///1: PLL Locked
    Locked = 1,
}
impl From<PLLRDY_A> for bool {
    #[inline(always)]
    fn from(variant: PLLRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLRDY_A {
        match self.bits {
            false => PLLRDY_A::Unlocked,
            true => PLLRDY_A::Locked,
        }
    }
    ///Checks if the value of the field is `Unlocked`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == PLLRDY_A::Unlocked
    }
    ///Checks if the value of the field is `Locked`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == PLLRDY_A::Locked
    }
}
impl R {
    ///Bit 0 - MSI clock enable
    #[inline(always)]
    pub fn msion(&self) -> MSION_R {
        MSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MSI clock ready flag (After reset this bit will be read 1 once the MSI is ready)
    #[inline(always)]
    pub fn msirdy(&self) -> MSIRDY_R {
        MSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSI clock PLL enable
    #[inline(always)]
    pub fn msipllen(&self) -> MSIPLLEN_R {
        MSIPLLEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MSI range control selection
    #[inline(always)]
    pub fn msirgsel(&self) -> MSIRGSEL_R {
        MSIRGSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:7 - MSI clock ranges
    #[inline(always)]
    pub fn msirange(&self) -> MSIRANGE_R {
        MSIRANGE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - HSI16 clock enable
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HSI16 always enable for peripheral kernel clocks.
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSI16 clock ready flag. (After wakeup from Stop this bit will be read 1 once the HSI16 is ready)
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - HSI16 automatic start from Stop
    #[inline(always)]
    pub fn hsiasfs(&self) -> HSIASFS_R {
        HSIASFS_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - HSI16 kernel clock ready flag for peripherals requests.
    #[inline(always)]
    pub fn hsikerdy(&self) -> HSIKERDY_R {
        HSIKERDY_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - HSE32 clock enable
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HSE32 clock ready flag
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - HSE32 Clock security system enable
    #[inline(always)]
    pub fn csson(&self) -> CSSON_R {
        CSSON_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - HSE32 sysclk prescaler
    #[inline(always)]
    pub fn hsepre(&self) -> HSEPRE_R {
        HSEPRE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO.
    #[inline(always)]
    pub fn hsebyppwr(&self) -> HSEBYPPWR_R {
        HSEBYPPWR_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - Main PLL enable
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Main PLL clock ready flag
    #[inline(always)]
    pub fn pllrdy(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - MSI clock enable
    #[inline(always)]
    #[must_use]
    pub fn msion(&mut self) -> MSION_W<0> {
        MSION_W::new(self)
    }
    ///Bit 2 - MSI clock PLL enable
    #[inline(always)]
    #[must_use]
    pub fn msipllen(&mut self) -> MSIPLLEN_W<2> {
        MSIPLLEN_W::new(self)
    }
    ///Bit 3 - MSI range control selection
    #[inline(always)]
    #[must_use]
    pub fn msirgsel(&mut self) -> MSIRGSEL_W<3> {
        MSIRGSEL_W::new(self)
    }
    ///Bits 4:7 - MSI clock ranges
    #[inline(always)]
    #[must_use]
    pub fn msirange(&mut self) -> MSIRANGE_W<4> {
        MSIRANGE_W::new(self)
    }
    ///Bit 8 - HSI16 clock enable
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HSION_W<8> {
        HSION_W::new(self)
    }
    ///Bit 9 - HSI16 always enable for peripheral kernel clocks.
    #[inline(always)]
    #[must_use]
    pub fn hsikeron(&mut self) -> HSIKERON_W<9> {
        HSIKERON_W::new(self)
    }
    ///Bit 11 - HSI16 automatic start from Stop
    #[inline(always)]
    #[must_use]
    pub fn hsiasfs(&mut self) -> HSIASFS_W<11> {
        HSIASFS_W::new(self)
    }
    ///Bit 16 - HSE32 clock enable
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HSEON_W<16> {
        HSEON_W::new(self)
    }
    ///Bit 19 - HSE32 Clock security system enable
    #[inline(always)]
    #[must_use]
    pub fn csson(&mut self) -> CSSON_W<19> {
        CSSON_W::new(self)
    }
    ///Bit 20 - HSE32 sysclk prescaler
    #[inline(always)]
    #[must_use]
    pub fn hsepre(&mut self) -> HSEPRE_W<20> {
        HSEPRE_W::new(self)
    }
    ///Bit 21 - Enable HSE32 VDDTCXO output on package pin PB0-VDDTCXO.
    #[inline(always)]
    #[must_use]
    pub fn hsebyppwr(&mut self) -> HSEBYPPWR_W<21> {
        HSEBYPPWR_W::new(self)
    }
    ///Bit 24 - Main PLL enable
    #[inline(always)]
    #[must_use]
    pub fn pllon(&mut self) -> PLLON_W<24> {
        PLLON_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Clock control register
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
///`reset()` method sets CR to value 0x61
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x61;
}
