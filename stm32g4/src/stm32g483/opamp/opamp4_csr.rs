///Register `OPAMP4_CSR` reader
pub struct R(crate::R<OPAMP4_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAMP4_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAMP4_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAMP4_CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OPAMP4_CSR` writer
pub struct W(crate::W<OPAMP4_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAMP4_CSR_SPEC>;
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
impl From<crate::W<OPAMP4_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAMP4_CSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OPAEN` reader - Operational amplifier Enable
pub type OPAEN_R = crate::BitReader<OPAEN_A>;
///Operational amplifier Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPAEN_A {
    ///0: OpAmp disabled
    Disabled = 0,
    ///1: OpAmp enabled
    Enabled = 1,
}
impl From<OPAEN_A> for bool {
    #[inline(always)]
    fn from(variant: OPAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl OPAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OPAEN_A {
        match self.bits {
            false => OPAEN_A::Disabled,
            true => OPAEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OPAEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OPAEN_A::Enabled
    }
}
///Field `OPAEN` writer - Operational amplifier Enable
pub type OPAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP4_CSR_SPEC, OPAEN_A, O>;
impl<'a, const O: u8> OPAEN_W<'a, O> {
    ///OpAmp disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OPAEN_A::Disabled)
    }
    ///OpAmp enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OPAEN_A::Enabled)
    }
}
///Field `FORCE_VP` reader - FORCE_VP
pub type FORCE_VP_R = crate::BitReader<FORCE_VP_A>;
///FORCE_VP
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORCE_VP_A {
    ///0: Non-inverting input connected configured inputs
    Normal = 0,
    ///1: Non-inverting input connected to calibration reference voltage
    CalibrationVerification = 1,
}
impl From<FORCE_VP_A> for bool {
    #[inline(always)]
    fn from(variant: FORCE_VP_A) -> Self {
        variant as u8 != 0
    }
}
impl FORCE_VP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FORCE_VP_A {
        match self.bits {
            false => FORCE_VP_A::Normal,
            true => FORCE_VP_A::CalibrationVerification,
        }
    }
    ///Checks if the value of the field is `Normal`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == FORCE_VP_A::Normal
    }
    ///Checks if the value of the field is `CalibrationVerification`
    #[inline(always)]
    pub fn is_calibration_verification(&self) -> bool {
        *self == FORCE_VP_A::CalibrationVerification
    }
}
///Field `FORCE_VP` writer - FORCE_VP
pub type FORCE_VP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP4_CSR_SPEC, FORCE_VP_A, O>;
impl<'a, const O: u8> FORCE_VP_W<'a, O> {
    ///Non-inverting input connected configured inputs
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(FORCE_VP_A::Normal)
    }
    ///Non-inverting input connected to calibration reference voltage
    #[inline(always)]
    pub fn calibration_verification(self) -> &'a mut W {
        self.variant(FORCE_VP_A::CalibrationVerification)
    }
}
///Field `VP_SEL` reader - VP_SEL
pub type VP_SEL_R = crate::FieldReader<u8, VP_SEL_A>;
///VP_SEL
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VP_SEL_A {
    ///0: VINP0 connected to VINP input
    Vinp0 = 0,
    ///1: VINP1 connected to VINP input
    Vinp1 = 1,
    ///2: VINP2 connected to VINP input
    Vinp2 = 2,
    ///3: DAC4_CH1 connected to VINP input
    Dac4Ch1 = 3,
}
impl From<VP_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VP_SEL_A) -> Self {
        variant as _
    }
}
impl VP_SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VP_SEL_A {
        match self.bits {
            0 => VP_SEL_A::Vinp0,
            1 => VP_SEL_A::Vinp1,
            2 => VP_SEL_A::Vinp2,
            3 => VP_SEL_A::Dac4Ch1,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Vinp0`
    #[inline(always)]
    pub fn is_vinp0(&self) -> bool {
        *self == VP_SEL_A::Vinp0
    }
    ///Checks if the value of the field is `Vinp1`
    #[inline(always)]
    pub fn is_vinp1(&self) -> bool {
        *self == VP_SEL_A::Vinp1
    }
    ///Checks if the value of the field is `Vinp2`
    #[inline(always)]
    pub fn is_vinp2(&self) -> bool {
        *self == VP_SEL_A::Vinp2
    }
    ///Checks if the value of the field is `Dac4Ch1`
    #[inline(always)]
    pub fn is_dac4_ch1(&self) -> bool {
        *self == VP_SEL_A::Dac4Ch1
    }
}
///Field `VP_SEL` writer - VP_SEL
pub type VP_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OPAMP4_CSR_SPEC, u8, VP_SEL_A, 2, O>;
impl<'a, const O: u8> VP_SEL_W<'a, O> {
    ///VINP0 connected to VINP input
    #[inline(always)]
    pub fn vinp0(self) -> &'a mut W {
        self.variant(VP_SEL_A::Vinp0)
    }
    ///VINP1 connected to VINP input
    #[inline(always)]
    pub fn vinp1(self) -> &'a mut W {
        self.variant(VP_SEL_A::Vinp1)
    }
    ///VINP2 connected to VINP input
    #[inline(always)]
    pub fn vinp2(self) -> &'a mut W {
        self.variant(VP_SEL_A::Vinp2)
    }
    ///DAC4_CH1 connected to VINP input
    #[inline(always)]
    pub fn dac4_ch1(self) -> &'a mut W {
        self.variant(VP_SEL_A::Dac4Ch1)
    }
}
///Field `USERTRIM` reader - USERTRIM
pub type USERTRIM_R = crate::BitReader<USERTRIM_A>;
///USERTRIM
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USERTRIM_A {
    ///0: Factory trim used
    Factory = 0,
    ///1: User trim used
    User = 1,
}
impl From<USERTRIM_A> for bool {
    #[inline(always)]
    fn from(variant: USERTRIM_A) -> Self {
        variant as u8 != 0
    }
}
impl USERTRIM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> USERTRIM_A {
        match self.bits {
            false => USERTRIM_A::Factory,
            true => USERTRIM_A::User,
        }
    }
    ///Checks if the value of the field is `Factory`
    #[inline(always)]
    pub fn is_factory(&self) -> bool {
        *self == USERTRIM_A::Factory
    }
    ///Checks if the value of the field is `User`
    #[inline(always)]
    pub fn is_user(&self) -> bool {
        *self == USERTRIM_A::User
    }
}
///Field `USERTRIM` writer - USERTRIM
pub type USERTRIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP4_CSR_SPEC, USERTRIM_A, O>;
impl<'a, const O: u8> USERTRIM_W<'a, O> {
    ///Factory trim used
    #[inline(always)]
    pub fn factory(self) -> &'a mut W {
        self.variant(USERTRIM_A::Factory)
    }
    ///User trim used
    #[inline(always)]
    pub fn user(self) -> &'a mut W {
        self.variant(USERTRIM_A::User)
    }
}
///Field `VM_SEL` reader - VM_SEL
pub type VM_SEL_R = crate::FieldReader<u8, VM_SEL_A>;
///VM_SEL
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VM_SEL_A {
    ///0: VINM0 connected to VINM input
    Vinm0 = 0,
    ///1: VINM1 connected to VINM input
    Vinm1 = 1,
    ///2: Feedback resistor connected to VINM (PGA mode)
    Pga = 2,
    ///3: OpAmp output connected to VINM (Follower mode)
    Output = 3,
}
impl From<VM_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VM_SEL_A) -> Self {
        variant as _
    }
}
impl VM_SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VM_SEL_A {
        match self.bits {
            0 => VM_SEL_A::Vinm0,
            1 => VM_SEL_A::Vinm1,
            2 => VM_SEL_A::Pga,
            3 => VM_SEL_A::Output,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Vinm0`
    #[inline(always)]
    pub fn is_vinm0(&self) -> bool {
        *self == VM_SEL_A::Vinm0
    }
    ///Checks if the value of the field is `Vinm1`
    #[inline(always)]
    pub fn is_vinm1(&self) -> bool {
        *self == VM_SEL_A::Vinm1
    }
    ///Checks if the value of the field is `Pga`
    #[inline(always)]
    pub fn is_pga(&self) -> bool {
        *self == VM_SEL_A::Pga
    }
    ///Checks if the value of the field is `Output`
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == VM_SEL_A::Output
    }
}
///Field `VM_SEL` writer - VM_SEL
pub type VM_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OPAMP4_CSR_SPEC, u8, VM_SEL_A, 2, O>;
impl<'a, const O: u8> VM_SEL_W<'a, O> {
    ///VINM0 connected to VINM input
    #[inline(always)]
    pub fn vinm0(self) -> &'a mut W {
        self.variant(VM_SEL_A::Vinm0)
    }
    ///VINM1 connected to VINM input
    #[inline(always)]
    pub fn vinm1(self) -> &'a mut W {
        self.variant(VM_SEL_A::Vinm1)
    }
    ///Feedback resistor connected to VINM (PGA mode)
    #[inline(always)]
    pub fn pga(self) -> &'a mut W {
        self.variant(VM_SEL_A::Pga)
    }
    ///OpAmp output connected to VINM (Follower mode)
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(VM_SEL_A::Output)
    }
}
///Field `OPAHSM` reader - OPAHSM
pub type OPAHSM_R = crate::BitReader<OPAHSM_A>;
///OPAHSM
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPAHSM_A {
    ///0: OpAmp in normal mode
    Normal = 0,
    ///1: OpAmp in high speed mode
    HighSpeed = 1,
}
impl From<OPAHSM_A> for bool {
    #[inline(always)]
    fn from(variant: OPAHSM_A) -> Self {
        variant as u8 != 0
    }
}
impl OPAHSM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OPAHSM_A {
        match self.bits {
            false => OPAHSM_A::Normal,
            true => OPAHSM_A::HighSpeed,
        }
    }
    ///Checks if the value of the field is `Normal`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == OPAHSM_A::Normal
    }
    ///Checks if the value of the field is `HighSpeed`
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == OPAHSM_A::HighSpeed
    }
}
///Field `OPAHSM` writer - OPAHSM
pub type OPAHSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP4_CSR_SPEC, OPAHSM_A, O>;
impl<'a, const O: u8> OPAHSM_W<'a, O> {
    ///OpAmp in normal mode
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(OPAHSM_A::Normal)
    }
    ///OpAmp in high speed mode
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(OPAHSM_A::HighSpeed)
    }
}
///Field `OPAINTOEN` reader - OPAINTOEN
pub type OPAINTOEN_R = crate::BitReader<OPAINTOEN_A>;
///OPAINTOEN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPAINTOEN_A {
    ///0: Output is connected to the output Pin
    OutputPin = 0,
    ///1: Output is connected internally to ADC channel
    Adcchannel = 1,
}
impl From<OPAINTOEN_A> for bool {
    #[inline(always)]
    fn from(variant: OPAINTOEN_A) -> Self {
        variant as u8 != 0
    }
}
impl OPAINTOEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OPAINTOEN_A {
        match self.bits {
            false => OPAINTOEN_A::OutputPin,
            true => OPAINTOEN_A::Adcchannel,
        }
    }
    ///Checks if the value of the field is `OutputPin`
    #[inline(always)]
    pub fn is_output_pin(&self) -> bool {
        *self == OPAINTOEN_A::OutputPin
    }
    ///Checks if the value of the field is `Adcchannel`
    #[inline(always)]
    pub fn is_adcchannel(&self) -> bool {
        *self == OPAINTOEN_A::Adcchannel
    }
}
///Field `OPAINTOEN` writer - OPAINTOEN
pub type OPAINTOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP4_CSR_SPEC, OPAINTOEN_A, O>;
impl<'a, const O: u8> OPAINTOEN_W<'a, O> {
    ///Output is connected to the output Pin
    #[inline(always)]
    pub fn output_pin(self) -> &'a mut W {
        self.variant(OPAINTOEN_A::OutputPin)
    }
    ///Output is connected internally to ADC channel
    #[inline(always)]
    pub fn adcchannel(self) -> &'a mut W {
        self.variant(OPAINTOEN_A::Adcchannel)
    }
}
///Field `CALON` reader - CALON
pub type CALON_R = crate::BitReader<CALON_A>;
///CALON
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALON_A {
    ///0: Calibration mode disabled
    Disabled = 0,
    ///1: Calibration mode enabled
    Enabled = 1,
}
impl From<CALON_A> for bool {
    #[inline(always)]
    fn from(variant: CALON_A) -> Self {
        variant as u8 != 0
    }
}
impl CALON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CALON_A {
        match self.bits {
            false => CALON_A::Disabled,
            true => CALON_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CALON_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CALON_A::Enabled
    }
}
///Field `CALON` writer - CALON
pub type CALON_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP4_CSR_SPEC, CALON_A, O>;
impl<'a, const O: u8> CALON_W<'a, O> {
    ///Calibration mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CALON_A::Disabled)
    }
    ///Calibration mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CALON_A::Enabled)
    }
}
///Field `CALSEL` reader - CALSEL
pub type CALSEL_R = crate::FieldReader<u8, CALSEL_A>;
///CALSEL
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CALSEL_A {
    ///0: 0.033*VDDA applied to OPAMP inputs during calibration
    Percent33 = 0,
    ///1: 0.1*VDDA applied to OPAMP inputs during calibration
    Percent10 = 1,
    ///2: 0.5*VDDA applied to OPAMP inputs during calibration
    Percent50 = 2,
    ///3: 0.9*VDDA applied to OPAMP inputs during calibration
    Percent90 = 3,
}
impl From<CALSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CALSEL_A) -> Self {
        variant as _
    }
}
impl CALSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CALSEL_A {
        match self.bits {
            0 => CALSEL_A::Percent33,
            1 => CALSEL_A::Percent10,
            2 => CALSEL_A::Percent50,
            3 => CALSEL_A::Percent90,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Percent33`
    #[inline(always)]
    pub fn is_percent3_3(&self) -> bool {
        *self == CALSEL_A::Percent33
    }
    ///Checks if the value of the field is `Percent10`
    #[inline(always)]
    pub fn is_percent10(&self) -> bool {
        *self == CALSEL_A::Percent10
    }
    ///Checks if the value of the field is `Percent50`
    #[inline(always)]
    pub fn is_percent50(&self) -> bool {
        *self == CALSEL_A::Percent50
    }
    ///Checks if the value of the field is `Percent90`
    #[inline(always)]
    pub fn is_percent90(&self) -> bool {
        *self == CALSEL_A::Percent90
    }
}
///Field `CALSEL` writer - CALSEL
pub type CALSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OPAMP4_CSR_SPEC, u8, CALSEL_A, 2, O>;
impl<'a, const O: u8> CALSEL_W<'a, O> {
    ///0.033*VDDA applied to OPAMP inputs during calibration
    #[inline(always)]
    pub fn percent3_3(self) -> &'a mut W {
        self.variant(CALSEL_A::Percent33)
    }
    ///0.1*VDDA applied to OPAMP inputs during calibration
    #[inline(always)]
    pub fn percent10(self) -> &'a mut W {
        self.variant(CALSEL_A::Percent10)
    }
    ///0.5*VDDA applied to OPAMP inputs during calibration
    #[inline(always)]
    pub fn percent50(self) -> &'a mut W {
        self.variant(CALSEL_A::Percent50)
    }
    ///0.9*VDDA applied to OPAMP inputs during calibration
    #[inline(always)]
    pub fn percent90(self) -> &'a mut W {
        self.variant(CALSEL_A::Percent90)
    }
}
///Field `PGA_GAIN` reader - PGA_GAIN
pub type PGA_GAIN_R = crate::FieldReader<u8, PGA_GAIN_A>;
///PGA_GAIN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PGA_GAIN_A {
    ///0: Gain 2
    Gain2 = 0,
    ///1: Gain 4
    Gain4 = 1,
    ///2: Gain 8
    Gain8 = 2,
    ///3: Gain 16
    Gain16 = 3,
    ///4: Gain 32
    Gain32 = 4,
    ///5: Gain 64
    Gain64 = 5,
    ///8: Gain 2, input/bias connected to VINM0 or inverting gain
    Gain2InputVinm0 = 8,
    ///9: Gain 4, input/bias connected to VINM0 or inverting gain
    Gain4InputVinm0 = 9,
    ///10: Gain 8, input/bias connected to VINM0 or inverting gain
    Gain8InputVinm0 = 10,
    ///11: Gain 16, input/bias connected to VINM0 or inverting gain
    Gain16InputVinm0 = 11,
    ///12: Gain 32, input/bias connected to VINM0 or inverting gain
    Gain32InputVinm0 = 12,
    ///13: Gain 64, input/bias connected to VINM0 or inverting gain
    Gain64InputVinm0 = 13,
    ///16: Gain 2, with filtering on VINM0
    Gain2FilteringVinm0 = 16,
    ///17: Gain 4, with filtering on VINM0
    Gain4FilteringVinm0 = 17,
    ///18: Gain 8, with filtering on VINM0
    Gain8FilteringVinm0 = 18,
    ///19: Gain 16, with filtering on VINM0
    Gain16FilteringVinm0 = 19,
    ///20: Gain 32, with filtering on VINM0
    Gain32FilteringVinm0 = 20,
    ///21: Gain 64, with filtering on VINM0
    Gain64FilteringVinm0 = 21,
    ///24: Gain 2, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
    Gain2InputVinm0filteringVinm1 = 24,
    ///25: Gain 4, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
    Gain4InputVinm0filteringVinm1 = 25,
    ///26: Gain 8, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
    Gain8InputVinm0filteringVinm1 = 26,
    ///27: Gain 16, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
    Gain16InputVinm0filteringVinm1 = 27,
    ///28: Gain 32, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
    Gain32InputVinm0filteringVinm1 = 28,
    ///29: Gain 64, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
    Gain64InputVinm0filteringVinm1 = 29,
}
impl From<PGA_GAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: PGA_GAIN_A) -> Self {
        variant as _
    }
}
impl PGA_GAIN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PGA_GAIN_A> {
        match self.bits {
            0 => Some(PGA_GAIN_A::Gain2),
            1 => Some(PGA_GAIN_A::Gain4),
            2 => Some(PGA_GAIN_A::Gain8),
            3 => Some(PGA_GAIN_A::Gain16),
            4 => Some(PGA_GAIN_A::Gain32),
            5 => Some(PGA_GAIN_A::Gain64),
            8 => Some(PGA_GAIN_A::Gain2InputVinm0),
            9 => Some(PGA_GAIN_A::Gain4InputVinm0),
            10 => Some(PGA_GAIN_A::Gain8InputVinm0),
            11 => Some(PGA_GAIN_A::Gain16InputVinm0),
            12 => Some(PGA_GAIN_A::Gain32InputVinm0),
            13 => Some(PGA_GAIN_A::Gain64InputVinm0),
            16 => Some(PGA_GAIN_A::Gain2FilteringVinm0),
            17 => Some(PGA_GAIN_A::Gain4FilteringVinm0),
            18 => Some(PGA_GAIN_A::Gain8FilteringVinm0),
            19 => Some(PGA_GAIN_A::Gain16FilteringVinm0),
            20 => Some(PGA_GAIN_A::Gain32FilteringVinm0),
            21 => Some(PGA_GAIN_A::Gain64FilteringVinm0),
            24 => Some(PGA_GAIN_A::Gain2InputVinm0filteringVinm1),
            25 => Some(PGA_GAIN_A::Gain4InputVinm0filteringVinm1),
            26 => Some(PGA_GAIN_A::Gain8InputVinm0filteringVinm1),
            27 => Some(PGA_GAIN_A::Gain16InputVinm0filteringVinm1),
            28 => Some(PGA_GAIN_A::Gain32InputVinm0filteringVinm1),
            29 => Some(PGA_GAIN_A::Gain64InputVinm0filteringVinm1),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Gain2`
    #[inline(always)]
    pub fn is_gain2(&self) -> bool {
        *self == PGA_GAIN_A::Gain2
    }
    ///Checks if the value of the field is `Gain4`
    #[inline(always)]
    pub fn is_gain4(&self) -> bool {
        *self == PGA_GAIN_A::Gain4
    }
    ///Checks if the value of the field is `Gain8`
    #[inline(always)]
    pub fn is_gain8(&self) -> bool {
        *self == PGA_GAIN_A::Gain8
    }
    ///Checks if the value of the field is `Gain16`
    #[inline(always)]
    pub fn is_gain16(&self) -> bool {
        *self == PGA_GAIN_A::Gain16
    }
    ///Checks if the value of the field is `Gain32`
    #[inline(always)]
    pub fn is_gain32(&self) -> bool {
        *self == PGA_GAIN_A::Gain32
    }
    ///Checks if the value of the field is `Gain64`
    #[inline(always)]
    pub fn is_gain64(&self) -> bool {
        *self == PGA_GAIN_A::Gain64
    }
    ///Checks if the value of the field is `Gain2InputVinm0`
    #[inline(always)]
    pub fn is_gain2_input_vinm0(&self) -> bool {
        *self == PGA_GAIN_A::Gain2InputVinm0
    }
    ///Checks if the value of the field is `Gain4InputVinm0`
    #[inline(always)]
    pub fn is_gain4_input_vinm0(&self) -> bool {
        *self == PGA_GAIN_A::Gain4InputVinm0
    }
    ///Checks if the value of the field is `Gain8InputVinm0`
    #[inline(always)]
    pub fn is_gain8_input_vinm0(&self) -> bool {
        *self == PGA_GAIN_A::Gain8InputVinm0
    }
    ///Checks if the value of the field is `Gain16InputVinm0`
    #[inline(always)]
    pub fn is_gain16_input_vinm0(&self) -> bool {
        *self == PGA_GAIN_A::Gain16InputVinm0
    }
    ///Checks if the value of the field is `Gain32InputVinm0`
    #[inline(always)]
    pub fn is_gain32_input_vinm0(&self) -> bool {
        *self == PGA_GAIN_A::Gain32InputVinm0
    }
    ///Checks if the value of the field is `Gain64InputVinm0`
    #[inline(always)]
    pub fn is_gain64_input_vinm0(&self) -> bool {
        *self == PGA_GAIN_A::Gain64InputVinm0
    }
    ///Checks if the value of the field is `Gain2FilteringVinm0`
    #[inline(always)]
    pub fn is_gain2_filtering_vinm0(&self) -> bool {
        *self == PGA_GAIN_A::Gain2FilteringVinm0
    }
    ///Checks if the value of the field is `Gain4FilteringVinm0`
    #[inline(always)]
    pub fn is_gain4_filtering_vinm0(&self) -> bool {
        *self == PGA_GAIN_A::Gain4FilteringVinm0
    }
    ///Checks if the value of the field is `Gain8FilteringVinm0`
    #[inline(always)]
    pub fn is_gain8_filtering_vinm0(&self) -> bool {
        *self == PGA_GAIN_A::Gain8FilteringVinm0
    }
    ///Checks if the value of the field is `Gain16FilteringVinm0`
    #[inline(always)]
    pub fn is_gain16_filtering_vinm0(&self) -> bool {
        *self == PGA_GAIN_A::Gain16FilteringVinm0
    }
    ///Checks if the value of the field is `Gain32FilteringVinm0`
    #[inline(always)]
    pub fn is_gain32_filtering_vinm0(&self) -> bool {
        *self == PGA_GAIN_A::Gain32FilteringVinm0
    }
    ///Checks if the value of the field is `Gain64FilteringVinm0`
    #[inline(always)]
    pub fn is_gain64_filtering_vinm0(&self) -> bool {
        *self == PGA_GAIN_A::Gain64FilteringVinm0
    }
    ///Checks if the value of the field is `Gain2InputVinm0filteringVinm1`
    #[inline(always)]
    pub fn is_gain2_input_vinm0filtering_vinm1(&self) -> bool {
        *self == PGA_GAIN_A::Gain2InputVinm0filteringVinm1
    }
    ///Checks if the value of the field is `Gain4InputVinm0filteringVinm1`
    #[inline(always)]
    pub fn is_gain4_input_vinm0filtering_vinm1(&self) -> bool {
        *self == PGA_GAIN_A::Gain4InputVinm0filteringVinm1
    }
    ///Checks if the value of the field is `Gain8InputVinm0filteringVinm1`
    #[inline(always)]
    pub fn is_gain8_input_vinm0filtering_vinm1(&self) -> bool {
        *self == PGA_GAIN_A::Gain8InputVinm0filteringVinm1
    }
    ///Checks if the value of the field is `Gain16InputVinm0filteringVinm1`
    #[inline(always)]
    pub fn is_gain16_input_vinm0filtering_vinm1(&self) -> bool {
        *self == PGA_GAIN_A::Gain16InputVinm0filteringVinm1
    }
    ///Checks if the value of the field is `Gain32InputVinm0filteringVinm1`
    #[inline(always)]
    pub fn is_gain32_input_vinm0filtering_vinm1(&self) -> bool {
        *self == PGA_GAIN_A::Gain32InputVinm0filteringVinm1
    }
    ///Checks if the value of the field is `Gain64InputVinm0filteringVinm1`
    #[inline(always)]
    pub fn is_gain64_input_vinm0filtering_vinm1(&self) -> bool {
        *self == PGA_GAIN_A::Gain64InputVinm0filteringVinm1
    }
}
///Field `PGA_GAIN` writer - PGA_GAIN
pub type PGA_GAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPAMP4_CSR_SPEC, u8, PGA_GAIN_A, 5, O>;
impl<'a, const O: u8> PGA_GAIN_W<'a, O> {
    ///Gain 2
    #[inline(always)]
    pub fn gain2(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain2)
    }
    ///Gain 4
    #[inline(always)]
    pub fn gain4(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain4)
    }
    ///Gain 8
    #[inline(always)]
    pub fn gain8(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain8)
    }
    ///Gain 16
    #[inline(always)]
    pub fn gain16(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain16)
    }
    ///Gain 32
    #[inline(always)]
    pub fn gain32(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain32)
    }
    ///Gain 64
    #[inline(always)]
    pub fn gain64(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain64)
    }
    ///Gain 2, input/bias connected to VINM0 or inverting gain
    #[inline(always)]
    pub fn gain2_input_vinm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain2InputVinm0)
    }
    ///Gain 4, input/bias connected to VINM0 or inverting gain
    #[inline(always)]
    pub fn gain4_input_vinm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain4InputVinm0)
    }
    ///Gain 8, input/bias connected to VINM0 or inverting gain
    #[inline(always)]
    pub fn gain8_input_vinm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain8InputVinm0)
    }
    ///Gain 16, input/bias connected to VINM0 or inverting gain
    #[inline(always)]
    pub fn gain16_input_vinm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain16InputVinm0)
    }
    ///Gain 32, input/bias connected to VINM0 or inverting gain
    #[inline(always)]
    pub fn gain32_input_vinm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain32InputVinm0)
    }
    ///Gain 64, input/bias connected to VINM0 or inverting gain
    #[inline(always)]
    pub fn gain64_input_vinm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain64InputVinm0)
    }
    ///Gain 2, with filtering on VINM0
    #[inline(always)]
    pub fn gain2_filtering_vinm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain2FilteringVinm0)
    }
    ///Gain 4, with filtering on VINM0
    #[inline(always)]
    pub fn gain4_filtering_vinm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain4FilteringVinm0)
    }
    ///Gain 8, with filtering on VINM0
    #[inline(always)]
    pub fn gain8_filtering_vinm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain8FilteringVinm0)
    }
    ///Gain 16, with filtering on VINM0
    #[inline(always)]
    pub fn gain16_filtering_vinm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain16FilteringVinm0)
    }
    ///Gain 32, with filtering on VINM0
    #[inline(always)]
    pub fn gain32_filtering_vinm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain32FilteringVinm0)
    }
    ///Gain 64, with filtering on VINM0
    #[inline(always)]
    pub fn gain64_filtering_vinm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain64FilteringVinm0)
    }
    ///Gain 2, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
    #[inline(always)]
    pub fn gain2_input_vinm0filtering_vinm1(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain2InputVinm0filteringVinm1)
    }
    ///Gain 4, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
    #[inline(always)]
    pub fn gain4_input_vinm0filtering_vinm1(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain4InputVinm0filteringVinm1)
    }
    ///Gain 8, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
    #[inline(always)]
    pub fn gain8_input_vinm0filtering_vinm1(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain8InputVinm0filteringVinm1)
    }
    ///Gain 16, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
    #[inline(always)]
    pub fn gain16_input_vinm0filtering_vinm1(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain16InputVinm0filteringVinm1)
    }
    ///Gain 32, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
    #[inline(always)]
    pub fn gain32_input_vinm0filtering_vinm1(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain32InputVinm0filteringVinm1)
    }
    ///Gain 64, input/bias connected to VINM0 with filtering on VINM1 or inverting gain
    #[inline(always)]
    pub fn gain64_input_vinm0filtering_vinm1(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain64InputVinm0filteringVinm1)
    }
}
///Field `TRIMOFFSETP` reader - TRIMOFFSETP
pub type TRIMOFFSETP_R = crate::FieldReader<u8, u8>;
///Field `TRIMOFFSETP` writer - TRIMOFFSETP
pub type TRIMOFFSETP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OPAMP4_CSR_SPEC, u8, u8, 5, O>;
///Field `TRIMOFFSETN` reader - TRIMOFFSETN
pub type TRIMOFFSETN_R = crate::FieldReader<u8, u8>;
///Field `TRIMOFFSETN` writer - TRIMOFFSETN
pub type TRIMOFFSETN_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OPAMP4_CSR_SPEC, u8, u8, 5, O>;
///Field `CALOUT` reader - CALOUT
pub type CALOUT_R = crate::BitReader<bool>;
///Field `CALOUT` writer - CALOUT
pub type CALOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP4_CSR_SPEC, bool, O>;
///Field `LOCK` reader - LOCK
pub type LOCK_R = crate::BitReader<LOCK_A>;
///LOCK
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_A {
    ///0: CSR is read-write
    ReadWrite = 0,
    ///1: CSR is read-only, can only be cleared by system reset
    ReadOnly = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::ReadWrite,
            true => LOCK_A::ReadOnly,
        }
    }
    ///Checks if the value of the field is `ReadWrite`
    #[inline(always)]
    pub fn is_read_write(&self) -> bool {
        *self == LOCK_A::ReadWrite
    }
    ///Checks if the value of the field is `ReadOnly`
    #[inline(always)]
    pub fn is_read_only(&self) -> bool {
        *self == LOCK_A::ReadOnly
    }
}
///Field `LOCK` writer - LOCK
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP4_CSR_SPEC, LOCK_A, O>;
impl<'a, const O: u8> LOCK_W<'a, O> {
    ///CSR is read-write
    #[inline(always)]
    pub fn read_write(self) -> &'a mut W {
        self.variant(LOCK_A::ReadWrite)
    }
    ///CSR is read-only, can only be cleared by system reset
    #[inline(always)]
    pub fn read_only(self) -> &'a mut W {
        self.variant(LOCK_A::ReadOnly)
    }
}
impl R {
    ///Bit 0 - Operational amplifier Enable
    #[inline(always)]
    pub fn opaen(&self) -> OPAEN_R {
        OPAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FORCE_VP
    #[inline(always)]
    pub fn force_vp(&self) -> FORCE_VP_R {
        FORCE_VP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - VP_SEL
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - USERTRIM
    #[inline(always)]
    pub fn usertrim(&self) -> USERTRIM_R {
        USERTRIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - VM_SEL
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - OPAHSM
    #[inline(always)]
    pub fn opahsm(&self) -> OPAHSM_R {
        OPAHSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - OPAINTOEN
    #[inline(always)]
    pub fn opaintoen(&self) -> OPAINTOEN_R {
        OPAINTOEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - CALON
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - CALSEL
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:18 - PGA_GAIN
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    ///Bits 19:23 - TRIMOFFSETP
    #[inline(always)]
    pub fn trimoffsetp(&self) -> TRIMOFFSETP_R {
        TRIMOFFSETP_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    ///Bits 24:28 - TRIMOFFSETN
    #[inline(always)]
    pub fn trimoffsetn(&self) -> TRIMOFFSETN_R {
        TRIMOFFSETN_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    ///Bit 30 - CALOUT
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - LOCK
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Operational amplifier Enable
    #[inline(always)]
    #[must_use]
    pub fn opaen(&mut self) -> OPAEN_W<0> {
        OPAEN_W::new(self)
    }
    ///Bit 1 - FORCE_VP
    #[inline(always)]
    #[must_use]
    pub fn force_vp(&mut self) -> FORCE_VP_W<1> {
        FORCE_VP_W::new(self)
    }
    ///Bits 2:3 - VP_SEL
    #[inline(always)]
    #[must_use]
    pub fn vp_sel(&mut self) -> VP_SEL_W<2> {
        VP_SEL_W::new(self)
    }
    ///Bit 4 - USERTRIM
    #[inline(always)]
    #[must_use]
    pub fn usertrim(&mut self) -> USERTRIM_W<4> {
        USERTRIM_W::new(self)
    }
    ///Bits 5:6 - VM_SEL
    #[inline(always)]
    #[must_use]
    pub fn vm_sel(&mut self) -> VM_SEL_W<5> {
        VM_SEL_W::new(self)
    }
    ///Bit 7 - OPAHSM
    #[inline(always)]
    #[must_use]
    pub fn opahsm(&mut self) -> OPAHSM_W<7> {
        OPAHSM_W::new(self)
    }
    ///Bit 8 - OPAINTOEN
    #[inline(always)]
    #[must_use]
    pub fn opaintoen(&mut self) -> OPAINTOEN_W<8> {
        OPAINTOEN_W::new(self)
    }
    ///Bit 11 - CALON
    #[inline(always)]
    #[must_use]
    pub fn calon(&mut self) -> CALON_W<11> {
        CALON_W::new(self)
    }
    ///Bits 12:13 - CALSEL
    #[inline(always)]
    #[must_use]
    pub fn calsel(&mut self) -> CALSEL_W<12> {
        CALSEL_W::new(self)
    }
    ///Bits 14:18 - PGA_GAIN
    #[inline(always)]
    #[must_use]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<14> {
        PGA_GAIN_W::new(self)
    }
    ///Bits 19:23 - TRIMOFFSETP
    #[inline(always)]
    #[must_use]
    pub fn trimoffsetp(&mut self) -> TRIMOFFSETP_W<19> {
        TRIMOFFSETP_W::new(self)
    }
    ///Bits 24:28 - TRIMOFFSETN
    #[inline(always)]
    #[must_use]
    pub fn trimoffsetn(&mut self) -> TRIMOFFSETN_W<24> {
        TRIMOFFSETN_W::new(self)
    }
    ///Bit 30 - CALOUT
    #[inline(always)]
    #[must_use]
    pub fn calout(&mut self) -> CALOUT_W<30> {
        CALOUT_W::new(self)
    }
    ///Bit 31 - LOCK
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<31> {
        LOCK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OPAMP4 control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [opamp4_csr](index.html) module
pub struct OPAMP4_CSR_SPEC;
impl crate::RegisterSpec for OPAMP4_CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [opamp4_csr::R](R) reader structure
impl crate::Readable for OPAMP4_CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [opamp4_csr::W](W) writer structure
impl crate::Writable for OPAMP4_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OPAMP4_CSR to value 0
impl crate::Resettable for OPAMP4_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
