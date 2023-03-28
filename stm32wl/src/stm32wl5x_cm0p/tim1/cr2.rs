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
///Field `CCPC` reader - Capture/compare preloaded control
pub type CCPC_R = crate::BitReader<CCPC_A>;
///Capture/compare preloaded control
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCPC_A {
    ///0: CCxE, CCxNE and OCxM bits are not preloaded
    NotPreloaded = 0,
    ///1: CCxE, CCxNE and OCxM bits are preloaded
    Preloaded = 1,
}
impl From<CCPC_A> for bool {
    #[inline(always)]
    fn from(variant: CCPC_A) -> Self {
        variant as u8 != 0
    }
}
impl CCPC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CCPC_A {
        match self.bits {
            false => CCPC_A::NotPreloaded,
            true => CCPC_A::Preloaded,
        }
    }
    ///Checks if the value of the field is `NotPreloaded`
    #[inline(always)]
    pub fn is_not_preloaded(&self) -> bool {
        *self == CCPC_A::NotPreloaded
    }
    ///Checks if the value of the field is `Preloaded`
    #[inline(always)]
    pub fn is_preloaded(&self) -> bool {
        *self == CCPC_A::Preloaded
    }
}
///Field `CCPC` writer - Capture/compare preloaded control
pub type CCPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, CCPC_A, O>;
impl<'a, const O: u8> CCPC_W<'a, O> {
    ///CCxE, CCxNE and OCxM bits are not preloaded
    #[inline(always)]
    pub fn not_preloaded(self) -> &'a mut W {
        self.variant(CCPC_A::NotPreloaded)
    }
    ///CCxE, CCxNE and OCxM bits are preloaded
    #[inline(always)]
    pub fn preloaded(self) -> &'a mut W {
        self.variant(CCPC_A::Preloaded)
    }
}
///Field `CCUS` reader - Capture/compare control update selection
pub type CCUS_R = crate::BitReader<CCUS_A>;
///Capture/compare control update selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCUS_A {
    ///0: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only
    Bit = 0,
    ///1: When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI
    BitOrEdge = 1,
}
impl From<CCUS_A> for bool {
    #[inline(always)]
    fn from(variant: CCUS_A) -> Self {
        variant as u8 != 0
    }
}
impl CCUS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CCUS_A {
        match self.bits {
            false => CCUS_A::Bit,
            true => CCUS_A::BitOrEdge,
        }
    }
    ///Checks if the value of the field is `Bit`
    #[inline(always)]
    pub fn is_bit_(&self) -> bool {
        *self == CCUS_A::Bit
    }
    ///Checks if the value of the field is `BitOrEdge`
    #[inline(always)]
    pub fn is_bit_or_edge(&self) -> bool {
        *self == CCUS_A::BitOrEdge
    }
}
///Field `CCUS` writer - Capture/compare control update selection
pub type CCUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, CCUS_A, O>;
impl<'a, const O: u8> CCUS_W<'a, O> {
    ///When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit only
    #[inline(always)]
    pub fn bit_(self) -> &'a mut W {
        self.variant(CCUS_A::Bit)
    }
    ///When capture/compare control bits are preloaded (CCPC=1), they are updated by setting the COMG bit or when an rising edge occurs on TRGI
    #[inline(always)]
    pub fn bit_or_edge(self) -> &'a mut W {
        self.variant(CCUS_A::BitOrEdge)
    }
}
///Field `CCDS` reader - Capture/compare DMA selection
pub type CCDS_R = crate::BitReader<CCDS_A>;
///Capture/compare DMA selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCDS_A {
    ///0: CCx DMA request sent when CCx event occurs
    OnCompare = 0,
    ///1: CCx DMA request sent when update event occurs
    OnUpdate = 1,
}
impl From<CCDS_A> for bool {
    #[inline(always)]
    fn from(variant: CCDS_A) -> Self {
        variant as u8 != 0
    }
}
impl CCDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CCDS_A {
        match self.bits {
            false => CCDS_A::OnCompare,
            true => CCDS_A::OnUpdate,
        }
    }
    ///Checks if the value of the field is `OnCompare`
    #[inline(always)]
    pub fn is_on_compare(&self) -> bool {
        *self == CCDS_A::OnCompare
    }
    ///Checks if the value of the field is `OnUpdate`
    #[inline(always)]
    pub fn is_on_update(&self) -> bool {
        *self == CCDS_A::OnUpdate
    }
}
///Field `CCDS` writer - Capture/compare DMA selection
pub type CCDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, CCDS_A, O>;
impl<'a, const O: u8> CCDS_W<'a, O> {
    ///CCx DMA request sent when CCx event occurs
    #[inline(always)]
    pub fn on_compare(self) -> &'a mut W {
        self.variant(CCDS_A::OnCompare)
    }
    ///CCx DMA request sent when update event occurs
    #[inline(always)]
    pub fn on_update(self) -> &'a mut W {
        self.variant(CCDS_A::OnUpdate)
    }
}
///Field `MMS` reader - Master mode selection
pub type MMS_R = crate::FieldReader<u8, MMS_A>;
///Master mode selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MMS_A {
    ///0: The UG bit from the TIMx_EGR register is used as trigger output
    Reset = 0,
    ///1: The counter enable signal, CNT_EN, is used as trigger output
    Enable = 1,
    ///2: The update event is selected as trigger output
    Update = 2,
    ///3: The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred
    ComparePulse = 3,
    ///4: OC1REF signal is used as trigger output
    CompareOc1 = 4,
    ///5: OC2REF signal is used as trigger output
    CompareOc2 = 5,
    ///6: OC3REF signal is used as trigger output
    CompareOc3 = 6,
    ///7: OC4REF signal is used as trigger output
    CompareOc4 = 7,
}
impl From<MMS_A> for u8 {
    #[inline(always)]
    fn from(variant: MMS_A) -> Self {
        variant as _
    }
}
impl MMS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MMS_A {
        match self.bits {
            0 => MMS_A::Reset,
            1 => MMS_A::Enable,
            2 => MMS_A::Update,
            3 => MMS_A::ComparePulse,
            4 => MMS_A::CompareOc1,
            5 => MMS_A::CompareOc2,
            6 => MMS_A::CompareOc3,
            7 => MMS_A::CompareOc4,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == MMS_A::Reset
    }
    ///Checks if the value of the field is `Enable`
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MMS_A::Enable
    }
    ///Checks if the value of the field is `Update`
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == MMS_A::Update
    }
    ///Checks if the value of the field is `ComparePulse`
    #[inline(always)]
    pub fn is_compare_pulse(&self) -> bool {
        *self == MMS_A::ComparePulse
    }
    ///Checks if the value of the field is `CompareOc1`
    #[inline(always)]
    pub fn is_compare_oc1(&self) -> bool {
        *self == MMS_A::CompareOc1
    }
    ///Checks if the value of the field is `CompareOc2`
    #[inline(always)]
    pub fn is_compare_oc2(&self) -> bool {
        *self == MMS_A::CompareOc2
    }
    ///Checks if the value of the field is `CompareOc3`
    #[inline(always)]
    pub fn is_compare_oc3(&self) -> bool {
        *self == MMS_A::CompareOc3
    }
    ///Checks if the value of the field is `CompareOc4`
    #[inline(always)]
    pub fn is_compare_oc4(&self) -> bool {
        *self == MMS_A::CompareOc4
    }
}
///Field `MMS` writer - Master mode selection
pub type MMS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR2_SPEC, u8, MMS_A, 3, O>;
impl<'a, const O: u8> MMS_W<'a, O> {
    ///The UG bit from the TIMx_EGR register is used as trigger output
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(MMS_A::Reset)
    }
    ///The counter enable signal, CNT_EN, is used as trigger output
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MMS_A::Enable)
    }
    ///The update event is selected as trigger output
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(MMS_A::Update)
    }
    ///The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred
    #[inline(always)]
    pub fn compare_pulse(self) -> &'a mut W {
        self.variant(MMS_A::ComparePulse)
    }
    ///OC1REF signal is used as trigger output
    #[inline(always)]
    pub fn compare_oc1(self) -> &'a mut W {
        self.variant(MMS_A::CompareOc1)
    }
    ///OC2REF signal is used as trigger output
    #[inline(always)]
    pub fn compare_oc2(self) -> &'a mut W {
        self.variant(MMS_A::CompareOc2)
    }
    ///OC3REF signal is used as trigger output
    #[inline(always)]
    pub fn compare_oc3(self) -> &'a mut W {
        self.variant(MMS_A::CompareOc3)
    }
    ///OC4REF signal is used as trigger output
    #[inline(always)]
    pub fn compare_oc4(self) -> &'a mut W {
        self.variant(MMS_A::CompareOc4)
    }
}
///Field `TI1S` reader - TI1 selection
pub type TI1S_R = crate::BitReader<TI1S_A>;
///TI1 selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TI1S_A {
    ///0: The TIMx_CH1 pin is connected to TI1 input
    Normal = 0,
    ///1: The TIMx_CH1, CH2, CH3 pins are connected to TI1 input
    Xor = 1,
}
impl From<TI1S_A> for bool {
    #[inline(always)]
    fn from(variant: TI1S_A) -> Self {
        variant as u8 != 0
    }
}
impl TI1S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TI1S_A {
        match self.bits {
            false => TI1S_A::Normal,
            true => TI1S_A::Xor,
        }
    }
    ///Checks if the value of the field is `Normal`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TI1S_A::Normal
    }
    ///Checks if the value of the field is `Xor`
    #[inline(always)]
    pub fn is_xor(&self) -> bool {
        *self == TI1S_A::Xor
    }
}
///Field `TI1S` writer - TI1 selection
pub type TI1S_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, TI1S_A, O>;
impl<'a, const O: u8> TI1S_W<'a, O> {
    ///The TIMx_CH1 pin is connected to TI1 input
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TI1S_A::Normal)
    }
    ///The TIMx_CH1, CH2, CH3 pins are connected to TI1 input
    #[inline(always)]
    pub fn xor(self) -> &'a mut W {
        self.variant(TI1S_A::Xor)
    }
}
///Field `OIS1` reader - Output Idle state 1 (OC1 output)
pub type OIS1_R = crate::BitReader<OIS1_A>;
///Output Idle state 1 (OC1 output)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OIS1_A {
    ///0: OCx=0 (after a dead-time if OCx(N) is implemented) when MOE=0
    Disabled = 0,
    ///1: OCx=1 (after a dead-time if OCx(N) is implemented) when MOE=0
    Enabled = 1,
}
impl From<OIS1_A> for bool {
    #[inline(always)]
    fn from(variant: OIS1_A) -> Self {
        variant as u8 != 0
    }
}
impl OIS1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OIS1_A {
        match self.bits {
            false => OIS1_A::Disabled,
            true => OIS1_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OIS1_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OIS1_A::Enabled
    }
}
///Field `OIS1` writer - Output Idle state 1 (OC1 output)
pub type OIS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, OIS1_A, O>;
impl<'a, const O: u8> OIS1_W<'a, O> {
    ///OCx=0 (after a dead-time if OCx(N) is implemented) when MOE=0
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OIS1_A::Disabled)
    }
    ///OCx=1 (after a dead-time if OCx(N) is implemented) when MOE=0
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OIS1_A::Enabled)
    }
}
///Field `OIS1N` reader - Output Idle state 1 (OC1N output)
pub type OIS1N_R = crate::BitReader<OIS1N_A>;
///Output Idle state 1 (OC1N output)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OIS1N_A {
    ///0: OCxN=0 after a dead-time when MOE=0
    Disabled = 0,
    ///1: OCxN=1 after a dead-time when MOE=0
    Enabled = 1,
}
impl From<OIS1N_A> for bool {
    #[inline(always)]
    fn from(variant: OIS1N_A) -> Self {
        variant as u8 != 0
    }
}
impl OIS1N_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OIS1N_A {
        match self.bits {
            false => OIS1N_A::Disabled,
            true => OIS1N_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OIS1N_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OIS1N_A::Enabled
    }
}
///Field `OIS1N` writer - Output Idle state 1 (OC1N output)
pub type OIS1N_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, OIS1N_A, O>;
impl<'a, const O: u8> OIS1N_W<'a, O> {
    ///OCxN=0 after a dead-time when MOE=0
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OIS1N_A::Disabled)
    }
    ///OCxN=1 after a dead-time when MOE=0
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OIS1N_A::Enabled)
    }
}
///Field `OIS2N` reader - Output Idle state 2 (OC2N output)
pub use OIS1N_R as OIS2N_R;
///Field `OIS3N` reader - Output Idle state 3 (OC3N output)
pub use OIS1N_R as OIS3N_R;
///Field `OIS2N` writer - Output Idle state 2 (OC2N output)
pub use OIS1N_W as OIS2N_W;
///Field `OIS3N` writer - Output Idle state 3 (OC3N output)
pub use OIS1N_W as OIS3N_W;
///Field `OIS2` reader - Output Idle state 2 (OC2 output)
pub use OIS1_R as OIS2_R;
///Field `OIS3` reader - Output Idle state 3 (OC3 output)
pub use OIS1_R as OIS3_R;
///Field `OIS4` reader - Output Idle state 4 (OC4 output)
pub use OIS1_R as OIS4_R;
///Field `OIS5` reader - Output Idle state 5 (OC5 output)
pub use OIS1_R as OIS5_R;
///Field `OIS6` reader - Output Idle state 6 (OC6 output)
pub use OIS1_R as OIS6_R;
///Field `OIS2` writer - Output Idle state 2 (OC2 output)
pub use OIS1_W as OIS2_W;
///Field `OIS3` writer - Output Idle state 3 (OC3 output)
pub use OIS1_W as OIS3_W;
///Field `OIS4` writer - Output Idle state 4 (OC4 output)
pub use OIS1_W as OIS4_W;
///Field `OIS5` writer - Output Idle state 5 (OC5 output)
pub use OIS1_W as OIS5_W;
///Field `OIS6` writer - Output Idle state 6 (OC6 output)
pub use OIS1_W as OIS6_W;
///Field `MMS2` reader - Master mode selection 2
pub type MMS2_R = crate::FieldReader<u8, MMS2_A>;
///Master mode selection 2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MMS2_A {
    ///0: Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO2). If the reset is generated by the trigger input (slave mode controller configured in reset mode), the signal on TRGO2 is delayed compared to the actual reset
    Reset = 0,
    ///1: Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO2). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated by a logic AND between the CEN control bit and the trigger input when configured in Gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO2, except if the Master/Slave mode is selected (see the MSM bit description in TIMx_SMCR register)
    Enable = 1,
    ///2: Update - the update event is selected as trigger output (TRGO2). For instance, a master timer can then be used as a prescaler for a slave timer
    Update = 2,
    ///3: Compare pulse - the trigger output sends a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or compare match occurs (TRGO2)
    ComparePulse = 3,
    ///4: Compare - OC1REFC signal is used as trigger output (TRGO2)
    CompareOc1 = 4,
    ///5: Compare - OC2REFC signal is used as trigger output (TRGO2)
    CompareOc2 = 5,
    ///6: Compare - OC3REFC signal is used as trigger output (TRGO2)
    CompareOc3 = 6,
    ///7: Compare - OC4REFC signal is used as trigger output (TRGO2)
    CompareOc4 = 7,
    ///8: Compare - OC5REFC signal is used as trigger output (TRGO2)
    CompareOc5 = 8,
    ///9: Compare - OC6REFC signal is used as trigger output (TRGO2)
    CompareOc6 = 9,
    ///10: Compare Pulse - OC4REFC rising or falling edges generate pulses on TRGO2
    PulseOc4 = 10,
    ///11: Compare Pulse - OC6REFC rising or falling edges generate pulses on TRGO2
    PulseOc6 = 11,
    ///12: Compare Pulse - OC4REFC or OC6REFC rising edges generate pulses on TRGO2
    RisingOc46 = 12,
    ///13: Compare Pulse - OC4REFC rising or OC6REFC falling edges generate pulses on TRGO2
    RisingOc4FallingOc6 = 13,
    ///14: Compare Pulse - OC5REFC or OC6REFC rising edges generate pulses on TRGO2
    RisingOc56 = 14,
    ///15: Compare Pulse - OC5REFC rising or OC6REFC falling edges generate pulses on TRGO2
    RisingOc5FallingOc6 = 15,
}
impl From<MMS2_A> for u8 {
    #[inline(always)]
    fn from(variant: MMS2_A) -> Self {
        variant as _
    }
}
impl MMS2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MMS2_A {
        match self.bits {
            0 => MMS2_A::Reset,
            1 => MMS2_A::Enable,
            2 => MMS2_A::Update,
            3 => MMS2_A::ComparePulse,
            4 => MMS2_A::CompareOc1,
            5 => MMS2_A::CompareOc2,
            6 => MMS2_A::CompareOc3,
            7 => MMS2_A::CompareOc4,
            8 => MMS2_A::CompareOc5,
            9 => MMS2_A::CompareOc6,
            10 => MMS2_A::PulseOc4,
            11 => MMS2_A::PulseOc6,
            12 => MMS2_A::RisingOc46,
            13 => MMS2_A::RisingOc4FallingOc6,
            14 => MMS2_A::RisingOc56,
            15 => MMS2_A::RisingOc5FallingOc6,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == MMS2_A::Reset
    }
    ///Checks if the value of the field is `Enable`
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MMS2_A::Enable
    }
    ///Checks if the value of the field is `Update`
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == MMS2_A::Update
    }
    ///Checks if the value of the field is `ComparePulse`
    #[inline(always)]
    pub fn is_compare_pulse(&self) -> bool {
        *self == MMS2_A::ComparePulse
    }
    ///Checks if the value of the field is `CompareOc1`
    #[inline(always)]
    pub fn is_compare_oc1(&self) -> bool {
        *self == MMS2_A::CompareOc1
    }
    ///Checks if the value of the field is `CompareOc2`
    #[inline(always)]
    pub fn is_compare_oc2(&self) -> bool {
        *self == MMS2_A::CompareOc2
    }
    ///Checks if the value of the field is `CompareOc3`
    #[inline(always)]
    pub fn is_compare_oc3(&self) -> bool {
        *self == MMS2_A::CompareOc3
    }
    ///Checks if the value of the field is `CompareOc4`
    #[inline(always)]
    pub fn is_compare_oc4(&self) -> bool {
        *self == MMS2_A::CompareOc4
    }
    ///Checks if the value of the field is `CompareOc5`
    #[inline(always)]
    pub fn is_compare_oc5(&self) -> bool {
        *self == MMS2_A::CompareOc5
    }
    ///Checks if the value of the field is `CompareOc6`
    #[inline(always)]
    pub fn is_compare_oc6(&self) -> bool {
        *self == MMS2_A::CompareOc6
    }
    ///Checks if the value of the field is `PulseOc4`
    #[inline(always)]
    pub fn is_pulse_oc4(&self) -> bool {
        *self == MMS2_A::PulseOc4
    }
    ///Checks if the value of the field is `PulseOc6`
    #[inline(always)]
    pub fn is_pulse_oc6(&self) -> bool {
        *self == MMS2_A::PulseOc6
    }
    ///Checks if the value of the field is `RisingOc46`
    #[inline(always)]
    pub fn is_rising_oc4_6(&self) -> bool {
        *self == MMS2_A::RisingOc46
    }
    ///Checks if the value of the field is `RisingOc4FallingOc6`
    #[inline(always)]
    pub fn is_rising_oc4_falling_oc6(&self) -> bool {
        *self == MMS2_A::RisingOc4FallingOc6
    }
    ///Checks if the value of the field is `RisingOc56`
    #[inline(always)]
    pub fn is_rising_oc5_6(&self) -> bool {
        *self == MMS2_A::RisingOc56
    }
    ///Checks if the value of the field is `RisingOc5FallingOc6`
    #[inline(always)]
    pub fn is_rising_oc5_falling_oc6(&self) -> bool {
        *self == MMS2_A::RisingOc5FallingOc6
    }
}
///Field `MMS2` writer - Master mode selection 2
pub type MMS2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR2_SPEC, u8, MMS2_A, 4, O>;
impl<'a, const O: u8> MMS2_W<'a, O> {
    ///Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO2). If the reset is generated by the trigger input (slave mode controller configured in reset mode), the signal on TRGO2 is delayed compared to the actual reset
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(MMS2_A::Reset)
    }
    ///Enable - the Counter Enable signal CNT_EN is used as trigger output (TRGO2). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated by a logic AND between the CEN control bit and the trigger input when configured in Gated mode. When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO2, except if the Master/Slave mode is selected (see the MSM bit description in TIMx_SMCR register)
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MMS2_A::Enable)
    }
    ///Update - the update event is selected as trigger output (TRGO2). For instance, a master timer can then be used as a prescaler for a slave timer
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(MMS2_A::Update)
    }
    ///Compare pulse - the trigger output sends a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or compare match occurs (TRGO2)
    #[inline(always)]
    pub fn compare_pulse(self) -> &'a mut W {
        self.variant(MMS2_A::ComparePulse)
    }
    ///Compare - OC1REFC signal is used as trigger output (TRGO2)
    #[inline(always)]
    pub fn compare_oc1(self) -> &'a mut W {
        self.variant(MMS2_A::CompareOc1)
    }
    ///Compare - OC2REFC signal is used as trigger output (TRGO2)
    #[inline(always)]
    pub fn compare_oc2(self) -> &'a mut W {
        self.variant(MMS2_A::CompareOc2)
    }
    ///Compare - OC3REFC signal is used as trigger output (TRGO2)
    #[inline(always)]
    pub fn compare_oc3(self) -> &'a mut W {
        self.variant(MMS2_A::CompareOc3)
    }
    ///Compare - OC4REFC signal is used as trigger output (TRGO2)
    #[inline(always)]
    pub fn compare_oc4(self) -> &'a mut W {
        self.variant(MMS2_A::CompareOc4)
    }
    ///Compare - OC5REFC signal is used as trigger output (TRGO2)
    #[inline(always)]
    pub fn compare_oc5(self) -> &'a mut W {
        self.variant(MMS2_A::CompareOc5)
    }
    ///Compare - OC6REFC signal is used as trigger output (TRGO2)
    #[inline(always)]
    pub fn compare_oc6(self) -> &'a mut W {
        self.variant(MMS2_A::CompareOc6)
    }
    ///Compare Pulse - OC4REFC rising or falling edges generate pulses on TRGO2
    #[inline(always)]
    pub fn pulse_oc4(self) -> &'a mut W {
        self.variant(MMS2_A::PulseOc4)
    }
    ///Compare Pulse - OC6REFC rising or falling edges generate pulses on TRGO2
    #[inline(always)]
    pub fn pulse_oc6(self) -> &'a mut W {
        self.variant(MMS2_A::PulseOc6)
    }
    ///Compare Pulse - OC4REFC or OC6REFC rising edges generate pulses on TRGO2
    #[inline(always)]
    pub fn rising_oc4_6(self) -> &'a mut W {
        self.variant(MMS2_A::RisingOc46)
    }
    ///Compare Pulse - OC4REFC rising or OC6REFC falling edges generate pulses on TRGO2
    #[inline(always)]
    pub fn rising_oc4_falling_oc6(self) -> &'a mut W {
        self.variant(MMS2_A::RisingOc4FallingOc6)
    }
    ///Compare Pulse - OC5REFC or OC6REFC rising edges generate pulses on TRGO2
    #[inline(always)]
    pub fn rising_oc5_6(self) -> &'a mut W {
        self.variant(MMS2_A::RisingOc56)
    }
    ///Compare Pulse - OC5REFC rising or OC6REFC falling edges generate pulses on TRGO2
    #[inline(always)]
    pub fn rising_oc5_falling_oc6(self) -> &'a mut W {
        self.variant(MMS2_A::RisingOc5FallingOc6)
    }
}
impl R {
    ///Bit 0 - Capture/compare preloaded control
    #[inline(always)]
    pub fn ccpc(&self) -> CCPC_R {
        CCPC_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Capture/compare control update selection
    #[inline(always)]
    pub fn ccus(&self) -> CCUS_R {
        CCUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - Master mode selection
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - TI1 selection
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Output Idle state 1 (OC1 output)
    #[inline(always)]
    pub fn ois1(&self) -> OIS1_R {
        OIS1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Output Idle state 1 (OC1N output)
    #[inline(always)]
    pub fn ois1n(&self) -> OIS1N_R {
        OIS1N_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Output Idle state 2 (OC2 output)
    #[inline(always)]
    pub fn ois2(&self) -> OIS2_R {
        OIS2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output Idle state 2 (OC2N output)
    #[inline(always)]
    pub fn ois2n(&self) -> OIS2N_R {
        OIS2N_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Output Idle state 3 (OC3 output)
    #[inline(always)]
    pub fn ois3(&self) -> OIS3_R {
        OIS3_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Output Idle state 3 (OC3N output)
    #[inline(always)]
    pub fn ois3n(&self) -> OIS3N_R {
        OIS3N_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Output Idle state 4 (OC4 output)
    #[inline(always)]
    pub fn ois4(&self) -> OIS4_R {
        OIS4_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Output Idle state 5 (OC5 output)
    #[inline(always)]
    pub fn ois5(&self) -> OIS5_R {
        OIS5_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Output Idle state 6 (OC6 output)
    #[inline(always)]
    pub fn ois6(&self) -> OIS6_R {
        OIS6_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 20:23 - Master mode selection 2
    #[inline(always)]
    pub fn mms2(&self) -> MMS2_R {
        MMS2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - Capture/compare preloaded control
    #[inline(always)]
    #[must_use]
    pub fn ccpc(&mut self) -> CCPC_W<0> {
        CCPC_W::new(self)
    }
    ///Bit 2 - Capture/compare control update selection
    #[inline(always)]
    #[must_use]
    pub fn ccus(&mut self) -> CCUS_W<2> {
        CCUS_W::new(self)
    }
    ///Bit 3 - Capture/compare DMA selection
    #[inline(always)]
    #[must_use]
    pub fn ccds(&mut self) -> CCDS_W<3> {
        CCDS_W::new(self)
    }
    ///Bits 4:6 - Master mode selection
    #[inline(always)]
    #[must_use]
    pub fn mms(&mut self) -> MMS_W<4> {
        MMS_W::new(self)
    }
    ///Bit 7 - TI1 selection
    #[inline(always)]
    #[must_use]
    pub fn ti1s(&mut self) -> TI1S_W<7> {
        TI1S_W::new(self)
    }
    ///Bit 8 - Output Idle state 1 (OC1 output)
    #[inline(always)]
    #[must_use]
    pub fn ois1(&mut self) -> OIS1_W<8> {
        OIS1_W::new(self)
    }
    ///Bit 9 - Output Idle state 1 (OC1N output)
    #[inline(always)]
    #[must_use]
    pub fn ois1n(&mut self) -> OIS1N_W<9> {
        OIS1N_W::new(self)
    }
    ///Bit 10 - Output Idle state 2 (OC2 output)
    #[inline(always)]
    #[must_use]
    pub fn ois2(&mut self) -> OIS2_W<10> {
        OIS2_W::new(self)
    }
    ///Bit 11 - Output Idle state 2 (OC2N output)
    #[inline(always)]
    #[must_use]
    pub fn ois2n(&mut self) -> OIS2N_W<11> {
        OIS2N_W::new(self)
    }
    ///Bit 12 - Output Idle state 3 (OC3 output)
    #[inline(always)]
    #[must_use]
    pub fn ois3(&mut self) -> OIS3_W<12> {
        OIS3_W::new(self)
    }
    ///Bit 13 - Output Idle state 3 (OC3N output)
    #[inline(always)]
    #[must_use]
    pub fn ois3n(&mut self) -> OIS3N_W<13> {
        OIS3N_W::new(self)
    }
    ///Bit 14 - Output Idle state 4 (OC4 output)
    #[inline(always)]
    #[must_use]
    pub fn ois4(&mut self) -> OIS4_W<14> {
        OIS4_W::new(self)
    }
    ///Bit 16 - Output Idle state 5 (OC5 output)
    #[inline(always)]
    #[must_use]
    pub fn ois5(&mut self) -> OIS5_W<16> {
        OIS5_W::new(self)
    }
    ///Bit 18 - Output Idle state 6 (OC6 output)
    #[inline(always)]
    #[must_use]
    pub fn ois6(&mut self) -> OIS6_W<18> {
        OIS6_W::new(self)
    }
    ///Bits 20:23 - Master mode selection 2
    #[inline(always)]
    #[must_use]
    pub fn mms2(&mut self) -> MMS2_W<20> {
        MMS2_W::new(self)
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
