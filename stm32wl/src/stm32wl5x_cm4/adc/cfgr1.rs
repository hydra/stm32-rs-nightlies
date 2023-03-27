///Register `CFGR1` reader
pub struct R(crate::R<CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR1` writer
pub struct W(crate::W<CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR1_SPEC>;
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
impl From<crate::W<CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DMAEN` reader - DMAEN
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
///DMAEN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    ///0: DMA disabled
    Disabled = 0,
    ///1: DMA enabled
    Enabled = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::Disabled,
            true => DMAEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN_A::Enabled
    }
}
///Field `DMAEN` writer - DMAEN
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, DMAEN_A, O>;
impl<'a, const O: u8> DMAEN_W<'a, O> {
    ///DMA disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN_A::Disabled)
    }
    ///DMA enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN_A::Enabled)
    }
}
///Field `DMACFG` reader - DMACFG
pub type DMACFG_R = crate::BitReader<DMACFG_A>;
///DMACFG
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMACFG_A {
    ///0: DMA one shot mode selected
    OneShot = 0,
    ///1: DMA circular mode selected
    Circular = 1,
}
impl From<DMACFG_A> for bool {
    #[inline(always)]
    fn from(variant: DMACFG_A) -> Self {
        variant as u8 != 0
    }
}
impl DMACFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMACFG_A {
        match self.bits {
            false => DMACFG_A::OneShot,
            true => DMACFG_A::Circular,
        }
    }
    ///Checks if the value of the field is `OneShot`
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == DMACFG_A::OneShot
    }
    ///Checks if the value of the field is `Circular`
    #[inline(always)]
    pub fn is_circular(&self) -> bool {
        *self == DMACFG_A::Circular
    }
}
///Field `DMACFG` writer - DMACFG
pub type DMACFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, DMACFG_A, O>;
impl<'a, const O: u8> DMACFG_W<'a, O> {
    ///DMA one shot mode selected
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut W {
        self.variant(DMACFG_A::OneShot)
    }
    ///DMA circular mode selected
    #[inline(always)]
    pub fn circular(self) -> &'a mut W {
        self.variant(DMACFG_A::Circular)
    }
}
///Field `SCANDIR` reader - SCANDIR
pub type SCANDIR_R = crate::BitReader<SCANDIR_A>;
///SCANDIR
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCANDIR_A {
    ///0: Upward scan (from CHSEL0 to CHSEL17)
    Upward = 0,
    ///1: Backward scan (from CHSEL17 to CHSEL0)
    Backward = 1,
}
impl From<SCANDIR_A> for bool {
    #[inline(always)]
    fn from(variant: SCANDIR_A) -> Self {
        variant as u8 != 0
    }
}
impl SCANDIR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SCANDIR_A {
        match self.bits {
            false => SCANDIR_A::Upward,
            true => SCANDIR_A::Backward,
        }
    }
    ///Checks if the value of the field is `Upward`
    #[inline(always)]
    pub fn is_upward(&self) -> bool {
        *self == SCANDIR_A::Upward
    }
    ///Checks if the value of the field is `Backward`
    #[inline(always)]
    pub fn is_backward(&self) -> bool {
        *self == SCANDIR_A::Backward
    }
}
///Field `SCANDIR` writer - SCANDIR
pub type SCANDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, SCANDIR_A, O>;
impl<'a, const O: u8> SCANDIR_W<'a, O> {
    ///Upward scan (from CHSEL0 to CHSEL17)
    #[inline(always)]
    pub fn upward(self) -> &'a mut W {
        self.variant(SCANDIR_A::Upward)
    }
    ///Backward scan (from CHSEL17 to CHSEL0)
    #[inline(always)]
    pub fn backward(self) -> &'a mut W {
        self.variant(SCANDIR_A::Backward)
    }
}
///Field `RES` reader - RES
pub type RES_R = crate::FieldReader<u8, RES_A>;
///RES
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RES_A {
    ///0: 12 bits
    Bits12 = 0,
    ///1: 10 bits
    Bits10 = 1,
    ///2: 8 bits
    Bits8 = 2,
    ///3: 6 bits
    Bits6 = 3,
}
impl From<RES_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_A) -> Self {
        variant as _
    }
}
impl RES_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RES_A {
        match self.bits {
            0 => RES_A::Bits12,
            1 => RES_A::Bits10,
            2 => RES_A::Bits8,
            3 => RES_A::Bits6,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Bits12`
    #[inline(always)]
    pub fn is_bits12(&self) -> bool {
        *self == RES_A::Bits12
    }
    ///Checks if the value of the field is `Bits10`
    #[inline(always)]
    pub fn is_bits10(&self) -> bool {
        *self == RES_A::Bits10
    }
    ///Checks if the value of the field is `Bits8`
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == RES_A::Bits8
    }
    ///Checks if the value of the field is `Bits6`
    #[inline(always)]
    pub fn is_bits6(&self) -> bool {
        *self == RES_A::Bits6
    }
}
///Field `RES` writer - RES
pub type RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR1_SPEC, u8, RES_A, 2, O>;
impl<'a, const O: u8> RES_W<'a, O> {
    ///12 bits
    #[inline(always)]
    pub fn bits12(self) -> &'a mut W {
        self.variant(RES_A::Bits12)
    }
    ///10 bits
    #[inline(always)]
    pub fn bits10(self) -> &'a mut W {
        self.variant(RES_A::Bits10)
    }
    ///8 bits
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(RES_A::Bits8)
    }
    ///6 bits
    #[inline(always)]
    pub fn bits6(self) -> &'a mut W {
        self.variant(RES_A::Bits6)
    }
}
///Field `ALIGN` reader - ALIGN
pub type ALIGN_R = crate::BitReader<ALIGN_A>;
///ALIGN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALIGN_A {
    ///0: Right alignment
    Right = 0,
    ///1: Left alignment
    Left = 1,
}
impl From<ALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN_A) -> Self {
        variant as u8 != 0
    }
}
impl ALIGN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALIGN_A {
        match self.bits {
            false => ALIGN_A::Right,
            true => ALIGN_A::Left,
        }
    }
    ///Checks if the value of the field is `Right`
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == ALIGN_A::Right
    }
    ///Checks if the value of the field is `Left`
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == ALIGN_A::Left
    }
}
///Field `ALIGN` writer - ALIGN
pub type ALIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, ALIGN_A, O>;
impl<'a, const O: u8> ALIGN_W<'a, O> {
    ///Right alignment
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(ALIGN_A::Right)
    }
    ///Left alignment
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(ALIGN_A::Left)
    }
}
///Field `EXTSEL` reader - EXTSEL
pub type EXTSEL_R = crate::FieldReader<u8, EXTSEL_A>;
///EXTSEL
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTSEL_A {
    ///0: Timer 1 TRGO event
    Tim1Trgo = 0,
    ///1: Timer 1 CC4 event
    Tim1Cc4 = 1,
    ///2: Timer 2 TRGO event
    Tim2Trgo = 2,
    ///3: Timer 2 CH4 event
    Tim2Ch4 = 3,
    ///5: Timer 2 CH3 event
    Tim2Ch3 = 5,
    ///7: EXTI line 11 event
    ExtiLine11 = 7,
}
impl From<EXTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTSEL_A) -> Self {
        variant as _
    }
}
impl EXTSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTSEL_A> {
        match self.bits {
            0 => Some(EXTSEL_A::Tim1Trgo),
            1 => Some(EXTSEL_A::Tim1Cc4),
            2 => Some(EXTSEL_A::Tim2Trgo),
            3 => Some(EXTSEL_A::Tim2Ch4),
            5 => Some(EXTSEL_A::Tim2Ch3),
            7 => Some(EXTSEL_A::ExtiLine11),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Tim1Trgo`
    #[inline(always)]
    pub fn is_tim1_trgo(&self) -> bool {
        *self == EXTSEL_A::Tim1Trgo
    }
    ///Checks if the value of the field is `Tim1Cc4`
    #[inline(always)]
    pub fn is_tim1_cc4(&self) -> bool {
        *self == EXTSEL_A::Tim1Cc4
    }
    ///Checks if the value of the field is `Tim2Trgo`
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == EXTSEL_A::Tim2Trgo
    }
    ///Checks if the value of the field is `Tim2Ch4`
    #[inline(always)]
    pub fn is_tim2_ch4(&self) -> bool {
        *self == EXTSEL_A::Tim2Ch4
    }
    ///Checks if the value of the field is `Tim2Ch3`
    #[inline(always)]
    pub fn is_tim2_ch3(&self) -> bool {
        *self == EXTSEL_A::Tim2Ch3
    }
    ///Checks if the value of the field is `ExtiLine11`
    #[inline(always)]
    pub fn is_exti_line11(&self) -> bool {
        *self == EXTSEL_A::ExtiLine11
    }
}
///Field `EXTSEL` writer - EXTSEL
pub type EXTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, EXTSEL_A, 3, O>;
impl<'a, const O: u8> EXTSEL_W<'a, O> {
    ///Timer 1 TRGO event
    #[inline(always)]
    pub fn tim1_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim1Trgo)
    }
    ///Timer 1 CC4 event
    #[inline(always)]
    pub fn tim1_cc4(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim1Cc4)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim2Trgo)
    }
    ///Timer 2 CH4 event
    #[inline(always)]
    pub fn tim2_ch4(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim2Ch4)
    }
    ///Timer 2 CH3 event
    #[inline(always)]
    pub fn tim2_ch3(self) -> &'a mut W {
        self.variant(EXTSEL_A::Tim2Ch3)
    }
    ///EXTI line 11 event
    #[inline(always)]
    pub fn exti_line11(self) -> &'a mut W {
        self.variant(EXTSEL_A::ExtiLine11)
    }
}
///Field `EXTEN` reader - EXTEN
pub type EXTEN_R = crate::FieldReader<u8, EXTEN_A>;
///EXTEN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTEN_A {
    ///0: Hardware trigger detection disabled
    Disabled = 0,
    ///1: Hardware trigger detection on the rising edge
    RisingEdge = 1,
    ///2: Hardware trigger detection on the falling edge
    FallingEdge = 2,
    ///3: Hardware trigger detection on both the rising and falling edges
    BothEdges = 3,
}
impl From<EXTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTEN_A) -> Self {
        variant as _
    }
}
impl EXTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EXTEN_A {
        match self.bits {
            0 => EXTEN_A::Disabled,
            1 => EXTEN_A::RisingEdge,
            2 => EXTEN_A::FallingEdge,
            3 => EXTEN_A::BothEdges,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTEN_A::Disabled
    }
    ///Checks if the value of the field is `RisingEdge`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTEN_A::RisingEdge
    }
    ///Checks if the value of the field is `FallingEdge`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTEN_A::FallingEdge
    }
    ///Checks if the value of the field is `BothEdges`
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == EXTEN_A::BothEdges
    }
}
///Field `EXTEN` writer - EXTEN
pub type EXTEN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR1_SPEC, u8, EXTEN_A, 2, O>;
impl<'a, const O: u8> EXTEN_W<'a, O> {
    ///Hardware trigger detection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTEN_A::Disabled)
    }
    ///Hardware trigger detection on the rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EXTEN_A::RisingEdge)
    }
    ///Hardware trigger detection on the falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EXTEN_A::FallingEdge)
    }
    ///Hardware trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(EXTEN_A::BothEdges)
    }
}
///Field `OVRMOD` reader - OVRMOD
pub type OVRMOD_R = crate::BitReader<OVRMOD_A>;
///OVRMOD
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRMOD_A {
    ///0: ADC_DR register is preserved with the old data when an overrun is detected
    Preserve = 0,
    ///1: ADC_DR register is overwritten with the last conversion result when an overrun is detected
    Overwrite = 1,
}
impl From<OVRMOD_A> for bool {
    #[inline(always)]
    fn from(variant: OVRMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVRMOD_A {
        match self.bits {
            false => OVRMOD_A::Preserve,
            true => OVRMOD_A::Overwrite,
        }
    }
    ///Checks if the value of the field is `Preserve`
    #[inline(always)]
    pub fn is_preserve(&self) -> bool {
        *self == OVRMOD_A::Preserve
    }
    ///Checks if the value of the field is `Overwrite`
    #[inline(always)]
    pub fn is_overwrite(&self) -> bool {
        *self == OVRMOD_A::Overwrite
    }
}
///Field `OVRMOD` writer - OVRMOD
pub type OVRMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, OVRMOD_A, O>;
impl<'a, const O: u8> OVRMOD_W<'a, O> {
    ///ADC_DR register is preserved with the old data when an overrun is detected
    #[inline(always)]
    pub fn preserve(self) -> &'a mut W {
        self.variant(OVRMOD_A::Preserve)
    }
    ///ADC_DR register is overwritten with the last conversion result when an overrun is detected
    #[inline(always)]
    pub fn overwrite(self) -> &'a mut W {
        self.variant(OVRMOD_A::Overwrite)
    }
}
///Field `CONT` reader - CONT
pub type CONT_R = crate::BitReader<CONT_A>;
///CONT
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONT_A {
    ///0: Single conversion mode
    Single = 0,
    ///1: Continuous conversion mode
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
            false => CONT_A::Single,
            true => CONT_A::Continuous,
        }
    }
    ///Checks if the value of the field is `Single`
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == CONT_A::Single
    }
    ///Checks if the value of the field is `Continuous`
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CONT_A::Continuous
    }
}
///Field `CONT` writer - CONT
pub type CONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, CONT_A, O>;
impl<'a, const O: u8> CONT_W<'a, O> {
    ///Single conversion mode
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(CONT_A::Single)
    }
    ///Continuous conversion mode
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CONT_A::Continuous)
    }
}
///Field `WAIT` reader - WAIT
pub type WAIT_R = crate::BitReader<WAIT_A>;
///WAIT
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAIT_A {
    ///0: Wait conversion mode off
    Disabled = 0,
    ///1: Wait conversion mode on
    Enabled = 1,
}
impl From<WAIT_A> for bool {
    #[inline(always)]
    fn from(variant: WAIT_A) -> Self {
        variant as u8 != 0
    }
}
impl WAIT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WAIT_A {
        match self.bits {
            false => WAIT_A::Disabled,
            true => WAIT_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAIT_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAIT_A::Enabled
    }
}
///Field `WAIT` writer - WAIT
pub type WAIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, WAIT_A, O>;
impl<'a, const O: u8> WAIT_W<'a, O> {
    ///Wait conversion mode off
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAIT_A::Disabled)
    }
    ///Wait conversion mode on
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAIT_A::Enabled)
    }
}
///Field `AUTOFF` reader - AUTOFF
pub type AUTOFF_R = crate::BitReader<AUTOFF_A>;
///AUTOFF
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOFF_A {
    ///0: Auto-off mode disabled
    Disabled = 0,
    ///1: Auto-off mode enabled
    Enabled = 1,
}
impl From<AUTOFF_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTOFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AUTOFF_A {
        match self.bits {
            false => AUTOFF_A::Disabled,
            true => AUTOFF_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AUTOFF_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AUTOFF_A::Enabled
    }
}
///Field `AUTOFF` writer - AUTOFF
pub type AUTOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, AUTOFF_A, O>;
impl<'a, const O: u8> AUTOFF_W<'a, O> {
    ///Auto-off mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AUTOFF_A::Disabled)
    }
    ///Auto-off mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AUTOFF_A::Enabled)
    }
}
///Field `DISCEN` reader - DISCEN
pub type DISCEN_R = crate::BitReader<DISCEN_A>;
///DISCEN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISCEN_A {
    ///0: Discontinuous mode disabled
    Disabled = 0,
    ///1: Discontinuous mode enabled
    Enabled = 1,
}
impl From<DISCEN_A> for bool {
    #[inline(always)]
    fn from(variant: DISCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DISCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DISCEN_A {
        match self.bits {
            false => DISCEN_A::Disabled,
            true => DISCEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISCEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISCEN_A::Enabled
    }
}
///Field `DISCEN` writer - DISCEN
pub type DISCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, DISCEN_A, O>;
impl<'a, const O: u8> DISCEN_W<'a, O> {
    ///Discontinuous mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISCEN_A::Disabled)
    }
    ///Discontinuous mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISCEN_A::Enabled)
    }
}
///Field `CHSELRMOD` reader - CHSELRMOD
pub type CHSELRMOD_R = crate::BitReader<CHSELRMOD_A>;
///CHSELRMOD
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSELRMOD_A {
    ///0: Each bit of the ADC_CHSELR register enables an input
    BitPerInput = 0,
    ///1: ADC_CHSELR register is able to sequence up to 8 channels
    Sequence = 1,
}
impl From<CHSELRMOD_A> for bool {
    #[inline(always)]
    fn from(variant: CHSELRMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSELRMOD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CHSELRMOD_A {
        match self.bits {
            false => CHSELRMOD_A::BitPerInput,
            true => CHSELRMOD_A::Sequence,
        }
    }
    ///Checks if the value of the field is `BitPerInput`
    #[inline(always)]
    pub fn is_bit_per_input(&self) -> bool {
        *self == CHSELRMOD_A::BitPerInput
    }
    ///Checks if the value of the field is `Sequence`
    #[inline(always)]
    pub fn is_sequence(&self) -> bool {
        *self == CHSELRMOD_A::Sequence
    }
}
///Field `CHSELRMOD` writer - CHSELRMOD
pub type CHSELRMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, CHSELRMOD_A, O>;
impl<'a, const O: u8> CHSELRMOD_W<'a, O> {
    ///Each bit of the ADC_CHSELR register enables an input
    #[inline(always)]
    pub fn bit_per_input(self) -> &'a mut W {
        self.variant(CHSELRMOD_A::BitPerInput)
    }
    ///ADC_CHSELR register is able to sequence up to 8 channels
    #[inline(always)]
    pub fn sequence(self) -> &'a mut W {
        self.variant(CHSELRMOD_A::Sequence)
    }
}
///Field `AWD1SGL` reader - AWD1SGL
pub type AWD1SGL_R = crate::BitReader<AWD1SGL_A>;
///AWD1SGL
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1SGL_A {
    ///0: Analog watchdog 1 enabled on all channels
    AllChannels = 0,
    ///1: Analog watchdog 1 enabled on a single channel
    SingleChannel = 1,
}
impl From<AWD1SGL_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1SGL_A) -> Self {
        variant as u8 != 0
    }
}
impl AWD1SGL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AWD1SGL_A {
        match self.bits {
            false => AWD1SGL_A::AllChannels,
            true => AWD1SGL_A::SingleChannel,
        }
    }
    ///Checks if the value of the field is `AllChannels`
    #[inline(always)]
    pub fn is_all_channels(&self) -> bool {
        *self == AWD1SGL_A::AllChannels
    }
    ///Checks if the value of the field is `SingleChannel`
    #[inline(always)]
    pub fn is_single_channel(&self) -> bool {
        *self == AWD1SGL_A::SingleChannel
    }
}
///Field `AWD1SGL` writer - AWD1SGL
pub type AWD1SGL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, AWD1SGL_A, O>;
impl<'a, const O: u8> AWD1SGL_W<'a, O> {
    ///Analog watchdog 1 enabled on all channels
    #[inline(always)]
    pub fn all_channels(self) -> &'a mut W {
        self.variant(AWD1SGL_A::AllChannels)
    }
    ///Analog watchdog 1 enabled on a single channel
    #[inline(always)]
    pub fn single_channel(self) -> &'a mut W {
        self.variant(AWD1SGL_A::SingleChannel)
    }
}
///Field `AWD1EN` reader - AWD1EN
pub type AWD1EN_R = crate::BitReader<AWD1EN_A>;
///AWD1EN
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1EN_A {
    ///0: Analog watchdog 1 disabled
    Disabled = 0,
    ///1: Analog watchdog 1 enabled
    Enabled = 1,
}
impl From<AWD1EN_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl AWD1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AWD1EN_A {
        match self.bits {
            false => AWD1EN_A::Disabled,
            true => AWD1EN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWD1EN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWD1EN_A::Enabled
    }
}
///Field `AWD1EN` writer - AWD1EN
pub type AWD1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR1_SPEC, AWD1EN_A, O>;
impl<'a, const O: u8> AWD1EN_W<'a, O> {
    ///Analog watchdog 1 disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AWD1EN_A::Disabled)
    }
    ///Analog watchdog 1 enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AWD1EN_A::Enabled)
    }
}
///Field `AWD1CH` reader - AWD1CH
pub type AWD1CH_R = crate::FieldReader<u8, u8>;
///Field `AWD1CH` writer - AWD1CH
pub type AWD1CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR1_SPEC, u8, u8, 5, O>;
impl R {
    ///Bit 0 - DMAEN
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMACFG
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SCANDIR
    #[inline(always)]
    pub fn scandir(&self) -> SCANDIR_R {
        SCANDIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - RES
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - ALIGN
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:8 - EXTSEL
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 10:11 - EXTEN
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - OVRMOD
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CONT
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - WAIT
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - AUTOFF
    #[inline(always)]
    pub fn autoff(&self) -> AUTOFF_R {
        AUTOFF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - DISCEN
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 21 - CHSELRMOD
    #[inline(always)]
    pub fn chselrmod(&self) -> CHSELRMOD_R {
        CHSELRMOD_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - AWD1SGL
    #[inline(always)]
    pub fn awd1sgl(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - AWD1EN
    #[inline(always)]
    pub fn awd1en(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 26:30 - AWD1CH
    #[inline(always)]
    pub fn awd1ch(&self) -> AWD1CH_R {
        AWD1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    ///Bit 0 - DMAEN
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<0> {
        DMAEN_W::new(self)
    }
    ///Bit 1 - DMACFG
    #[inline(always)]
    #[must_use]
    pub fn dmacfg(&mut self) -> DMACFG_W<1> {
        DMACFG_W::new(self)
    }
    ///Bit 2 - SCANDIR
    #[inline(always)]
    #[must_use]
    pub fn scandir(&mut self) -> SCANDIR_W<2> {
        SCANDIR_W::new(self)
    }
    ///Bits 3:4 - RES
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<3> {
        RES_W::new(self)
    }
    ///Bit 5 - ALIGN
    #[inline(always)]
    #[must_use]
    pub fn align(&mut self) -> ALIGN_W<5> {
        ALIGN_W::new(self)
    }
    ///Bits 6:8 - EXTSEL
    #[inline(always)]
    #[must_use]
    pub fn extsel(&mut self) -> EXTSEL_W<6> {
        EXTSEL_W::new(self)
    }
    ///Bits 10:11 - EXTEN
    #[inline(always)]
    #[must_use]
    pub fn exten(&mut self) -> EXTEN_W<10> {
        EXTEN_W::new(self)
    }
    ///Bit 12 - OVRMOD
    #[inline(always)]
    #[must_use]
    pub fn ovrmod(&mut self) -> OVRMOD_W<12> {
        OVRMOD_W::new(self)
    }
    ///Bit 13 - CONT
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<13> {
        CONT_W::new(self)
    }
    ///Bit 14 - WAIT
    #[inline(always)]
    #[must_use]
    pub fn wait(&mut self) -> WAIT_W<14> {
        WAIT_W::new(self)
    }
    ///Bit 15 - AUTOFF
    #[inline(always)]
    #[must_use]
    pub fn autoff(&mut self) -> AUTOFF_W<15> {
        AUTOFF_W::new(self)
    }
    ///Bit 16 - DISCEN
    #[inline(always)]
    #[must_use]
    pub fn discen(&mut self) -> DISCEN_W<16> {
        DISCEN_W::new(self)
    }
    ///Bit 21 - CHSELRMOD
    #[inline(always)]
    #[must_use]
    pub fn chselrmod(&mut self) -> CHSELRMOD_W<21> {
        CHSELRMOD_W::new(self)
    }
    ///Bit 22 - AWD1SGL
    #[inline(always)]
    #[must_use]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W<22> {
        AWD1SGL_W::new(self)
    }
    ///Bit 23 - AWD1EN
    #[inline(always)]
    #[must_use]
    pub fn awd1en(&mut self) -> AWD1EN_W<23> {
        AWD1EN_W::new(self)
    }
    ///Bits 26:30 - AWD1CH
    #[inline(always)]
    #[must_use]
    pub fn awd1ch(&mut self) -> AWD1CH_W<26> {
        AWD1CH_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr1](index.html) module
pub struct CFGR1_SPEC;
impl crate::RegisterSpec for CFGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr1::R](R) reader structure
impl crate::Readable for CFGR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr1::W](W) writer structure
impl crate::Writable for CFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CFGR1 to value 0
impl crate::Resettable for CFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
