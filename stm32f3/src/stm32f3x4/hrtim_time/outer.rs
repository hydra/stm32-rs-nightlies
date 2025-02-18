///Register `OUTER` reader
pub struct R(crate::R<OUTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OUTER` writer
pub struct W(crate::W<OUTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTER_SPEC>;
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
impl From<crate::W<OUTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `POL1` reader - Output 1 polarity
pub type POL1_R = crate::BitReader<POL1_A>;
///Output 1 polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL1_A {
    ///0: Positive polarity (output active high)
    ActiveHigh = 0,
    ///1: Negative polarity (output active low)
    ActiveLow = 1,
}
impl From<POL1_A> for bool {
    #[inline(always)]
    fn from(variant: POL1_A) -> Self {
        variant as u8 != 0
    }
}
impl POL1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> POL1_A {
        match self.bits {
            false => POL1_A::ActiveHigh,
            true => POL1_A::ActiveLow,
        }
    }
    ///Checks if the value of the field is `ActiveHigh`
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == POL1_A::ActiveHigh
    }
    ///Checks if the value of the field is `ActiveLow`
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == POL1_A::ActiveLow
    }
}
///Field `POL1` writer - Output 1 polarity
pub type POL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTER_SPEC, POL1_A, O>;
impl<'a, const O: u8> POL1_W<'a, O> {
    ///Positive polarity (output active high)
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(POL1_A::ActiveHigh)
    }
    ///Negative polarity (output active low)
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(POL1_A::ActiveLow)
    }
}
///Field `IDLEM1` reader - Output 1 Idle mode
pub type IDLEM1_R = crate::BitReader<IDLEM1_A>;
///Output 1 Idle mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLEM1_A {
    ///0: No action: the output is not affected by the burst mode operation
    NoEffect = 0,
    ///1: The output is in idle state when requested by the burst mode controller
    SetIdle = 1,
}
impl From<IDLEM1_A> for bool {
    #[inline(always)]
    fn from(variant: IDLEM1_A) -> Self {
        variant as u8 != 0
    }
}
impl IDLEM1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IDLEM1_A {
        match self.bits {
            false => IDLEM1_A::NoEffect,
            true => IDLEM1_A::SetIdle,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == IDLEM1_A::NoEffect
    }
    ///Checks if the value of the field is `SetIdle`
    #[inline(always)]
    pub fn is_set_idle(&self) -> bool {
        *self == IDLEM1_A::SetIdle
    }
}
///Field `IDLEM1` writer - Output 1 Idle mode
pub type IDLEM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTER_SPEC, IDLEM1_A, O>;
impl<'a, const O: u8> IDLEM1_W<'a, O> {
    ///No action: the output is not affected by the burst mode operation
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(IDLEM1_A::NoEffect)
    }
    ///The output is in idle state when requested by the burst mode controller
    #[inline(always)]
    pub fn set_idle(self) -> &'a mut W {
        self.variant(IDLEM1_A::SetIdle)
    }
}
///Field `IDLES1` reader - Output 1 Idle State
pub type IDLES1_R = crate::BitReader<IDLES1_A>;
///Output 1 Idle State
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLES1_A {
    ///0: Output idle state is inactive
    Inactive = 0,
    ///1: Output idle state is active
    Active = 1,
}
impl From<IDLES1_A> for bool {
    #[inline(always)]
    fn from(variant: IDLES1_A) -> Self {
        variant as u8 != 0
    }
}
impl IDLES1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IDLES1_A {
        match self.bits {
            false => IDLES1_A::Inactive,
            true => IDLES1_A::Active,
        }
    }
    ///Checks if the value of the field is `Inactive`
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == IDLES1_A::Inactive
    }
    ///Checks if the value of the field is `Active`
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == IDLES1_A::Active
    }
}
///Field `IDLES1` writer - Output 1 Idle State
pub type IDLES1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTER_SPEC, IDLES1_A, O>;
impl<'a, const O: u8> IDLES1_W<'a, O> {
    ///Output idle state is inactive
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(IDLES1_A::Inactive)
    }
    ///Output idle state is active
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(IDLES1_A::Active)
    }
}
///Field `FAULT1` reader - Output 1 Fault state
pub type FAULT1_R = crate::FieldReader<u8, FAULT1_A>;
///Output 1 Fault state
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FAULT1_A {
    ///0: No action: the output is not affected by the fault input and stays in run mode
    Disabled = 0,
    ///1: Output goes to active state after a fault event
    SetActive = 1,
    ///2: Output goes to inactive state after a fault event
    SetInactive = 2,
    ///3: Output goes to high-z state after a fault event
    SetHighZ = 3,
}
impl From<FAULT1_A> for u8 {
    #[inline(always)]
    fn from(variant: FAULT1_A) -> Self {
        variant as _
    }
}
impl FAULT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FAULT1_A {
        match self.bits {
            0 => FAULT1_A::Disabled,
            1 => FAULT1_A::SetActive,
            2 => FAULT1_A::SetInactive,
            3 => FAULT1_A::SetHighZ,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FAULT1_A::Disabled
    }
    ///Checks if the value of the field is `SetActive`
    #[inline(always)]
    pub fn is_set_active(&self) -> bool {
        *self == FAULT1_A::SetActive
    }
    ///Checks if the value of the field is `SetInactive`
    #[inline(always)]
    pub fn is_set_inactive(&self) -> bool {
        *self == FAULT1_A::SetInactive
    }
    ///Checks if the value of the field is `SetHighZ`
    #[inline(always)]
    pub fn is_set_high_z(&self) -> bool {
        *self == FAULT1_A::SetHighZ
    }
}
///Field `FAULT1` writer - Output 1 Fault state
pub type FAULT1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OUTER_SPEC, u8, FAULT1_A, 2, O>;
impl<'a, const O: u8> FAULT1_W<'a, O> {
    ///No action: the output is not affected by the fault input and stays in run mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FAULT1_A::Disabled)
    }
    ///Output goes to active state after a fault event
    #[inline(always)]
    pub fn set_active(self) -> &'a mut W {
        self.variant(FAULT1_A::SetActive)
    }
    ///Output goes to inactive state after a fault event
    #[inline(always)]
    pub fn set_inactive(self) -> &'a mut W {
        self.variant(FAULT1_A::SetInactive)
    }
    ///Output goes to high-z state after a fault event
    #[inline(always)]
    pub fn set_high_z(self) -> &'a mut W {
        self.variant(FAULT1_A::SetHighZ)
    }
}
///Field `CHP1` reader - Output 1 Chopper enable
pub type CHP1_R = crate::BitReader<CHP1_A>;
///Output 1 Chopper enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHP1_A {
    ///0: Output signal not altered
    Disabled = 0,
    ///1: Output signal is chopped by a carrier signal
    Enabled = 1,
}
impl From<CHP1_A> for bool {
    #[inline(always)]
    fn from(variant: CHP1_A) -> Self {
        variant as u8 != 0
    }
}
impl CHP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CHP1_A {
        match self.bits {
            false => CHP1_A::Disabled,
            true => CHP1_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CHP1_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CHP1_A::Enabled
    }
}
///Field `CHP1` writer - Output 1 Chopper enable
pub type CHP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTER_SPEC, CHP1_A, O>;
impl<'a, const O: u8> CHP1_W<'a, O> {
    ///Output signal not altered
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CHP1_A::Disabled)
    }
    ///Output signal is chopped by a carrier signal
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CHP1_A::Enabled)
    }
}
///Field `DIDL1` reader - Output 1 Deadtime upon burst mode Idle entry
pub type DIDL1_R = crate::BitReader<DIDL1_A>;
///Output 1 Deadtime upon burst mode Idle entry
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIDL1_A {
    ///0: The programmed idle state is applied immediately to the output
    Disabled = 0,
    ///1: Deadtime (inactive level) is inserted on output before entering the idle mode
    Enabled = 1,
}
impl From<DIDL1_A> for bool {
    #[inline(always)]
    fn from(variant: DIDL1_A) -> Self {
        variant as u8 != 0
    }
}
impl DIDL1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DIDL1_A {
        match self.bits {
            false => DIDL1_A::Disabled,
            true => DIDL1_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DIDL1_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DIDL1_A::Enabled
    }
}
///Field `DIDL1` writer - Output 1 Deadtime upon burst mode Idle entry
pub type DIDL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTER_SPEC, DIDL1_A, O>;
impl<'a, const O: u8> DIDL1_W<'a, O> {
    ///The programmed idle state is applied immediately to the output
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIDL1_A::Disabled)
    }
    ///Deadtime (inactive level) is inserted on output before entering the idle mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIDL1_A::Enabled)
    }
}
///Field `DTEN` reader - Deadtime enable
pub type DTEN_R = crate::BitReader<DTEN_A>;
///Deadtime enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTEN_A {
    ///0: Output 1 and 2 signals are independent
    Disabled = 0,
    ///1: Deadtime is inserted between output 1 and output 2
    Enabled = 1,
}
impl From<DTEN_A> for bool {
    #[inline(always)]
    fn from(variant: DTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DTEN_A {
        match self.bits {
            false => DTEN_A::Disabled,
            true => DTEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DTEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DTEN_A::Enabled
    }
}
///Field `DTEN` writer - Deadtime enable
pub type DTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTER_SPEC, DTEN_A, O>;
impl<'a, const O: u8> DTEN_W<'a, O> {
    ///Output 1 and 2 signals are independent
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DTEN_A::Disabled)
    }
    ///Deadtime is inserted between output 1 and output 2
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DTEN_A::Enabled)
    }
}
///Field `DLYPRTEN` reader - Delayed Protection Enable
pub type DLYPRTEN_R = crate::BitReader<DLYPRTEN_A>;
///Delayed Protection Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYPRTEN_A {
    ///0: No action
    Disabled = 0,
    ///1: Delayed protection is enabled, as per DLYPRT bits
    Enabled = 1,
}
impl From<DLYPRTEN_A> for bool {
    #[inline(always)]
    fn from(variant: DLYPRTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DLYPRTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DLYPRTEN_A {
        match self.bits {
            false => DLYPRTEN_A::Disabled,
            true => DLYPRTEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DLYPRTEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DLYPRTEN_A::Enabled
    }
}
///Field `DLYPRTEN` writer - Delayed Protection Enable
pub type DLYPRTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTER_SPEC, DLYPRTEN_A, O>;
impl<'a, const O: u8> DLYPRTEN_W<'a, O> {
    ///No action
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DLYPRTEN_A::Disabled)
    }
    ///Delayed protection is enabled, as per DLYPRT bits
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DLYPRTEN_A::Enabled)
    }
}
///Field `DLYPRT` reader - Delayed Protection
pub type DLYPRT_R = crate::FieldReader<u8, DLYPRT_A>;
///Delayed Protection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DLYPRT_A {
    ///0: Output 1 delayed idle on external event 8
    Output1Ee8 = 0,
    ///1: Output 2 delayed idle on external event 8
    Output2Ee8 = 1,
    ///2: Output 1 and 2 delayed idle on external event 8
    Output12Ee8 = 2,
    ///3: Balanced idle on external event 8
    BalancedEe8 = 3,
    ///4: Output 1 delayed idle on external event 9
    Output1Ee9 = 4,
    ///5: Output 2 delayed idle on external event 9
    Output2Ee9 = 5,
    ///6: Output 1 and 2 delayed idle on external event 9
    Output12Ee9 = 6,
    ///7: Balanced idle on external event 9
    BalancedEe9 = 7,
}
impl From<DLYPRT_A> for u8 {
    #[inline(always)]
    fn from(variant: DLYPRT_A) -> Self {
        variant as _
    }
}
impl DLYPRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DLYPRT_A {
        match self.bits {
            0 => DLYPRT_A::Output1Ee8,
            1 => DLYPRT_A::Output2Ee8,
            2 => DLYPRT_A::Output12Ee8,
            3 => DLYPRT_A::BalancedEe8,
            4 => DLYPRT_A::Output1Ee9,
            5 => DLYPRT_A::Output2Ee9,
            6 => DLYPRT_A::Output12Ee9,
            7 => DLYPRT_A::BalancedEe9,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Output1Ee8`
    #[inline(always)]
    pub fn is_output1_ee8(&self) -> bool {
        *self == DLYPRT_A::Output1Ee8
    }
    ///Checks if the value of the field is `Output2Ee8`
    #[inline(always)]
    pub fn is_output2_ee8(&self) -> bool {
        *self == DLYPRT_A::Output2Ee8
    }
    ///Checks if the value of the field is `Output12Ee8`
    #[inline(always)]
    pub fn is_output1_2_ee8(&self) -> bool {
        *self == DLYPRT_A::Output12Ee8
    }
    ///Checks if the value of the field is `BalancedEe8`
    #[inline(always)]
    pub fn is_balanced_ee8(&self) -> bool {
        *self == DLYPRT_A::BalancedEe8
    }
    ///Checks if the value of the field is `Output1Ee9`
    #[inline(always)]
    pub fn is_output1_ee9(&self) -> bool {
        *self == DLYPRT_A::Output1Ee9
    }
    ///Checks if the value of the field is `Output2Ee9`
    #[inline(always)]
    pub fn is_output2_ee9(&self) -> bool {
        *self == DLYPRT_A::Output2Ee9
    }
    ///Checks if the value of the field is `Output12Ee9`
    #[inline(always)]
    pub fn is_output1_2_ee9(&self) -> bool {
        *self == DLYPRT_A::Output12Ee9
    }
    ///Checks if the value of the field is `BalancedEe9`
    #[inline(always)]
    pub fn is_balanced_ee9(&self) -> bool {
        *self == DLYPRT_A::BalancedEe9
    }
}
///Field `DLYPRT` writer - Delayed Protection
pub type DLYPRT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OUTER_SPEC, u8, DLYPRT_A, 3, O>;
impl<'a, const O: u8> DLYPRT_W<'a, O> {
    ///Output 1 delayed idle on external event 8
    #[inline(always)]
    pub fn output1_ee8(self) -> &'a mut W {
        self.variant(DLYPRT_A::Output1Ee8)
    }
    ///Output 2 delayed idle on external event 8
    #[inline(always)]
    pub fn output2_ee8(self) -> &'a mut W {
        self.variant(DLYPRT_A::Output2Ee8)
    }
    ///Output 1 and 2 delayed idle on external event 8
    #[inline(always)]
    pub fn output1_2_ee8(self) -> &'a mut W {
        self.variant(DLYPRT_A::Output12Ee8)
    }
    ///Balanced idle on external event 8
    #[inline(always)]
    pub fn balanced_ee8(self) -> &'a mut W {
        self.variant(DLYPRT_A::BalancedEe8)
    }
    ///Output 1 delayed idle on external event 9
    #[inline(always)]
    pub fn output1_ee9(self) -> &'a mut W {
        self.variant(DLYPRT_A::Output1Ee9)
    }
    ///Output 2 delayed idle on external event 9
    #[inline(always)]
    pub fn output2_ee9(self) -> &'a mut W {
        self.variant(DLYPRT_A::Output2Ee9)
    }
    ///Output 1 and 2 delayed idle on external event 9
    #[inline(always)]
    pub fn output1_2_ee9(self) -> &'a mut W {
        self.variant(DLYPRT_A::Output12Ee9)
    }
    ///Balanced idle on external event 9
    #[inline(always)]
    pub fn balanced_ee9(self) -> &'a mut W {
        self.variant(DLYPRT_A::BalancedEe9)
    }
}
///Field `CHP2` reader - Output 2 Chopper enable
pub use CHP1_R as CHP2_R;
///Field `CHP2` writer - Output 2 Chopper enable
pub use CHP1_W as CHP2_W;
///Field `DIDL2` reader - Output 2 Deadtime upon burst mode Idle entry
pub use DIDL1_R as DIDL2_R;
///Field `DIDL2` writer - Output 2 Deadtime upon burst mode Idle entry
pub use DIDL1_W as DIDL2_W;
///Field `FAULT2` reader - Output 2 Fault state
pub use FAULT1_R as FAULT2_R;
///Field `FAULT2` writer - Output 2 Fault state
pub use FAULT1_W as FAULT2_W;
///Field `IDLEM2` reader - Output 2 Idle mode
pub use IDLEM1_R as IDLEM2_R;
///Field `IDLEM2` writer - Output 2 Idle mode
pub use IDLEM1_W as IDLEM2_W;
///Field `IDLES2` reader - Output 2 Idle State
pub use IDLES1_R as IDLES2_R;
///Field `IDLES2` writer - Output 2 Idle State
pub use IDLES1_W as IDLES2_W;
///Field `POL2` reader - Output 2 polarity
pub use POL1_R as POL2_R;
///Field `POL2` writer - Output 2 polarity
pub use POL1_W as POL2_W;
impl R {
    ///Bit 1 - Output 1 polarity
    #[inline(always)]
    pub fn pol1(&self) -> POL1_R {
        POL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Output 1 Idle mode
    #[inline(always)]
    pub fn idlem1(&self) -> IDLEM1_R {
        IDLEM1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Output 1 Idle State
    #[inline(always)]
    pub fn idles1(&self) -> IDLES1_R {
        IDLES1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - Output 1 Fault state
    #[inline(always)]
    pub fn fault1(&self) -> FAULT1_R {
        FAULT1_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Output 1 Chopper enable
    #[inline(always)]
    pub fn chp1(&self) -> CHP1_R {
        CHP1_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Output 1 Deadtime upon burst mode Idle entry
    #[inline(always)]
    pub fn didl1(&self) -> DIDL1_R {
        DIDL1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Deadtime enable
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Delayed Protection Enable
    #[inline(always)]
    pub fn dlyprten(&self) -> DLYPRTEN_R {
        DLYPRTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:12 - Delayed Protection
    #[inline(always)]
    pub fn dlyprt(&self) -> DLYPRT_R {
        DLYPRT_R::new(((self.bits >> 10) & 7) as u8)
    }
    ///Bit 17 - Output 2 polarity
    #[inline(always)]
    pub fn pol2(&self) -> POL2_R {
        POL2_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Output 2 Idle mode
    #[inline(always)]
    pub fn idlem2(&self) -> IDLEM2_R {
        IDLEM2_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Output 2 Idle State
    #[inline(always)]
    pub fn idles2(&self) -> IDLES2_R {
        IDLES2_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:21 - Output 2 Fault state
    #[inline(always)]
    pub fn fault2(&self) -> FAULT2_R {
        FAULT2_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - Output 2 Chopper enable
    #[inline(always)]
    pub fn chp2(&self) -> CHP2_R {
        CHP2_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Output 2 Deadtime upon burst mode Idle entry
    #[inline(always)]
    pub fn didl2(&self) -> DIDL2_R {
        DIDL2_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - Output 1 polarity
    #[inline(always)]
    #[must_use]
    pub fn pol1(&mut self) -> POL1_W<1> {
        POL1_W::new(self)
    }
    ///Bit 2 - Output 1 Idle mode
    #[inline(always)]
    #[must_use]
    pub fn idlem1(&mut self) -> IDLEM1_W<2> {
        IDLEM1_W::new(self)
    }
    ///Bit 3 - Output 1 Idle State
    #[inline(always)]
    #[must_use]
    pub fn idles1(&mut self) -> IDLES1_W<3> {
        IDLES1_W::new(self)
    }
    ///Bits 4:5 - Output 1 Fault state
    #[inline(always)]
    #[must_use]
    pub fn fault1(&mut self) -> FAULT1_W<4> {
        FAULT1_W::new(self)
    }
    ///Bit 6 - Output 1 Chopper enable
    #[inline(always)]
    #[must_use]
    pub fn chp1(&mut self) -> CHP1_W<6> {
        CHP1_W::new(self)
    }
    ///Bit 7 - Output 1 Deadtime upon burst mode Idle entry
    #[inline(always)]
    #[must_use]
    pub fn didl1(&mut self) -> DIDL1_W<7> {
        DIDL1_W::new(self)
    }
    ///Bit 8 - Deadtime enable
    #[inline(always)]
    #[must_use]
    pub fn dten(&mut self) -> DTEN_W<8> {
        DTEN_W::new(self)
    }
    ///Bit 9 - Delayed Protection Enable
    #[inline(always)]
    #[must_use]
    pub fn dlyprten(&mut self) -> DLYPRTEN_W<9> {
        DLYPRTEN_W::new(self)
    }
    ///Bits 10:12 - Delayed Protection
    #[inline(always)]
    #[must_use]
    pub fn dlyprt(&mut self) -> DLYPRT_W<10> {
        DLYPRT_W::new(self)
    }
    ///Bit 17 - Output 2 polarity
    #[inline(always)]
    #[must_use]
    pub fn pol2(&mut self) -> POL2_W<17> {
        POL2_W::new(self)
    }
    ///Bit 18 - Output 2 Idle mode
    #[inline(always)]
    #[must_use]
    pub fn idlem2(&mut self) -> IDLEM2_W<18> {
        IDLEM2_W::new(self)
    }
    ///Bit 19 - Output 2 Idle State
    #[inline(always)]
    #[must_use]
    pub fn idles2(&mut self) -> IDLES2_W<19> {
        IDLES2_W::new(self)
    }
    ///Bits 20:21 - Output 2 Fault state
    #[inline(always)]
    #[must_use]
    pub fn fault2(&mut self) -> FAULT2_W<20> {
        FAULT2_W::new(self)
    }
    ///Bit 22 - Output 2 Chopper enable
    #[inline(always)]
    #[must_use]
    pub fn chp2(&mut self) -> CHP2_W<22> {
        CHP2_W::new(self)
    }
    ///Bit 23 - Output 2 Deadtime upon burst mode Idle entry
    #[inline(always)]
    #[must_use]
    pub fn didl2(&mut self) -> DIDL2_W<23> {
        DIDL2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timerx Output Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [outer](index.html) module
pub struct OUTER_SPEC;
impl crate::RegisterSpec for OUTER_SPEC {
    type Ux = u32;
}
///`read()` method returns [outer::R](R) reader structure
impl crate::Readable for OUTER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [outer::W](W) writer structure
impl crate::Writable for OUTER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OUTER to value 0
impl crate::Resettable for OUTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
