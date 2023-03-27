///Register `CRL` reader
pub struct R(crate::R<CRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CRL` writer
pub struct W(crate::W<CRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRL_SPEC>;
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
impl From<crate::W<CRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SECF` reader - Second Flag
pub type SECF_R = crate::BitReader<SECFR_A>;
///Second Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECFR_A {
    ///0: Second flag condition not met
    NoPrescalerOverflow = 0,
    ///1: Second flag condition met
    PrescalerOverflow = 1,
}
impl From<SECFR_A> for bool {
    #[inline(always)]
    fn from(variant: SECFR_A) -> Self {
        variant as u8 != 0
    }
}
impl SECF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SECFR_A {
        match self.bits {
            false => SECFR_A::NoPrescalerOverflow,
            true => SECFR_A::PrescalerOverflow,
        }
    }
    ///Checks if the value of the field is `NoPrescalerOverflow`
    #[inline(always)]
    pub fn is_no_prescaler_overflow(&self) -> bool {
        *self == SECFR_A::NoPrescalerOverflow
    }
    ///Checks if the value of the field is `PrescalerOverflow`
    #[inline(always)]
    pub fn is_prescaler_overflow(&self) -> bool {
        *self == SECFR_A::PrescalerOverflow
    }
}
///Second Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECFW_AW {
    ///0: Clear flag
    Clear = 0,
}
impl From<SECFW_AW> for bool {
    #[inline(always)]
    fn from(variant: SECFW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `SECF` writer - Second Flag
pub type SECF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, CRL_SPEC, SECFW_AW, O>;
impl<'a, const O: u8> SECF_W<'a, O> {
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SECFW_AW::Clear)
    }
}
///Field `ALRF` reader - Alarm Flag
pub type ALRF_R = crate::BitReader<ALRFR_A>;
///Alarm Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRFR_A {
    ///0: Alarm not detected
    NoAlarm = 0,
    ///1: Alarm detected
    Alarm = 1,
}
impl From<ALRFR_A> for bool {
    #[inline(always)]
    fn from(variant: ALRFR_A) -> Self {
        variant as u8 != 0
    }
}
impl ALRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ALRFR_A {
        match self.bits {
            false => ALRFR_A::NoAlarm,
            true => ALRFR_A::Alarm,
        }
    }
    ///Checks if the value of the field is `NoAlarm`
    #[inline(always)]
    pub fn is_no_alarm(&self) -> bool {
        *self == ALRFR_A::NoAlarm
    }
    ///Checks if the value of the field is `Alarm`
    #[inline(always)]
    pub fn is_alarm(&self) -> bool {
        *self == ALRFR_A::Alarm
    }
}
///Alarm Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRFW_AW {
    ///0: Clear flag
    Clear = 0,
}
impl From<ALRFW_AW> for bool {
    #[inline(always)]
    fn from(variant: ALRFW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRF` writer - Alarm Flag
pub type ALRF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, CRL_SPEC, ALRFW_AW, O>;
impl<'a, const O: u8> ALRF_W<'a, O> {
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ALRFW_AW::Clear)
    }
}
///Field `OWF` reader - Overflow Flag
pub type OWF_R = crate::BitReader<OWFR_A>;
///Overflow Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OWFR_A {
    ///0: Overflow not detected
    NoOverflow = 0,
    ///1: 32-bit programmable counter overflow occurred
    Overflow = 1,
}
impl From<OWFR_A> for bool {
    #[inline(always)]
    fn from(variant: OWFR_A) -> Self {
        variant as u8 != 0
    }
}
impl OWF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OWFR_A {
        match self.bits {
            false => OWFR_A::NoOverflow,
            true => OWFR_A::Overflow,
        }
    }
    ///Checks if the value of the field is `NoOverflow`
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == OWFR_A::NoOverflow
    }
    ///Checks if the value of the field is `Overflow`
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == OWFR_A::Overflow
    }
}
///Overflow Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OWFW_AW {
    ///0: Clear flag
    Clear = 0,
}
impl From<OWFW_AW> for bool {
    #[inline(always)]
    fn from(variant: OWFW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `OWF` writer - Overflow Flag
pub type OWF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, CRL_SPEC, OWFW_AW, O>;
impl<'a, const O: u8> OWF_W<'a, O> {
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OWFW_AW::Clear)
    }
}
///Field `RSF` reader - Registers Synchronized Flag
pub type RSF_R = crate::BitReader<RSFR_A>;
///Registers Synchronized Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSFR_A {
    ///0: Registers not yet synchronized
    NotSynchronized = 0,
    ///1: Registers synchronized
    Synchronized = 1,
}
impl From<RSFR_A> for bool {
    #[inline(always)]
    fn from(variant: RSFR_A) -> Self {
        variant as u8 != 0
    }
}
impl RSF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RSFR_A {
        match self.bits {
            false => RSFR_A::NotSynchronized,
            true => RSFR_A::Synchronized,
        }
    }
    ///Checks if the value of the field is `NotSynchronized`
    #[inline(always)]
    pub fn is_not_synchronized(&self) -> bool {
        *self == RSFR_A::NotSynchronized
    }
    ///Checks if the value of the field is `Synchronized`
    #[inline(always)]
    pub fn is_synchronized(&self) -> bool {
        *self == RSFR_A::Synchronized
    }
}
///Registers Synchronized Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSFW_AW {
    ///0: Clear flag
    Clear = 0,
}
impl From<RSFW_AW> for bool {
    #[inline(always)]
    fn from(variant: RSFW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `RSF` writer - Registers Synchronized Flag
pub type RSF_W<'a, const O: u8> = crate::BitWriter0C<'a, u32, CRL_SPEC, RSFW_AW, O>;
impl<'a, const O: u8> RSF_W<'a, O> {
    ///Clear flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RSFW_AW::Clear)
    }
}
///Field `CNF` reader - Configuration Flag
pub type CNF_R = crate::BitReader<CNF_A>;
///Configuration Flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNF_A {
    ///0: Exit configuration mode (start update of RTC registers)
    Exit = 0,
    ///1: Enter configuration mode
    Enter = 1,
}
impl From<CNF_A> for bool {
    #[inline(always)]
    fn from(variant: CNF_A) -> Self {
        variant as u8 != 0
    }
}
impl CNF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CNF_A {
        match self.bits {
            false => CNF_A::Exit,
            true => CNF_A::Enter,
        }
    }
    ///Checks if the value of the field is `Exit`
    #[inline(always)]
    pub fn is_exit(&self) -> bool {
        *self == CNF_A::Exit
    }
    ///Checks if the value of the field is `Enter`
    #[inline(always)]
    pub fn is_enter(&self) -> bool {
        *self == CNF_A::Enter
    }
}
///Field `CNF` writer - Configuration Flag
pub type CNF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRL_SPEC, CNF_A, O>;
impl<'a, const O: u8> CNF_W<'a, O> {
    ///Exit configuration mode (start update of RTC registers)
    #[inline(always)]
    pub fn exit(self) -> &'a mut W {
        self.variant(CNF_A::Exit)
    }
    ///Enter configuration mode
    #[inline(always)]
    pub fn enter(self) -> &'a mut W {
        self.variant(CNF_A::Enter)
    }
}
///Field `RTOFF` reader - RTC operation OFF
pub type RTOFF_R = crate::BitReader<RTOFF_A>;
///RTC operation OFF
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTOFF_A {
    ///0: Last write operation on RTC registers is still ongoing
    Enabled = 0,
    ///1: Last write operation on RTC registers terminated
    Disabled = 1,
}
impl From<RTOFF_A> for bool {
    #[inline(always)]
    fn from(variant: RTOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl RTOFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RTOFF_A {
        match self.bits {
            false => RTOFF_A::Enabled,
            true => RTOFF_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTOFF_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTOFF_A::Disabled
    }
}
impl R {
    ///Bit 0 - Second Flag
    #[inline(always)]
    pub fn secf(&self) -> SECF_R {
        SECF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Alarm Flag
    #[inline(always)]
    pub fn alrf(&self) -> ALRF_R {
        ALRF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Overflow Flag
    #[inline(always)]
    pub fn owf(&self) -> OWF_R {
        OWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Registers Synchronized Flag
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Configuration Flag
    #[inline(always)]
    pub fn cnf(&self) -> CNF_R {
        CNF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RTC operation OFF
    #[inline(always)]
    pub fn rtoff(&self) -> RTOFF_R {
        RTOFF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Second Flag
    #[inline(always)]
    #[must_use]
    pub fn secf(&mut self) -> SECF_W<0> {
        SECF_W::new(self)
    }
    ///Bit 1 - Alarm Flag
    #[inline(always)]
    #[must_use]
    pub fn alrf(&mut self) -> ALRF_W<1> {
        ALRF_W::new(self)
    }
    ///Bit 2 - Overflow Flag
    #[inline(always)]
    #[must_use]
    pub fn owf(&mut self) -> OWF_W<2> {
        OWF_W::new(self)
    }
    ///Bit 3 - Registers Synchronized Flag
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<3> {
        RSF_W::new(self)
    }
    ///Bit 4 - Configuration Flag
    #[inline(always)]
    #[must_use]
    pub fn cnf(&mut self) -> CNF_W<4> {
        CNF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RTC Control Register Low
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [crl](index.html) module
pub struct CRL_SPEC;
impl crate::RegisterSpec for CRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [crl::R](R) reader structure
impl crate::Readable for CRL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [crl::W](W) writer structure
impl crate::Writable for CRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0f;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CRL to value 0x20
impl crate::Resettable for CRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
