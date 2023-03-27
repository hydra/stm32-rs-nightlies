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
///Field `SW` reader - System clock switch
pub type SW_R = crate::FieldReader<u8, SW_A>;
///System clock switch
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
///Field `SW` writer - System clock switch
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
///Field `SWS` reader - System clock switch status
pub type SWS_R = crate::FieldReader<u8, SWSR_A>;
///System clock switch status
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
///Field `SWS` writer - System clock switch status
pub type SWS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, SWSR_A, 2, O>;
impl<'a, const O: u8> SWS_W<'a, O> {
    ///HSI oscillator used as system clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(SWSR_A::Hsi)
    }
    ///HSE oscillator used as system clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(SWSR_A::Hse)
    }
    ///PLL used as system clock
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(SWSR_A::Pll)
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
///Field `MCO1EN` reader - MCO output enable
pub type MCO1EN_R = crate::BitReader<MCO1EN_A>;
///MCO output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCO1EN_A {
    ///0: MCO output disabled
    Disabled = 0,
    ///1: MCO output enabled
    Enabled = 1,
}
impl From<MCO1EN_A> for bool {
    #[inline(always)]
    fn from(variant: MCO1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl MCO1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MCO1EN_A {
        match self.bits {
            false => MCO1EN_A::Disabled,
            true => MCO1EN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCO1EN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCO1EN_A::Enabled
    }
}
///Field `MCO1EN` writer - MCO output enable
pub type MCO1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, MCO1EN_A, O>;
impl<'a, const O: u8> MCO1EN_W<'a, O> {
    ///MCO output disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MCO1EN_A::Disabled)
    }
    ///MCO output enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MCO1EN_A::Enabled)
    }
}
///Field `MCO2EN` reader - MCO output enable
pub use MCO1EN_R as MCO2EN_R;
///Field `MCO2EN` writer - MCO output enable
pub use MCO1EN_W as MCO2EN_W;
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
///Field `PPRE2` reader - APB high-speed prescaler (APB2)
pub use PPRE1_R as PPRE2_R;
///Field `PPRE2` writer - APB high-speed prescaler (APB2)
pub use PPRE1_W as PPRE2_W;
///Field `RTCPRE` reader - HSE division factor for RTC clock
pub type RTCPRE_R = crate::FieldReader<u8, u8>;
///Field `RTCPRE` writer - HSE division factor for RTC clock
pub type RTCPRE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, u8, 5, O>;
///Field `MCO1` reader - Microcontroller clock output 1
pub type MCO1_R = crate::FieldReader<u8, MCO1_A>;
///Microcontroller clock output 1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO1_A {
    ///0: HSI clock selected
    Hsi = 0,
    ///1: LSE oscillator selected
    Lse = 1,
    ///2: HSE oscillator clock selected
    Hse = 2,
    ///3: PLL clock selected
    Pll = 3,
}
impl From<MCO1_A> for u8 {
    #[inline(always)]
    fn from(variant: MCO1_A) -> Self {
        variant as _
    }
}
impl MCO1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MCO1_A {
        match self.bits {
            0 => MCO1_A::Hsi,
            1 => MCO1_A::Lse,
            2 => MCO1_A::Hse,
            3 => MCO1_A::Pll,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Hsi`
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == MCO1_A::Hsi
    }
    ///Checks if the value of the field is `Lse`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == MCO1_A::Lse
    }
    ///Checks if the value of the field is `Hse`
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCO1_A::Hse
    }
    ///Checks if the value of the field is `Pll`
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == MCO1_A::Pll
    }
}
///Field `MCO1` writer - Microcontroller clock output 1
pub type MCO1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, MCO1_A, 2, O>;
impl<'a, const O: u8> MCO1_W<'a, O> {
    ///HSI clock selected
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(MCO1_A::Hsi)
    }
    ///LSE oscillator selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(MCO1_A::Lse)
    }
    ///HSE oscillator clock selected
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(MCO1_A::Hse)
    }
    ///PLL clock selected
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(MCO1_A::Pll)
    }
}
///Field `MCO1PRE` reader - MCO1 prescaler
pub type MCO1PRE_R = crate::FieldReader<u8, MCO1PRE_A>;
///MCO1 prescaler
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO1PRE_A {
    ///0: No division
    Div1 = 0,
    ///4: Division by 2
    Div2 = 4,
    ///5: Division by 3
    Div3 = 5,
    ///6: Division by 4
    Div4 = 6,
    ///7: Division by 5
    Div5 = 7,
}
impl From<MCO1PRE_A> for u8 {
    #[inline(always)]
    fn from(variant: MCO1PRE_A) -> Self {
        variant as _
    }
}
impl MCO1PRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<MCO1PRE_A> {
        match self.bits {
            0 => Some(MCO1PRE_A::Div1),
            4 => Some(MCO1PRE_A::Div2),
            5 => Some(MCO1PRE_A::Div3),
            6 => Some(MCO1PRE_A::Div4),
            7 => Some(MCO1PRE_A::Div5),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == MCO1PRE_A::Div1
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == MCO1PRE_A::Div2
    }
    ///Checks if the value of the field is `Div3`
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == MCO1PRE_A::Div3
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == MCO1PRE_A::Div4
    }
    ///Checks if the value of the field is `Div5`
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == MCO1PRE_A::Div5
    }
}
///Field `MCO1PRE` writer - MCO1 prescaler
pub type MCO1PRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, MCO1PRE_A, 3, O>;
impl<'a, const O: u8> MCO1PRE_W<'a, O> {
    ///No division
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(MCO1PRE_A::Div1)
    }
    ///Division by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(MCO1PRE_A::Div2)
    }
    ///Division by 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut W {
        self.variant(MCO1PRE_A::Div3)
    }
    ///Division by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(MCO1PRE_A::Div4)
    }
    ///Division by 5
    #[inline(always)]
    pub fn div5(self) -> &'a mut W {
        self.variant(MCO1PRE_A::Div5)
    }
}
///Field `MCO2PRE` reader - MCO2 prescaler
pub use MCO1PRE_R as MCO2PRE_R;
///Field `MCO2PRE` writer - MCO2 prescaler
pub use MCO1PRE_W as MCO2PRE_W;
///Field `MCO2` reader - Microcontroller clock output 2
pub type MCO2_R = crate::FieldReader<u8, MCO2_A>;
///Microcontroller clock output 2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO2_A {
    ///0: System clock (SYSCLK) selected
    Sysclk = 0,
    ///1: PLLI2S clock selected
    Plli2s = 1,
    ///2: HSE oscillator clock selected
    Hse = 2,
    ///3: PLL clock selected
    Pll = 3,
}
impl From<MCO2_A> for u8 {
    #[inline(always)]
    fn from(variant: MCO2_A) -> Self {
        variant as _
    }
}
impl MCO2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MCO2_A {
        match self.bits {
            0 => MCO2_A::Sysclk,
            1 => MCO2_A::Plli2s,
            2 => MCO2_A::Hse,
            3 => MCO2_A::Pll,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Sysclk`
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == MCO2_A::Sysclk
    }
    ///Checks if the value of the field is `Plli2s`
    #[inline(always)]
    pub fn is_plli2s(&self) -> bool {
        *self == MCO2_A::Plli2s
    }
    ///Checks if the value of the field is `Hse`
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCO2_A::Hse
    }
    ///Checks if the value of the field is `Pll`
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == MCO2_A::Pll
    }
}
///Field `MCO2` writer - Microcontroller clock output 2
pub type MCO2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, MCO2_A, 2, O>;
impl<'a, const O: u8> MCO2_W<'a, O> {
    ///System clock (SYSCLK) selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(MCO2_A::Sysclk)
    }
    ///PLLI2S clock selected
    #[inline(always)]
    pub fn plli2s(self) -> &'a mut W {
        self.variant(MCO2_A::Plli2s)
    }
    ///HSE oscillator clock selected
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(MCO2_A::Hse)
    }
    ///PLL clock selected
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(MCO2_A::Pll)
    }
}
impl R {
    ///Bits 0:1 - System clock switch
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - System clock switch status
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - AHB prescaler
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bit 8 - MCO output enable
    #[inline(always)]
    pub fn mco1en(&self) -> MCO1EN_R {
        MCO1EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - MCO output enable
    #[inline(always)]
    pub fn mco2en(&self) -> MCO2EN_R {
        MCO2EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:12 - APB Low speed prescaler (APB1)
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 10) & 7) as u8)
    }
    ///Bits 13:15 - APB high-speed prescaler (APB2)
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 16:20 - HSE division factor for RTC clock
    #[inline(always)]
    pub fn rtcpre(&self) -> RTCPRE_R {
        RTCPRE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 21:22 - Microcontroller clock output 1
    #[inline(always)]
    pub fn mco1(&self) -> MCO1_R {
        MCO1_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bits 24:26 - MCO1 prescaler
    #[inline(always)]
    pub fn mco1pre(&self) -> MCO1PRE_R {
        MCO1PRE_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:29 - MCO2 prescaler
    #[inline(always)]
    pub fn mco2pre(&self) -> MCO2PRE_R {
        MCO2PRE_R::new(((self.bits >> 27) & 7) as u8)
    }
    ///Bits 30:31 - Microcontroller clock output 2
    #[inline(always)]
    pub fn mco2(&self) -> MCO2_R {
        MCO2_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - System clock switch
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<0> {
        SW_W::new(self)
    }
    ///Bits 2:3 - System clock switch status
    #[inline(always)]
    #[must_use]
    pub fn sws(&mut self) -> SWS_W<2> {
        SWS_W::new(self)
    }
    ///Bits 4:7 - AHB prescaler
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HPRE_W<4> {
        HPRE_W::new(self)
    }
    ///Bit 8 - MCO output enable
    #[inline(always)]
    #[must_use]
    pub fn mco1en(&mut self) -> MCO1EN_W<8> {
        MCO1EN_W::new(self)
    }
    ///Bit 9 - MCO output enable
    #[inline(always)]
    #[must_use]
    pub fn mco2en(&mut self) -> MCO2EN_W<9> {
        MCO2EN_W::new(self)
    }
    ///Bits 10:12 - APB Low speed prescaler (APB1)
    #[inline(always)]
    #[must_use]
    pub fn ppre1(&mut self) -> PPRE1_W<10> {
        PPRE1_W::new(self)
    }
    ///Bits 13:15 - APB high-speed prescaler (APB2)
    #[inline(always)]
    #[must_use]
    pub fn ppre2(&mut self) -> PPRE2_W<13> {
        PPRE2_W::new(self)
    }
    ///Bits 16:20 - HSE division factor for RTC clock
    #[inline(always)]
    #[must_use]
    pub fn rtcpre(&mut self) -> RTCPRE_W<16> {
        RTCPRE_W::new(self)
    }
    ///Bits 21:22 - Microcontroller clock output 1
    #[inline(always)]
    #[must_use]
    pub fn mco1(&mut self) -> MCO1_W<21> {
        MCO1_W::new(self)
    }
    ///Bits 24:26 - MCO1 prescaler
    #[inline(always)]
    #[must_use]
    pub fn mco1pre(&mut self) -> MCO1PRE_W<24> {
        MCO1PRE_W::new(self)
    }
    ///Bits 27:29 - MCO2 prescaler
    #[inline(always)]
    #[must_use]
    pub fn mco2pre(&mut self) -> MCO2PRE_W<27> {
        MCO2PRE_W::new(self)
    }
    ///Bits 30:31 - Microcontroller clock output 2
    #[inline(always)]
    #[must_use]
    pub fn mco2(&mut self) -> MCO2_W<30> {
        MCO2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///clock configuration register
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
