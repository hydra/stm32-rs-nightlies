///Register `SMCR` reader
pub struct R(crate::R<SMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SMCR` writer
pub struct W(crate::W<SMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMCR_SPEC>;
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
impl From<crate::W<SMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SMS` reader - Slave mode selection
pub type SMS_R = crate::FieldReader<u8, SMS_A>;
///Slave mode selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMS_A {
    ///0: Slave mode disabled - if CEN = ‘1 then the prescaler is clocked directly by the internal clock.
    Disabled = 0,
    ///1: Encoder mode 1 - Counter counts up/down on TI2FP1 edge depending on TI1FP2 level.
    EncoderMode1 = 1,
    ///2: Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level.
    EncoderMode2 = 2,
    ///3: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input.
    EncoderMode3 = 3,
    ///4: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers.
    ResetMode = 4,
    ///5: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled.
    GatedMode = 5,
    ///6: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled.
    TriggerMode = 6,
    ///7: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter.
    ExtClockMode = 7,
}
impl From<SMS_A> for u8 {
    #[inline(always)]
    fn from(variant: SMS_A) -> Self {
        variant as _
    }
}
impl SMS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SMS_A {
        match self.bits {
            0 => SMS_A::Disabled,
            1 => SMS_A::EncoderMode1,
            2 => SMS_A::EncoderMode2,
            3 => SMS_A::EncoderMode3,
            4 => SMS_A::ResetMode,
            5 => SMS_A::GatedMode,
            6 => SMS_A::TriggerMode,
            7 => SMS_A::ExtClockMode,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMS_A::Disabled
    }
    ///Checks if the value of the field is `EncoderMode1`
    #[inline(always)]
    pub fn is_encoder_mode_1(&self) -> bool {
        *self == SMS_A::EncoderMode1
    }
    ///Checks if the value of the field is `EncoderMode2`
    #[inline(always)]
    pub fn is_encoder_mode_2(&self) -> bool {
        *self == SMS_A::EncoderMode2
    }
    ///Checks if the value of the field is `EncoderMode3`
    #[inline(always)]
    pub fn is_encoder_mode_3(&self) -> bool {
        *self == SMS_A::EncoderMode3
    }
    ///Checks if the value of the field is `ResetMode`
    #[inline(always)]
    pub fn is_reset_mode(&self) -> bool {
        *self == SMS_A::ResetMode
    }
    ///Checks if the value of the field is `GatedMode`
    #[inline(always)]
    pub fn is_gated_mode(&self) -> bool {
        *self == SMS_A::GatedMode
    }
    ///Checks if the value of the field is `TriggerMode`
    #[inline(always)]
    pub fn is_trigger_mode(&self) -> bool {
        *self == SMS_A::TriggerMode
    }
    ///Checks if the value of the field is `ExtClockMode`
    #[inline(always)]
    pub fn is_ext_clock_mode(&self) -> bool {
        *self == SMS_A::ExtClockMode
    }
}
///Field `SMS` writer - Slave mode selection
pub type SMS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SMCR_SPEC, u8, SMS_A, 3, O>;
impl<'a, const O: u8> SMS_W<'a, O> {
    ///Slave mode disabled - if CEN = ‘1 then the prescaler is clocked directly by the internal clock.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SMS_A::Disabled)
    }
    ///Encoder mode 1 - Counter counts up/down on TI2FP1 edge depending on TI1FP2 level.
    #[inline(always)]
    pub fn encoder_mode_1(self) -> &'a mut W {
        self.variant(SMS_A::EncoderMode1)
    }
    ///Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level.
    #[inline(always)]
    pub fn encoder_mode_2(self) -> &'a mut W {
        self.variant(SMS_A::EncoderMode2)
    }
    ///Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input.
    #[inline(always)]
    pub fn encoder_mode_3(self) -> &'a mut W {
        self.variant(SMS_A::EncoderMode3)
    }
    ///Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers.
    #[inline(always)]
    pub fn reset_mode(self) -> &'a mut W {
        self.variant(SMS_A::ResetMode)
    }
    ///Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled.
    #[inline(always)]
    pub fn gated_mode(self) -> &'a mut W {
        self.variant(SMS_A::GatedMode)
    }
    ///Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled.
    #[inline(always)]
    pub fn trigger_mode(self) -> &'a mut W {
        self.variant(SMS_A::TriggerMode)
    }
    ///External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter.
    #[inline(always)]
    pub fn ext_clock_mode(self) -> &'a mut W {
        self.variant(SMS_A::ExtClockMode)
    }
}
///Field `TS` reader - Trigger selection
pub type TS_R = crate::FieldReader<u8, TS_A>;
///Trigger selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TS_A {
    ///0: Internal Trigger 0 (ITR0)
    Itr0 = 0,
    ///1: Internal Trigger 1 (ITR1)
    Itr1 = 1,
    ///2: Internal Trigger 2 (ITR2)
    Itr2 = 2,
    ///4: TI1 Edge Detector (TI1F_ED)
    Ti1fEd = 4,
    ///5: Filtered Timer Input 1 (TI1FP1)
    Ti1fp1 = 5,
    ///6: Filtered Timer Input 2 (TI2FP2)
    Ti2fp2 = 6,
    ///7: External Trigger input (ETRF)
    Etrf = 7,
}
impl From<TS_A> for u8 {
    #[inline(always)]
    fn from(variant: TS_A) -> Self {
        variant as _
    }
}
impl TS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<TS_A> {
        match self.bits {
            0 => Some(TS_A::Itr0),
            1 => Some(TS_A::Itr1),
            2 => Some(TS_A::Itr2),
            4 => Some(TS_A::Ti1fEd),
            5 => Some(TS_A::Ti1fp1),
            6 => Some(TS_A::Ti2fp2),
            7 => Some(TS_A::Etrf),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Itr0`
    #[inline(always)]
    pub fn is_itr0(&self) -> bool {
        *self == TS_A::Itr0
    }
    ///Checks if the value of the field is `Itr1`
    #[inline(always)]
    pub fn is_itr1(&self) -> bool {
        *self == TS_A::Itr1
    }
    ///Checks if the value of the field is `Itr2`
    #[inline(always)]
    pub fn is_itr2(&self) -> bool {
        *self == TS_A::Itr2
    }
    ///Checks if the value of the field is `Ti1fEd`
    #[inline(always)]
    pub fn is_ti1f_ed(&self) -> bool {
        *self == TS_A::Ti1fEd
    }
    ///Checks if the value of the field is `Ti1fp1`
    #[inline(always)]
    pub fn is_ti1fp1(&self) -> bool {
        *self == TS_A::Ti1fp1
    }
    ///Checks if the value of the field is `Ti2fp2`
    #[inline(always)]
    pub fn is_ti2fp2(&self) -> bool {
        *self == TS_A::Ti2fp2
    }
    ///Checks if the value of the field is `Etrf`
    #[inline(always)]
    pub fn is_etrf(&self) -> bool {
        *self == TS_A::Etrf
    }
}
///Field `TS` writer - Trigger selection
pub type TS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMCR_SPEC, u8, TS_A, 3, O>;
impl<'a, const O: u8> TS_W<'a, O> {
    ///Internal Trigger 0 (ITR0)
    #[inline(always)]
    pub fn itr0(self) -> &'a mut W {
        self.variant(TS_A::Itr0)
    }
    ///Internal Trigger 1 (ITR1)
    #[inline(always)]
    pub fn itr1(self) -> &'a mut W {
        self.variant(TS_A::Itr1)
    }
    ///Internal Trigger 2 (ITR2)
    #[inline(always)]
    pub fn itr2(self) -> &'a mut W {
        self.variant(TS_A::Itr2)
    }
    ///TI1 Edge Detector (TI1F_ED)
    #[inline(always)]
    pub fn ti1f_ed(self) -> &'a mut W {
        self.variant(TS_A::Ti1fEd)
    }
    ///Filtered Timer Input 1 (TI1FP1)
    #[inline(always)]
    pub fn ti1fp1(self) -> &'a mut W {
        self.variant(TS_A::Ti1fp1)
    }
    ///Filtered Timer Input 2 (TI2FP2)
    #[inline(always)]
    pub fn ti2fp2(self) -> &'a mut W {
        self.variant(TS_A::Ti2fp2)
    }
    ///External Trigger input (ETRF)
    #[inline(always)]
    pub fn etrf(self) -> &'a mut W {
        self.variant(TS_A::Etrf)
    }
}
///Field `MSM` reader - Master/Slave mode
pub type MSM_R = crate::BitReader<MSM_A>;
///Master/Slave mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSM_A {
    ///0: No action
    NoSync = 0,
    ///1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event.
    Sync = 1,
}
impl From<MSM_A> for bool {
    #[inline(always)]
    fn from(variant: MSM_A) -> Self {
        variant as u8 != 0
    }
}
impl MSM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MSM_A {
        match self.bits {
            false => MSM_A::NoSync,
            true => MSM_A::Sync,
        }
    }
    ///Checks if the value of the field is `NoSync`
    #[inline(always)]
    pub fn is_no_sync(&self) -> bool {
        *self == MSM_A::NoSync
    }
    ///Checks if the value of the field is `Sync`
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == MSM_A::Sync
    }
}
///Field `MSM` writer - Master/Slave mode
pub type MSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMCR_SPEC, MSM_A, O>;
impl<'a, const O: u8> MSM_W<'a, O> {
    ///No action
    #[inline(always)]
    pub fn no_sync(self) -> &'a mut W {
        self.variant(MSM_A::NoSync)
    }
    ///The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event.
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(MSM_A::Sync)
    }
}
///Field `ETF` reader - External trigger filter
pub type ETF_R = crate::FieldReader<u8, ETF_A>;
///External trigger filter
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETF_A {
    ///0: No filter, sampling is done at fDTS
    NoFilter = 0,
    ///1: fSAMPLING=fCK_INT, N=2
    FckIntN2 = 1,
    ///2: fSAMPLING=fCK_INT, N=4
    FckIntN4 = 2,
    ///3: fSAMPLING=fCK_INT, N=8
    FckIntN8 = 3,
    ///4: fSAMPLING=fDTS/2, N=6
    FdtsDiv2N6 = 4,
    ///5: fSAMPLING=fDTS/2, N=8
    FdtsDiv2N8 = 5,
    ///6: fSAMPLING=fDTS/4, N=6
    FdtsDiv4N6 = 6,
    ///7: fSAMPLING=fDTS/4, N=8
    FdtsDiv4N8 = 7,
    ///8: fSAMPLING=fDTS/8, N=6
    FdtsDiv8N6 = 8,
    ///9: fSAMPLING=fDTS/8, N=8
    FdtsDiv8N8 = 9,
    ///10: fSAMPLING=fDTS/16, N=5
    FdtsDiv16N5 = 10,
    ///11: fSAMPLING=fDTS/16, N=6
    FdtsDiv16N6 = 11,
    ///12: fSAMPLING=fDTS/16, N=8
    FdtsDiv16N8 = 12,
    ///13: fSAMPLING=fDTS/32, N=5
    FdtsDiv32N5 = 13,
    ///14: fSAMPLING=fDTS/32, N=6
    FdtsDiv32N6 = 14,
    ///15: fSAMPLING=fDTS/32, N=8
    FdtsDiv32N8 = 15,
}
impl From<ETF_A> for u8 {
    #[inline(always)]
    fn from(variant: ETF_A) -> Self {
        variant as _
    }
}
impl ETF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ETF_A {
        match self.bits {
            0 => ETF_A::NoFilter,
            1 => ETF_A::FckIntN2,
            2 => ETF_A::FckIntN4,
            3 => ETF_A::FckIntN8,
            4 => ETF_A::FdtsDiv2N6,
            5 => ETF_A::FdtsDiv2N8,
            6 => ETF_A::FdtsDiv4N6,
            7 => ETF_A::FdtsDiv4N8,
            8 => ETF_A::FdtsDiv8N6,
            9 => ETF_A::FdtsDiv8N8,
            10 => ETF_A::FdtsDiv16N5,
            11 => ETF_A::FdtsDiv16N6,
            12 => ETF_A::FdtsDiv16N8,
            13 => ETF_A::FdtsDiv32N5,
            14 => ETF_A::FdtsDiv32N6,
            15 => ETF_A::FdtsDiv32N8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NoFilter`
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == ETF_A::NoFilter
    }
    ///Checks if the value of the field is `FckIntN2`
    #[inline(always)]
    pub fn is_fck_int_n2(&self) -> bool {
        *self == ETF_A::FckIntN2
    }
    ///Checks if the value of the field is `FckIntN4`
    #[inline(always)]
    pub fn is_fck_int_n4(&self) -> bool {
        *self == ETF_A::FckIntN4
    }
    ///Checks if the value of the field is `FckIntN8`
    #[inline(always)]
    pub fn is_fck_int_n8(&self) -> bool {
        *self == ETF_A::FckIntN8
    }
    ///Checks if the value of the field is `FdtsDiv2N6`
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        *self == ETF_A::FdtsDiv2N6
    }
    ///Checks if the value of the field is `FdtsDiv2N8`
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        *self == ETF_A::FdtsDiv2N8
    }
    ///Checks if the value of the field is `FdtsDiv4N6`
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        *self == ETF_A::FdtsDiv4N6
    }
    ///Checks if the value of the field is `FdtsDiv4N8`
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        *self == ETF_A::FdtsDiv4N8
    }
    ///Checks if the value of the field is `FdtsDiv8N6`
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        *self == ETF_A::FdtsDiv8N6
    }
    ///Checks if the value of the field is `FdtsDiv8N8`
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        *self == ETF_A::FdtsDiv8N8
    }
    ///Checks if the value of the field is `FdtsDiv16N5`
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        *self == ETF_A::FdtsDiv16N5
    }
    ///Checks if the value of the field is `FdtsDiv16N6`
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        *self == ETF_A::FdtsDiv16N6
    }
    ///Checks if the value of the field is `FdtsDiv16N8`
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        *self == ETF_A::FdtsDiv16N8
    }
    ///Checks if the value of the field is `FdtsDiv32N5`
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        *self == ETF_A::FdtsDiv32N5
    }
    ///Checks if the value of the field is `FdtsDiv32N6`
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        *self == ETF_A::FdtsDiv32N6
    }
    ///Checks if the value of the field is `FdtsDiv32N8`
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        *self == ETF_A::FdtsDiv32N8
    }
}
///Field `ETF` writer - External trigger filter
pub type ETF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SMCR_SPEC, u8, ETF_A, 4, O>;
impl<'a, const O: u8> ETF_W<'a, O> {
    ///No filter, sampling is done at fDTS
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(ETF_A::NoFilter)
    }
    ///fSAMPLING=fCK_INT, N=2
    #[inline(always)]
    pub fn fck_int_n2(self) -> &'a mut W {
        self.variant(ETF_A::FckIntN2)
    }
    ///fSAMPLING=fCK_INT, N=4
    #[inline(always)]
    pub fn fck_int_n4(self) -> &'a mut W {
        self.variant(ETF_A::FckIntN4)
    }
    ///fSAMPLING=fCK_INT, N=8
    #[inline(always)]
    pub fn fck_int_n8(self) -> &'a mut W {
        self.variant(ETF_A::FckIntN8)
    }
    ///fSAMPLING=fDTS/2, N=6
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut W {
        self.variant(ETF_A::FdtsDiv2N6)
    }
    ///fSAMPLING=fDTS/2, N=8
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut W {
        self.variant(ETF_A::FdtsDiv2N8)
    }
    ///fSAMPLING=fDTS/4, N=6
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut W {
        self.variant(ETF_A::FdtsDiv4N6)
    }
    ///fSAMPLING=fDTS/4, N=8
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut W {
        self.variant(ETF_A::FdtsDiv4N8)
    }
    ///fSAMPLING=fDTS/8, N=6
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut W {
        self.variant(ETF_A::FdtsDiv8N6)
    }
    ///fSAMPLING=fDTS/8, N=8
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut W {
        self.variant(ETF_A::FdtsDiv8N8)
    }
    ///fSAMPLING=fDTS/16, N=5
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut W {
        self.variant(ETF_A::FdtsDiv16N5)
    }
    ///fSAMPLING=fDTS/16, N=6
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut W {
        self.variant(ETF_A::FdtsDiv16N6)
    }
    ///fSAMPLING=fDTS/16, N=8
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut W {
        self.variant(ETF_A::FdtsDiv16N8)
    }
    ///fSAMPLING=fDTS/32, N=5
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut W {
        self.variant(ETF_A::FdtsDiv32N5)
    }
    ///fSAMPLING=fDTS/32, N=6
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut W {
        self.variant(ETF_A::FdtsDiv32N6)
    }
    ///fSAMPLING=fDTS/32, N=8
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut W {
        self.variant(ETF_A::FdtsDiv32N8)
    }
}
///Field `ETPS` reader - External trigger prescaler
pub type ETPS_R = crate::FieldReader<u8, ETPS_A>;
///External trigger prescaler
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETPS_A {
    ///0: Prescaler OFF
    Div1 = 0,
    ///1: ETRP frequency divided by 2
    Div2 = 1,
    ///2: ETRP frequency divided by 4
    Div4 = 2,
    ///3: ETRP frequency divided by 8
    Div8 = 3,
}
impl From<ETPS_A> for u8 {
    #[inline(always)]
    fn from(variant: ETPS_A) -> Self {
        variant as _
    }
}
impl ETPS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ETPS_A {
        match self.bits {
            0 => ETPS_A::Div1,
            1 => ETPS_A::Div2,
            2 => ETPS_A::Div4,
            3 => ETPS_A::Div8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == ETPS_A::Div1
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == ETPS_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == ETPS_A::Div4
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == ETPS_A::Div8
    }
}
///Field `ETPS` writer - External trigger prescaler
pub type ETPS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SMCR_SPEC, u8, ETPS_A, 2, O>;
impl<'a, const O: u8> ETPS_W<'a, O> {
    ///Prescaler OFF
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(ETPS_A::Div1)
    }
    ///ETRP frequency divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(ETPS_A::Div2)
    }
    ///ETRP frequency divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(ETPS_A::Div4)
    }
    ///ETRP frequency divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(ETPS_A::Div8)
    }
}
///Field `ECE` reader - External clock enable
pub type ECE_R = crate::BitReader<ECE_A>;
///External clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECE_A {
    ///0: External clock mode 2 disabled
    Disabled = 0,
    ///1: External clock mode 2 enabled
    Enabled = 1,
}
impl From<ECE_A> for bool {
    #[inline(always)]
    fn from(variant: ECE_A) -> Self {
        variant as u8 != 0
    }
}
impl ECE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ECE_A {
        match self.bits {
            false => ECE_A::Disabled,
            true => ECE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ECE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ECE_A::Enabled
    }
}
///Field `ECE` writer - External clock enable
pub type ECE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMCR_SPEC, ECE_A, O>;
impl<'a, const O: u8> ECE_W<'a, O> {
    ///External clock mode 2 disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ECE_A::Disabled)
    }
    ///External clock mode 2 enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ECE_A::Enabled)
    }
}
///Field `ETP` reader - External trigger polarity
pub type ETP_R = crate::BitReader<ETP_A>;
///External trigger polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETP_A {
    ///0: ETR is non-inverted, active at high level or rising edge
    RisingEdge = 0,
    ///1: ETR is inverted, active at low level or falling edge
    FallingEdge = 1,
}
impl From<ETP_A> for bool {
    #[inline(always)]
    fn from(variant: ETP_A) -> Self {
        variant as u8 != 0
    }
}
impl ETP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ETP_A {
        match self.bits {
            false => ETP_A::RisingEdge,
            true => ETP_A::FallingEdge,
        }
    }
    ///Checks if the value of the field is `RisingEdge`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == ETP_A::RisingEdge
    }
    ///Checks if the value of the field is `FallingEdge`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == ETP_A::FallingEdge
    }
}
///Field `ETP` writer - External trigger polarity
pub type ETP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMCR_SPEC, ETP_A, O>;
impl<'a, const O: u8> ETP_W<'a, O> {
    ///ETR is non-inverted, active at high level or rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(ETP_A::RisingEdge)
    }
    ///ETR is inverted, active at low level or falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(ETP_A::FallingEdge)
    }
}
impl R {
    ///Bits 0:2 - Slave mode selection
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:6 - Trigger selection
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Master/Slave mode
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - External trigger filter
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:13 - External trigger prescaler
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - External clock enable
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - External trigger polarity
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - Slave mode selection
    #[inline(always)]
    #[must_use]
    pub fn sms(&mut self) -> SMS_W<0> {
        SMS_W::new(self)
    }
    ///Bits 4:6 - Trigger selection
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<4> {
        TS_W::new(self)
    }
    ///Bit 7 - Master/Slave mode
    #[inline(always)]
    #[must_use]
    pub fn msm(&mut self) -> MSM_W<7> {
        MSM_W::new(self)
    }
    ///Bits 8:11 - External trigger filter
    #[inline(always)]
    #[must_use]
    pub fn etf(&mut self) -> ETF_W<8> {
        ETF_W::new(self)
    }
    ///Bits 12:13 - External trigger prescaler
    #[inline(always)]
    #[must_use]
    pub fn etps(&mut self) -> ETPS_W<12> {
        ETPS_W::new(self)
    }
    ///Bit 14 - External clock enable
    #[inline(always)]
    #[must_use]
    pub fn ece(&mut self) -> ECE_W<14> {
        ECE_W::new(self)
    }
    ///Bit 15 - External trigger polarity
    #[inline(always)]
    #[must_use]
    pub fn etp(&mut self) -> ETP_W<15> {
        ETP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///slave mode control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [smcr](index.html) module
pub struct SMCR_SPEC;
impl crate::RegisterSpec for SMCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [smcr::R](R) reader structure
impl crate::Readable for SMCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [smcr::W](W) writer structure
impl crate::Writable for SMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SMCR to value 0
impl crate::Resettable for SMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
