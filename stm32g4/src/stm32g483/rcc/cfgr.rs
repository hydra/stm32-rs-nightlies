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
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SW_A {
    ///0: MSI selected as system clock
    Msi = 0,
    ///1: HSI selected as system clock
    Hsi = 1,
    ///2: HSE selected as system clock
    Hse = 2,
    ///3: PLL selected as system clock
    Pll = 3,
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
            0 => SW_A::Msi,
            1 => SW_A::Hsi,
            2 => SW_A::Hse,
            3 => SW_A::Pll,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Msi`
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == SW_A::Msi
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
pub type SW_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, SW_A, 2, O>;
impl<'a, const O: u8> SW_W<'a, O> {
    ///MSI selected as system clock
    #[inline(always)]
    pub fn msi(self) -> &'a mut W {
        self.variant(SW_A::Msi)
    }
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
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWSR_A {
    ///0: MSI oscillator used as system clock
    Msi = 0,
    ///1: HSI oscillator used as system clock
    Hsi = 1,
    ///2: HSE used as system clock
    Hse = 2,
    ///3: PLL used as system clock
    Pll = 3,
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
            0 => SWSR_A::Msi,
            1 => SWSR_A::Hsi,
            2 => SWSR_A::Hse,
            3 => SWSR_A::Pll,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Msi`
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == SWSR_A::Msi
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
///Field `PPRE1` reader - PB low-speed prescaler (APB1)
pub type PPRE1_R = crate::FieldReader<u8, PPRE1_A>;
///PB low-speed prescaler (APB1)
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
///Field `PPRE1` writer - PB low-speed prescaler (APB1)
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
///Field `MCOSEL` reader - Microcontroller clock output
pub type MCOSEL_R = crate::FieldReader<u8, MCOSEL_A>;
///Microcontroller clock output
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCOSEL_A {
    ///0: MCO output disabled, no clock on MCO
    None = 0,
    ///1: SYSCLK system clock selected
    Sysclk = 1,
    ///2: MSI clock selected
    Msi = 2,
    ///3: HSI clock selected
    Hsi = 3,
    ///4: HSE clock selected
    Hse = 4,
    ///5: Main PLL clock selected
    Pll = 5,
    ///6: LSI clock selected
    Lsi = 6,
    ///7: LSE clock selected
    Lse = 7,
    ///8: Internal HSI48 clock selected
    Hsi48 = 8,
}
impl From<MCOSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MCOSEL_A) -> Self {
        variant as _
    }
}
impl MCOSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<MCOSEL_A> {
        match self.bits {
            0 => Some(MCOSEL_A::None),
            1 => Some(MCOSEL_A::Sysclk),
            2 => Some(MCOSEL_A::Msi),
            3 => Some(MCOSEL_A::Hsi),
            4 => Some(MCOSEL_A::Hse),
            5 => Some(MCOSEL_A::Pll),
            6 => Some(MCOSEL_A::Lsi),
            7 => Some(MCOSEL_A::Lse),
            8 => Some(MCOSEL_A::Hsi48),
            _ => None,
        }
    }
    ///Checks if the value of the field is `None`
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == MCOSEL_A::None
    }
    ///Checks if the value of the field is `Sysclk`
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == MCOSEL_A::Sysclk
    }
    ///Checks if the value of the field is `Msi`
    #[inline(always)]
    pub fn is_msi(&self) -> bool {
        *self == MCOSEL_A::Msi
    }
    ///Checks if the value of the field is `Hsi`
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == MCOSEL_A::Hsi
    }
    ///Checks if the value of the field is `Hse`
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCOSEL_A::Hse
    }
    ///Checks if the value of the field is `Pll`
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == MCOSEL_A::Pll
    }
    ///Checks if the value of the field is `Lsi`
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == MCOSEL_A::Lsi
    }
    ///Checks if the value of the field is `Lse`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == MCOSEL_A::Lse
    }
    ///Checks if the value of the field is `Hsi48`
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == MCOSEL_A::Hsi48
    }
}
///Field `MCOSEL` writer - Microcontroller clock output
pub type MCOSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, MCOSEL_A, 4, O>;
impl<'a, const O: u8> MCOSEL_W<'a, O> {
    ///MCO output disabled, no clock on MCO
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(MCOSEL_A::None)
    }
    ///SYSCLK system clock selected
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(MCOSEL_A::Sysclk)
    }
    ///MSI clock selected
    #[inline(always)]
    pub fn msi(self) -> &'a mut W {
        self.variant(MCOSEL_A::Msi)
    }
    ///HSI clock selected
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(MCOSEL_A::Hsi)
    }
    ///HSE clock selected
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(MCOSEL_A::Hse)
    }
    ///Main PLL clock selected
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(MCOSEL_A::Pll)
    }
    ///LSI clock selected
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(MCOSEL_A::Lsi)
    }
    ///LSE clock selected
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(MCOSEL_A::Lse)
    }
    ///Internal HSI48 clock selected
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut W {
        self.variant(MCOSEL_A::Hsi48)
    }
}
///Field `MCOPRE` reader - Microcontroller clock output prescaler
pub type MCOPRE_R = crate::FieldReader<u8, MCOPRE_A>;
///Microcontroller clock output prescaler
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCOPRE_A {
    ///0: MCO divided by 1
    Div1 = 0,
    ///1: MCO divided by 2
    Div2 = 1,
    ///2: MCO divided by 4
    Div4 = 2,
    ///3: MCO divided by 8
    Div8 = 3,
    ///4: MCO divided by 16
    Div16 = 4,
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
    pub fn variant(&self) -> Option<MCOPRE_A> {
        match self.bits {
            0 => Some(MCOPRE_A::Div1),
            1 => Some(MCOPRE_A::Div2),
            2 => Some(MCOPRE_A::Div4),
            3 => Some(MCOPRE_A::Div8),
            4 => Some(MCOPRE_A::Div16),
            _ => None,
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
}
///Field `MCOPRE` writer - Microcontroller clock output prescaler
pub type MCOPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, MCOPRE_A, 3, O>;
impl<'a, const O: u8> MCOPRE_W<'a, O> {
    ///MCO divided by 1
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div1)
    }
    ///MCO divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div2)
    }
    ///MCO divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div4)
    }
    ///MCO divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div8)
    }
    ///MCO divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(MCOPRE_A::Div16)
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
    ///Bits 8:10 - PB low-speed prescaler (APB1)
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:13 - APB high-speed prescaler (APB2)
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 11) & 7) as u8)
    }
    ///Bits 24:27 - Microcontroller clock output
    #[inline(always)]
    pub fn mcosel(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:30 - Microcontroller clock output prescaler
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    ///Bits 0:1 - System clock switch
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
    ///Bits 8:10 - PB low-speed prescaler (APB1)
    #[inline(always)]
    #[must_use]
    pub fn ppre1(&mut self) -> PPRE1_W<8> {
        PPRE1_W::new(self)
    }
    ///Bits 11:13 - APB high-speed prescaler (APB2)
    #[inline(always)]
    #[must_use]
    pub fn ppre2(&mut self) -> PPRE2_W<11> {
        PPRE2_W::new(self)
    }
    ///Bits 24:27 - Microcontroller clock output
    #[inline(always)]
    #[must_use]
    pub fn mcosel(&mut self) -> MCOSEL_W<24> {
        MCOSEL_W::new(self)
    }
    ///Bits 28:30 - Microcontroller clock output prescaler
    #[inline(always)]
    #[must_use]
    pub fn mcopre(&mut self) -> MCOPRE_W<28> {
        MCOPRE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Clock configuration register
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
///`reset()` method sets CFGR to value 0x05
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
