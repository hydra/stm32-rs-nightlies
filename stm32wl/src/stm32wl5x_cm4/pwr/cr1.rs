///Register `CR1` reader
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR1` writer
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LPMS` reader - Low-power mode selection for CPU1
pub type LPMS_R = crate::FieldReader<u8, LPMS_A>;
///Low-power mode selection for CPU1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPMS_A {
    ///0: Stop 0 mode
    Stop0 = 0,
    ///1: Stop 1 mode
    Stop1 = 1,
    ///2: Stop 2 mode
    Stop2 = 2,
    ///3: Standby mode
    Standby = 3,
    ///4: Shutdown mode
    Shutdown = 4,
}
impl From<LPMS_A> for u8 {
    #[inline(always)]
    fn from(variant: LPMS_A) -> Self {
        variant as _
    }
}
impl LPMS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<LPMS_A> {
        match self.bits {
            0 => Some(LPMS_A::Stop0),
            1 => Some(LPMS_A::Stop1),
            2 => Some(LPMS_A::Stop2),
            3 => Some(LPMS_A::Standby),
            4 => Some(LPMS_A::Shutdown),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Stop0`
    #[inline(always)]
    pub fn is_stop0(&self) -> bool {
        *self == LPMS_A::Stop0
    }
    ///Checks if the value of the field is `Stop1`
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == LPMS_A::Stop1
    }
    ///Checks if the value of the field is `Stop2`
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == LPMS_A::Stop2
    }
    ///Checks if the value of the field is `Standby`
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == LPMS_A::Standby
    }
    ///Checks if the value of the field is `Shutdown`
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == LPMS_A::Shutdown
    }
}
///Field `LPMS` writer - Low-power mode selection for CPU1
pub type LPMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, LPMS_A, 3, O>;
impl<'a, const O: u8> LPMS_W<'a, O> {
    ///Stop 0 mode
    #[inline(always)]
    pub fn stop0(self) -> &'a mut W {
        self.variant(LPMS_A::Stop0)
    }
    ///Stop 1 mode
    #[inline(always)]
    pub fn stop1(self) -> &'a mut W {
        self.variant(LPMS_A::Stop1)
    }
    ///Stop 2 mode
    #[inline(always)]
    pub fn stop2(self) -> &'a mut W {
        self.variant(LPMS_A::Stop2)
    }
    ///Standby mode
    #[inline(always)]
    pub fn standby(self) -> &'a mut W {
        self.variant(LPMS_A::Standby)
    }
    ///Shutdown mode
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut W {
        self.variant(LPMS_A::Shutdown)
    }
}
///Field `SUBGHZSPINSSSEL` reader - sub-GHz SPI NSS source select
pub type SUBGHZSPINSSSEL_R = crate::BitReader<SUBGHZSPINSSSEL_A>;
///sub-GHz SPI NSS source select
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUBGHZSPINSSSEL_A {
    ///0: sub-GHz SPI NSS signal driven from PWR_SUBGHZSPICR.NSS (RFBUSYMS functionality enabled)
    Subghzspicr = 0,
    ///1: sub-GHz SPI NSS signal driven from LPTIM3_OUT (RFBUSYMS functionality disabled)
    Lptim3 = 1,
}
impl From<SUBGHZSPINSSSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SUBGHZSPINSSSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SUBGHZSPINSSSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SUBGHZSPINSSSEL_A {
        match self.bits {
            false => SUBGHZSPINSSSEL_A::Subghzspicr,
            true => SUBGHZSPINSSSEL_A::Lptim3,
        }
    }
    ///Checks if the value of the field is `Subghzspicr`
    #[inline(always)]
    pub fn is_subghzspicr(&self) -> bool {
        *self == SUBGHZSPINSSSEL_A::Subghzspicr
    }
    ///Checks if the value of the field is `Lptim3`
    #[inline(always)]
    pub fn is_lptim3(&self) -> bool {
        *self == SUBGHZSPINSSSEL_A::Lptim3
    }
}
///Field `SUBGHZSPINSSSEL` writer - sub-GHz SPI NSS source select
pub type SUBGHZSPINSSSEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CR1_SPEC, SUBGHZSPINSSSEL_A, O>;
impl<'a, const O: u8> SUBGHZSPINSSSEL_W<'a, O> {
    ///sub-GHz SPI NSS signal driven from PWR_SUBGHZSPICR.NSS (RFBUSYMS functionality enabled)
    #[inline(always)]
    pub fn subghzspicr(self) -> &'a mut W {
        self.variant(SUBGHZSPINSSSEL_A::Subghzspicr)
    }
    ///sub-GHz SPI NSS signal driven from LPTIM3_OUT (RFBUSYMS functionality disabled)
    #[inline(always)]
    pub fn lptim3(self) -> &'a mut W {
        self.variant(SUBGHZSPINSSSEL_A::Lptim3)
    }
}
///Field `FPDR` reader - Flash memory power down mode during LPRun for CPU1
pub type FPDR_R = crate::BitReader<FPDR_A>;
///Flash memory power down mode during LPRun for CPU1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPDR_A {
    ///0: Flash memory in Idle mode when system is in LPRun mode
    Idle = 0,
    ///1: Flash memory in Power-down mode when system is in LPRun mode
    PowerDown = 1,
}
impl From<FPDR_A> for bool {
    #[inline(always)]
    fn from(variant: FPDR_A) -> Self {
        variant as u8 != 0
    }
}
impl FPDR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FPDR_A {
        match self.bits {
            false => FPDR_A::Idle,
            true => FPDR_A::PowerDown,
        }
    }
    ///Checks if the value of the field is `Idle`
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == FPDR_A::Idle
    }
    ///Checks if the value of the field is `PowerDown`
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == FPDR_A::PowerDown
    }
}
///Field `FPDR` writer - Flash memory power down mode during LPRun for CPU1
pub type FPDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, FPDR_A, O>;
impl<'a, const O: u8> FPDR_W<'a, O> {
    ///Flash memory in Idle mode when system is in LPRun mode
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(FPDR_A::Idle)
    }
    ///Flash memory in Power-down mode when system is in LPRun mode
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(FPDR_A::PowerDown)
    }
}
///Field `FPDS` reader - Flash memory power down mode during LPSleep for CPU1
pub type FPDS_R = crate::BitReader<FPDS_A>;
///Flash memory power down mode during LPSleep for CPU1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPDS_A {
    ///0: Flash memory in Idle mode when system is in LPSleep mode
    Idle = 0,
    ///1: Flash memory in Power-down mode when system is in LPSleep mode
    PowerDown = 1,
}
impl From<FPDS_A> for bool {
    #[inline(always)]
    fn from(variant: FPDS_A) -> Self {
        variant as u8 != 0
    }
}
impl FPDS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FPDS_A {
        match self.bits {
            false => FPDS_A::Idle,
            true => FPDS_A::PowerDown,
        }
    }
    ///Checks if the value of the field is `Idle`
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == FPDS_A::Idle
    }
    ///Checks if the value of the field is `PowerDown`
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == FPDS_A::PowerDown
    }
}
///Field `FPDS` writer - Flash memory power down mode during LPSleep for CPU1
pub type FPDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, FPDS_A, O>;
impl<'a, const O: u8> FPDS_W<'a, O> {
    ///Flash memory in Idle mode when system is in LPSleep mode
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(FPDS_A::Idle)
    }
    ///Flash memory in Power-down mode when system is in LPSleep mode
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(FPDS_A::PowerDown)
    }
}
///Field `DBP` reader - Disable backup domain write protection
pub type DBP_R = crate::BitReader<DBP_A>;
///Disable backup domain write protection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBP_A {
    ///0: Access to RTC and backup registers disabled
    Disabled = 0,
    ///1: Access to RTC and backup registers enabled
    Enabled = 1,
}
impl From<DBP_A> for bool {
    #[inline(always)]
    fn from(variant: DBP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBP_A {
        match self.bits {
            false => DBP_A::Disabled,
            true => DBP_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBP_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBP_A::Enabled
    }
}
///Field `DBP` writer - Disable backup domain write protection
pub type DBP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, DBP_A, O>;
impl<'a, const O: u8> DBP_W<'a, O> {
    ///Access to RTC and backup registers disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBP_A::Disabled)
    }
    ///Access to RTC and backup registers enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBP_A::Enabled)
    }
}
///Field `VOS` reader - Voltage scaling range selection
pub type VOS_R = crate::FieldReader<u8, VOS_A>;
///Voltage scaling range selection
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VOS_A {
    ///1: 1.2 V (range 1)
    V12 = 1,
    ///2: 1.0 V (range 2)
    V10 = 2,
}
impl From<VOS_A> for u8 {
    #[inline(always)]
    fn from(variant: VOS_A) -> Self {
        variant as _
    }
}
impl VOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<VOS_A> {
        match self.bits {
            1 => Some(VOS_A::V12),
            2 => Some(VOS_A::V10),
            _ => None,
        }
    }
    ///Checks if the value of the field is `V12`
    #[inline(always)]
    pub fn is_v1_2(&self) -> bool {
        *self == VOS_A::V12
    }
    ///Checks if the value of the field is `V10`
    #[inline(always)]
    pub fn is_v1_0(&self) -> bool {
        *self == VOS_A::V10
    }
}
///Field `VOS` writer - Voltage scaling range selection
pub type VOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, VOS_A, 2, O>;
impl<'a, const O: u8> VOS_W<'a, O> {
    ///1.2 V (range 1)
    #[inline(always)]
    pub fn v1_2(self) -> &'a mut W {
        self.variant(VOS_A::V12)
    }
    ///1.0 V (range 2)
    #[inline(always)]
    pub fn v1_0(self) -> &'a mut W {
        self.variant(VOS_A::V10)
    }
}
///Field `LPR` reader - Low-power run
pub type LPR_R = crate::BitReader<LPR_A>;
///Low-power run
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPR_A {
    ///0: Voltage regulator in Main mode in Low-power run mode
    MainMode = 0,
    ///1: Voltage regulator in low-power mode in Low-power run mode
    LowPowerMode = 1,
}
impl From<LPR_A> for bool {
    #[inline(always)]
    fn from(variant: LPR_A) -> Self {
        variant as u8 != 0
    }
}
impl LPR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LPR_A {
        match self.bits {
            false => LPR_A::MainMode,
            true => LPR_A::LowPowerMode,
        }
    }
    ///Checks if the value of the field is `MainMode`
    #[inline(always)]
    pub fn is_main_mode(&self) -> bool {
        *self == LPR_A::MainMode
    }
    ///Checks if the value of the field is `LowPowerMode`
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        *self == LPR_A::LowPowerMode
    }
}
///Field `LPR` writer - Low-power run
pub type LPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, LPR_A, O>;
impl<'a, const O: u8> LPR_W<'a, O> {
    ///Voltage regulator in Main mode in Low-power run mode
    #[inline(always)]
    pub fn main_mode(self) -> &'a mut W {
        self.variant(LPR_A::MainMode)
    }
    ///Voltage regulator in low-power mode in Low-power run mode
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut W {
        self.variant(LPR_A::LowPowerMode)
    }
}
impl R {
    ///Bits 0:2 - Low-power mode selection for CPU1
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - sub-GHz SPI NSS source select
    #[inline(always)]
    pub fn subghzspinsssel(&self) -> SUBGHZSPINSSSEL_R {
        SUBGHZSPINSSSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Flash memory power down mode during LPRun for CPU1
    #[inline(always)]
    pub fn fpdr(&self) -> FPDR_R {
        FPDR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Flash memory power down mode during LPSleep for CPU1
    #[inline(always)]
    pub fn fpds(&self) -> FPDS_R {
        FPDS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - Voltage scaling range selection
    #[inline(always)]
    pub fn vos(&self) -> VOS_R {
        VOS_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bit 14 - Low-power run
    #[inline(always)]
    pub fn lpr(&self) -> LPR_R {
        LPR_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - Low-power mode selection for CPU1
    #[inline(always)]
    #[must_use]
    pub fn lpms(&mut self) -> LPMS_W<0> {
        LPMS_W::new(self)
    }
    ///Bit 3 - sub-GHz SPI NSS source select
    #[inline(always)]
    #[must_use]
    pub fn subghzspinsssel(&mut self) -> SUBGHZSPINSSSEL_W<3> {
        SUBGHZSPINSSSEL_W::new(self)
    }
    ///Bit 4 - Flash memory power down mode during LPRun for CPU1
    #[inline(always)]
    #[must_use]
    pub fn fpdr(&mut self) -> FPDR_W<4> {
        FPDR_W::new(self)
    }
    ///Bit 5 - Flash memory power down mode during LPSleep for CPU1
    #[inline(always)]
    #[must_use]
    pub fn fpds(&mut self) -> FPDS_W<5> {
        FPDS_W::new(self)
    }
    ///Bit 8 - Disable backup domain write protection
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DBP_W<8> {
        DBP_W::new(self)
    }
    ///Bits 9:10 - Voltage scaling range selection
    #[inline(always)]
    #[must_use]
    pub fn vos(&mut self) -> VOS_W<9> {
        VOS_W::new(self)
    }
    ///Bit 14 - Low-power run
    #[inline(always)]
    #[must_use]
    pub fn lpr(&mut self) -> LPR_W<14> {
        LPR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Power control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr1](index.html) module
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr1::R](R) reader structure
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr1::W](W) writer structure
impl crate::Writable for CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR1 to value 0x0200
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
