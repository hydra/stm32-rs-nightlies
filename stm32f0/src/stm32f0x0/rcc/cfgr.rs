///Register `CFGR` reader
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR` writer
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SW` reader - System clock Switch
pub type SW_R = crate::FieldReader<u8, SW_A>;
///System clock Switch
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SW_A {
    ///0: HSI selected as system clock
    Hsi = 0,
    ///1: HSE selected as system clock
    Hse = 1,
    ///2: PLL selected as system clock
    Pll = 2,
    ///3: HSI48 selected as system clock (when available)
    Hsi48 = 3,
}
impl From<SW_A> for u8 {
    #[inline(always)]
    fn from(variant: SW_A) -> Self {
        variant as _
    }
}
impl SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SW_A {
        match self.bits {
            0 => SW_A::Hsi,
            1 => SW_A::Hse,
            2 => SW_A::Pll,
            3 => SW_A::Hsi48,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Hsi`
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SW_A::Hsi
    }
    ///Checks if the value of the field is `Hse`
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SW_A::Hse
    }
    ///Checks if the value of the field is `Pll`
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SW_A::Pll
    }
    ///Checks if the value of the field is `Hsi48`
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == SW_A::Hsi48
    }
}
///Field `SW` writer - System clock Switch
pub type SW_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, SW_A, 2, O>;
impl<'a, const O: u8> SW_W<'a, O> {
    ///HSI selected as system clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(SW_A::Hsi)
    }
    ///HSE selected as system clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(SW_A::Hse)
    }
    ///PLL selected as system clock
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(SW_A::Pll)
    }
    ///HSI48 selected as system clock (when available)
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut W {
        self.variant(SW_A::Hsi48)
    }
}
///Field `SWS` reader - System Clock Switch Status
pub type SWS_R = crate::FieldReader<u8, SWSR_A>;
///System Clock Switch Status
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWSR_A {
    ///0: HSI oscillator used as system clock
    Hsi = 0,
    ///1: HSE oscillator used as system clock
    Hse = 1,
    ///2: PLL used as system clock
    Pll = 2,
    ///3: HSI48 used as system clock (when avaiable)
    Hsi48 = 3,
}
impl From<SWSR_A> for u8 {
    #[inline(always)]
    fn from(variant: SWSR_A) -> Self {
        variant as _
    }
}
impl SWS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SWSR_A {
        match self.bits {
            0 => SWSR_A::Hsi,
            1 => SWSR_A::Hse,
            2 => SWSR_A::Pll,
            3 => SWSR_A::Hsi48,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Hsi`
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SWSR_A::Hsi
    }
    ///Checks if the value of the field is `Hse`
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SWSR_A::Hse
    }
    ///Checks if the value of the field is `Pll`
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SWSR_A::Pll
    }
    ///Checks if the value of the field is `Hsi48`
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == SWSR_A::Hsi48
    }
}
///Field `HPRE` reader - AHB prescaler
pub type HPRE_R = crate::FieldReader<u8, HPRE_A>;
///AHB prescaler
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPRE_A {
    ///0: SYSCLK not divided
    Div1 = 0,
    ///8: SYSCLK divided by 2
    Div2 = 8,
    ///9: SYSCLK divided by 4
    Div4 = 9,
    ///10: SYSCLK divided by 8
    Div8 = 10,
    ///11: SYSCLK divided by 16
    Div16 = 11,
    ///12: SYSCLK divided by 64
    Div64 = 12,
    ///13: SYSCLK divided by 128
    Div128 = 13,
    ///14: SYSCLK divided by 256
    Div256 = 14,
    ///15: SYSCLK divided by 512
    Div512 = 15,
}
impl From<HPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: HPRE_A) -> Self {
        variant as _
    }
}
impl HPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<HPRE_A> {
        match self.bits {
            0 => Some(HPRE_A::Div1),
            8 => Some(HPRE_A::Div2),
            9 => Some(HPRE_A::Div4),
            10 => Some(HPRE_A::Div8),
            11 => Some(HPRE_A::Div16),
            12 => Some(HPRE_A::Div64),
            13 => Some(HPRE_A::Div128),
            14 => Some(HPRE_A::Div256),
            15 => Some(HPRE_A::Div512),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HPRE_A::Div1
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HPRE_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HPRE_A::Div4
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HPRE_A::Div8
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == HPRE_A::Div16
    }
    ///Checks if the value of the field is `Div64`
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == HPRE_A::Div64
    }
    ///Checks if the value of the field is `Div128`
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == HPRE_A::Div128
    }
    ///Checks if the value of the field is `Div256`
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == HPRE_A::Div256
    }
    ///Checks if the value of the field is `Div512`
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == HPRE_A::Div512
    }
}
///Field `HPRE` writer - AHB prescaler
pub type HPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, HPRE_A, 4, O>;
impl<'a, const O: u8> HPRE_W<'a, O> {
    ///SYSCLK not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(HPRE_A::Div1)
    }
    ///SYSCLK divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HPRE_A::Div2)
    }
    ///SYSCLK divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(HPRE_A::Div4)
    }
    ///SYSCLK divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(HPRE_A::Div8)
    }
    ///SYSCLK divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(HPRE_A::Div16)
    }
    ///SYSCLK divided by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(HPRE_A::Div64)
    }
    ///SYSCLK divided by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(HPRE_A::Div128)
    }
    ///SYSCLK divided by 256
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(HPRE_A::Div256)
    }
    ///SYSCLK divided by 512
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(HPRE_A::Div512)
    }
}
///Field `PPRE` reader - APB Low speed prescaler (APB1)
pub type PPRE_R = crate::FieldReader<u8, PPRE_A>;
///APB Low speed prescaler (APB1)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PPRE_A {
    ///0: HCLK not divided
    Div1 = 0,
    ///4: HCLK divided by 2
    Div2 = 4,
    ///5: HCLK divided by 4
    Div4 = 5,
    ///6: HCLK divided by 8
    Div8 = 6,
    ///7: HCLK divided by 16
    Div16 = 7,
}
impl From<PPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: PPRE_A) -> Self {
        variant as _
    }
}
impl PPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PPRE_A> {
        match self.bits {
            0 => Some(PPRE_A::Div1),
            4 => Some(PPRE_A::Div2),
            5 => Some(PPRE_A::Div4),
            6 => Some(PPRE_A::Div8),
            7 => Some(PPRE_A::Div16),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PPRE_A::Div1
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PPRE_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PPRE_A::Div4
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PPRE_A::Div8
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PPRE_A::Div16
    }
}
///Field `PPRE` writer - APB Low speed prescaler (APB1)
pub type PPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, PPRE_A, 3, O>;
impl<'a, const O: u8> PPRE_W<'a, O> {
    ///HCLK not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PPRE_A::Div1)
    }
    ///HCLK divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PPRE_A::Div2)
    }
    ///HCLK divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PPRE_A::Div4)
    }
    ///HCLK divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PPRE_A::Div8)
    }
    ///HCLK divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PPRE_A::Div16)
    }
}
///Field `ADCPRE` reader - ADCPRE is deprecated. See ADC field in CFGR2 register.
pub type ADCPRE_R = crate::BitReader<bool>;
///Field `ADCPRE` writer - ADCPRE is deprecated. See ADC field in CFGR2 register.
pub type ADCPRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
///Field `PLLSRC` reader - PLL input clock source
pub type PLLSRC_R = crate::BitReader<PLLSRC_A>;
///PLL input clock source
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSRC_A {
    ///0: HSI divided by 2 selected as PLL input clock
    HsiDiv2 = 0,
    ///1: HSE divided by PREDIV selected as PLL input clock
    HseDivPrediv = 1,
}
impl From<PLLSRC_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSRC_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLSRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLSRC_A {
        match self.bits {
            false => PLLSRC_A::HsiDiv2,
            true => PLLSRC_A::HseDivPrediv,
        }
    }
    ///Checks if the value of the field is `HsiDiv2`
    #[inline(always)]
    pub fn is_hsi_div2(&self) -> bool {
        *self == PLLSRC_A::HsiDiv2
    }
    ///Checks if the value of the field is `HseDivPrediv`
    #[inline(always)]
    pub fn is_hse_div_prediv(&self) -> bool {
        *self == PLLSRC_A::HseDivPrediv
    }
}
///Field `PLLSRC` writer - PLL input clock source
pub type PLLSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, PLLSRC_A, O>;
impl<'a, const O: u8> PLLSRC_W<'a, O> {
    ///HSI divided by 2 selected as PLL input clock
    #[inline(always)]
    pub fn hsi_div2(self) -> &'a mut W {
        self.variant(PLLSRC_A::HsiDiv2)
    }
    ///HSE divided by PREDIV selected as PLL input clock
    #[inline(always)]
    pub fn hse_div_prediv(self) -> &'a mut W {
        self.variant(PLLSRC_A::HseDivPrediv)
    }
}
///Field `PLLXTPRE` reader - HSE divider for PLL entry. Same bit as PREDIC\[0\]
///from CFGR2 register. Refer to it for its meaning
pub type PLLXTPRE_R = crate::BitReader<PLLXTPRE_A>;
///HSE divider for PLL entry. Same bit as PREDIC\[0\]
///from CFGR2 register. Refer to it for its meaning
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLXTPRE_A {
    ///0: HSE clock not divided
    Div1 = 0,
    ///1: HSE clock divided by 2
    Div2 = 1,
}
impl From<PLLXTPRE_A> for bool {
    #[inline(always)]
    fn from(variant: PLLXTPRE_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLXTPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLXTPRE_A {
        match self.bits {
            false => PLLXTPRE_A::Div1,
            true => PLLXTPRE_A::Div2,
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLXTPRE_A::Div1
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLXTPRE_A::Div2
    }
}
///Field `PLLXTPRE` writer - HSE divider for PLL entry. Same bit as PREDIC\[0\]
///from CFGR2 register. Refer to it for its meaning
pub type PLLXTPRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, PLLXTPRE_A, O>;
impl<'a, const O: u8> PLLXTPRE_W<'a, O> {
    ///HSE clock not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLXTPRE_A::Div1)
    }
    ///HSE clock divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLXTPRE_A::Div2)
    }
}
///Field `PLLMUL` reader - PLL Multiplication Factor
pub type PLLMUL_R = crate::FieldReader<u8, PLLMUL_A>;
///PLL Multiplication Factor
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLMUL_A {
    ///0: PLL input clock x2
    Mul2 = 0,
    ///1: PLL input clock x3
    Mul3 = 1,
    ///2: PLL input clock x4
    Mul4 = 2,
    ///3: PLL input clock x5
    Mul5 = 3,
    ///4: PLL input clock x6
    Mul6 = 4,
    ///5: PLL input clock x7
    Mul7 = 5,
    ///6: PLL input clock x8
    Mul8 = 6,
    ///7: PLL input clock x9
    Mul9 = 7,
    ///8: PLL input clock x10
    Mul10 = 8,
    ///9: PLL input clock x11
    Mul11 = 9,
    ///10: PLL input clock x12
    Mul12 = 10,
    ///11: PLL input clock x13
    Mul13 = 11,
    ///12: PLL input clock x14
    Mul14 = 12,
    ///13: PLL input clock x15
    Mul15 = 13,
    ///14: PLL input clock x16
    Mul16 = 14,
    ///15: PLL input clock x16
    Mul16x = 15,
}
impl From<PLLMUL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLMUL_A) -> Self {
        variant as _
    }
}
impl PLLMUL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLMUL_A {
        match self.bits {
            0 => PLLMUL_A::Mul2,
            1 => PLLMUL_A::Mul3,
            2 => PLLMUL_A::Mul4,
            3 => PLLMUL_A::Mul5,
            4 => PLLMUL_A::Mul6,
            5 => PLLMUL_A::Mul7,
            6 => PLLMUL_A::Mul8,
            7 => PLLMUL_A::Mul9,
            8 => PLLMUL_A::Mul10,
            9 => PLLMUL_A::Mul11,
            10 => PLLMUL_A::Mul12,
            11 => PLLMUL_A::Mul13,
            12 => PLLMUL_A::Mul14,
            13 => PLLMUL_A::Mul15,
            14 => PLLMUL_A::Mul16,
            15 => PLLMUL_A::Mul16x,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Mul2`
    #[inline(always)]
    pub fn is_mul2(&self) -> bool {
        *self == PLLMUL_A::Mul2
    }
    ///Checks if the value of the field is `Mul3`
    #[inline(always)]
    pub fn is_mul3(&self) -> bool {
        *self == PLLMUL_A::Mul3
    }
    ///Checks if the value of the field is `Mul4`
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        *self == PLLMUL_A::Mul4
    }
    ///Checks if the value of the field is `Mul5`
    #[inline(always)]
    pub fn is_mul5(&self) -> bool {
        *self == PLLMUL_A::Mul5
    }
    ///Checks if the value of the field is `Mul6`
    #[inline(always)]
    pub fn is_mul6(&self) -> bool {
        *self == PLLMUL_A::Mul6
    }
    ///Checks if the value of the field is `Mul7`
    #[inline(always)]
    pub fn is_mul7(&self) -> bool {
        *self == PLLMUL_A::Mul7
    }
    ///Checks if the value of the field is `Mul8`
    #[inline(always)]
    pub fn is_mul8(&self) -> bool {
        *self == PLLMUL_A::Mul8
    }
    ///Checks if the value of the field is `Mul9`
    #[inline(always)]
    pub fn is_mul9(&self) -> bool {
        *self == PLLMUL_A::Mul9
    }
    ///Checks if the value of the field is `Mul10`
    #[inline(always)]
    pub fn is_mul10(&self) -> bool {
        *self == PLLMUL_A::Mul10
    }
    ///Checks if the value of the field is `Mul11`
    #[inline(always)]
    pub fn is_mul11(&self) -> bool {
        *self == PLLMUL_A::Mul11
    }
    ///Checks if the value of the field is `Mul12`
    #[inline(always)]
    pub fn is_mul12(&self) -> bool {
        *self == PLLMUL_A::Mul12
    }
    ///Checks if the value of the field is `Mul13`
    #[inline(always)]
    pub fn is_mul13(&self) -> bool {
        *self == PLLMUL_A::Mul13
    }
    ///Checks if the value of the field is `Mul14`
    #[inline(always)]
    pub fn is_mul14(&self) -> bool {
        *self == PLLMUL_A::Mul14
    }
    ///Checks if the value of the field is `Mul15`
    #[inline(always)]
    pub fn is_mul15(&self) -> bool {
        *self == PLLMUL_A::Mul15
    }
    ///Checks if the value of the field is `Mul16`
    #[inline(always)]
    pub fn is_mul16(&self) -> bool {
        *self == PLLMUL_A::Mul16
    }
    ///Checks if the value of the field is `Mul16x`
    #[inline(always)]
    pub fn is_mul16x(&self) -> bool {
        *self == PLLMUL_A::Mul16x
    }
}
///Field `PLLMUL` writer - PLL Multiplication Factor
pub type PLLMUL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, PLLMUL_A, 4, O>;
impl<'a, const O: u8> PLLMUL_W<'a, O> {
    ///PLL input clock x2
    #[inline(always)]
    pub fn mul2(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul2)
    }
    ///PLL input clock x3
    #[inline(always)]
    pub fn mul3(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul3)
    }
    ///PLL input clock x4
    #[inline(always)]
    pub fn mul4(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul4)
    }
    ///PLL input clock x5
    #[inline(always)]
    pub fn mul5(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul5)
    }
    ///PLL input clock x6
    #[inline(always)]
    pub fn mul6(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul6)
    }
    ///PLL input clock x7
    #[inline(always)]
    pub fn mul7(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul7)
    }
    ///PLL input clock x8
    #[inline(always)]
    pub fn mul8(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul8)
    }
    ///PLL input clock x9
    #[inline(always)]
    pub fn mul9(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul9)
    }
    ///PLL input clock x10
    #[inline(always)]
    pub fn mul10(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul10)
    }
    ///PLL input clock x11
    #[inline(always)]
    pub fn mul11(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul11)
    }
    ///PLL input clock x12
    #[inline(always)]
    pub fn mul12(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul12)
    }
    ///PLL input clock x13
    #[inline(always)]
    pub fn mul13(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul13)
    }
    ///PLL input clock x14
    #[inline(always)]
    pub fn mul14(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul14)
    }
    ///PLL input clock x15
    #[inline(always)]
    pub fn mul15(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul15)
    }
    ///PLL input clock x16
    #[inline(always)]
    pub fn mul16(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul16)
    }
    ///PLL input clock x16
    #[inline(always)]
    pub fn mul16x(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul16x)
    }
}
///Field `MCO` reader - Microcontroller clock output
pub type MCO_R = crate::FieldReader<u8, MCO_A>;
///Microcontroller clock output
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO_A {
    ///0: MCO output disabled, no clock on MCO
    NoMco = 0,
    ///1: Internal RC 14 MHz (HSI14) oscillator clock selected
    Hsi14 = 1,
    ///2: Internal low speed (LSI) oscillator clock selected
    Lsi = 2,
    ///3: External low speed (LSE) oscillator clock selected
    Lse = 3,
    ///4: System clock selected
    Sysclk = 4,
    ///5: Internal RC 8 MHz (HSI) oscillator clock selected
    Hsi = 5,
    ///6: External 4-32 MHz (HSE) oscillator clock selected
    Hse = 6,
    ///7: PLL clock selected (divided by 1 or 2, depending en PLLNODIV)
    Pll = 7,
    ///8: Internal RC 48 MHz (HSI48) oscillator clock selected
    Hsi48 = 8,
}
impl From<MCO_A> for u8 {
    #[inline(always)]
    fn from(variant: MCO_A) -> Self {
        variant as _
    }
}
impl MCO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<MCO_A> {
        match self.bits {
            0 => Some(MCO_A::NoMco),
            1 => Some(MCO_A::Hsi14),
            2 => Some(MCO_A::Lsi),
            3 => Some(MCO_A::Lse),
            4 => Some(MCO_A::Sysclk),
            5 => Some(MCO_A::Hsi),
            6 => Some(MCO_A::Hse),
            7 => Some(MCO_A::Pll),
            8 => Some(MCO_A::Hsi48),
            _ => None,
        }
    }
    ///Checks if the value of the field is `NoMco`
    #[inline(always)]
    pub fn is_no_mco(&self) -> bool {
        *self == MCO_A::NoMco
    }
    ///Checks if the value of the field is `Hsi14`
    #[inline(always)]
    pub fn is_hsi14(&self) -> bool {
        *self == MCO_A::Hsi14
    }
    ///Checks if the value of the field is `Lsi`
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == MCO_A::Lsi
    }
    ///Checks if the value of the field is `Lse`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == MCO_A::Lse
    }
    ///Checks if the value of the field is `Sysclk`
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == MCO_A::Sysclk
    }
    ///Checks if the value of the field is `Hsi`
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == MCO_A::Hsi
    }
    ///Checks if the value of the field is `Hse`
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCO_A::Hse
    }
    ///Checks if the value of the field is `Pll`
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == MCO_A::Pll
    }
    ///Checks if the value of the field is `Hsi48`
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == MCO_A::Hsi48
    }
}
///Field `MCO` writer - Microcontroller clock output
pub type MCO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, MCO_A, 4, O>;
impl<'a, const O: u8> MCO_W<'a, O> {
    ///MCO output disabled, no clock on MCO
    #[inline(always)]
    pub fn no_mco(self) -> &'a mut W {
        self.variant(MCO_A::NoMco)
    }
    ///Internal RC 14 MHz (HSI14) oscillator clock selected
    #[inline(always)]
    pub fn hsi14(self) -> &'a mut W {
        self.variant(MCO_A::Hsi14)
    }
    ///Internal low speed (LSI) oscillator clock selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(MCO_A::Lsi)
    }
    ///External low speed (LSE) oscillator clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(MCO_A::Lse)
    }
    ///System clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(MCO_A::Sysclk)
    }
    ///Internal RC 8 MHz (HSI) oscillator clock selected
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(MCO_A::Hsi)
    }
    ///External 4-32 MHz (HSE) oscillator clock selected
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(MCO_A::Hse)
    }
    ///PLL clock selected (divided by 1 or 2, depending en PLLNODIV)
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(MCO_A::Pll)
    }
    ///Internal RC 48 MHz (HSI48) oscillator clock selected
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut W {
        self.variant(MCO_A::Hsi48)
    }
}
///Field `MCOPRE` reader - Microcontroller Clock Output Prescaler
pub type MCOPRE_R = crate::FieldReader<u8, MCOPRE_A>;
///Microcontroller Clock Output Prescaler
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCOPRE_A {
    ///0: MCO is divided by 1
    Div1 = 0,
    ///1: MCO is divided by 2
    Div2 = 1,
    ///2: MCO is divided by 4
    Div4 = 2,
    ///3: MCO is divided by 8
    Div8 = 3,
    ///4: MCO is divided by 16
    Div16 = 4,
    ///5: MCO is divided by 32
    Div32 = 5,
    ///6: MCO is divided by 64
    Div64 = 6,
    ///7: MCO is divided by 128
    Div128 = 7,
}
impl From<MCOPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: MCOPRE_A) -> Self {
        variant as _
    }
}
impl MCOPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MCOPRE_A {
        match self.bits {
            0 => MCOPRE_A::Div1,
            1 => MCOPRE_A::Div2,
            2 => MCOPRE_A::Div4,
            3 => MCOPRE_A::Div8,
            4 => MCOPRE_A::Div16,
            5 => MCOPRE_A::Div32,
            6 => MCOPRE_A::Div64,
            7 => MCOPRE_A::Div128,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == MCOPRE_A::Div1
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == MCOPRE_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == MCOPRE_A::Div4
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == MCOPRE_A::Div8
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == MCOPRE_A::Div16
    }
    ///Checks if the value of the field is `Div32`
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == MCOPRE_A::Div32
    }
    ///Checks if the value of the field is `Div64`
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == MCOPRE_A::Div64
    }
    ///Checks if the value of the field is `Div128`
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == MCOPRE_A::Div128
    }
}
///Field `MCOPRE` writer - Microcontroller Clock Output Prescaler
pub type MCOPRE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, MCOPRE_A, 3, O>;
impl<'a, const O: u8> MCOPRE_W<'a, O> {
    ///MCO is divided by 1
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div1)
    }
    ///MCO is divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div2)
    }
    ///MCO is divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div4)
    }
    ///MCO is divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div8)
    }
    ///MCO is divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div16)
    }
    ///MCO is divided by 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div32)
    }
    ///MCO is divided by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div64)
    }
    ///MCO is divided by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div128)
    }
}
///Field `PLLNODIV` reader - PLL clock not divided for MCO
pub type PLLNODIV_R = crate::BitReader<PLLNODIV_A>;
///PLL clock not divided for MCO
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLNODIV_A {
    ///0: PLL is divided by 2 for MCO
    Div2 = 0,
    ///1: PLL is not divided for MCO
    Div1 = 1,
}
impl From<PLLNODIV_A> for bool {
    #[inline(always)]
    fn from(variant: PLLNODIV_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLNODIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLLNODIV_A {
        match self.bits {
            false => PLLNODIV_A::Div2,
            true => PLLNODIV_A::Div1,
        }
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLNODIV_A::Div2
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLNODIV_A::Div1
    }
}
///Field `PLLNODIV` writer - PLL clock not divided for MCO
pub type PLLNODIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, PLLNODIV_A, O>;
impl<'a, const O: u8> PLLNODIV_W<'a, O> {
    ///PLL is divided by 2 for MCO
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLNODIV_A::Div2)
    }
    ///PLL is not divided for MCO
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLNODIV_A::Div1)
    }
}
impl R {
    ///Bits 0:1 - System clock Switch
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - System Clock Switch Status
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - AHB prescaler
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:10 - APB Low speed prescaler (APB1)
    #[inline(always)]
    pub fn ppre(&self) -> PPRE_R {
        PPRE_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 14 - ADCPRE is deprecated. See ADC field in CFGR2 register.
    #[inline(always)]
    pub fn adcpre(&self) -> ADCPRE_R {
        ADCPRE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - PLL input clock source
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HSE divider for PLL entry. Same bit as PREDIC\[0\]
    ///from CFGR2 register. Refer to it for its meaning
    #[inline(always)]
    pub fn pllxtpre(&self) -> PLLXTPRE_R {
        PLLXTPRE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:21 - PLL Multiplication Factor
    #[inline(always)]
    pub fn pllmul(&self) -> PLLMUL_R {
        PLLMUL_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bits 24:27 - Microcontroller clock output
    #[inline(always)]
    pub fn mco(&self) -> MCO_R {
        MCO_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:30 - Microcontroller Clock Output Prescaler
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 7) as u8)
    }
    ///Bit 31 - PLL clock not divided for MCO
    #[inline(always)]
    pub fn pllnodiv(&self) -> PLLNODIV_R {
        PLLNODIV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - System clock Switch
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<0> {
        SW_W::new(self)
    }
    ///Bits 4:7 - AHB prescaler
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HPRE_W<4> {
        HPRE_W::new(self)
    }
    ///Bits 8:10 - APB Low speed prescaler (APB1)
    #[inline(always)]
    #[must_use]
    pub fn ppre(&mut self) -> PPRE_W<8> {
        PPRE_W::new(self)
    }
    ///Bit 14 - ADCPRE is deprecated. See ADC field in CFGR2 register.
    #[inline(always)]
    #[must_use]
    pub fn adcpre(&mut self) -> ADCPRE_W<14> {
        ADCPRE_W::new(self)
    }
    ///Bit 16 - PLL input clock source
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<16> {
        PLLSRC_W::new(self)
    }
    ///Bit 17 - HSE divider for PLL entry. Same bit as PREDIC\[0\]
    ///from CFGR2 register. Refer to it for its meaning
    #[inline(always)]
    #[must_use]
    pub fn pllxtpre(&mut self) -> PLLXTPRE_W<17> {
        PLLXTPRE_W::new(self)
    }
    ///Bits 18:21 - PLL Multiplication Factor
    #[inline(always)]
    #[must_use]
    pub fn pllmul(&mut self) -> PLLMUL_W<18> {
        PLLMUL_W::new(self)
    }
    ///Bits 24:27 - Microcontroller clock output
    #[inline(always)]
    #[must_use]
    pub fn mco(&mut self) -> MCO_W<24> {
        MCO_W::new(self)
    }
    ///Bits 28:30 - Microcontroller Clock Output Prescaler
    #[inline(always)]
    #[must_use]
    pub fn mcopre(&mut self) -> MCOPRE_W<28> {
        MCOPRE_W::new(self)
    }
    ///Bit 31 - PLL clock not divided for MCO
    #[inline(always)]
    #[must_use]
    pub fn pllnodiv(&mut self) -> PLLNODIV_W<31> {
        PLLNODIV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Clock configuration register (RCC_CFGR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr](index.html) module
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr::R](R) reader structure
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr::W](W) writer structure
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
