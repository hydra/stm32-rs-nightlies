///Register `COMP6_CSR` reader
pub struct R(crate::R<COMP6_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP6_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP6_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP6_CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `COMP6_CSR` writer
pub struct W(crate::W<COMP6_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP6_CSR_SPEC>;
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
impl From<crate::W<COMP6_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP6_CSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `COMP6EN` reader - Comparator 6 enable
pub type COMP6EN_R = crate::BitReader<COMP6EN_A>;
///Comparator 6 enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP6EN_A {
    ///0: Comparator disabled
    Disabled = 0,
    ///1: Comparator enabled
    Enabled = 1,
}
impl From<COMP6EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMP6EN_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP6EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> COMP6EN_A {
        match self.bits {
            false => COMP6EN_A::Disabled,
            true => COMP6EN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMP6EN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMP6EN_A::Enabled
    }
}
///Field `COMP6EN` writer - Comparator 6 enable
pub type COMP6EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP6_CSR_SPEC, COMP6EN_A, O>;
impl<'a, const O: u8> COMP6EN_W<'a, O> {
    ///Comparator disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMP6EN_A::Disabled)
    }
    ///Comparator enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMP6EN_A::Enabled)
    }
}
///Field `COMP6MODE` reader - Comparator 6 mode
pub type COMP6MODE_R = crate::FieldReader<u8, u8>;
///Field `COMP6MODE` writer - Comparator 6 mode
pub type COMP6MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP6_CSR_SPEC, u8, u8, 2, O>;
///Field `COMP6INMSEL` reader - Comparator 6 inverting input selection
pub type COMP6INMSEL_R = crate::FieldReader<u8, COMP6INMSEL_A>;
///Comparator 6 inverting input selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP6INMSEL_A {
    ///0: 1/4 of VRefint
    OneQuarterVref = 0,
    ///1: 1/2 of VRefint
    OneHalfVref = 1,
    ///2: 3/4 of VRefint
    ThreeQuarterVref = 2,
    ///3: VRefint
    Vref = 3,
    ///4: PA4 or DAC1_CH1 output if enabled
    Pa4Dac1Ch1 = 4,
    ///5: DAC1_CH2
    Dac1Ch2 = 5,
    ///7: PB15
    Pb15 = 7,
}
impl From<COMP6INMSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP6INMSEL_A) -> Self {
        variant as _
    }
}
impl COMP6INMSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP6INMSEL_A> {
        match self.bits {
            0 => Some(COMP6INMSEL_A::OneQuarterVref),
            1 => Some(COMP6INMSEL_A::OneHalfVref),
            2 => Some(COMP6INMSEL_A::ThreeQuarterVref),
            3 => Some(COMP6INMSEL_A::Vref),
            4 => Some(COMP6INMSEL_A::Pa4Dac1Ch1),
            5 => Some(COMP6INMSEL_A::Dac1Ch2),
            7 => Some(COMP6INMSEL_A::Pb15),
            _ => None,
        }
    }
    ///Checks if the value of the field is `OneQuarterVref`
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == COMP6INMSEL_A::OneQuarterVref
    }
    ///Checks if the value of the field is `OneHalfVref`
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == COMP6INMSEL_A::OneHalfVref
    }
    ///Checks if the value of the field is `ThreeQuarterVref`
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == COMP6INMSEL_A::ThreeQuarterVref
    }
    ///Checks if the value of the field is `Vref`
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == COMP6INMSEL_A::Vref
    }
    ///Checks if the value of the field is `Pa4Dac1Ch1`
    #[inline(always)]
    pub fn is_pa4_dac1_ch1(&self) -> bool {
        *self == COMP6INMSEL_A::Pa4Dac1Ch1
    }
    ///Checks if the value of the field is `Dac1Ch2`
    #[inline(always)]
    pub fn is_dac1_ch2(&self) -> bool {
        *self == COMP6INMSEL_A::Dac1Ch2
    }
    ///Checks if the value of the field is `Pb15`
    #[inline(always)]
    pub fn is_pb15(&self) -> bool {
        *self == COMP6INMSEL_A::Pb15
    }
}
///Field `COMP6INMSEL` writer - Comparator 6 inverting input selection
pub type COMP6INMSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP6_CSR_SPEC, u8, COMP6INMSEL_A, 3, O>;
impl<'a, const O: u8> COMP6INMSEL_W<'a, O> {
    ///1/4 of VRefint
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::OneQuarterVref)
    }
    ///1/2 of VRefint
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::OneHalfVref)
    }
    ///3/4 of VRefint
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::ThreeQuarterVref)
    }
    ///VRefint
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::Vref)
    }
    ///PA4 or DAC1_CH1 output if enabled
    #[inline(always)]
    pub fn pa4_dac1_ch1(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::Pa4Dac1Ch1)
    }
    ///DAC1_CH2
    #[inline(always)]
    pub fn dac1_ch2(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::Dac1Ch2)
    }
    ///PB15
    #[inline(always)]
    pub fn pb15(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::Pb15)
    }
}
///Field `COMP6INPSEL` reader - Comparator 6 non inverted input
pub type COMP6INPSEL_R = crate::BitReader<bool>;
///Field `COMP6INPSEL` writer - Comparator 6 non inverted input
pub type COMP6INPSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP6_CSR_SPEC, bool, O>;
///Field `COMP6WINMODE` reader - Comparator 6 window mode
pub type COMP6WINMODE_R = crate::BitReader<bool>;
///Field `COMP6WINMODE` writer - Comparator 6 window mode
pub type COMP6WINMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP6_CSR_SPEC, bool, O>;
///Field `COMP6OUTSEL` reader - Comparator 6 output selection
pub type COMP6OUTSEL_R = crate::FieldReader<u8, COMP6OUTSEL_A>;
///Comparator 6 output selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP6OUTSEL_A {
    ///0: No selection
    NoSelection = 0,
    ///1: Timer 1 break input
    Timer1breakInput = 1,
    ///2: Timer 1 break input 2
    Timer1breakInput2 = 2,
    ///6: Timer 2 input capture 2
    Timer2inputCapture2 = 6,
    ///8: Timer 2 OCREF_CLR input
    Timer2ocrefClearInput = 8,
    ///9: Timer 16 OCREF_CLR input
    Timer16ocrefClearInput = 9,
    ///10: Timer 16 input capture 1
    Timer16inputCapture1 = 10,
}
impl From<COMP6OUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP6OUTSEL_A) -> Self {
        variant as _
    }
}
impl COMP6OUTSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP6OUTSEL_A> {
        match self.bits {
            0 => Some(COMP6OUTSEL_A::NoSelection),
            1 => Some(COMP6OUTSEL_A::Timer1breakInput),
            2 => Some(COMP6OUTSEL_A::Timer1breakInput2),
            6 => Some(COMP6OUTSEL_A::Timer2inputCapture2),
            8 => Some(COMP6OUTSEL_A::Timer2ocrefClearInput),
            9 => Some(COMP6OUTSEL_A::Timer16ocrefClearInput),
            10 => Some(COMP6OUTSEL_A::Timer16inputCapture1),
            _ => None,
        }
    }
    ///Checks if the value of the field is `NoSelection`
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        *self == COMP6OUTSEL_A::NoSelection
    }
    ///Checks if the value of the field is `Timer1breakInput`
    #[inline(always)]
    pub fn is_timer1break_input(&self) -> bool {
        *self == COMP6OUTSEL_A::Timer1breakInput
    }
    ///Checks if the value of the field is `Timer1breakInput2`
    #[inline(always)]
    pub fn is_timer1break_input2(&self) -> bool {
        *self == COMP6OUTSEL_A::Timer1breakInput2
    }
    ///Checks if the value of the field is `Timer2inputCapture2`
    #[inline(always)]
    pub fn is_timer2input_capture2(&self) -> bool {
        *self == COMP6OUTSEL_A::Timer2inputCapture2
    }
    ///Checks if the value of the field is `Timer2ocrefClearInput`
    #[inline(always)]
    pub fn is_timer2ocref_clear_input(&self) -> bool {
        *self == COMP6OUTSEL_A::Timer2ocrefClearInput
    }
    ///Checks if the value of the field is `Timer16ocrefClearInput`
    #[inline(always)]
    pub fn is_timer16ocref_clear_input(&self) -> bool {
        *self == COMP6OUTSEL_A::Timer16ocrefClearInput
    }
    ///Checks if the value of the field is `Timer16inputCapture1`
    #[inline(always)]
    pub fn is_timer16input_capture1(&self) -> bool {
        *self == COMP6OUTSEL_A::Timer16inputCapture1
    }
}
///Field `COMP6OUTSEL` writer - Comparator 6 output selection
pub type COMP6OUTSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP6_CSR_SPEC, u8, COMP6OUTSEL_A, 4, O>;
impl<'a, const O: u8> COMP6OUTSEL_W<'a, O> {
    ///No selection
    #[inline(always)]
    pub fn no_selection(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::NoSelection)
    }
    ///Timer 1 break input
    #[inline(always)]
    pub fn timer1break_input(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::Timer1breakInput)
    }
    ///Timer 1 break input 2
    #[inline(always)]
    pub fn timer1break_input2(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::Timer1breakInput2)
    }
    ///Timer 2 input capture 2
    #[inline(always)]
    pub fn timer2input_capture2(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::Timer2inputCapture2)
    }
    ///Timer 2 OCREF_CLR input
    #[inline(always)]
    pub fn timer2ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::Timer2ocrefClearInput)
    }
    ///Timer 16 OCREF_CLR input
    #[inline(always)]
    pub fn timer16ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::Timer16ocrefClearInput)
    }
    ///Timer 16 input capture 1
    #[inline(always)]
    pub fn timer16input_capture1(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::Timer16inputCapture1)
    }
}
///Field `COMP6POL` reader - Comparator 6 output polarity
pub type COMP6POL_R = crate::BitReader<COMP6POL_A>;
///Comparator 6 output polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP6POL_A {
    ///0: Output is not inverted
    NotInverted = 0,
    ///1: Output is inverted
    Inverted = 1,
}
impl From<COMP6POL_A> for bool {
    #[inline(always)]
    fn from(variant: COMP6POL_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP6POL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> COMP6POL_A {
        match self.bits {
            false => COMP6POL_A::NotInverted,
            true => COMP6POL_A::Inverted,
        }
    }
    ///Checks if the value of the field is `NotInverted`
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == COMP6POL_A::NotInverted
    }
    ///Checks if the value of the field is `Inverted`
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == COMP6POL_A::Inverted
    }
}
///Field `COMP6POL` writer - Comparator 6 output polarity
pub type COMP6POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP6_CSR_SPEC, COMP6POL_A, O>;
impl<'a, const O: u8> COMP6POL_W<'a, O> {
    ///Output is not inverted
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(COMP6POL_A::NotInverted)
    }
    ///Output is inverted
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(COMP6POL_A::Inverted)
    }
}
///Field `COMP6HYST` reader - Comparator 6 hysteresis
pub type COMP6HYST_R = crate::FieldReader<u8, u8>;
///Field `COMP6HYST` writer - Comparator 6 hysteresis
pub type COMP6HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP6_CSR_SPEC, u8, u8, 2, O>;
///Field `COMP6_BLANKING` reader - Comparator 6 blanking source
pub type COMP6_BLANKING_R = crate::FieldReader<u8, COMP6_BLANKING_A>;
///Comparator 6 blanking source
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP6_BLANKING_A {
    ///0: No blanking
    NoBlanking = 0,
    ///3: TIM2 OC4 selected as blanking source
    Tim2oc4 = 3,
    ///4: TIM15 OC2 selected as blanking source
    Tim15oc2 = 4,
}
impl From<COMP6_BLANKING_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP6_BLANKING_A) -> Self {
        variant as _
    }
}
impl COMP6_BLANKING_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP6_BLANKING_A> {
        match self.bits {
            0 => Some(COMP6_BLANKING_A::NoBlanking),
            3 => Some(COMP6_BLANKING_A::Tim2oc4),
            4 => Some(COMP6_BLANKING_A::Tim15oc2),
            _ => None,
        }
    }
    ///Checks if the value of the field is `NoBlanking`
    #[inline(always)]
    pub fn is_no_blanking(&self) -> bool {
        *self == COMP6_BLANKING_A::NoBlanking
    }
    ///Checks if the value of the field is `Tim2oc4`
    #[inline(always)]
    pub fn is_tim2oc4(&self) -> bool {
        *self == COMP6_BLANKING_A::Tim2oc4
    }
    ///Checks if the value of the field is `Tim15oc2`
    #[inline(always)]
    pub fn is_tim15oc2(&self) -> bool {
        *self == COMP6_BLANKING_A::Tim15oc2
    }
}
///Field `COMP6_BLANKING` writer - Comparator 6 blanking source
pub type COMP6_BLANKING_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP6_CSR_SPEC, u8, COMP6_BLANKING_A, 3, O>;
impl<'a, const O: u8> COMP6_BLANKING_W<'a, O> {
    ///No blanking
    #[inline(always)]
    pub fn no_blanking(self) -> &'a mut W {
        self.variant(COMP6_BLANKING_A::NoBlanking)
    }
    ///TIM2 OC4 selected as blanking source
    #[inline(always)]
    pub fn tim2oc4(self) -> &'a mut W {
        self.variant(COMP6_BLANKING_A::Tim2oc4)
    }
    ///TIM15 OC2 selected as blanking source
    #[inline(always)]
    pub fn tim15oc2(self) -> &'a mut W {
        self.variant(COMP6_BLANKING_A::Tim15oc2)
    }
}
///Field `COMP6INMSEL3` reader - Comparator 6 inverting input selection
pub type COMP6INMSEL3_R = crate::BitReader<bool>;
///Field `COMP6INMSEL3` writer - Comparator 6 inverting input selection
pub type COMP6INMSEL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP6_CSR_SPEC, bool, O>;
///Field `COMP6OUT` reader - Comparator 6 output
pub type COMP6OUT_R = crate::BitReader<COMP6OUT_A>;
///Comparator 6 output
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP6OUT_A {
    ///0: Non-inverting input below inverting input
    Low = 0,
    ///1: Non-inverting input above inverting input
    High = 1,
}
impl From<COMP6OUT_A> for bool {
    #[inline(always)]
    fn from(variant: COMP6OUT_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP6OUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> COMP6OUT_A {
        match self.bits {
            false => COMP6OUT_A::Low,
            true => COMP6OUT_A::High,
        }
    }
    ///Checks if the value of the field is `Low`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == COMP6OUT_A::Low
    }
    ///Checks if the value of the field is `High`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == COMP6OUT_A::High
    }
}
///Field `COMP6LOCK` reader - Comparator 6 lock
pub type COMP6LOCK_R = crate::BitReader<COMP6LOCK_A>;
///Comparator 6 lock
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP6LOCK_A {
    ///0: Comparator CSR bits are read-write
    Unlocked = 0,
    ///1: Comparator CSR bits are read-only
    Locked = 1,
}
impl From<COMP6LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: COMP6LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP6LOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> COMP6LOCK_A {
        match self.bits {
            false => COMP6LOCK_A::Unlocked,
            true => COMP6LOCK_A::Locked,
        }
    }
    ///Checks if the value of the field is `Unlocked`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == COMP6LOCK_A::Unlocked
    }
    ///Checks if the value of the field is `Locked`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == COMP6LOCK_A::Locked
    }
}
///Field `COMP6LOCK` writer - Comparator 6 lock
pub type COMP6LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP6_CSR_SPEC, COMP6LOCK_A, O>;
impl<'a, const O: u8> COMP6LOCK_W<'a, O> {
    ///Comparator CSR bits are read-write
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(COMP6LOCK_A::Unlocked)
    }
    ///Comparator CSR bits are read-only
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(COMP6LOCK_A::Locked)
    }
}
impl R {
    ///Bit 0 - Comparator 6 enable
    #[inline(always)]
    pub fn comp6en(&self) -> COMP6EN_R {
        COMP6EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:3 - Comparator 6 mode
    #[inline(always)]
    pub fn comp6mode(&self) -> COMP6MODE_R {
        COMP6MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:6 - Comparator 6 inverting input selection
    #[inline(always)]
    pub fn comp6inmsel(&self) -> COMP6INMSEL_R {
        COMP6INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Comparator 6 non inverted input
    #[inline(always)]
    pub fn comp6inpsel(&self) -> COMP6INPSEL_R {
        COMP6INPSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - Comparator 6 window mode
    #[inline(always)]
    pub fn comp6winmode(&self) -> COMP6WINMODE_R {
        COMP6WINMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:13 - Comparator 6 output selection
    #[inline(always)]
    pub fn comp6outsel(&self) -> COMP6OUTSEL_R {
        COMP6OUTSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    ///Bit 15 - Comparator 6 output polarity
    #[inline(always)]
    pub fn comp6pol(&self) -> COMP6POL_R {
        COMP6POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Comparator 6 hysteresis
    #[inline(always)]
    pub fn comp6hyst(&self) -> COMP6HYST_R {
        COMP6HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:20 - Comparator 6 blanking source
    #[inline(always)]
    pub fn comp6_blanking(&self) -> COMP6_BLANKING_R {
        COMP6_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 22 - Comparator 6 inverting input selection
    #[inline(always)]
    pub fn comp6inmsel3(&self) -> COMP6INMSEL3_R {
        COMP6INMSEL3_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 30 - Comparator 6 output
    #[inline(always)]
    pub fn comp6out(&self) -> COMP6OUT_R {
        COMP6OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Comparator 6 lock
    #[inline(always)]
    pub fn comp6lock(&self) -> COMP6LOCK_R {
        COMP6LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Comparator 6 enable
    #[inline(always)]
    #[must_use]
    pub fn comp6en(&mut self) -> COMP6EN_W<0> {
        COMP6EN_W::new(self)
    }
    ///Bits 2:3 - Comparator 6 mode
    #[inline(always)]
    #[must_use]
    pub fn comp6mode(&mut self) -> COMP6MODE_W<2> {
        COMP6MODE_W::new(self)
    }
    ///Bits 4:6 - Comparator 6 inverting input selection
    #[inline(always)]
    #[must_use]
    pub fn comp6inmsel(&mut self) -> COMP6INMSEL_W<4> {
        COMP6INMSEL_W::new(self)
    }
    ///Bit 7 - Comparator 6 non inverted input
    #[inline(always)]
    #[must_use]
    pub fn comp6inpsel(&mut self) -> COMP6INPSEL_W<7> {
        COMP6INPSEL_W::new(self)
    }
    ///Bit 9 - Comparator 6 window mode
    #[inline(always)]
    #[must_use]
    pub fn comp6winmode(&mut self) -> COMP6WINMODE_W<9> {
        COMP6WINMODE_W::new(self)
    }
    ///Bits 10:13 - Comparator 6 output selection
    #[inline(always)]
    #[must_use]
    pub fn comp6outsel(&mut self) -> COMP6OUTSEL_W<10> {
        COMP6OUTSEL_W::new(self)
    }
    ///Bit 15 - Comparator 6 output polarity
    #[inline(always)]
    #[must_use]
    pub fn comp6pol(&mut self) -> COMP6POL_W<15> {
        COMP6POL_W::new(self)
    }
    ///Bits 16:17 - Comparator 6 hysteresis
    #[inline(always)]
    #[must_use]
    pub fn comp6hyst(&mut self) -> COMP6HYST_W<16> {
        COMP6HYST_W::new(self)
    }
    ///Bits 18:20 - Comparator 6 blanking source
    #[inline(always)]
    #[must_use]
    pub fn comp6_blanking(&mut self) -> COMP6_BLANKING_W<18> {
        COMP6_BLANKING_W::new(self)
    }
    ///Bit 22 - Comparator 6 inverting input selection
    #[inline(always)]
    #[must_use]
    pub fn comp6inmsel3(&mut self) -> COMP6INMSEL3_W<22> {
        COMP6INMSEL3_W::new(self)
    }
    ///Bit 31 - Comparator 6 lock
    #[inline(always)]
    #[must_use]
    pub fn comp6lock(&mut self) -> COMP6LOCK_W<31> {
        COMP6LOCK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [comp6_csr](index.html) module
pub struct COMP6_CSR_SPEC;
impl crate::RegisterSpec for COMP6_CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [comp6_csr::R](R) reader structure
impl crate::Readable for COMP6_CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [comp6_csr::W](W) writer structure
impl crate::Writable for COMP6_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets COMP6_CSR to value 0
impl crate::Resettable for COMP6_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
