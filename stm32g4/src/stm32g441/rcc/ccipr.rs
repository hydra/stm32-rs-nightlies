///Register `CCIPR` reader
pub struct R(crate::R<CCIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCIPR` writer
pub struct W(crate::W<CCIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR_SPEC>;
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
impl From<crate::W<CCIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `USART1SEL` reader - USART1 clock source selection
pub type USART1SEL_R = crate::FieldReader<u8, u8>;
///Field `USART1SEL` writer - USART1 clock source selection
pub type USART1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, u8, 2, O>;
///Field `USART2SEL` reader - USART2 clock source selection
pub type USART2SEL_R = crate::FieldReader<u8, u8>;
///Field `USART2SEL` writer - USART2 clock source selection
pub type USART2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, u8, 2, O>;
///Field `USART3SEL` reader - USART3 clock source selection
pub type USART3SEL_R = crate::FieldReader<u8, u8>;
///Field `USART3SEL` writer - USART3 clock source selection
pub type USART3SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, u8, 2, O>;
///Field `UART4SEL` reader - UART4 clock source selection
pub type UART4SEL_R = crate::FieldReader<u8, UART4SEL_A>;
///UART4 clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UART4SEL_A {
    ///0: PCLK clock selected as UART clock
    Pclk = 0,
    ///1: System clock (SYSCLK) selected as UART clock
    System = 1,
    ///2: HSI16 clock selected as UART clock
    Hsi16 = 2,
    ///3: LSE clock selected as UART clock
    Lse = 3,
}
impl From<UART4SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UART4SEL_A) -> Self {
        variant as _
    }
}
impl UART4SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UART4SEL_A {
        match self.bits {
            0 => UART4SEL_A::Pclk,
            1 => UART4SEL_A::System,
            2 => UART4SEL_A::Hsi16,
            3 => UART4SEL_A::Lse,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Pclk`
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == UART4SEL_A::Pclk
    }
    ///Checks if the value of the field is `System`
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == UART4SEL_A::System
    }
    ///Checks if the value of the field is `Hsi16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == UART4SEL_A::Hsi16
    }
    ///Checks if the value of the field is `Lse`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == UART4SEL_A::Lse
    }
}
///Field `UART4SEL` writer - UART4 clock source selection
pub type UART4SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCIPR_SPEC, u8, UART4SEL_A, 2, O>;
impl<'a, const O: u8> UART4SEL_W<'a, O> {
    ///PCLK clock selected as UART clock
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(UART4SEL_A::Pclk)
    }
    ///System clock (SYSCLK) selected as UART clock
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(UART4SEL_A::System)
    }
    ///HSI16 clock selected as UART clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(UART4SEL_A::Hsi16)
    }
    ///LSE clock selected as UART clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(UART4SEL_A::Lse)
    }
}
///Field `UART5SEL` reader - UART5 clock source selection
pub use UART4SEL_R as UART5SEL_R;
///Field `LPUART1SEL` reader - LPUART1 clock source selection
pub use UART4SEL_R as LPUART1SEL_R;
///Field `UART5SEL` writer - UART5 clock source selection
pub use UART4SEL_W as UART5SEL_W;
///Field `LPUART1SEL` writer - LPUART1 clock source selection
pub use UART4SEL_W as LPUART1SEL_W;
///Field `I2C1SEL` reader - I2C1 clock source selection
pub type I2C1SEL_R = crate::FieldReader<u8, I2C1SEL_A>;
///I2C1 clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C1SEL_A {
    ///0: PCLK clock selected as I2C clock
    Pclk = 0,
    ///1: System clock (SYSCLK) selected as I2C clock
    System = 1,
    ///2: HSI16 clock selected as I2C clock
    Hsi16 = 2,
}
impl From<I2C1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C1SEL_A) -> Self {
        variant as _
    }
}
impl I2C1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<I2C1SEL_A> {
        match self.bits {
            0 => Some(I2C1SEL_A::Pclk),
            1 => Some(I2C1SEL_A::System),
            2 => Some(I2C1SEL_A::Hsi16),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Pclk`
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == I2C1SEL_A::Pclk
    }
    ///Checks if the value of the field is `System`
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == I2C1SEL_A::System
    }
    ///Checks if the value of the field is `Hsi16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == I2C1SEL_A::Hsi16
    }
}
///Field `I2C1SEL` writer - I2C1 clock source selection
pub type I2C1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, I2C1SEL_A, 2, O>;
impl<'a, const O: u8> I2C1SEL_W<'a, O> {
    ///PCLK clock selected as I2C clock
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(I2C1SEL_A::Pclk)
    }
    ///System clock (SYSCLK) selected as I2C clock
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(I2C1SEL_A::System)
    }
    ///HSI16 clock selected as I2C clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(I2C1SEL_A::Hsi16)
    }
}
///Field `I2C2SEL` reader - I2C2 clock source selection
pub use I2C1SEL_R as I2C2SEL_R;
///Field `I2C3SEL` reader - I2C3 clock source selection
pub use I2C1SEL_R as I2C3SEL_R;
///Field `I2C2SEL` writer - I2C2 clock source selection
pub use I2C1SEL_W as I2C2SEL_W;
///Field `I2C3SEL` writer - I2C3 clock source selection
pub use I2C1SEL_W as I2C3SEL_W;
///Field `LPTIM1SEL` reader - Low power timer 1 clock source selection
pub type LPTIM1SEL_R = crate::FieldReader<u8, LPTIM1SEL_A>;
///Low power timer 1 clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM1SEL_A {
    ///0: PCLK clock selected as LPTIM1 clock
    Pclk = 0,
    ///1: LSI clock selected as LPTIM1 clock
    Lsi = 1,
    ///2: HSI16 clock selected as LPTIM1 clock
    Hsi16 = 2,
    ///3: LSE clock selected as LPTIM1 clock
    Lse = 3,
}
impl From<LPTIM1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM1SEL_A) -> Self {
        variant as _
    }
}
impl LPTIM1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LPTIM1SEL_A {
        match self.bits {
            0 => LPTIM1SEL_A::Pclk,
            1 => LPTIM1SEL_A::Lsi,
            2 => LPTIM1SEL_A::Hsi16,
            3 => LPTIM1SEL_A::Lse,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Pclk`
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == LPTIM1SEL_A::Pclk
    }
    ///Checks if the value of the field is `Lsi`
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL_A::Lsi
    }
    ///Checks if the value of the field is `Hsi16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == LPTIM1SEL_A::Hsi16
    }
    ///Checks if the value of the field is `Lse`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM1SEL_A::Lse
    }
}
///Field `LPTIM1SEL` writer - Low power timer 1 clock source selection
pub type LPTIM1SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCIPR_SPEC, u8, LPTIM1SEL_A, 2, O>;
impl<'a, const O: u8> LPTIM1SEL_W<'a, O> {
    ///PCLK clock selected as LPTIM1 clock
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Pclk)
    }
    ///LSI clock selected as LPTIM1 clock
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Lsi)
    }
    ///HSI16 clock selected as LPTIM1 clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Hsi16)
    }
    ///LSE clock selected as LPTIM1 clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Lse)
    }
}
///Field `SAI1SEL` reader - Low power timer 2 clock source selection
pub type SAI1SEL_R = crate::FieldReader<u8, SAI1SEL_A>;
///Low power timer 2 clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI1SEL_A {
    ///0: System clock selected as SAI clock
    System = 0,
    ///1: PLL 'Q' clock selected as SAI clock
    Pllq = 1,
    ///2: Clock provided on I2S_CKIN pin is selected as SAI clock
    I2sCkin = 2,
    ///3: HSI16 clock selected as SAI clock
    Hsi16 = 3,
}
impl From<SAI1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI1SEL_A) -> Self {
        variant as _
    }
}
impl SAI1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SAI1SEL_A {
        match self.bits {
            0 => SAI1SEL_A::System,
            1 => SAI1SEL_A::Pllq,
            2 => SAI1SEL_A::I2sCkin,
            3 => SAI1SEL_A::Hsi16,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `System`
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == SAI1SEL_A::System
    }
    ///Checks if the value of the field is `Pllq`
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        *self == SAI1SEL_A::Pllq
    }
    ///Checks if the value of the field is `I2sCkin`
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == SAI1SEL_A::I2sCkin
    }
    ///Checks if the value of the field is `Hsi16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == SAI1SEL_A::Hsi16
    }
}
///Field `SAI1SEL` writer - Low power timer 2 clock source selection
pub type SAI1SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCIPR_SPEC, u8, SAI1SEL_A, 2, O>;
impl<'a, const O: u8> SAI1SEL_W<'a, O> {
    ///System clock selected as SAI clock
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(SAI1SEL_A::System)
    }
    ///PLL 'Q' clock selected as SAI clock
    #[inline(always)]
    pub fn pllq(self) -> &'a mut W {
        self.variant(SAI1SEL_A::Pllq)
    }
    ///Clock provided on I2S_CKIN pin is selected as SAI clock
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(SAI1SEL_A::I2sCkin)
    }
    ///HSI16 clock selected as SAI clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(SAI1SEL_A::Hsi16)
    }
}
///Field `I2S23SEL` reader - SAI1 clock source selection
pub type I2S23SEL_R = crate::FieldReader<u8, I2S23SEL_A>;
///SAI1 clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2S23SEL_A {
    ///0: System clock selected as I2S23 clock
    System = 0,
    ///1: PLL 'Q' clock selected as I2S23 clock
    Pllq = 1,
    ///2: Clock provided on I2S_CKIN pin is selected as I2S23 clock
    I2sCkin = 2,
    ///3: HSI16 clock selected as I2S23 clock
    Hsi16 = 3,
}
impl From<I2S23SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2S23SEL_A) -> Self {
        variant as _
    }
}
impl I2S23SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2S23SEL_A {
        match self.bits {
            0 => I2S23SEL_A::System,
            1 => I2S23SEL_A::Pllq,
            2 => I2S23SEL_A::I2sCkin,
            3 => I2S23SEL_A::Hsi16,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `System`
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == I2S23SEL_A::System
    }
    ///Checks if the value of the field is `Pllq`
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        *self == I2S23SEL_A::Pllq
    }
    ///Checks if the value of the field is `I2sCkin`
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == I2S23SEL_A::I2sCkin
    }
    ///Checks if the value of the field is `Hsi16`
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == I2S23SEL_A::Hsi16
    }
}
///Field `I2S23SEL` writer - SAI1 clock source selection
pub type I2S23SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CCIPR_SPEC, u8, I2S23SEL_A, 2, O>;
impl<'a, const O: u8> I2S23SEL_W<'a, O> {
    ///System clock selected as I2S23 clock
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(I2S23SEL_A::System)
    }
    ///PLL 'Q' clock selected as I2S23 clock
    #[inline(always)]
    pub fn pllq(self) -> &'a mut W {
        self.variant(I2S23SEL_A::Pllq)
    }
    ///Clock provided on I2S_CKIN pin is selected as I2S23 clock
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(I2S23SEL_A::I2sCkin)
    }
    ///HSI16 clock selected as I2S23 clock
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(I2S23SEL_A::Hsi16)
    }
}
///Field `FDCANSEL` reader - SAI2 clock source selection
pub type FDCANSEL_R = crate::FieldReader<u8, FDCANSEL_A>;
///SAI2 clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FDCANSEL_A {
    ///0: HSE clock selected as FDCAN clock
    Hse = 0,
    ///1: PLL 'Q' clock selected as FDCAN clock
    Pllq = 1,
    ///2: PCLK clock selected as FDCAN clock
    Pclk = 2,
}
impl From<FDCANSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FDCANSEL_A) -> Self {
        variant as _
    }
}
impl FDCANSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<FDCANSEL_A> {
        match self.bits {
            0 => Some(FDCANSEL_A::Hse),
            1 => Some(FDCANSEL_A::Pllq),
            2 => Some(FDCANSEL_A::Pclk),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Hse`
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == FDCANSEL_A::Hse
    }
    ///Checks if the value of the field is `Pllq`
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        *self == FDCANSEL_A::Pllq
    }
    ///Checks if the value of the field is `Pclk`
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == FDCANSEL_A::Pclk
    }
}
///Field `FDCANSEL` writer - SAI2 clock source selection
pub type FDCANSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, FDCANSEL_A, 2, O>;
impl<'a, const O: u8> FDCANSEL_W<'a, O> {
    ///HSE clock selected as FDCAN clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(FDCANSEL_A::Hse)
    }
    ///PLL 'Q' clock selected as FDCAN clock
    #[inline(always)]
    pub fn pllq(self) -> &'a mut W {
        self.variant(FDCANSEL_A::Pllq)
    }
    ///PCLK clock selected as FDCAN clock
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(FDCANSEL_A::Pclk)
    }
}
///Field `CLK48SEL` reader - 48 MHz clock source selection
pub type CLK48SEL_R = crate::FieldReader<u8, CLK48SEL_A>;
///48 MHz clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK48SEL_A {
    ///0: HSI48 clock selected as 48MHz clock
    Hsi48 = 0,
    ///2: PLL 'Q' (PLL48M1CLK) clock selected as 48MHz clock
    Pllq = 2,
}
impl From<CLK48SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK48SEL_A) -> Self {
        variant as _
    }
}
impl CLK48SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CLK48SEL_A> {
        match self.bits {
            0 => Some(CLK48SEL_A::Hsi48),
            2 => Some(CLK48SEL_A::Pllq),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Hsi48`
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == CLK48SEL_A::Hsi48
    }
    ///Checks if the value of the field is `Pllq`
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        *self == CLK48SEL_A::Pllq
    }
}
///Field `CLK48SEL` writer - 48 MHz clock source selection
pub type CLK48SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, CLK48SEL_A, 2, O>;
impl<'a, const O: u8> CLK48SEL_W<'a, O> {
    ///HSI48 clock selected as 48MHz clock
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut W {
        self.variant(CLK48SEL_A::Hsi48)
    }
    ///PLL 'Q' (PLL48M1CLK) clock selected as 48MHz clock
    #[inline(always)]
    pub fn pllq(self) -> &'a mut W {
        self.variant(CLK48SEL_A::Pllq)
    }
}
///Field `ADC12SEL` reader - ADCs clock source selection
pub type ADC12SEL_R = crate::FieldReader<u8, ADC12SEL_A>;
///ADCs clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC12SEL_A {
    ///0: No clock selected for ADC
    None = 0,
    ///1: PLL 'P' clock selected for ADC
    Pllp = 1,
    ///2: System clock selected for ADC
    System = 2,
}
impl From<ADC12SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12SEL_A) -> Self {
        variant as _
    }
}
impl ADC12SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC12SEL_A> {
        match self.bits {
            0 => Some(ADC12SEL_A::None),
            1 => Some(ADC12SEL_A::Pllp),
            2 => Some(ADC12SEL_A::System),
            _ => None,
        }
    }
    ///Checks if the value of the field is `None`
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ADC12SEL_A::None
    }
    ///Checks if the value of the field is `Pllp`
    #[inline(always)]
    pub fn is_pllp(&self) -> bool {
        *self == ADC12SEL_A::Pllp
    }
    ///Checks if the value of the field is `System`
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        *self == ADC12SEL_A::System
    }
}
///Field `ADC12SEL` writer - ADCs clock source selection
pub type ADC12SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CCIPR_SPEC, u8, ADC12SEL_A, 2, O>;
impl<'a, const O: u8> ADC12SEL_W<'a, O> {
    ///No clock selected for ADC
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ADC12SEL_A::None)
    }
    ///PLL 'P' clock selected for ADC
    #[inline(always)]
    pub fn pllp(self) -> &'a mut W {
        self.variant(ADC12SEL_A::Pllp)
    }
    ///System clock selected for ADC
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(ADC12SEL_A::System)
    }
}
///Field `ADC345SEL` reader - ADC3/4/5 clock source selection
pub use ADC12SEL_R as ADC345SEL_R;
///Field `ADC345SEL` writer - ADC3/4/5 clock source selection
pub use ADC12SEL_W as ADC345SEL_W;
impl R {
    ///Bits 0:1 - USART1 clock source selection
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - USART2 clock source selection
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - USART3 clock source selection
    #[inline(always)]
    pub fn usart3sel(&self) -> USART3SEL_R {
        USART3SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - UART4 clock source selection
    #[inline(always)]
    pub fn uart4sel(&self) -> UART4SEL_R {
        UART4SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - UART5 clock source selection
    #[inline(always)]
    pub fn uart5sel(&self) -> UART5SEL_R {
        UART5SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - LPUART1 clock source selection
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - I2C1 clock source selection
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - I2C2 clock source selection
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2C2SEL_R {
        I2C2SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - I2C3 clock source selection
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Low power timer 1 clock source selection
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Low power timer 2 clock source selection
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - SAI1 clock source selection
    #[inline(always)]
    pub fn i2s23sel(&self) -> I2S23SEL_R {
        I2S23SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - SAI2 clock source selection
    #[inline(always)]
    pub fn fdcansel(&self) -> FDCANSEL_R {
        FDCANSEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - 48 MHz clock source selection
    #[inline(always)]
    pub fn clk48sel(&self) -> CLK48SEL_R {
        CLK48SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - ADCs clock source selection
    #[inline(always)]
    pub fn adc12sel(&self) -> ADC12SEL_R {
        ADC12SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - ADC3/4/5 clock source selection
    #[inline(always)]
    pub fn adc345sel(&self) -> ADC345SEL_R {
        ADC345SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - USART1 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn usart1sel(&mut self) -> USART1SEL_W<0> {
        USART1SEL_W::new(self)
    }
    ///Bits 2:3 - USART2 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn usart2sel(&mut self) -> USART2SEL_W<2> {
        USART2SEL_W::new(self)
    }
    ///Bits 4:5 - USART3 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn usart3sel(&mut self) -> USART3SEL_W<4> {
        USART3SEL_W::new(self)
    }
    ///Bits 6:7 - UART4 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn uart4sel(&mut self) -> UART4SEL_W<6> {
        UART4SEL_W::new(self)
    }
    ///Bits 8:9 - UART5 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn uart5sel(&mut self) -> UART5SEL_W<8> {
        UART5SEL_W::new(self)
    }
    ///Bits 10:11 - LPUART1 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<10> {
        LPUART1SEL_W::new(self)
    }
    ///Bits 12:13 - I2C1 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<12> {
        I2C1SEL_W::new(self)
    }
    ///Bits 14:15 - I2C2 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn i2c2sel(&mut self) -> I2C2SEL_W<14> {
        I2C2SEL_W::new(self)
    }
    ///Bits 16:17 - I2C3 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W<16> {
        I2C3SEL_W::new(self)
    }
    ///Bits 18:19 - Low power timer 1 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<18> {
        LPTIM1SEL_W::new(self)
    }
    ///Bits 20:21 - Low power timer 2 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn sai1sel(&mut self) -> SAI1SEL_W<20> {
        SAI1SEL_W::new(self)
    }
    ///Bits 22:23 - SAI1 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn i2s23sel(&mut self) -> I2S23SEL_W<22> {
        I2S23SEL_W::new(self)
    }
    ///Bits 24:25 - SAI2 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn fdcansel(&mut self) -> FDCANSEL_W<24> {
        FDCANSEL_W::new(self)
    }
    ///Bits 26:27 - 48 MHz clock source selection
    #[inline(always)]
    #[must_use]
    pub fn clk48sel(&mut self) -> CLK48SEL_W<26> {
        CLK48SEL_W::new(self)
    }
    ///Bits 28:29 - ADCs clock source selection
    #[inline(always)]
    #[must_use]
    pub fn adc12sel(&mut self) -> ADC12SEL_W<28> {
        ADC12SEL_W::new(self)
    }
    ///Bits 30:31 - ADC3/4/5 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn adc345sel(&mut self) -> ADC345SEL_W<30> {
        ADC345SEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CCIPR
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccipr](index.html) module
pub struct CCIPR_SPEC;
impl crate::RegisterSpec for CCIPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccipr::R](R) reader structure
impl crate::Readable for CCIPR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccipr::W](W) writer structure
impl crate::Writable for CCIPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCIPR to value 0
impl crate::Resettable for CCIPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
