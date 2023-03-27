///Register `C2CR3` reader
pub struct R(crate::R<C2CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2CR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C2CR3` writer
pub struct W(crate::W<C2CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2CR3_SPEC>;
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
impl From<crate::W<C2CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2CR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EWUP1` reader - Enable Wakeup pin WKUP1 for CPU2
pub type EWUP1_R = crate::BitReader<EWUP1_A>;
///Enable Wakeup pin WKUP1 for CPU2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWUP1_A {
    ///0: WKUP pin 1 is used for general purpose I/Os. An event on the WKUP pin 1 does not wakeup the device from Standby mode
    Disabled = 0,
    ///1: WKUP pin 1 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 1 wakes-up the system from Standby mode)
    Enabled = 1,
}
impl From<EWUP1_A> for bool {
    #[inline(always)]
    fn from(variant: EWUP1_A) -> Self {
        variant as u8 != 0
    }
}
impl EWUP1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EWUP1_A {
        match self.bits {
            false => EWUP1_A::Disabled,
            true => EWUP1_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWUP1_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWUP1_A::Enabled
    }
}
///Field `EWUP1` writer - Enable Wakeup pin WKUP1 for CPU2
pub type EWUP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, EWUP1_A, O>;
impl<'a, const O: u8> EWUP1_W<'a, O> {
    ///WKUP pin 1 is used for general purpose I/Os. An event on the WKUP pin 1 does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWUP1_A::Disabled)
    }
    ///WKUP pin 1 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 1 wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWUP1_A::Enabled)
    }
}
///Field `EWUP2` reader - Enable Wakeup pin WKUP2 for CPU2
pub type EWUP2_R = crate::BitReader<EWUP2_A>;
///Enable Wakeup pin WKUP2 for CPU2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWUP2_A {
    ///0: WKUP pin 2 is used for general purpose I/Os. An event on the WKUP pin 2 does not wakeup the device from Standby mode
    Disabled = 0,
    ///1: WKUP pin 2 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 2 wakes-up the system from Standby mode)
    Enabled = 1,
}
impl From<EWUP2_A> for bool {
    #[inline(always)]
    fn from(variant: EWUP2_A) -> Self {
        variant as u8 != 0
    }
}
impl EWUP2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EWUP2_A {
        match self.bits {
            false => EWUP2_A::Disabled,
            true => EWUP2_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWUP2_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWUP2_A::Enabled
    }
}
///Field `EWUP2` writer - Enable Wakeup pin WKUP2 for CPU2
pub type EWUP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, EWUP2_A, O>;
impl<'a, const O: u8> EWUP2_W<'a, O> {
    ///WKUP pin 2 is used for general purpose I/Os. An event on the WKUP pin 2 does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWUP2_A::Disabled)
    }
    ///WKUP pin 2 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 2 wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWUP2_A::Enabled)
    }
}
///Field `EWUP3` reader - Enable Wakeup pin WKUP3 for CPU2
pub type EWUP3_R = crate::BitReader<EWUP3_A>;
///Enable Wakeup pin WKUP3 for CPU2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWUP3_A {
    ///0: WKUP pin 3 is used for general purpose I/Os. An event on the WKUP pin 3 does not wakeup the device from Standby mode
    Disabled = 0,
    ///1: WKUP pin 3 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 3wakes-up the system from Standby mode)
    Enabled = 1,
}
impl From<EWUP3_A> for bool {
    #[inline(always)]
    fn from(variant: EWUP3_A) -> Self {
        variant as u8 != 0
    }
}
impl EWUP3_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EWUP3_A {
        match self.bits {
            false => EWUP3_A::Disabled,
            true => EWUP3_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWUP3_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWUP3_A::Enabled
    }
}
///Field `EWUP3` writer - Enable Wakeup pin WKUP3 for CPU2
pub type EWUP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, EWUP3_A, O>;
impl<'a, const O: u8> EWUP3_W<'a, O> {
    ///WKUP pin 3 is used for general purpose I/Os. An event on the WKUP pin 3 does not wakeup the device from Standby mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWUP3_A::Disabled)
    }
    ///WKUP pin 3 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 3wakes-up the system from Standby mode)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWUP3_A::Enabled)
    }
}
///Field `EWPVD` reader - Enable wakeup PVD for CPU2
pub type EWPVD_R = crate::BitReader<EWPVD_A>;
///Enable wakeup PVD for CPU2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWPVD_A {
    ///0: PVD not enabled by the sub-GHz radio active state
    Disabled = 0,
    ///1: PVD enabled while the sub-GHz radio is active
    Enabled = 1,
}
impl From<EWPVD_A> for bool {
    #[inline(always)]
    fn from(variant: EWPVD_A) -> Self {
        variant as u8 != 0
    }
}
impl EWPVD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EWPVD_A {
        match self.bits {
            false => EWPVD_A::Disabled,
            true => EWPVD_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWPVD_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWPVD_A::Enabled
    }
}
///Field `EWPVD` writer - Enable wakeup PVD for CPU2
pub type EWPVD_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, EWPVD_A, O>;
impl<'a, const O: u8> EWPVD_W<'a, O> {
    ///PVD not enabled by the sub-GHz radio active state
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWPVD_A::Disabled)
    }
    ///PVD enabled while the sub-GHz radio is active
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWPVD_A::Enabled)
    }
}
///Field `APC` reader - Apply pull-up and pull-down configuration for CPU2
pub type APC_R = crate::BitReader<APC_A>;
///Apply pull-up and pull-down configuration for CPU2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum APC_A {
    ///0: I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied
    Disabled = 0,
    ///1: PWR_PUCRx and PWR_PDCRx registers are NOT applied to the I/Os
    Enabled = 1,
}
impl From<APC_A> for bool {
    #[inline(always)]
    fn from(variant: APC_A) -> Self {
        variant as u8 != 0
    }
}
impl APC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> APC_A {
        match self.bits {
            false => APC_A::Disabled,
            true => APC_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == APC_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == APC_A::Enabled
    }
}
///Field `APC` writer - Apply pull-up and pull-down configuration for CPU2
pub type APC_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, APC_A, O>;
impl<'a, const O: u8> APC_W<'a, O> {
    ///I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(APC_A::Disabled)
    }
    ///PWR_PUCRx and PWR_PDCRx registers are NOT applied to the I/Os
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(APC_A::Enabled)
    }
}
///Field `EWRFBUSY` reader - EWRFBUSY
pub type EWRFBUSY_R = crate::BitReader<EWRFBUSY_A>;
///EWRFBUSY
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWRFBUSY_A {
    ///0: Radio Busy is disabled and does not trigger a wakeup from Standby event to CPU2 when a rising or a falling edge occurs
    Disabled = 0,
    ///1: Radio Busy is enabled and triggers a wakeup from Standby event to CPU2 when a rising or a falling edge occurs. The active edge is configured via the WRFBUSYP bit in PWR_CR4
    Enabled = 1,
}
impl From<EWRFBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: EWRFBUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl EWRFBUSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EWRFBUSY_A {
        match self.bits {
            false => EWRFBUSY_A::Disabled,
            true => EWRFBUSY_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWRFBUSY_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWRFBUSY_A::Enabled
    }
}
///Field `EWRFBUSY` writer - EWRFBUSY
pub type EWRFBUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, EWRFBUSY_A, O>;
impl<'a, const O: u8> EWRFBUSY_W<'a, O> {
    ///Radio Busy is disabled and does not trigger a wakeup from Standby event to CPU2 when a rising or a falling edge occurs
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWRFBUSY_A::Disabled)
    }
    ///Radio Busy is enabled and triggers a wakeup from Standby event to CPU2 when a rising or a falling edge occurs. The active edge is configured via the WRFBUSYP bit in PWR_CR4
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWRFBUSY_A::Enabled)
    }
}
///Field `EWRFIRQ` reader - akeup for CPU2
pub type EWRFIRQ_R = crate::BitReader<EWRFIRQ_A>;
///akeup for CPU2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWRFIRQ_A {
    ///0: Radio IRQ\[2:0\]
    ///is disabled and does not trigger a wakeup from Standby event to CPU2.
    Disabled = 0,
    ///1: Radio IRQ\[2:0\]
    ///is enabled and triggers a wakeup from Standby event to CPU2.
    Enabled = 1,
}
impl From<EWRFIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: EWRFIRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl EWRFIRQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EWRFIRQ_A {
        match self.bits {
            false => EWRFIRQ_A::Disabled,
            true => EWRFIRQ_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWRFIRQ_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWRFIRQ_A::Enabled
    }
}
///Field `EWRFIRQ` writer - akeup for CPU2
pub type EWRFIRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, EWRFIRQ_A, O>;
impl<'a, const O: u8> EWRFIRQ_W<'a, O> {
    ///Radio IRQ\[2:0\]
    ///is disabled and does not trigger a wakeup from Standby event to CPU2.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWRFIRQ_A::Disabled)
    }
    ///Radio IRQ\[2:0\]
    ///is enabled and triggers a wakeup from Standby event to CPU2.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWRFIRQ_A::Enabled)
    }
}
///Field `EIWUL` reader - Enable internal wakeup line for CPU2
pub type EIWUL_R = crate::BitReader<EIWUL_A>;
///Enable internal wakeup line for CPU2
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIWUL_A {
    ///0: Internal wakeup line interrupt to CPU2 disabled
    Disabled = 0,
    ///1: Internal wakeup line interrupt to CPU2 enabled
    Enabled = 1,
}
impl From<EIWUL_A> for bool {
    #[inline(always)]
    fn from(variant: EIWUL_A) -> Self {
        variant as u8 != 0
    }
}
impl EIWUL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EIWUL_A {
        match self.bits {
            false => EIWUL_A::Disabled,
            true => EIWUL_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EIWUL_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EIWUL_A::Enabled
    }
}
///Field `EIWUL` writer - Enable internal wakeup line for CPU2
pub type EIWUL_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, EIWUL_A, O>;
impl<'a, const O: u8> EIWUL_W<'a, O> {
    ///Internal wakeup line interrupt to CPU2 disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EIWUL_A::Disabled)
    }
    ///Internal wakeup line interrupt to CPU2 enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EIWUL_A::Enabled)
    }
}
impl R {
    ///Bit 0 - Enable Wakeup pin WKUP1 for CPU2
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Enable Wakeup pin WKUP2 for CPU2
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Enable Wakeup pin WKUP3 for CPU2
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - Enable wakeup PVD for CPU2
    #[inline(always)]
    pub fn ewpvd(&self) -> EWPVD_R {
        EWPVD_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 10 - Apply pull-up and pull-down configuration for CPU2
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - EWRFBUSY
    #[inline(always)]
    pub fn ewrfbusy(&self) -> EWRFBUSY_R {
        EWRFBUSY_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - akeup for CPU2
    #[inline(always)]
    pub fn ewrfirq(&self) -> EWRFIRQ_R {
        EWRFIRQ_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - Enable internal wakeup line for CPU2
    #[inline(always)]
    pub fn eiwul(&self) -> EIWUL_R {
        EIWUL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Enable Wakeup pin WKUP1 for CPU2
    #[inline(always)]
    #[must_use]
    pub fn ewup1(&mut self) -> EWUP1_W<0> {
        EWUP1_W::new(self)
    }
    ///Bit 1 - Enable Wakeup pin WKUP2 for CPU2
    #[inline(always)]
    #[must_use]
    pub fn ewup2(&mut self) -> EWUP2_W<1> {
        EWUP2_W::new(self)
    }
    ///Bit 2 - Enable Wakeup pin WKUP3 for CPU2
    #[inline(always)]
    #[must_use]
    pub fn ewup3(&mut self) -> EWUP3_W<2> {
        EWUP3_W::new(self)
    }
    ///Bit 8 - Enable wakeup PVD for CPU2
    #[inline(always)]
    #[must_use]
    pub fn ewpvd(&mut self) -> EWPVD_W<8> {
        EWPVD_W::new(self)
    }
    ///Bit 10 - Apply pull-up and pull-down configuration for CPU2
    #[inline(always)]
    #[must_use]
    pub fn apc(&mut self) -> APC_W<10> {
        APC_W::new(self)
    }
    ///Bit 11 - EWRFBUSY
    #[inline(always)]
    #[must_use]
    pub fn ewrfbusy(&mut self) -> EWRFBUSY_W<11> {
        EWRFBUSY_W::new(self)
    }
    ///Bit 13 - akeup for CPU2
    #[inline(always)]
    #[must_use]
    pub fn ewrfirq(&mut self) -> EWRFIRQ_W<13> {
        EWRFIRQ_W::new(self)
    }
    ///Bit 15 - Enable internal wakeup line for CPU2
    #[inline(always)]
    #[must_use]
    pub fn eiwul(&mut self) -> EIWUL_W<15> {
        EIWUL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Power CPU2 control register 3 \[dual core device only\]
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2cr3](index.html) module
pub struct C2CR3_SPEC;
impl crate::RegisterSpec for C2CR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [c2cr3::R](R) reader structure
impl crate::Readable for C2CR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c2cr3::W](W) writer structure
impl crate::Writable for C2CR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C2CR3 to value 0x8000
impl crate::Resettable for C2CR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
