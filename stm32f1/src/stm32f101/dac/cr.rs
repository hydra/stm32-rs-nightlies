///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EN1` reader - DAC channel1 enable
pub type EN1_R = crate::BitReader<EN1_A>;
///DAC channel1 enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN1_A {
    ///0: DAC channel X disabled
    Disabled = 0,
    ///1: DAC channel X enabled
    Enabled = 1,
}
impl From<EN1_A> for bool {
    #[inline(always)]
    fn from(variant: EN1_A) -> Self {
        variant as u8 != 0
    }
}
impl EN1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EN1_A {
        match self.bits {
            false => EN1_A::Disabled,
            true => EN1_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN1_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN1_A::Enabled
    }
}
///Field `EN1` writer - DAC channel1 enable
pub type EN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, EN1_A, O>;
impl<'a, const O: u8> EN1_W<'a, O> {
    ///DAC channel X disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN1_A::Disabled)
    }
    ///DAC channel X enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN1_A::Enabled)
    }
}
///Field `BOFF1` reader - DAC channel1 output buffer disable
pub type BOFF1_R = crate::BitReader<BOFF1_A>;
///DAC channel1 output buffer disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOFF1_A {
    ///0: DAC channel X output buffer enabled
    Enabled = 0,
    ///1: DAC channel X output buffer disabled
    Disabled = 1,
}
impl From<BOFF1_A> for bool {
    #[inline(always)]
    fn from(variant: BOFF1_A) -> Self {
        variant as u8 != 0
    }
}
impl BOFF1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BOFF1_A {
        match self.bits {
            false => BOFF1_A::Enabled,
            true => BOFF1_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BOFF1_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BOFF1_A::Disabled
    }
}
///Field `BOFF1` writer - DAC channel1 output buffer disable
pub type BOFF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, BOFF1_A, O>;
impl<'a, const O: u8> BOFF1_W<'a, O> {
    ///DAC channel X output buffer enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BOFF1_A::Enabled)
    }
    ///DAC channel X output buffer disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BOFF1_A::Disabled)
    }
}
///Field `TEN1` reader - DAC channel1 trigger enable
pub type TEN1_R = crate::BitReader<TEN1_A>;
///DAC channel1 trigger enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEN1_A {
    ///0: DAC channel X trigger disabled
    Disabled = 0,
    ///1: DAC channel X trigger enabled
    Enabled = 1,
}
impl From<TEN1_A> for bool {
    #[inline(always)]
    fn from(variant: TEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl TEN1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TEN1_A {
        match self.bits {
            false => TEN1_A::Disabled,
            true => TEN1_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEN1_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEN1_A::Enabled
    }
}
///Field `TEN1` writer - DAC channel1 trigger enable
pub type TEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TEN1_A, O>;
impl<'a, const O: u8> TEN1_W<'a, O> {
    ///DAC channel X trigger disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEN1_A::Disabled)
    }
    ///DAC channel X trigger enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEN1_A::Enabled)
    }
}
///Field `TSEL1` reader - DAC channel1 trigger selection
pub type TSEL1_R = crate::FieldReader<u8, TSEL1_A>;
///DAC channel1 trigger selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSEL1_A {
    ///0: Timer 6 TRGO event
    Tim6Trgo = 0,
    ///1: Timer 3 TRGO event
    Tim3Trgo = 1,
    ///2: Timer 7 TRGO event
    Tim7Trgo = 2,
    ///3: Timer 15 TRGO event
    Tim15Trgo = 3,
    ///4: Timer 2 TRGO event
    Tim2Trgo = 4,
    ///6: EXTI line9
    Exti9 = 6,
    ///7: Software trigger
    Software = 7,
}
impl From<TSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEL1_A) -> Self {
        variant as _
    }
}
impl TSEL1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<TSEL1_A> {
        match self.bits {
            0 => Some(TSEL1_A::Tim6Trgo),
            1 => Some(TSEL1_A::Tim3Trgo),
            2 => Some(TSEL1_A::Tim7Trgo),
            3 => Some(TSEL1_A::Tim15Trgo),
            4 => Some(TSEL1_A::Tim2Trgo),
            6 => Some(TSEL1_A::Exti9),
            7 => Some(TSEL1_A::Software),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Tim6Trgo`
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        *self == TSEL1_A::Tim6Trgo
    }
    ///Checks if the value of the field is `Tim3Trgo`
    #[inline(always)]
    pub fn is_tim3_trgo(&self) -> bool {
        *self == TSEL1_A::Tim3Trgo
    }
    ///Checks if the value of the field is `Tim7Trgo`
    #[inline(always)]
    pub fn is_tim7_trgo(&self) -> bool {
        *self == TSEL1_A::Tim7Trgo
    }
    ///Checks if the value of the field is `Tim15Trgo`
    #[inline(always)]
    pub fn is_tim15_trgo(&self) -> bool {
        *self == TSEL1_A::Tim15Trgo
    }
    ///Checks if the value of the field is `Tim2Trgo`
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == TSEL1_A::Tim2Trgo
    }
    ///Checks if the value of the field is `Exti9`
    #[inline(always)]
    pub fn is_exti9(&self) -> bool {
        *self == TSEL1_A::Exti9
    }
    ///Checks if the value of the field is `Software`
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == TSEL1_A::Software
    }
}
///Field `TSEL1` writer - DAC channel1 trigger selection
pub type TSEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, TSEL1_A, 3, O>;
impl<'a, const O: u8> TSEL1_W<'a, O> {
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::Tim6Trgo)
    }
    ///Timer 3 TRGO event
    #[inline(always)]
    pub fn tim3_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::Tim3Trgo)
    }
    ///Timer 7 TRGO event
    #[inline(always)]
    pub fn tim7_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::Tim7Trgo)
    }
    ///Timer 15 TRGO event
    #[inline(always)]
    pub fn tim15_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::Tim15Trgo)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut W {
        self.variant(TSEL1_A::Tim2Trgo)
    }
    ///EXTI line9
    #[inline(always)]
    pub fn exti9(self) -> &'a mut W {
        self.variant(TSEL1_A::Exti9)
    }
    ///Software trigger
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(TSEL1_A::Software)
    }
}
///Field `WAVE1` reader - DAC channel1 noise/triangle wave generation enable
pub type WAVE1_R = crate::FieldReader<u8, WAVE1_A>;
///DAC channel1 noise/triangle wave generation enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WAVE1_A {
    ///0: Wave generation disabled
    Disabled = 0,
    ///1: Noise wave generation enabled
    Noise = 1,
    ///2: Triangle wave generation enabled
    Triangle = 2,
}
impl From<WAVE1_A> for u8 {
    #[inline(always)]
    fn from(variant: WAVE1_A) -> Self {
        variant as _
    }
}
impl WAVE1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<WAVE1_A> {
        match self.bits {
            0 => Some(WAVE1_A::Disabled),
            1 => Some(WAVE1_A::Noise),
            2 => Some(WAVE1_A::Triangle),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAVE1_A::Disabled
    }
    ///Checks if the value of the field is `Noise`
    #[inline(always)]
    pub fn is_noise(&self) -> bool {
        *self == WAVE1_A::Noise
    }
    ///Checks if the value of the field is `Triangle`
    #[inline(always)]
    pub fn is_triangle(&self) -> bool {
        *self == WAVE1_A::Triangle
    }
}
///Field `WAVE1` writer - DAC channel1 noise/triangle wave generation enable
pub type WAVE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, WAVE1_A, 2, O>;
impl<'a, const O: u8> WAVE1_W<'a, O> {
    ///Wave generation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAVE1_A::Disabled)
    }
    ///Noise wave generation enabled
    #[inline(always)]
    pub fn noise(self) -> &'a mut W {
        self.variant(WAVE1_A::Noise)
    }
    ///Triangle wave generation enabled
    #[inline(always)]
    pub fn triangle(self) -> &'a mut W {
        self.variant(WAVE1_A::Triangle)
    }
}
///Field `MAMP1` reader - DAC channel1 mask/amplitude selector
pub type MAMP1_R = crate::FieldReader<u8, u8>;
///Field `MAMP1` writer - DAC channel1 mask/amplitude selector
pub type MAMP1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, u8, 4, O>;
///Field `DMAEN1` reader - DAC channel1 DMA enable
pub type DMAEN1_R = crate::BitReader<DMAEN1_A>;
///DAC channel1 DMA enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN1_A {
    ///0: DAC channel X DMA mode disabled
    Disabled = 0,
    ///1: DAC channel X DMA mode enabled
    Enabled = 1,
}
impl From<DMAEN1_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAEN1_A {
        match self.bits {
            false => DMAEN1_A::Disabled,
            true => DMAEN1_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN1_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN1_A::Enabled
    }
}
///Field `DMAEN1` writer - DAC channel1 DMA enable
pub type DMAEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DMAEN1_A, O>;
impl<'a, const O: u8> DMAEN1_W<'a, O> {
    ///DAC channel X DMA mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN1_A::Disabled)
    }
    ///DAC channel X DMA mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN1_A::Enabled)
    }
}
///Field `BOFF2` reader - DAC channel2 output buffer disable
pub use BOFF1_R as BOFF2_R;
///Field `BOFF2` writer - DAC channel2 output buffer disable
pub use BOFF1_W as BOFF2_W;
///Field `EN2` reader - DAC channel2 enable
pub use EN1_R as EN2_R;
///Field `EN2` writer - DAC channel2 enable
pub use EN1_W as EN2_W;
///Field `TEN2` reader - DAC channel2 trigger enable
pub use TEN1_R as TEN2_R;
///Field `TEN2` writer - DAC channel2 trigger enable
pub use TEN1_W as TEN2_W;
///Field `TSEL2` reader - DAC channel2 trigger selection
pub type TSEL2_R = crate::FieldReader<u8, TSEL2_A>;
///DAC channel2 trigger selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSEL2_A {
    ///0: Timer 6 TRGO event
    Tim6Trgo = 0,
    ///1: Timer 8 TRGO event
    Tim8Trgo = 1,
    ///2: Timer 7 TRGO event
    Tim7Trgo = 2,
    ///3: Timer 5 TRGO event
    Tim5Trgo = 3,
    ///4: Timer 2 TRGO event
    Tim2Trgo = 4,
    ///5: Timer 4 TRGO event
    Tim4Trgo = 5,
    ///6: EXTI line9
    Exti9 = 6,
    ///7: Software trigger
    Software = 7,
}
impl From<TSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: TSEL2_A) -> Self {
        variant as _
    }
}
impl TSEL2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TSEL2_A {
        match self.bits {
            0 => TSEL2_A::Tim6Trgo,
            1 => TSEL2_A::Tim8Trgo,
            2 => TSEL2_A::Tim7Trgo,
            3 => TSEL2_A::Tim5Trgo,
            4 => TSEL2_A::Tim2Trgo,
            5 => TSEL2_A::Tim4Trgo,
            6 => TSEL2_A::Exti9,
            7 => TSEL2_A::Software,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Tim6Trgo`
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        *self == TSEL2_A::Tim6Trgo
    }
    ///Checks if the value of the field is `Tim8Trgo`
    #[inline(always)]
    pub fn is_tim8_trgo(&self) -> bool {
        *self == TSEL2_A::Tim8Trgo
    }
    ///Checks if the value of the field is `Tim7Trgo`
    #[inline(always)]
    pub fn is_tim7_trgo(&self) -> bool {
        *self == TSEL2_A::Tim7Trgo
    }
    ///Checks if the value of the field is `Tim5Trgo`
    #[inline(always)]
    pub fn is_tim5_trgo(&self) -> bool {
        *self == TSEL2_A::Tim5Trgo
    }
    ///Checks if the value of the field is `Tim2Trgo`
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == TSEL2_A::Tim2Trgo
    }
    ///Checks if the value of the field is `Tim4Trgo`
    #[inline(always)]
    pub fn is_tim4_trgo(&self) -> bool {
        *self == TSEL2_A::Tim4Trgo
    }
    ///Checks if the value of the field is `Exti9`
    #[inline(always)]
    pub fn is_exti9(&self) -> bool {
        *self == TSEL2_A::Exti9
    }
    ///Checks if the value of the field is `Software`
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == TSEL2_A::Software
    }
}
///Field `TSEL2` writer - DAC channel2 trigger selection
pub type TSEL2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, TSEL2_A, 3, O>;
impl<'a, const O: u8> TSEL2_W<'a, O> {
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut W {
        self.variant(TSEL2_A::Tim6Trgo)
    }
    ///Timer 8 TRGO event
    #[inline(always)]
    pub fn tim8_trgo(self) -> &'a mut W {
        self.variant(TSEL2_A::Tim8Trgo)
    }
    ///Timer 7 TRGO event
    #[inline(always)]
    pub fn tim7_trgo(self) -> &'a mut W {
        self.variant(TSEL2_A::Tim7Trgo)
    }
    ///Timer 5 TRGO event
    #[inline(always)]
    pub fn tim5_trgo(self) -> &'a mut W {
        self.variant(TSEL2_A::Tim5Trgo)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut W {
        self.variant(TSEL2_A::Tim2Trgo)
    }
    ///Timer 4 TRGO event
    #[inline(always)]
    pub fn tim4_trgo(self) -> &'a mut W {
        self.variant(TSEL2_A::Tim4Trgo)
    }
    ///EXTI line9
    #[inline(always)]
    pub fn exti9(self) -> &'a mut W {
        self.variant(TSEL2_A::Exti9)
    }
    ///Software trigger
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(TSEL2_A::Software)
    }
}
///Field `WAVE2` reader - DAC channel2 noise/triangle wave generation enable
pub type WAVE2_R = crate::FieldReader<u8, WAVE2_A>;
///DAC channel2 noise/triangle wave generation enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WAVE2_A {
    ///0: Wave generation disabled
    Disabled = 0,
    ///1: Noise wave generation enabled
    Noise = 1,
    ///2: Triangle wave generation enabled
    Triangle = 2,
}
impl From<WAVE2_A> for u8 {
    #[inline(always)]
    fn from(variant: WAVE2_A) -> Self {
        variant as _
    }
}
impl WAVE2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<WAVE2_A> {
        match self.bits {
            0 => Some(WAVE2_A::Disabled),
            1 => Some(WAVE2_A::Noise),
            2 => Some(WAVE2_A::Triangle),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAVE2_A::Disabled
    }
    ///Checks if the value of the field is `Noise`
    #[inline(always)]
    pub fn is_noise(&self) -> bool {
        *self == WAVE2_A::Noise
    }
    ///Checks if the value of the field is `Triangle`
    #[inline(always)]
    pub fn is_triangle(&self) -> bool {
        *self == WAVE2_A::Triangle
    }
}
///Field `WAVE2` writer - DAC channel2 noise/triangle wave generation enable
pub type WAVE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, WAVE2_A, 2, O>;
impl<'a, const O: u8> WAVE2_W<'a, O> {
    ///Wave generation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAVE2_A::Disabled)
    }
    ///Noise wave generation enabled
    #[inline(always)]
    pub fn noise(self) -> &'a mut W {
        self.variant(WAVE2_A::Noise)
    }
    ///Triangle wave generation enabled
    #[inline(always)]
    pub fn triangle(self) -> &'a mut W {
        self.variant(WAVE2_A::Triangle)
    }
}
///Field `MAMP2` reader - DAC channel2 mask/amplitude selector
pub type MAMP2_R = crate::FieldReader<u8, u8>;
///Field `MAMP2` writer - DAC channel2 mask/amplitude selector
pub type MAMP2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, u8, 4, O>;
///Field `DMAEN2` reader - DAC channel2 DMA enable
pub use DMAEN1_R as DMAEN2_R;
///Field `DMAEN2` writer - DAC channel2 DMA enable
pub use DMAEN1_W as DMAEN2_W;
impl R {
    ///Bit 0 - DAC channel1 enable
    #[inline(always)]
    pub fn en1(&self) -> EN1_R {
        EN1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DAC channel1 output buffer disable
    #[inline(always)]
    pub fn boff1(&self) -> BOFF1_R {
        BOFF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DAC channel1 trigger enable
    #[inline(always)]
    pub fn ten1(&self) -> TEN1_R {
        TEN1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:5 - DAC channel1 trigger selection
    #[inline(always)]
    pub fn tsel1(&self) -> TSEL1_R {
        TSEL1_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:7 - DAC channel1 noise/triangle wave generation enable
    #[inline(always)]
    pub fn wave1(&self) -> WAVE1_R {
        WAVE1_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:11 - DAC channel1 mask/amplitude selector
    #[inline(always)]
    pub fn mamp1(&self) -> MAMP1_R {
        MAMP1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - DAC channel1 DMA enable
    #[inline(always)]
    pub fn dmaen1(&self) -> DMAEN1_R {
        DMAEN1_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - DAC channel2 enable
    #[inline(always)]
    pub fn en2(&self) -> EN2_R {
        EN2_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DAC channel2 output buffer disable
    #[inline(always)]
    pub fn boff2(&self) -> BOFF2_R {
        BOFF2_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DAC channel2 trigger enable
    #[inline(always)]
    pub fn ten2(&self) -> TEN2_R {
        TEN2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:21 - DAC channel2 trigger selection
    #[inline(always)]
    pub fn tsel2(&self) -> TSEL2_R {
        TSEL2_R::new(((self.bits >> 19) & 7) as u8)
    }
    ///Bits 22:23 - DAC channel2 noise/triangle wave generation enable
    #[inline(always)]
    pub fn wave2(&self) -> WAVE2_R {
        WAVE2_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:27 - DAC channel2 mask/amplitude selector
    #[inline(always)]
    pub fn mamp2(&self) -> MAMP2_R {
        MAMP2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bit 28 - DAC channel2 DMA enable
    #[inline(always)]
    pub fn dmaen2(&self) -> DMAEN2_R {
        DMAEN2_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DAC channel1 enable
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> EN1_W<0> {
        EN1_W::new(self)
    }
    ///Bit 1 - DAC channel1 output buffer disable
    #[inline(always)]
    #[must_use]
    pub fn boff1(&mut self) -> BOFF1_W<1> {
        BOFF1_W::new(self)
    }
    ///Bit 2 - DAC channel1 trigger enable
    #[inline(always)]
    #[must_use]
    pub fn ten1(&mut self) -> TEN1_W<2> {
        TEN1_W::new(self)
    }
    ///Bits 3:5 - DAC channel1 trigger selection
    #[inline(always)]
    #[must_use]
    pub fn tsel1(&mut self) -> TSEL1_W<3> {
        TSEL1_W::new(self)
    }
    ///Bits 6:7 - DAC channel1 noise/triangle wave generation enable
    #[inline(always)]
    #[must_use]
    pub fn wave1(&mut self) -> WAVE1_W<6> {
        WAVE1_W::new(self)
    }
    ///Bits 8:11 - DAC channel1 mask/amplitude selector
    #[inline(always)]
    #[must_use]
    pub fn mamp1(&mut self) -> MAMP1_W<8> {
        MAMP1_W::new(self)
    }
    ///Bit 12 - DAC channel1 DMA enable
    #[inline(always)]
    #[must_use]
    pub fn dmaen1(&mut self) -> DMAEN1_W<12> {
        DMAEN1_W::new(self)
    }
    ///Bit 16 - DAC channel2 enable
    #[inline(always)]
    #[must_use]
    pub fn en2(&mut self) -> EN2_W<16> {
        EN2_W::new(self)
    }
    ///Bit 17 - DAC channel2 output buffer disable
    #[inline(always)]
    #[must_use]
    pub fn boff2(&mut self) -> BOFF2_W<17> {
        BOFF2_W::new(self)
    }
    ///Bit 18 - DAC channel2 trigger enable
    #[inline(always)]
    #[must_use]
    pub fn ten2(&mut self) -> TEN2_W<18> {
        TEN2_W::new(self)
    }
    ///Bits 19:21 - DAC channel2 trigger selection
    #[inline(always)]
    #[must_use]
    pub fn tsel2(&mut self) -> TSEL2_W<19> {
        TSEL2_W::new(self)
    }
    ///Bits 22:23 - DAC channel2 noise/triangle wave generation enable
    #[inline(always)]
    #[must_use]
    pub fn wave2(&mut self) -> WAVE2_W<22> {
        WAVE2_W::new(self)
    }
    ///Bits 24:27 - DAC channel2 mask/amplitude selector
    #[inline(always)]
    #[must_use]
    pub fn mamp2(&mut self) -> MAMP2_W<24> {
        MAMP2_W::new(self)
    }
    ///Bit 28 - DAC channel2 DMA enable
    #[inline(always)]
    #[must_use]
    pub fn dmaen2(&mut self) -> DMAEN2_W<28> {
        DMAEN2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Control register (DAC_CR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
