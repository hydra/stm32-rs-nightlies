///Register `EXTSCR` reader
pub struct R(crate::R<EXTSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTSCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EXTSCR` writer
pub struct W(crate::W<EXTSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTSCR_SPEC>;
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
impl From<crate::W<EXTSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTSCR_SPEC>) -> Self {
        W(writer)
    }
}
///Clear CPU1 Stop Standby flags
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1CSSFW_AW {
    ///1: Setting this bit clears the C1STOPF and C1SBF bits
    Clear = 1,
}
impl From<C1CSSFW_AW> for bool {
    #[inline(always)]
    fn from(variant: C1CSSFW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `C1CSSF` writer - Clear CPU1 Stop Standby flags
pub type C1CSSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTSCR_SPEC, C1CSSFW_AW, O>;
impl<'a, const O: u8> C1CSSF_W<'a, O> {
    ///Setting this bit clears the C1STOPF and C1SBF bits
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(C1CSSFW_AW::Clear)
    }
}
///Field `C1SBF` reader - System Standby flag for CPU1. (no core states retained)
pub type C1SBF_R = crate::BitReader<C1SBF_A>;
///System Standby flag for CPU1. (no core states retained)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1SBF_A {
    ///0: System has not been in Standby mode
    NoStandby = 0,
    ///1: System has been in Standby mode
    Standby = 1,
}
impl From<C1SBF_A> for bool {
    #[inline(always)]
    fn from(variant: C1SBF_A) -> Self {
        variant as u8 != 0
    }
}
impl C1SBF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> C1SBF_A {
        match self.bits {
            false => C1SBF_A::NoStandby,
            true => C1SBF_A::Standby,
        }
    }
    ///Checks if the value of the field is `NoStandby`
    #[inline(always)]
    pub fn is_no_standby(&self) -> bool {
        *self == C1SBF_A::NoStandby
    }
    ///Checks if the value of the field is `Standby`
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == C1SBF_A::Standby
    }
}
///Field `C1STOP2F` reader - System Stop2 flag for CPU1. (partial core states retained)
pub type C1STOP2F_R = crate::BitReader<C1STOP2F_A>;
///System Stop2 flag for CPU1. (partial core states retained)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1STOP2F_A {
    ///0: System has not been in Stop 2 mode
    NoStop = 0,
    ///1: System has been in Stop 2 mode
    Stop = 1,
}
impl From<C1STOP2F_A> for bool {
    #[inline(always)]
    fn from(variant: C1STOP2F_A) -> Self {
        variant as u8 != 0
    }
}
impl C1STOP2F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> C1STOP2F_A {
        match self.bits {
            false => C1STOP2F_A::NoStop,
            true => C1STOP2F_A::Stop,
        }
    }
    ///Checks if the value of the field is `NoStop`
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == C1STOP2F_A::NoStop
    }
    ///Checks if the value of the field is `Stop`
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == C1STOP2F_A::Stop
    }
}
///Field `C1STOPF` reader - System Stop0, 1 flag for CPU1. (All core states retained)
pub type C1STOPF_R = crate::BitReader<C1STOPF_A>;
///System Stop0, 1 flag for CPU1. (All core states retained)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1STOPF_A {
    ///0: System has not been in Stop 0 or 1 mode
    NoStop = 0,
    ///1: System has been in Stop 0 or 1 mode
    Stop = 1,
}
impl From<C1STOPF_A> for bool {
    #[inline(always)]
    fn from(variant: C1STOPF_A) -> Self {
        variant as u8 != 0
    }
}
impl C1STOPF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> C1STOPF_A {
        match self.bits {
            false => C1STOPF_A::NoStop,
            true => C1STOPF_A::Stop,
        }
    }
    ///Checks if the value of the field is `NoStop`
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == C1STOPF_A::NoStop
    }
    ///Checks if the value of the field is `Stop`
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == C1STOPF_A::Stop
    }
}
///Field `C1DS` reader - CPU1 deepsleep mode
pub type C1DS_R = crate::BitReader<C1DS_A>;
///CPU1 deepsleep mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1DS_A {
    ///0: CPU is running or in sleep
    RunningOrSleep = 0,
    ///1: CPU is in Deep-Sleep
    DeepSleep = 1,
}
impl From<C1DS_A> for bool {
    #[inline(always)]
    fn from(variant: C1DS_A) -> Self {
        variant as u8 != 0
    }
}
impl C1DS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> C1DS_A {
        match self.bits {
            false => C1DS_A::RunningOrSleep,
            true => C1DS_A::DeepSleep,
        }
    }
    ///Checks if the value of the field is `RunningOrSleep`
    #[inline(always)]
    pub fn is_running_or_sleep(&self) -> bool {
        *self == C1DS_A::RunningOrSleep
    }
    ///Checks if the value of the field is `DeepSleep`
    #[inline(always)]
    pub fn is_deep_sleep(&self) -> bool {
        *self == C1DS_A::DeepSleep
    }
}
impl R {
    ///Bit 8 - System Standby flag for CPU1. (no core states retained)
    #[inline(always)]
    pub fn c1sbf(&self) -> C1SBF_R {
        C1SBF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - System Stop2 flag for CPU1. (partial core states retained)
    #[inline(always)]
    pub fn c1stop2f(&self) -> C1STOP2F_R {
        C1STOP2F_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - System Stop0, 1 flag for CPU1. (All core states retained)
    #[inline(always)]
    pub fn c1stopf(&self) -> C1STOPF_R {
        C1STOPF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 14 - CPU1 deepsleep mode
    #[inline(always)]
    pub fn c1ds(&self) -> C1DS_R {
        C1DS_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Clear CPU1 Stop Standby flags
    #[inline(always)]
    #[must_use]
    pub fn c1cssf(&mut self) -> C1CSSF_W<0> {
        C1CSSF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Power extended status and status clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [extscr](index.html) module
pub struct EXTSCR_SPEC;
impl crate::RegisterSpec for EXTSCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [extscr::R](R) reader structure
impl crate::Readable for EXTSCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [extscr::W](W) writer structure
impl crate::Writable for EXTSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EXTSCR to value 0
impl crate::Resettable for EXTSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
