///Register `CR2` reader
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR2` writer
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADON` reader - A/D converter ON / OFF
pub type ADON_R = crate::BitReader<ADON_A>;
///A/D converter ON / OFF
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADON_A {
    ///0: Disable ADC conversion/calibration and go to power down mode
    Disabled = 0,
    ///1: Enable ADC and to start conversion
    Enabled = 1,
}
impl From<ADON_A> for bool {
    #[inline(always)]
    fn from(variant: ADON_A) -> Self {
        variant as u8 != 0
    }
}
impl ADON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADON_A {
        match self.bits {
            false => ADON_A::Disabled,
            true => ADON_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADON_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADON_A::Enabled
    }
}
///Field `ADON` writer - A/D converter ON / OFF
pub type ADON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, ADON_A, O>;
impl<'a, const O: u8> ADON_W<'a, O> {
    ///Disable ADC conversion/calibration and go to power down mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADON_A::Disabled)
    }
    ///Enable ADC and to start conversion
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADON_A::Enabled)
    }
}
///Field `CONT` reader - Continuous conversion
pub type CONT_R = crate::BitReader<CONT_A>;
///Continuous conversion
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONT_A {
    ///0: Single conversion mode
    Single = 0,
    ///1: Continuous conversion mode
    Continuous = 1,
}
impl From<CONT_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_A) -> Self {
        variant as u8 != 0
    }
}
impl CONT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CONT_A {
        match self.bits {
            false => CONT_A::Single,
            true => CONT_A::Continuous,
        }
    }
    ///Checks if the value of the field is `Single`
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == CONT_A::Single
    }
    ///Checks if the value of the field is `Continuous`
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CONT_A::Continuous
    }
}
///Field `CONT` writer - Continuous conversion
pub type CONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, CONT_A, O>;
impl<'a, const O: u8> CONT_W<'a, O> {
    ///Single conversion mode
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(CONT_A::Single)
    }
    ///Continuous conversion mode
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CONT_A::Continuous)
    }
}
///Field `CAL` reader - A/D calibration
pub type CAL_R = crate::BitReader<CALR_A>;
///A/D calibration
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALR_A {
    ///0: Calibration completed
    Complete = 0,
    ///1: Calibrating
    NotComplete = 1,
}
impl From<CALR_A> for bool {
    #[inline(always)]
    fn from(variant: CALR_A) -> Self {
        variant as u8 != 0
    }
}
impl CAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CALR_A {
        match self.bits {
            false => CALR_A::Complete,
            true => CALR_A::NotComplete,
        }
    }
    ///Checks if the value of the field is `Complete`
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == CALR_A::Complete
    }
    ///Checks if the value of the field is `NotComplete`
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == CALR_A::NotComplete
    }
}
///A/D calibration
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALW_AW {
    ///1: Enable calibration
    Start = 1,
}
impl From<CALW_AW> for bool {
    #[inline(always)]
    fn from(variant: CALW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CAL` writer - A/D calibration
pub type CAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, CALW_AW, O>;
impl<'a, const O: u8> CAL_W<'a, O> {
    ///Enable calibration
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(CALW_AW::Start)
    }
}
///Field `RSTCAL` reader - Reset calibration
pub type RSTCAL_R = crate::BitReader<RSTCALR_A>;
///Reset calibration
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTCALR_A {
    ///0: Calibration register initialized
    Initialized = 0,
    ///1: Initializing calibration register
    NotInitialized = 1,
}
impl From<RSTCALR_A> for bool {
    #[inline(always)]
    fn from(variant: RSTCALR_A) -> Self {
        variant as u8 != 0
    }
}
impl RSTCAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RSTCALR_A {
        match self.bits {
            false => RSTCALR_A::Initialized,
            true => RSTCALR_A::NotInitialized,
        }
    }
    ///Checks if the value of the field is `Initialized`
    #[inline(always)]
    pub fn is_initialized(&self) -> bool {
        *self == RSTCALR_A::Initialized
    }
    ///Checks if the value of the field is `NotInitialized`
    #[inline(always)]
    pub fn is_not_initialized(&self) -> bool {
        *self == RSTCALR_A::NotInitialized
    }
}
///Reset calibration
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTCALW_AW {
    ///1: Initialize calibration register
    Initialize = 1,
}
impl From<RSTCALW_AW> for bool {
    #[inline(always)]
    fn from(variant: RSTCALW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `RSTCAL` writer - Reset calibration
pub type RSTCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, RSTCALW_AW, O>;
impl<'a, const O: u8> RSTCAL_W<'a, O> {
    ///Initialize calibration register
    #[inline(always)]
    pub fn initialize(self) -> &'a mut W {
        self.variant(RSTCALW_AW::Initialize)
    }
}
///Field `DMA` reader - Direct memory access mode
pub type DMA_R = crate::BitReader<DMA_A>;
///Direct memory access mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_A {
    ///0: DMA mode disabled
    Disabled = 0,
    ///1: DMA mode enabled
    Enabled = 1,
}
impl From<DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            false => DMA_A::Disabled,
            true => DMA_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA_A::Enabled
    }
}
///Field `DMA` writer - Direct memory access mode
pub type DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, DMA_A, O>;
impl<'a, const O: u8> DMA_W<'a, O> {
    ///DMA mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA_A::Disabled)
    }
    ///DMA mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA_A::Enabled)
    }
}
///Field `ALIGN` reader - Data alignment
pub type ALIGN_R = crate::BitReader<ALIGN_A>;
///Data alignment
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALIGN_A {
    ///0: Right Alignment
    Right = 0,
    ///1: Left Alignment
    Left = 1,
}
impl From<ALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN_A) -> Self {
        variant as u8 != 0
    }
}
impl ALIGN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALIGN_A {
        match self.bits {
            false => ALIGN_A::Right,
            true => ALIGN_A::Left,
        }
    }
    ///Checks if the value of the field is `Right`
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == ALIGN_A::Right
    }
    ///Checks if the value of the field is `Left`
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == ALIGN_A::Left
    }
}
///Field `ALIGN` writer - Data alignment
pub type ALIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, ALIGN_A, O>;
impl<'a, const O: u8> ALIGN_W<'a, O> {
    ///Right Alignment
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(ALIGN_A::Right)
    }
    ///Left Alignment
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(ALIGN_A::Left)
    }
}
///Field `JEXTSEL` reader - External event select for injected group
pub type JEXTSEL_R = crate::FieldReader<u8, JEXTSEL_A>;
///External event select for injected group
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JEXTSEL_A {
    ///0: Timer 1 TRGO event
    Tim1trgo = 0,
    ///1: Timer 1 CC4 event
    Tim1cc4 = 1,
    ///2: Timer 2 TRGO event
    Tim2trgo = 2,
    ///3: Timer 2 CC1 event
    Tim2cc1 = 3,
    ///4: Timer 3 CC4 event
    Tim3cc4 = 4,
    ///5: Timer 4 TRGO event
    Tim4trgo = 5,
    ///6: EXTI line15/TIM8_CC4 event (TIM8_CC4 is available only in high-density and XL-density devices)
    Exti15 = 6,
    ///7: JSWSTART
    Jswstart = 7,
}
impl From<JEXTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: JEXTSEL_A) -> Self {
        variant as _
    }
}
impl JEXTSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JEXTSEL_A {
        match self.bits {
            0 => JEXTSEL_A::Tim1trgo,
            1 => JEXTSEL_A::Tim1cc4,
            2 => JEXTSEL_A::Tim2trgo,
            3 => JEXTSEL_A::Tim2cc1,
            4 => JEXTSEL_A::Tim3cc4,
            5 => JEXTSEL_A::Tim4trgo,
            6 => JEXTSEL_A::Exti15,
            7 => JEXTSEL_A::Jswstart,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Tim1trgo`
    #[inline(always)]
    pub fn is_tim1trgo(&self) -> bool {
        *self == JEXTSEL_A::Tim1trgo
    }
    ///Checks if the value of the field is `Tim1cc4`
    #[inline(always)]
    pub fn is_tim1cc4(&self) -> bool {
        *self == JEXTSEL_A::Tim1cc4
    }
    ///Checks if the value of the field is `Tim2trgo`
    #[inline(always)]
    pub fn is_tim2trgo(&self) -> bool {
        *self == JEXTSEL_A::Tim2trgo
    }
    ///Checks if the value of the field is `Tim2cc1`
    #[inline(always)]
    pub fn is_tim2cc1(&self) -> bool {
        *self == JEXTSEL_A::Tim2cc1
    }
    ///Checks if the value of the field is `Tim3cc4`
    #[inline(always)]
    pub fn is_tim3cc4(&self) -> bool {
        *self == JEXTSEL_A::Tim3cc4
    }
    ///Checks if the value of the field is `Tim4trgo`
    #[inline(always)]
    pub fn is_tim4trgo(&self) -> bool {
        *self == JEXTSEL_A::Tim4trgo
    }
    ///Checks if the value of the field is `Exti15`
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        *self == JEXTSEL_A::Exti15
    }
    ///Checks if the value of the field is `Jswstart`
    #[inline(always)]
    pub fn is_jswstart(&self) -> bool {
        *self == JEXTSEL_A::Jswstart
    }
}
///Field `JEXTSEL` writer - External event select for injected group
pub type JEXTSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CR2_SPEC, u8, JEXTSEL_A, 3, O>;
impl<'a, const O: u8> JEXTSEL_W<'a, O> {
    ///Timer 1 TRGO event
    #[inline(always)]
    pub fn tim1trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim1trgo)
    }
    ///Timer 1 CC4 event
    #[inline(always)]
    pub fn tim1cc4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim1cc4)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim2trgo)
    }
    ///Timer 2 CC1 event
    #[inline(always)]
    pub fn tim2cc1(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim2cc1)
    }
    ///Timer 3 CC4 event
    #[inline(always)]
    pub fn tim3cc4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim3cc4)
    }
    ///Timer 4 TRGO event
    #[inline(always)]
    pub fn tim4trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim4trgo)
    }
    ///EXTI line15/TIM8_CC4 event (TIM8_CC4 is available only in high-density and XL-density devices)
    #[inline(always)]
    pub fn exti15(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Exti15)
    }
    ///JSWSTART
    #[inline(always)]
    pub fn jswstart(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Jswstart)
    }
}
///Field `JEXTTRIG` reader - External trigger conversion mode for injected channels
pub type JEXTTRIG_R = crate::BitReader<JEXTTRIG_A>;
///External trigger conversion mode for injected channels
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEXTTRIG_A {
    ///0: Conversion on external event disabled
    Disabled = 0,
    ///1: Conversion on external event enabled
    Enabled = 1,
}
impl From<JEXTTRIG_A> for bool {
    #[inline(always)]
    fn from(variant: JEXTTRIG_A) -> Self {
        variant as u8 != 0
    }
}
impl JEXTTRIG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JEXTTRIG_A {
        match self.bits {
            false => JEXTTRIG_A::Disabled,
            true => JEXTTRIG_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEXTTRIG_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JEXTTRIG_A::Enabled
    }
}
///Field `JEXTTRIG` writer - External trigger conversion mode for injected channels
pub type JEXTTRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, JEXTTRIG_A, O>;
impl<'a, const O: u8> JEXTTRIG_W<'a, O> {
    ///Conversion on external event disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JEXTTRIG_A::Disabled)
    }
    ///Conversion on external event enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JEXTTRIG_A::Enabled)
    }
}
///Field `EXTSEL` reader - External event select for regular group
pub type EXTSEL_R = crate::FieldReader<u8, EXTSEL_A>;
///External event select for regular group
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTSEL_A {
    ///0: Timer 1 CC1 event
    Tim1cc1 = 0,
    ///1: Timer 1 CC2 event
    Tim1cc2 = 1,
    ///2: Timer 1 CC3 event
    Tim1cc3 = 2,
    ///3: Timer 2 CC2 event
    Tim2cc2 = 3,
    ///4: Timer 3 TRGO event
    Tim3trgo = 4,
    ///5: Timer 4 CC4 event
    Tim4cc4 = 5,
    ///6: EXTI line 11/TIM8_TRGO event (TIM8_TRGO is available only in high-density and XL-density devices)
    Exti11 = 6,
    ///7: SWSTART
    Swstart = 7,
}
impl From<EXTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTSEL_A) -> Self {
        variant as _
    }
}
impl EXTSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EXTSEL_A {
        match self.bits {
            0 => EXTSEL_A::Tim1cc1,
            1 => EXTSEL_A::Tim1cc2,
            2 => EXTSEL_A::Tim1cc3,
            3 => EXTSEL_A::Tim2cc2,
            4 => EXTSEL_A::Tim3trgo,
            5 => EXTSEL_A::Tim4cc4,
            6 => EXTSEL_A::Exti11,
            7 => EXTSEL_A::Swstart,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Tim1cc1`
    #[inline(always)]
    pub fn is_tim1cc1(&self) -> bool {
        *self == EXTSEL_A::Tim1cc1
    }
    ///Checks if the value of the field is `Tim1cc2`
    #[inline(always)]
    pub fn is_tim1cc2(&self) -> bool {
        *self == EXTSEL_A::Tim1cc2
    }
    ///Checks if the value of the field is `Tim1cc3`
    #[inline(always)]
    pub fn is_tim1cc3(&self) -> bool {
        *self == EXTSEL_A::Tim1cc3
    }
    ///Checks if the value of the field is `Tim2cc2`
    #[inline(always)]
    pub fn is_tim2cc2(&self) -> bool {
        *self == EXTSEL_A::Tim2cc2
    }
    ///Checks if the value of the field is `Tim3trgo`
    #[inline(always)]
    pub fn is_tim3trgo(&self) -> bool {
        *self == EXTSEL_A::Tim3trgo
    }
    ///Checks if the value of the field is `Tim4cc4`
    #[inline(always)]
    pub fn is_tim4cc4(&self) -> bool {
        *self == EXTSEL_A::Tim4cc4
    }
    ///Checks if the value of the field is `Exti11`
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        *self == EXTSEL_A::Exti11
    }
    ///Checks if the value of the field is `Swstart`
    #[inline(always)]
    pub fn is_swstart(&self) -> bool {
        *self == EXTSEL_A::Swstart
    }
}
///Field `EXTSEL` writer - External event select for regular group
pub type EXTSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR2_SPEC, u8, EXTSEL_A, 3, O>;
impl<'a, const O: u8> EXTSEL_W<'a, O> {
    ///Timer 1 CC1 event
    #[inline(always)]
    pub fn tim1cc1(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim1cc1)
    }
    ///Timer 1 CC2 event
    #[inline(always)]
    pub fn tim1cc2(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim1cc2)
    }
    ///Timer 1 CC3 event
    #[inline(always)]
    pub fn tim1cc3(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim1cc3)
    }
    ///Timer 2 CC2 event
    #[inline(always)]
    pub fn tim2cc2(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim2cc2)
    }
    ///Timer 3 TRGO event
    #[inline(always)]
    pub fn tim3trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim3trgo)
    }
    ///Timer 4 CC4 event
    #[inline(always)]
    pub fn tim4cc4(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim4cc4)
    }
    ///EXTI line 11/TIM8_TRGO event (TIM8_TRGO is available only in high-density and XL-density devices)
    #[inline(always)]
    pub fn exti11(self) -> &'a mut W {
        self.variant(EXTSEL_A::Exti11)
    }
    ///SWSTART
    #[inline(always)]
    pub fn swstart(self) -> &'a mut W {
        self.variant(EXTSEL_A::Swstart)
    }
}
///Field `EXTTRIG` reader - External trigger conversion mode for regular channels
pub type EXTTRIG_R = crate::BitReader<EXTTRIG_A>;
///External trigger conversion mode for regular channels
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTTRIG_A {
    ///0: Conversion on external event disabled
    Disabled = 0,
    ///1: Conversion on external event enabled
    Enabled = 1,
}
impl From<EXTTRIG_A> for bool {
    #[inline(always)]
    fn from(variant: EXTTRIG_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTTRIG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EXTTRIG_A {
        match self.bits {
            false => EXTTRIG_A::Disabled,
            true => EXTTRIG_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTTRIG_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EXTTRIG_A::Enabled
    }
}
///Field `EXTTRIG` writer - External trigger conversion mode for regular channels
pub type EXTTRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, EXTTRIG_A, O>;
impl<'a, const O: u8> EXTTRIG_W<'a, O> {
    ///Conversion on external event disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTTRIG_A::Disabled)
    }
    ///Conversion on external event enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EXTTRIG_A::Enabled)
    }
}
///Field `JSWSTART` reader - Start conversion of injected channels
pub type JSWSTART_R = crate::BitReader<JSWSTARTR_A>;
///Start conversion of injected channels
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSWSTARTR_A {
    ///0: Reset state
    Started = 0,
    ///1: Starting conversion of injected channels
    NotStarted = 1,
}
impl From<JSWSTARTR_A> for bool {
    #[inline(always)]
    fn from(variant: JSWSTARTR_A) -> Self {
        variant as u8 != 0
    }
}
impl JSWSTART_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JSWSTARTR_A {
        match self.bits {
            false => JSWSTARTR_A::Started,
            true => JSWSTARTR_A::NotStarted,
        }
    }
    ///Checks if the value of the field is `Started`
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == JSWSTARTR_A::Started
    }
    ///Checks if the value of the field is `NotStarted`
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == JSWSTARTR_A::NotStarted
    }
}
///Start conversion of injected channels
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSWSTARTW_AW {
    ///1: Start conversion of injected channels
    Start = 1,
}
impl From<JSWSTARTW_AW> for bool {
    #[inline(always)]
    fn from(variant: JSWSTARTW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `JSWSTART` writer - Start conversion of injected channels
pub type JSWSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, JSWSTARTW_AW, O>;
impl<'a, const O: u8> JSWSTART_W<'a, O> {
    ///Start conversion of injected channels
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(JSWSTARTW_AW::Start)
    }
}
///Field `SWSTART` reader - Start conversion of regular channels
pub type SWSTART_R = crate::BitReader<SWSTARTR_A>;
///Start conversion of regular channels
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWSTARTR_A {
    ///0: Reset state
    Started = 0,
    ///1: Starting conversion of regular channels
    NotStarted = 1,
}
impl From<SWSTARTR_A> for bool {
    #[inline(always)]
    fn from(variant: SWSTARTR_A) -> Self {
        variant as u8 != 0
    }
}
impl SWSTART_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SWSTARTR_A {
        match self.bits {
            false => SWSTARTR_A::Started,
            true => SWSTARTR_A::NotStarted,
        }
    }
    ///Checks if the value of the field is `Started`
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == SWSTARTR_A::Started
    }
    ///Checks if the value of the field is `NotStarted`
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == SWSTARTR_A::NotStarted
    }
}
///Start conversion of regular channels
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWSTARTW_AW {
    ///1: Start conversion of regular channels
    Start = 1,
}
impl From<SWSTARTW_AW> for bool {
    #[inline(always)]
    fn from(variant: SWSTARTW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `SWSTART` writer - Start conversion of regular channels
pub type SWSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, SWSTARTW_AW, O>;
impl<'a, const O: u8> SWSTART_W<'a, O> {
    ///Start conversion of regular channels
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SWSTARTW_AW::Start)
    }
}
///Field `TSVREFE` reader - Temperature sensor and VREFINT enable
pub type TSVREFE_R = crate::BitReader<TSVREFE_A>;
///Temperature sensor and VREFINT enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSVREFE_A {
    ///0: Temperature sensor and V_REFINT channel disabled
    Disabled = 0,
    ///1: Temperature sensor and V_REFINT channel enabled
    Enabled = 1,
}
impl From<TSVREFE_A> for bool {
    #[inline(always)]
    fn from(variant: TSVREFE_A) -> Self {
        variant as u8 != 0
    }
}
impl TSVREFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TSVREFE_A {
        match self.bits {
            false => TSVREFE_A::Disabled,
            true => TSVREFE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSVREFE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSVREFE_A::Enabled
    }
}
///Field `TSVREFE` writer - Temperature sensor and VREFINT enable
pub type TSVREFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, TSVREFE_A, O>;
impl<'a, const O: u8> TSVREFE_W<'a, O> {
    ///Temperature sensor and V_REFINT channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSVREFE_A::Disabled)
    }
    ///Temperature sensor and V_REFINT channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSVREFE_A::Enabled)
    }
}
impl R {
    ///Bit 0 - A/D converter ON / OFF
    #[inline(always)]
    pub fn adon(&self) -> ADON_R {
        ADON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Continuous conversion
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - A/D calibration
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Reset calibration
    #[inline(always)]
    pub fn rstcal(&self) -> RSTCAL_R {
        RSTCAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - Direct memory access mode
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - Data alignment
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - External event select for injected group
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bit 15 - External trigger conversion mode for injected channels
    #[inline(always)]
    pub fn jexttrig(&self) -> JEXTTRIG_R {
        JEXTTRIG_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 17:19 - External event select for regular group
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bit 20 - External trigger conversion mode for regular channels
    #[inline(always)]
    pub fn exttrig(&self) -> EXTTRIG_R {
        EXTTRIG_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Start conversion of injected channels
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Start conversion of regular channels
    #[inline(always)]
    pub fn swstart(&self) -> SWSTART_R {
        SWSTART_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Temperature sensor and VREFINT enable
    #[inline(always)]
    pub fn tsvrefe(&self) -> TSVREFE_R {
        TSVREFE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - A/D converter ON / OFF
    #[inline(always)]
    #[must_use]
    pub fn adon(&mut self) -> ADON_W<0> {
        ADON_W::new(self)
    }
    ///Bit 1 - Continuous conversion
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<1> {
        CONT_W::new(self)
    }
    ///Bit 2 - A/D calibration
    #[inline(always)]
    #[must_use]
    pub fn cal(&mut self) -> CAL_W<2> {
        CAL_W::new(self)
    }
    ///Bit 3 - Reset calibration
    #[inline(always)]
    #[must_use]
    pub fn rstcal(&mut self) -> RSTCAL_W<3> {
        RSTCAL_W::new(self)
    }
    ///Bit 8 - Direct memory access mode
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<8> {
        DMA_W::new(self)
    }
    ///Bit 11 - Data alignment
    #[inline(always)]
    #[must_use]
    pub fn align(&mut self) -> ALIGN_W<11> {
        ALIGN_W::new(self)
    }
    ///Bits 12:14 - External event select for injected group
    #[inline(always)]
    #[must_use]
    pub fn jextsel(&mut self) -> JEXTSEL_W<12> {
        JEXTSEL_W::new(self)
    }
    ///Bit 15 - External trigger conversion mode for injected channels
    #[inline(always)]
    #[must_use]
    pub fn jexttrig(&mut self) -> JEXTTRIG_W<15> {
        JEXTTRIG_W::new(self)
    }
    ///Bits 17:19 - External event select for regular group
    #[inline(always)]
    #[must_use]
    pub fn extsel(&mut self) -> EXTSEL_W<17> {
        EXTSEL_W::new(self)
    }
    ///Bit 20 - External trigger conversion mode for regular channels
    #[inline(always)]
    #[must_use]
    pub fn exttrig(&mut self) -> EXTTRIG_W<20> {
        EXTTRIG_W::new(self)
    }
    ///Bit 21 - Start conversion of injected channels
    #[inline(always)]
    #[must_use]
    pub fn jswstart(&mut self) -> JSWSTART_W<21> {
        JSWSTART_W::new(self)
    }
    ///Bit 22 - Start conversion of regular channels
    #[inline(always)]
    #[must_use]
    pub fn swstart(&mut self) -> SWSTART_W<22> {
        SWSTART_W::new(self)
    }
    ///Bit 23 - Temperature sensor and VREFINT enable
    #[inline(always)]
    #[must_use]
    pub fn tsvrefe(&mut self) -> TSVREFE_W<23> {
        TSVREFE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr2](index.html) module
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr2::R](R) reader structure
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr2::W](W) writer structure
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
