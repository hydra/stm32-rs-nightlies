///Register `TIMAISR` reader
pub struct R(crate::R<TIMAISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMAISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMAISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMAISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CMP1` reader - Compare 1 Interrupt Flag
pub type CMP1_R = crate::BitReader<CMP1_A>;
///Compare 1 Interrupt Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP1_A {
    ///0: No compare interrupt occurred
    NoEvent = 0,
    ///1: Compare interrupt occurred
    Event = 1,
}
impl From<CMP1_A> for bool {
    #[inline(always)]
    fn from(variant: CMP1_A) -> Self {
        variant as u8 != 0
    }
}
impl CMP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CMP1_A {
        match self.bits {
            false => CMP1_A::NoEvent,
            true => CMP1_A::Event,
        }
    }
    ///Checks if the value of the field is `NoEvent`
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == CMP1_A::NoEvent
    }
    ///Checks if the value of the field is `Event`
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CMP1_A::Event
    }
}
///Field `CMP2` reader - Compare 2 Interrupt Flag
pub use CMP1_R as CMP2_R;
///Field `CMP3` reader - Compare 3 Interrupt Flag
pub use CMP1_R as CMP3_R;
///Field `CMP4` reader - Compare 4 Interrupt Flag
pub use CMP1_R as CMP4_R;
///Field `REP` reader - Repetition Interrupt Flag
pub type REP_R = crate::BitReader<REP_A>;
///Repetition Interrupt Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REP_A {
    ///0: No timer repetition interrupt occurred
    NoEvent = 0,
    ///1: Timer repetition interrupt occurred
    Event = 1,
}
impl From<REP_A> for bool {
    #[inline(always)]
    fn from(variant: REP_A) -> Self {
        variant as u8 != 0
    }
}
impl REP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> REP_A {
        match self.bits {
            false => REP_A::NoEvent,
            true => REP_A::Event,
        }
    }
    ///Checks if the value of the field is `NoEvent`
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == REP_A::NoEvent
    }
    ///Checks if the value of the field is `Event`
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == REP_A::Event
    }
}
///Field `UPD` reader - Update Interrupt Flag
pub type UPD_R = crate::BitReader<UPD_A>;
///Update Interrupt Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPD_A {
    ///0: No timer update interrupt occurred
    NoEvent = 0,
    ///1: Timer update interrupt occurred
    Event = 1,
}
impl From<UPD_A> for bool {
    #[inline(always)]
    fn from(variant: UPD_A) -> Self {
        variant as u8 != 0
    }
}
impl UPD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UPD_A {
        match self.bits {
            false => UPD_A::NoEvent,
            true => UPD_A::Event,
        }
    }
    ///Checks if the value of the field is `NoEvent`
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == UPD_A::NoEvent
    }
    ///Checks if the value of the field is `Event`
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == UPD_A::Event
    }
}
///Field `CPT1` reader - Capture1 Interrupt Flag
pub type CPT1_R = crate::BitReader<CPT1_A>;
///Capture1 Interrupt Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPT1_A {
    ///0: No timer x capture reset interrupt occurred
    NoEvent = 0,
    ///1: Timer x capture reset interrupt occurred
    Event = 1,
}
impl From<CPT1_A> for bool {
    #[inline(always)]
    fn from(variant: CPT1_A) -> Self {
        variant as u8 != 0
    }
}
impl CPT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CPT1_A {
        match self.bits {
            false => CPT1_A::NoEvent,
            true => CPT1_A::Event,
        }
    }
    ///Checks if the value of the field is `NoEvent`
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == CPT1_A::NoEvent
    }
    ///Checks if the value of the field is `Event`
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CPT1_A::Event
    }
}
///Field `CPT2` reader - Capture2 Interrupt Flag
pub use CPT1_R as CPT2_R;
///Field `SETx1` reader - Output 1 Set Interrupt Flag
pub type SETX1_R = crate::BitReader<SETX1_A>;
///Output 1 Set Interrupt Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETX1_A {
    ///0: No Tx output set interrupt occurred
    NoEvent = 0,
    ///1: Tx output set interrupt occurred
    Event = 1,
}
impl From<SETX1_A> for bool {
    #[inline(always)]
    fn from(variant: SETX1_A) -> Self {
        variant as u8 != 0
    }
}
impl SETX1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SETX1_A {
        match self.bits {
            false => SETX1_A::NoEvent,
            true => SETX1_A::Event,
        }
    }
    ///Checks if the value of the field is `NoEvent`
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == SETX1_A::NoEvent
    }
    ///Checks if the value of the field is `Event`
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == SETX1_A::Event
    }
}
///Field `RSTx1` reader - Output 1 Reset Interrupt Flag
pub type RSTX1_R = crate::BitReader<RSTX1_A>;
///Output 1 Reset Interrupt Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTX1_A {
    ///0: No Tx output reset interrupt occurred
    NoEvent = 0,
    ///1: Tx output reset interrupt occurred
    Event = 1,
}
impl From<RSTX1_A> for bool {
    #[inline(always)]
    fn from(variant: RSTX1_A) -> Self {
        variant as u8 != 0
    }
}
impl RSTX1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RSTX1_A {
        match self.bits {
            false => RSTX1_A::NoEvent,
            true => RSTX1_A::Event,
        }
    }
    ///Checks if the value of the field is `NoEvent`
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == RSTX1_A::NoEvent
    }
    ///Checks if the value of the field is `Event`
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == RSTX1_A::Event
    }
}
///Field `RSTx2` reader - Output 2 Reset Interrupt Flag
pub use RSTX1_R as RSTX2_R;
///Field `SETx2` reader - Output 2 Set Interrupt Flag
pub use SETX1_R as SETX2_R;
///Field `RST` reader - Reset Interrupt Flag
pub type RST_R = crate::BitReader<RST_A>;
///Reset Interrupt Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RST_A {
    ///0: No TIMx counter reset/roll-over interrupt occurred
    NoEvent = 0,
    ///1: TIMx counter reset/roll-over interrupt occurred
    Event = 1,
}
impl From<RST_A> for bool {
    #[inline(always)]
    fn from(variant: RST_A) -> Self {
        variant as u8 != 0
    }
}
impl RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RST_A {
        match self.bits {
            false => RST_A::NoEvent,
            true => RST_A::Event,
        }
    }
    ///Checks if the value of the field is `NoEvent`
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == RST_A::NoEvent
    }
    ///Checks if the value of the field is `Event`
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == RST_A::Event
    }
}
///Field `DLYPRT` reader - Delayed Protection Flag
pub type DLYPRT_R = crate::BitReader<DLYPRT_A>;
///Delayed Protection Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYPRT_A {
    ///0: Not in delayed idle or balanced idle mode
    Inactive = 0,
    ///1: Delayed idle or balanced idle mode entry
    Active = 1,
}
impl From<DLYPRT_A> for bool {
    #[inline(always)]
    fn from(variant: DLYPRT_A) -> Self {
        variant as u8 != 0
    }
}
impl DLYPRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DLYPRT_A {
        match self.bits {
            false => DLYPRT_A::Inactive,
            true => DLYPRT_A::Active,
        }
    }
    ///Checks if the value of the field is `Inactive`
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DLYPRT_A::Inactive
    }
    ///Checks if the value of the field is `Active`
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == DLYPRT_A::Active
    }
}
///Field `CPPSTAT` reader - Current Push Pull Status
pub type CPPSTAT_R = crate::BitReader<CPPSTAT_A>;
///Current Push Pull Status
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPPSTAT_A {
    ///0: Signal applied on output 1 and output 2 forced inactive
    Output1active = 0,
    ///1: Signal applied on output 2 and output 1 forced inactive
    Output2active = 1,
}
impl From<CPPSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: CPPSTAT_A) -> Self {
        variant as u8 != 0
    }
}
impl CPPSTAT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CPPSTAT_A {
        match self.bits {
            false => CPPSTAT_A::Output1active,
            true => CPPSTAT_A::Output2active,
        }
    }
    ///Checks if the value of the field is `Output1active`
    #[inline(always)]
    pub fn is_output1active(&self) -> bool {
        *self == CPPSTAT_A::Output1active
    }
    ///Checks if the value of the field is `Output2active`
    #[inline(always)]
    pub fn is_output2active(&self) -> bool {
        *self == CPPSTAT_A::Output2active
    }
}
///Field `IPPSTAT` reader - Idle Push Pull Status
pub type IPPSTAT_R = crate::BitReader<IPPSTAT_A>;
///Idle Push Pull Status
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPPSTAT_A {
    ///0: Protection occurred when the output 1 was active and output 2 forced inactive
    Output1active = 0,
    ///1: Protection occurred when the output 2 was active and output 1 forced inactive
    Output2active = 1,
}
impl From<IPPSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: IPPSTAT_A) -> Self {
        variant as u8 != 0
    }
}
impl IPPSTAT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IPPSTAT_A {
        match self.bits {
            false => IPPSTAT_A::Output1active,
            true => IPPSTAT_A::Output2active,
        }
    }
    ///Checks if the value of the field is `Output1active`
    #[inline(always)]
    pub fn is_output1active(&self) -> bool {
        *self == IPPSTAT_A::Output1active
    }
    ///Checks if the value of the field is `Output2active`
    #[inline(always)]
    pub fn is_output2active(&self) -> bool {
        *self == IPPSTAT_A::Output2active
    }
}
///Field `O1STAT` reader - Output 1 State
pub type O1STAT_R = crate::BitReader<O1STAT_A>;
///Output 1 State
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum O1STAT_A {
    ///0: Output was inactive
    Inactive = 0,
    ///1: Output was active
    Active = 1,
}
impl From<O1STAT_A> for bool {
    #[inline(always)]
    fn from(variant: O1STAT_A) -> Self {
        variant as u8 != 0
    }
}
impl O1STAT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> O1STAT_A {
        match self.bits {
            false => O1STAT_A::Inactive,
            true => O1STAT_A::Active,
        }
    }
    ///Checks if the value of the field is `Inactive`
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == O1STAT_A::Inactive
    }
    ///Checks if the value of the field is `Active`
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == O1STAT_A::Active
    }
}
///Field `O2STAT` reader - Output 2 State
pub use O1STAT_R as O2STAT_R;
///Field `O1CPY` reader - Output 1 Copy
pub type O1CPY_R = crate::BitReader<O1CPY_A>;
///Output 1 Copy
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum O1CPY_A {
    ///0: Output is inactive
    Inactive = 0,
    ///1: Output is active
    Active = 1,
}
impl From<O1CPY_A> for bool {
    #[inline(always)]
    fn from(variant: O1CPY_A) -> Self {
        variant as u8 != 0
    }
}
impl O1CPY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> O1CPY_A {
        match self.bits {
            false => O1CPY_A::Inactive,
            true => O1CPY_A::Active,
        }
    }
    ///Checks if the value of the field is `Inactive`
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == O1CPY_A::Inactive
    }
    ///Checks if the value of the field is `Active`
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == O1CPY_A::Active
    }
}
///Field `O2CPY` reader - Output 2 Copy
pub use O1CPY_R as O2CPY_R;
impl R {
    ///Bit 0 - Compare 1 Interrupt Flag
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Compare 2 Interrupt Flag
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Compare 3 Interrupt Flag
    #[inline(always)]
    pub fn cmp3(&self) -> CMP3_R {
        CMP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Compare 4 Interrupt Flag
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Repetition Interrupt Flag
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Update Interrupt Flag
    #[inline(always)]
    pub fn upd(&self) -> UPD_R {
        UPD_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Capture1 Interrupt Flag
    #[inline(always)]
    pub fn cpt1(&self) -> CPT1_R {
        CPT1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Capture2 Interrupt Flag
    #[inline(always)]
    pub fn cpt2(&self) -> CPT2_R {
        CPT2_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Output 1 Set Interrupt Flag
    #[inline(always)]
    pub fn setx1(&self) -> SETX1_R {
        SETX1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Output 1 Reset Interrupt Flag
    #[inline(always)]
    pub fn rstx1(&self) -> RSTX1_R {
        RSTX1_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output 2 Set Interrupt Flag
    #[inline(always)]
    pub fn setx2(&self) -> SETX2_R {
        SETX2_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Output 2 Reset Interrupt Flag
    #[inline(always)]
    pub fn rstx2(&self) -> RSTX2_R {
        RSTX2_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Reset Interrupt Flag
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Delayed Protection Flag
    #[inline(always)]
    pub fn dlyprt(&self) -> DLYPRT_R {
        DLYPRT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Current Push Pull Status
    #[inline(always)]
    pub fn cppstat(&self) -> CPPSTAT_R {
        CPPSTAT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Idle Push Pull Status
    #[inline(always)]
    pub fn ippstat(&self) -> IPPSTAT_R {
        IPPSTAT_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Output 1 State
    #[inline(always)]
    pub fn o1stat(&self) -> O1STAT_R {
        O1STAT_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Output 2 State
    #[inline(always)]
    pub fn o2stat(&self) -> O2STAT_R {
        O2STAT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Output 1 Copy
    #[inline(always)]
    pub fn o1cpy(&self) -> O1CPY_R {
        O1CPY_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Output 2 Copy
    #[inline(always)]
    pub fn o2cpy(&self) -> O2CPY_R {
        O2CPY_R::new(((self.bits >> 21) & 1) != 0)
    }
}
///Timerx Interrupt Status Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timaisr](index.html) module
pub struct TIMAISR_SPEC;
impl crate::RegisterSpec for TIMAISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [timaisr::R](R) reader structure
impl crate::Readable for TIMAISR_SPEC {
    type Reader = R;
}
///`reset()` method sets TIMAISR to value 0
impl crate::Resettable for TIMAISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
