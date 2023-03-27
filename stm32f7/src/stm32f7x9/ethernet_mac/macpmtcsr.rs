///Register `MACPMTCSR` reader
pub struct R(crate::R<MACPMTCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACPMTCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACPMTCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACPMTCSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACPMTCSR` writer
pub struct W(crate::W<MACPMTCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACPMTCSR_SPEC>;
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
impl From<crate::W<MACPMTCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACPMTCSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PD` reader - Power down
pub type PD_R = crate::BitReader<PD_A>;
///Power down
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD_A {
    ///1: All received frames will be dropped. Cleared automatically when a magic packet or wakeup frame is received
    Enabled = 1,
}
impl From<PD_A> for bool {
    #[inline(always)]
    fn from(variant: PD_A) -> Self {
        variant as u8 != 0
    }
}
impl PD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PD_A> {
        match self.bits {
            true => Some(PD_A::Enabled),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PD_A::Enabled
    }
}
///Field `PD` writer - Power down
pub type PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, PD_A, O>;
impl<'a, const O: u8> PD_W<'a, O> {
    ///All received frames will be dropped. Cleared automatically when a magic packet or wakeup frame is received
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD_A::Enabled)
    }
}
///Field `MPE` reader - Magic packet enable
pub type MPE_R = crate::BitReader<MPE_A>;
///Magic packet enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MPE_A {
    ///0: No power management event generated due to Magic Packet reception
    Disabled = 0,
    ///1: Enable generation of a power management event due to Magic Packet reception
    Enabled = 1,
}
impl From<MPE_A> for bool {
    #[inline(always)]
    fn from(variant: MPE_A) -> Self {
        variant as u8 != 0
    }
}
impl MPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MPE_A {
        match self.bits {
            false => MPE_A::Disabled,
            true => MPE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MPE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MPE_A::Enabled
    }
}
///Field `MPE` writer - Magic packet enable
pub type MPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, MPE_A, O>;
impl<'a, const O: u8> MPE_W<'a, O> {
    ///No power management event generated due to Magic Packet reception
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MPE_A::Disabled)
    }
    ///Enable generation of a power management event due to Magic Packet reception
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MPE_A::Enabled)
    }
}
///Field `WFE` reader - Wakeup frame enable
pub type WFE_R = crate::BitReader<WFE_A>;
///Wakeup frame enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WFE_A {
    ///0: No power management event generated due to wakeup frame reception
    Disabled = 0,
    ///1: Enable generation of a power management event due to wakeup frame reception
    Enabled = 1,
}
impl From<WFE_A> for bool {
    #[inline(always)]
    fn from(variant: WFE_A) -> Self {
        variant as u8 != 0
    }
}
impl WFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WFE_A {
        match self.bits {
            false => WFE_A::Disabled,
            true => WFE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WFE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WFE_A::Enabled
    }
}
///Field `WFE` writer - Wakeup frame enable
pub type WFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, WFE_A, O>;
impl<'a, const O: u8> WFE_W<'a, O> {
    ///No power management event generated due to wakeup frame reception
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WFE_A::Disabled)
    }
    ///Enable generation of a power management event due to wakeup frame reception
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WFE_A::Enabled)
    }
}
///Field `MPR` reader - Magic packet received
pub type MPR_R = crate::BitReader<bool>;
///Field `MPR` writer - Magic packet received
pub type MPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, O>;
///Field `WFR` reader - Wakeup frame received
pub type WFR_R = crate::BitReader<bool>;
///Field `WFR` writer - Wakeup frame received
pub type WFR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, bool, O>;
///Field `GU` reader - Global unicast
pub type GU_R = crate::BitReader<GU_A>;
///Global unicast
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GU_A {
    ///0: Normal operation
    Disabled = 0,
    ///1: Any unicast packet filtered by the MAC address recognition may be a wakeup frame
    Enabled = 1,
}
impl From<GU_A> for bool {
    #[inline(always)]
    fn from(variant: GU_A) -> Self {
        variant as u8 != 0
    }
}
impl GU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> GU_A {
        match self.bits {
            false => GU_A::Disabled,
            true => GU_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GU_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GU_A::Enabled
    }
}
///Field `GU` writer - Global unicast
pub type GU_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, GU_A, O>;
impl<'a, const O: u8> GU_W<'a, O> {
    ///Normal operation
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GU_A::Disabled)
    }
    ///Any unicast packet filtered by the MAC address recognition may be a wakeup frame
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GU_A::Enabled)
    }
}
///Field `WFFRPR` reader - Wakeup frame filter register pointer reset
pub type WFFRPR_R = crate::BitReader<WFFRPR_A>;
///Wakeup frame filter register pointer reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WFFRPR_A {
    ///1: Reset wakeup frame filter register point to 0b000. Automatically cleared
    Reset = 1,
}
impl From<WFFRPR_A> for bool {
    #[inline(always)]
    fn from(variant: WFFRPR_A) -> Self {
        variant as u8 != 0
    }
}
impl WFFRPR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<WFFRPR_A> {
        match self.bits {
            true => Some(WFFRPR_A::Reset),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == WFFRPR_A::Reset
    }
}
///Field `WFFRPR` writer - Wakeup frame filter register pointer reset
pub type WFFRPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACPMTCSR_SPEC, WFFRPR_A, O>;
impl<'a, const O: u8> WFFRPR_W<'a, O> {
    ///Reset wakeup frame filter register point to 0b000. Automatically cleared
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(WFFRPR_A::Reset)
    }
}
impl R {
    ///Bit 0 - Power down
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Magic packet enable
    #[inline(always)]
    pub fn mpe(&self) -> MPE_R {
        MPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup frame enable
    #[inline(always)]
    pub fn wfe(&self) -> WFE_R {
        WFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - Magic packet received
    #[inline(always)]
    pub fn mpr(&self) -> MPR_R {
        MPR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Wakeup frame received
    #[inline(always)]
    pub fn wfr(&self) -> WFR_R {
        WFR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - Global unicast
    #[inline(always)]
    pub fn gu(&self) -> GU_R {
        GU_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 31 - Wakeup frame filter register pointer reset
    #[inline(always)]
    pub fn wffrpr(&self) -> WFFRPR_R {
        WFFRPR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Power down
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<0> {
        PD_W::new(self)
    }
    ///Bit 1 - Magic packet enable
    #[inline(always)]
    #[must_use]
    pub fn mpe(&mut self) -> MPE_W<1> {
        MPE_W::new(self)
    }
    ///Bit 2 - Wakeup frame enable
    #[inline(always)]
    #[must_use]
    pub fn wfe(&mut self) -> WFE_W<2> {
        WFE_W::new(self)
    }
    ///Bit 5 - Magic packet received
    #[inline(always)]
    #[must_use]
    pub fn mpr(&mut self) -> MPR_W<5> {
        MPR_W::new(self)
    }
    ///Bit 6 - Wakeup frame received
    #[inline(always)]
    #[must_use]
    pub fn wfr(&mut self) -> WFR_W<6> {
        WFR_W::new(self)
    }
    ///Bit 9 - Global unicast
    #[inline(always)]
    #[must_use]
    pub fn gu(&mut self) -> GU_W<9> {
        GU_W::new(self)
    }
    ///Bit 31 - Wakeup frame filter register pointer reset
    #[inline(always)]
    #[must_use]
    pub fn wffrpr(&mut self) -> WFFRPR_W<31> {
        WFFRPR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet MAC PMT control and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macpmtcsr](index.html) module
pub struct MACPMTCSR_SPEC;
impl crate::RegisterSpec for MACPMTCSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macpmtcsr::R](R) reader structure
impl crate::Readable for MACPMTCSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macpmtcsr::W](W) writer structure
impl crate::Writable for MACPMTCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACPMTCSR to value 0
impl crate::Resettable for MACPMTCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
