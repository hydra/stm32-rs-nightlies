///Register `SRDCCIPR` reader
pub struct R(crate::R<SRDCCIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRDCCIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRDCCIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRDCCIPR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SRDCCIPR` writer
pub struct W(crate::W<SRDCCIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRDCCIPR_SPEC>;
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
impl From<crate::W<SRDCCIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRDCCIPR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LPUART1SEL` reader - LPUART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type LPUART1SEL_R = crate::FieldReader<u8, LPUART1SEL_A>;
///LPUART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPUART1SEL_A {
    ///0: rcc_pclk_d3 selected as peripheral clock
    RccPclkD3 = 0,
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
impl From<LPUART1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPUART1SEL_A) -> Self {
        variant as _
    }
}
impl LPUART1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<LPUART1SEL_A> {
        match self.bits {
            0 => Some(LPUART1SEL_A::RccPclkD3),
            1 => Some(LPUART1SEL_A::Pll2Q),
            2 => Some(LPUART1SEL_A::Pll3Q),
            3 => Some(LPUART1SEL_A::HsiKer),
            4 => Some(LPUART1SEL_A::CsiKer),
            5 => Some(LPUART1SEL_A::Lse),
            _ => None,
        }
    }
    ///Checks if the value of the field is `RccPclkD3`
    #[inline(always)]
    pub fn is_rcc_pclk_d3(&self) -> bool {
        *self == LPUART1SEL_A::RccPclkD3
    }
    ///Checks if the value of the field is `Pll2Q`
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == LPUART1SEL_A::Pll2Q
    }
    ///Checks if the value of the field is `Pll3Q`
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == LPUART1SEL_A::Pll3Q
    }
    ///Checks if the value of the field is `HsiKer`
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == LPUART1SEL_A::HsiKer
    }
    ///Checks if the value of the field is `CsiKer`
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == LPUART1SEL_A::CsiKer
    }
    ///Checks if the value of the field is `Lse`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPUART1SEL_A::Lse
    }
}
///Field `LPUART1SEL` writer - LPUART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type LPUART1SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRDCCIPR_SPEC, u8, LPUART1SEL_A, 3, O>;
impl<'a, const O: u8> LPUART1SEL_W<'a, O> {
    ///rcc_pclk_d3 selected as peripheral clock
    #[inline(always)]
    pub fn rcc_pclk_d3(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::RccPclkD3)
    }
    ///pll2_q selected as peripheral clock
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::Pll2Q)
    }
    ///pll3_q selected as peripheral clock
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::Pll3Q)
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::HsiKer)
    }
    ///csi_ker selected as peripheral clock
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::CsiKer)
    }
    ///LSE selected as peripheral clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::Lse)
    }
}
///Field `I2C4SEL` reader - I2C4 kernel clock source selection Set and reset by software.
pub type I2C4SEL_R = crate::FieldReader<u8, I2C4SEL_A>;
///I2C4 kernel clock source selection Set and reset by software.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C4SEL_A {
    ///0: rcc_pclk4 selected as peripheral clock
    RccPclk4 = 0,
    ///1: pll3_r selected as peripheral clock
    Pll3R = 1,
    ///2: hsi_ker selected as peripheral clock
    HsiKer = 2,
    ///3: csi_ker selected as peripheral clock
    CsiKer = 3,
}
impl From<I2C4SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C4SEL_A) -> Self {
        variant as _
    }
}
impl I2C4SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2C4SEL_A {
        match self.bits {
            0 => I2C4SEL_A::RccPclk4,
            1 => I2C4SEL_A::Pll3R,
            2 => I2C4SEL_A::HsiKer,
            3 => I2C4SEL_A::CsiKer,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `RccPclk4`
    #[inline(always)]
    pub fn is_rcc_pclk4(&self) -> bool {
        *self == I2C4SEL_A::RccPclk4
    }
    ///Checks if the value of the field is `Pll3R`
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == I2C4SEL_A::Pll3R
    }
    ///Checks if the value of the field is `HsiKer`
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == I2C4SEL_A::HsiKer
    }
    ///Checks if the value of the field is `CsiKer`
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == I2C4SEL_A::CsiKer
    }
}
///Field `I2C4SEL` writer - I2C4 kernel clock source selection Set and reset by software.
pub type I2C4SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SRDCCIPR_SPEC, u8, I2C4SEL_A, 2, O>;
impl<'a, const O: u8> I2C4SEL_W<'a, O> {
    ///rcc_pclk4 selected as peripheral clock
    #[inline(always)]
    pub fn rcc_pclk4(self) -> &'a mut W {
        self.variant(I2C4SEL_A::RccPclk4)
    }
    ///pll3_r selected as peripheral clock
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut W {
        self.variant(I2C4SEL_A::Pll3R)
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(I2C4SEL_A::HsiKer)
    }
    ///csi_ker selected as peripheral clock
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut W {
        self.variant(I2C4SEL_A::CsiKer)
    }
}
///Field `LPTIM2SEL` reader - LPTIM2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type LPTIM2SEL_R = crate::FieldReader<u8, LPTIM2SEL_A>;
///LPTIM2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM2SEL_A {
    ///0: rcc_pclk4 selected as peripheral clock
    RccPclk4 = 0,
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
impl From<LPTIM2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM2SEL_A) -> Self {
        variant as _
    }
}
impl LPTIM2SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<LPTIM2SEL_A> {
        match self.bits {
            0 => Some(LPTIM2SEL_A::RccPclk4),
            1 => Some(LPTIM2SEL_A::Pll2P),
            2 => Some(LPTIM2SEL_A::Pll3R),
            3 => Some(LPTIM2SEL_A::Lse),
            4 => Some(LPTIM2SEL_A::Lsi),
            5 => Some(LPTIM2SEL_A::Per),
            _ => None,
        }
    }
    ///Checks if the value of the field is `RccPclk4`
    #[inline(always)]
    pub fn is_rcc_pclk4(&self) -> bool {
        *self == LPTIM2SEL_A::RccPclk4
    }
    ///Checks if the value of the field is `Pll2P`
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == LPTIM2SEL_A::Pll2P
    }
    ///Checks if the value of the field is `Pll3R`
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == LPTIM2SEL_A::Pll3R
    }
    ///Checks if the value of the field is `Lse`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM2SEL_A::Lse
    }
    ///Checks if the value of the field is `Lsi`
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM2SEL_A::Lsi
    }
    ///Checks if the value of the field is `Per`
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == LPTIM2SEL_A::Per
    }
}
///Field `LPTIM2SEL` writer - LPTIM2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type LPTIM2SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRDCCIPR_SPEC, u8, LPTIM2SEL_A, 3, O>;
impl<'a, const O: u8> LPTIM2SEL_W<'a, O> {
    ///rcc_pclk4 selected as peripheral clock
    #[inline(always)]
    pub fn rcc_pclk4(self) -> &'a mut W {
        self.variant(LPTIM2SEL_A::RccPclk4)
    }
    ///pll2_p selected as peripheral clock
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut W {
        self.variant(LPTIM2SEL_A::Pll2P)
    }
    ///pll3_r selected as peripheral clock
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut W {
        self.variant(LPTIM2SEL_A::Pll3R)
    }
    ///LSE selected as peripheral clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPTIM2SEL_A::Lse)
    }
    ///LSI selected as peripheral clock
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(LPTIM2SEL_A::Lsi)
    }
    ///PER selected as peripheral clock
    #[inline(always)]
    pub fn per(self) -> &'a mut W {
        self.variant(LPTIM2SEL_A::Per)
    }
}
///Field `LPTIM3SEL` reader - LPTIM3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type LPTIM3SEL_R = crate::FieldReader<u8, u8>;
///Field `LPTIM3SEL` writer - LPTIM3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type LPTIM3SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRDCCIPR_SPEC, u8, u8, 3, O>;
///Field `ADCSEL` reader - SAR ADC kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type ADCSEL_R = crate::FieldReader<u8, ADCSEL_A>;
///SAR ADC kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCSEL_A {
    ///0: pll2_p selected as peripheral clock
    Pll2P = 0,
    ///1: pll3_r selected as peripheral clock
    Pll3R = 1,
    ///2: PER selected as peripheral clock
    Per = 2,
}
impl From<ADCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCSEL_A) -> Self {
        variant as _
    }
}
impl ADCSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<ADCSEL_A> {
        match self.bits {
            0 => Some(ADCSEL_A::Pll2P),
            1 => Some(ADCSEL_A::Pll3R),
            2 => Some(ADCSEL_A::Per),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Pll2P`
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == ADCSEL_A::Pll2P
    }
    ///Checks if the value of the field is `Pll3R`
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == ADCSEL_A::Pll3R
    }
    ///Checks if the value of the field is `Per`
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == ADCSEL_A::Per
    }
}
///Field `ADCSEL` writer - SAR ADC kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type ADCSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRDCCIPR_SPEC, u8, ADCSEL_A, 2, O>;
impl<'a, const O: u8> ADCSEL_W<'a, O> {
    ///pll2_p selected as peripheral clock
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut W {
        self.variant(ADCSEL_A::Pll2P)
    }
    ///pll3_r selected as peripheral clock
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut W {
        self.variant(ADCSEL_A::Pll3R)
    }
    ///PER selected as peripheral clock
    #[inline(always)]
    pub fn per(self) -> &'a mut W {
        self.variant(ADCSEL_A::Per)
    }
}
///Field `DFSDM2SEL` reader - DFSDM2 kernel Clk clock source selection Set and reset by software. Note: The DFSDM2 Aclk clock source selection is done by SPI6SEL (see and ).
pub type DFSDM2SEL_R = crate::BitReader<bool>;
///Field `DFSDM2SEL` writer - DFSDM2 kernel Clk clock source selection Set and reset by software. Note: The DFSDM2 Aclk clock source selection is done by SPI6SEL (see and ).
pub type DFSDM2SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRDCCIPR_SPEC, bool, O>;
///Field `SPI6SEL` reader - SPI6 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type SPI6SEL_R = crate::FieldReader<u8, SPI6SEL_A>;
///SPI6 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI6SEL_A {
    ///0: rcc_pclk4 selected as peripheral clock
    RccPclk4 = 0,
    ///1: pll2_q selected as peripheral clock
    Pll2Q = 1,
    ///2: pll3_q selected as peripheral clock
    Pll3Q = 2,
    ///3: hsi_ker selected as peripheral clock
    HsiKer = 3,
    ///4: csi_ker selected as peripheral clock
    CsiKer = 4,
    ///5: HSE selected as peripheral clock
    Hse = 5,
}
impl From<SPI6SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI6SEL_A) -> Self {
        variant as _
    }
}
impl SPI6SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SPI6SEL_A> {
        match self.bits {
            0 => Some(SPI6SEL_A::RccPclk4),
            1 => Some(SPI6SEL_A::Pll2Q),
            2 => Some(SPI6SEL_A::Pll3Q),
            3 => Some(SPI6SEL_A::HsiKer),
            4 => Some(SPI6SEL_A::CsiKer),
            5 => Some(SPI6SEL_A::Hse),
            _ => None,
        }
    }
    ///Checks if the value of the field is `RccPclk4`
    #[inline(always)]
    pub fn is_rcc_pclk4(&self) -> bool {
        *self == SPI6SEL_A::RccPclk4
    }
    ///Checks if the value of the field is `Pll2Q`
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == SPI6SEL_A::Pll2Q
    }
    ///Checks if the value of the field is `Pll3Q`
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == SPI6SEL_A::Pll3Q
    }
    ///Checks if the value of the field is `HsiKer`
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == SPI6SEL_A::HsiKer
    }
    ///Checks if the value of the field is `CsiKer`
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == SPI6SEL_A::CsiKer
    }
    ///Checks if the value of the field is `Hse`
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SPI6SEL_A::Hse
    }
}
///Field `SPI6SEL` writer - SPI6 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
pub type SPI6SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRDCCIPR_SPEC, u8, SPI6SEL_A, 3, O>;
impl<'a, const O: u8> SPI6SEL_W<'a, O> {
    ///rcc_pclk4 selected as peripheral clock
    #[inline(always)]
    pub fn rcc_pclk4(self) -> &'a mut W {
        self.variant(SPI6SEL_A::RccPclk4)
    }
    ///pll2_q selected as peripheral clock
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut W {
        self.variant(SPI6SEL_A::Pll2Q)
    }
    ///pll3_q selected as peripheral clock
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut W {
        self.variant(SPI6SEL_A::Pll3Q)
    }
    ///hsi_ker selected as peripheral clock
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(SPI6SEL_A::HsiKer)
    }
    ///csi_ker selected as peripheral clock
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut W {
        self.variant(SPI6SEL_A::CsiKer)
    }
    ///HSE selected as peripheral clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(SPI6SEL_A::Hse)
    }
}
impl R {
    ///Bits 0:2 - LPUART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:9 - I2C4 kernel clock source selection Set and reset by software.
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:12 - LPTIM2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 10) & 7) as u8)
    }
    ///Bits 13:15 - LPTIM3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn lptim3sel(&self) -> LPTIM3SEL_R {
        LPTIM3SEL_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 16:17 - SAR ADC kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 27 - DFSDM2 kernel Clk clock source selection Set and reset by software. Note: The DFSDM2 Aclk clock source selection is done by SPI6SEL (see and ).
    #[inline(always)]
    pub fn dfsdm2sel(&self) -> DFSDM2SEL_R {
        DFSDM2SEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:30 - SPI6 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    pub fn spi6sel(&self) -> SPI6SEL_R {
        SPI6SEL_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - LPUART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W<0> {
        LPUART1SEL_W::new(self)
    }
    ///Bits 8:9 - I2C4 kernel clock source selection Set and reset by software.
    #[inline(always)]
    #[must_use]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W<8> {
        I2C4SEL_W::new(self)
    }
    ///Bits 10:12 - LPTIM2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W<10> {
        LPTIM2SEL_W::new(self)
    }
    ///Bits 13:15 - LPTIM3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn lptim3sel(&mut self) -> LPTIM3SEL_W<13> {
        LPTIM3SEL_W::new(self)
    }
    ///Bits 16:17 - SAR ADC kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn adcsel(&mut self) -> ADCSEL_W<16> {
        ADCSEL_W::new(self)
    }
    ///Bit 27 - DFSDM2 kernel Clk clock source selection Set and reset by software. Note: The DFSDM2 Aclk clock source selection is done by SPI6SEL (see and ).
    #[inline(always)]
    #[must_use]
    pub fn dfsdm2sel(&mut self) -> DFSDM2SEL_W<27> {
        DFSDM2SEL_W::new(self)
    }
    ///Bits 28:30 - SPI6 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled
    #[inline(always)]
    #[must_use]
    pub fn spi6sel(&mut self) -> SPI6SEL_W<28> {
        SPI6SEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC SmartRun domain kernel clock configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [srdccipr](index.html) module
pub struct SRDCCIPR_SPEC;
impl crate::RegisterSpec for SRDCCIPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [srdccipr::R](R) reader structure
impl crate::Readable for SRDCCIPR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [srdccipr::W](W) writer structure
impl crate::Writable for SRDCCIPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SRDCCIPR to value 0
impl crate::Resettable for SRDCCIPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
