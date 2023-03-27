///Register `OPAMP1_CSR` reader
pub struct R(crate::R<OPAMP1_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAMP1_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAMP1_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAMP1_CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OPAMP1_CSR` writer
pub struct W(crate::W<OPAMP1_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAMP1_CSR_SPEC>;
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
impl From<crate::W<OPAMP1_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAMP1_CSR_SPEC>) -> Self {
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
pub type OPAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP1_CSR_SPEC, OPAEN_A, O>;
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
///Field `OPALPM` reader - Operational amplifier Low Power Mode
pub type OPALPM_R = crate::BitReader<OPALPM_A>;
///Operational amplifier Low Power Mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPALPM_A {
    ///0: OpAmp in normal mode
    Normal = 0,
    ///1: OpAmp in low power mode
    Low = 1,
}
impl From<OPALPM_A> for bool {
    #[inline(always)]
    fn from(variant: OPALPM_A) -> Self {
        variant as u8 != 0
    }
}
impl OPALPM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OPALPM_A {
        match self.bits {
            false => OPALPM_A::Normal,
            true => OPALPM_A::Low,
        }
    }
    ///Checks if the value of the field is `Normal`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == OPALPM_A::Normal
    }
    ///Checks if the value of the field is `Low`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OPALPM_A::Low
    }
}
///Field `OPALPM` writer - Operational amplifier Low Power Mode
pub type OPALPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP1_CSR_SPEC, OPALPM_A, O>;
impl<'a, const O: u8> OPALPM_W<'a, O> {
    ///OpAmp in normal mode
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(OPALPM_A::Normal)
    }
    ///OpAmp in low power mode
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OPALPM_A::Low)
    }
}
///Field `OPAMODE` reader - Operational amplifier PGA mode
pub type OPAMODE_R = crate::FieldReader<u8, OPAMODE_A>;
///Operational amplifier PGA mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAMODE_A {
    ///0: internal PGA diabled
    PgaDisabled = 0,
    ///2: internal PGA enabled, gain programmed in PGA_GAIN
    PgaEnabled = 2,
    ///3: internal follower
    Follower = 3,
}
impl From<OPAMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAMODE_A) -> Self {
        variant as _
    }
}
impl OPAMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAMODE_A> {
        match self.bits {
            0 => Some(OPAMODE_A::PgaDisabled),
            2 => Some(OPAMODE_A::PgaEnabled),
            3 => Some(OPAMODE_A::Follower),
            _ => None,
        }
    }
    ///Checks if the value of the field is `PgaDisabled`
    #[inline(always)]
    pub fn is_pga_disabled(&self) -> bool {
        *self == OPAMODE_A::PgaDisabled
    }
    ///Checks if the value of the field is `PgaEnabled`
    #[inline(always)]
    pub fn is_pga_enabled(&self) -> bool {
        *self == OPAMODE_A::PgaEnabled
    }
    ///Checks if the value of the field is `Follower`
    #[inline(always)]
    pub fn is_follower(&self) -> bool {
        *self == OPAMODE_A::Follower
    }
}
///Field `OPAMODE` writer - Operational amplifier PGA mode
pub type OPAMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPAMP1_CSR_SPEC, u8, OPAMODE_A, 2, O>;
impl<'a, const O: u8> OPAMODE_W<'a, O> {
    ///internal PGA diabled
    #[inline(always)]
    pub fn pga_disabled(self) -> &'a mut W {
        self.variant(OPAMODE_A::PgaDisabled)
    }
    ///internal PGA enabled, gain programmed in PGA_GAIN
    #[inline(always)]
    pub fn pga_enabled(self) -> &'a mut W {
        self.variant(OPAMODE_A::PgaEnabled)
    }
    ///internal follower
    #[inline(always)]
    pub fn follower(self) -> &'a mut W {
        self.variant(OPAMODE_A::Follower)
    }
}
///Field `PGA_GAIN` reader - Operational amplifier Programmable amplifier gain value
pub type PGA_GAIN_R = crate::FieldReader<u8, PGA_GAIN_A>;
///Operational amplifier Programmable amplifier gain value
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
    pub fn variant(&self) -> PGA_GAIN_A {
        match self.bits {
            0 => PGA_GAIN_A::Gain2,
            1 => PGA_GAIN_A::Gain4,
            2 => PGA_GAIN_A::Gain8,
            3 => PGA_GAIN_A::Gain16,
            _ => unreachable!(),
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
}
///Field `PGA_GAIN` writer - Operational amplifier Programmable amplifier gain value
pub type PGA_GAIN_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OPAMP1_CSR_SPEC, u8, PGA_GAIN_A, 2, O>;
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
}
///Field `VM_SEL` reader - Inverting input selection
pub type VM_SEL_R = crate::FieldReader<u8, VM_SEL_A>;
///Inverting input selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VM_SEL_A {
    ///0: GPIO connectet to VINM
    Gpio = 0,
    ///1: Low leakage inputs connecte (only available in certen BGA cases
    LowLeakage = 1,
    ///2: OPAMP in PGA mode
    PgaMode = 2,
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
    pub fn variant(&self) -> Option<VM_SEL_A> {
        match self.bits {
            0 => Some(VM_SEL_A::Gpio),
            1 => Some(VM_SEL_A::LowLeakage),
            2 => Some(VM_SEL_A::PgaMode),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Gpio`
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == VM_SEL_A::Gpio
    }
    ///Checks if the value of the field is `LowLeakage`
    #[inline(always)]
    pub fn is_low_leakage(&self) -> bool {
        *self == VM_SEL_A::LowLeakage
    }
    ///Checks if the value of the field is `PgaMode`
    #[inline(always)]
    pub fn is_pga_mode(&self) -> bool {
        *self == VM_SEL_A::PgaMode
    }
}
///Field `VM_SEL` writer - Inverting input selection
pub type VM_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPAMP1_CSR_SPEC, u8, VM_SEL_A, 2, O>;
impl<'a, const O: u8> VM_SEL_W<'a, O> {
    ///GPIO connectet to VINM
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(VM_SEL_A::Gpio)
    }
    ///Low leakage inputs connecte (only available in certen BGA cases
    #[inline(always)]
    pub fn low_leakage(self) -> &'a mut W {
        self.variant(VM_SEL_A::LowLeakage)
    }
    ///OPAMP in PGA mode
    #[inline(always)]
    pub fn pga_mode(self) -> &'a mut W {
        self.variant(VM_SEL_A::PgaMode)
    }
}
///Field `VP_SEL` reader - Non inverted input selection
pub type VP_SEL_R = crate::BitReader<VP_SEL_A>;
///Non inverted input selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VP_SEL_A {
    ///0: GPIO connectet to VINP
    Gpio = 0,
    ///1: DAC connected to VPINP
    Dac = 1,
}
impl From<VP_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: VP_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl VP_SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VP_SEL_A {
        match self.bits {
            false => VP_SEL_A::Gpio,
            true => VP_SEL_A::Dac,
        }
    }
    ///Checks if the value of the field is `Gpio`
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == VP_SEL_A::Gpio
    }
    ///Checks if the value of the field is `Dac`
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == VP_SEL_A::Dac
    }
}
///Field `VP_SEL` writer - Non inverted input selection
pub type VP_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP1_CSR_SPEC, VP_SEL_A, O>;
impl<'a, const O: u8> VP_SEL_W<'a, O> {
    ///GPIO connectet to VINP
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(VP_SEL_A::Gpio)
    }
    ///DAC connected to VPINP
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(VP_SEL_A::Dac)
    }
}
///Field `CALON` reader - Calibration mode enabled
pub type CALON_R = crate::BitReader<CALON_A>;
///Calibration mode enabled
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALON_A {
    ///0: Normal mode
    Disabled = 0,
    ///1: Calibration mode
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
///Field `CALON` writer - Calibration mode enabled
pub type CALON_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP1_CSR_SPEC, CALON_A, O>;
impl<'a, const O: u8> CALON_W<'a, O> {
    ///Normal mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CALON_A::Disabled)
    }
    ///Calibration mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CALON_A::Enabled)
    }
}
///Field `CALSEL` reader - Calibration selection
pub type CALSEL_R = crate::BitReader<CALSEL_A>;
///Calibration selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALSEL_A {
    ///0: 0.2V applied to OPAMP inputs during calibration
    Nmos = 0,
    ///1: VDDA-0.2V applied to OPAMP inputs during calibration"
    Pmos = 1,
}
impl From<CALSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CALSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CALSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CALSEL_A {
        match self.bits {
            false => CALSEL_A::Nmos,
            true => CALSEL_A::Pmos,
        }
    }
    ///Checks if the value of the field is `Nmos`
    #[inline(always)]
    pub fn is_nmos(&self) -> bool {
        *self == CALSEL_A::Nmos
    }
    ///Checks if the value of the field is `Pmos`
    #[inline(always)]
    pub fn is_pmos(&self) -> bool {
        *self == CALSEL_A::Pmos
    }
}
///Field `CALSEL` writer - Calibration selection
pub type CALSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP1_CSR_SPEC, CALSEL_A, O>;
impl<'a, const O: u8> CALSEL_W<'a, O> {
    ///0.2V applied to OPAMP inputs during calibration
    #[inline(always)]
    pub fn nmos(self) -> &'a mut W {
        self.variant(CALSEL_A::Nmos)
    }
    ///VDDA-0.2V applied to OPAMP inputs during calibration"
    #[inline(always)]
    pub fn pmos(self) -> &'a mut W {
        self.variant(CALSEL_A::Pmos)
    }
}
///Field `USERTRIM` reader - allows to switch from AOP offset trimmed values to AOP offset
pub type USERTRIM_R = crate::BitReader<USERTRIM_A>;
///allows to switch from AOP offset trimmed values to AOP offset
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
///Field `USERTRIM` writer - allows to switch from AOP offset trimmed values to AOP offset
pub type USERTRIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP1_CSR_SPEC, USERTRIM_A, O>;
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
///Field `CALOUT` reader - Operational amplifier calibration output
pub type CALOUT_R = crate::BitReader<bool>;
///Field `CALOUT` writer - Operational amplifier calibration output
pub type CALOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP1_CSR_SPEC, bool, O>;
///Field `OPA_RANGE` reader - Operational amplifier power supply range for stability
pub type OPA_RANGE_R = crate::BitReader<OPA_RANGE_A>;
///Operational amplifier power supply range for stability
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPA_RANGE_A {
    ///0: low range (VDDA &lt; 2.4V
    Low = 0,
    ///1: low range (VDDA >2.4V
    High = 1,
}
impl From<OPA_RANGE_A> for bool {
    #[inline(always)]
    fn from(variant: OPA_RANGE_A) -> Self {
        variant as u8 != 0
    }
}
impl OPA_RANGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OPA_RANGE_A {
        match self.bits {
            false => OPA_RANGE_A::Low,
            true => OPA_RANGE_A::High,
        }
    }
    ///Checks if the value of the field is `Low`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OPA_RANGE_A::Low
    }
    ///Checks if the value of the field is `High`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OPA_RANGE_A::High
    }
}
///Field `OPA_RANGE` writer - Operational amplifier power supply range for stability
pub type OPA_RANGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP1_CSR_SPEC, OPA_RANGE_A, O>;
impl<'a, const O: u8> OPA_RANGE_W<'a, O> {
    ///low range (VDDA &lt; 2.4V
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OPA_RANGE_A::Low)
    }
    ///low range (VDDA >2.4V
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OPA_RANGE_A::High)
    }
}
impl R {
    ///Bit 0 - Operational amplifier Enable
    #[inline(always)]
    pub fn opaen(&self) -> OPAEN_R {
        OPAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Operational amplifier Low Power Mode
    #[inline(always)]
    pub fn opalpm(&self) -> OPALPM_R {
        OPALPM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Operational amplifier PGA mode
    #[inline(always)]
    pub fn opamode(&self) -> OPAMODE_R {
        OPAMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Operational amplifier Programmable amplifier gain value
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:9 - Inverting input selection
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Non inverted input selection
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - Calibration mode enabled
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Calibration selection
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - allows to switch from AOP offset trimmed values to AOP offset
    #[inline(always)]
    pub fn usertrim(&self) -> USERTRIM_R {
        USERTRIM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Operational amplifier calibration output
    #[inline(always)]
    pub fn calout(&self) -> CALOUT_R {
        CALOUT_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 31 - Operational amplifier power supply range for stability
    #[inline(always)]
    pub fn opa_range(&self) -> OPA_RANGE_R {
        OPA_RANGE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Operational amplifier Enable
    #[inline(always)]
    #[must_use]
    pub fn opaen(&mut self) -> OPAEN_W<0> {
        OPAEN_W::new(self)
    }
    ///Bit 1 - Operational amplifier Low Power Mode
    #[inline(always)]
    #[must_use]
    pub fn opalpm(&mut self) -> OPALPM_W<1> {
        OPALPM_W::new(self)
    }
    ///Bits 2:3 - Operational amplifier PGA mode
    #[inline(always)]
    #[must_use]
    pub fn opamode(&mut self) -> OPAMODE_W<2> {
        OPAMODE_W::new(self)
    }
    ///Bits 4:5 - Operational amplifier Programmable amplifier gain value
    #[inline(always)]
    #[must_use]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<4> {
        PGA_GAIN_W::new(self)
    }
    ///Bits 8:9 - Inverting input selection
    #[inline(always)]
    #[must_use]
    pub fn vm_sel(&mut self) -> VM_SEL_W<8> {
        VM_SEL_W::new(self)
    }
    ///Bit 10 - Non inverted input selection
    #[inline(always)]
    #[must_use]
    pub fn vp_sel(&mut self) -> VP_SEL_W<10> {
        VP_SEL_W::new(self)
    }
    ///Bit 12 - Calibration mode enabled
    #[inline(always)]
    #[must_use]
    pub fn calon(&mut self) -> CALON_W<12> {
        CALON_W::new(self)
    }
    ///Bit 13 - Calibration selection
    #[inline(always)]
    #[must_use]
    pub fn calsel(&mut self) -> CALSEL_W<13> {
        CALSEL_W::new(self)
    }
    ///Bit 14 - allows to switch from AOP offset trimmed values to AOP offset
    #[inline(always)]
    #[must_use]
    pub fn usertrim(&mut self) -> USERTRIM_W<14> {
        USERTRIM_W::new(self)
    }
    ///Bit 15 - Operational amplifier calibration output
    #[inline(always)]
    #[must_use]
    pub fn calout(&mut self) -> CALOUT_W<15> {
        CALOUT_W::new(self)
    }
    ///Bit 31 - Operational amplifier power supply range for stability
    #[inline(always)]
    #[must_use]
    pub fn opa_range(&mut self) -> OPA_RANGE_W<31> {
        OPA_RANGE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OPAMP1 control/status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [opamp1_csr](index.html) module
pub struct OPAMP1_CSR_SPEC;
impl crate::RegisterSpec for OPAMP1_CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [opamp1_csr::R](R) reader structure
impl crate::Readable for OPAMP1_CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [opamp1_csr::W](W) writer structure
impl crate::Writable for OPAMP1_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OPAMP1_CSR to value 0
impl crate::Resettable for OPAMP1_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
