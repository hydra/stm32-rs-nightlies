///Register `JSQR` reader
pub struct R(crate::R<JSQR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JSQR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JSQR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JSQR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `JSQR` writer
pub struct W(crate::W<JSQR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JSQR_SPEC>;
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
impl From<crate::W<JSQR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JSQR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `JL` reader - ADC group injected sequencer scan length
pub type JL_R = crate::FieldReader<u8, u8>;
///Field `JL` writer - ADC group injected sequencer scan length
pub type JL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, JSQR_SPEC, u8, u8, 2, O>;
///Field `JEXTSEL` reader - ADC group injected external trigger source
pub type JEXTSEL_R = crate::FieldReader<u8, JEXTSEL_A>;
///ADC group injected external trigger source
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JEXTSEL_A {
    ///0: Timer 1 TRGO event
    Tim1Trgo = 0,
    ///1: Timer 1 CC4 event
    Tim1Cc4 = 1,
    ///2: Timer 2 TRGO event
    Tim2Trgo = 2,
    ///3: Timer 2 CC1 event
    Tim2Cc1 = 3,
    ///4: Timer 3 CC4 event
    Tim3Cc4 = 4,
    ///5: Timer 4 TRGO event
    Tim4Trgo = 5,
    ///6: EXTI line 15
    Exti15 = 6,
    ///7: Timer 8 CC4 event
    Tim8Cc4 = 7,
    ///8: Timer 1 TRGO2 event
    Tim1Trgo2 = 8,
    ///9: Timer 8 TRGO event
    Tim8Trgo = 9,
    ///10: Timer 8 TRGO2 event
    Tim8Trgo2 = 10,
    ///11: Timer 3 CC3 event
    Tim3Cc3 = 11,
    ///12: Timer 3 TRGO event
    Tim3Trgo = 12,
    ///13: Timer 3 CC1 event
    Tim3Cc1 = 13,
    ///14: Timer 6 TRGO event
    Tim6Trgo = 14,
    ///15: Timer 15 TRGO event
    Tim15Trgo = 15,
    ///16: HRTIM1_ADCTRG2 event
    Hrtim1Adctrg2 = 16,
    ///17: HRTIM1_ADCTRG4 event
    Hrtim1Adctrg4 = 17,
    ///18: LPTIM1_OUT event
    Lptim1Out = 18,
    ///19: LPTIM2_OUT event
    Lptim2Out = 19,
    ///20: LPTIM3_OUT event
    Lptim3Out = 20,
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
    pub fn variant(&self) -> Option<JEXTSEL_A> {
        match self.bits {
            0 => Some(JEXTSEL_A::Tim1Trgo),
            1 => Some(JEXTSEL_A::Tim1Cc4),
            2 => Some(JEXTSEL_A::Tim2Trgo),
            3 => Some(JEXTSEL_A::Tim2Cc1),
            4 => Some(JEXTSEL_A::Tim3Cc4),
            5 => Some(JEXTSEL_A::Tim4Trgo),
            6 => Some(JEXTSEL_A::Exti15),
            7 => Some(JEXTSEL_A::Tim8Cc4),
            8 => Some(JEXTSEL_A::Tim1Trgo2),
            9 => Some(JEXTSEL_A::Tim8Trgo),
            10 => Some(JEXTSEL_A::Tim8Trgo2),
            11 => Some(JEXTSEL_A::Tim3Cc3),
            12 => Some(JEXTSEL_A::Tim3Trgo),
            13 => Some(JEXTSEL_A::Tim3Cc1),
            14 => Some(JEXTSEL_A::Tim6Trgo),
            15 => Some(JEXTSEL_A::Tim15Trgo),
            16 => Some(JEXTSEL_A::Hrtim1Adctrg2),
            17 => Some(JEXTSEL_A::Hrtim1Adctrg4),
            18 => Some(JEXTSEL_A::Lptim1Out),
            19 => Some(JEXTSEL_A::Lptim2Out),
            20 => Some(JEXTSEL_A::Lptim3Out),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Tim1Trgo`
    #[inline(always)]
    pub fn is_tim1_trgo(&self) -> bool {
        *self == JEXTSEL_A::Tim1Trgo
    }
    ///Checks if the value of the field is `Tim1Cc4`
    #[inline(always)]
    pub fn is_tim1_cc4(&self) -> bool {
        *self == JEXTSEL_A::Tim1Cc4
    }
    ///Checks if the value of the field is `Tim2Trgo`
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == JEXTSEL_A::Tim2Trgo
    }
    ///Checks if the value of the field is `Tim2Cc1`
    #[inline(always)]
    pub fn is_tim2_cc1(&self) -> bool {
        *self == JEXTSEL_A::Tim2Cc1
    }
    ///Checks if the value of the field is `Tim3Cc4`
    #[inline(always)]
    pub fn is_tim3_cc4(&self) -> bool {
        *self == JEXTSEL_A::Tim3Cc4
    }
    ///Checks if the value of the field is `Tim4Trgo`
    #[inline(always)]
    pub fn is_tim4_trgo(&self) -> bool {
        *self == JEXTSEL_A::Tim4Trgo
    }
    ///Checks if the value of the field is `Exti15`
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        *self == JEXTSEL_A::Exti15
    }
    ///Checks if the value of the field is `Tim8Cc4`
    #[inline(always)]
    pub fn is_tim8_cc4(&self) -> bool {
        *self == JEXTSEL_A::Tim8Cc4
    }
    ///Checks if the value of the field is `Tim1Trgo2`
    #[inline(always)]
    pub fn is_tim1_trgo2(&self) -> bool {
        *self == JEXTSEL_A::Tim1Trgo2
    }
    ///Checks if the value of the field is `Tim8Trgo`
    #[inline(always)]
    pub fn is_tim8_trgo(&self) -> bool {
        *self == JEXTSEL_A::Tim8Trgo
    }
    ///Checks if the value of the field is `Tim8Trgo2`
    #[inline(always)]
    pub fn is_tim8_trgo2(&self) -> bool {
        *self == JEXTSEL_A::Tim8Trgo2
    }
    ///Checks if the value of the field is `Tim3Cc3`
    #[inline(always)]
    pub fn is_tim3_cc3(&self) -> bool {
        *self == JEXTSEL_A::Tim3Cc3
    }
    ///Checks if the value of the field is `Tim3Trgo`
    #[inline(always)]
    pub fn is_tim3_trgo(&self) -> bool {
        *self == JEXTSEL_A::Tim3Trgo
    }
    ///Checks if the value of the field is `Tim3Cc1`
    #[inline(always)]
    pub fn is_tim3_cc1(&self) -> bool {
        *self == JEXTSEL_A::Tim3Cc1
    }
    ///Checks if the value of the field is `Tim6Trgo`
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        *self == JEXTSEL_A::Tim6Trgo
    }
    ///Checks if the value of the field is `Tim15Trgo`
    #[inline(always)]
    pub fn is_tim15_trgo(&self) -> bool {
        *self == JEXTSEL_A::Tim15Trgo
    }
    ///Checks if the value of the field is `Hrtim1Adctrg2`
    #[inline(always)]
    pub fn is_hrtim1_adctrg2(&self) -> bool {
        *self == JEXTSEL_A::Hrtim1Adctrg2
    }
    ///Checks if the value of the field is `Hrtim1Adctrg4`
    #[inline(always)]
    pub fn is_hrtim1_adctrg4(&self) -> bool {
        *self == JEXTSEL_A::Hrtim1Adctrg4
    }
    ///Checks if the value of the field is `Lptim1Out`
    #[inline(always)]
    pub fn is_lptim1_out(&self) -> bool {
        *self == JEXTSEL_A::Lptim1Out
    }
    ///Checks if the value of the field is `Lptim2Out`
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        *self == JEXTSEL_A::Lptim2Out
    }
    ///Checks if the value of the field is `Lptim3Out`
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        *self == JEXTSEL_A::Lptim3Out
    }
}
///Field `JEXTSEL` writer - ADC group injected external trigger source
pub type JEXTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JSQR_SPEC, u8, JEXTSEL_A, 5, O>;
impl<'a, const O: u8> JEXTSEL_W<'a, O> {
    ///Timer 1 TRGO event
    #[inline(always)]
    pub fn tim1_trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim1Trgo)
    }
    ///Timer 1 CC4 event
    #[inline(always)]
    pub fn tim1_cc4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim1Cc4)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim2Trgo)
    }
    ///Timer 2 CC1 event
    #[inline(always)]
    pub fn tim2_cc1(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim2Cc1)
    }
    ///Timer 3 CC4 event
    #[inline(always)]
    pub fn tim3_cc4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim3Cc4)
    }
    ///Timer 4 TRGO event
    #[inline(always)]
    pub fn tim4_trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim4Trgo)
    }
    ///EXTI line 15
    #[inline(always)]
    pub fn exti15(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Exti15)
    }
    ///Timer 8 CC4 event
    #[inline(always)]
    pub fn tim8_cc4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim8Cc4)
    }
    ///Timer 1 TRGO2 event
    #[inline(always)]
    pub fn tim1_trgo2(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim1Trgo2)
    }
    ///Timer 8 TRGO event
    #[inline(always)]
    pub fn tim8_trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim8Trgo)
    }
    ///Timer 8 TRGO2 event
    #[inline(always)]
    pub fn tim8_trgo2(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim8Trgo2)
    }
    ///Timer 3 CC3 event
    #[inline(always)]
    pub fn tim3_cc3(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim3Cc3)
    }
    ///Timer 3 TRGO event
    #[inline(always)]
    pub fn tim3_trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim3Trgo)
    }
    ///Timer 3 CC1 event
    #[inline(always)]
    pub fn tim3_cc1(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim3Cc1)
    }
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim6Trgo)
    }
    ///Timer 15 TRGO event
    #[inline(always)]
    pub fn tim15_trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Tim15Trgo)
    }
    ///HRTIM1_ADCTRG2 event
    #[inline(always)]
    pub fn hrtim1_adctrg2(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Hrtim1Adctrg2)
    }
    ///HRTIM1_ADCTRG4 event
    #[inline(always)]
    pub fn hrtim1_adctrg4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Hrtim1Adctrg4)
    }
    ///LPTIM1_OUT event
    #[inline(always)]
    pub fn lptim1_out(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Lptim1Out)
    }
    ///LPTIM2_OUT event
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Lptim2Out)
    }
    ///LPTIM3_OUT event
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut W {
        self.variant(JEXTSEL_A::Lptim3Out)
    }
}
///Field `JEXTEN` reader - ADC group injected external trigger polarity
pub type JEXTEN_R = crate::FieldReader<u8, JEXTEN_A>;
///ADC group injected external trigger polarity
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
///Field `JEXTEN` writer - ADC group injected external trigger polarity
pub type JEXTEN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, JSQR_SPEC, u8, JEXTEN_A, 2, O>;
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
///Field `JSQ1` reader - ADC group injected sequencer rank 1
pub type JSQ1_R = crate::FieldReader<u8, u8>;
///Field `JSQ1` writer - ADC group injected sequencer rank 1
pub type JSQ1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JSQR_SPEC, u8, u8, 5, O>;
///Field `JSQ2` reader - ADC group injected sequencer rank 2
pub type JSQ2_R = crate::FieldReader<u8, u8>;
///Field `JSQ2` writer - ADC group injected sequencer rank 2
pub type JSQ2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JSQR_SPEC, u8, u8, 5, O>;
///Field `JSQ3` reader - ADC group injected sequencer rank 3
pub type JSQ3_R = crate::FieldReader<u8, u8>;
///Field `JSQ3` writer - ADC group injected sequencer rank 3
pub type JSQ3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JSQR_SPEC, u8, u8, 5, O>;
///Field `JSQ4` reader - ADC group injected sequencer rank 4
pub type JSQ4_R = crate::FieldReader<u8, u8>;
///Field `JSQ4` writer - ADC group injected sequencer rank 4
pub type JSQ4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JSQR_SPEC, u8, u8, 5, O>;
impl R {
    ///Bits 0:1 - ADC group injected sequencer scan length
    #[inline(always)]
    pub fn jl(&self) -> JL_R {
        JL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:6 - ADC group injected external trigger source
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    ///Bits 7:8 - ADC group injected external trigger polarity
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bits 9:13 - ADC group injected sequencer rank 1
    #[inline(always)]
    pub fn jsq1(&self) -> JSQ1_R {
        JSQ1_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    ///Bits 15:19 - ADC group injected sequencer rank 2
    #[inline(always)]
    pub fn jsq2(&self) -> JSQ2_R {
        JSQ2_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bits 21:25 - ADC group injected sequencer rank 3
    #[inline(always)]
    pub fn jsq3(&self) -> JSQ3_R {
        JSQ3_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    ///Bits 27:31 - ADC group injected sequencer rank 4
    #[inline(always)]
    pub fn jsq4(&self) -> JSQ4_R {
        JSQ4_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:1 - ADC group injected sequencer scan length
    #[inline(always)]
    #[must_use]
    pub fn jl(&mut self) -> JL_W<0> {
        JL_W::new(self)
    }
    ///Bits 2:6 - ADC group injected external trigger source
    #[inline(always)]
    #[must_use]
    pub fn jextsel(&mut self) -> JEXTSEL_W<2> {
        JEXTSEL_W::new(self)
    }
    ///Bits 7:8 - ADC group injected external trigger polarity
    #[inline(always)]
    #[must_use]
    pub fn jexten(&mut self) -> JEXTEN_W<7> {
        JEXTEN_W::new(self)
    }
    ///Bits 9:13 - ADC group injected sequencer rank 1
    #[inline(always)]
    #[must_use]
    pub fn jsq1(&mut self) -> JSQ1_W<9> {
        JSQ1_W::new(self)
    }
    ///Bits 15:19 - ADC group injected sequencer rank 2
    #[inline(always)]
    #[must_use]
    pub fn jsq2(&mut self) -> JSQ2_W<15> {
        JSQ2_W::new(self)
    }
    ///Bits 21:25 - ADC group injected sequencer rank 3
    #[inline(always)]
    #[must_use]
    pub fn jsq3(&mut self) -> JSQ3_W<21> {
        JSQ3_W::new(self)
    }
    ///Bits 27:31 - ADC group injected sequencer rank 4
    #[inline(always)]
    #[must_use]
    pub fn jsq4(&mut self) -> JSQ4_W<27> {
        JSQ4_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC group injected sequencer register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [jsqr](index.html) module
pub struct JSQR_SPEC;
impl crate::RegisterSpec for JSQR_SPEC {
    type Ux = u32;
}
///`read()` method returns [jsqr::R](R) reader structure
impl crate::Readable for JSQR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [jsqr::W](W) writer structure
impl crate::Writable for JSQR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets JSQR to value 0
impl crate::Resettable for JSQR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
