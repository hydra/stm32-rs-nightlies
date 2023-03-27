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
    ///0: MSI oscillator OFF
    Disabled = 0,
    ///1: MSI oscillator ON
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
    ///MSI oscillator OFF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSION_A::Disabled)
    }
    ///MSI oscillator ON
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSION_A::Enabled)
    }
}
///Field `MSIRDY` reader - MSI clock ready flag
pub type MSIRDY_R = crate::BitReader<MSIRDY_A>;
///MSI clock ready flag
///
///Value on reset: 1
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
    ///0: MSI PLL OFF
    Disabled = 0,
    ///1: MSI PLL ON
    Enabled = 1,
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
            false => MSIPLLEN_A::Disabled,
            true => MSIPLLEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSIPLLEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSIPLLEN_A::Enabled
    }
}
///Field `MSIPLLEN` writer - MSI clock PLL enable
pub type MSIPLLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, MSIPLLEN_A, O>;
impl<'a, const O: u8> MSIPLLEN_W<'a, O> {
    ///MSI PLL OFF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSIPLLEN_A::Disabled)
    }
    ///MSI PLL ON
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSIPLLEN_A::Enabled)
    }
}
///MSI clock range selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSIRGSEL_AW {
    ///0: MSI Range is provided by MSISRANGE\[3:0\]
    ///in RCC_CSR register
    Csr = 0,
    ///1: MSI Range is provided by MSIRANGE\[3:0\]
    ///in the RCC_CR register
    Cr = 1,
}
impl From<MSIRGSEL_AW> for bool {
    #[inline(always)]
    fn from(variant: MSIRGSEL_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `MSIRGSEL` writer - MSI clock range selection
pub type MSIRGSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, MSIRGSEL_AW, O>;
impl<'a, const O: u8> MSIRGSEL_W<'a, O> {
    ///MSI Range is provided by MSISRANGE\[3:0\]
    ///in RCC_CSR register
    #[inline(always)]
    pub fn csr(self) -> &'a mut W {
        self.variant(MSIRGSEL_AW::Csr)
    }
    ///MSI Range is provided by MSIRANGE\[3:0\]
    ///in the RCC_CR register
    #[inline(always)]
    pub fn cr(self) -> &'a mut W {
        self.variant(MSIRGSEL_AW::Cr)
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
    ///6: range 6 around 4 MHz
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
    ///range 6 around 4 MHz
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
///Field `HSION` reader - HSI clock enable
pub type HSION_R = crate::BitReader<HSION_A>;
///HSI clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSION_A {
    ///0: HSI16 oscillator OFF
    Disabled = 0,
    ///1: HSI16 oscillator ON
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
///Field `HSION` writer - HSI clock enable
pub type HSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSION_A, O>;
impl<'a, const O: u8> HSION_W<'a, O> {
    ///HSI16 oscillator OFF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSION_A::Disabled)
    }
    ///HSI16 oscillator ON
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSION_A::Enabled)
    }
}
///Field `HSIKERON` reader - HSI always enable for peripheral kernels
pub type HSIKERON_R = crate::BitReader<HSIKERON_A>;
///HSI always enable for peripheral kernels
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIKERON_A {
    ///0: No effect on HSI16 oscillator
    Disabled = 0,
    ///1: HSI16 oscillator is forced ON even in Stop mode
    Enabled = 1,
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
            false => HSIKERON_A::Disabled,
            true => HSIKERON_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HSIKERON_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HSIKERON_A::Enabled
    }
}
///Field `HSIKERON` writer - HSI always enable for peripheral kernels
pub type HSIKERON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSIKERON_A, O>;
impl<'a, const O: u8> HSIKERON_W<'a, O> {
    ///No effect on HSI16 oscillator
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSIKERON_A::Disabled)
    }
    ///HSI16 oscillator is forced ON even in Stop mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSIKERON_A::Enabled)
    }
}
///Field `HSIRDY` reader - HSI clock ready flag
pub type HSIRDY_R = crate::BitReader<HSIRDY_A>;
///HSI clock ready flag
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
///Field `HSIASFS` reader - HSI automatic start from Stop
pub type HSIASFS_R = crate::BitReader<HSIASFS_A>;
///HSI automatic start from Stop
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSIASFS_A {
    ///0: HSI16 oscillator is not enabled by hardware when exiting Stop mode with MSI as wakeup clock
    Disabled = 0,
    ///1: HSI16 oscillator is enabled by hardware when exiting Stop mode with MSI as wakeup clock
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
///Field `HSIASFS` writer - HSI automatic start from Stop
pub type HSIASFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSIASFS_A, O>;
impl<'a, const O: u8> HSIASFS_W<'a, O> {
    ///HSI16 oscillator is not enabled by hardware when exiting Stop mode with MSI as wakeup clock
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSIASFS_A::Disabled)
    }
    ///HSI16 oscillator is enabled by hardware when exiting Stop mode with MSI as wakeup clock
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSIASFS_A::Enabled)
    }
}
///Field `HSEON` reader - HSE clock enable
pub type HSEON_R = crate::BitReader<HSEON_A>;
///HSE clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEON_A {
    ///0: HSE oscillator OFF
    Disabled = 0,
    ///1: HSE oscillator ON
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
///Field `HSEON` writer - HSE clock enable
pub type HSEON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSEON_A, O>;
impl<'a, const O: u8> HSEON_W<'a, O> {
    ///HSE oscillator OFF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HSEON_A::Disabled)
    }
    ///HSE oscillator ON
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HSEON_A::Enabled)
    }
}
///Field `HSERDY` reader - HSE clock ready flag
pub type HSERDY_R = crate::BitReader<HSERDY_A>;
///HSE clock ready flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSERDY_A {
    ///0: HSE oscillator not ready
    NotReady = 0,
    ///1: HSE oscillator ready
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
///Field `HSEBYP` reader - HSE crystal oscillator bypass
pub type HSEBYP_R = crate::BitReader<HSEBYP_A>;
///HSE crystal oscillator bypass
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSEBYP_A {
    ///0: HSE crystal oscillator not bypassed
    NotBypassed = 0,
    ///1: HSE crystal oscillator bypassed with external clock
    Bypassed = 1,
}
impl From<HSEBYP_A> for bool {
    #[inline(always)]
    fn from(variant: HSEBYP_A) -> Self {
        variant as u8 != 0
    }
}
impl HSEBYP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HSEBYP_A {
        match self.bits {
            false => HSEBYP_A::NotBypassed,
            true => HSEBYP_A::Bypassed,
        }
    }
    ///Checks if the value of the field is `NotBypassed`
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == HSEBYP_A::NotBypassed
    }
    ///Checks if the value of the field is `Bypassed`
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == HSEBYP_A::Bypassed
    }
}
///Field `HSEBYP` writer - HSE crystal oscillator bypass
pub type HSEBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HSEBYP_A, O>;
impl<'a, const O: u8> HSEBYP_W<'a, O> {
    ///HSE crystal oscillator not bypassed
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut W {
        self.variant(HSEBYP_A::NotBypassed)
    }
    ///HSE crystal oscillator bypassed with external clock
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(HSEBYP_A::Bypassed)
    }
}
///Clock security system enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSSON_AW {
    ///0: Clock security system OFF (clock detector OFF)
    Disabled = 0,
    ///1: Clock security system ON (Clock detector ON if the HSE oscillator is stable, OFF if not)
    Enabled = 1,
}
impl From<CSSON_AW> for bool {
    #[inline(always)]
    fn from(variant: CSSON_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CSSON` writer - Clock security system enable
pub type CSSON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CSSON_AW, O>;
impl<'a, const O: u8> CSSON_W<'a, O> {
    ///Clock security system OFF (clock detector OFF)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CSSON_AW::Disabled)
    }
    ///Clock security system ON (Clock detector ON if the HSE oscillator is stable, OFF if not)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CSSON_AW::Enabled)
    }
}
///Field `PLLON` reader - Main PLL enable
pub type PLLON_R = crate::BitReader<PLLON_A>;
///Main PLL enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLON_A {
    ///0: PLL OFF
    Disabled = 0,
    ///1: PLL ON
    Enabled = 1,
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
            false => PLLON_A::Disabled,
            true => PLLON_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLON_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLON_A::Enabled
    }
}
///Field `PLLON` writer - Main PLL enable
pub type PLLON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, PLLON_A, O>;
impl<'a, const O: u8> PLLON_W<'a, O> {
    ///PLL OFF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLON_A::Disabled)
    }
    ///PLL ON
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLON_A::Enabled)
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
    ///1: PLL locked
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
///Field `PLLSAI1ON` reader - SAI1 PLL enable
pub type PLLSAI1ON_R = crate::BitReader<PLLSAI1ON_A>;
///SAI1 PLL enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI1ON_A {
    ///0: PLLSAI1 OFF
    Disabled = 0,
    ///1: PLLSAI1 ON
    Enabled = 1,
}
impl From<PLLSAI1ON_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI1ON_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSAI1ON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI1ON_A {
        match self.bits {
            false => PLLSAI1ON_A::Disabled,
            true => PLLSAI1ON_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSAI1ON_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSAI1ON_A::Enabled
    }
}
///Field `PLLSAI1ON` writer - SAI1 PLL enable
pub type PLLSAI1ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, PLLSAI1ON_A, O>;
impl<'a, const O: u8> PLLSAI1ON_W<'a, O> {
    ///PLLSAI1 OFF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLSAI1ON_A::Disabled)
    }
    ///PLLSAI1 ON
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLSAI1ON_A::Enabled)
    }
}
///Field `PLLSAI1RDY` reader - SAI1 PLL clock ready flag
pub type PLLSAI1RDY_R = crate::BitReader<PLLSAI1RDY_A>;
///SAI1 PLL clock ready flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI1RDY_A {
    ///0: PLLSAI1 unlocked
    Unlocked = 0,
    ///1: PLLSAI1 locked
    Locked = 1,
}
impl From<PLLSAI1RDY_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI1RDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSAI1RDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI1RDY_A {
        match self.bits {
            false => PLLSAI1RDY_A::Unlocked,
            true => PLLSAI1RDY_A::Locked,
        }
    }
    ///Checks if the value of the field is `Unlocked`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == PLLSAI1RDY_A::Unlocked
    }
    ///Checks if the value of the field is `Locked`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == PLLSAI1RDY_A::Locked
    }
}
///Field `PLLSAI2ON` reader - SAI2 PLL enable
pub type PLLSAI2ON_R = crate::BitReader<PLLSAI2ON_A>;
///SAI2 PLL enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI2ON_A {
    ///0: PLLSAI2 OFF
    Disabled = 0,
    ///1: PLLSAI2 ON
    Enabled = 1,
}
impl From<PLLSAI2ON_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI2ON_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSAI2ON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI2ON_A {
        match self.bits {
            false => PLLSAI2ON_A::Disabled,
            true => PLLSAI2ON_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLSAI2ON_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLSAI2ON_A::Enabled
    }
}
///Field `PLLSAI2ON` writer - SAI2 PLL enable
pub type PLLSAI2ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, PLLSAI2ON_A, O>;
impl<'a, const O: u8> PLLSAI2ON_W<'a, O> {
    ///PLLSAI2 OFF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLSAI2ON_A::Disabled)
    }
    ///PLLSAI2 ON
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLSAI2ON_A::Enabled)
    }
}
///Field `PLLSAI2RDY` reader - SAI2 PLL clock ready flag
pub type PLLSAI2RDY_R = crate::BitReader<PLLSAI2RDY_A>;
///SAI2 PLL clock ready flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSAI2RDY_A {
    ///0: PLLSAI2 unlocked
    Unlocked = 0,
    ///1: PLLSAI2 locked
    Locked = 1,
}
impl From<PLLSAI2RDY_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSAI2RDY_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSAI2RDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSAI2RDY_A {
        match self.bits {
            false => PLLSAI2RDY_A::Unlocked,
            true => PLLSAI2RDY_A::Locked,
        }
    }
    ///Checks if the value of the field is `Unlocked`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == PLLSAI2RDY_A::Unlocked
    }
    ///Checks if the value of the field is `Locked`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == PLLSAI2RDY_A::Locked
    }
}
impl R {
    ///Bit 0 - MSI clock enable
    #[inline(always)]
    pub fn msion(&self) -> MSION_R {
        MSION_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MSI clock ready flag
    #[inline(always)]
    pub fn msirdy(&self) -> MSIRDY_R {
        MSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSI clock PLL enable
    #[inline(always)]
    pub fn msipllen(&self) -> MSIPLLEN_R {
        MSIPLLEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 4:7 - MSI clock ranges
    #[inline(always)]
    pub fn msirange(&self) -> MSIRANGE_R {
        MSIRANGE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - HSI clock enable
    #[inline(always)]
    pub fn hsion(&self) -> HSION_R {
        HSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - HSI always enable for peripheral kernels
    #[inline(always)]
    pub fn hsikeron(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSI clock ready flag
    #[inline(always)]
    pub fn hsirdy(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - HSI automatic start from Stop
    #[inline(always)]
    pub fn hsiasfs(&self) -> HSIASFS_R {
        HSIASFS_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 16 - HSE clock enable
    #[inline(always)]
    pub fn hseon(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HSE clock ready flag
    #[inline(always)]
    pub fn hserdy(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - HSE crystal oscillator bypass
    #[inline(always)]
    pub fn hsebyp(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
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
    ///Bit 26 - SAI1 PLL enable
    #[inline(always)]
    pub fn pllsai1on(&self) -> PLLSAI1ON_R {
        PLLSAI1ON_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - SAI1 PLL clock ready flag
    #[inline(always)]
    pub fn pllsai1rdy(&self) -> PLLSAI1RDY_R {
        PLLSAI1RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SAI2 PLL enable
    #[inline(always)]
    pub fn pllsai2on(&self) -> PLLSAI2ON_R {
        PLLSAI2ON_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SAI2 PLL clock ready flag
    #[inline(always)]
    pub fn pllsai2rdy(&self) -> PLLSAI2RDY_R {
        PLLSAI2RDY_R::new(((self.bits >> 29) & 1) != 0)
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
    ///Bit 3 - MSI clock range selection
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
    ///Bit 8 - HSI clock enable
    #[inline(always)]
    #[must_use]
    pub fn hsion(&mut self) -> HSION_W<8> {
        HSION_W::new(self)
    }
    ///Bit 9 - HSI always enable for peripheral kernels
    #[inline(always)]
    #[must_use]
    pub fn hsikeron(&mut self) -> HSIKERON_W<9> {
        HSIKERON_W::new(self)
    }
    ///Bit 11 - HSI automatic start from Stop
    #[inline(always)]
    #[must_use]
    pub fn hsiasfs(&mut self) -> HSIASFS_W<11> {
        HSIASFS_W::new(self)
    }
    ///Bit 16 - HSE clock enable
    #[inline(always)]
    #[must_use]
    pub fn hseon(&mut self) -> HSEON_W<16> {
        HSEON_W::new(self)
    }
    ///Bit 18 - HSE crystal oscillator bypass
    #[inline(always)]
    #[must_use]
    pub fn hsebyp(&mut self) -> HSEBYP_W<18> {
        HSEBYP_W::new(self)
    }
    ///Bit 19 - Clock security system enable
    #[inline(always)]
    #[must_use]
    pub fn csson(&mut self) -> CSSON_W<19> {
        CSSON_W::new(self)
    }
    ///Bit 24 - Main PLL enable
    #[inline(always)]
    #[must_use]
    pub fn pllon(&mut self) -> PLLON_W<24> {
        PLLON_W::new(self)
    }
    ///Bit 26 - SAI1 PLL enable
    #[inline(always)]
    #[must_use]
    pub fn pllsai1on(&mut self) -> PLLSAI1ON_W<26> {
        PLLSAI1ON_W::new(self)
    }
    ///Bit 28 - SAI2 PLL enable
    #[inline(always)]
    #[must_use]
    pub fn pllsai2on(&mut self) -> PLLSAI2ON_W<28> {
        PLLSAI2ON_W::new(self)
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
///`reset()` method sets CR to value 0x63
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x63;
}
