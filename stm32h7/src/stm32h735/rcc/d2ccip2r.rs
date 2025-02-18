///Register `D2CCIP2R` reader
pub struct R(crate::R<D2CCIP2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D2CCIP2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D2CCIP2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D2CCIP2R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `D2CCIP2R` writer
pub struct W(crate::W<D2CCIP2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D2CCIP2R_SPEC>;
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
impl From<crate::W<D2CCIP2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D2CCIP2R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `USART234578SEL` reader - USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection
pub type USART234578SEL_R = crate::FieldReader<u8, USART234578SEL_A>;
///USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART234578SEL_A {
    ///0: rcc_pclk1 selected as peripheral clock
    RccPclk1 = 0,
    ///1: pll2_q selected as peripheral clock
    Pll2Q = 1,
    ///2: pll3_q selected as peripheral clock
    Pll3Q = 2,
    ///3: hsi_ker selected as peripheral clock
    HsiKer = 3,
    ///4: csi_ker selected as peripheral clock
    CsiKer = 4,
    ///5: LSE selected as peripheral clock
    Lse = 5,
}
impl From<USART234578SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USART234578SEL_A) -> Self {
        variant as _
    }
}
impl USART234578SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<USART234578SEL_A> {
        match self.bits {
            0 => Some(USART234578SEL_A::RccPclk1),
            1 => Some(USART234578SEL_A::Pll2Q),
            2 => Some(USART234578SEL_A::Pll3Q),
            3 => Some(USART234578SEL_A::HsiKer),
            4 => Some(USART234578SEL_A::CsiKer),
            5 => Some(USART234578SEL_A::Lse),
            _ => None,
        }
    }
    ///Checks if the value of the field is `RccPclk1`
    #[inline(always)]
    pub fn is_rcc_pclk1(&self) -> bool {
        *self == USART234578SEL_A::RccPclk1
    }
    ///Checks if the value of the field is `Pll2Q`
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == USART234578SEL_A::Pll2Q
    }
    ///Checks if the value of the field is `Pll3Q`
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == USART234578SEL_A::Pll3Q
    }
    ///Checks if the value of the field is `HsiKer`
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == USART234578SEL_A::HsiKer
    }
    ///Checks if the value of the field is `CsiKer`
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == USART234578SEL_A::CsiKer
    }
    ///Checks if the value of the field is `Lse`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART234578SEL_A::Lse
    }
}
///Field `USART234578SEL` writer - USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection
pub type USART234578SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D2CCIP2R_SPEC, u8, USART234578SEL_A, 3, O>;
impl<'a, const O: u8> USART234578SEL_W<'a, O> {
    ///rcc_pclk1 selected as peripheral clock
    #[inline(always)]
    pub fn rcc_pclk1(self) -> &'a mut W {
        self.variant(USART234578SEL_A::RccPclk1)
    }
    ///pll2_q selected as peripheral clock
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut W {
        self.variant(USART234578SEL_A::Pll2Q)
    }
    ///pll3_q selected as peripheral clock
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut W {
        self.variant(USART234578SEL_A::Pll3Q)
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(USART234578SEL_A::HsiKer)
    }
    ///csi_ker selected as peripheral clock
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut W {
        self.variant(USART234578SEL_A::CsiKer)
    }
    ///LSE selected as peripheral clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART234578SEL_A::Lse)
    }
}
///Field `USART16910SEL` reader - USART1 and 6 kernel clock source selection
pub type USART16910SEL_R = crate::FieldReader<u8, USART16910SEL_A>;
///USART1 and 6 kernel clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART16910SEL_A {
    ///0: rcc_pclk2 selected as peripheral clock
    RccPclk2 = 0,
    ///1: pll2_q selected as peripheral clock
    Pll2Q = 1,
    ///2: pll3_q selected as peripheral clock
    Pll3Q = 2,
    ///3: hsi_ker selected as peripheral clock
    HsiKer = 3,
    ///4: csi_ker selected as peripheral clock
    CsiKer = 4,
    ///5: LSE selected as peripheral clock
    Lse = 5,
}
impl From<USART16910SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USART16910SEL_A) -> Self {
        variant as _
    }
}
impl USART16910SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<USART16910SEL_A> {
        match self.bits {
            0 => Some(USART16910SEL_A::RccPclk2),
            1 => Some(USART16910SEL_A::Pll2Q),
            2 => Some(USART16910SEL_A::Pll3Q),
            3 => Some(USART16910SEL_A::HsiKer),
            4 => Some(USART16910SEL_A::CsiKer),
            5 => Some(USART16910SEL_A::Lse),
            _ => None,
        }
    }
    ///Checks if the value of the field is `RccPclk2`
    #[inline(always)]
    pub fn is_rcc_pclk2(&self) -> bool {
        *self == USART16910SEL_A::RccPclk2
    }
    ///Checks if the value of the field is `Pll2Q`
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == USART16910SEL_A::Pll2Q
    }
    ///Checks if the value of the field is `Pll3Q`
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == USART16910SEL_A::Pll3Q
    }
    ///Checks if the value of the field is `HsiKer`
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == USART16910SEL_A::HsiKer
    }
    ///Checks if the value of the field is `CsiKer`
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == USART16910SEL_A::CsiKer
    }
    ///Checks if the value of the field is `Lse`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART16910SEL_A::Lse
    }
}
///Field `USART16910SEL` writer - USART1 and 6 kernel clock source selection
pub type USART16910SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D2CCIP2R_SPEC, u8, USART16910SEL_A, 3, O>;
impl<'a, const O: u8> USART16910SEL_W<'a, O> {
    ///rcc_pclk2 selected as peripheral clock
    #[inline(always)]
    pub fn rcc_pclk2(self) -> &'a mut W {
        self.variant(USART16910SEL_A::RccPclk2)
    }
    ///pll2_q selected as peripheral clock
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut W {
        self.variant(USART16910SEL_A::Pll2Q)
    }
    ///pll3_q selected as peripheral clock
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut W {
        self.variant(USART16910SEL_A::Pll3Q)
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(USART16910SEL_A::HsiKer)
    }
    ///csi_ker selected as peripheral clock
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut W {
        self.variant(USART16910SEL_A::CsiKer)
    }
    ///LSE selected as peripheral clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART16910SEL_A::Lse)
    }
}
///Field `RNGSEL` reader - RNG kernel clock source selection
pub type RNGSEL_R = crate::FieldReader<u8, RNGSEL_A>;
///RNG kernel clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RNGSEL_A {
    ///0: HSI48 selected as peripheral clock
    Hsi48 = 0,
    ///1: pll1_q selected as peripheral clock
    Pll1Q = 1,
    ///2: LSE selected as peripheral clock
    Lse = 2,
    ///3: LSI selected as peripheral clock
    Lsi = 3,
}
impl From<RNGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RNGSEL_A) -> Self {
        variant as _
    }
}
impl RNGSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RNGSEL_A {
        match self.bits {
            0 => RNGSEL_A::Hsi48,
            1 => RNGSEL_A::Pll1Q,
            2 => RNGSEL_A::Lse,
            3 => RNGSEL_A::Lsi,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Hsi48`
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == RNGSEL_A::Hsi48
    }
    ///Checks if the value of the field is `Pll1Q`
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == RNGSEL_A::Pll1Q
    }
    ///Checks if the value of the field is `Lse`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == RNGSEL_A::Lse
    }
    ///Checks if the value of the field is `Lsi`
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == RNGSEL_A::Lsi
    }
}
///Field `RNGSEL` writer - RNG kernel clock source selection
pub type RNGSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, D2CCIP2R_SPEC, u8, RNGSEL_A, 2, O>;
impl<'a, const O: u8> RNGSEL_W<'a, O> {
    ///HSI48 selected as peripheral clock
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut W {
        self.variant(RNGSEL_A::Hsi48)
    }
    ///pll1_q selected as peripheral clock
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(RNGSEL_A::Pll1Q)
    }
    ///LSE selected as peripheral clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(RNGSEL_A::Lse)
    }
    ///LSI selected as peripheral clock
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(RNGSEL_A::Lsi)
    }
}
///Field `I2C1235SEL` reader - I2C1,2,3 kernel clock source selection
pub type I2C1235SEL_R = crate::FieldReader<u8, I2C1235SEL_A>;
///I2C1,2,3 kernel clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C1235SEL_A {
    ///0: rcc_pclk1 selected as peripheral clock
    RccPclk1 = 0,
    ///1: pll3_r selected as peripheral clock
    Pll3R = 1,
    ///2: hsi_ker selected as peripheral clock
    HsiKer = 2,
    ///3: csi_ker selected as peripheral clock
    CsiKer = 3,
}
impl From<I2C1235SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C1235SEL_A) -> Self {
        variant as _
    }
}
impl I2C1235SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2C1235SEL_A {
        match self.bits {
            0 => I2C1235SEL_A::RccPclk1,
            1 => I2C1235SEL_A::Pll3R,
            2 => I2C1235SEL_A::HsiKer,
            3 => I2C1235SEL_A::CsiKer,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `RccPclk1`
    #[inline(always)]
    pub fn is_rcc_pclk1(&self) -> bool {
        *self == I2C1235SEL_A::RccPclk1
    }
    ///Checks if the value of the field is `Pll3R`
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == I2C1235SEL_A::Pll3R
    }
    ///Checks if the value of the field is `HsiKer`
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == I2C1235SEL_A::HsiKer
    }
    ///Checks if the value of the field is `CsiKer`
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == I2C1235SEL_A::CsiKer
    }
}
///Field `I2C1235SEL` writer - I2C1,2,3 kernel clock source selection
pub type I2C1235SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, D2CCIP2R_SPEC, u8, I2C1235SEL_A, 2, O>;
impl<'a, const O: u8> I2C1235SEL_W<'a, O> {
    ///rcc_pclk1 selected as peripheral clock
    #[inline(always)]
    pub fn rcc_pclk1(self) -> &'a mut W {
        self.variant(I2C1235SEL_A::RccPclk1)
    }
    ///pll3_r selected as peripheral clock
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut W {
        self.variant(I2C1235SEL_A::Pll3R)
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(I2C1235SEL_A::HsiKer)
    }
    ///csi_ker selected as peripheral clock
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut W {
        self.variant(I2C1235SEL_A::CsiKer)
    }
}
///Field `USBSEL` reader - USBOTG 1 and 2 kernel clock source selection
pub type USBSEL_R = crate::FieldReader<u8, USBSEL_A>;
///USBOTG 1 and 2 kernel clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USBSEL_A {
    ///0: Disable the kernel clock
    Disable = 0,
    ///1: pll1_q selected as peripheral clock
    Pll1Q = 1,
    ///2: pll3_q selected as peripheral clock
    Pll3Q = 2,
    ///3: HSI48 selected as peripheral clock
    Hsi48 = 3,
}
impl From<USBSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USBSEL_A) -> Self {
        variant as _
    }
}
impl USBSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> USBSEL_A {
        match self.bits {
            0 => USBSEL_A::Disable,
            1 => USBSEL_A::Pll1Q,
            2 => USBSEL_A::Pll3Q,
            3 => USBSEL_A::Hsi48,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Disable`
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == USBSEL_A::Disable
    }
    ///Checks if the value of the field is `Pll1Q`
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == USBSEL_A::Pll1Q
    }
    ///Checks if the value of the field is `Pll3Q`
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == USBSEL_A::Pll3Q
    }
    ///Checks if the value of the field is `Hsi48`
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == USBSEL_A::Hsi48
    }
}
///Field `USBSEL` writer - USBOTG 1 and 2 kernel clock source selection
pub type USBSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, D2CCIP2R_SPEC, u8, USBSEL_A, 2, O>;
impl<'a, const O: u8> USBSEL_W<'a, O> {
    ///Disable the kernel clock
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USBSEL_A::Disable)
    }
    ///pll1_q selected as peripheral clock
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(USBSEL_A::Pll1Q)
    }
    ///pll3_q selected as peripheral clock
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut W {
        self.variant(USBSEL_A::Pll3Q)
    }
    ///HSI48 selected as peripheral clock
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut W {
        self.variant(USBSEL_A::Hsi48)
    }
}
///Field `CECSEL` reader - HDMI-CEC kernel clock source selection
pub type CECSEL_R = crate::FieldReader<u8, CECSEL_A>;
///HDMI-CEC kernel clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CECSEL_A {
    ///0: LSE selected as peripheral clock
    Lse = 0,
    ///1: LSI selected as peripheral clock
    Lsi = 1,
    ///2: csi_ker selected as peripheral clock
    CsiKer = 2,
}
impl From<CECSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CECSEL_A) -> Self {
        variant as _
    }
}
impl CECSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CECSEL_A> {
        match self.bits {
            0 => Some(CECSEL_A::Lse),
            1 => Some(CECSEL_A::Lsi),
            2 => Some(CECSEL_A::CsiKer),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Lse`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == CECSEL_A::Lse
    }
    ///Checks if the value of the field is `Lsi`
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == CECSEL_A::Lsi
    }
    ///Checks if the value of the field is `CsiKer`
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == CECSEL_A::CsiKer
    }
}
///Field `CECSEL` writer - HDMI-CEC kernel clock source selection
pub type CECSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, D2CCIP2R_SPEC, u8, CECSEL_A, 2, O>;
impl<'a, const O: u8> CECSEL_W<'a, O> {
    ///LSE selected as peripheral clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(CECSEL_A::Lse)
    }
    ///LSI selected as peripheral clock
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(CECSEL_A::Lsi)
    }
    ///csi_ker selected as peripheral clock
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut W {
        self.variant(CECSEL_A::CsiKer)
    }
}
///Field `LPTIM1SEL` reader - LPTIM1 kernel clock source selection
pub type LPTIM1SEL_R = crate::FieldReader<u8, LPTIM1SEL_A>;
///LPTIM1 kernel clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM1SEL_A {
    ///0: rcc_pclk1 selected as peripheral clock
    RccPclk1 = 0,
    ///1: pll2_p selected as peripheral clock
    Pll2P = 1,
    ///2: pll3_r selected as peripheral clock
    Pll3R = 2,
    ///3: LSE selected as peripheral clock
    Lse = 3,
    ///4: LSI selected as peripheral clock
    Lsi = 4,
    ///5: PER selected as peripheral clock
    Per = 5,
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
    pub fn variant(&self) -> Option<LPTIM1SEL_A> {
        match self.bits {
            0 => Some(LPTIM1SEL_A::RccPclk1),
            1 => Some(LPTIM1SEL_A::Pll2P),
            2 => Some(LPTIM1SEL_A::Pll3R),
            3 => Some(LPTIM1SEL_A::Lse),
            4 => Some(LPTIM1SEL_A::Lsi),
            5 => Some(LPTIM1SEL_A::Per),
            _ => None,
        }
    }
    ///Checks if the value of the field is `RccPclk1`
    #[inline(always)]
    pub fn is_rcc_pclk1(&self) -> bool {
        *self == LPTIM1SEL_A::RccPclk1
    }
    ///Checks if the value of the field is `Pll2P`
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == LPTIM1SEL_A::Pll2P
    }
    ///Checks if the value of the field is `Pll3R`
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == LPTIM1SEL_A::Pll3R
    }
    ///Checks if the value of the field is `Lse`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM1SEL_A::Lse
    }
    ///Checks if the value of the field is `Lsi`
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL_A::Lsi
    }
    ///Checks if the value of the field is `Per`
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == LPTIM1SEL_A::Per
    }
}
///Field `LPTIM1SEL` writer - LPTIM1 kernel clock source selection
pub type LPTIM1SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, D2CCIP2R_SPEC, u8, LPTIM1SEL_A, 3, O>;
impl<'a, const O: u8> LPTIM1SEL_W<'a, O> {
    ///rcc_pclk1 selected as peripheral clock
    #[inline(always)]
    pub fn rcc_pclk1(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::RccPclk1)
    }
    ///pll2_p selected as peripheral clock
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Pll2P)
    }
    ///pll3_r selected as peripheral clock
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Pll3R)
    }
    ///LSE selected as peripheral clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Lse)
    }
    ///LSI selected as peripheral clock
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Lsi)
    }
    ///PER selected as peripheral clock
    #[inline(always)]
    pub fn per(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Per)
    }
}
impl R {
    ///Bits 0:2 - USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection
    #[inline(always)]
    pub fn usart234578sel(&self) -> USART234578SEL_R {
        USART234578SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - USART1 and 6 kernel clock source selection
    #[inline(always)]
    pub fn usart16910sel(&self) -> USART16910SEL_R {
        USART16910SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 8:9 - RNG kernel clock source selection
    #[inline(always)]
    pub fn rngsel(&self) -> RNGSEL_R {
        RNGSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 12:13 - I2C1,2,3 kernel clock source selection
    #[inline(always)]
    pub fn i2c1235sel(&self) -> I2C1235SEL_R {
        I2C1235SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 20:21 - USBOTG 1 and 2 kernel clock source selection
    #[inline(always)]
    pub fn usbsel(&self) -> USBSEL_R {
        USBSEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - HDMI-CEC kernel clock source selection
    #[inline(always)]
    pub fn cecsel(&self) -> CECSEL_R {
        CECSEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 28:30 - LPTIM1 kernel clock source selection
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection
    #[inline(always)]
    #[must_use]
    pub fn usart234578sel(&mut self) -> USART234578SEL_W<0> {
        USART234578SEL_W::new(self)
    }
    ///Bits 3:5 - USART1 and 6 kernel clock source selection
    #[inline(always)]
    #[must_use]
    pub fn usart16910sel(&mut self) -> USART16910SEL_W<3> {
        USART16910SEL_W::new(self)
    }
    ///Bits 8:9 - RNG kernel clock source selection
    #[inline(always)]
    #[must_use]
    pub fn rngsel(&mut self) -> RNGSEL_W<8> {
        RNGSEL_W::new(self)
    }
    ///Bits 12:13 - I2C1,2,3 kernel clock source selection
    #[inline(always)]
    #[must_use]
    pub fn i2c1235sel(&mut self) -> I2C1235SEL_W<12> {
        I2C1235SEL_W::new(self)
    }
    ///Bits 20:21 - USBOTG 1 and 2 kernel clock source selection
    #[inline(always)]
    #[must_use]
    pub fn usbsel(&mut self) -> USBSEL_W<20> {
        USBSEL_W::new(self)
    }
    ///Bits 22:23 - HDMI-CEC kernel clock source selection
    #[inline(always)]
    #[must_use]
    pub fn cecsel(&mut self) -> CECSEL_W<22> {
        CECSEL_W::new(self)
    }
    ///Bits 28:30 - LPTIM1 kernel clock source selection
    #[inline(always)]
    #[must_use]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<28> {
        LPTIM1SEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC Domain 2 Kernel Clock Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [d2ccip2r](index.html) module
pub struct D2CCIP2R_SPEC;
impl crate::RegisterSpec for D2CCIP2R_SPEC {
    type Ux = u32;
}
///`read()` method returns [d2ccip2r::R](R) reader structure
impl crate::Readable for D2CCIP2R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [d2ccip2r::W](W) writer structure
impl crate::Writable for D2CCIP2R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets D2CCIP2R to value 0
impl crate::Resettable for D2CCIP2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
