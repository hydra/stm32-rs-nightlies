///Register `RG1CR` reader
pub struct R(crate::R<RG1CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RG1CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RG1CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RG1CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RG1CR` writer
pub struct W(crate::W<RG1CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RG1CR_SPEC>;
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
impl From<crate::W<RG1CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RG1CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SIG_ID` reader - Signal identification
pub type SIG_ID_R = crate::FieldReader<u8, SIG_ID_A>;
///Signal identification
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIG_ID_A {
    ///0: Signal `EXTIx` selected as synchronization input
    Exti0 = 0,
    ///1: Signal `EXTIx` selected as synchronization input
    Exti1 = 1,
    ///2: Signal `EXTIx` selected as synchronization input
    Exti2 = 2,
    ///3: Signal `EXTIx` selected as synchronization input
    Exti3 = 3,
    ///4: Signal `EXTIx` selected as synchronization input
    Exti4 = 4,
    ///5: Signal `EXTIx` selected as synchronization input
    Exti5 = 5,
    ///6: Signal `EXTIx` selected as synchronization input
    Exti6 = 6,
    ///7: Signal `EXTIx` selected as synchronization input
    Exti7 = 7,
    ///8: Signal `EXTIx` selected as synchronization input
    Exti8 = 8,
    ///9: Signal `EXTIx` selected as synchronization input
    Exti9 = 9,
    ///10: Signal `EXTIx` selected as synchronization input
    Exti10 = 10,
    ///11: Signal `EXTIx` selected as synchronization input
    Exti11 = 11,
    ///12: Signal `EXTIx` selected as synchronization input
    Exti12 = 12,
    ///13: Signal `EXTIx` selected as synchronization input
    Exti13 = 13,
    ///14: Signal `EXTIx` selected as synchronization input
    Exti14 = 14,
    ///15: Signal `EXTIx` selected as synchronization input
    Exti15 = 15,
    ///16: Signal `dmamux1_evt0` selected as synchronization input
    Dmamux1Evt0 = 16,
    ///17: Signal `dmamux1_evt1` selected as synchronization input
    Dmamux1Evt1 = 17,
    ///18: Signal `lptim1_out` selected as synchronization input
    Lptim1Out = 18,
    ///19: Signal `lptim2_out` selected as synchronization input
    Lptim2Out = 19,
    ///20: Signal `lptim3_out` selected as synchronization input
    Lptim3Out = 20,
}
impl From<SIG_ID_A> for u8 {
    #[inline(always)]
    fn from(variant: SIG_ID_A) -> Self {
        variant as _
    }
}
impl SIG_ID_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SIG_ID_A> {
        match self.bits {
            0 => Some(SIG_ID_A::Exti0),
            1 => Some(SIG_ID_A::Exti1),
            2 => Some(SIG_ID_A::Exti2),
            3 => Some(SIG_ID_A::Exti3),
            4 => Some(SIG_ID_A::Exti4),
            5 => Some(SIG_ID_A::Exti5),
            6 => Some(SIG_ID_A::Exti6),
            7 => Some(SIG_ID_A::Exti7),
            8 => Some(SIG_ID_A::Exti8),
            9 => Some(SIG_ID_A::Exti9),
            10 => Some(SIG_ID_A::Exti10),
            11 => Some(SIG_ID_A::Exti11),
            12 => Some(SIG_ID_A::Exti12),
            13 => Some(SIG_ID_A::Exti13),
            14 => Some(SIG_ID_A::Exti14),
            15 => Some(SIG_ID_A::Exti15),
            16 => Some(SIG_ID_A::Dmamux1Evt0),
            17 => Some(SIG_ID_A::Dmamux1Evt1),
            18 => Some(SIG_ID_A::Lptim1Out),
            19 => Some(SIG_ID_A::Lptim2Out),
            20 => Some(SIG_ID_A::Lptim3Out),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Exti0`
    #[inline(always)]
    pub fn is_exti0(&self) -> bool {
        *self == SIG_ID_A::Exti0
    }
    ///Checks if the value of the field is `Exti1`
    #[inline(always)]
    pub fn is_exti1(&self) -> bool {
        *self == SIG_ID_A::Exti1
    }
    ///Checks if the value of the field is `Exti2`
    #[inline(always)]
    pub fn is_exti2(&self) -> bool {
        *self == SIG_ID_A::Exti2
    }
    ///Checks if the value of the field is `Exti3`
    #[inline(always)]
    pub fn is_exti3(&self) -> bool {
        *self == SIG_ID_A::Exti3
    }
    ///Checks if the value of the field is `Exti4`
    #[inline(always)]
    pub fn is_exti4(&self) -> bool {
        *self == SIG_ID_A::Exti4
    }
    ///Checks if the value of the field is `Exti5`
    #[inline(always)]
    pub fn is_exti5(&self) -> bool {
        *self == SIG_ID_A::Exti5
    }
    ///Checks if the value of the field is `Exti6`
    #[inline(always)]
    pub fn is_exti6(&self) -> bool {
        *self == SIG_ID_A::Exti6
    }
    ///Checks if the value of the field is `Exti7`
    #[inline(always)]
    pub fn is_exti7(&self) -> bool {
        *self == SIG_ID_A::Exti7
    }
    ///Checks if the value of the field is `Exti8`
    #[inline(always)]
    pub fn is_exti8(&self) -> bool {
        *self == SIG_ID_A::Exti8
    }
    ///Checks if the value of the field is `Exti9`
    #[inline(always)]
    pub fn is_exti9(&self) -> bool {
        *self == SIG_ID_A::Exti9
    }
    ///Checks if the value of the field is `Exti10`
    #[inline(always)]
    pub fn is_exti10(&self) -> bool {
        *self == SIG_ID_A::Exti10
    }
    ///Checks if the value of the field is `Exti11`
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        *self == SIG_ID_A::Exti11
    }
    ///Checks if the value of the field is `Exti12`
    #[inline(always)]
    pub fn is_exti12(&self) -> bool {
        *self == SIG_ID_A::Exti12
    }
    ///Checks if the value of the field is `Exti13`
    #[inline(always)]
    pub fn is_exti13(&self) -> bool {
        *self == SIG_ID_A::Exti13
    }
    ///Checks if the value of the field is `Exti14`
    #[inline(always)]
    pub fn is_exti14(&self) -> bool {
        *self == SIG_ID_A::Exti14
    }
    ///Checks if the value of the field is `Exti15`
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        *self == SIG_ID_A::Exti15
    }
    ///Checks if the value of the field is `Dmamux1Evt0`
    #[inline(always)]
    pub fn is_dmamux1_evt0(&self) -> bool {
        *self == SIG_ID_A::Dmamux1Evt0
    }
    ///Checks if the value of the field is `Dmamux1Evt1`
    #[inline(always)]
    pub fn is_dmamux1_evt1(&self) -> bool {
        *self == SIG_ID_A::Dmamux1Evt1
    }
    ///Checks if the value of the field is `Lptim1Out`
    #[inline(always)]
    pub fn is_lptim1_out(&self) -> bool {
        *self == SIG_ID_A::Lptim1Out
    }
    ///Checks if the value of the field is `Lptim2Out`
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        *self == SIG_ID_A::Lptim2Out
    }
    ///Checks if the value of the field is `Lptim3Out`
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        *self == SIG_ID_A::Lptim3Out
    }
}
///Field `SIG_ID` writer - Signal identification
pub type SIG_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RG1CR_SPEC, u8, SIG_ID_A, 5, O>;
impl<'a, const O: u8> SIG_ID_W<'a, O> {
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti0(self) -> &'a mut W {
        self.variant(SIG_ID_A::Exti0)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti1(self) -> &'a mut W {
        self.variant(SIG_ID_A::Exti1)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti2(self) -> &'a mut W {
        self.variant(SIG_ID_A::Exti2)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti3(self) -> &'a mut W {
        self.variant(SIG_ID_A::Exti3)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti4(self) -> &'a mut W {
        self.variant(SIG_ID_A::Exti4)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti5(self) -> &'a mut W {
        self.variant(SIG_ID_A::Exti5)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti6(self) -> &'a mut W {
        self.variant(SIG_ID_A::Exti6)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti7(self) -> &'a mut W {
        self.variant(SIG_ID_A::Exti7)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti8(self) -> &'a mut W {
        self.variant(SIG_ID_A::Exti8)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti9(self) -> &'a mut W {
        self.variant(SIG_ID_A::Exti9)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti10(self) -> &'a mut W {
        self.variant(SIG_ID_A::Exti10)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti11(self) -> &'a mut W {
        self.variant(SIG_ID_A::Exti11)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti12(self) -> &'a mut W {
        self.variant(SIG_ID_A::Exti12)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti13(self) -> &'a mut W {
        self.variant(SIG_ID_A::Exti13)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti14(self) -> &'a mut W {
        self.variant(SIG_ID_A::Exti14)
    }
    ///Signal `EXTIx` selected as synchronization input
    #[inline(always)]
    pub fn exti15(self) -> &'a mut W {
        self.variant(SIG_ID_A::Exti15)
    }
    ///Signal `dmamux1_evt0` selected as synchronization input
    #[inline(always)]
    pub fn dmamux1_evt0(self) -> &'a mut W {
        self.variant(SIG_ID_A::Dmamux1Evt0)
    }
    ///Signal `dmamux1_evt1` selected as synchronization input
    #[inline(always)]
    pub fn dmamux1_evt1(self) -> &'a mut W {
        self.variant(SIG_ID_A::Dmamux1Evt1)
    }
    ///Signal `lptim1_out` selected as synchronization input
    #[inline(always)]
    pub fn lptim1_out(self) -> &'a mut W {
        self.variant(SIG_ID_A::Lptim1Out)
    }
    ///Signal `lptim2_out` selected as synchronization input
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut W {
        self.variant(SIG_ID_A::Lptim2Out)
    }
    ///Signal `lptim3_out` selected as synchronization input
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut W {
        self.variant(SIG_ID_A::Lptim3Out)
    }
}
///Field `OIE` reader - Trigger overrun interrupt enable
pub type OIE_R = crate::BitReader<OIE_A>;
///Trigger overrun interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OIE_A {
    ///0: Trigger overrun interrupt disabled
    Disabled = 0,
    ///1: Trigger overrun interrupt enabled
    Enabled = 1,
}
impl From<OIE_A> for bool {
    #[inline(always)]
    fn from(variant: OIE_A) -> Self {
        variant as u8 != 0
    }
}
impl OIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OIE_A {
        match self.bits {
            false => OIE_A::Disabled,
            true => OIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OIE_A::Enabled
    }
}
///Field `OIE` writer - Trigger overrun interrupt enable
pub type OIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RG1CR_SPEC, OIE_A, O>;
impl<'a, const O: u8> OIE_W<'a, O> {
    ///Trigger overrun interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OIE_A::Disabled)
    }
    ///Trigger overrun interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OIE_A::Enabled)
    }
}
///Field `GE` reader - DMA request generator channel 1 enable
pub type GE_R = crate::BitReader<GE_A>;
///DMA request generator channel 1 enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GE_A {
    ///0: DMA request generation disabled
    Disabled = 0,
    ///1: DMA request enabled
    Enabled = 1,
}
impl From<GE_A> for bool {
    #[inline(always)]
    fn from(variant: GE_A) -> Self {
        variant as u8 != 0
    }
}
impl GE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> GE_A {
        match self.bits {
            false => GE_A::Disabled,
            true => GE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GE_A::Enabled
    }
}
///Field `GE` writer - DMA request generator channel 1 enable
pub type GE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RG1CR_SPEC, GE_A, O>;
impl<'a, const O: u8> GE_W<'a, O> {
    ///DMA request generation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GE_A::Disabled)
    }
    ///DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GE_A::Enabled)
    }
}
///Field `GPOL` reader - DMA request generator trigger polarity
pub type GPOL_R = crate::FieldReader<u8, GPOL_A>;
///DMA request generator trigger polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GPOL_A {
    ///0: No event, i.e. no detection nor generation
    NoEdge = 0,
    ///1: Rising edge
    RisingEdge = 1,
    ///2: Falling edge
    FallingEdge = 2,
    ///3: Rising and falling edges
    BothEdges = 3,
}
impl From<GPOL_A> for u8 {
    #[inline(always)]
    fn from(variant: GPOL_A) -> Self {
        variant as _
    }
}
impl GPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> GPOL_A {
        match self.bits {
            0 => GPOL_A::NoEdge,
            1 => GPOL_A::RisingEdge,
            2 => GPOL_A::FallingEdge,
            3 => GPOL_A::BothEdges,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NoEdge`
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == GPOL_A::NoEdge
    }
    ///Checks if the value of the field is `RisingEdge`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == GPOL_A::RisingEdge
    }
    ///Checks if the value of the field is `FallingEdge`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == GPOL_A::FallingEdge
    }
    ///Checks if the value of the field is `BothEdges`
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == GPOL_A::BothEdges
    }
}
///Field `GPOL` writer - DMA request generator trigger polarity
pub type GPOL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, RG1CR_SPEC, u8, GPOL_A, 2, O>;
impl<'a, const O: u8> GPOL_W<'a, O> {
    ///No event, i.e. no detection nor generation
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut W {
        self.variant(GPOL_A::NoEdge)
    }
    ///Rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(GPOL_A::RisingEdge)
    }
    ///Falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(GPOL_A::FallingEdge)
    }
    ///Rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(GPOL_A::BothEdges)
    }
}
///Field `GNBREQ` reader - Number of DMA requests to be generated minus 1
pub type GNBREQ_R = crate::FieldReader<u8, u8>;
///Field `GNBREQ` writer - Number of DMA requests to be generated minus 1
pub type GNBREQ_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, RG1CR_SPEC, u8, u8, 5, O>;
impl R {
    ///Bits 0:4 - Signal identification
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 8 - Trigger overrun interrupt enable
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - DMA request generator channel 1 enable
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - DMA request generator trigger polarity
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:23 - Number of DMA requests to be generated minus 1
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:4 - Signal identification
    #[inline(always)]
    #[must_use]
    pub fn sig_id(&mut self) -> SIG_ID_W<0> {
        SIG_ID_W::new(self)
    }
    ///Bit 8 - Trigger overrun interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn oie(&mut self) -> OIE_W<8> {
        OIE_W::new(self)
    }
    ///Bit 16 - DMA request generator channel 1 enable
    #[inline(always)]
    #[must_use]
    pub fn ge(&mut self) -> GE_W<16> {
        GE_W::new(self)
    }
    ///Bits 17:18 - DMA request generator trigger polarity
    #[inline(always)]
    #[must_use]
    pub fn gpol(&mut self) -> GPOL_W<17> {
        GPOL_W::new(self)
    }
    ///Bits 19:23 - Number of DMA requests to be generated minus 1
    #[inline(always)]
    #[must_use]
    pub fn gnbreq(&mut self) -> GNBREQ_W<19> {
        GNBREQ_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///request generator channel 1 configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rg1cr](index.html) module
pub struct RG1CR_SPEC;
impl crate::RegisterSpec for RG1CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rg1cr::R](R) reader structure
impl crate::Readable for RG1CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rg1cr::W](W) writer structure
impl crate::Writable for RG1CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RG1CR to value 0
impl crate::Resettable for RG1CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
