///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DBG_SLEEP` reader - Debug Sleep Mode
pub type DBG_SLEEP_R = crate::BitReader<DBG_SLEEP_A>;
///Debug Sleep Mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_SLEEP_A {
    ///0: Debug Sleep Mode Disabled
    Disabled = 0,
    ///1: Debug Sleep Mode Enabled
    Enabled = 1,
}
impl From<DBG_SLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_SLEEP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_SLEEP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_SLEEP_A {
        match self.bits {
            false => DBG_SLEEP_A::Disabled,
            true => DBG_SLEEP_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBG_SLEEP_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBG_SLEEP_A::Enabled
    }
}
///Field `DBG_SLEEP` writer - Debug Sleep Mode
pub type DBG_SLEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DBG_SLEEP_A, O>;
impl<'a, const O: u8> DBG_SLEEP_W<'a, O> {
    ///Debug Sleep Mode Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBG_SLEEP_A::Disabled)
    }
    ///Debug Sleep Mode Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBG_SLEEP_A::Enabled)
    }
}
///Field `DBG_STOP` reader - Debug Stop Mode
pub type DBG_STOP_R = crate::BitReader<DBG_STOP_A>;
///Debug Stop Mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_STOP_A {
    ///0: Debug Stop Mode Disabled
    Disabled = 0,
    ///1: Debug Stop Mode Enabled
    Enabled = 1,
}
impl From<DBG_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_STOP_A {
        match self.bits {
            false => DBG_STOP_A::Disabled,
            true => DBG_STOP_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBG_STOP_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBG_STOP_A::Enabled
    }
}
///Field `DBG_STOP` writer - Debug Stop Mode
pub type DBG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DBG_STOP_A, O>;
impl<'a, const O: u8> DBG_STOP_W<'a, O> {
    ///Debug Stop Mode Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBG_STOP_A::Disabled)
    }
    ///Debug Stop Mode Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBG_STOP_A::Enabled)
    }
}
///Field `DBG_STANDBY` reader - Debug Standby Mode
pub type DBG_STANDBY_R = crate::BitReader<DBG_STANDBY_A>;
///Debug Standby Mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_STANDBY_A {
    ///0: Debug Standby Mode Disabled
    Disabled = 0,
    ///1: Debug Standby Mode Enabled
    Enabled = 1,
}
impl From<DBG_STANDBY_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_STANDBY_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_STANDBY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DBG_STANDBY_A {
        match self.bits {
            false => DBG_STANDBY_A::Disabled,
            true => DBG_STANDBY_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBG_STANDBY_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBG_STANDBY_A::Enabled
    }
}
///Field `DBG_STANDBY` writer - Debug Standby Mode
pub type DBG_STANDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DBG_STANDBY_A, O>;
impl<'a, const O: u8> DBG_STANDBY_W<'a, O> {
    ///Debug Standby Mode Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBG_STANDBY_A::Disabled)
    }
    ///Debug Standby Mode Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBG_STANDBY_A::Enabled)
    }
}
///Field `TRACE_IOEN` reader - Trace pin assignment control
pub type TRACE_IOEN_R = crate::BitReader<TRACE_IOEN_A>;
///Trace pin assignment control
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRACE_IOEN_A {
    ///0: Trace pins not assigned (default state)
    Disabled = 0,
    ///1: Trace pins assigned
    Enabled = 1,
}
impl From<TRACE_IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: TRACE_IOEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TRACE_IOEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TRACE_IOEN_A {
        match self.bits {
            false => TRACE_IOEN_A::Disabled,
            true => TRACE_IOEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRACE_IOEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRACE_IOEN_A::Enabled
    }
}
///Field `TRACE_IOEN` writer - Trace pin assignment control
pub type TRACE_IOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, TRACE_IOEN_A, O>;
impl<'a, const O: u8> TRACE_IOEN_W<'a, O> {
    ///Trace pins not assigned (default state)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRACE_IOEN_A::Disabled)
    }
    ///Trace pins assigned
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRACE_IOEN_A::Enabled)
    }
}
///Field `TRACE_MODE` reader - Trace pin assignment control
pub type TRACE_MODE_R = crate::FieldReader<u8, TRACE_MODE_A>;
///Trace pin assignment control
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRACE_MODE_A {
    ///0: TRACE pin assignment for Asynchronous Mode
    Asynchronous = 0,
    ///1: TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 1
    Size1 = 1,
    ///2: TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 2
    Size2 = 2,
    ///3: TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 4
    Size4 = 3,
}
impl From<TRACE_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TRACE_MODE_A) -> Self {
        variant as _
    }
}
impl TRACE_MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TRACE_MODE_A {
        match self.bits {
            0 => TRACE_MODE_A::Asynchronous,
            1 => TRACE_MODE_A::Size1,
            2 => TRACE_MODE_A::Size2,
            3 => TRACE_MODE_A::Size4,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Asynchronous`
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == TRACE_MODE_A::Asynchronous
    }
    ///Checks if the value of the field is `Size1`
    #[inline(always)]
    pub fn is_size1(&self) -> bool {
        *self == TRACE_MODE_A::Size1
    }
    ///Checks if the value of the field is `Size2`
    #[inline(always)]
    pub fn is_size2(&self) -> bool {
        *self == TRACE_MODE_A::Size2
    }
    ///Checks if the value of the field is `Size4`
    #[inline(always)]
    pub fn is_size4(&self) -> bool {
        *self == TRACE_MODE_A::Size4
    }
}
///Field `TRACE_MODE` writer - Trace pin assignment control
pub type TRACE_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CR_SPEC, u8, TRACE_MODE_A, 2, O>;
impl<'a, const O: u8> TRACE_MODE_W<'a, O> {
    ///TRACE pin assignment for Asynchronous Mode
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(TRACE_MODE_A::Asynchronous)
    }
    ///TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 1
    #[inline(always)]
    pub fn size1(self) -> &'a mut W {
        self.variant(TRACE_MODE_A::Size1)
    }
    ///TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 2
    #[inline(always)]
    pub fn size2(self) -> &'a mut W {
        self.variant(TRACE_MODE_A::Size2)
    }
    ///TRACE pin assignment for Synchronous Mode with a TRACEDATA size of 4
    #[inline(always)]
    pub fn size4(self) -> &'a mut W {
        self.variant(TRACE_MODE_A::Size4)
    }
}
impl R {
    ///Bit 0 - Debug Sleep Mode
    #[inline(always)]
    pub fn dbg_sleep(&self) -> DBG_SLEEP_R {
        DBG_SLEEP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Debug Stop Mode
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Debug Standby Mode
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - Trace pin assignment control
    #[inline(always)]
    pub fn trace_ioen(&self) -> TRACE_IOEN_R {
        TRACE_IOEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - Trace pin assignment control
    #[inline(always)]
    pub fn trace_mode(&self) -> TRACE_MODE_R {
        TRACE_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - Debug Sleep Mode
    #[inline(always)]
    #[must_use]
    pub fn dbg_sleep(&mut self) -> DBG_SLEEP_W<0> {
        DBG_SLEEP_W::new(self)
    }
    ///Bit 1 - Debug Stop Mode
    #[inline(always)]
    #[must_use]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W<1> {
        DBG_STOP_W::new(self)
    }
    ///Bit 2 - Debug Standby Mode
    #[inline(always)]
    #[must_use]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W<2> {
        DBG_STANDBY_W::new(self)
    }
    ///Bit 5 - Trace pin assignment control
    #[inline(always)]
    #[must_use]
    pub fn trace_ioen(&mut self) -> TRACE_IOEN_W<5> {
        TRACE_IOEN_W::new(self)
    }
    ///Bits 6:7 - Trace pin assignment control
    #[inline(always)]
    #[must_use]
    pub fn trace_mode(&mut self) -> TRACE_MODE_W<6> {
        TRACE_MODE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Debug MCU Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
