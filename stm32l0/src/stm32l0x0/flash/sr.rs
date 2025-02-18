///Register `SR` reader
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SR` writer
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BSY` reader - Write/erase operations in progress
pub type BSY_R = crate::BitReader<BSY_A>;
///Write/erase operations in progress
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSY_A {
    ///0: No write/erase operation is in progress
    Inactive = 0,
    ///1: No write/erase operation is in progress
    Active = 1,
}
impl From<BSY_A> for bool {
    #[inline(always)]
    fn from(variant: BSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BSY_A {
        match self.bits {
            false => BSY_A::Inactive,
            true => BSY_A::Active,
        }
    }
    ///Checks if the value of the field is `Inactive`
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == BSY_A::Inactive
    }
    ///Checks if the value of the field is `Active`
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BSY_A::Active
    }
}
///Field `EOP` reader - End of operation
pub type EOP_R = crate::BitReader<EOP_A>;
///End of operation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOP_A {
    ///0: No EOP operation occurred
    NoEvent = 0,
    ///1: An EOP event occurred
    Event = 1,
}
impl From<EOP_A> for bool {
    #[inline(always)]
    fn from(variant: EOP_A) -> Self {
        variant as u8 != 0
    }
}
impl EOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOP_A {
        match self.bits {
            false => EOP_A::NoEvent,
            true => EOP_A::Event,
        }
    }
    ///Checks if the value of the field is `NoEvent`
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == EOP_A::NoEvent
    }
    ///Checks if the value of the field is `Event`
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == EOP_A::Event
    }
}
///Field `ENDHV` reader - End of high voltage
pub type ENDHV_R = crate::BitReader<ENDHV_A>;
///End of high voltage
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDHV_A {
    ///0: High voltage is executing a write/erase operation in the NVM
    Active = 0,
    ///1: High voltage is off, no write/erase operation is ongoing
    Inactive = 1,
}
impl From<ENDHV_A> for bool {
    #[inline(always)]
    fn from(variant: ENDHV_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDHV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ENDHV_A {
        match self.bits {
            false => ENDHV_A::Active,
            true => ENDHV_A::Inactive,
        }
    }
    ///Checks if the value of the field is `Active`
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ENDHV_A::Active
    }
    ///Checks if the value of the field is `Inactive`
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ENDHV_A::Inactive
    }
}
///Field `READY` reader - Flash memory module ready after low power mode
pub type READY_R = crate::BitReader<READY_A>;
///Flash memory module ready after low power mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READY_A {
    ///0: The NVM is not ready
    NotReady = 0,
    ///1: The NVM is ready
    Ready = 1,
}
impl From<READY_A> for bool {
    #[inline(always)]
    fn from(variant: READY_A) -> Self {
        variant as u8 != 0
    }
}
impl READY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> READY_A {
        match self.bits {
            false => READY_A::NotReady,
            true => READY_A::Ready,
        }
    }
    ///Checks if the value of the field is `NotReady`
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == READY_A::NotReady
    }
    ///Checks if the value of the field is `Ready`
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == READY_A::Ready
    }
}
///Field `WRPERR` reader - Write protected error
pub type WRPERR_R = crate::BitReader<WRPERRR_A>;
///Write protected error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPERRR_A {
    ///0: No protection error happened
    NoError = 0,
    ///1: One protection error happened
    Error = 1,
}
impl From<WRPERRR_A> for bool {
    #[inline(always)]
    fn from(variant: WRPERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl WRPERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WRPERRR_A {
        match self.bits {
            false => WRPERRR_A::NoError,
            true => WRPERRR_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == WRPERRR_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == WRPERRR_A::Error
    }
}
///Write protected error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPERRW_AW {
    ///1: Clear the flag
    Clear = 1,
}
impl From<WRPERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: WRPERRW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `WRPERR` writer - Write protected error
pub type WRPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, WRPERRW_AW, O>;
impl<'a, const O: u8> WRPERR_W<'a, O> {
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WRPERRW_AW::Clear)
    }
}
///Field `PGAERR` reader - Programming alignment error
pub type PGAERR_R = crate::BitReader<PGAERRR_A>;
///Programming alignment error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGAERRR_A {
    ///0: No alignment error happened
    NoError = 0,
    ///1: One alignment error happened
    Error = 1,
}
impl From<PGAERRR_A> for bool {
    #[inline(always)]
    fn from(variant: PGAERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl PGAERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PGAERRR_A {
        match self.bits {
            false => PGAERRR_A::NoError,
            true => PGAERRR_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PGAERRR_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PGAERRR_A::Error
    }
}
///Programming alignment error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGAERRW_AW {
    ///1: Clear the flag
    Clear = 1,
}
impl From<PGAERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: PGAERRW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `PGAERR` writer - Programming alignment error
pub type PGAERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, PGAERRW_AW, O>;
impl<'a, const O: u8> PGAERR_W<'a, O> {
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PGAERRW_AW::Clear)
    }
}
///Field `SIZERR` reader - Size error
pub type SIZERR_R = crate::BitReader<SIZERRR_A>;
///Size error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIZERRR_A {
    ///0: No size error happened
    NoError = 0,
    ///1: One size error happened
    Error = 1,
}
impl From<SIZERRR_A> for bool {
    #[inline(always)]
    fn from(variant: SIZERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl SIZERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SIZERRR_A {
        match self.bits {
            false => SIZERRR_A::NoError,
            true => SIZERRR_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == SIZERRR_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == SIZERRR_A::Error
    }
}
///Size error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIZERRW_AW {
    ///1: Clear the flag
    Clear = 1,
}
impl From<SIZERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: SIZERRW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `SIZERR` writer - Size error
pub type SIZERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, SIZERRW_AW, O>;
impl<'a, const O: u8> SIZERR_W<'a, O> {
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SIZERRW_AW::Clear)
    }
}
///Field `OPTVERR` reader - Option validity error
pub type OPTVERR_R = crate::BitReader<OPTVERRR_A>;
///Option validity error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTVERRR_A {
    ///0: No error happened during the Option bytes loading
    NoError = 0,
    ///1: One or more errors happened during the Option bytes loading
    Error = 1,
}
impl From<OPTVERRR_A> for bool {
    #[inline(always)]
    fn from(variant: OPTVERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl OPTVERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OPTVERRR_A {
        match self.bits {
            false => OPTVERRR_A::NoError,
            true => OPTVERRR_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == OPTVERRR_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == OPTVERRR_A::Error
    }
}
///Option validity error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTVERRW_AW {
    ///1: Clear the flag
    Clear = 1,
}
impl From<OPTVERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: OPTVERRW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `OPTVERR` writer - Option validity error
pub type OPTVERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, OPTVERRW_AW, O>;
impl<'a, const O: u8> OPTVERR_W<'a, O> {
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OPTVERRW_AW::Clear)
    }
}
///Field `RDERR` reader - RDERR
pub type RDERR_R = crate::BitReader<RDERRR_A>;
///RDERR
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDERRR_A {
    ///0: No read protection error happened.
    NoError = 0,
    ///1: One read protection error happened
    Error = 1,
}
impl From<RDERRR_A> for bool {
    #[inline(always)]
    fn from(variant: RDERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl RDERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RDERRR_A {
        match self.bits {
            false => RDERRR_A::NoError,
            true => RDERRR_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == RDERRR_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == RDERRR_A::Error
    }
}
///RDERR
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDERRW_AW {
    ///1: Clear the flag
    Clear = 1,
}
impl From<RDERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: RDERRW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `RDERR` writer - RDERR
pub type RDERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, RDERRW_AW, O>;
impl<'a, const O: u8> RDERR_W<'a, O> {
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RDERRW_AW::Clear)
    }
}
///Field `NOTZEROERR` reader - NOTZEROERR
pub type NOTZEROERR_R = crate::BitReader<NOTZEROERRR_A>;
///NOTZEROERR
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTZEROERRR_A {
    ///0: The write operation is done in an erased region or the memory interface can apply an erase before a write
    NoEvent = 0,
    ///1: The write operation is attempting to write to a not-erased region and the memory interface cannot apply an erase before a write
    Event = 1,
}
impl From<NOTZEROERRR_A> for bool {
    #[inline(always)]
    fn from(variant: NOTZEROERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl NOTZEROERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NOTZEROERRR_A {
        match self.bits {
            false => NOTZEROERRR_A::NoEvent,
            true => NOTZEROERRR_A::Event,
        }
    }
    ///Checks if the value of the field is `NoEvent`
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == NOTZEROERRR_A::NoEvent
    }
    ///Checks if the value of the field is `Event`
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == NOTZEROERRR_A::Event
    }
}
///NOTZEROERR
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTZEROERRW_AW {
    ///1: Clear the flag
    Clear = 1,
}
impl From<NOTZEROERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTZEROERRW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `NOTZEROERR` writer - NOTZEROERR
pub type NOTZEROERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, NOTZEROERRW_AW, O>;
impl<'a, const O: u8> NOTZEROERR_W<'a, O> {
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(NOTZEROERRW_AW::Clear)
    }
}
///Field `FWWERR` reader - FWWERR
pub type FWWERR_R = crate::BitReader<FWWERRR_A>;
///FWWERR
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWWERRR_A {
    ///0: No write/erase operation aborted to perform a fetch
    NoError = 0,
    ///1: A write/erase operation aborted to perform a fetch
    Error = 1,
}
impl From<FWWERRR_A> for bool {
    #[inline(always)]
    fn from(variant: FWWERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl FWWERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FWWERRR_A {
        match self.bits {
            false => FWWERRR_A::NoError,
            true => FWWERRR_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == FWWERRR_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FWWERRR_A::Error
    }
}
///FWWERR
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWWERRW_AW {
    ///1: Clear the flag
    Clear = 1,
}
impl From<FWWERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: FWWERRW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `FWWERR` writer - FWWERR
pub type FWWERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, FWWERRW_AW, O>;
impl<'a, const O: u8> FWWERR_W<'a, O> {
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FWWERRW_AW::Clear)
    }
}
impl R {
    ///Bit 0 - Write/erase operations in progress
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of operation
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - End of high voltage
    #[inline(always)]
    pub fn endhv(&self) -> ENDHV_R {
        ENDHV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Flash memory module ready after low power mode
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 8 - Write protected error
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Programming alignment error
    #[inline(always)]
    pub fn pgaerr(&self) -> PGAERR_R {
        PGAERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Size error
    #[inline(always)]
    pub fn sizerr(&self) -> SIZERR_R {
        SIZERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Option validity error
    #[inline(always)]
    pub fn optverr(&self) -> OPTVERR_R {
        OPTVERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 13 - RDERR
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - NOTZEROERR
    #[inline(always)]
    pub fn notzeroerr(&self) -> NOTZEROERR_R {
        NOTZEROERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - FWWERR
    #[inline(always)]
    pub fn fwwerr(&self) -> FWWERR_R {
        FWWERR_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    ///Bit 8 - Write protected error
    #[inline(always)]
    #[must_use]
    pub fn wrperr(&mut self) -> WRPERR_W<8> {
        WRPERR_W::new(self)
    }
    ///Bit 9 - Programming alignment error
    #[inline(always)]
    #[must_use]
    pub fn pgaerr(&mut self) -> PGAERR_W<9> {
        PGAERR_W::new(self)
    }
    ///Bit 10 - Size error
    #[inline(always)]
    #[must_use]
    pub fn sizerr(&mut self) -> SIZERR_W<10> {
        SIZERR_W::new(self)
    }
    ///Bit 11 - Option validity error
    #[inline(always)]
    #[must_use]
    pub fn optverr(&mut self) -> OPTVERR_W<11> {
        OPTVERR_W::new(self)
    }
    ///Bit 13 - RDERR
    #[inline(always)]
    #[must_use]
    pub fn rderr(&mut self) -> RDERR_W<13> {
        RDERR_W::new(self)
    }
    ///Bit 16 - NOTZEROERR
    #[inline(always)]
    #[must_use]
    pub fn notzeroerr(&mut self) -> NOTZEROERR_W<16> {
        NOTZEROERR_W::new(self)
    }
    ///Bit 17 - FWWERR
    #[inline(always)]
    #[must_use]
    pub fn fwwerr(&mut self) -> FWWERR_W<17> {
        FWWERR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](index.html) module
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr::R](R) reader structure
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sr::W](W) writer structure
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SR to value 0x04
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
