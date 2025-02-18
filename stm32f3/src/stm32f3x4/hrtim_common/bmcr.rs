///Register `BMCR` reader
pub struct R(crate::R<BMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BMCR` writer
pub struct W(crate::W<BMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMCR_SPEC>;
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
impl From<crate::W<BMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BME` reader - Burst Mode enable
pub type BME_R = crate::BitReader<BME_A>;
///Burst Mode enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BME_A {
    ///0: Burst mode disabled
    Disabled = 0,
    ///1: Burst mode enabled
    Enabled = 1,
}
impl From<BME_A> for bool {
    #[inline(always)]
    fn from(variant: BME_A) -> Self {
        variant as u8 != 0
    }
}
impl BME_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BME_A {
        match self.bits {
            false => BME_A::Disabled,
            true => BME_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BME_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BME_A::Enabled
    }
}
///Field `BME` writer - Burst Mode enable
pub type BME_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMCR_SPEC, BME_A, O>;
impl<'a, const O: u8> BME_W<'a, O> {
    ///Burst mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BME_A::Disabled)
    }
    ///Burst mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BME_A::Enabled)
    }
}
///Field `BMOM` reader - Burst Mode operating mode
pub type BMOM_R = crate::BitReader<BMOM_A>;
///Burst Mode operating mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BMOM_A {
    ///0: Single-shot mode
    SingleShot = 0,
    ///1: Continuous operation
    Continuous = 1,
}
impl From<BMOM_A> for bool {
    #[inline(always)]
    fn from(variant: BMOM_A) -> Self {
        variant as u8 != 0
    }
}
impl BMOM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BMOM_A {
        match self.bits {
            false => BMOM_A::SingleShot,
            true => BMOM_A::Continuous,
        }
    }
    ///Checks if the value of the field is `SingleShot`
    #[inline(always)]
    pub fn is_single_shot(&self) -> bool {
        *self == BMOM_A::SingleShot
    }
    ///Checks if the value of the field is `Continuous`
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == BMOM_A::Continuous
    }
}
///Field `BMOM` writer - Burst Mode operating mode
pub type BMOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMCR_SPEC, BMOM_A, O>;
impl<'a, const O: u8> BMOM_W<'a, O> {
    ///Single-shot mode
    #[inline(always)]
    pub fn single_shot(self) -> &'a mut W {
        self.variant(BMOM_A::SingleShot)
    }
    ///Continuous operation
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(BMOM_A::Continuous)
    }
}
///Field `BMCLK` reader - Burst Mode Clock source
pub type BMCLK_R = crate::FieldReader<u8, BMCLK_A>;
///Burst Mode Clock source
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BMCLK_A {
    ///0: Master timer reset/roll-over
    Master = 0,
    ///1: Timer A counter reset/roll-over
    TimerA = 1,
    ///2: Timer B counter reset/roll-over
    TimerB = 2,
    ///3: Timer C counter reset/roll-over
    TimerC = 3,
    ///4: Timer D counter reset/roll-over
    TimerD = 4,
    ///5: Timer E counter reset/roll-over
    TimerE = 5,
    ///6: On-chip Event 1 (BMClk\[1\]), acting as a burst mode counter clock
    Event1 = 6,
    ///7: On-chip Event 2 (BMClk\[2\]), acting as a burst mode counter clock
    Event2 = 7,
    ///8: On-chip Event 3 (BMClk\[3\]), acting as a burst mode counter clock
    Event3 = 8,
    ///9: On-chip Event 4 (BMClk\[4\]), acting as a burst mode counter clock
    Event4 = 9,
    ///10: Prescaled f_HRTIM clock (as per BMPRSC\[3:0\]
    ///setting
    Clock = 10,
}
impl From<BMCLK_A> for u8 {
    #[inline(always)]
    fn from(variant: BMCLK_A) -> Self {
        variant as _
    }
}
impl BMCLK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<BMCLK_A> {
        match self.bits {
            0 => Some(BMCLK_A::Master),
            1 => Some(BMCLK_A::TimerA),
            2 => Some(BMCLK_A::TimerB),
            3 => Some(BMCLK_A::TimerC),
            4 => Some(BMCLK_A::TimerD),
            5 => Some(BMCLK_A::TimerE),
            6 => Some(BMCLK_A::Event1),
            7 => Some(BMCLK_A::Event2),
            8 => Some(BMCLK_A::Event3),
            9 => Some(BMCLK_A::Event4),
            10 => Some(BMCLK_A::Clock),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Master`
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == BMCLK_A::Master
    }
    ///Checks if the value of the field is `TimerA`
    #[inline(always)]
    pub fn is_timer_a(&self) -> bool {
        *self == BMCLK_A::TimerA
    }
    ///Checks if the value of the field is `TimerB`
    #[inline(always)]
    pub fn is_timer_b(&self) -> bool {
        *self == BMCLK_A::TimerB
    }
    ///Checks if the value of the field is `TimerC`
    #[inline(always)]
    pub fn is_timer_c(&self) -> bool {
        *self == BMCLK_A::TimerC
    }
    ///Checks if the value of the field is `TimerD`
    #[inline(always)]
    pub fn is_timer_d(&self) -> bool {
        *self == BMCLK_A::TimerD
    }
    ///Checks if the value of the field is `TimerE`
    #[inline(always)]
    pub fn is_timer_e(&self) -> bool {
        *self == BMCLK_A::TimerE
    }
    ///Checks if the value of the field is `Event1`
    #[inline(always)]
    pub fn is_event1(&self) -> bool {
        *self == BMCLK_A::Event1
    }
    ///Checks if the value of the field is `Event2`
    #[inline(always)]
    pub fn is_event2(&self) -> bool {
        *self == BMCLK_A::Event2
    }
    ///Checks if the value of the field is `Event3`
    #[inline(always)]
    pub fn is_event3(&self) -> bool {
        *self == BMCLK_A::Event3
    }
    ///Checks if the value of the field is `Event4`
    #[inline(always)]
    pub fn is_event4(&self) -> bool {
        *self == BMCLK_A::Event4
    }
    ///Checks if the value of the field is `Clock`
    #[inline(always)]
    pub fn is_clock(&self) -> bool {
        *self == BMCLK_A::Clock
    }
}
///Field `BMCLK` writer - Burst Mode Clock source
pub type BMCLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMCR_SPEC, u8, BMCLK_A, 4, O>;
impl<'a, const O: u8> BMCLK_W<'a, O> {
    ///Master timer reset/roll-over
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(BMCLK_A::Master)
    }
    ///Timer A counter reset/roll-over
    #[inline(always)]
    pub fn timer_a(self) -> &'a mut W {
        self.variant(BMCLK_A::TimerA)
    }
    ///Timer B counter reset/roll-over
    #[inline(always)]
    pub fn timer_b(self) -> &'a mut W {
        self.variant(BMCLK_A::TimerB)
    }
    ///Timer C counter reset/roll-over
    #[inline(always)]
    pub fn timer_c(self) -> &'a mut W {
        self.variant(BMCLK_A::TimerC)
    }
    ///Timer D counter reset/roll-over
    #[inline(always)]
    pub fn timer_d(self) -> &'a mut W {
        self.variant(BMCLK_A::TimerD)
    }
    ///Timer E counter reset/roll-over
    #[inline(always)]
    pub fn timer_e(self) -> &'a mut W {
        self.variant(BMCLK_A::TimerE)
    }
    ///On-chip Event 1 (BMClk\[1\]), acting as a burst mode counter clock
    #[inline(always)]
    pub fn event1(self) -> &'a mut W {
        self.variant(BMCLK_A::Event1)
    }
    ///On-chip Event 2 (BMClk\[2\]), acting as a burst mode counter clock
    #[inline(always)]
    pub fn event2(self) -> &'a mut W {
        self.variant(BMCLK_A::Event2)
    }
    ///On-chip Event 3 (BMClk\[3\]), acting as a burst mode counter clock
    #[inline(always)]
    pub fn event3(self) -> &'a mut W {
        self.variant(BMCLK_A::Event3)
    }
    ///On-chip Event 4 (BMClk\[4\]), acting as a burst mode counter clock
    #[inline(always)]
    pub fn event4(self) -> &'a mut W {
        self.variant(BMCLK_A::Event4)
    }
    ///Prescaled f_HRTIM clock (as per BMPRSC\[3:0\]
    ///setting
    #[inline(always)]
    pub fn clock(self) -> &'a mut W {
        self.variant(BMCLK_A::Clock)
    }
}
///Field `BMPRSC` reader - Burst Mode Prescaler
pub type BMPRSC_R = crate::FieldReader<u8, BMPRSC_A>;
///Burst Mode Prescaler
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BMPRSC_A {
    ///0: Clock not divided
    Div1 = 0,
    ///1: Division by 2
    Div2 = 1,
    ///2: Division by 4
    Div4 = 2,
    ///3: Division by 8
    Div8 = 3,
    ///4: Division by 16
    Div16 = 4,
    ///5: Division by 32
    Div32 = 5,
    ///6: Division by 64
    Div64 = 6,
    ///7: Division by 128
    Div128 = 7,
    ///8: Division by 256
    Div256 = 8,
    ///9: Division by 512
    Div512 = 9,
    ///10: Division by 1024
    Div1024 = 10,
    ///11: Division by 2048
    Div2048 = 11,
    ///12: Division by 4096
    Div4096 = 12,
    ///13: Division by 8192
    Div8192 = 13,
    ///14: Division by 16384
    Div16384 = 14,
    ///15: Division by 32768
    Div32768 = 15,
}
impl From<BMPRSC_A> for u8 {
    #[inline(always)]
    fn from(variant: BMPRSC_A) -> Self {
        variant as _
    }
}
impl BMPRSC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BMPRSC_A {
        match self.bits {
            0 => BMPRSC_A::Div1,
            1 => BMPRSC_A::Div2,
            2 => BMPRSC_A::Div4,
            3 => BMPRSC_A::Div8,
            4 => BMPRSC_A::Div16,
            5 => BMPRSC_A::Div32,
            6 => BMPRSC_A::Div64,
            7 => BMPRSC_A::Div128,
            8 => BMPRSC_A::Div256,
            9 => BMPRSC_A::Div512,
            10 => BMPRSC_A::Div1024,
            11 => BMPRSC_A::Div2048,
            12 => BMPRSC_A::Div4096,
            13 => BMPRSC_A::Div8192,
            14 => BMPRSC_A::Div16384,
            15 => BMPRSC_A::Div32768,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == BMPRSC_A::Div1
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == BMPRSC_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == BMPRSC_A::Div4
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == BMPRSC_A::Div8
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == BMPRSC_A::Div16
    }
    ///Checks if the value of the field is `Div32`
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == BMPRSC_A::Div32
    }
    ///Checks if the value of the field is `Div64`
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == BMPRSC_A::Div64
    }
    ///Checks if the value of the field is `Div128`
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == BMPRSC_A::Div128
    }
    ///Checks if the value of the field is `Div256`
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == BMPRSC_A::Div256
    }
    ///Checks if the value of the field is `Div512`
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == BMPRSC_A::Div512
    }
    ///Checks if the value of the field is `Div1024`
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == BMPRSC_A::Div1024
    }
    ///Checks if the value of the field is `Div2048`
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == BMPRSC_A::Div2048
    }
    ///Checks if the value of the field is `Div4096`
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == BMPRSC_A::Div4096
    }
    ///Checks if the value of the field is `Div8192`
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == BMPRSC_A::Div8192
    }
    ///Checks if the value of the field is `Div16384`
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == BMPRSC_A::Div16384
    }
    ///Checks if the value of the field is `Div32768`
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == BMPRSC_A::Div32768
    }
}
///Field `BMPRSC` writer - Burst Mode Prescaler
pub type BMPRSC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BMCR_SPEC, u8, BMPRSC_A, 4, O>;
impl<'a, const O: u8> BMPRSC_W<'a, O> {
    ///Clock not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(BMPRSC_A::Div1)
    }
    ///Division by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(BMPRSC_A::Div2)
    }
    ///Division by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(BMPRSC_A::Div4)
    }
    ///Division by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(BMPRSC_A::Div8)
    }
    ///Division by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(BMPRSC_A::Div16)
    }
    ///Division by 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(BMPRSC_A::Div32)
    }
    ///Division by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(BMPRSC_A::Div64)
    }
    ///Division by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(BMPRSC_A::Div128)
    }
    ///Division by 256
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(BMPRSC_A::Div256)
    }
    ///Division by 512
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(BMPRSC_A::Div512)
    }
    ///Division by 1024
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(BMPRSC_A::Div1024)
    }
    ///Division by 2048
    #[inline(always)]
    pub fn div2048(self) -> &'a mut W {
        self.variant(BMPRSC_A::Div2048)
    }
    ///Division by 4096
    #[inline(always)]
    pub fn div4096(self) -> &'a mut W {
        self.variant(BMPRSC_A::Div4096)
    }
    ///Division by 8192
    #[inline(always)]
    pub fn div8192(self) -> &'a mut W {
        self.variant(BMPRSC_A::Div8192)
    }
    ///Division by 16384
    #[inline(always)]
    pub fn div16384(self) -> &'a mut W {
        self.variant(BMPRSC_A::Div16384)
    }
    ///Division by 32768
    #[inline(always)]
    pub fn div32768(self) -> &'a mut W {
        self.variant(BMPRSC_A::Div32768)
    }
}
///Field `BMPREN` reader - Burst Mode Preload Enable
pub type BMPREN_R = crate::BitReader<BMPREN_A>;
///Burst Mode Preload Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BMPREN_A {
    ///0: Preload disabled: the write access is directly done into active registers
    Disabled = 0,
    ///1: Preload enabled: the write access is done into preload registers
    Enabled = 1,
}
impl From<BMPREN_A> for bool {
    #[inline(always)]
    fn from(variant: BMPREN_A) -> Self {
        variant as u8 != 0
    }
}
impl BMPREN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BMPREN_A {
        match self.bits {
            false => BMPREN_A::Disabled,
            true => BMPREN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BMPREN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BMPREN_A::Enabled
    }
}
///Field `BMPREN` writer - Burst Mode Preload Enable
pub type BMPREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMCR_SPEC, BMPREN_A, O>;
impl<'a, const O: u8> BMPREN_W<'a, O> {
    ///Preload disabled: the write access is directly done into active registers
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BMPREN_A::Disabled)
    }
    ///Preload enabled: the write access is done into preload registers
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BMPREN_A::Enabled)
    }
}
///Field `MTBM` reader - Master Timer Burst Mode
pub type MTBM_R = crate::BitReader<MTBM_A>;
///Master Timer Burst Mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTBM_A {
    ///0: Counter clock is maintained and timer operates normally
    Normal = 0,
    ///1: Counter clock is stopped and counter is reset
    Stopped = 1,
}
impl From<MTBM_A> for bool {
    #[inline(always)]
    fn from(variant: MTBM_A) -> Self {
        variant as u8 != 0
    }
}
impl MTBM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MTBM_A {
        match self.bits {
            false => MTBM_A::Normal,
            true => MTBM_A::Stopped,
        }
    }
    ///Checks if the value of the field is `Normal`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MTBM_A::Normal
    }
    ///Checks if the value of the field is `Stopped`
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == MTBM_A::Stopped
    }
}
///Field `MTBM` writer - Master Timer Burst Mode
pub type MTBM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMCR_SPEC, MTBM_A, O>;
impl<'a, const O: u8> MTBM_W<'a, O> {
    ///Counter clock is maintained and timer operates normally
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MTBM_A::Normal)
    }
    ///Counter clock is stopped and counter is reset
    #[inline(always)]
    pub fn stopped(self) -> &'a mut W {
        self.variant(MTBM_A::Stopped)
    }
}
///Field `TABM` reader - Timer A Burst Mode
pub use MTBM_R as TABM_R;
///Field `TBBM` reader - Timer B Burst Mode
pub use MTBM_R as TBBM_R;
///Field `TCBM` reader - Timer C Burst Mode
pub use MTBM_R as TCBM_R;
///Field `TDBM` reader - Timer D Burst Mode
pub use MTBM_R as TDBM_R;
///Field `TEBM` reader - Timer E Burst Mode
pub use MTBM_R as TEBM_R;
///Field `TABM` writer - Timer A Burst Mode
pub use MTBM_W as TABM_W;
///Field `TBBM` writer - Timer B Burst Mode
pub use MTBM_W as TBBM_W;
///Field `TCBM` writer - Timer C Burst Mode
pub use MTBM_W as TCBM_W;
///Field `TDBM` writer - Timer D Burst Mode
pub use MTBM_W as TDBM_W;
///Field `TEBM` writer - Timer E Burst Mode
pub use MTBM_W as TEBM_W;
///Field `BMSTAT` reader - Burst Mode Status
pub type BMSTAT_R = crate::BitReader<BMSTATR_A>;
///Burst Mode Status
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BMSTATR_A {
    ///0: Normal operation
    Normal = 0,
    ///1: Burst operation ongoing
    Burst = 1,
}
impl From<BMSTATR_A> for bool {
    #[inline(always)]
    fn from(variant: BMSTATR_A) -> Self {
        variant as u8 != 0
    }
}
impl BMSTAT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BMSTATR_A {
        match self.bits {
            false => BMSTATR_A::Normal,
            true => BMSTATR_A::Burst,
        }
    }
    ///Checks if the value of the field is `Normal`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == BMSTATR_A::Normal
    }
    ///Checks if the value of the field is `Burst`
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == BMSTATR_A::Burst
    }
}
///Burst Mode Status
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BMSTATW_AW {
    ///0: Terminate burst mode
    Cancel = 0,
}
impl From<BMSTATW_AW> for bool {
    #[inline(always)]
    fn from(variant: BMSTATW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `BMSTAT` writer - Burst Mode Status
pub type BMSTAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMCR_SPEC, BMSTATW_AW, O>;
impl<'a, const O: u8> BMSTAT_W<'a, O> {
    ///Terminate burst mode
    #[inline(always)]
    pub fn cancel(self) -> &'a mut W {
        self.variant(BMSTATW_AW::Cancel)
    }
}
impl R {
    ///Bit 0 - Burst Mode enable
    #[inline(always)]
    pub fn bme(&self) -> BME_R {
        BME_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Burst Mode operating mode
    #[inline(always)]
    pub fn bmom(&self) -> BMOM_R {
        BMOM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:5 - Burst Mode Clock source
    #[inline(always)]
    pub fn bmclk(&self) -> BMCLK_R {
        BMCLK_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bits 6:9 - Burst Mode Prescaler
    #[inline(always)]
    pub fn bmprsc(&self) -> BMPRSC_R {
        BMPRSC_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    ///Bit 10 - Burst Mode Preload Enable
    #[inline(always)]
    pub fn bmpren(&self) -> BMPREN_R {
        BMPREN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 16 - Master Timer Burst Mode
    #[inline(always)]
    pub fn mtbm(&self) -> MTBM_R {
        MTBM_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Timer A Burst Mode
    #[inline(always)]
    pub fn tabm(&self) -> TABM_R {
        TABM_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Timer B Burst Mode
    #[inline(always)]
    pub fn tbbm(&self) -> TBBM_R {
        TBBM_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Timer C Burst Mode
    #[inline(always)]
    pub fn tcbm(&self) -> TCBM_R {
        TCBM_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Timer D Burst Mode
    #[inline(always)]
    pub fn tdbm(&self) -> TDBM_R {
        TDBM_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Timer E Burst Mode
    #[inline(always)]
    pub fn tebm(&self) -> TEBM_R {
        TEBM_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 31 - Burst Mode Status
    #[inline(always)]
    pub fn bmstat(&self) -> BMSTAT_R {
        BMSTAT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Burst Mode enable
    #[inline(always)]
    #[must_use]
    pub fn bme(&mut self) -> BME_W<0> {
        BME_W::new(self)
    }
    ///Bit 1 - Burst Mode operating mode
    #[inline(always)]
    #[must_use]
    pub fn bmom(&mut self) -> BMOM_W<1> {
        BMOM_W::new(self)
    }
    ///Bits 2:5 - Burst Mode Clock source
    #[inline(always)]
    #[must_use]
    pub fn bmclk(&mut self) -> BMCLK_W<2> {
        BMCLK_W::new(self)
    }
    ///Bits 6:9 - Burst Mode Prescaler
    #[inline(always)]
    #[must_use]
    pub fn bmprsc(&mut self) -> BMPRSC_W<6> {
        BMPRSC_W::new(self)
    }
    ///Bit 10 - Burst Mode Preload Enable
    #[inline(always)]
    #[must_use]
    pub fn bmpren(&mut self) -> BMPREN_W<10> {
        BMPREN_W::new(self)
    }
    ///Bit 16 - Master Timer Burst Mode
    #[inline(always)]
    #[must_use]
    pub fn mtbm(&mut self) -> MTBM_W<16> {
        MTBM_W::new(self)
    }
    ///Bit 17 - Timer A Burst Mode
    #[inline(always)]
    #[must_use]
    pub fn tabm(&mut self) -> TABM_W<17> {
        TABM_W::new(self)
    }
    ///Bit 18 - Timer B Burst Mode
    #[inline(always)]
    #[must_use]
    pub fn tbbm(&mut self) -> TBBM_W<18> {
        TBBM_W::new(self)
    }
    ///Bit 19 - Timer C Burst Mode
    #[inline(always)]
    #[must_use]
    pub fn tcbm(&mut self) -> TCBM_W<19> {
        TCBM_W::new(self)
    }
    ///Bit 20 - Timer D Burst Mode
    #[inline(always)]
    #[must_use]
    pub fn tdbm(&mut self) -> TDBM_W<20> {
        TDBM_W::new(self)
    }
    ///Bit 21 - Timer E Burst Mode
    #[inline(always)]
    #[must_use]
    pub fn tebm(&mut self) -> TEBM_W<21> {
        TEBM_W::new(self)
    }
    ///Bit 31 - Burst Mode Status
    #[inline(always)]
    #[must_use]
    pub fn bmstat(&mut self) -> BMSTAT_W<31> {
        BMSTAT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Burst Mode Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bmcr](index.html) module
pub struct BMCR_SPEC;
impl crate::RegisterSpec for BMCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [bmcr::R](R) reader structure
impl crate::Readable for BMCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bmcr::W](W) writer structure
impl crate::Writable for BMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BMCR to value 0
impl crate::Resettable for BMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
