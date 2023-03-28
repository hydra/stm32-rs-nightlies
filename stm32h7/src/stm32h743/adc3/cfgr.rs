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
///Field `DMNGT` reader - ADC DMA transfer enable
pub type DMNGT_R = crate::FieldReader<u8, DMNGT_A>;
///ADC DMA transfer enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMNGT_A {
    ///0: Store output data in DR only
    Dr = 0,
    ///1: DMA One Shot Mode selected
    DmaOneShot = 1,
    ///2: DFSDM mode selected
    Dfsdm = 2,
    ///3: DMA Circular Mode selected
    DmaCircular = 3,
}
impl From<DMNGT_A> for u8 {
    #[inline(always)]
    fn from(variant: DMNGT_A) -> Self {
        variant as _
    }
}
impl DMNGT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMNGT_A {
        match self.bits {
            0 => DMNGT_A::Dr,
            1 => DMNGT_A::DmaOneShot,
            2 => DMNGT_A::Dfsdm,
            3 => DMNGT_A::DmaCircular,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Dr`
    #[inline(always)]
    pub fn is_dr(&self) -> bool {
        *self == DMNGT_A::Dr
    }
    ///Checks if the value of the field is `DmaOneShot`
    #[inline(always)]
    pub fn is_dma_one_shot(&self) -> bool {
        *self == DMNGT_A::DmaOneShot
    }
    ///Checks if the value of the field is `Dfsdm`
    #[inline(always)]
    pub fn is_dfsdm(&self) -> bool {
        *self == DMNGT_A::Dfsdm
    }
    ///Checks if the value of the field is `DmaCircular`
    #[inline(always)]
    pub fn is_dma_circular(&self) -> bool {
        *self == DMNGT_A::DmaCircular
    }
}
///Field `DMNGT` writer - ADC DMA transfer enable
pub type DMNGT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, DMNGT_A, 2, O>;
impl<'a, const O: u8> DMNGT_W<'a, O> {
    ///Store output data in DR only
    #[inline(always)]
    pub fn dr(self) -> &'a mut W {
        self.variant(DMNGT_A::Dr)
    }
    ///DMA One Shot Mode selected
    #[inline(always)]
    pub fn dma_one_shot(self) -> &'a mut W {
        self.variant(DMNGT_A::DmaOneShot)
    }
    ///DFSDM mode selected
    #[inline(always)]
    pub fn dfsdm(self) -> &'a mut W {
        self.variant(DMNGT_A::Dfsdm)
    }
    ///DMA Circular Mode selected
    #[inline(always)]
    pub fn dma_circular(self) -> &'a mut W {
        self.variant(DMNGT_A::DmaCircular)
    }
}
///Field `RES` reader - ADC data resolution
pub type RES_R = crate::FieldReader<u8, RES_A>;
///ADC data resolution
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RES_A {
    ///0: 16-bit resolution
    SixteenBit = 0,
    ///1: 14-bit resolution
    FourteenBit = 1,
    ///2: 12-bit resolution
    TwelveBit = 2,
    ///3: 10-bit resolution
    TenBit = 3,
    ///4: 8-bit resolution
    EightBit = 4,
}
impl From<RES_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_A) -> Self {
        variant as _
    }
}
impl RES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<RES_A> {
        match self.bits {
            0 => Some(RES_A::SixteenBit),
            1 => Some(RES_A::FourteenBit),
            2 => Some(RES_A::TwelveBit),
            3 => Some(RES_A::TenBit),
            4 => Some(RES_A::EightBit),
            _ => None,
        }
    }
    ///Checks if the value of the field is `SixteenBit`
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == RES_A::SixteenBit
    }
    ///Checks if the value of the field is `FourteenBit`
    #[inline(always)]
    pub fn is_fourteen_bit(&self) -> bool {
        *self == RES_A::FourteenBit
    }
    ///Checks if the value of the field is `TwelveBit`
    #[inline(always)]
    pub fn is_twelve_bit(&self) -> bool {
        *self == RES_A::TwelveBit
    }
    ///Checks if the value of the field is `TenBit`
    #[inline(always)]
    pub fn is_ten_bit(&self) -> bool {
        *self == RES_A::TenBit
    }
    ///Checks if the value of the field is `EightBit`
    #[inline(always)]
    pub fn is_eight_bit(&self) -> bool {
        *self == RES_A::EightBit
    }
}
///Field `RES` writer - ADC data resolution
pub type RES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, RES_A, 3, O>;
impl<'a, const O: u8> RES_W<'a, O> {
    ///16-bit resolution
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut W {
        self.variant(RES_A::SixteenBit)
    }
    ///14-bit resolution
    #[inline(always)]
    pub fn fourteen_bit(self) -> &'a mut W {
        self.variant(RES_A::FourteenBit)
    }
    ///12-bit resolution
    #[inline(always)]
    pub fn twelve_bit(self) -> &'a mut W {
        self.variant(RES_A::TwelveBit)
    }
    ///10-bit resolution
    #[inline(always)]
    pub fn ten_bit(self) -> &'a mut W {
        self.variant(RES_A::TenBit)
    }
    ///8-bit resolution
    #[inline(always)]
    pub fn eight_bit(self) -> &'a mut W {
        self.variant(RES_A::EightBit)
    }
}
///Field `EXTSEL` reader - ADC group regular external trigger source
pub type EXTSEL_R = crate::FieldReader<u8, EXTSEL_A>;
///ADC group regular external trigger source
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTSEL_A {
    ///0: Timer 1 CC1 event
    Tim1Cc1 = 0,
    ///1: Timer 1 CC2 event
    Tim1Cc2 = 1,
    ///2: Timer 1 CC3 event
    Tim1Cc3 = 2,
    ///3: Timer 2 CC2 event
    Tim2Cc2 = 3,
    ///4: Timer 3 TRGO event
    Tim3Trgo = 4,
    ///5: Timer 4 CC4 event
    Tim4Cc4 = 5,
    ///6: EXTI line 11
    Exti11 = 6,
    ///7: Timer 8 TRGO event
    Tim8Trgo = 7,
    ///8: Timer 8 TRGO2 event
    Tim8Trgo2 = 8,
    ///9: Timer 1 TRGO event
    Tim1Trgo = 9,
    ///10: Timer 1 TRGO2 event
    Tim1Trgo2 = 10,
    ///11: Timer 2 TRGO event
    Tim2Trgo = 11,
    ///12: Timer 4 TRGO event
    Tim4Trgo = 12,
    ///13: Timer 6 TRGO event
    Tim6Trgo = 13,
    ///14: Timer 15 TRGO event
    Tim15Trgo = 14,
    ///15: Timer 3 CC4 event
    Tim3Cc4 = 15,
    ///16: HRTIM1_ADCTRG1 event
    Hrtim1Adctrg1 = 16,
    ///17: HRTIM1_ADCTRG3 event
    Hrtim1Adctrg3 = 17,
    ///18: LPTIM1_OUT event
    Lptim1Out = 18,
    ///19: LPTIM2_OUT event
    Lptim2Out = 19,
    ///20: LPTIM3_OUT event
    Lptim3Out = 20,
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
    pub fn variant(&self) -> Option<EXTSEL_A> {
        match self.bits {
            0 => Some(EXTSEL_A::Tim1Cc1),
            1 => Some(EXTSEL_A::Tim1Cc2),
            2 => Some(EXTSEL_A::Tim1Cc3),
            3 => Some(EXTSEL_A::Tim2Cc2),
            4 => Some(EXTSEL_A::Tim3Trgo),
            5 => Some(EXTSEL_A::Tim4Cc4),
            6 => Some(EXTSEL_A::Exti11),
            7 => Some(EXTSEL_A::Tim8Trgo),
            8 => Some(EXTSEL_A::Tim8Trgo2),
            9 => Some(EXTSEL_A::Tim1Trgo),
            10 => Some(EXTSEL_A::Tim1Trgo2),
            11 => Some(EXTSEL_A::Tim2Trgo),
            12 => Some(EXTSEL_A::Tim4Trgo),
            13 => Some(EXTSEL_A::Tim6Trgo),
            14 => Some(EXTSEL_A::Tim15Trgo),
            15 => Some(EXTSEL_A::Tim3Cc4),
            16 => Some(EXTSEL_A::Hrtim1Adctrg1),
            17 => Some(EXTSEL_A::Hrtim1Adctrg3),
            18 => Some(EXTSEL_A::Lptim1Out),
            19 => Some(EXTSEL_A::Lptim2Out),
            20 => Some(EXTSEL_A::Lptim3Out),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Tim1Cc1`
    #[inline(always)]
    pub fn is_tim1_cc1(&self) -> bool {
        *self == EXTSEL_A::Tim1Cc1
    }
    ///Checks if the value of the field is `Tim1Cc2`
    #[inline(always)]
    pub fn is_tim1_cc2(&self) -> bool {
        *self == EXTSEL_A::Tim1Cc2
    }
    ///Checks if the value of the field is `Tim1Cc3`
    #[inline(always)]
    pub fn is_tim1_cc3(&self) -> bool {
        *self == EXTSEL_A::Tim1Cc3
    }
    ///Checks if the value of the field is `Tim2Cc2`
    #[inline(always)]
    pub fn is_tim2_cc2(&self) -> bool {
        *self == EXTSEL_A::Tim2Cc2
    }
    ///Checks if the value of the field is `Tim3Trgo`
    #[inline(always)]
    pub fn is_tim3_trgo(&self) -> bool {
        *self == EXTSEL_A::Tim3Trgo
    }
    ///Checks if the value of the field is `Tim4Cc4`
    #[inline(always)]
    pub fn is_tim4_cc4(&self) -> bool {
        *self == EXTSEL_A::Tim4Cc4
    }
    ///Checks if the value of the field is `Exti11`
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        *self == EXTSEL_A::Exti11
    }
    ///Checks if the value of the field is `Tim8Trgo`
    #[inline(always)]
    pub fn is_tim8_trgo(&self) -> bool {
        *self == EXTSEL_A::Tim8Trgo
    }
    ///Checks if the value of the field is `Tim8Trgo2`
    #[inline(always)]
    pub fn is_tim8_trgo2(&self) -> bool {
        *self == EXTSEL_A::Tim8Trgo2
    }
    ///Checks if the value of the field is `Tim1Trgo`
    #[inline(always)]
    pub fn is_tim1_trgo(&self) -> bool {
        *self == EXTSEL_A::Tim1Trgo
    }
    ///Checks if the value of the field is `Tim1Trgo2`
    #[inline(always)]
    pub fn is_tim1_trgo2(&self) -> bool {
        *self == EXTSEL_A::Tim1Trgo2
    }
    ///Checks if the value of the field is `Tim2Trgo`
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == EXTSEL_A::Tim2Trgo
    }
    ///Checks if the value of the field is `Tim4Trgo`
    #[inline(always)]
    pub fn is_tim4_trgo(&self) -> bool {
        *self == EXTSEL_A::Tim4Trgo
    }
    ///Checks if the value of the field is `Tim6Trgo`
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        *self == EXTSEL_A::Tim6Trgo
    }
    ///Checks if the value of the field is `Tim15Trgo`
    #[inline(always)]
    pub fn is_tim15_trgo(&self) -> bool {
        *self == EXTSEL_A::Tim15Trgo
    }
    ///Checks if the value of the field is `Tim3Cc4`
    #[inline(always)]
    pub fn is_tim3_cc4(&self) -> bool {
        *self == EXTSEL_A::Tim3Cc4
    }
    ///Checks if the value of the field is `Hrtim1Adctrg1`
    #[inline(always)]
    pub fn is_hrtim1_adctrg1(&self) -> bool {
        *self == EXTSEL_A::Hrtim1Adctrg1
    }
    ///Checks if the value of the field is `Hrtim1Adctrg3`
    #[inline(always)]
    pub fn is_hrtim1_adctrg3(&self) -> bool {
        *self == EXTSEL_A::Hrtim1Adctrg3
    }
    ///Checks if the value of the field is `Lptim1Out`
    #[inline(always)]
    pub fn is_lptim1_out(&self) -> bool {
        *self == EXTSEL_A::Lptim1Out
    }
    ///Checks if the value of the field is `Lptim2Out`
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        *self == EXTSEL_A::Lptim2Out
    }
    ///Checks if the value of the field is `Lptim3Out`
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        *self == EXTSEL_A::Lptim3Out
    }
}
///Field `EXTSEL` writer - ADC group regular external trigger source
pub type EXTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, EXTSEL_A, 5, O>;
impl<'a, const O: u8> EXTSEL_W<'a, O> {
    ///Timer 1 CC1 event
    #[inline(always)]
    pub fn tim1_cc1(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim1Cc1)
    }
    ///Timer 1 CC2 event
    #[inline(always)]
    pub fn tim1_cc2(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim1Cc2)
    }
    ///Timer 1 CC3 event
    #[inline(always)]
    pub fn tim1_cc3(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim1Cc3)
    }
    ///Timer 2 CC2 event
    #[inline(always)]
    pub fn tim2_cc2(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim2Cc2)
    }
    ///Timer 3 TRGO event
    #[inline(always)]
    pub fn tim3_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim3Trgo)
    }
    ///Timer 4 CC4 event
    #[inline(always)]
    pub fn tim4_cc4(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim4Cc4)
    }
    ///EXTI line 11
    #[inline(always)]
    pub fn exti11(self) -> &'a mut W {
        self.variant(EXTSEL_A::Exti11)
    }
    ///Timer 8 TRGO event
    #[inline(always)]
    pub fn tim8_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim8Trgo)
    }
    ///Timer 8 TRGO2 event
    #[inline(always)]
    pub fn tim8_trgo2(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim8Trgo2)
    }
    ///Timer 1 TRGO event
    #[inline(always)]
    pub fn tim1_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim1Trgo)
    }
    ///Timer 1 TRGO2 event
    #[inline(always)]
    pub fn tim1_trgo2(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim1Trgo2)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim2Trgo)
    }
    ///Timer 4 TRGO event
    #[inline(always)]
    pub fn tim4_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim4Trgo)
    }
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim6Trgo)
    }
    ///Timer 15 TRGO event
    #[inline(always)]
    pub fn tim15_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim15Trgo)
    }
    ///Timer 3 CC4 event
    #[inline(always)]
    pub fn tim3_cc4(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim3Cc4)
    }
    ///HRTIM1_ADCTRG1 event
    #[inline(always)]
    pub fn hrtim1_adctrg1(self) -> &'a mut W {
        self.variant(EXTSEL_A::Hrtim1Adctrg1)
    }
    ///HRTIM1_ADCTRG3 event
    #[inline(always)]
    pub fn hrtim1_adctrg3(self) -> &'a mut W {
        self.variant(EXTSEL_A::Hrtim1Adctrg3)
    }
    ///LPTIM1_OUT event
    #[inline(always)]
    pub fn lptim1_out(self) -> &'a mut W {
        self.variant(EXTSEL_A::Lptim1Out)
    }
    ///LPTIM2_OUT event
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut W {
        self.variant(EXTSEL_A::Lptim2Out)
    }
    ///LPTIM3_OUT event
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut W {
        self.variant(EXTSEL_A::Lptim3Out)
    }
}
///Field `EXTEN` reader - ADC group regular external trigger polarity
pub type EXTEN_R = crate::FieldReader<u8, EXTEN_A>;
///ADC group regular external trigger polarity
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
///Field `EXTEN` writer - ADC group regular external trigger polarity
pub type EXTEN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, EXTEN_A, 2, O>;
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
///Field `OVRMOD` reader - ADC group regular overrun configuration
pub type OVRMOD_R = crate::BitReader<OVRMOD_A>;
///ADC group regular overrun configuration
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRMOD_A {
    ///0: Preserve DR register when an overrun is detected
    Preserve = 0,
    ///1: Overwrite DR register when an overrun is detected
    Overwrite = 1,
}
impl From<OVRMOD_A> for bool {
    #[inline(always)]
    fn from(variant: OVRMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVRMOD_A {
        match self.bits {
            false => OVRMOD_A::Preserve,
            true => OVRMOD_A::Overwrite,
        }
    }
    ///Checks if the value of the field is `Preserve`
    #[inline(always)]
    pub fn is_preserve(&self) -> bool {
        *self == OVRMOD_A::Preserve
    }
    ///Checks if the value of the field is `Overwrite`
    #[inline(always)]
    pub fn is_overwrite(&self) -> bool {
        *self == OVRMOD_A::Overwrite
    }
}
///Field `OVRMOD` writer - ADC group regular overrun configuration
pub type OVRMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, OVRMOD_A, O>;
impl<'a, const O: u8> OVRMOD_W<'a, O> {
    ///Preserve DR register when an overrun is detected
    #[inline(always)]
    pub fn preserve(self) -> &'a mut W {
        self.variant(OVRMOD_A::Preserve)
    }
    ///Overwrite DR register when an overrun is detected
    #[inline(always)]
    pub fn overwrite(self) -> &'a mut W {
        self.variant(OVRMOD_A::Overwrite)
    }
}
///Field `CONT` reader - ADC group regular continuous conversion mode
pub type CONT_R = crate::BitReader<CONT_A>;
///ADC group regular continuous conversion mode
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
///Field `CONT` writer - ADC group regular continuous conversion mode
pub type CONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, CONT_A, O>;
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
///Field `AUTDLY` reader - ADC low power auto wait
pub type AUTDLY_R = crate::BitReader<AUTDLY_A>;
///ADC low power auto wait
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTDLY_A {
    ///0: Auto delayed conversion mode off
    Off = 0,
    ///1: Auto delayed conversion mode on
    On = 1,
}
impl From<AUTDLY_A> for bool {
    #[inline(always)]
    fn from(variant: AUTDLY_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTDLY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AUTDLY_A {
        match self.bits {
            false => AUTDLY_A::Off,
            true => AUTDLY_A::On,
        }
    }
    ///Checks if the value of the field is `Off`
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == AUTDLY_A::Off
    }
    ///Checks if the value of the field is `On`
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == AUTDLY_A::On
    }
}
///Field `AUTDLY` writer - ADC low power auto wait
pub type AUTDLY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, AUTDLY_A, O>;
impl<'a, const O: u8> AUTDLY_W<'a, O> {
    ///Auto delayed conversion mode off
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(AUTDLY_A::Off)
    }
    ///Auto delayed conversion mode on
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(AUTDLY_A::On)
    }
}
///Field `DISCEN` reader - ADC group regular sequencer discontinuous mode
pub type DISCEN_R = crate::BitReader<DISCEN_A>;
///ADC group regular sequencer discontinuous mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISCEN_A {
    ///0: Discontinuous mode on regular channels disabled
    Disabled = 0,
    ///1: Discontinuous mode on regular channels enabled
    Enabled = 1,
}
impl From<DISCEN_A> for bool {
    #[inline(always)]
    fn from(variant: DISCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DISCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DISCEN_A {
        match self.bits {
            false => DISCEN_A::Disabled,
            true => DISCEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISCEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISCEN_A::Enabled
    }
}
///Field `DISCEN` writer - ADC group regular sequencer discontinuous mode
pub type DISCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, DISCEN_A, O>;
impl<'a, const O: u8> DISCEN_W<'a, O> {
    ///Discontinuous mode on regular channels disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISCEN_A::Disabled)
    }
    ///Discontinuous mode on regular channels enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISCEN_A::Enabled)
    }
}
///Field `DISCNUM` reader - ADC group regular sequencer discontinuous number of ranks
pub type DISCNUM_R = crate::FieldReader<u8, u8>;
///Field `DISCNUM` writer - ADC group regular sequencer discontinuous number of ranks
pub type DISCNUM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, u8, 3, O>;
///Field `JDISCEN` reader - ADC group injected sequencer discontinuous mode
pub type JDISCEN_R = crate::BitReader<JDISCEN_A>;
///ADC group injected sequencer discontinuous mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JDISCEN_A {
    ///0: Discontinuous mode on injected channels disabled
    Disabled = 0,
    ///1: Discontinuous mode on injected channels enabled
    Enabled = 1,
}
impl From<JDISCEN_A> for bool {
    #[inline(always)]
    fn from(variant: JDISCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl JDISCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JDISCEN_A {
        match self.bits {
            false => JDISCEN_A::Disabled,
            true => JDISCEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JDISCEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JDISCEN_A::Enabled
    }
}
///Field `JDISCEN` writer - ADC group injected sequencer discontinuous mode
pub type JDISCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, JDISCEN_A, O>;
impl<'a, const O: u8> JDISCEN_W<'a, O> {
    ///Discontinuous mode on injected channels disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JDISCEN_A::Disabled)
    }
    ///Discontinuous mode on injected channels enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JDISCEN_A::Enabled)
    }
}
///Field `JQM` reader - ADC group injected contexts queue mode
pub type JQM_R = crate::BitReader<JQM_A>;
///ADC group injected contexts queue mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQM_A {
    ///0: JSQR Mode 0: Queue maintains the last written configuration into JSQR
    Mode0 = 0,
    ///1: JSQR Mode 1: An empty queue disables software and hardware triggers of the injected sequence
    Mode1 = 1,
}
impl From<JQM_A> for bool {
    #[inline(always)]
    fn from(variant: JQM_A) -> Self {
        variant as u8 != 0
    }
}
impl JQM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JQM_A {
        match self.bits {
            false => JQM_A::Mode0,
            true => JQM_A::Mode1,
        }
    }
    ///Checks if the value of the field is `Mode0`
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == JQM_A::Mode0
    }
    ///Checks if the value of the field is `Mode1`
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == JQM_A::Mode1
    }
}
///Field `JQM` writer - ADC group injected contexts queue mode
pub type JQM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, JQM_A, O>;
impl<'a, const O: u8> JQM_W<'a, O> {
    ///JSQR Mode 0: Queue maintains the last written configuration into JSQR
    #[inline(always)]
    pub fn mode0(self) -> &'a mut W {
        self.variant(JQM_A::Mode0)
    }
    ///JSQR Mode 1: An empty queue disables software and hardware triggers of the injected sequence
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(JQM_A::Mode1)
    }
}
///Field `AWD1SGL` reader - ADC analog watchdog 1 monitoring a single channel or all channels
pub type AWD1SGL_R = crate::BitReader<AWD1SGL_A>;
///ADC analog watchdog 1 monitoring a single channel or all channels
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1SGL_A {
    ///0: Analog watchdog 1 enabled on all channels
    All = 0,
    ///1: Analog watchdog 1 enabled on single channel selected in AWD1CH
    Single = 1,
}
impl From<AWD1SGL_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1SGL_A) -> Self {
        variant as u8 != 0
    }
}
impl AWD1SGL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AWD1SGL_A {
        match self.bits {
            false => AWD1SGL_A::All,
            true => AWD1SGL_A::Single,
        }
    }
    ///Checks if the value of the field is `All`
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == AWD1SGL_A::All
    }
    ///Checks if the value of the field is `Single`
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == AWD1SGL_A::Single
    }
}
///Field `AWD1SGL` writer - ADC analog watchdog 1 monitoring a single channel or all channels
pub type AWD1SGL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, AWD1SGL_A, O>;
impl<'a, const O: u8> AWD1SGL_W<'a, O> {
    ///Analog watchdog 1 enabled on all channels
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(AWD1SGL_A::All)
    }
    ///Analog watchdog 1 enabled on single channel selected in AWD1CH
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(AWD1SGL_A::Single)
    }
}
///Field `AWD1EN` reader - ADC analog watchdog 1 enable on scope ADC group regular
pub type AWD1EN_R = crate::BitReader<AWD1EN_A>;
///ADC analog watchdog 1 enable on scope ADC group regular
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1EN_A {
    ///0: Analog watchdog 1 disabled on regular channels
    Disabled = 0,
    ///1: Analog watchdog 1 enabled on regular channels
    Enabled = 1,
}
impl From<AWD1EN_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl AWD1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AWD1EN_A {
        match self.bits {
            false => AWD1EN_A::Disabled,
            true => AWD1EN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWD1EN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWD1EN_A::Enabled
    }
}
///Field `AWD1EN` writer - ADC analog watchdog 1 enable on scope ADC group regular
pub type AWD1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, AWD1EN_A, O>;
impl<'a, const O: u8> AWD1EN_W<'a, O> {
    ///Analog watchdog 1 disabled on regular channels
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AWD1EN_A::Disabled)
    }
    ///Analog watchdog 1 enabled on regular channels
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AWD1EN_A::Enabled)
    }
}
///Field `JAWD1EN` reader - ADC analog watchdog 1 enable on scope ADC group injected
pub type JAWD1EN_R = crate::BitReader<JAWD1EN_A>;
///ADC analog watchdog 1 enable on scope ADC group injected
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JAWD1EN_A {
    ///0: Analog watchdog 1 disabled on injected channels
    Disabled = 0,
    ///1: Analog watchdog 1 enabled on injected channels
    Enabled = 1,
}
impl From<JAWD1EN_A> for bool {
    #[inline(always)]
    fn from(variant: JAWD1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl JAWD1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JAWD1EN_A {
        match self.bits {
            false => JAWD1EN_A::Disabled,
            true => JAWD1EN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JAWD1EN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JAWD1EN_A::Enabled
    }
}
///Field `JAWD1EN` writer - ADC analog watchdog 1 enable on scope ADC group injected
pub type JAWD1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, JAWD1EN_A, O>;
impl<'a, const O: u8> JAWD1EN_W<'a, O> {
    ///Analog watchdog 1 disabled on injected channels
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JAWD1EN_A::Disabled)
    }
    ///Analog watchdog 1 enabled on injected channels
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JAWD1EN_A::Enabled)
    }
}
///Field `JAUTO` reader - ADC group injected automatic trigger mode
pub type JAUTO_R = crate::BitReader<JAUTO_A>;
///ADC group injected automatic trigger mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JAUTO_A {
    ///0: Automatic injected group conversion disabled
    Disabled = 0,
    ///1: Automatic injected group conversion enabled
    Enabled = 1,
}
impl From<JAUTO_A> for bool {
    #[inline(always)]
    fn from(variant: JAUTO_A) -> Self {
        variant as u8 != 0
    }
}
impl JAUTO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JAUTO_A {
        match self.bits {
            false => JAUTO_A::Disabled,
            true => JAUTO_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JAUTO_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JAUTO_A::Enabled
    }
}
///Field `JAUTO` writer - ADC group injected automatic trigger mode
pub type JAUTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, JAUTO_A, O>;
impl<'a, const O: u8> JAUTO_W<'a, O> {
    ///Automatic injected group conversion disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JAUTO_A::Disabled)
    }
    ///Automatic injected group conversion enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JAUTO_A::Enabled)
    }
}
///Field `AWD1CH` reader - ADC analog watchdog 1 monitored channel selection
pub type AWD1CH_R = crate::FieldReader<u8, u8>;
///Field `AWD1CH` writer - ADC analog watchdog 1 monitored channel selection
pub type AWD1CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 5, O>;
///Field `JQDIS` reader - ADC group injected contexts queue disable
pub type JQDIS_R = crate::BitReader<JQDIS_A>;
///ADC group injected contexts queue disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQDIS_A {
    ///0: Injected Queue enabled
    Enabled = 0,
    ///1: Injected Queue disabled
    Disabled = 1,
}
impl From<JQDIS_A> for bool {
    #[inline(always)]
    fn from(variant: JQDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl JQDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JQDIS_A {
        match self.bits {
            false => JQDIS_A::Enabled,
            true => JQDIS_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JQDIS_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JQDIS_A::Disabled
    }
}
///Field `JQDIS` writer - ADC group injected contexts queue disable
pub type JQDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, JQDIS_A, O>;
impl<'a, const O: u8> JQDIS_W<'a, O> {
    ///Injected Queue enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JQDIS_A::Enabled)
    }
    ///Injected Queue disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JQDIS_A::Disabled)
    }
}
impl R {
    ///Bits 0:1 - ADC DMA transfer enable
    #[inline(always)]
    pub fn dmngt(&self) -> DMNGT_R {
        DMNGT_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:4 - ADC data resolution
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bits 5:9 - ADC group regular external trigger source
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:11 - ADC group regular external trigger polarity
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - ADC group regular overrun configuration
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ADC group regular continuous conversion mode
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ADC low power auto wait
    #[inline(always)]
    pub fn autdly(&self) -> AUTDLY_R {
        AUTDLY_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - ADC group regular sequencer discontinuous mode
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:19 - ADC group regular sequencer discontinuous number of ranks
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bit 20 - ADC group injected sequencer discontinuous mode
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ADC group injected contexts queue mode
    #[inline(always)]
    pub fn jqm(&self) -> JQM_R {
        JQM_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ADC analog watchdog 1 monitoring a single channel or all channels
    #[inline(always)]
    pub fn awd1sgl(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ADC analog watchdog 1 enable on scope ADC group regular
    #[inline(always)]
    pub fn awd1en(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - ADC analog watchdog 1 enable on scope ADC group injected
    #[inline(always)]
    pub fn jawd1en(&self) -> JAWD1EN_R {
        JAWD1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - ADC group injected automatic trigger mode
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bits 26:30 - ADC analog watchdog 1 monitored channel selection
    #[inline(always)]
    pub fn awd1ch(&self) -> AWD1CH_R {
        AWD1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - ADC group injected contexts queue disable
    #[inline(always)]
    pub fn jqdis(&self) -> JQDIS_R {
        JQDIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - ADC DMA transfer enable
    #[inline(always)]
    #[must_use]
    pub fn dmngt(&mut self) -> DMNGT_W<0> {
        DMNGT_W::new(self)
    }
    ///Bits 2:4 - ADC data resolution
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<2> {
        RES_W::new(self)
    }
    ///Bits 5:9 - ADC group regular external trigger source
    #[inline(always)]
    #[must_use]
    pub fn extsel(&mut self) -> EXTSEL_W<5> {
        EXTSEL_W::new(self)
    }
    ///Bits 10:11 - ADC group regular external trigger polarity
    #[inline(always)]
    #[must_use]
    pub fn exten(&mut self) -> EXTEN_W<10> {
        EXTEN_W::new(self)
    }
    ///Bit 12 - ADC group regular overrun configuration
    #[inline(always)]
    #[must_use]
    pub fn ovrmod(&mut self) -> OVRMOD_W<12> {
        OVRMOD_W::new(self)
    }
    ///Bit 13 - ADC group regular continuous conversion mode
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<13> {
        CONT_W::new(self)
    }
    ///Bit 14 - ADC low power auto wait
    #[inline(always)]
    #[must_use]
    pub fn autdly(&mut self) -> AUTDLY_W<14> {
        AUTDLY_W::new(self)
    }
    ///Bit 16 - ADC group regular sequencer discontinuous mode
    #[inline(always)]
    #[must_use]
    pub fn discen(&mut self) -> DISCEN_W<16> {
        DISCEN_W::new(self)
    }
    ///Bits 17:19 - ADC group regular sequencer discontinuous number of ranks
    #[inline(always)]
    #[must_use]
    pub fn discnum(&mut self) -> DISCNUM_W<17> {
        DISCNUM_W::new(self)
    }
    ///Bit 20 - ADC group injected sequencer discontinuous mode
    #[inline(always)]
    #[must_use]
    pub fn jdiscen(&mut self) -> JDISCEN_W<20> {
        JDISCEN_W::new(self)
    }
    ///Bit 21 - ADC group injected contexts queue mode
    #[inline(always)]
    #[must_use]
    pub fn jqm(&mut self) -> JQM_W<21> {
        JQM_W::new(self)
    }
    ///Bit 22 - ADC analog watchdog 1 monitoring a single channel or all channels
    #[inline(always)]
    #[must_use]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W<22> {
        AWD1SGL_W::new(self)
    }
    ///Bit 23 - ADC analog watchdog 1 enable on scope ADC group regular
    #[inline(always)]
    #[must_use]
    pub fn awd1en(&mut self) -> AWD1EN_W<23> {
        AWD1EN_W::new(self)
    }
    ///Bit 24 - ADC analog watchdog 1 enable on scope ADC group injected
    #[inline(always)]
    #[must_use]
    pub fn jawd1en(&mut self) -> JAWD1EN_W<24> {
        JAWD1EN_W::new(self)
    }
    ///Bit 25 - ADC group injected automatic trigger mode
    #[inline(always)]
    #[must_use]
    pub fn jauto(&mut self) -> JAUTO_W<25> {
        JAUTO_W::new(self)
    }
    ///Bits 26:30 - ADC analog watchdog 1 monitored channel selection
    #[inline(always)]
    #[must_use]
    pub fn awd1ch(&mut self) -> AWD1CH_W<26> {
        AWD1CH_W::new(self)
    }
    ///Bit 31 - ADC group injected contexts queue disable
    #[inline(always)]
    #[must_use]
    pub fn jqdis(&mut self) -> JQDIS_W<31> {
        JQDIS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC configuration register 1
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
