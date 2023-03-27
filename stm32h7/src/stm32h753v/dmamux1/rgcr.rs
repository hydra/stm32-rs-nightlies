///Register `RGCR%s` reader
pub struct R(crate::R<RGCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RGCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RGCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RGCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RGCR%s` writer
pub struct W(crate::W<RGCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RGCR_SPEC>;
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
impl From<crate::W<RGCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RGCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SIG_ID` reader - DMA request trigger input selected
pub type SIG_ID_R = crate::FieldReader<u8, SIG_ID_A>;
///DMA request trigger input selected
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIG_ID_A {
    ///0: Signal `dmamux1_evt0` selected as trigger input
    Dmamux1Evt0 = 0,
    ///1: Signal `dmamux1_evt1` selected as trigger input
    Dmamux1Evt1 = 1,
    ///2: Signal `dmamux1_evt2` selected as trigger input
    Dmamux1Evt2 = 2,
    ///3: Signal `lptim1_out` selected as trigger input
    Lptim1Out = 3,
    ///4: Signal `lptim2_out` selected as trigger input
    Lptim2Out = 4,
    ///5: Signal `lptim3_out` selected as trigger input
    Lptim3Out = 5,
    ///6: Signal `extit0` selected as trigger input
    Extit0 = 6,
    ///7: Signal `tim12_trgo` selected as trigger input
    Tim12Trgo = 7,
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
            0 => Some(SIG_ID_A::Dmamux1Evt0),
            1 => Some(SIG_ID_A::Dmamux1Evt1),
            2 => Some(SIG_ID_A::Dmamux1Evt2),
            3 => Some(SIG_ID_A::Lptim1Out),
            4 => Some(SIG_ID_A::Lptim2Out),
            5 => Some(SIG_ID_A::Lptim3Out),
            6 => Some(SIG_ID_A::Extit0),
            7 => Some(SIG_ID_A::Tim12Trgo),
            _ => None,
        }
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
    ///Checks if the value of the field is `Dmamux1Evt2`
    #[inline(always)]
    pub fn is_dmamux1_evt2(&self) -> bool {
        *self == SIG_ID_A::Dmamux1Evt2
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
    ///Checks if the value of the field is `Extit0`
    #[inline(always)]
    pub fn is_extit0(&self) -> bool {
        *self == SIG_ID_A::Extit0
    }
    ///Checks if the value of the field is `Tim12Trgo`
    #[inline(always)]
    pub fn is_tim12_trgo(&self) -> bool {
        *self == SIG_ID_A::Tim12Trgo
    }
}
///Field `SIG_ID` writer - DMA request trigger input selected
pub type SIG_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RGCR_SPEC, u8, SIG_ID_A, 5, O>;
impl<'a, const O: u8> SIG_ID_W<'a, O> {
    ///Signal `dmamux1_evt0` selected as trigger input
    #[inline(always)]
    pub fn dmamux1_evt0(self) -> &'a mut W {
        self.variant(SIG_ID_A::Dmamux1Evt0)
    }
    ///Signal `dmamux1_evt1` selected as trigger input
    #[inline(always)]
    pub fn dmamux1_evt1(self) -> &'a mut W {
        self.variant(SIG_ID_A::Dmamux1Evt1)
    }
    ///Signal `dmamux1_evt2` selected as trigger input
    #[inline(always)]
    pub fn dmamux1_evt2(self) -> &'a mut W {
        self.variant(SIG_ID_A::Dmamux1Evt2)
    }
    ///Signal `lptim1_out` selected as trigger input
    #[inline(always)]
    pub fn lptim1_out(self) -> &'a mut W {
        self.variant(SIG_ID_A::Lptim1Out)
    }
    ///Signal `lptim2_out` selected as trigger input
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut W {
        self.variant(SIG_ID_A::Lptim2Out)
    }
    ///Signal `lptim3_out` selected as trigger input
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut W {
        self.variant(SIG_ID_A::Lptim3Out)
    }
    ///Signal `extit0` selected as trigger input
    #[inline(always)]
    pub fn extit0(self) -> &'a mut W {
        self.variant(SIG_ID_A::Extit0)
    }
    ///Signal `tim12_trgo` selected as trigger input
    #[inline(always)]
    pub fn tim12_trgo(self) -> &'a mut W {
        self.variant(SIG_ID_A::Tim12Trgo)
    }
}
///Field `OIE` reader - Interrupt enable at trigger event overrun
pub type OIE_R = crate::BitReader<OIE_A>;
///Interrupt enable at trigger event overrun
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
///Field `OIE` writer - Interrupt enable at trigger event overrun
pub type OIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGCR_SPEC, OIE_A, O>;
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
///Field `GE` reader - DMA request generator channel enable/disable
pub type GE_R = crate::BitReader<GE_A>;
///DMA request generator channel enable/disable
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
///Field `GE` writer - DMA request generator channel enable/disable
pub type GE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGCR_SPEC, GE_A, O>;
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
///Field `GPOL` reader - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input
pub type GPOL_R = crate::FieldReader<u8, GPOL_A>;
///DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input
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
///Field `GPOL` writer - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input
pub type GPOL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, RGCR_SPEC, u8, GPOL_A, 2, O>;
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
///Field `GNBREQ` reader - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset.
pub type GNBREQ_R = crate::FieldReader<u8, u8>;
///Field `GNBREQ` writer - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset.
pub type GNBREQ_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, RGCR_SPEC, u8, u8, 5, O>;
impl R {
    ///Bits 0:4 - DMA request trigger input selected
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 8 - Interrupt enable at trigger event overrun
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - DMA request generator channel enable/disable
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:23 - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset.
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:4 - DMA request trigger input selected
    #[inline(always)]
    #[must_use]
    pub fn sig_id(&mut self) -> SIG_ID_W<0> {
        SIG_ID_W::new(self)
    }
    ///Bit 8 - Interrupt enable at trigger event overrun
    #[inline(always)]
    #[must_use]
    pub fn oie(&mut self) -> OIE_W<8> {
        OIE_W::new(self)
    }
    ///Bit 16 - DMA request generator channel enable/disable
    #[inline(always)]
    #[must_use]
    pub fn ge(&mut self) -> GE_W<16> {
        GE_W::new(self)
    }
    ///Bits 17:18 - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input
    #[inline(always)]
    #[must_use]
    pub fn gpol(&mut self) -> GPOL_W<17> {
        GPOL_W::new(self)
    }
    ///Bits 19:23 - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset.
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
///DMAMux - DMA request generator channel x control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rgcr](index.html) module
pub struct RGCR_SPEC;
impl crate::RegisterSpec for RGCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rgcr::R](R) reader structure
impl crate::Readable for RGCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rgcr::W](W) writer structure
impl crate::Writable for RGCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RGCR%s to value 0
impl crate::Resettable for RGCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
