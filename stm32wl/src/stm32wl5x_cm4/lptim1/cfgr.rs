///Register `CFGR` reader
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR` writer
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CKSEL` reader - CKSEL
pub type CKSEL_R = crate::BitReader<CKSEL_A>;
///CKSEL
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKSEL_A {
    ///0: LPTIM is clocked by internal clock source (APB clock or any of the embedded oscillators)
    Internal = 0,
    ///1: LPTIM is clocked by an external clock source through the LPTIM external Input1
    External = 1,
}
impl From<CKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CKSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CKSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CKSEL_A {
        match self.bits {
            false => CKSEL_A::Internal,
            true => CKSEL_A::External,
        }
    }
    ///Checks if the value of the field is `Internal`
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == CKSEL_A::Internal
    }
    ///Checks if the value of the field is `External`
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == CKSEL_A::External
    }
}
///Field `CKSEL` writer - CKSEL
pub type CKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, CKSEL_A, O>;
impl<'a, const O: u8> CKSEL_W<'a, O> {
    ///LPTIM is clocked by internal clock source (APB clock or any of the embedded oscillators)
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(CKSEL_A::Internal)
    }
    ///LPTIM is clocked by an external clock source through the LPTIM external Input1
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(CKSEL_A::External)
    }
}
///Field `CKPOL` reader - CKPOL
pub type CKPOL_R = crate::FieldReader<u8, CKPOL_A>;
///CKPOL
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKPOL_A {
    ///0: The rising edge is the active edge used for counting. If LPTIM is in encoder mode: Encoder sub-mode 1 is active.
    RisingEdge = 0,
    ///1: The falling edge is the active edge used for counting. If LPTIM is in encoder mode: Encoder sub-mode 2 is active.
    FallingEdge = 1,
    ///2: Both edges are active edge. If LPTIM is in encoder mode: Encoder sub-mode 3 is active.
    BothEdges = 2,
}
impl From<CKPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: CKPOL_A) -> Self {
        variant as _
    }
}
impl CKPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CKPOL_A> {
        match self.bits {
            0 => Some(CKPOL_A::RisingEdge),
            1 => Some(CKPOL_A::FallingEdge),
            2 => Some(CKPOL_A::BothEdges),
            _ => None,
        }
    }
    ///Checks if the value of the field is `RisingEdge`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CKPOL_A::RisingEdge
    }
    ///Checks if the value of the field is `FallingEdge`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CKPOL_A::FallingEdge
    }
    ///Checks if the value of the field is `BothEdges`
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == CKPOL_A::BothEdges
    }
}
///Field `CKPOL` writer - CKPOL
pub type CKPOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, CKPOL_A, 2, O>;
impl<'a, const O: u8> CKPOL_W<'a, O> {
    ///The rising edge is the active edge used for counting. If LPTIM is in encoder mode: Encoder sub-mode 1 is active.
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(CKPOL_A::RisingEdge)
    }
    ///The falling edge is the active edge used for counting. If LPTIM is in encoder mode: Encoder sub-mode 2 is active.
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(CKPOL_A::FallingEdge)
    }
    ///Both edges are active edge. If LPTIM is in encoder mode: Encoder sub-mode 3 is active.
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(CKPOL_A::BothEdges)
    }
}
///Field `CKFLT` reader - CKFLT
pub type CKFLT_R = crate::FieldReader<u8, CKFLT_A>;
///CKFLT
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKFLT_A {
    ///0: Any external clock signal level change is considered as a valid transition
    Immediate = 0,
    ///1: External clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition
    Clocks2 = 1,
    ///2: External clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition
    Clocks4 = 2,
    ///3: External clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition
    Clocks8 = 3,
}
impl From<CKFLT_A> for u8 {
    #[inline(always)]
    fn from(variant: CKFLT_A) -> Self {
        variant as _
    }
}
impl CKFLT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CKFLT_A {
        match self.bits {
            0 => CKFLT_A::Immediate,
            1 => CKFLT_A::Clocks2,
            2 => CKFLT_A::Clocks4,
            3 => CKFLT_A::Clocks8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Immediate`
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == CKFLT_A::Immediate
    }
    ///Checks if the value of the field is `Clocks2`
    #[inline(always)]
    pub fn is_clocks2(&self) -> bool {
        *self == CKFLT_A::Clocks2
    }
    ///Checks if the value of the field is `Clocks4`
    #[inline(always)]
    pub fn is_clocks4(&self) -> bool {
        *self == CKFLT_A::Clocks4
    }
    ///Checks if the value of the field is `Clocks8`
    #[inline(always)]
    pub fn is_clocks8(&self) -> bool {
        *self == CKFLT_A::Clocks8
    }
}
///Field `CKFLT` writer - CKFLT
pub type CKFLT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, CKFLT_A, 2, O>;
impl<'a, const O: u8> CKFLT_W<'a, O> {
    ///Any external clock signal level change is considered as a valid transition
    #[inline(always)]
    pub fn immediate(self) -> &'a mut W {
        self.variant(CKFLT_A::Immediate)
    }
    ///External clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition
    #[inline(always)]
    pub fn clocks2(self) -> &'a mut W {
        self.variant(CKFLT_A::Clocks2)
    }
    ///External clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition
    #[inline(always)]
    pub fn clocks4(self) -> &'a mut W {
        self.variant(CKFLT_A::Clocks4)
    }
    ///External clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition
    #[inline(always)]
    pub fn clocks8(self) -> &'a mut W {
        self.variant(CKFLT_A::Clocks8)
    }
}
///Field `TRGFLT` reader - TRGFLT
pub type TRGFLT_R = crate::FieldReader<u8, TRGFLT_A>;
///TRGFLT
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRGFLT_A {
    ///0: Any trigger active level change is considered as a valid trigger
    Immediate = 0,
    ///1: Trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger
    Clocks2 = 1,
    ///2: Trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger
    Clocks4 = 2,
    ///3: Trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger
    Clocks8 = 3,
}
impl From<TRGFLT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGFLT_A) -> Self {
        variant as _
    }
}
impl TRGFLT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TRGFLT_A {
        match self.bits {
            0 => TRGFLT_A::Immediate,
            1 => TRGFLT_A::Clocks2,
            2 => TRGFLT_A::Clocks4,
            3 => TRGFLT_A::Clocks8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Immediate`
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == TRGFLT_A::Immediate
    }
    ///Checks if the value of the field is `Clocks2`
    #[inline(always)]
    pub fn is_clocks2(&self) -> bool {
        *self == TRGFLT_A::Clocks2
    }
    ///Checks if the value of the field is `Clocks4`
    #[inline(always)]
    pub fn is_clocks4(&self) -> bool {
        *self == TRGFLT_A::Clocks4
    }
    ///Checks if the value of the field is `Clocks8`
    #[inline(always)]
    pub fn is_clocks8(&self) -> bool {
        *self == TRGFLT_A::Clocks8
    }
}
///Field `TRGFLT` writer - TRGFLT
pub type TRGFLT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, TRGFLT_A, 2, O>;
impl<'a, const O: u8> TRGFLT_W<'a, O> {
    ///Any trigger active level change is considered as a valid trigger
    #[inline(always)]
    pub fn immediate(self) -> &'a mut W {
        self.variant(TRGFLT_A::Immediate)
    }
    ///Trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger
    #[inline(always)]
    pub fn clocks2(self) -> &'a mut W {
        self.variant(TRGFLT_A::Clocks2)
    }
    ///Trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger
    #[inline(always)]
    pub fn clocks4(self) -> &'a mut W {
        self.variant(TRGFLT_A::Clocks4)
    }
    ///Trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger
    #[inline(always)]
    pub fn clocks8(self) -> &'a mut W {
        self.variant(TRGFLT_A::Clocks8)
    }
}
///Field `PRESC` reader - PRESC
pub type PRESC_R = crate::FieldReader<u8, PRESC_A>;
///PRESC
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC_A {
    ///0: /1
    Div1 = 0,
    ///1: /2
    Div2 = 1,
    ///2: /4
    Div4 = 2,
    ///3: /8
    Div8 = 3,
    ///4: /16
    Div16 = 4,
    ///5: /32
    Div32 = 5,
    ///6: /64
    Div64 = 6,
    ///7: /128
    Div128 = 7,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
impl PRESC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PRESC_A {
        match self.bits {
            0 => PRESC_A::Div1,
            1 => PRESC_A::Div2,
            2 => PRESC_A::Div4,
            3 => PRESC_A::Div8,
            4 => PRESC_A::Div16,
            5 => PRESC_A::Div32,
            6 => PRESC_A::Div64,
            7 => PRESC_A::Div128,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESC_A::Div1
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESC_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESC_A::Div4
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESC_A::Div8
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESC_A::Div16
    }
    ///Checks if the value of the field is `Div32`
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESC_A::Div32
    }
    ///Checks if the value of the field is `Div64`
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESC_A::Div64
    }
    ///Checks if the value of the field is `Div128`
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESC_A::Div128
    }
}
///Field `PRESC` writer - PRESC
pub type PRESC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, PRESC_A, 3, O>;
impl<'a, const O: u8> PRESC_W<'a, O> {
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESC_A::Div1)
    }
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESC_A::Div2)
    }
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESC_A::Div4)
    }
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESC_A::Div8)
    }
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESC_A::Div16)
    }
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESC_A::Div32)
    }
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESC_A::Div64)
    }
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESC_A::Div128)
    }
}
///Field `TRIGSEL` reader - TRIGSEL
pub type TRIGSEL_R = crate::FieldReader<u8, TRIGSEL_A>;
///TRIGSEL
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGSEL_A {
    ///0: lptim_ext_trig0
    Trig0 = 0,
    ///1: lptim_ext_trig1
    Trig1 = 1,
    ///2: lptim_ext_trig2
    Trig2 = 2,
    ///3: lptim_ext_trig3
    Trig3 = 3,
    ///4: lptim_ext_trig4
    Trig4 = 4,
    ///5: lptim_ext_trig5
    Trig5 = 5,
    ///6: lptim_ext_trig6
    Trig6 = 6,
    ///7: lptim_ext_trig7
    Trig7 = 7,
}
impl From<TRIGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSEL_A) -> Self {
        variant as _
    }
}
impl TRIGSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TRIGSEL_A {
        match self.bits {
            0 => TRIGSEL_A::Trig0,
            1 => TRIGSEL_A::Trig1,
            2 => TRIGSEL_A::Trig2,
            3 => TRIGSEL_A::Trig3,
            4 => TRIGSEL_A::Trig4,
            5 => TRIGSEL_A::Trig5,
            6 => TRIGSEL_A::Trig6,
            7 => TRIGSEL_A::Trig7,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Trig0`
    #[inline(always)]
    pub fn is_trig0(&self) -> bool {
        *self == TRIGSEL_A::Trig0
    }
    ///Checks if the value of the field is `Trig1`
    #[inline(always)]
    pub fn is_trig1(&self) -> bool {
        *self == TRIGSEL_A::Trig1
    }
    ///Checks if the value of the field is `Trig2`
    #[inline(always)]
    pub fn is_trig2(&self) -> bool {
        *self == TRIGSEL_A::Trig2
    }
    ///Checks if the value of the field is `Trig3`
    #[inline(always)]
    pub fn is_trig3(&self) -> bool {
        *self == TRIGSEL_A::Trig3
    }
    ///Checks if the value of the field is `Trig4`
    #[inline(always)]
    pub fn is_trig4(&self) -> bool {
        *self == TRIGSEL_A::Trig4
    }
    ///Checks if the value of the field is `Trig5`
    #[inline(always)]
    pub fn is_trig5(&self) -> bool {
        *self == TRIGSEL_A::Trig5
    }
    ///Checks if the value of the field is `Trig6`
    #[inline(always)]
    pub fn is_trig6(&self) -> bool {
        *self == TRIGSEL_A::Trig6
    }
    ///Checks if the value of the field is `Trig7`
    #[inline(always)]
    pub fn is_trig7(&self) -> bool {
        *self == TRIGSEL_A::Trig7
    }
}
///Field `TRIGSEL` writer - TRIGSEL
pub type TRIGSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, TRIGSEL_A, 3, O>;
impl<'a, const O: u8> TRIGSEL_W<'a, O> {
    ///lptim_ext_trig0
    #[inline(always)]
    pub fn trig0(self) -> &'a mut W {
        self.variant(TRIGSEL_A::Trig0)
    }
    ///lptim_ext_trig1
    #[inline(always)]
    pub fn trig1(self) -> &'a mut W {
        self.variant(TRIGSEL_A::Trig1)
    }
    ///lptim_ext_trig2
    #[inline(always)]
    pub fn trig2(self) -> &'a mut W {
        self.variant(TRIGSEL_A::Trig2)
    }
    ///lptim_ext_trig3
    #[inline(always)]
    pub fn trig3(self) -> &'a mut W {
        self.variant(TRIGSEL_A::Trig3)
    }
    ///lptim_ext_trig4
    #[inline(always)]
    pub fn trig4(self) -> &'a mut W {
        self.variant(TRIGSEL_A::Trig4)
    }
    ///lptim_ext_trig5
    #[inline(always)]
    pub fn trig5(self) -> &'a mut W {
        self.variant(TRIGSEL_A::Trig5)
    }
    ///lptim_ext_trig6
    #[inline(always)]
    pub fn trig6(self) -> &'a mut W {
        self.variant(TRIGSEL_A::Trig6)
    }
    ///lptim_ext_trig7
    #[inline(always)]
    pub fn trig7(self) -> &'a mut W {
        self.variant(TRIGSEL_A::Trig7)
    }
}
///Field `TRIGEN` reader - TRIGEN
pub type TRIGEN_R = crate::FieldReader<u8, TRIGEN_A>;
///TRIGEN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGEN_A {
    ///0: Software trigger (counting start is initiated by software)
    Sw = 0,
    ///1: Rising edge is the active edge
    RisingEdge = 1,
    ///2: Falling edge is the active edge
    FallingEdge = 2,
    ///3: Both edges are active edges
    BothEdges = 3,
}
impl From<TRIGEN_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGEN_A) -> Self {
        variant as _
    }
}
impl TRIGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TRIGEN_A {
        match self.bits {
            0 => TRIGEN_A::Sw,
            1 => TRIGEN_A::RisingEdge,
            2 => TRIGEN_A::FallingEdge,
            3 => TRIGEN_A::BothEdges,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Sw`
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == TRIGEN_A::Sw
    }
    ///Checks if the value of the field is `RisingEdge`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == TRIGEN_A::RisingEdge
    }
    ///Checks if the value of the field is `FallingEdge`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == TRIGEN_A::FallingEdge
    }
    ///Checks if the value of the field is `BothEdges`
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == TRIGEN_A::BothEdges
    }
}
///Field `TRIGEN` writer - TRIGEN
pub type TRIGEN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR_SPEC, u8, TRIGEN_A, 2, O>;
impl<'a, const O: u8> TRIGEN_W<'a, O> {
    ///Software trigger (counting start is initiated by software)
    #[inline(always)]
    pub fn sw(self) -> &'a mut W {
        self.variant(TRIGEN_A::Sw)
    }
    ///Rising edge is the active edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(TRIGEN_A::RisingEdge)
    }
    ///Falling edge is the active edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(TRIGEN_A::FallingEdge)
    }
    ///Both edges are active edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(TRIGEN_A::BothEdges)
    }
}
///Field `TIMOUT` reader - TIMOUT
pub type TIMOUT_R = crate::BitReader<TIMOUT_A>;
///TIMOUT
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMOUT_A {
    ///0: A trigger event arriving when the timer is already started will be ignored
    Disabled = 0,
    ///1: A trigger event arriving when the timer is already started will reset and restart the counter
    Enabled = 1,
}
impl From<TIMOUT_A> for bool {
    #[inline(always)]
    fn from(variant: TIMOUT_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMOUT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TIMOUT_A {
        match self.bits {
            false => TIMOUT_A::Disabled,
            true => TIMOUT_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIMOUT_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIMOUT_A::Enabled
    }
}
///Field `TIMOUT` writer - TIMOUT
pub type TIMOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, TIMOUT_A, O>;
impl<'a, const O: u8> TIMOUT_W<'a, O> {
    ///A trigger event arriving when the timer is already started will be ignored
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIMOUT_A::Disabled)
    }
    ///A trigger event arriving when the timer is already started will reset and restart the counter
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIMOUT_A::Enabled)
    }
}
///Field `WAVE` reader - WAVE
pub type WAVE_R = crate::BitReader<WAVE_A>;
///WAVE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAVE_A {
    ///0: Deactivate Set-once mode, PWM / One Pulse waveform (depending on OPMODE bit)
    Inactive = 0,
    ///1: Activate the Set-once mode
    Active = 1,
}
impl From<WAVE_A> for bool {
    #[inline(always)]
    fn from(variant: WAVE_A) -> Self {
        variant as u8 != 0
    }
}
impl WAVE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WAVE_A {
        match self.bits {
            false => WAVE_A::Inactive,
            true => WAVE_A::Active,
        }
    }
    ///Checks if the value of the field is `Inactive`
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == WAVE_A::Inactive
    }
    ///Checks if the value of the field is `Active`
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == WAVE_A::Active
    }
}
///Field `WAVE` writer - WAVE
pub type WAVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, WAVE_A, O>;
impl<'a, const O: u8> WAVE_W<'a, O> {
    ///Deactivate Set-once mode, PWM / One Pulse waveform (depending on OPMODE bit)
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(WAVE_A::Inactive)
    }
    ///Activate the Set-once mode
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(WAVE_A::Active)
    }
}
///Field `WAVPOL` reader - WAVPOL
pub type WAVPOL_R = crate::BitReader<WAVPOL_A>;
///WAVPOL
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAVPOL_A {
    ///0: The LPTIM output reflects the compare results between LPTIM_ARR and LPTIM_CMP registers
    Positive = 0,
    ///1: The LPTIM output reflects the inverse of the compare results between LPTIM_ARR and LPTIM_CMP registers
    Negative = 1,
}
impl From<WAVPOL_A> for bool {
    #[inline(always)]
    fn from(variant: WAVPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl WAVPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WAVPOL_A {
        match self.bits {
            false => WAVPOL_A::Positive,
            true => WAVPOL_A::Negative,
        }
    }
    ///Checks if the value of the field is `Positive`
    #[inline(always)]
    pub fn is_positive(&self) -> bool {
        *self == WAVPOL_A::Positive
    }
    ///Checks if the value of the field is `Negative`
    #[inline(always)]
    pub fn is_negative(&self) -> bool {
        *self == WAVPOL_A::Negative
    }
}
///Field `WAVPOL` writer - WAVPOL
pub type WAVPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, WAVPOL_A, O>;
impl<'a, const O: u8> WAVPOL_W<'a, O> {
    ///The LPTIM output reflects the compare results between LPTIM_ARR and LPTIM_CMP registers
    #[inline(always)]
    pub fn positive(self) -> &'a mut W {
        self.variant(WAVPOL_A::Positive)
    }
    ///The LPTIM output reflects the inverse of the compare results between LPTIM_ARR and LPTIM_CMP registers
    #[inline(always)]
    pub fn negative(self) -> &'a mut W {
        self.variant(WAVPOL_A::Negative)
    }
}
///Field `PRELOAD` reader - PRELOAD
pub type PRELOAD_R = crate::BitReader<PRELOAD_A>;
///PRELOAD
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRELOAD_A {
    ///0: Registers are updated after each APB bus write access
    Immediate = 0,
    ///1: Registers are updated at the end of the current LPTIM period
    EndOfPeriod = 1,
}
impl From<PRELOAD_A> for bool {
    #[inline(always)]
    fn from(variant: PRELOAD_A) -> Self {
        variant as u8 != 0
    }
}
impl PRELOAD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PRELOAD_A {
        match self.bits {
            false => PRELOAD_A::Immediate,
            true => PRELOAD_A::EndOfPeriod,
        }
    }
    ///Checks if the value of the field is `Immediate`
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == PRELOAD_A::Immediate
    }
    ///Checks if the value of the field is `EndOfPeriod`
    #[inline(always)]
    pub fn is_end_of_period(&self) -> bool {
        *self == PRELOAD_A::EndOfPeriod
    }
}
///Field `PRELOAD` writer - PRELOAD
pub type PRELOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, PRELOAD_A, O>;
impl<'a, const O: u8> PRELOAD_W<'a, O> {
    ///Registers are updated after each APB bus write access
    #[inline(always)]
    pub fn immediate(self) -> &'a mut W {
        self.variant(PRELOAD_A::Immediate)
    }
    ///Registers are updated at the end of the current LPTIM period
    #[inline(always)]
    pub fn end_of_period(self) -> &'a mut W {
        self.variant(PRELOAD_A::EndOfPeriod)
    }
}
///Field `COUNTMODE` reader - COUNTMODE
pub type COUNTMODE_R = crate::BitReader<COUNTMODE_A>;
///COUNTMODE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COUNTMODE_A {
    ///0: The counter is incremented following each internal clock pulse
    Internal = 0,
    ///1: The counter is incremented following each valid clock pulse on the LPTIM external Input1
    External = 1,
}
impl From<COUNTMODE_A> for bool {
    #[inline(always)]
    fn from(variant: COUNTMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl COUNTMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> COUNTMODE_A {
        match self.bits {
            false => COUNTMODE_A::Internal,
            true => COUNTMODE_A::External,
        }
    }
    ///Checks if the value of the field is `Internal`
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == COUNTMODE_A::Internal
    }
    ///Checks if the value of the field is `External`
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == COUNTMODE_A::External
    }
}
///Field `COUNTMODE` writer - COUNTMODE
pub type COUNTMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, COUNTMODE_A, O>;
impl<'a, const O: u8> COUNTMODE_W<'a, O> {
    ///The counter is incremented following each internal clock pulse
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(COUNTMODE_A::Internal)
    }
    ///The counter is incremented following each valid clock pulse on the LPTIM external Input1
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(COUNTMODE_A::External)
    }
}
///Field `ENC` reader - ENC
pub type ENC_R = crate::BitReader<ENC_A>;
///ENC
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENC_A {
    ///0: Encoder mode disabled
    Disabled = 0,
    ///1: Encoder mode enabled
    Enabled = 1,
}
impl From<ENC_A> for bool {
    #[inline(always)]
    fn from(variant: ENC_A) -> Self {
        variant as u8 != 0
    }
}
impl ENC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ENC_A {
        match self.bits {
            false => ENC_A::Disabled,
            true => ENC_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENC_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENC_A::Enabled
    }
}
///Field `ENC` writer - ENC
pub type ENC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, ENC_A, O>;
impl<'a, const O: u8> ENC_W<'a, O> {
    ///Encoder mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENC_A::Disabled)
    }
    ///Encoder mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENC_A::Enabled)
    }
}
impl R {
    ///Bit 0 - CKSEL
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - CKPOL
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 3:4 - CKFLT
    #[inline(always)]
    pub fn ckflt(&self) -> CKFLT_R {
        CKFLT_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bits 6:7 - TRGFLT
    #[inline(always)]
    pub fn trgflt(&self) -> TRGFLT_R {
        TRGFLT_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 9:11 - PRESC
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 13:15 - TRIGSEL
    #[inline(always)]
    pub fn trigsel(&self) -> TRIGSEL_R {
        TRIGSEL_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 17:18 - TRIGEN
    #[inline(always)]
    pub fn trigen(&self) -> TRIGEN_R {
        TRIGEN_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bit 19 - TIMOUT
    #[inline(always)]
    pub fn timout(&self) -> TIMOUT_R {
        TIMOUT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - WAVE
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - WAVPOL
    #[inline(always)]
    pub fn wavpol(&self) -> WAVPOL_R {
        WAVPOL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - PRELOAD
    #[inline(always)]
    pub fn preload(&self) -> PRELOAD_R {
        PRELOAD_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - COUNTMODE
    #[inline(always)]
    pub fn countmode(&self) -> COUNTMODE_R {
        COUNTMODE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - ENC
    #[inline(always)]
    pub fn enc(&self) -> ENC_R {
        ENC_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CKSEL
    #[inline(always)]
    #[must_use]
    pub fn cksel(&mut self) -> CKSEL_W<0> {
        CKSEL_W::new(self)
    }
    ///Bits 1:2 - CKPOL
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CKPOL_W<1> {
        CKPOL_W::new(self)
    }
    ///Bits 3:4 - CKFLT
    #[inline(always)]
    #[must_use]
    pub fn ckflt(&mut self) -> CKFLT_W<3> {
        CKFLT_W::new(self)
    }
    ///Bits 6:7 - TRGFLT
    #[inline(always)]
    #[must_use]
    pub fn trgflt(&mut self) -> TRGFLT_W<6> {
        TRGFLT_W::new(self)
    }
    ///Bits 9:11 - PRESC
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<9> {
        PRESC_W::new(self)
    }
    ///Bits 13:15 - TRIGSEL
    #[inline(always)]
    #[must_use]
    pub fn trigsel(&mut self) -> TRIGSEL_W<13> {
        TRIGSEL_W::new(self)
    }
    ///Bits 17:18 - TRIGEN
    #[inline(always)]
    #[must_use]
    pub fn trigen(&mut self) -> TRIGEN_W<17> {
        TRIGEN_W::new(self)
    }
    ///Bit 19 - TIMOUT
    #[inline(always)]
    #[must_use]
    pub fn timout(&mut self) -> TIMOUT_W<19> {
        TIMOUT_W::new(self)
    }
    ///Bit 20 - WAVE
    #[inline(always)]
    #[must_use]
    pub fn wave(&mut self) -> WAVE_W<20> {
        WAVE_W::new(self)
    }
    ///Bit 21 - WAVPOL
    #[inline(always)]
    #[must_use]
    pub fn wavpol(&mut self) -> WAVPOL_W<21> {
        WAVPOL_W::new(self)
    }
    ///Bit 22 - PRELOAD
    #[inline(always)]
    #[must_use]
    pub fn preload(&mut self) -> PRELOAD_W<22> {
        PRELOAD_W::new(self)
    }
    ///Bit 23 - COUNTMODE
    #[inline(always)]
    #[must_use]
    pub fn countmode(&mut self) -> COUNTMODE_W<23> {
        COUNTMODE_W::new(self)
    }
    ///Bit 24 - ENC
    #[inline(always)]
    #[must_use]
    pub fn enc(&mut self) -> ENC_W<24> {
        ENC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr](index.html) module
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr::R](R) reader structure
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr::W](W) writer structure
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
