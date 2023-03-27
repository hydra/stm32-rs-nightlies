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
    pub fn variant(&self) -> Option<SW_A> {
        match self.bits {
            0 => Some(SW_A::Hsi),
            1 => Some(SW_A::Hse),
            2 => Some(SW_A::Pll),
            _ => None,
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
}
///Field `SW` writer - System clock Switch
pub type SW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, SW_A, 2, O>;
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
    pub fn variant(&self) -> Option<SWSR_A> {
        match self.bits {
            0 => Some(SWSR_A::Hsi),
            1 => Some(SWSR_A::Hse),
            2 => Some(SWSR_A::Pll),
            _ => None,
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
///Field `PPRE1` reader - APB Low speed prescaler (APB1)
pub type PPRE1_R = crate::FieldReader<u8, PPRE1_A>;
///APB Low speed prescaler (APB1)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PPRE1_A {
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
impl From<PPRE1_A> for u8 {
    #[inline(always)]
    fn from(variant: PPRE1_A) -> Self {
        variant as _
    }
}
impl PPRE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PPRE1_A> {
        match self.bits {
            0 => Some(PPRE1_A::Div1),
            4 => Some(PPRE1_A::Div2),
            5 => Some(PPRE1_A::Div4),
            6 => Some(PPRE1_A::Div8),
            7 => Some(PPRE1_A::Div16),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PPRE1_A::Div1
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PPRE1_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PPRE1_A::Div4
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PPRE1_A::Div8
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PPRE1_A::Div16
    }
}
///Field `PPRE1` writer - APB Low speed prescaler (APB1)
pub type PPRE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, PPRE1_A, 3, O>;
impl<'a, const O: u8> PPRE1_W<'a, O> {
    ///HCLK not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PPRE1_A::Div1)
    }
    ///HCLK divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PPRE1_A::Div2)
    }
    ///HCLK divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PPRE1_A::Div4)
    }
    ///HCLK divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PPRE1_A::Div8)
    }
    ///HCLK divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PPRE1_A::Div16)
    }
}
///Field `PPRE2` reader - APB High speed prescaler (APB2)
pub use PPRE1_R as PPRE2_R;
///Field `PPRE2` writer - APB High speed prescaler (APB2)
pub use PPRE1_W as PPRE2_W;
///Field `ADCPRE` reader - ADC prescaler
pub type ADCPRE_R = crate::FieldReader<u8, ADCPRE_A>;
///ADC prescaler
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCPRE_A {
    ///0: PCLK2 divided by 2
    Div2 = 0,
    ///1: PCLK2 divided by 4
    Div4 = 1,
    ///2: PCLK2 divided by 8
    Div6 = 2,
    ///3: PCLK2 divided by 16
    Div8 = 3,
}
impl From<ADCPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCPRE_A) -> Self {
        variant as _
    }
}
impl ADCPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADCPRE_A {
        match self.bits {
            0 => ADCPRE_A::Div2,
            1 => ADCPRE_A::Div4,
            2 => ADCPRE_A::Div6,
            3 => ADCPRE_A::Div8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == ADCPRE_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == ADCPRE_A::Div4
    }
    ///Checks if the value of the field is `Div6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == ADCPRE_A::Div6
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == ADCPRE_A::Div8
    }
}
///Field `ADCPRE` writer - ADC prescaler
pub type ADCPRE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, ADCPRE_A, 2, O>;
impl<'a, const O: u8> ADCPRE_W<'a, O> {
    ///PCLK2 divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(ADCPRE_A::Div2)
    }
    ///PCLK2 divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(ADCPRE_A::Div4)
    }
    ///PCLK2 divided by 8
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(ADCPRE_A::Div6)
    }
    ///PCLK2 divided by 16
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(ADCPRE_A::Div8)
    }
}
///Field `PLLSRC` reader - PLL entry clock source
pub type PLLSRC_R = crate::BitReader<PLLSRC_A>;
///PLL entry clock source
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
///Field `PLLSRC` writer - PLL entry clock source
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
///Field `PLLXTPRE` reader - HSE divider for PLL entry
pub type PLLXTPRE_R = crate::BitReader<PLLXTPRE_A>;
///HSE divider for PLL entry
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
///Field `PLLXTPRE` writer - HSE divider for PLL entry
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
    ///13: PLL input clock x6.5
    Mul65 = 13,
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
    pub fn variant(&self) -> Option<PLLMUL_A> {
        match self.bits {
            2 => Some(PLLMUL_A::Mul4),
            3 => Some(PLLMUL_A::Mul5),
            4 => Some(PLLMUL_A::Mul6),
            5 => Some(PLLMUL_A::Mul7),
            6 => Some(PLLMUL_A::Mul8),
            7 => Some(PLLMUL_A::Mul9),
            13 => Some(PLLMUL_A::Mul65),
            _ => None,
        }
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
    ///Checks if the value of the field is `Mul65`
    #[inline(always)]
    pub fn is_mul6_5(&self) -> bool {
        *self == PLLMUL_A::Mul65
    }
}
///Field `PLLMUL` writer - PLL Multiplication Factor
pub type PLLMUL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, PLLMUL_A, 4, O>;
impl<'a, const O: u8> PLLMUL_W<'a, O> {
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
    ///PLL input clock x6.5
    #[inline(always)]
    pub fn mul6_5(self) -> &'a mut W {
        self.variant(PLLMUL_A::Mul65)
    }
}
///Field `OTGFSPRE` reader - USB OTG FS prescaler
pub type OTGFSPRE_R = crate::BitReader<OTGFSPRE_A>;
///USB OTG FS prescaler
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTGFSPRE_A {
    ///0: PLL clock is divided by 1.5
    Div15 = 0,
    ///1: PLL clock is not divided
    Div1 = 1,
}
impl From<OTGFSPRE_A> for bool {
    #[inline(always)]
    fn from(variant: OTGFSPRE_A) -> Self {
        variant as u8 != 0
    }
}
impl OTGFSPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OTGFSPRE_A {
        match self.bits {
            false => OTGFSPRE_A::Div15,
            true => OTGFSPRE_A::Div1,
        }
    }
    ///Checks if the value of the field is `Div15`
    #[inline(always)]
    pub fn is_div1_5(&self) -> bool {
        *self == OTGFSPRE_A::Div15
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == OTGFSPRE_A::Div1
    }
}
///Field `OTGFSPRE` writer - USB OTG FS prescaler
pub type OTGFSPRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, OTGFSPRE_A, O>;
impl<'a, const O: u8> OTGFSPRE_W<'a, O> {
    ///PLL clock is divided by 1.5
    #[inline(always)]
    pub fn div1_5(self) -> &'a mut W {
        self.variant(OTGFSPRE_A::Div15)
    }
    ///PLL clock is not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(OTGFSPRE_A::Div1)
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
    ///4: System clock selected
    Sysclk = 4,
    ///5: HSI oscillator clock selected
    Hsi = 5,
    ///6: HSE oscillator clock selected
    Hse = 6,
    ///7: PLL clock divided by 2 selected
    Pll = 7,
    ///8: PLL2 clock selected
    Pll2 = 8,
    ///9: PLL3 clock divided by 2 selected
    Pll3 = 9,
    ///10: XT1 external 3-25 MHz oscillator clock selected (for Ethernet)
    Xt1 = 10,
    ///11: PLL3 clock selected (for Ethernet)
    Pll3ethernet = 11,
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
            4 => Some(MCO_A::Sysclk),
            5 => Some(MCO_A::Hsi),
            6 => Some(MCO_A::Hse),
            7 => Some(MCO_A::Pll),
            8 => Some(MCO_A::Pll2),
            9 => Some(MCO_A::Pll3),
            10 => Some(MCO_A::Xt1),
            11 => Some(MCO_A::Pll3ethernet),
            _ => None,
        }
    }
    ///Checks if the value of the field is `NoMco`
    #[inline(always)]
    pub fn is_no_mco(&self) -> bool {
        *self == MCO_A::NoMco
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
    ///Checks if the value of the field is `Pll2`
    #[inline(always)]
    pub fn is_pll2(&self) -> bool {
        *self == MCO_A::Pll2
    }
    ///Checks if the value of the field is `Pll3`
    #[inline(always)]
    pub fn is_pll3(&self) -> bool {
        *self == MCO_A::Pll3
    }
    ///Checks if the value of the field is `Xt1`
    #[inline(always)]
    pub fn is_xt1(&self) -> bool {
        *self == MCO_A::Xt1
    }
    ///Checks if the value of the field is `Pll3ethernet`
    #[inline(always)]
    pub fn is_pll3ethernet(&self) -> bool {
        *self == MCO_A::Pll3ethernet
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
    ///System clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(MCO_A::Sysclk)
    }
    ///HSI oscillator clock selected
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(MCO_A::Hsi)
    }
    ///HSE oscillator clock selected
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(MCO_A::Hse)
    }
    ///PLL clock divided by 2 selected
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(MCO_A::Pll)
    }
    ///PLL2 clock selected
    #[inline(always)]
    pub fn pll2(self) -> &'a mut W {
        self.variant(MCO_A::Pll2)
    }
    ///PLL3 clock divided by 2 selected
    #[inline(always)]
    pub fn pll3(self) -> &'a mut W {
        self.variant(MCO_A::Pll3)
    }
    ///XT1 external 3-25 MHz oscillator clock selected (for Ethernet)
    #[inline(always)]
    pub fn xt1(self) -> &'a mut W {
        self.variant(MCO_A::Xt1)
    }
    ///PLL3 clock selected (for Ethernet)
    #[inline(always)]
    pub fn pll3ethernet(self) -> &'a mut W {
        self.variant(MCO_A::Pll3ethernet)
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
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:13 - APB High speed prescaler (APB2)
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 11) & 7) as u8)
    }
    ///Bits 14:15 - ADC prescaler
    #[inline(always)]
    pub fn adcpre(&self) -> ADCPRE_R {
        ADCPRE_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - PLL entry clock source
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HSE divider for PLL entry
    #[inline(always)]
    pub fn pllxtpre(&self) -> PLLXTPRE_R {
        PLLXTPRE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:21 - PLL Multiplication Factor
    #[inline(always)]
    pub fn pllmul(&self) -> PLLMUL_R {
        PLLMUL_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bit 22 - USB OTG FS prescaler
    #[inline(always)]
    pub fn otgfspre(&self) -> OTGFSPRE_R {
        OTGFSPRE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 24:27 - Microcontroller clock output
    #[inline(always)]
    pub fn mco(&self) -> MCO_R {
        MCO_R::new(((self.bits >> 24) & 0x0f) as u8)
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
    pub fn ppre1(&mut self) -> PPRE1_W<8> {
        PPRE1_W::new(self)
    }
    ///Bits 11:13 - APB High speed prescaler (APB2)
    #[inline(always)]
    #[must_use]
    pub fn ppre2(&mut self) -> PPRE2_W<11> {
        PPRE2_W::new(self)
    }
    ///Bits 14:15 - ADC prescaler
    #[inline(always)]
    #[must_use]
    pub fn adcpre(&mut self) -> ADCPRE_W<14> {
        ADCPRE_W::new(self)
    }
    ///Bit 16 - PLL entry clock source
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<16> {
        PLLSRC_W::new(self)
    }
    ///Bit 17 - HSE divider for PLL entry
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
    ///Bit 22 - USB OTG FS prescaler
    #[inline(always)]
    #[must_use]
    pub fn otgfspre(&mut self) -> OTGFSPRE_W<22> {
        OTGFSPRE_W::new(self)
    }
    ///Bits 24:27 - Microcontroller clock output
    #[inline(always)]
    #[must_use]
    pub fn mco(&mut self) -> MCO_W<24> {
        MCO_W::new(self)
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
