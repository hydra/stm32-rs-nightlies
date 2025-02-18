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
///Field `ADON` reader - A/D Converter ON / OFF
pub type ADON_R = crate::BitReader<ADON_A>;
///A/D Converter ON / OFF
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADON_A {
    ///0: Disable ADC conversion and go to power down mode
    Disabled = 0,
    ///1: Enable ADC
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
///Field `ADON` writer - A/D Converter ON / OFF
pub type ADON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, ADON_A, O>;
impl<'a, const O: u8> ADON_W<'a, O> {
    ///Disable ADC conversion and go to power down mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADON_A::Disabled)
    }
    ///Enable ADC
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
///Field `DMA` reader - Direct memory access mode (for single ADC mode)
pub type DMA_R = crate::BitReader<DMA_A>;
///Direct memory access mode (for single ADC mode)
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
///Field `DMA` writer - Direct memory access mode (for single ADC mode)
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
///Field `DDS` reader - DMA disable selection (for single ADC mode)
pub type DDS_R = crate::BitReader<DDS_A>;
///DMA disable selection (for single ADC mode)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDS_A {
    ///0: No new DMA request is issued after the last transfer
    Single = 0,
    ///1: DMA requests are issued as long as data are converted and DMA=1
    Continuous = 1,
}
impl From<DDS_A> for bool {
    #[inline(always)]
    fn from(variant: DDS_A) -> Self {
        variant as u8 != 0
    }
}
impl DDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DDS_A {
        match self.bits {
            false => DDS_A::Single,
            true => DDS_A::Continuous,
        }
    }
    ///Checks if the value of the field is `Single`
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == DDS_A::Single
    }
    ///Checks if the value of the field is `Continuous`
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == DDS_A::Continuous
    }
}
///Field `DDS` writer - DMA disable selection (for single ADC mode)
pub type DDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, DDS_A, O>;
impl<'a, const O: u8> DDS_W<'a, O> {
    ///No new DMA request is issued after the last transfer
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(DDS_A::Single)
    }
    ///DMA requests are issued as long as data are converted and DMA=1
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(DDS_A::Continuous)
    }
}
///Field `EOCS` reader - End of conversion selection
pub type EOCS_R = crate::BitReader<EOCS_A>;
///End of conversion selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCS_A {
    ///0: The EOC bit is set at the end of each sequence of regular conversions
    EachSequence = 0,
    ///1: The EOC bit is set at the end of each regular conversion
    EachConversion = 1,
}
impl From<EOCS_A> for bool {
    #[inline(always)]
    fn from(variant: EOCS_A) -> Self {
        variant as u8 != 0
    }
}
impl EOCS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOCS_A {
        match self.bits {
            false => EOCS_A::EachSequence,
            true => EOCS_A::EachConversion,
        }
    }
    ///Checks if the value of the field is `EachSequence`
    #[inline(always)]
    pub fn is_each_sequence(&self) -> bool {
        *self == EOCS_A::EachSequence
    }
    ///Checks if the value of the field is `EachConversion`
    #[inline(always)]
    pub fn is_each_conversion(&self) -> bool {
        *self == EOCS_A::EachConversion
    }
}
///Field `EOCS` writer - End of conversion selection
pub type EOCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, EOCS_A, O>;
impl<'a, const O: u8> EOCS_W<'a, O> {
    ///The EOC bit is set at the end of each sequence of regular conversions
    #[inline(always)]
    pub fn each_sequence(self) -> &'a mut W {
        self.variant(EOCS_A::EachSequence)
    }
    ///The EOC bit is set at the end of each regular conversion
    #[inline(always)]
    pub fn each_conversion(self) -> &'a mut W {
        self.variant(EOCS_A::EachConversion)
    }
}
///Field `ALIGN` reader - Data alignment
pub type ALIGN_R = crate::BitReader<ALIGN_A>;
///Data alignment
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALIGN_A {
    ///0: Right alignment
    Right = 0,
    ///1: Left alignment
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
    ///Right alignment
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(ALIGN_A::Right)
    }
    ///Left alignment
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
    ///0: Timer 1 CC4 event
    Tim1cc4 = 0,
    ///1: Timer 1 TRGO event
    Tim1trgo = 1,
    ///2: Timer 2 CC1 event
    Tim2cc1 = 2,
    ///3: Timer 2 TRGO event
    Tim2trgo = 3,
    ///4: Timer 3 CC2 event
    Tim3cc2 = 4,
    ///5: Timer 3 CC4 event
    Tim3cc4 = 5,
    ///6: Timer 4 CC1 event
    Tim4cc1 = 6,
    ///7: Timer 4 CC2 event
    Tim4cc2 = 7,
    ///8: Timer 4 CC3 event
    Tim4cc3 = 8,
    ///9: Timer 4 TRGO event
    Tim4trgo = 9,
    ///10: Timer 5 CC4 event
    Tim5cc4 = 10,
    ///11: Timer 5 TRGO event
    Tim5trgo = 11,
    ///12: Timer 8 CC2 event
    Tim8cc2 = 12,
    ///13: Timer 8 CC3 event
    Tim8cc3 = 13,
    ///14: Timer 8 CC4 event
    Tim8cc4 = 14,
    ///15: EXTI line 15
    Exti15 = 15,
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
            0 => JEXTSEL_A::Tim1cc4,
            1 => JEXTSEL_A::Tim1trgo,
            2 => JEXTSEL_A::Tim2cc1,
            3 => JEXTSEL_A::Tim2trgo,
            4 => JEXTSEL_A::Tim3cc2,
            5 => JEXTSEL_A::Tim3cc4,
            6 => JEXTSEL_A::Tim4cc1,
            7 => JEXTSEL_A::Tim4cc2,
            8 => JEXTSEL_A::Tim4cc3,
            9 => JEXTSEL_A::Tim4trgo,
            10 => JEXTSEL_A::Tim5cc4,
            11 => JEXTSEL_A::Tim5trgo,
            12 => JEXTSEL_A::Tim8cc2,
            13 => JEXTSEL_A::Tim8cc3,
            14 => JEXTSEL_A::Tim8cc4,
            15 => JEXTSEL_A::Exti15,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Tim1cc4`
    #[inline(always)]
    pub fn is_tim1cc4(&self) -> bool {
        *self == JEXTSEL_A::Tim1cc4
    }
    ///Checks if the value of the field is `Tim1trgo`
    #[inline(always)]
    pub fn is_tim1trgo(&self) -> bool {
        *self == JEXTSEL_A::Tim1trgo
    }
    ///Checks if the value of the field is `Tim2cc1`
    #[inline(always)]
    pub fn is_tim2cc1(&self) -> bool {
        *self == JEXTSEL_A::Tim2cc1
    }
    ///Checks if the value of the field is `Tim2trgo`
    #[inline(always)]
    pub fn is_tim2trgo(&self) -> bool {
        *self == JEXTSEL_A::Tim2trgo
    }
    ///Checks if the value of the field is `Tim3cc2`
    #[inline(always)]
    pub fn is_tim3cc2(&self) -> bool {
        *self == JEXTSEL_A::Tim3cc2
    }
    ///Checks if the value of the field is `Tim3cc4`
    #[inline(always)]
    pub fn is_tim3cc4(&self) -> bool {
        *self == JEXTSEL_A::Tim3cc4
    }
    ///Checks if the value of the field is `Tim4cc1`
    #[inline(always)]
    pub fn is_tim4cc1(&self) -> bool {
        *self == JEXTSEL_A::Tim4cc1
    }
    ///Checks if the value of the field is `Tim4cc2`
    #[inline(always)]
    pub fn is_tim4cc2(&self) -> bool {
        *self == JEXTSEL_A::Tim4cc2
    }
    ///Checks if the value of the field is `Tim4cc3`
    #[inline(always)]
    pub fn is_tim4cc3(&self) -> bool {
        *self == JEXTSEL_A::Tim4cc3
    }
    ///Checks if the value of the field is `Tim4trgo`
    #[inline(always)]
    pub fn is_tim4trgo(&self) -> bool {
        *self == JEXTSEL_A::Tim4trgo
    }
    ///Checks if the value of the field is `Tim5cc4`
    #[inline(always)]
    pub fn is_tim5cc4(&self) -> bool {
        *self == JEXTSEL_A::Tim5cc4
    }
    ///Checks if the value of the field is `Tim5trgo`
    #[inline(always)]
    pub fn is_tim5trgo(&self) -> bool {
        *self == JEXTSEL_A::Tim5trgo
    }
    ///Checks if the value of the field is `Tim8cc2`
    #[inline(always)]
    pub fn is_tim8cc2(&self) -> bool {
        *self == JEXTSEL_A::Tim8cc2
    }
    ///Checks if the value of the field is `Tim8cc3`
    #[inline(always)]
    pub fn is_tim8cc3(&self) -> bool {
        *self == JEXTSEL_A::Tim8cc3
    }
    ///Checks if the value of the field is `Tim8cc4`
    #[inline(always)]
    pub fn is_tim8cc4(&self) -> bool {
        *self == JEXTSEL_A::Tim8cc4
    }
    ///Checks if the value of the field is `Exti15`
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        *self == JEXTSEL_A::Exti15
    }
}
///Field `JEXTSEL` writer - External event select for injected group
pub type JEXTSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CR2_SPEC, u8, JEXTSEL_A, 4, O>;
impl<'a, const O: u8> JEXTSEL_W<'a, O> {
    ///Timer 1 CC4 event
    #[inline(always)]
    pub fn tim1cc4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim1cc4)
    }
    ///Timer 1 TRGO event
    #[inline(always)]
    pub fn tim1trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim1trgo)
    }
    ///Timer 2 CC1 event
    #[inline(always)]
    pub fn tim2cc1(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim2cc1)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim2trgo)
    }
    ///Timer 3 CC2 event
    #[inline(always)]
    pub fn tim3cc2(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim3cc2)
    }
    ///Timer 3 CC4 event
    #[inline(always)]
    pub fn tim3cc4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim3cc4)
    }
    ///Timer 4 CC1 event
    #[inline(always)]
    pub fn tim4cc1(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim4cc1)
    }
    ///Timer 4 CC2 event
    #[inline(always)]
    pub fn tim4cc2(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim4cc2)
    }
    ///Timer 4 CC3 event
    #[inline(always)]
    pub fn tim4cc3(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim4cc3)
    }
    ///Timer 4 TRGO event
    #[inline(always)]
    pub fn tim4trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim4trgo)
    }
    ///Timer 5 CC4 event
    #[inline(always)]
    pub fn tim5cc4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim5cc4)
    }
    ///Timer 5 TRGO event
    #[inline(always)]
    pub fn tim5trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim5trgo)
    }
    ///Timer 8 CC2 event
    #[inline(always)]
    pub fn tim8cc2(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim8cc2)
    }
    ///Timer 8 CC3 event
    #[inline(always)]
    pub fn tim8cc3(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim8cc3)
    }
    ///Timer 8 CC4 event
    #[inline(always)]
    pub fn tim8cc4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim8cc4)
    }
    ///EXTI line 15
    #[inline(always)]
    pub fn exti15(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Exti15)
    }
}
///Field `JEXTEN` reader - External trigger enable for injected channels
pub type JEXTEN_R = crate::FieldReader<u8, JEXTEN_A>;
///External trigger enable for injected channels
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JEXTEN_A {
    ///0: Trigger detection disabled
    Disabled = 0,
    ///1: Trigger detection on the rising edge
    RisingEdge = 1,
    ///2: Trigger detection on the falling edge
    FallingEdge = 2,
    ///3: Trigger detection on both the rising and falling edges
    BothEdges = 3,
}
impl From<JEXTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: JEXTEN_A) -> Self {
        variant as _
    }
}
impl JEXTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JEXTEN_A {
        match self.bits {
            0 => JEXTEN_A::Disabled,
            1 => JEXTEN_A::RisingEdge,
            2 => JEXTEN_A::FallingEdge,
            3 => JEXTEN_A::BothEdges,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEXTEN_A::Disabled
    }
    ///Checks if the value of the field is `RisingEdge`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == JEXTEN_A::RisingEdge
    }
    ///Checks if the value of the field is `FallingEdge`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == JEXTEN_A::FallingEdge
    }
    ///Checks if the value of the field is `BothEdges`
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == JEXTEN_A::BothEdges
    }
}
///Field `JEXTEN` writer - External trigger enable for injected channels
pub type JEXTEN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR2_SPEC, u8, JEXTEN_A, 2, O>;
impl<'a, const O: u8> JEXTEN_W<'a, O> {
    ///Trigger detection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JEXTEN_A::Disabled)
    }
    ///Trigger detection on the rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(JEXTEN_A::RisingEdge)
    }
    ///Trigger detection on the falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(JEXTEN_A::FallingEdge)
    }
    ///Trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(JEXTEN_A::BothEdges)
    }
}
///Field `JSWSTART` reader - Start conversion of injected channels
pub type JSWSTART_R = crate::BitReader<JSWSTARTW_A>;
///Start conversion of injected channels
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSWSTARTW_A {
    ///1: Starts conversion of injected channels
    Start = 1,
}
impl From<JSWSTARTW_A> for bool {
    #[inline(always)]
    fn from(variant: JSWSTARTW_A) -> Self {
        variant as u8 != 0
    }
}
impl JSWSTART_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<JSWSTARTW_A> {
        match self.bits {
            true => Some(JSWSTARTW_A::Start),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Start`
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == JSWSTARTW_A::Start
    }
}
///Field `JSWSTART` writer - Start conversion of injected channels
pub type JSWSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, JSWSTARTW_A, O>;
impl<'a, const O: u8> JSWSTART_W<'a, O> {
    ///Starts conversion of injected channels
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(JSWSTARTW_A::Start)
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
    ///4: Timer 2 CC3 event
    Tim2cc3 = 4,
    ///5: Timer 2 CC4 event
    Tim2cc4 = 5,
    ///6: Timer 2 TRGO event
    Tim2trgo = 6,
    ///7: Timer 3 CC1 event
    Tim3cc1 = 7,
    ///8: Timer 3 TRGO event
    Tim3trgo = 8,
    ///9: Timer 4 CC4 event
    Tim4cc4 = 9,
    ///10: Timer 5 CC1 event
    Tim5cc1 = 10,
    ///11: Timer 5 CC2 event
    Tim5cc2 = 11,
    ///12: Timer 5 CC3 event
    Tim5cc3 = 12,
    ///13: Timer 8 CC1 event
    Tim8cc1 = 13,
    ///14: Timer 8 TRGO event
    Tim8trgo = 14,
    ///15: EXTI line 11
    Exti11 = 15,
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
            4 => EXTSEL_A::Tim2cc3,
            5 => EXTSEL_A::Tim2cc4,
            6 => EXTSEL_A::Tim2trgo,
            7 => EXTSEL_A::Tim3cc1,
            8 => EXTSEL_A::Tim3trgo,
            9 => EXTSEL_A::Tim4cc4,
            10 => EXTSEL_A::Tim5cc1,
            11 => EXTSEL_A::Tim5cc2,
            12 => EXTSEL_A::Tim5cc3,
            13 => EXTSEL_A::Tim8cc1,
            14 => EXTSEL_A::Tim8trgo,
            15 => EXTSEL_A::Exti11,
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
    ///Checks if the value of the field is `Tim2cc3`
    #[inline(always)]
    pub fn is_tim2cc3(&self) -> bool {
        *self == EXTSEL_A::Tim2cc3
    }
    ///Checks if the value of the field is `Tim2cc4`
    #[inline(always)]
    pub fn is_tim2cc4(&self) -> bool {
        *self == EXTSEL_A::Tim2cc4
    }
    ///Checks if the value of the field is `Tim2trgo`
    #[inline(always)]
    pub fn is_tim2trgo(&self) -> bool {
        *self == EXTSEL_A::Tim2trgo
    }
    ///Checks if the value of the field is `Tim3cc1`
    #[inline(always)]
    pub fn is_tim3cc1(&self) -> bool {
        *self == EXTSEL_A::Tim3cc1
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
    ///Checks if the value of the field is `Tim5cc1`
    #[inline(always)]
    pub fn is_tim5cc1(&self) -> bool {
        *self == EXTSEL_A::Tim5cc1
    }
    ///Checks if the value of the field is `Tim5cc2`
    #[inline(always)]
    pub fn is_tim5cc2(&self) -> bool {
        *self == EXTSEL_A::Tim5cc2
    }
    ///Checks if the value of the field is `Tim5cc3`
    #[inline(always)]
    pub fn is_tim5cc3(&self) -> bool {
        *self == EXTSEL_A::Tim5cc3
    }
    ///Checks if the value of the field is `Tim8cc1`
    #[inline(always)]
    pub fn is_tim8cc1(&self) -> bool {
        *self == EXTSEL_A::Tim8cc1
    }
    ///Checks if the value of the field is `Tim8trgo`
    #[inline(always)]
    pub fn is_tim8trgo(&self) -> bool {
        *self == EXTSEL_A::Tim8trgo
    }
    ///Checks if the value of the field is `Exti11`
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        *self == EXTSEL_A::Exti11
    }
}
///Field `EXTSEL` writer - External event select for regular group
pub type EXTSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR2_SPEC, u8, EXTSEL_A, 4, O>;
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
    ///Timer 2 CC3 event
    #[inline(always)]
    pub fn tim2cc3(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim2cc3)
    }
    ///Timer 2 CC4 event
    #[inline(always)]
    pub fn tim2cc4(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim2cc4)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim2trgo)
    }
    ///Timer 3 CC1 event
    #[inline(always)]
    pub fn tim3cc1(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim3cc1)
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
    ///Timer 5 CC1 event
    #[inline(always)]
    pub fn tim5cc1(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim5cc1)
    }
    ///Timer 5 CC2 event
    #[inline(always)]
    pub fn tim5cc2(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim5cc2)
    }
    ///Timer 5 CC3 event
    #[inline(always)]
    pub fn tim5cc3(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim5cc3)
    }
    ///Timer 8 CC1 event
    #[inline(always)]
    pub fn tim8cc1(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim8cc1)
    }
    ///Timer 8 TRGO event
    #[inline(always)]
    pub fn tim8trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim8trgo)
    }
    ///EXTI line 11
    #[inline(always)]
    pub fn exti11(self) -> &'a mut W {
        self.variant(EXTSEL_A::Exti11)
    }
}
///Field `EXTEN` reader - External trigger enable for regular channels
pub type EXTEN_R = crate::FieldReader<u8, EXTEN_A>;
///External trigger enable for regular channels
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTEN_A {
    ///0: Trigger detection disabled
    Disabled = 0,
    ///1: Trigger detection on the rising edge
    RisingEdge = 1,
    ///2: Trigger detection on the falling edge
    FallingEdge = 2,
    ///3: Trigger detection on both the rising and falling edges
    BothEdges = 3,
}
impl From<EXTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTEN_A) -> Self {
        variant as _
    }
}
impl EXTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EXTEN_A {
        match self.bits {
            0 => EXTEN_A::Disabled,
            1 => EXTEN_A::RisingEdge,
            2 => EXTEN_A::FallingEdge,
            3 => EXTEN_A::BothEdges,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTEN_A::Disabled
    }
    ///Checks if the value of the field is `RisingEdge`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTEN_A::RisingEdge
    }
    ///Checks if the value of the field is `FallingEdge`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTEN_A::FallingEdge
    }
    ///Checks if the value of the field is `BothEdges`
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == EXTEN_A::BothEdges
    }
}
///Field `EXTEN` writer - External trigger enable for regular channels
pub type EXTEN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR2_SPEC, u8, EXTEN_A, 2, O>;
impl<'a, const O: u8> EXTEN_W<'a, O> {
    ///Trigger detection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTEN_A::Disabled)
    }
    ///Trigger detection on the rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EXTEN_A::RisingEdge)
    }
    ///Trigger detection on the falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EXTEN_A::FallingEdge)
    }
    ///Trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(EXTEN_A::BothEdges)
    }
}
///Field `SWSTART` reader - Start conversion of regular channels
pub type SWSTART_R = crate::BitReader<SWSTARTW_A>;
///Start conversion of regular channels
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWSTARTW_A {
    ///1: Starts conversion of regular channels
    Start = 1,
}
impl From<SWSTARTW_A> for bool {
    #[inline(always)]
    fn from(variant: SWSTARTW_A) -> Self {
        variant as u8 != 0
    }
}
impl SWSTART_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SWSTARTW_A> {
        match self.bits {
            true => Some(SWSTARTW_A::Start),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Start`
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SWSTARTW_A::Start
    }
}
///Field `SWSTART` writer - Start conversion of regular channels
pub type SWSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, SWSTARTW_A, O>;
impl<'a, const O: u8> SWSTART_W<'a, O> {
    ///Starts conversion of regular channels
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(SWSTARTW_A::Start)
    }
}
impl R {
    ///Bit 0 - A/D Converter ON / OFF
    #[inline(always)]
    pub fn adon(&self) -> ADON_R {
        ADON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Continuous conversion
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Direct memory access mode (for single ADC mode)
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DMA disable selection (for single ADC mode)
    #[inline(always)]
    pub fn dds(&self) -> DDS_R {
        DDS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - End of conversion selection
    #[inline(always)]
    pub fn eocs(&self) -> EOCS_R {
        EOCS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Data alignment
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 16:19 - External event select for injected group
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:21 - External trigger enable for injected channels
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - Start conversion of injected channels
    #[inline(always)]
    pub fn jswstart(&self) -> JSWSTART_R {
        JSWSTART_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 24:27 - External event select for regular group
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:29 - External trigger enable for regular channels
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bit 30 - Start conversion of regular channels
    #[inline(always)]
    pub fn swstart(&self) -> SWSTART_R {
        SWSTART_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - A/D Converter ON / OFF
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
    ///Bit 8 - Direct memory access mode (for single ADC mode)
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<8> {
        DMA_W::new(self)
    }
    ///Bit 9 - DMA disable selection (for single ADC mode)
    #[inline(always)]
    #[must_use]
    pub fn dds(&mut self) -> DDS_W<9> {
        DDS_W::new(self)
    }
    ///Bit 10 - End of conversion selection
    #[inline(always)]
    #[must_use]
    pub fn eocs(&mut self) -> EOCS_W<10> {
        EOCS_W::new(self)
    }
    ///Bit 11 - Data alignment
    #[inline(always)]
    #[must_use]
    pub fn align(&mut self) -> ALIGN_W<11> {
        ALIGN_W::new(self)
    }
    ///Bits 16:19 - External event select for injected group
    #[inline(always)]
    #[must_use]
    pub fn jextsel(&mut self) -> JEXTSEL_W<16> {
        JEXTSEL_W::new(self)
    }
    ///Bits 20:21 - External trigger enable for injected channels
    #[inline(always)]
    #[must_use]
    pub fn jexten(&mut self) -> JEXTEN_W<20> {
        JEXTEN_W::new(self)
    }
    ///Bit 22 - Start conversion of injected channels
    #[inline(always)]
    #[must_use]
    pub fn jswstart(&mut self) -> JSWSTART_W<22> {
        JSWSTART_W::new(self)
    }
    ///Bits 24:27 - External event select for regular group
    #[inline(always)]
    #[must_use]
    pub fn extsel(&mut self) -> EXTSEL_W<24> {
        EXTSEL_W::new(self)
    }
    ///Bits 28:29 - External trigger enable for regular channels
    #[inline(always)]
    #[must_use]
    pub fn exten(&mut self) -> EXTEN_W<28> {
        EXTEN_W::new(self)
    }
    ///Bit 30 - Start conversion of regular channels
    #[inline(always)]
    #[must_use]
    pub fn swstart(&mut self) -> SWSTART_W<30> {
        SWSTART_W::new(self)
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
