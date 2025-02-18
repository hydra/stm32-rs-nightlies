///Register `DCKCFGR2` reader
pub struct R(crate::R<DCKCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCKCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCKCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCKCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DCKCFGR2` writer
pub struct W(crate::W<DCKCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCKCFGR2_SPEC>;
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
impl From<crate::W<DCKCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCKCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `USART1SEL` reader - USART 1 clock source selection
pub type USART1SEL_R = crate::FieldReader<u8, USART1SEL_A>;
///USART 1 clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART1SEL_A {
    ///0: APB2 clock (PCLK2) is selected as USART clock
    Apb2 = 0,
    ///1: System clock is selected as USART clock
    Sysclk = 1,
    ///2: HSI clock is selected as USART clock
    Hsi = 2,
    ///3: LSE clock is selected as USART clock
    Lse = 3,
}
impl From<USART1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USART1SEL_A) -> Self {
        variant as _
    }
}
impl USART1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> USART1SEL_A {
        match self.bits {
            0 => USART1SEL_A::Apb2,
            1 => USART1SEL_A::Sysclk,
            2 => USART1SEL_A::Hsi,
            3 => USART1SEL_A::Lse,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Apb2`
    #[inline(always)]
    pub fn is_apb2(&self) -> bool {
        *self == USART1SEL_A::Apb2
    }
    ///Checks if the value of the field is `Sysclk`
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == USART1SEL_A::Sysclk
    }
    ///Checks if the value of the field is `Hsi`
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == USART1SEL_A::Hsi
    }
    ///Checks if the value of the field is `Lse`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART1SEL_A::Lse
    }
}
///Field `USART1SEL` writer - USART 1 clock source selection
pub type USART1SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DCKCFGR2_SPEC, u8, USART1SEL_A, 2, O>;
impl<'a, const O: u8> USART1SEL_W<'a, O> {
    ///APB2 clock (PCLK2) is selected as USART clock
    #[inline(always)]
    pub fn apb2(self) -> &'a mut W {
        self.variant(USART1SEL_A::Apb2)
    }
    ///System clock is selected as USART clock
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(USART1SEL_A::Sysclk)
    }
    ///HSI clock is selected as USART clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(USART1SEL_A::Hsi)
    }
    ///LSE clock is selected as USART clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART1SEL_A::Lse)
    }
}
///Field `USART2SEL` reader - USART 2 clock source selection
pub type USART2SEL_R = crate::FieldReader<u8, USART2SEL_A>;
///USART 2 clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART2SEL_A {
    ///0: APB1 clock (PCLK1) is selected as USART clock
    Apb1 = 0,
    ///1: System clock is selected as USART clock
    Sysclk = 1,
    ///2: HSI clock is selected as USART clock
    Hsi = 2,
    ///3: LSE clock is selected as USART clock
    Lse = 3,
}
impl From<USART2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USART2SEL_A) -> Self {
        variant as _
    }
}
impl USART2SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> USART2SEL_A {
        match self.bits {
            0 => USART2SEL_A::Apb1,
            1 => USART2SEL_A::Sysclk,
            2 => USART2SEL_A::Hsi,
            3 => USART2SEL_A::Lse,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Apb1`
    #[inline(always)]
    pub fn is_apb1(&self) -> bool {
        *self == USART2SEL_A::Apb1
    }
    ///Checks if the value of the field is `Sysclk`
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == USART2SEL_A::Sysclk
    }
    ///Checks if the value of the field is `Hsi`
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == USART2SEL_A::Hsi
    }
    ///Checks if the value of the field is `Lse`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART2SEL_A::Lse
    }
}
///Field `USART2SEL` writer - USART 2 clock source selection
pub type USART2SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DCKCFGR2_SPEC, u8, USART2SEL_A, 2, O>;
impl<'a, const O: u8> USART2SEL_W<'a, O> {
    ///APB1 clock (PCLK1) is selected as USART clock
    #[inline(always)]
    pub fn apb1(self) -> &'a mut W {
        self.variant(USART2SEL_A::Apb1)
    }
    ///System clock is selected as USART clock
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(USART2SEL_A::Sysclk)
    }
    ///HSI clock is selected as USART clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(USART2SEL_A::Hsi)
    }
    ///LSE clock is selected as USART clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART2SEL_A::Lse)
    }
}
///Field `USART6SEL` reader - USART 6 clock source selection
pub use USART1SEL_R as USART6SEL_R;
///Field `USART6SEL` writer - USART 6 clock source selection
pub use USART1SEL_W as USART6SEL_W;
///Field `USART3SEL` reader - USART 3 clock source selection
pub use USART2SEL_R as USART3SEL_R;
///Field `UART4SEL` reader - UART 4 clock source selection
pub use USART2SEL_R as UART4SEL_R;
///Field `UART5SEL` reader - UART 5 clock source selection
pub use USART2SEL_R as UART5SEL_R;
///Field `UART7SEL` reader - UART 7 clock source selection
pub use USART2SEL_R as UART7SEL_R;
///Field `UART8SEL` reader - UART 8 clock source selection
pub use USART2SEL_R as UART8SEL_R;
///Field `USART3SEL` writer - USART 3 clock source selection
pub use USART2SEL_W as USART3SEL_W;
///Field `UART4SEL` writer - UART 4 clock source selection
pub use USART2SEL_W as UART4SEL_W;
///Field `UART5SEL` writer - UART 5 clock source selection
pub use USART2SEL_W as UART5SEL_W;
///Field `UART7SEL` writer - UART 7 clock source selection
pub use USART2SEL_W as UART7SEL_W;
///Field `UART8SEL` writer - UART 8 clock source selection
pub use USART2SEL_W as UART8SEL_W;
///Field `I2C1SEL` reader - I2C1 clock source selection
pub type I2C1SEL_R = crate::FieldReader<u8, I2C1SEL_A>;
///I2C1 clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C1SEL_A {
    ///0: APB clock selected as I2C clock
    Apb = 0,
    ///1: System clock selected as I2C clock
    Sysclk = 1,
    ///2: HSI clock selected as I2C clock
    Hsi = 2,
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
            0 => Some(I2C1SEL_A::Apb),
            1 => Some(I2C1SEL_A::Sysclk),
            2 => Some(I2C1SEL_A::Hsi),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Apb`
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == I2C1SEL_A::Apb
    }
    ///Checks if the value of the field is `Sysclk`
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == I2C1SEL_A::Sysclk
    }
    ///Checks if the value of the field is `Hsi`
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == I2C1SEL_A::Hsi
    }
}
///Field `I2C1SEL` writer - I2C1 clock source selection
pub type I2C1SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCKCFGR2_SPEC, u8, I2C1SEL_A, 2, O>;
impl<'a, const O: u8> I2C1SEL_W<'a, O> {
    ///APB clock selected as I2C clock
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(I2C1SEL_A::Apb)
    }
    ///System clock selected as I2C clock
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2C1SEL_A::Sysclk)
    }
    ///HSI clock selected as I2C clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(I2C1SEL_A::Hsi)
    }
}
///Field `I2C2SEL` reader - I2C2 clock source selection
pub use I2C1SEL_R as I2C2SEL_R;
///Field `I2C3SEL` reader - I2C3 clock source selection
pub use I2C1SEL_R as I2C3SEL_R;
///Field `I2C4SEL` reader - I2C4 clock source selection
pub use I2C1SEL_R as I2C4SEL_R;
///Field `I2C2SEL` writer - I2C2 clock source selection
pub use I2C1SEL_W as I2C2SEL_W;
///Field `I2C3SEL` writer - I2C3 clock source selection
pub use I2C1SEL_W as I2C3SEL_W;
///Field `I2C4SEL` writer - I2C4 clock source selection
pub use I2C1SEL_W as I2C4SEL_W;
///Field `LPTIM1SEL` reader - Low power timer 1 clock source selection
pub type LPTIM1SEL_R = crate::FieldReader<u8, LPTIM1SEL_A>;
///Low power timer 1 clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPTIM1SEL_A {
    ///0: APB1 clock (PCLK1) selected as LPTILM1 clock
    Apb1 = 0,
    ///1: LSI clock is selected as LPTILM1 clock
    Lsi = 1,
    ///2: HSI clock is selected as LPTILM1 clock
    Hsi = 2,
    ///3: LSE clock is selected as LPTILM1 clock
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
            0 => LPTIM1SEL_A::Apb1,
            1 => LPTIM1SEL_A::Lsi,
            2 => LPTIM1SEL_A::Hsi,
            3 => LPTIM1SEL_A::Lse,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Apb1`
    #[inline(always)]
    pub fn is_apb1(&self) -> bool {
        *self == LPTIM1SEL_A::Apb1
    }
    ///Checks if the value of the field is `Lsi`
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL_A::Lsi
    }
    ///Checks if the value of the field is `Hsi`
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == LPTIM1SEL_A::Hsi
    }
    ///Checks if the value of the field is `Lse`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM1SEL_A::Lse
    }
}
///Field `LPTIM1SEL` writer - Low power timer 1 clock source selection
pub type LPTIM1SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DCKCFGR2_SPEC, u8, LPTIM1SEL_A, 2, O>;
impl<'a, const O: u8> LPTIM1SEL_W<'a, O> {
    ///APB1 clock (PCLK1) selected as LPTILM1 clock
    #[inline(always)]
    pub fn apb1(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Apb1)
    }
    ///LSI clock is selected as LPTILM1 clock
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Lsi)
    }
    ///HSI clock is selected as LPTILM1 clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Hsi)
    }
    ///LSE clock is selected as LPTILM1 clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::Lse)
    }
}
///Field `CECSEL` reader - HDMI-CEC clock source selection
pub type CECSEL_R = crate::BitReader<CECSEL_A>;
///HDMI-CEC clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CECSEL_A {
    ///0: LSE clock is selected as HDMI-CEC clock
    Lse = 0,
    ///1: HSI divided by 488 clock is selected as HDMI-CEC clock
    HsiDiv488 = 1,
}
impl From<CECSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CECSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CECSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CECSEL_A {
        match self.bits {
            false => CECSEL_A::Lse,
            true => CECSEL_A::HsiDiv488,
        }
    }
    ///Checks if the value of the field is `Lse`
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == CECSEL_A::Lse
    }
    ///Checks if the value of the field is `HsiDiv488`
    #[inline(always)]
    pub fn is_hsi_div488(&self) -> bool {
        *self == CECSEL_A::HsiDiv488
    }
}
///Field `CECSEL` writer - HDMI-CEC clock source selection
pub type CECSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCKCFGR2_SPEC, CECSEL_A, O>;
impl<'a, const O: u8> CECSEL_W<'a, O> {
    ///LSE clock is selected as HDMI-CEC clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(CECSEL_A::Lse)
    }
    ///HSI divided by 488 clock is selected as HDMI-CEC clock
    #[inline(always)]
    pub fn hsi_div488(self) -> &'a mut W {
        self.variant(CECSEL_A::HsiDiv488)
    }
}
///Field `CK48MSEL` reader - 48MHz clock source selection
pub type CK48MSEL_R = crate::BitReader<CK48MSEL_A>;
///48MHz clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CK48MSEL_A {
    ///0: 48MHz clock from PLL is selected
    Pll = 0,
    ///1: 48MHz clock from PLLSAI is selected
    Pllsai = 1,
}
impl From<CK48MSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CK48MSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CK48MSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CK48MSEL_A {
        match self.bits {
            false => CK48MSEL_A::Pll,
            true => CK48MSEL_A::Pllsai,
        }
    }
    ///Checks if the value of the field is `Pll`
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == CK48MSEL_A::Pll
    }
    ///Checks if the value of the field is `Pllsai`
    #[inline(always)]
    pub fn is_pllsai(&self) -> bool {
        *self == CK48MSEL_A::Pllsai
    }
}
///Field `CK48MSEL` writer - 48MHz clock source selection
pub type CK48MSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCKCFGR2_SPEC, CK48MSEL_A, O>;
impl<'a, const O: u8> CK48MSEL_W<'a, O> {
    ///48MHz clock from PLL is selected
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(CK48MSEL_A::Pll)
    }
    ///48MHz clock from PLLSAI is selected
    #[inline(always)]
    pub fn pllsai(self) -> &'a mut W {
        self.variant(CK48MSEL_A::Pllsai)
    }
}
///Field `SDMMC1SEL` reader - SDMMC clock source selection
pub type SDMMC1SEL_R = crate::BitReader<SDMMC1SEL_A>;
///SDMMC clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMC1SEL_A {
    ///0: 48 MHz clock is selected as SD clock
    Ck48m = 0,
    ///1: System clock is selected as SD clock
    Sysclk = 1,
}
impl From<SDMMC1SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SDMMC1SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SDMMC1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SDMMC1SEL_A {
        match self.bits {
            false => SDMMC1SEL_A::Ck48m,
            true => SDMMC1SEL_A::Sysclk,
        }
    }
    ///Checks if the value of the field is `Ck48m`
    #[inline(always)]
    pub fn is_ck48m(&self) -> bool {
        *self == SDMMC1SEL_A::Ck48m
    }
    ///Checks if the value of the field is `Sysclk`
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == SDMMC1SEL_A::Sysclk
    }
}
///Field `SDMMC1SEL` writer - SDMMC clock source selection
pub type SDMMC1SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCKCFGR2_SPEC, SDMMC1SEL_A, O>;
impl<'a, const O: u8> SDMMC1SEL_W<'a, O> {
    ///48 MHz clock is selected as SD clock
    #[inline(always)]
    pub fn ck48m(self) -> &'a mut W {
        self.variant(SDMMC1SEL_A::Ck48m)
    }
    ///System clock is selected as SD clock
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(SDMMC1SEL_A::Sysclk)
    }
}
///Field `SDMMC2SEL` reader - SDMMC2 clock source selection
pub use SDMMC1SEL_R as SDMMC2SEL_R;
///Field `SDMMC2SEL` writer - SDMMC2 clock source selection
pub use SDMMC1SEL_W as SDMMC2SEL_W;
///Field `DSISEL` reader - DSI clock source selection
pub type DSISEL_R = crate::BitReader<DSISEL_A>;
///DSI clock source selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSISEL_A {
    ///0: DSI-PHY used as DSI byte lane clock source (usual case)
    DsiPhy = 0,
    ///1: PLLR used as DSI byte lane clock source, used in case DSI PLL and DSI-PHY are off (low power mode)
    Pllr = 1,
}
impl From<DSISEL_A> for bool {
    #[inline(always)]
    fn from(variant: DSISEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DSISEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DSISEL_A {
        match self.bits {
            false => DSISEL_A::DsiPhy,
            true => DSISEL_A::Pllr,
        }
    }
    ///Checks if the value of the field is `DsiPhy`
    #[inline(always)]
    pub fn is_dsi_phy(&self) -> bool {
        *self == DSISEL_A::DsiPhy
    }
    ///Checks if the value of the field is `Pllr`
    #[inline(always)]
    pub fn is_pllr(&self) -> bool {
        *self == DSISEL_A::Pllr
    }
}
///Field `DSISEL` writer - DSI clock source selection
pub type DSISEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCKCFGR2_SPEC, DSISEL_A, O>;
impl<'a, const O: u8> DSISEL_W<'a, O> {
    ///DSI-PHY used as DSI byte lane clock source (usual case)
    #[inline(always)]
    pub fn dsi_phy(self) -> &'a mut W {
        self.variant(DSISEL_A::DsiPhy)
    }
    ///PLLR used as DSI byte lane clock source, used in case DSI PLL and DSI-PHY are off (low power mode)
    #[inline(always)]
    pub fn pllr(self) -> &'a mut W {
        self.variant(DSISEL_A::Pllr)
    }
}
impl R {
    ///Bits 0:1 - USART 1 clock source selection
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - USART 2 clock source selection
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - USART 3 clock source selection
    #[inline(always)]
    pub fn usart3sel(&self) -> USART3SEL_R {
        USART3SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - UART 4 clock source selection
    #[inline(always)]
    pub fn uart4sel(&self) -> UART4SEL_R {
        UART4SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - UART 5 clock source selection
    #[inline(always)]
    pub fn uart5sel(&self) -> UART5SEL_R {
        UART5SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - USART 6 clock source selection
    #[inline(always)]
    pub fn usart6sel(&self) -> USART6SEL_R {
        USART6SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - UART 7 clock source selection
    #[inline(always)]
    pub fn uart7sel(&self) -> UART7SEL_R {
        UART7SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - UART 8 clock source selection
    #[inline(always)]
    pub fn uart8sel(&self) -> UART8SEL_R {
        UART8SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - I2C1 clock source selection
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - I2C2 clock source selection
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2C2SEL_R {
        I2C2SEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - I2C3 clock source selection
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - I2C4 clock source selection
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - Low power timer 1 clock source selection
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 26 - HDMI-CEC clock source selection
    #[inline(always)]
    pub fn cecsel(&self) -> CECSEL_R {
        CECSEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - 48MHz clock source selection
    #[inline(always)]
    pub fn ck48msel(&self) -> CK48MSEL_R {
        CK48MSEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SDMMC clock source selection
    #[inline(always)]
    pub fn sdmmc1sel(&self) -> SDMMC1SEL_R {
        SDMMC1SEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SDMMC2 clock source selection
    #[inline(always)]
    pub fn sdmmc2sel(&self) -> SDMMC2SEL_R {
        SDMMC2SEL_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - DSI clock source selection
    #[inline(always)]
    pub fn dsisel(&self) -> DSISEL_R {
        DSISEL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - USART 1 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn usart1sel(&mut self) -> USART1SEL_W<0> {
        USART1SEL_W::new(self)
    }
    ///Bits 2:3 - USART 2 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn usart2sel(&mut self) -> USART2SEL_W<2> {
        USART2SEL_W::new(self)
    }
    ///Bits 4:5 - USART 3 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn usart3sel(&mut self) -> USART3SEL_W<4> {
        USART3SEL_W::new(self)
    }
    ///Bits 6:7 - UART 4 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn uart4sel(&mut self) -> UART4SEL_W<6> {
        UART4SEL_W::new(self)
    }
    ///Bits 8:9 - UART 5 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn uart5sel(&mut self) -> UART5SEL_W<8> {
        UART5SEL_W::new(self)
    }
    ///Bits 10:11 - USART 6 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn usart6sel(&mut self) -> USART6SEL_W<10> {
        USART6SEL_W::new(self)
    }
    ///Bits 12:13 - UART 7 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn uart7sel(&mut self) -> UART7SEL_W<12> {
        UART7SEL_W::new(self)
    }
    ///Bits 14:15 - UART 8 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn uart8sel(&mut self) -> UART8SEL_W<14> {
        UART8SEL_W::new(self)
    }
    ///Bits 16:17 - I2C1 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W<16> {
        I2C1SEL_W::new(self)
    }
    ///Bits 18:19 - I2C2 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn i2c2sel(&mut self) -> I2C2SEL_W<18> {
        I2C2SEL_W::new(self)
    }
    ///Bits 20:21 - I2C3 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W<20> {
        I2C3SEL_W::new(self)
    }
    ///Bits 22:23 - I2C4 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W<22> {
        I2C4SEL_W::new(self)
    }
    ///Bits 24:25 - Low power timer 1 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W<24> {
        LPTIM1SEL_W::new(self)
    }
    ///Bit 26 - HDMI-CEC clock source selection
    #[inline(always)]
    #[must_use]
    pub fn cecsel(&mut self) -> CECSEL_W<26> {
        CECSEL_W::new(self)
    }
    ///Bit 27 - 48MHz clock source selection
    #[inline(always)]
    #[must_use]
    pub fn ck48msel(&mut self) -> CK48MSEL_W<27> {
        CK48MSEL_W::new(self)
    }
    ///Bit 28 - SDMMC clock source selection
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1sel(&mut self) -> SDMMC1SEL_W<28> {
        SDMMC1SEL_W::new(self)
    }
    ///Bit 29 - SDMMC2 clock source selection
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2sel(&mut self) -> SDMMC2SEL_W<29> {
        SDMMC2SEL_W::new(self)
    }
    ///Bit 30 - DSI clock source selection
    #[inline(always)]
    #[must_use]
    pub fn dsisel(&mut self) -> DSISEL_W<30> {
        DSISEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///dedicated clocks configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dckcfgr2](index.html) module
pub struct DCKCFGR2_SPEC;
impl crate::RegisterSpec for DCKCFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [dckcfgr2::R](R) reader structure
impl crate::Readable for DCKCFGR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dckcfgr2::W](W) writer structure
impl crate::Writable for DCKCFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DCKCFGR2 to value 0
impl crate::Resettable for DCKCFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
