///Register `COMP2_CSR` reader
pub struct R(crate::R<COMP2_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP2_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP2_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP2_CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `COMP2_CSR` writer
pub struct W(crate::W<COMP2_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP2_CSR_SPEC>;
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
impl From<crate::W<COMP2_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP2_CSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EN` reader - Comparator 2 enable bit
pub type EN_R = crate::BitReader<EN_A>;
///Comparator 2 enable bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    ///0: Comparator x switched OFF
    Disabled = 0,
    ///1: Comparator x switched ON
    Enabled = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::Disabled,
            true => EN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_A::Enabled
    }
}
///Field `EN` writer - Comparator 2 enable bit
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    ///Comparator x switched OFF
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_A::Disabled)
    }
    ///Comparator x switched ON
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_A::Enabled)
    }
}
///Field `PWRMODE` reader - Power Mode of the comparator 2
pub type PWRMODE_R = crate::FieldReader<u8, PWRMODE_A>;
///Power Mode of the comparator 2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWRMODE_A {
    ///0: High speed
    High = 0,
    ///1: Medium speed
    Medium = 1,
    ///3: Ultra low power
    Low = 3,
}
impl From<PWRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRMODE_A) -> Self {
        variant as _
    }
}
impl PWRMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PWRMODE_A> {
        match self.bits {
            0 => Some(PWRMODE_A::High),
            1 => Some(PWRMODE_A::Medium),
            3 => Some(PWRMODE_A::Low),
            _ => None,
        }
    }
    ///Checks if the value of the field is `High`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PWRMODE_A::High
    }
    ///Checks if the value of the field is `Medium`
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == PWRMODE_A::Medium
    }
    ///Checks if the value of the field is `Low`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PWRMODE_A::Low
    }
}
///Field `PWRMODE` writer - Power Mode of the comparator 2
pub type PWRMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, PWRMODE_A, 2, O>;
impl<'a, const O: u8> PWRMODE_W<'a, O> {
    ///High speed
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PWRMODE_A::High)
    }
    ///Medium speed
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(PWRMODE_A::Medium)
    }
    ///Ultra low power
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PWRMODE_A::Low)
    }
}
///Field `INMSEL` reader - Comparator 2 Input Minus connection configuration bit
pub type INMSEL_R = crate::FieldReader<u8, u8>;
///Field `INMSEL` writer - Comparator 2 Input Minus connection configuration bit
pub type INMSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, u8, 3, O>;
///Field `INPSEL` reader - Comparator 2 Input Plus connection configuration bit
pub type INPSEL_R = crate::BitReader<INPSEL_A>;
///Comparator 2 Input Plus connection configuration bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INPSEL_A {
    ///0: PB4
    Pb4 = 0,
    ///1: PB6
    Pb6 = 1,
}
impl From<INPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: INPSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl INPSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> INPSEL_A {
        match self.bits {
            false => INPSEL_A::Pb4,
            true => INPSEL_A::Pb6,
        }
    }
    ///Checks if the value of the field is `Pb4`
    #[inline(always)]
    pub fn is_pb4(&self) -> bool {
        *self == INPSEL_A::Pb4
    }
    ///Checks if the value of the field is `Pb6`
    #[inline(always)]
    pub fn is_pb6(&self) -> bool {
        *self == INPSEL_A::Pb6
    }
}
///Field `INPSEL` writer - Comparator 2 Input Plus connection configuration bit
pub type INPSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, INPSEL_A, O>;
impl<'a, const O: u8> INPSEL_W<'a, O> {
    ///PB4
    #[inline(always)]
    pub fn pb4(self) -> &'a mut W {
        self.variant(INPSEL_A::Pb4)
    }
    ///PB6
    #[inline(always)]
    pub fn pb6(self) -> &'a mut W {
        self.variant(INPSEL_A::Pb6)
    }
}
///Field `WINMODE` reader - Windows mode selection bit
pub type WINMODE_R = crate::BitReader<WINMODE_A>;
///Windows mode selection bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WINMODE_A {
    ///0: Input plus of Comparator 2 is not connected to Comparator 1
    Disabled = 0,
    ///1: Input plus of Comparator 2 is connected with input plus of Comparator 1
    Enabled = 1,
}
impl From<WINMODE_A> for bool {
    #[inline(always)]
    fn from(variant: WINMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl WINMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WINMODE_A {
        match self.bits {
            false => WINMODE_A::Disabled,
            true => WINMODE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WINMODE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WINMODE_A::Enabled
    }
}
///Field `WINMODE` writer - Windows mode selection bit
pub type WINMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, WINMODE_A, O>;
impl<'a, const O: u8> WINMODE_W<'a, O> {
    ///Input plus of Comparator 2 is not connected to Comparator 1
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WINMODE_A::Disabled)
    }
    ///Input plus of Comparator 2 is connected with input plus of Comparator 1
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WINMODE_A::Enabled)
    }
}
///Field `POLARITY` reader - Comparator 2 polarity selection bit
pub type POLARITY_R = crate::BitReader<POLARITY_A>;
///Comparator 2 polarity selection bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POLARITY_A {
    ///0: Comparator x output value not inverted
    Normal = 0,
    ///1: Comparator x output value inverted
    Inverted = 1,
}
impl From<POLARITY_A> for bool {
    #[inline(always)]
    fn from(variant: POLARITY_A) -> Self {
        variant as u8 != 0
    }
}
impl POLARITY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> POLARITY_A {
        match self.bits {
            false => POLARITY_A::Normal,
            true => POLARITY_A::Inverted,
        }
    }
    ///Checks if the value of the field is `Normal`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == POLARITY_A::Normal
    }
    ///Checks if the value of the field is `Inverted`
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == POLARITY_A::Inverted
    }
}
///Field `POLARITY` writer - Comparator 2 polarity selection bit
pub type POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, POLARITY_A, O>;
impl<'a, const O: u8> POLARITY_W<'a, O> {
    ///Comparator x output value not inverted
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(POLARITY_A::Normal)
    }
    ///Comparator x output value inverted
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(POLARITY_A::Inverted)
    }
}
///Field `HYST` reader - Comparator 2 hysteresis selection bits
pub type HYST_R = crate::FieldReader<u8, HYST_A>;
///Comparator 2 hysteresis selection bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HYST_A {
    ///0: No hysteresis
    None = 0,
    ///1: Low hysteresis
    Low = 1,
    ///2: Medium hysteresis
    Medium = 2,
    ///3: High hysteresis
    High = 3,
}
impl From<HYST_A> for u8 {
    #[inline(always)]
    fn from(variant: HYST_A) -> Self {
        variant as _
    }
}
impl HYST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HYST_A {
        match self.bits {
            0 => HYST_A::None,
            1 => HYST_A::Low,
            2 => HYST_A::Medium,
            3 => HYST_A::High,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `None`
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == HYST_A::None
    }
    ///Checks if the value of the field is `Low`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == HYST_A::Low
    }
    ///Checks if the value of the field is `Medium`
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == HYST_A::Medium
    }
    ///Checks if the value of the field is `High`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == HYST_A::High
    }
}
///Field `HYST` writer - Comparator 2 hysteresis selection bits
pub type HYST_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, COMP2_CSR_SPEC, u8, HYST_A, 2, O>;
impl<'a, const O: u8> HYST_W<'a, O> {
    ///No hysteresis
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(HYST_A::None)
    }
    ///Low hysteresis
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(HYST_A::Low)
    }
    ///Medium hysteresis
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(HYST_A::Medium)
    }
    ///High hysteresis
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(HYST_A::High)
    }
}
///Field `BLANKING` reader - Comparator 2 blanking source selection bits
pub type BLANKING_R = crate::FieldReader<u8, BLANKING_A>;
///Comparator 2 blanking source selection bits
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BLANKING_A {
    ///0: No blanking
    Disabled = 0,
    ///4: TIM15 OC1 selected as blanking source
    Tim15oc1 = 4,
}
impl From<BLANKING_A> for u8 {
    #[inline(always)]
    fn from(variant: BLANKING_A) -> Self {
        variant as _
    }
}
impl BLANKING_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<BLANKING_A> {
        match self.bits {
            0 => Some(BLANKING_A::Disabled),
            4 => Some(BLANKING_A::Tim15oc1),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BLANKING_A::Disabled
    }
    ///Checks if the value of the field is `Tim15oc1`
    #[inline(always)]
    pub fn is_tim15oc1(&self) -> bool {
        *self == BLANKING_A::Tim15oc1
    }
}
///Field `BLANKING` writer - Comparator 2 blanking source selection bits
pub type BLANKING_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, BLANKING_A, 3, O>;
impl<'a, const O: u8> BLANKING_W<'a, O> {
    ///No blanking
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BLANKING_A::Disabled)
    }
    ///TIM15 OC1 selected as blanking source
    #[inline(always)]
    pub fn tim15oc1(self) -> &'a mut W {
        self.variant(BLANKING_A::Tim15oc1)
    }
}
///Field `BRGEN` reader - Scaler bridge enable
pub type BRGEN_R = crate::BitReader<BRGEN_A>;
///Scaler bridge enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRGEN_A {
    ///0: Scaler resistor bridge disabled (if BRGEN bit of COMP2_CSR register is also reset)
    Disabled = 0,
    ///1: Scaler resistor bridge enabled
    Enabled = 1,
}
impl From<BRGEN_A> for bool {
    #[inline(always)]
    fn from(variant: BRGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BRGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BRGEN_A {
        match self.bits {
            false => BRGEN_A::Disabled,
            true => BRGEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BRGEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BRGEN_A::Enabled
    }
}
///Field `BRGEN` writer - Scaler bridge enable
pub type BRGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, BRGEN_A, O>;
impl<'a, const O: u8> BRGEN_W<'a, O> {
    ///Scaler resistor bridge disabled (if BRGEN bit of COMP2_CSR register is also reset)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BRGEN_A::Disabled)
    }
    ///Scaler resistor bridge enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BRGEN_A::Enabled)
    }
}
///Field `SCALEN` reader - Voltage scaler enable bit
pub type SCALEN_R = crate::BitReader<SCALEN_A>;
///Voltage scaler enable bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCALEN_A {
    ///0: Bandgap scaler disabled (if SCALEN bit of COMP2_CSR register is also reset)
    Disabled = 0,
    ///1: Bandgap scaler enabled
    Enabled = 1,
}
impl From<SCALEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCALEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SCALEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SCALEN_A {
        match self.bits {
            false => SCALEN_A::Disabled,
            true => SCALEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCALEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCALEN_A::Enabled
    }
}
///Field `SCALEN` writer - Voltage scaler enable bit
pub type SCALEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, SCALEN_A, O>;
impl<'a, const O: u8> SCALEN_W<'a, O> {
    ///Bandgap scaler disabled (if SCALEN bit of COMP2_CSR register is also reset)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCALEN_A::Disabled)
    }
    ///Bandgap scaler enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCALEN_A::Enabled)
    }
}
///Field `VALUE` reader - Comparator 2 output status bit
pub type VALUE_R = crate::BitReader<bool>;
///COMP2_CSR register lock bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_AW {
    ///0: COMPx_CSR\[31:0\]
    ///for comparator x are read/write
    Unlocked = 0,
    ///1: COMPx_CSR\[31:0\]
    ///for comparator x are read-only
    Locked = 1,
}
impl From<LOCK_AW> for bool {
    #[inline(always)]
    fn from(variant: LOCK_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `LOCK` writer - COMP2_CSR register lock bit
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, LOCK_AW, O>;
impl<'a, const O: u8> LOCK_W<'a, O> {
    ///COMPx_CSR\[31:0\]
    ///for comparator x are read/write
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCK_AW::Unlocked)
    }
    ///COMPx_CSR\[31:0\]
    ///for comparator x are read-only
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCK_AW::Locked)
    }
}
impl R {
    ///Bit 0 - Comparator 2 enable bit
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:3 - Power Mode of the comparator 2
    #[inline(always)]
    pub fn pwrmode(&self) -> PWRMODE_R {
        PWRMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:6 - Comparator 2 Input Minus connection configuration bit
    #[inline(always)]
    pub fn inmsel(&self) -> INMSEL_R {
        INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Comparator 2 Input Plus connection configuration bit
    #[inline(always)]
    pub fn inpsel(&self) -> INPSEL_R {
        INPSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - Windows mode selection bit
    #[inline(always)]
    pub fn winmode(&self) -> WINMODE_R {
        WINMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 15 - Comparator 2 polarity selection bit
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Comparator 2 hysteresis selection bits
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:20 - Comparator 2 blanking source selection bits
    #[inline(always)]
    pub fn blanking(&self) -> BLANKING_R {
        BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 22 - Scaler bridge enable
    #[inline(always)]
    pub fn brgen(&self) -> BRGEN_R {
        BRGEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Voltage scaler enable bit
    #[inline(always)]
    pub fn scalen(&self) -> SCALEN_R {
        SCALEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 30 - Comparator 2 output status bit
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Comparator 2 enable bit
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    ///Bits 2:3 - Power Mode of the comparator 2
    #[inline(always)]
    #[must_use]
    pub fn pwrmode(&mut self) -> PWRMODE_W<2> {
        PWRMODE_W::new(self)
    }
    ///Bits 4:6 - Comparator 2 Input Minus connection configuration bit
    #[inline(always)]
    #[must_use]
    pub fn inmsel(&mut self) -> INMSEL_W<4> {
        INMSEL_W::new(self)
    }
    ///Bit 7 - Comparator 2 Input Plus connection configuration bit
    #[inline(always)]
    #[must_use]
    pub fn inpsel(&mut self) -> INPSEL_W<7> {
        INPSEL_W::new(self)
    }
    ///Bit 9 - Windows mode selection bit
    #[inline(always)]
    #[must_use]
    pub fn winmode(&mut self) -> WINMODE_W<9> {
        WINMODE_W::new(self)
    }
    ///Bit 15 - Comparator 2 polarity selection bit
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> POLARITY_W<15> {
        POLARITY_W::new(self)
    }
    ///Bits 16:17 - Comparator 2 hysteresis selection bits
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<16> {
        HYST_W::new(self)
    }
    ///Bits 18:20 - Comparator 2 blanking source selection bits
    #[inline(always)]
    #[must_use]
    pub fn blanking(&mut self) -> BLANKING_W<18> {
        BLANKING_W::new(self)
    }
    ///Bit 22 - Scaler bridge enable
    #[inline(always)]
    #[must_use]
    pub fn brgen(&mut self) -> BRGEN_W<22> {
        BRGEN_W::new(self)
    }
    ///Bit 23 - Voltage scaler enable bit
    #[inline(always)]
    #[must_use]
    pub fn scalen(&mut self) -> SCALEN_W<23> {
        SCALEN_W::new(self)
    }
    ///Bit 31 - COMP2_CSR register lock bit
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
///Comparator 2 control and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [comp2_csr](index.html) module
pub struct COMP2_CSR_SPEC;
impl crate::RegisterSpec for COMP2_CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [comp2_csr::R](R) reader structure
impl crate::Readable for COMP2_CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [comp2_csr::W](W) writer structure
impl crate::Writable for COMP2_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets COMP2_CSR to value 0
impl crate::Resettable for COMP2_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
