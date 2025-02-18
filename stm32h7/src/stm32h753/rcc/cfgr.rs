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
    ///1: CSI selected as system clock
    Csi = 1,
    ///2: HSE selected as system clock
    Hse = 2,
    ///3: PLL1 selected as system clock
    Pll1 = 3,
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
            1 => Some(SW_A::Csi),
            2 => Some(SW_A::Hse),
            3 => Some(SW_A::Pll1),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Hsi`
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SW_A::Hsi
    }
    ///Checks if the value of the field is `Csi`
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == SW_A::Csi
    }
    ///Checks if the value of the field is `Hse`
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SW_A::Hse
    }
    ///Checks if the value of the field is `Pll1`
    #[inline(always)]
    pub fn is_pll1(&self) -> bool {
        *self == SW_A::Pll1
    }
}
///Field `SW` writer - System clock switch
pub type SW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, SW_A, 3, O>;
impl<'a, const O: u8> SW_W<'a, O> {
    ///HSI selected as system clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(SW_A::Hsi)
    }
    ///CSI selected as system clock
    #[inline(always)]
    pub fn csi(self) -> &'a mut W {
        self.variant(SW_A::Csi)
    }
    ///HSE selected as system clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(SW_A::Hse)
    }
    ///PLL1 selected as system clock
    #[inline(always)]
    pub fn pll1(self) -> &'a mut W {
        self.variant(SW_A::Pll1)
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
    ///1: CSI oscillator used as system clock
    Csi = 1,
    ///2: HSE oscillator used as system clock
    Hse = 2,
    ///3: PLL1 used as system clock
    Pll1 = 3,
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
            1 => Some(SWSR_A::Csi),
            2 => Some(SWSR_A::Hse),
            3 => Some(SWSR_A::Pll1),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Hsi`
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SWSR_A::Hsi
    }
    ///Checks if the value of the field is `Csi`
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == SWSR_A::Csi
    }
    ///Checks if the value of the field is `Hse`
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SWSR_A::Hse
    }
    ///Checks if the value of the field is `Pll1`
    #[inline(always)]
    pub fn is_pll1(&self) -> bool {
        *self == SWSR_A::Pll1
    }
}
///Field `SWS` writer - System clock switch status
pub type SWS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, SWSR_A, 3, O>;
impl<'a, const O: u8> SWS_W<'a, O> {
    ///HSI oscillator used as system clock
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(SWSR_A::Hsi)
    }
    ///CSI oscillator used as system clock
    #[inline(always)]
    pub fn csi(self) -> &'a mut W {
        self.variant(SWSR_A::Csi)
    }
    ///HSE oscillator used as system clock
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(SWSR_A::Hse)
    }
    ///PLL1 used as system clock
    #[inline(always)]
    pub fn pll1(self) -> &'a mut W {
        self.variant(SWSR_A::Pll1)
    }
}
///Field `STOPWUCK` reader - System clock selection after a wake up from system Stop
pub type STOPWUCK_R = crate::BitReader<STOPWUCK_A>;
///System clock selection after a wake up from system Stop
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPWUCK_A {
    ///0: HSI selected as wake up clock from system Stop
    Hsi = 0,
    ///1: CSI selected as wake up clock from system Stop
    Csi = 1,
}
impl From<STOPWUCK_A> for bool {
    #[inline(always)]
    fn from(variant: STOPWUCK_A) -> Self {
        variant as u8 != 0
    }
}
impl STOPWUCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> STOPWUCK_A {
        match self.bits {
            false => STOPWUCK_A::Hsi,
            true => STOPWUCK_A::Csi,
        }
    }
    ///Checks if the value of the field is `Hsi`
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == STOPWUCK_A::Hsi
    }
    ///Checks if the value of the field is `Csi`
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == STOPWUCK_A::Csi
    }
}
///Field `STOPWUCK` writer - System clock selection after a wake up from system Stop
pub type STOPWUCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, STOPWUCK_A, O>;
impl<'a, const O: u8> STOPWUCK_W<'a, O> {
    ///HSI selected as wake up clock from system Stop
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(STOPWUCK_A::Hsi)
    }
    ///CSI selected as wake up clock from system Stop
    #[inline(always)]
    pub fn csi(self) -> &'a mut W {
        self.variant(STOPWUCK_A::Csi)
    }
}
///Field `STOPKERWUCK` reader - Kernel clock selection after a wake up from system Stop
pub use STOPWUCK_R as STOPKERWUCK_R;
///Field `STOPKERWUCK` writer - Kernel clock selection after a wake up from system Stop
pub use STOPWUCK_W as STOPKERWUCK_W;
///Field `RTCPRE` reader - HSE division factor for RTC clock
pub type RTCPRE_R = crate::FieldReader<u8, u8>;
///Field `RTCPRE` writer - HSE division factor for RTC clock
pub type RTCPRE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, u8, 6, O>;
///Field `HRTIMSEL` reader - High Resolution Timer clock prescaler selection
pub type HRTIMSEL_R = crate::BitReader<HRTIMSEL_A>;
///High Resolution Timer clock prescaler selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRTIMSEL_A {
    ///0: The HRTIM prescaler clock source is the same as other timers (rcc_timy_ker_ck)
    TimyKer = 0,
    ///1: The HRTIM prescaler clock source is the CPU clock (c_ck)
    CCk = 1,
}
impl From<HRTIMSEL_A> for bool {
    #[inline(always)]
    fn from(variant: HRTIMSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl HRTIMSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HRTIMSEL_A {
        match self.bits {
            false => HRTIMSEL_A::TimyKer,
            true => HRTIMSEL_A::CCk,
        }
    }
    ///Checks if the value of the field is `TimyKer`
    #[inline(always)]
    pub fn is_timy_ker(&self) -> bool {
        *self == HRTIMSEL_A::TimyKer
    }
    ///Checks if the value of the field is `CCk`
    #[inline(always)]
    pub fn is_c_ck(&self) -> bool {
        *self == HRTIMSEL_A::CCk
    }
}
///Field `HRTIMSEL` writer - High Resolution Timer clock prescaler selection
pub type HRTIMSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, HRTIMSEL_A, O>;
impl<'a, const O: u8> HRTIMSEL_W<'a, O> {
    ///The HRTIM prescaler clock source is the same as other timers (rcc_timy_ker_ck)
    #[inline(always)]
    pub fn timy_ker(self) -> &'a mut W {
        self.variant(HRTIMSEL_A::TimyKer)
    }
    ///The HRTIM prescaler clock source is the CPU clock (c_ck)
    #[inline(always)]
    pub fn c_ck(self) -> &'a mut W {
        self.variant(HRTIMSEL_A::CCk)
    }
}
///Field `TIMPRE` reader - Timers clocks prescaler selection
pub type TIMPRE_R = crate::BitReader<TIMPRE_A>;
///Timers clocks prescaler selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMPRE_A {
    ///0: Timer kernel clock equal to 2x pclk by default
    DefaultX2 = 0,
    ///1: Timer kernel clock equal to 4x pclk by default
    DefaultX4 = 1,
}
impl From<TIMPRE_A> for bool {
    #[inline(always)]
    fn from(variant: TIMPRE_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMPRE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIMPRE_A {
        match self.bits {
            false => TIMPRE_A::DefaultX2,
            true => TIMPRE_A::DefaultX4,
        }
    }
    ///Checks if the value of the field is `DefaultX2`
    #[inline(always)]
    pub fn is_default_x2(&self) -> bool {
        *self == TIMPRE_A::DefaultX2
    }
    ///Checks if the value of the field is `DefaultX4`
    #[inline(always)]
    pub fn is_default_x4(&self) -> bool {
        *self == TIMPRE_A::DefaultX4
    }
}
///Field `TIMPRE` writer - Timers clocks prescaler selection
pub type TIMPRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, TIMPRE_A, O>;
impl<'a, const O: u8> TIMPRE_W<'a, O> {
    ///Timer kernel clock equal to 2x pclk by default
    #[inline(always)]
    pub fn default_x2(self) -> &'a mut W {
        self.variant(TIMPRE_A::DefaultX2)
    }
    ///Timer kernel clock equal to 4x pclk by default
    #[inline(always)]
    pub fn default_x4(self) -> &'a mut W {
        self.variant(TIMPRE_A::DefaultX4)
    }
}
///Field `MCO1PRE` reader - MCO1 prescaler
pub type MCO1PRE_R = crate::FieldReader<u8, u8>;
///Field `MCO1PRE` writer - MCO1 prescaler
pub type MCO1PRE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, u8, 4, O>;
///Field `MCO1` reader - Micro-controller clock output 1
pub type MCO1_R = crate::FieldReader<u8, MCO1_A>;
///Micro-controller clock output 1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO1_A {
    ///0: HSI selected for micro-controller clock output
    Hsi = 0,
    ///1: LSE selected for micro-controller clock output
    Lse = 1,
    ///2: HSE selected for micro-controller clock output
    Hse = 2,
    ///3: pll1_q selected for micro-controller clock output
    Pll1Q = 3,
    ///4: HSI48 selected for micro-controller clock output
    Hsi48 = 4,
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
    pub fn variant(&self) -> Option<MCO1_A> {
        match self.bits {
            0 => Some(MCO1_A::Hsi),
            1 => Some(MCO1_A::Lse),
            2 => Some(MCO1_A::Hse),
            3 => Some(MCO1_A::Pll1Q),
            4 => Some(MCO1_A::Hsi48),
            _ => None,
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
    ///Checks if the value of the field is `Pll1Q`
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == MCO1_A::Pll1Q
    }
    ///Checks if the value of the field is `Hsi48`
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == MCO1_A::Hsi48
    }
}
///Field `MCO1` writer - Micro-controller clock output 1
pub type MCO1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, MCO1_A, 3, O>;
impl<'a, const O: u8> MCO1_W<'a, O> {
    ///HSI selected for micro-controller clock output
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(MCO1_A::Hsi)
    }
    ///LSE selected for micro-controller clock output
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(MCO1_A::Lse)
    }
    ///HSE selected for micro-controller clock output
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(MCO1_A::Hse)
    }
    ///pll1_q selected for micro-controller clock output
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(MCO1_A::Pll1Q)
    }
    ///HSI48 selected for micro-controller clock output
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut W {
        self.variant(MCO1_A::Hsi48)
    }
}
///Field `MCO2PRE` reader - MCO2 prescaler
pub type MCO2PRE_R = crate::FieldReader<u8, u8>;
///Field `MCO2PRE` writer - MCO2 prescaler
pub type MCO2PRE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, u8, 4, O>;
///Field `MCO2` reader - Micro-controller clock output 2
pub type MCO2_R = crate::FieldReader<u8, MCO2_A>;
///Micro-controller clock output 2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO2_A {
    ///0: System clock selected for micro-controller clock output
    Sysclk = 0,
    ///1: pll2_p selected for micro-controller clock output
    Pll2P = 1,
    ///2: HSE selected for micro-controller clock output
    Hse = 2,
    ///3: pll1_p selected for micro-controller clock output
    Pll1P = 3,
    ///4: CSI selected for micro-controller clock output
    Csi = 4,
    ///5: LSI selected for micro-controller clock output
    Lsi = 5,
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
    pub fn variant(&self) -> Option<MCO2_A> {
        match self.bits {
            0 => Some(MCO2_A::Sysclk),
            1 => Some(MCO2_A::Pll2P),
            2 => Some(MCO2_A::Hse),
            3 => Some(MCO2_A::Pll1P),
            4 => Some(MCO2_A::Csi),
            5 => Some(MCO2_A::Lsi),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Sysclk`
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == MCO2_A::Sysclk
    }
    ///Checks if the value of the field is `Pll2P`
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == MCO2_A::Pll2P
    }
    ///Checks if the value of the field is `Hse`
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCO2_A::Hse
    }
    ///Checks if the value of the field is `Pll1P`
    #[inline(always)]
    pub fn is_pll1_p(&self) -> bool {
        *self == MCO2_A::Pll1P
    }
    ///Checks if the value of the field is `Csi`
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == MCO2_A::Csi
    }
    ///Checks if the value of the field is `Lsi`
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == MCO2_A::Lsi
    }
}
///Field `MCO2` writer - Micro-controller clock output 2
pub type MCO2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, MCO2_A, 3, O>;
impl<'a, const O: u8> MCO2_W<'a, O> {
    ///System clock selected for micro-controller clock output
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(MCO2_A::Sysclk)
    }
    ///pll2_p selected for micro-controller clock output
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut W {
        self.variant(MCO2_A::Pll2P)
    }
    ///HSE selected for micro-controller clock output
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(MCO2_A::Hse)
    }
    ///pll1_p selected for micro-controller clock output
    #[inline(always)]
    pub fn pll1_p(self) -> &'a mut W {
        self.variant(MCO2_A::Pll1P)
    }
    ///CSI selected for micro-controller clock output
    #[inline(always)]
    pub fn csi(self) -> &'a mut W {
        self.variant(MCO2_A::Csi)
    }
    ///LSI selected for micro-controller clock output
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(MCO2_A::Lsi)
    }
}
impl R {
    ///Bits 0:2 - System clock switch
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - System clock switch status
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bit 6 - System clock selection after a wake up from system Stop
    #[inline(always)]
    pub fn stopwuck(&self) -> STOPWUCK_R {
        STOPWUCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Kernel clock selection after a wake up from system Stop
    #[inline(always)]
    pub fn stopkerwuck(&self) -> STOPKERWUCK_R {
        STOPKERWUCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:13 - HSE division factor for RTC clock
    #[inline(always)]
    pub fn rtcpre(&self) -> RTCPRE_R {
        RTCPRE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bit 14 - High Resolution Timer clock prescaler selection
    #[inline(always)]
    pub fn hrtimsel(&self) -> HRTIMSEL_R {
        HRTIMSEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Timers clocks prescaler selection
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 18:21 - MCO1 prescaler
    #[inline(always)]
    pub fn mco1pre(&self) -> MCO1PRE_R {
        MCO1PRE_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bits 22:24 - Micro-controller clock output 1
    #[inline(always)]
    pub fn mco1(&self) -> MCO1_R {
        MCO1_R::new(((self.bits >> 22) & 7) as u8)
    }
    ///Bits 25:28 - MCO2 prescaler
    #[inline(always)]
    pub fn mco2pre(&self) -> MCO2PRE_R {
        MCO2PRE_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    ///Bits 29:31 - Micro-controller clock output 2
    #[inline(always)]
    pub fn mco2(&self) -> MCO2_R {
        MCO2_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    ///Bits 0:2 - System clock switch
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<0> {
        SW_W::new(self)
    }
    ///Bits 3:5 - System clock switch status
    #[inline(always)]
    #[must_use]
    pub fn sws(&mut self) -> SWS_W<3> {
        SWS_W::new(self)
    }
    ///Bit 6 - System clock selection after a wake up from system Stop
    #[inline(always)]
    #[must_use]
    pub fn stopwuck(&mut self) -> STOPWUCK_W<6> {
        STOPWUCK_W::new(self)
    }
    ///Bit 7 - Kernel clock selection after a wake up from system Stop
    #[inline(always)]
    #[must_use]
    pub fn stopkerwuck(&mut self) -> STOPKERWUCK_W<7> {
        STOPKERWUCK_W::new(self)
    }
    ///Bits 8:13 - HSE division factor for RTC clock
    #[inline(always)]
    #[must_use]
    pub fn rtcpre(&mut self) -> RTCPRE_W<8> {
        RTCPRE_W::new(self)
    }
    ///Bit 14 - High Resolution Timer clock prescaler selection
    #[inline(always)]
    #[must_use]
    pub fn hrtimsel(&mut self) -> HRTIMSEL_W<14> {
        HRTIMSEL_W::new(self)
    }
    ///Bit 15 - Timers clocks prescaler selection
    #[inline(always)]
    #[must_use]
    pub fn timpre(&mut self) -> TIMPRE_W<15> {
        TIMPRE_W::new(self)
    }
    ///Bits 18:21 - MCO1 prescaler
    #[inline(always)]
    #[must_use]
    pub fn mco1pre(&mut self) -> MCO1PRE_W<18> {
        MCO1PRE_W::new(self)
    }
    ///Bits 22:24 - Micro-controller clock output 1
    #[inline(always)]
    #[must_use]
    pub fn mco1(&mut self) -> MCO1_W<22> {
        MCO1_W::new(self)
    }
    ///Bits 25:28 - MCO2 prescaler
    #[inline(always)]
    #[must_use]
    pub fn mco2pre(&mut self) -> MCO2PRE_W<25> {
        MCO2PRE_W::new(self)
    }
    ///Bits 29:31 - Micro-controller clock output 2
    #[inline(always)]
    #[must_use]
    pub fn mco2(&mut self) -> MCO2_W<29> {
        MCO2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC Clock Configuration Register
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
