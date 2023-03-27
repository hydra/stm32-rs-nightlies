///Register `MCR` reader
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MCR` writer
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CKPSC` reader - HRTIM Master Clock prescaler
pub type CKPSC_R = crate::FieldReader<u8, u8>;
///Field `CKPSC` writer - HRTIM Master Clock prescaler
pub type CKPSC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MCR_SPEC, u8, u8, 3, O>;
///Field `CONT` reader - Master Continuous mode
pub type CONT_R = crate::BitReader<CONT_A>;
///Master Continuous mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONT_A {
    ///0: The timer operates in single-shot mode and stops when it reaches the MPER value
    SingleShot = 0,
    ///1: The timer operates in continuous (free-running) mode and rolls over to zero when it reaches the MPER value
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
            false => CONT_A::SingleShot,
            true => CONT_A::Continuous,
        }
    }
    ///Checks if the value of the field is `SingleShot`
    #[inline(always)]
    pub fn is_single_shot(&self) -> bool {
        *self == CONT_A::SingleShot
    }
    ///Checks if the value of the field is `Continuous`
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CONT_A::Continuous
    }
}
///Field `CONT` writer - Master Continuous mode
pub type CONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, CONT_A, O>;
impl<'a, const O: u8> CONT_W<'a, O> {
    ///The timer operates in single-shot mode and stops when it reaches the MPER value
    #[inline(always)]
    pub fn single_shot(self) -> &'a mut W {
        self.variant(CONT_A::SingleShot)
    }
    ///The timer operates in continuous (free-running) mode and rolls over to zero when it reaches the MPER value
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CONT_A::Continuous)
    }
}
///Field `RETRIG` reader - Master Re-triggerable mode
pub type RETRIG_R = crate::BitReader<RETRIG_A>;
///Master Re-triggerable mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RETRIG_A {
    ///0: The timer is not re-triggerable: a counter reset can be done only if the counter is stopped
    Disabled = 0,
    ///1: The timer is retriggerable: a counter reset is done whatever the counter state
    Enabled = 1,
}
impl From<RETRIG_A> for bool {
    #[inline(always)]
    fn from(variant: RETRIG_A) -> Self {
        variant as u8 != 0
    }
}
impl RETRIG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RETRIG_A {
        match self.bits {
            false => RETRIG_A::Disabled,
            true => RETRIG_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RETRIG_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RETRIG_A::Enabled
    }
}
///Field `RETRIG` writer - Master Re-triggerable mode
pub type RETRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, RETRIG_A, O>;
impl<'a, const O: u8> RETRIG_W<'a, O> {
    ///The timer is not re-triggerable: a counter reset can be done only if the counter is stopped
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RETRIG_A::Disabled)
    }
    ///The timer is retriggerable: a counter reset is done whatever the counter state
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RETRIG_A::Enabled)
    }
}
///Field `HALF` reader - Half mode enable
pub type HALF_R = crate::BitReader<HALF_A>;
///Half mode enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HALF_A {
    ///0: Half mode disabled
    Disabled = 0,
    ///1: Half mode enabled
    Enabled = 1,
}
impl From<HALF_A> for bool {
    #[inline(always)]
    fn from(variant: HALF_A) -> Self {
        variant as u8 != 0
    }
}
impl HALF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HALF_A {
        match self.bits {
            false => HALF_A::Disabled,
            true => HALF_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HALF_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HALF_A::Enabled
    }
}
///Field `HALF` writer - Half mode enable
pub type HALF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, HALF_A, O>;
impl<'a, const O: u8> HALF_W<'a, O> {
    ///Half mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HALF_A::Disabled)
    }
    ///Half mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HALF_A::Enabled)
    }
}
///Field `SYNCIN` reader - ynchronization input
pub type SYNCIN_R = crate::FieldReader<u8, SYNCIN_A>;
///ynchronization input
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCIN_A {
    ///0: Disabled. HRTIM is not synchronized and runs in standalone mode
    Disabled = 0,
    ///2: Internal event: the HRTIM is synchronized with the on-chip timer
    Internal = 2,
    ///3: External event: a positive pulse on HRTIM_SCIN input triggers the HRTIM
    External = 3,
}
impl From<SYNCIN_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCIN_A) -> Self {
        variant as _
    }
}
impl SYNCIN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SYNCIN_A> {
        match self.bits {
            0 => Some(SYNCIN_A::Disabled),
            2 => Some(SYNCIN_A::Internal),
            3 => Some(SYNCIN_A::External),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCIN_A::Disabled
    }
    ///Checks if the value of the field is `Internal`
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == SYNCIN_A::Internal
    }
    ///Checks if the value of the field is `External`
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == SYNCIN_A::External
    }
}
///Field `SYNCIN` writer - ynchronization input
pub type SYNCIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR_SPEC, u8, SYNCIN_A, 2, O>;
impl<'a, const O: u8> SYNCIN_W<'a, O> {
    ///Disabled. HRTIM is not synchronized and runs in standalone mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYNCIN_A::Disabled)
    }
    ///Internal event: the HRTIM is synchronized with the on-chip timer
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(SYNCIN_A::Internal)
    }
    ///External event: a positive pulse on HRTIM_SCIN input triggers the HRTIM
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(SYNCIN_A::External)
    }
}
///Field `SYNCRSTM` reader - Synchronization Resets Master
pub type SYNCRSTM_R = crate::BitReader<SYNCRSTM_A>;
///Synchronization Resets Master
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCRSTM_A {
    ///0: No effect on the master timer
    Disabled = 0,
    ///1: A synchroniation input event resets the master timer
    Enabled = 1,
}
impl From<SYNCRSTM_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCRSTM_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNCRSTM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SYNCRSTM_A {
        match self.bits {
            false => SYNCRSTM_A::Disabled,
            true => SYNCRSTM_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCRSTM_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYNCRSTM_A::Enabled
    }
}
///Field `SYNCRSTM` writer - Synchronization Resets Master
pub type SYNCRSTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, SYNCRSTM_A, O>;
impl<'a, const O: u8> SYNCRSTM_W<'a, O> {
    ///No effect on the master timer
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYNCRSTM_A::Disabled)
    }
    ///A synchroniation input event resets the master timer
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYNCRSTM_A::Enabled)
    }
}
///Field `SYNCSTRTM` reader - Synchronization Starts Master
pub type SYNCSTRTM_R = crate::BitReader<SYNCSTRTM_A>;
///Synchronization Starts Master
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCSTRTM_A {
    ///0: No effect on the master timer
    Disabled = 0,
    ///1: A synchroniation input event starts the master timer
    Enabled = 1,
}
impl From<SYNCSTRTM_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCSTRTM_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNCSTRTM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SYNCSTRTM_A {
        match self.bits {
            false => SYNCSTRTM_A::Disabled,
            true => SYNCSTRTM_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCSTRTM_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYNCSTRTM_A::Enabled
    }
}
///Field `SYNCSTRTM` writer - Synchronization Starts Master
pub type SYNCSTRTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, SYNCSTRTM_A, O>;
impl<'a, const O: u8> SYNCSTRTM_W<'a, O> {
    ///No effect on the master timer
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYNCSTRTM_A::Disabled)
    }
    ///A synchroniation input event starts the master timer
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYNCSTRTM_A::Enabled)
    }
}
///Field `SYNCOUT` reader - Synchronization output
pub type SYNCOUT_R = crate::FieldReader<u8, SYNCOUT_A>;
///Synchronization output
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCOUT_A {
    ///0: Disabled
    Disabled = 0,
    ///2: Positive pulse on SCOUT output (16x f_HRTIM clock cycles)
    PositivePulse = 2,
    ///3: Negative pulse on SCOUT output (16x f_HRTIM clock cycles)
    NegativePulse = 3,
}
impl From<SYNCOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCOUT_A) -> Self {
        variant as _
    }
}
impl SYNCOUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SYNCOUT_A> {
        match self.bits {
            0 => Some(SYNCOUT_A::Disabled),
            2 => Some(SYNCOUT_A::PositivePulse),
            3 => Some(SYNCOUT_A::NegativePulse),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYNCOUT_A::Disabled
    }
    ///Checks if the value of the field is `PositivePulse`
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        *self == SYNCOUT_A::PositivePulse
    }
    ///Checks if the value of the field is `NegativePulse`
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        *self == SYNCOUT_A::NegativePulse
    }
}
///Field `SYNCOUT` writer - Synchronization output
pub type SYNCOUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR_SPEC, u8, SYNCOUT_A, 2, O>;
impl<'a, const O: u8> SYNCOUT_W<'a, O> {
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYNCOUT_A::Disabled)
    }
    ///Positive pulse on SCOUT output (16x f_HRTIM clock cycles)
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(SYNCOUT_A::PositivePulse)
    }
    ///Negative pulse on SCOUT output (16x f_HRTIM clock cycles)
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(SYNCOUT_A::NegativePulse)
    }
}
///Field `SYNCSRC` reader - Synchronization source
pub type SYNCSRC_R = crate::FieldReader<u8, SYNCSRC_A>;
///Synchronization source
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCSRC_A {
    ///0: Master timer Start
    MasterStart = 0,
    ///1: Master timer Compare 1 event
    MasterCompare1 = 1,
    ///2: Timer A start/reset
    TimerAstart = 2,
    ///3: Timer A Compare 1 event
    TimerAcompare1 = 3,
}
impl From<SYNCSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCSRC_A) -> Self {
        variant as _
    }
}
impl SYNCSRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SYNCSRC_A {
        match self.bits {
            0 => SYNCSRC_A::MasterStart,
            1 => SYNCSRC_A::MasterCompare1,
            2 => SYNCSRC_A::TimerAstart,
            3 => SYNCSRC_A::TimerAcompare1,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `MasterStart`
    #[inline(always)]
    pub fn is_master_start(&self) -> bool {
        *self == SYNCSRC_A::MasterStart
    }
    ///Checks if the value of the field is `MasterCompare1`
    #[inline(always)]
    pub fn is_master_compare1(&self) -> bool {
        *self == SYNCSRC_A::MasterCompare1
    }
    ///Checks if the value of the field is `TimerAstart`
    #[inline(always)]
    pub fn is_timer_astart(&self) -> bool {
        *self == SYNCSRC_A::TimerAstart
    }
    ///Checks if the value of the field is `TimerAcompare1`
    #[inline(always)]
    pub fn is_timer_acompare1(&self) -> bool {
        *self == SYNCSRC_A::TimerAcompare1
    }
}
///Field `SYNCSRC` writer - Synchronization source
pub type SYNCSRC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MCR_SPEC, u8, SYNCSRC_A, 2, O>;
impl<'a, const O: u8> SYNCSRC_W<'a, O> {
    ///Master timer Start
    #[inline(always)]
    pub fn master_start(self) -> &'a mut W {
        self.variant(SYNCSRC_A::MasterStart)
    }
    ///Master timer Compare 1 event
    #[inline(always)]
    pub fn master_compare1(self) -> &'a mut W {
        self.variant(SYNCSRC_A::MasterCompare1)
    }
    ///Timer A start/reset
    #[inline(always)]
    pub fn timer_astart(self) -> &'a mut W {
        self.variant(SYNCSRC_A::TimerAstart)
    }
    ///Timer A Compare 1 event
    #[inline(always)]
    pub fn timer_acompare1(self) -> &'a mut W {
        self.variant(SYNCSRC_A::TimerAcompare1)
    }
}
///Field `MCEN` reader - Master Counter enable
pub type MCEN_R = crate::BitReader<MCEN_A>;
///Master Counter enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCEN_A {
    ///0: Master timer counter disabled
    Disabled = 0,
    ///1: Master timer counter enabled
    Enabled = 1,
}
impl From<MCEN_A> for bool {
    #[inline(always)]
    fn from(variant: MCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MCEN_A {
        match self.bits {
            false => MCEN_A::Disabled,
            true => MCEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCEN_A::Enabled
    }
}
///Field `MCEN` writer - Master Counter enable
pub type MCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MCEN_A, O>;
impl<'a, const O: u8> MCEN_W<'a, O> {
    ///Master timer counter disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MCEN_A::Disabled)
    }
    ///Master timer counter enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MCEN_A::Enabled)
    }
}
///Field `TACEN` reader - Timer A counter enable
pub type TACEN_R = crate::BitReader<TACEN_A>;
///Timer A counter enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TACEN_A {
    ///0: Timer counter disabled
    Disabled = 0,
    ///1: Timer counter enabled
    Enabled = 1,
}
impl From<TACEN_A> for bool {
    #[inline(always)]
    fn from(variant: TACEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TACEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TACEN_A {
        match self.bits {
            false => TACEN_A::Disabled,
            true => TACEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TACEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TACEN_A::Enabled
    }
}
///Field `TACEN` writer - Timer A counter enable
pub type TACEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, TACEN_A, O>;
impl<'a, const O: u8> TACEN_W<'a, O> {
    ///Timer counter disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TACEN_A::Disabled)
    }
    ///Timer counter enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TACEN_A::Enabled)
    }
}
///Field `TBCEN` reader - Timer B counter enable
pub use TACEN_R as TBCEN_R;
///Field `TCCEN` reader - Timer C counter enable
pub use TACEN_R as TCCEN_R;
///Field `TDCEN` reader - Timer D counter enable
pub use TACEN_R as TDCEN_R;
///Field `TECEN` reader - Timer E counter enable
pub use TACEN_R as TECEN_R;
///Field `TBCEN` writer - Timer B counter enable
pub use TACEN_W as TBCEN_W;
///Field `TCCEN` writer - Timer C counter enable
pub use TACEN_W as TCCEN_W;
///Field `TDCEN` writer - Timer D counter enable
pub use TACEN_W as TDCEN_W;
///Field `TECEN` writer - Timer E counter enable
pub use TACEN_W as TECEN_W;
///Field `DACSYNC` reader - AC Synchronization
pub type DACSYNC_R = crate::FieldReader<u8, DACSYNC_A>;
///AC Synchronization
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DACSYNC_A {
    ///0: No DAC trigger generated
    Disabled = 0,
    ///1: Trigger generated on DACSync1
    Dacsync1 = 1,
    ///2: Trigger generated on DACSync2
    Dacsync2 = 2,
    ///3: Trigger generated on DACSync3
    Dacsync3 = 3,
}
impl From<DACSYNC_A> for u8 {
    #[inline(always)]
    fn from(variant: DACSYNC_A) -> Self {
        variant as _
    }
}
impl DACSYNC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DACSYNC_A {
        match self.bits {
            0 => DACSYNC_A::Disabled,
            1 => DACSYNC_A::Dacsync1,
            2 => DACSYNC_A::Dacsync2,
            3 => DACSYNC_A::Dacsync3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DACSYNC_A::Disabled
    }
    ///Checks if the value of the field is `Dacsync1`
    #[inline(always)]
    pub fn is_dacsync1(&self) -> bool {
        *self == DACSYNC_A::Dacsync1
    }
    ///Checks if the value of the field is `Dacsync2`
    #[inline(always)]
    pub fn is_dacsync2(&self) -> bool {
        *self == DACSYNC_A::Dacsync2
    }
    ///Checks if the value of the field is `Dacsync3`
    #[inline(always)]
    pub fn is_dacsync3(&self) -> bool {
        *self == DACSYNC_A::Dacsync3
    }
}
///Field `DACSYNC` writer - AC Synchronization
pub type DACSYNC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MCR_SPEC, u8, DACSYNC_A, 2, O>;
impl<'a, const O: u8> DACSYNC_W<'a, O> {
    ///No DAC trigger generated
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DACSYNC_A::Disabled)
    }
    ///Trigger generated on DACSync1
    #[inline(always)]
    pub fn dacsync1(self) -> &'a mut W {
        self.variant(DACSYNC_A::Dacsync1)
    }
    ///Trigger generated on DACSync2
    #[inline(always)]
    pub fn dacsync2(self) -> &'a mut W {
        self.variant(DACSYNC_A::Dacsync2)
    }
    ///Trigger generated on DACSync3
    #[inline(always)]
    pub fn dacsync3(self) -> &'a mut W {
        self.variant(DACSYNC_A::Dacsync3)
    }
}
///Field `PREEN` reader - Preload enable
pub type PREEN_R = crate::BitReader<PREEN_A>;
///Preload enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PREEN_A {
    ///0: Preload disabled: the write access is directly done into the active register
    Disabled = 0,
    ///1: Preload enabled: the write access is done into the preload register
    Enabled = 1,
}
impl From<PREEN_A> for bool {
    #[inline(always)]
    fn from(variant: PREEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PREEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PREEN_A {
        match self.bits {
            false => PREEN_A::Disabled,
            true => PREEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PREEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PREEN_A::Enabled
    }
}
///Field `PREEN` writer - Preload enable
pub type PREEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, PREEN_A, O>;
impl<'a, const O: u8> PREEN_W<'a, O> {
    ///Preload disabled: the write access is directly done into the active register
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PREEN_A::Disabled)
    }
    ///Preload enabled: the write access is done into the preload register
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PREEN_A::Enabled)
    }
}
///Field `MREPU` reader - Master Timer Repetition update
pub type MREPU_R = crate::BitReader<MREPU_A>;
///Master Timer Repetition update
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MREPU_A {
    ///0: Update on repetition disabled
    Disabled = 0,
    ///1: Update on repetition enabled
    Enabled = 1,
}
impl From<MREPU_A> for bool {
    #[inline(always)]
    fn from(variant: MREPU_A) -> Self {
        variant as u8 != 0
    }
}
impl MREPU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MREPU_A {
        match self.bits {
            false => MREPU_A::Disabled,
            true => MREPU_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MREPU_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MREPU_A::Enabled
    }
}
///Field `MREPU` writer - Master Timer Repetition update
pub type MREPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MREPU_A, O>;
impl<'a, const O: u8> MREPU_W<'a, O> {
    ///Update on repetition disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MREPU_A::Disabled)
    }
    ///Update on repetition enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MREPU_A::Enabled)
    }
}
///Field `BRSTDMA` reader - Burst DMA Update
pub type BRSTDMA_R = crate::FieldReader<u8, BRSTDMA_A>;
///Burst DMA Update
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BRSTDMA_A {
    ///0: Update done independently from the DMA burst transfer completion
    Independent = 0,
    ///1: Update done when the DMA burst transfer is completed
    Completion = 1,
    ///2: Update done on master timer roll-over following a DMA burst transfer completion
    Rollover = 2,
}
impl From<BRSTDMA_A> for u8 {
    #[inline(always)]
    fn from(variant: BRSTDMA_A) -> Self {
        variant as _
    }
}
impl BRSTDMA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<BRSTDMA_A> {
        match self.bits {
            0 => Some(BRSTDMA_A::Independent),
            1 => Some(BRSTDMA_A::Completion),
            2 => Some(BRSTDMA_A::Rollover),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Independent`
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == BRSTDMA_A::Independent
    }
    ///Checks if the value of the field is `Completion`
    #[inline(always)]
    pub fn is_completion(&self) -> bool {
        *self == BRSTDMA_A::Completion
    }
    ///Checks if the value of the field is `Rollover`
    #[inline(always)]
    pub fn is_rollover(&self) -> bool {
        *self == BRSTDMA_A::Rollover
    }
}
///Field `BRSTDMA` writer - Burst DMA Update
pub type BRSTDMA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR_SPEC, u8, BRSTDMA_A, 2, O>;
impl<'a, const O: u8> BRSTDMA_W<'a, O> {
    ///Update done independently from the DMA burst transfer completion
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(BRSTDMA_A::Independent)
    }
    ///Update done when the DMA burst transfer is completed
    #[inline(always)]
    pub fn completion(self) -> &'a mut W {
        self.variant(BRSTDMA_A::Completion)
    }
    ///Update done on master timer roll-over following a DMA burst transfer completion
    #[inline(always)]
    pub fn rollover(self) -> &'a mut W {
        self.variant(BRSTDMA_A::Rollover)
    }
}
impl R {
    ///Bits 0:2 - HRTIM Master Clock prescaler
    #[inline(always)]
    pub fn ckpsc(&self) -> CKPSC_R {
        CKPSC_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Master Continuous mode
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Master Re-triggerable mode
    #[inline(always)]
    pub fn retrig(&self) -> RETRIG_R {
        RETRIG_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Half mode enable
    #[inline(always)]
    pub fn half(&self) -> HALF_R {
        HALF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:9 - ynchronization input
    #[inline(always)]
    pub fn syncin(&self) -> SYNCIN_R {
        SYNCIN_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Synchronization Resets Master
    #[inline(always)]
    pub fn syncrstm(&self) -> SYNCRSTM_R {
        SYNCRSTM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Synchronization Starts Master
    #[inline(always)]
    pub fn syncstrtm(&self) -> SYNCSTRTM_R {
        SYNCSTRTM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Synchronization output
    #[inline(always)]
    pub fn syncout(&self) -> SYNCOUT_R {
        SYNCOUT_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Synchronization source
    #[inline(always)]
    pub fn syncsrc(&self) -> SYNCSRC_R {
        SYNCSRC_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - Master Counter enable
    #[inline(always)]
    pub fn mcen(&self) -> MCEN_R {
        MCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Timer A counter enable
    #[inline(always)]
    pub fn tacen(&self) -> TACEN_R {
        TACEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Timer B counter enable
    #[inline(always)]
    pub fn tbcen(&self) -> TBCEN_R {
        TBCEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Timer C counter enable
    #[inline(always)]
    pub fn tccen(&self) -> TCCEN_R {
        TCCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Timer D counter enable
    #[inline(always)]
    pub fn tdcen(&self) -> TDCEN_R {
        TDCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Timer E counter enable
    #[inline(always)]
    pub fn tecen(&self) -> TECEN_R {
        TECEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 25:26 - AC Synchronization
    #[inline(always)]
    pub fn dacsync(&self) -> DACSYNC_R {
        DACSYNC_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bit 27 - Preload enable
    #[inline(always)]
    pub fn preen(&self) -> PREEN_R {
        PREEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 29 - Master Timer Repetition update
    #[inline(always)]
    pub fn mrepu(&self) -> MREPU_R {
        MREPU_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bits 30:31 - Burst DMA Update
    #[inline(always)]
    pub fn brstdma(&self) -> BRSTDMA_R {
        BRSTDMA_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    ///Bits 0:2 - HRTIM Master Clock prescaler
    #[inline(always)]
    #[must_use]
    pub fn ckpsc(&mut self) -> CKPSC_W<0> {
        CKPSC_W::new(self)
    }
    ///Bit 3 - Master Continuous mode
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<3> {
        CONT_W::new(self)
    }
    ///Bit 4 - Master Re-triggerable mode
    #[inline(always)]
    #[must_use]
    pub fn retrig(&mut self) -> RETRIG_W<4> {
        RETRIG_W::new(self)
    }
    ///Bit 5 - Half mode enable
    #[inline(always)]
    #[must_use]
    pub fn half(&mut self) -> HALF_W<5> {
        HALF_W::new(self)
    }
    ///Bits 8:9 - ynchronization input
    #[inline(always)]
    #[must_use]
    pub fn syncin(&mut self) -> SYNCIN_W<8> {
        SYNCIN_W::new(self)
    }
    ///Bit 10 - Synchronization Resets Master
    #[inline(always)]
    #[must_use]
    pub fn syncrstm(&mut self) -> SYNCRSTM_W<10> {
        SYNCRSTM_W::new(self)
    }
    ///Bit 11 - Synchronization Starts Master
    #[inline(always)]
    #[must_use]
    pub fn syncstrtm(&mut self) -> SYNCSTRTM_W<11> {
        SYNCSTRTM_W::new(self)
    }
    ///Bits 12:13 - Synchronization output
    #[inline(always)]
    #[must_use]
    pub fn syncout(&mut self) -> SYNCOUT_W<12> {
        SYNCOUT_W::new(self)
    }
    ///Bits 14:15 - Synchronization source
    #[inline(always)]
    #[must_use]
    pub fn syncsrc(&mut self) -> SYNCSRC_W<14> {
        SYNCSRC_W::new(self)
    }
    ///Bit 16 - Master Counter enable
    #[inline(always)]
    #[must_use]
    pub fn mcen(&mut self) -> MCEN_W<16> {
        MCEN_W::new(self)
    }
    ///Bit 17 - Timer A counter enable
    #[inline(always)]
    #[must_use]
    pub fn tacen(&mut self) -> TACEN_W<17> {
        TACEN_W::new(self)
    }
    ///Bit 18 - Timer B counter enable
    #[inline(always)]
    #[must_use]
    pub fn tbcen(&mut self) -> TBCEN_W<18> {
        TBCEN_W::new(self)
    }
    ///Bit 19 - Timer C counter enable
    #[inline(always)]
    #[must_use]
    pub fn tccen(&mut self) -> TCCEN_W<19> {
        TCCEN_W::new(self)
    }
    ///Bit 20 - Timer D counter enable
    #[inline(always)]
    #[must_use]
    pub fn tdcen(&mut self) -> TDCEN_W<20> {
        TDCEN_W::new(self)
    }
    ///Bit 21 - Timer E counter enable
    #[inline(always)]
    #[must_use]
    pub fn tecen(&mut self) -> TECEN_W<21> {
        TECEN_W::new(self)
    }
    ///Bits 25:26 - AC Synchronization
    #[inline(always)]
    #[must_use]
    pub fn dacsync(&mut self) -> DACSYNC_W<25> {
        DACSYNC_W::new(self)
    }
    ///Bit 27 - Preload enable
    #[inline(always)]
    #[must_use]
    pub fn preen(&mut self) -> PREEN_W<27> {
        PREEN_W::new(self)
    }
    ///Bit 29 - Master Timer Repetition update
    #[inline(always)]
    #[must_use]
    pub fn mrepu(&mut self) -> MREPU_W<29> {
        MREPU_W::new(self)
    }
    ///Bits 30:31 - Burst DMA Update
    #[inline(always)]
    #[must_use]
    pub fn brstdma(&mut self) -> BRSTDMA_W<30> {
        BRSTDMA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Master Timer Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mcr](index.html) module
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mcr::R](R) reader structure
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mcr::W](W) writer structure
impl crate::Writable for MCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MCR to value 0
impl crate::Resettable for MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
