///Register `OPAMP2_CSR` reader
pub struct R(crate::R<OPAMP2_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAMP2_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAMP2_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAMP2_CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OPAMP2_CSR` writer
pub struct W(crate::W<OPAMP2_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAMP2_CSR_SPEC>;
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
impl From<crate::W<OPAMP2_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAMP2_CSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OPAMP2EN` reader - OPAMP2 enable
pub type OPAMP2EN_R = crate::BitReader<OPAMP2EN_A>;
///OPAMP2 enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPAMP2EN_A {
    ///0: OPAMP2 is disabled
    Disabled = 0,
    ///1: OPAMP2 is enabled
    Enabled = 1,
}
impl From<OPAMP2EN_A> for bool {
    #[inline(always)]
    fn from(variant: OPAMP2EN_A) -> Self {
        variant as u8 != 0
    }
}
impl OPAMP2EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OPAMP2EN_A {
        match self.bits {
            false => OPAMP2EN_A::Disabled,
            true => OPAMP2EN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OPAMP2EN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OPAMP2EN_A::Enabled
    }
}
///Field `OPAMP2EN` writer - OPAMP2 enable
pub type OPAMP2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CSR_SPEC, OPAMP2EN_A, O>;
impl<'a, const O: u8> OPAMP2EN_W<'a, O> {
    ///OPAMP2 is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OPAMP2EN_A::Disabled)
    }
    ///OPAMP2 is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OPAMP2EN_A::Enabled)
    }
}
///Field `FORCE_VP` reader - FORCE_VP
pub type FORCE_VP_R = crate::BitReader<FORCE_VP_A>;
///FORCE_VP
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORCE_VP_A {
    ///0: Normal operating mode
    Normal = 0,
    ///1: Calibration mode. Non-inverting input connected to calibration reference
    Calibration = 1,
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
            true => FORCE_VP_A::Calibration,
        }
    }
    ///Checks if the value of the field is `Normal`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == FORCE_VP_A::Normal
    }
    ///Checks if the value of the field is `Calibration`
    #[inline(always)]
    pub fn is_calibration(&self) -> bool {
        *self == FORCE_VP_A::Calibration
    }
}
///Field `FORCE_VP` writer - FORCE_VP
pub type FORCE_VP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CSR_SPEC, FORCE_VP_A, O>;
impl<'a, const O: u8> FORCE_VP_W<'a, O> {
    ///Normal operating mode
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(FORCE_VP_A::Normal)
    }
    ///Calibration mode. Non-inverting input connected to calibration reference
    #[inline(always)]
    pub fn calibration(self) -> &'a mut W {
        self.variant(FORCE_VP_A::Calibration)
    }
}
///Field `VP_SEL` reader - OPAMP Non inverting input selection
pub type VP_SEL_R = crate::FieldReader<u8, VP_SEL_A>;
///OPAMP Non inverting input selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VP_SEL_A {
    ///1: PB14 used as OPAMP2 non-inverting input
    Pb14 = 1,
    ///2: PB0 used as OPAMP2 non-inverting input
    Pb0 = 2,
    ///3: PA7 used as OPAMP2 non-inverting input
    Pa7 = 3,
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
    pub fn variant(&self) -> Option<VP_SEL_A> {
        match self.bits {
            1 => Some(VP_SEL_A::Pb14),
            2 => Some(VP_SEL_A::Pb0),
            3 => Some(VP_SEL_A::Pa7),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Pb14`
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        *self == VP_SEL_A::Pb14
    }
    ///Checks if the value of the field is `Pb0`
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == VP_SEL_A::Pb0
    }
    ///Checks if the value of the field is `Pa7`
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        *self == VP_SEL_A::Pa7
    }
}
///Field `VP_SEL` writer - OPAMP Non inverting input selection
pub type VP_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPAMP2_CSR_SPEC, u8, VP_SEL_A, 2, O>;
impl<'a, const O: u8> VP_SEL_W<'a, O> {
    ///PB14 used as OPAMP2 non-inverting input
    #[inline(always)]
    pub fn pb14(self) -> &'a mut W {
        self.variant(VP_SEL_A::Pb14)
    }
    ///PB0 used as OPAMP2 non-inverting input
    #[inline(always)]
    pub fn pb0(self) -> &'a mut W {
        self.variant(VP_SEL_A::Pb0)
    }
    ///PA7 used as OPAMP2 non-inverting input
    #[inline(always)]
    pub fn pa7(self) -> &'a mut W {
        self.variant(VP_SEL_A::Pa7)
    }
}
///Field `VM_SEL` reader - OPAMP inverting input selection
pub type VM_SEL_R = crate::FieldReader<u8, VM_SEL_A>;
///OPAMP inverting input selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VM_SEL_A {
    ///0: PC5 (VM0) used as OPAMP2 inverting input
    Pc5 = 0,
    ///1: PA5 (VM1) used as OPAMP2 inverting input
    Pa5 = 1,
    ///2: Resistor feedback output (PGA mode)
    Pga = 2,
    ///3: Follower mode
    Follower = 3,
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
            0 => VM_SEL_A::Pc5,
            1 => VM_SEL_A::Pa5,
            2 => VM_SEL_A::Pga,
            3 => VM_SEL_A::Follower,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Pc5`
    #[inline(always)]
    pub fn is_pc5(&self) -> bool {
        *self == VM_SEL_A::Pc5
    }
    ///Checks if the value of the field is `Pa5`
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == VM_SEL_A::Pa5
    }
    ///Checks if the value of the field is `Pga`
    #[inline(always)]
    pub fn is_pga(&self) -> bool {
        *self == VM_SEL_A::Pga
    }
    ///Checks if the value of the field is `Follower`
    #[inline(always)]
    pub fn is_follower(&self) -> bool {
        *self == VM_SEL_A::Follower
    }
}
///Field `VM_SEL` writer - OPAMP inverting input selection
pub type VM_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OPAMP2_CSR_SPEC, u8, VM_SEL_A, 2, O>;
impl<'a, const O: u8> VM_SEL_W<'a, O> {
    ///PC5 (VM0) used as OPAMP2 inverting input
    #[inline(always)]
    pub fn pc5(self) -> &'a mut W {
        self.variant(VM_SEL_A::Pc5)
    }
    ///PA5 (VM1) used as OPAMP2 inverting input
    #[inline(always)]
    pub fn pa5(self) -> &'a mut W {
        self.variant(VM_SEL_A::Pa5)
    }
    ///Resistor feedback output (PGA mode)
    #[inline(always)]
    pub fn pga(self) -> &'a mut W {
        self.variant(VM_SEL_A::Pga)
    }
    ///Follower mode
    #[inline(always)]
    pub fn follower(self) -> &'a mut W {
        self.variant(VM_SEL_A::Follower)
    }
}
///Field `TCM_EN` reader - Timer controlled Mux mode enable
pub type TCM_EN_R = crate::BitReader<TCM_EN_A>;
///Timer controlled Mux mode enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCM_EN_A {
    ///0: Timer controlled mux disabled
    Disabled = 0,
    ///1: Timer controlled mux enabled
    Enabled = 1,
}
impl From<TCM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TCM_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TCM_EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TCM_EN_A {
        match self.bits {
            false => TCM_EN_A::Disabled,
            true => TCM_EN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCM_EN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCM_EN_A::Enabled
    }
}
///Field `TCM_EN` writer - Timer controlled Mux mode enable
pub type TCM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CSR_SPEC, TCM_EN_A, O>;
impl<'a, const O: u8> TCM_EN_W<'a, O> {
    ///Timer controlled mux disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCM_EN_A::Disabled)
    }
    ///Timer controlled mux enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCM_EN_A::Enabled)
    }
}
///Field `VMS_SEL` reader - OPAMP inverting input secondary selection
pub type VMS_SEL_R = crate::BitReader<VMS_SEL_A>;
///OPAMP inverting input secondary selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VMS_SEL_A {
    ///0: PC5 (VM0) used as OPAMP2 inverting input when TCM_EN=1
    Pc5 = 0,
    ///1: PA5 (VM1) used as OPAMP2 inverting input when TCM_EN=1
    Pa5 = 1,
}
impl From<VMS_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: VMS_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl VMS_SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VMS_SEL_A {
        match self.bits {
            false => VMS_SEL_A::Pc5,
            true => VMS_SEL_A::Pa5,
        }
    }
    ///Checks if the value of the field is `Pc5`
    #[inline(always)]
    pub fn is_pc5(&self) -> bool {
        *self == VMS_SEL_A::Pc5
    }
    ///Checks if the value of the field is `Pa5`
    #[inline(always)]
    pub fn is_pa5(&self) -> bool {
        *self == VMS_SEL_A::Pa5
    }
}
///Field `VMS_SEL` writer - OPAMP inverting input secondary selection
pub type VMS_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CSR_SPEC, VMS_SEL_A, O>;
impl<'a, const O: u8> VMS_SEL_W<'a, O> {
    ///PC5 (VM0) used as OPAMP2 inverting input when TCM_EN=1
    #[inline(always)]
    pub fn pc5(self) -> &'a mut W {
        self.variant(VMS_SEL_A::Pc5)
    }
    ///PA5 (VM1) used as OPAMP2 inverting input when TCM_EN=1
    #[inline(always)]
    pub fn pa5(self) -> &'a mut W {
        self.variant(VMS_SEL_A::Pa5)
    }
}
///Field `VPS_SEL` reader - OPAMP Non inverting input secondary selection
pub type VPS_SEL_R = crate::FieldReader<u8, VPS_SEL_A>;
///OPAMP Non inverting input secondary selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VPS_SEL_A {
    ///1: PB14 used as OPAMP2 non-inverting input when TCM_EN=1
    Pb14 = 1,
    ///2: PB0 used as OPAMP2 non-inverting input when TCM_EN=1
    Pb0 = 2,
    ///3: PA7 used as OPAMP2 non-inverting input when TCM_EN=1
    Pa7 = 3,
}
impl From<VPS_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VPS_SEL_A) -> Self {
        variant as _
    }
}
impl VPS_SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<VPS_SEL_A> {
        match self.bits {
            1 => Some(VPS_SEL_A::Pb14),
            2 => Some(VPS_SEL_A::Pb0),
            3 => Some(VPS_SEL_A::Pa7),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Pb14`
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        *self == VPS_SEL_A::Pb14
    }
    ///Checks if the value of the field is `Pb0`
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == VPS_SEL_A::Pb0
    }
    ///Checks if the value of the field is `Pa7`
    #[inline(always)]
    pub fn is_pa7(&self) -> bool {
        *self == VPS_SEL_A::Pa7
    }
}
///Field `VPS_SEL` writer - OPAMP Non inverting input secondary selection
pub type VPS_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPAMP2_CSR_SPEC, u8, VPS_SEL_A, 2, O>;
impl<'a, const O: u8> VPS_SEL_W<'a, O> {
    ///PB14 used as OPAMP2 non-inverting input when TCM_EN=1
    #[inline(always)]
    pub fn pb14(self) -> &'a mut W {
        self.variant(VPS_SEL_A::Pb14)
    }
    ///PB0 used as OPAMP2 non-inverting input when TCM_EN=1
    #[inline(always)]
    pub fn pb0(self) -> &'a mut W {
        self.variant(VPS_SEL_A::Pb0)
    }
    ///PA7 used as OPAMP2 non-inverting input when TCM_EN=1
    #[inline(always)]
    pub fn pa7(self) -> &'a mut W {
        self.variant(VPS_SEL_A::Pa7)
    }
}
///Field `CALON` reader - Calibration mode enable
pub type CALON_R = crate::BitReader<CALON_A>;
///Calibration mode enable
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
///Field `CALON` writer - Calibration mode enable
pub type CALON_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CSR_SPEC, CALON_A, O>;
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
///Field `CALSEL` reader - Calibration selection
pub type CALSEL_R = crate::FieldReader<u8, CALSEL_A>;
///Calibration selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CALSEL_A {
    ///0: VREFOPAMP=3.3% VDDA
    Percent33 = 0,
    ///1: VREFOPAMP=10% VDDA
    Percent10 = 1,
    ///2: VREFOPAMP=50% VDDA
    Percent50 = 2,
    ///3: VREFOPAMP=90% VDDA
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
///Field `CALSEL` writer - Calibration selection
pub type CALSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OPAMP2_CSR_SPEC, u8, CALSEL_A, 2, O>;
impl<'a, const O: u8> CALSEL_W<'a, O> {
    ///VREFOPAMP=3.3% VDDA
    #[inline(always)]
    pub fn percent3_3(self) -> &'a mut W {
        self.variant(CALSEL_A::Percent33)
    }
    ///VREFOPAMP=10% VDDA
    #[inline(always)]
    pub fn percent10(self) -> &'a mut W {
        self.variant(CALSEL_A::Percent10)
    }
    ///VREFOPAMP=50% VDDA
    #[inline(always)]
    pub fn percent50(self) -> &'a mut W {
        self.variant(CALSEL_A::Percent50)
    }
    ///VREFOPAMP=90% VDDA
    #[inline(always)]
    pub fn percent90(self) -> &'a mut W {
        self.variant(CALSEL_A::Percent90)
    }
}
///Field `PGA_GAIN` reader - Gain in PGA mode
pub type PGA_GAIN_R = crate::FieldReader<u8, PGA_GAIN_A>;
///Gain in PGA mode
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
    ///4: Gain 16
    Gain16 = 4,
    ///8: Gain 2, feedback connected to VM0
    Gain2Vm0 = 8,
    ///9: Gain 4, feedback connected to VM0
    Gain4Vm0 = 9,
    ///10: Gain 8, feedback connected to VM0
    Gain8Vm0 = 10,
    ///11: Gain 16, feedback connected to VM0
    Gain16Vm0 = 11,
    ///12: Gain 2, feedback connected to VM1
    Gain2Vm1 = 12,
    ///13: Gain 4, feedback connected to VM1
    Gain4Vm1 = 13,
    ///14: Gain 8, feedback connected to VM1
    Gain8Vm1 = 14,
    ///15: Gain 16, feedback connected to VM1
    Gain16Vm1 = 15,
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
            4 => Some(PGA_GAIN_A::Gain16),
            8 => Some(PGA_GAIN_A::Gain2Vm0),
            9 => Some(PGA_GAIN_A::Gain4Vm0),
            10 => Some(PGA_GAIN_A::Gain8Vm0),
            11 => Some(PGA_GAIN_A::Gain16Vm0),
            12 => Some(PGA_GAIN_A::Gain2Vm1),
            13 => Some(PGA_GAIN_A::Gain4Vm1),
            14 => Some(PGA_GAIN_A::Gain8Vm1),
            15 => Some(PGA_GAIN_A::Gain16Vm1),
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
    ///Checks if the value of the field is `Gain2Vm0`
    #[inline(always)]
    pub fn is_gain2_vm0(&self) -> bool {
        *self == PGA_GAIN_A::Gain2Vm0
    }
    ///Checks if the value of the field is `Gain4Vm0`
    #[inline(always)]
    pub fn is_gain4_vm0(&self) -> bool {
        *self == PGA_GAIN_A::Gain4Vm0
    }
    ///Checks if the value of the field is `Gain8Vm0`
    #[inline(always)]
    pub fn is_gain8_vm0(&self) -> bool {
        *self == PGA_GAIN_A::Gain8Vm0
    }
    ///Checks if the value of the field is `Gain16Vm0`
    #[inline(always)]
    pub fn is_gain16_vm0(&self) -> bool {
        *self == PGA_GAIN_A::Gain16Vm0
    }
    ///Checks if the value of the field is `Gain2Vm1`
    #[inline(always)]
    pub fn is_gain2_vm1(&self) -> bool {
        *self == PGA_GAIN_A::Gain2Vm1
    }
    ///Checks if the value of the field is `Gain4Vm1`
    #[inline(always)]
    pub fn is_gain4_vm1(&self) -> bool {
        *self == PGA_GAIN_A::Gain4Vm1
    }
    ///Checks if the value of the field is `Gain8Vm1`
    #[inline(always)]
    pub fn is_gain8_vm1(&self) -> bool {
        *self == PGA_GAIN_A::Gain8Vm1
    }
    ///Checks if the value of the field is `Gain16Vm1`
    #[inline(always)]
    pub fn is_gain16_vm1(&self) -> bool {
        *self == PGA_GAIN_A::Gain16Vm1
    }
}
///Field `PGA_GAIN` writer - Gain in PGA mode
pub type PGA_GAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPAMP2_CSR_SPEC, u8, PGA_GAIN_A, 4, O>;
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
    ///Gain 2, feedback connected to VM0
    #[inline(always)]
    pub fn gain2_vm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain2Vm0)
    }
    ///Gain 4, feedback connected to VM0
    #[inline(always)]
    pub fn gain4_vm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain4Vm0)
    }
    ///Gain 8, feedback connected to VM0
    #[inline(always)]
    pub fn gain8_vm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain8Vm0)
    }
    ///Gain 16, feedback connected to VM0
    #[inline(always)]
    pub fn gain16_vm0(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain16Vm0)
    }
    ///Gain 2, feedback connected to VM1
    #[inline(always)]
    pub fn gain2_vm1(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain2Vm1)
    }
    ///Gain 4, feedback connected to VM1
    #[inline(always)]
    pub fn gain4_vm1(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain4Vm1)
    }
    ///Gain 8, feedback connected to VM1
    #[inline(always)]
    pub fn gain8_vm1(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain8Vm1)
    }
    ///Gain 16, feedback connected to VM1
    #[inline(always)]
    pub fn gain16_vm1(self) -> &'a mut W {
        self.variant(PGA_GAIN_A::Gain16Vm1)
    }
}
///Field `USER_TRIM` reader - User trimming enable
pub type USER_TRIM_R = crate::BitReader<USER_TRIM_A>;
///User trimming enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USER_TRIM_A {
    ///0: User trimming disabled
    Disabled = 0,
    ///1: User trimming enabled
    Enabled = 1,
}
impl From<USER_TRIM_A> for bool {
    #[inline(always)]
    fn from(variant: USER_TRIM_A) -> Self {
        variant as u8 != 0
    }
}
impl USER_TRIM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> USER_TRIM_A {
        match self.bits {
            false => USER_TRIM_A::Disabled,
            true => USER_TRIM_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USER_TRIM_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USER_TRIM_A::Enabled
    }
}
///Field `USER_TRIM` writer - User trimming enable
pub type USER_TRIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CSR_SPEC, USER_TRIM_A, O>;
impl<'a, const O: u8> USER_TRIM_W<'a, O> {
    ///User trimming disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USER_TRIM_A::Disabled)
    }
    ///User trimming enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USER_TRIM_A::Enabled)
    }
}
///Field `TRIMOFFSETP` reader - Offset trimming value (PMOS)
pub type TRIMOFFSETP_R = crate::FieldReader<u8, u8>;
///Field `TRIMOFFSETP` writer - Offset trimming value (PMOS)
pub type TRIMOFFSETP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OPAMP2_CSR_SPEC, u8, u8, 5, O>;
///Field `TRIMOFFSETN` reader - Offset trimming value (NMOS)
pub type TRIMOFFSETN_R = crate::FieldReader<u8, u8>;
///Field `TRIMOFFSETN` writer - Offset trimming value (NMOS)
pub type TRIMOFFSETN_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OPAMP2_CSR_SPEC, u8, u8, 5, O>;
///Field `TSTREF` reader - TSTREF
pub type TSTREF_R = crate::BitReader<TSTREF_A>;
///TSTREF
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSTREF_A {
    ///0: VREFOPAMP2 is output
    Output = 0,
    ///1: VREFOPAMP2 is not output
    NotOutput = 1,
}
impl From<TSTREF_A> for bool {
    #[inline(always)]
    fn from(variant: TSTREF_A) -> Self {
        variant as u8 != 0
    }
}
impl TSTREF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TSTREF_A {
        match self.bits {
            false => TSTREF_A::Output,
            true => TSTREF_A::NotOutput,
        }
    }
    ///Checks if the value of the field is `Output`
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == TSTREF_A::Output
    }
    ///Checks if the value of the field is `NotOutput`
    #[inline(always)]
    pub fn is_not_output(&self) -> bool {
        *self == TSTREF_A::NotOutput
    }
}
///Field `TSTREF` writer - TSTREF
pub type TSTREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CSR_SPEC, TSTREF_A, O>;
impl<'a, const O: u8> TSTREF_W<'a, O> {
    ///VREFOPAMP2 is output
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(TSTREF_A::Output)
    }
    ///VREFOPAMP2 is not output
    #[inline(always)]
    pub fn not_output(self) -> &'a mut W {
        self.variant(TSTREF_A::NotOutput)
    }
}
///Field `OUTCAL` reader - OPAMP ouput status flag
pub type OUTCAL_R = crate::BitReader<OUTCAL_A>;
///OPAMP ouput status flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTCAL_A {
    ///0: Non-inverting &lt; inverting
    Low = 0,
    ///1: Non-inverting > inverting
    High = 1,
}
impl From<OUTCAL_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCAL_A) -> Self {
        variant as u8 != 0
    }
}
impl OUTCAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OUTCAL_A {
        match self.bits {
            false => OUTCAL_A::Low,
            true => OUTCAL_A::High,
        }
    }
    ///Checks if the value of the field is `Low`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OUTCAL_A::Low
    }
    ///Checks if the value of the field is `High`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OUTCAL_A::High
    }
}
///Field `LOCK` reader - OPAMP lock
pub type LOCK_R = crate::BitReader<LOCK_A>;
///OPAMP lock
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_A {
    ///0: Comparator CSR bits are read-write
    Unlocked = 0,
    ///1: Comparator CSR bits are read-only
    Locked = 1,
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
            false => LOCK_A::Unlocked,
            true => LOCK_A::Locked,
        }
    }
    ///Checks if the value of the field is `Unlocked`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK_A::Unlocked
    }
    ///Checks if the value of the field is `Locked`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK_A::Locked
    }
}
///Field `LOCK` writer - OPAMP lock
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPAMP2_CSR_SPEC, LOCK_A, O>;
impl<'a, const O: u8> LOCK_W<'a, O> {
    ///Comparator CSR bits are read-write
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCK_A::Unlocked)
    }
    ///Comparator CSR bits are read-only
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCK_A::Locked)
    }
}
impl R {
    ///Bit 0 - OPAMP2 enable
    #[inline(always)]
    pub fn opamp2en(&self) -> OPAMP2EN_R {
        OPAMP2EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FORCE_VP
    #[inline(always)]
    pub fn force_vp(&self) -> FORCE_VP_R {
        FORCE_VP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - OPAMP Non inverting input selection
    #[inline(always)]
    pub fn vp_sel(&self) -> VP_SEL_R {
        VP_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 5:6 - OPAMP inverting input selection
    #[inline(always)]
    pub fn vm_sel(&self) -> VM_SEL_R {
        VM_SEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - Timer controlled Mux mode enable
    #[inline(always)]
    pub fn tcm_en(&self) -> TCM_EN_R {
        TCM_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - OPAMP inverting input secondary selection
    #[inline(always)]
    pub fn vms_sel(&self) -> VMS_SEL_R {
        VMS_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - OPAMP Non inverting input secondary selection
    #[inline(always)]
    pub fn vps_sel(&self) -> VPS_SEL_R {
        VPS_SEL_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bit 11 - Calibration mode enable
    #[inline(always)]
    pub fn calon(&self) -> CALON_R {
        CALON_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Calibration selection
    #[inline(always)]
    pub fn calsel(&self) -> CALSEL_R {
        CALSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:17 - Gain in PGA mode
    #[inline(always)]
    pub fn pga_gain(&self) -> PGA_GAIN_R {
        PGA_GAIN_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    ///Bit 18 - User trimming enable
    #[inline(always)]
    pub fn user_trim(&self) -> USER_TRIM_R {
        USER_TRIM_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:23 - Offset trimming value (PMOS)
    #[inline(always)]
    pub fn trimoffsetp(&self) -> TRIMOFFSETP_R {
        TRIMOFFSETP_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    ///Bits 24:28 - Offset trimming value (NMOS)
    #[inline(always)]
    pub fn trimoffsetn(&self) -> TRIMOFFSETN_R {
        TRIMOFFSETN_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    ///Bit 29 - TSTREF
    #[inline(always)]
    pub fn tstref(&self) -> TSTREF_R {
        TSTREF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - OPAMP ouput status flag
    #[inline(always)]
    pub fn outcal(&self) -> OUTCAL_R {
        OUTCAL_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - OPAMP lock
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - OPAMP2 enable
    #[inline(always)]
    #[must_use]
    pub fn opamp2en(&mut self) -> OPAMP2EN_W<0> {
        OPAMP2EN_W::new(self)
    }
    ///Bit 1 - FORCE_VP
    #[inline(always)]
    #[must_use]
    pub fn force_vp(&mut self) -> FORCE_VP_W<1> {
        FORCE_VP_W::new(self)
    }
    ///Bits 2:3 - OPAMP Non inverting input selection
    #[inline(always)]
    #[must_use]
    pub fn vp_sel(&mut self) -> VP_SEL_W<2> {
        VP_SEL_W::new(self)
    }
    ///Bits 5:6 - OPAMP inverting input selection
    #[inline(always)]
    #[must_use]
    pub fn vm_sel(&mut self) -> VM_SEL_W<5> {
        VM_SEL_W::new(self)
    }
    ///Bit 7 - Timer controlled Mux mode enable
    #[inline(always)]
    #[must_use]
    pub fn tcm_en(&mut self) -> TCM_EN_W<7> {
        TCM_EN_W::new(self)
    }
    ///Bit 8 - OPAMP inverting input secondary selection
    #[inline(always)]
    #[must_use]
    pub fn vms_sel(&mut self) -> VMS_SEL_W<8> {
        VMS_SEL_W::new(self)
    }
    ///Bits 9:10 - OPAMP Non inverting input secondary selection
    #[inline(always)]
    #[must_use]
    pub fn vps_sel(&mut self) -> VPS_SEL_W<9> {
        VPS_SEL_W::new(self)
    }
    ///Bit 11 - Calibration mode enable
    #[inline(always)]
    #[must_use]
    pub fn calon(&mut self) -> CALON_W<11> {
        CALON_W::new(self)
    }
    ///Bits 12:13 - Calibration selection
    #[inline(always)]
    #[must_use]
    pub fn calsel(&mut self) -> CALSEL_W<12> {
        CALSEL_W::new(self)
    }
    ///Bits 14:17 - Gain in PGA mode
    #[inline(always)]
    #[must_use]
    pub fn pga_gain(&mut self) -> PGA_GAIN_W<14> {
        PGA_GAIN_W::new(self)
    }
    ///Bit 18 - User trimming enable
    #[inline(always)]
    #[must_use]
    pub fn user_trim(&mut self) -> USER_TRIM_W<18> {
        USER_TRIM_W::new(self)
    }
    ///Bits 19:23 - Offset trimming value (PMOS)
    #[inline(always)]
    #[must_use]
    pub fn trimoffsetp(&mut self) -> TRIMOFFSETP_W<19> {
        TRIMOFFSETP_W::new(self)
    }
    ///Bits 24:28 - Offset trimming value (NMOS)
    #[inline(always)]
    #[must_use]
    pub fn trimoffsetn(&mut self) -> TRIMOFFSETN_W<24> {
        TRIMOFFSETN_W::new(self)
    }
    ///Bit 29 - TSTREF
    #[inline(always)]
    #[must_use]
    pub fn tstref(&mut self) -> TSTREF_W<29> {
        TSTREF_W::new(self)
    }
    ///Bit 31 - OPAMP lock
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
///OPAMP2 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [opamp2_csr](index.html) module
pub struct OPAMP2_CSR_SPEC;
impl crate::RegisterSpec for OPAMP2_CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [opamp2_csr::R](R) reader structure
impl crate::Readable for OPAMP2_CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [opamp2_csr::W](W) writer structure
impl crate::Writable for OPAMP2_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OPAMP2_CSR to value 0
impl crate::Resettable for OPAMP2_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
