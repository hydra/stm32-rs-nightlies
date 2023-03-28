///Register `C2SR` reader
pub struct R(crate::R<C2SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2SR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C2SR` writer
pub struct W(crate::W<C2SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2SR_SPEC>;
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
impl From<crate::W<C2SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2SR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EOP` reader - End of operation
pub type EOP_R = crate::BitReader<EOPR_A>;
///End of operation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPR_A {
    ///0: No EOP operation occurred
    NoEvent = 0,
    ///1: An EOP event occurred
    Event = 1,
}
impl From<EOPR_A> for bool {
    #[inline(always)]
    fn from(variant: EOPR_A) -> Self {
        variant as u8 != 0
    }
}
impl EOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOPR_A {
        match self.bits {
            false => EOPR_A::NoEvent,
            true => EOPR_A::Event,
        }
    }
    ///Checks if the value of the field is `NoEvent`
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == EOPR_A::NoEvent
    }
    ///Checks if the value of the field is `Event`
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == EOPR_A::Event
    }
}
///End of operation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPW_AW {
    ///1: Clear the flag
    Clear = 1,
}
impl From<EOPW_AW> for bool {
    #[inline(always)]
    fn from(variant: EOPW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOP` writer - End of operation
pub type EOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2SR_SPEC, EOPW_AW, O>;
impl<'a, const O: u8> EOP_W<'a, O> {
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOPW_AW::Clear)
    }
}
///Field `OPERR` reader - Operation error
pub type OPERR_R = crate::BitReader<OPERRR_A>;
///Operation error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPERRR_A {
    ///0: No memory opreation error happened
    NoError = 0,
    ///1: Memory operation error happened
    Error = 1,
}
impl From<OPERRR_A> for bool {
    #[inline(always)]
    fn from(variant: OPERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl OPERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OPERRR_A {
        match self.bits {
            false => OPERRR_A::NoError,
            true => OPERRR_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == OPERRR_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == OPERRR_A::Error
    }
}
///Operation error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPERRW_AW {
    ///1: Clear the flag
    Clear = 1,
}
impl From<OPERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: OPERRW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `OPERR` writer - Operation error
pub type OPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2SR_SPEC, OPERRW_AW, O>;
impl<'a, const O: u8> OPERR_W<'a, O> {
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OPERRW_AW::Clear)
    }
}
///Field `PROGERR` reader - Programming error
pub type PROGERR_R = crate::BitReader<PROGERRR_A>;
///Programming error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROGERRR_A {
    ///0: No size programming error happened
    NoError = 0,
    ///1: Programming error happened
    Error = 1,
}
impl From<PROGERRR_A> for bool {
    #[inline(always)]
    fn from(variant: PROGERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl PROGERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PROGERRR_A {
        match self.bits {
            false => PROGERRR_A::NoError,
            true => PROGERRR_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PROGERRR_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PROGERRR_A::Error
    }
}
///Programming error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROGERRW_AW {
    ///1: Clear the flag
    Clear = 1,
}
impl From<PROGERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: PROGERRW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `PROGERR` writer - Programming error
pub type PROGERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2SR_SPEC, PROGERRW_AW, O>;
impl<'a, const O: u8> PROGERR_W<'a, O> {
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PROGERRW_AW::Clear)
    }
}
///Field `WRPERR` reader - WRPERR
pub type WRPERR_R = crate::BitReader<WRPERRR_A>;
///WRPERR
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPERRR_A {
    ///0: No write protection error happened
    NoError = 0,
    ///1: Write protection error happened
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
///WRPERR
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
///Field `WRPERR` writer - WRPERR
pub type WRPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2SR_SPEC, WRPERRW_AW, O>;
impl<'a, const O: u8> WRPERR_W<'a, O> {
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WRPERRW_AW::Clear)
    }
}
///Field `PGAERR` reader - PGAERR
pub type PGAERR_R = crate::BitReader<PGAERRR_A>;
///PGAERR
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGAERRR_A {
    ///0: No programming alignment error happened
    NoError = 0,
    ///1: Programming alignment error happened
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
///PGAERR
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
///Field `PGAERR` writer - PGAERR
pub type PGAERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2SR_SPEC, PGAERRW_AW, O>;
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
    ///1: Size error happened
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
pub type SIZERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2SR_SPEC, SIZERRW_AW, O>;
impl<'a, const O: u8> SIZERR_W<'a, O> {
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SIZERRW_AW::Clear)
    }
}
///Field `PGSERR` reader - Programming sequence error
pub type PGSERR_R = crate::BitReader<PGSERRR_A>;
///Programming sequence error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGSERRR_A {
    ///0: No fast programming sequence error happened
    NoError = 0,
    ///1: Fast programming sequence error happened
    Error = 1,
}
impl From<PGSERRR_A> for bool {
    #[inline(always)]
    fn from(variant: PGSERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl PGSERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PGSERRR_A {
        match self.bits {
            false => PGSERRR_A::NoError,
            true => PGSERRR_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == PGSERRR_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == PGSERRR_A::Error
    }
}
///Programming sequence error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGSERRW_AW {
    ///1: Clear the flag
    Clear = 1,
}
impl From<PGSERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: PGSERRW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `PGSERR` writer - Programming sequence error
pub type PGSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2SR_SPEC, PGSERRW_AW, O>;
impl<'a, const O: u8> PGSERR_W<'a, O> {
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PGSERRW_AW::Clear)
    }
}
///Field `MISSERR` reader - Fast programming data miss error
pub type MISSERR_R = crate::BitReader<MISSERRR_A>;
///Fast programming data miss error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MISSERRR_A {
    ///0: No fast programming data miss error happened
    NoError = 0,
    ///1: Fast programming data miss error happened
    Error = 1,
}
impl From<MISSERRR_A> for bool {
    #[inline(always)]
    fn from(variant: MISSERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl MISSERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MISSERRR_A {
        match self.bits {
            false => MISSERRR_A::NoError,
            true => MISSERRR_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == MISSERRR_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == MISSERRR_A::Error
    }
}
///Fast programming data miss error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MISSERRW_AW {
    ///1: Clear the flag
    Clear = 1,
}
impl From<MISSERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: MISSERRW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `MISSERR` writer - Fast programming data miss error
pub type MISSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2SR_SPEC, MISSERRW_AW, O>;
impl<'a, const O: u8> MISSERR_W<'a, O> {
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(MISSERRW_AW::Clear)
    }
}
///Field `FASTERR` reader - Fast programming error
pub type FASTERR_R = crate::BitReader<FASTERRR_A>;
///Fast programming error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FASTERRR_A {
    ///0: No fast programming error happened
    NoError = 0,
    ///1: Fast programming error happened
    Error = 1,
}
impl From<FASTERRR_A> for bool {
    #[inline(always)]
    fn from(variant: FASTERRR_A) -> Self {
        variant as u8 != 0
    }
}
impl FASTERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FASTERRR_A {
        match self.bits {
            false => FASTERRR_A::NoError,
            true => FASTERRR_A::Error,
        }
    }
    ///Checks if the value of the field is `NoError`
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == FASTERRR_A::NoError
    }
    ///Checks if the value of the field is `Error`
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FASTERRR_A::Error
    }
}
///Fast programming error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FASTERRW_AW {
    ///1: Clear the flag
    Clear = 1,
}
impl From<FASTERRW_AW> for bool {
    #[inline(always)]
    fn from(variant: FASTERRW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `FASTERR` writer - Fast programming error
pub type FASTERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2SR_SPEC, FASTERRW_AW, O>;
impl<'a, const O: u8> FASTERR_W<'a, O> {
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FASTERRW_AW::Clear)
    }
}
///Field `RDERR` reader - PCROP read error
pub type RDERR_R = crate::BitReader<RDERRR_A>;
///PCROP read error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDERRR_A {
    ///0: No read-only error happened
    NoError = 0,
    ///1: Read-only error happened
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
///PCROP read error
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
///Field `RDERR` writer - PCROP read error
pub type RDERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2SR_SPEC, RDERRW_AW, O>;
impl<'a, const O: u8> RDERR_W<'a, O> {
    ///Clear the flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RDERRW_AW::Clear)
    }
}
///Field `BSY` reader - BSY
pub type BSY_R = crate::BitReader<BSY_A>;
///BSY
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
///Field `CFGBSY` reader - CFGBSY
pub type CFGBSY_R = crate::BitReader<CFGBSY_A>;
///CFGBSY
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFGBSY_A {
    ///0: PG, PNB, PER, MER bits available for writing
    Free = 0,
    ///1: PG, PNB, PER, MER bits not available for writing (operation ongoing)
    Busy = 1,
}
impl From<CFGBSY_A> for bool {
    #[inline(always)]
    fn from(variant: CFGBSY_A) -> Self {
        variant as u8 != 0
    }
}
impl CFGBSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CFGBSY_A {
        match self.bits {
            false => CFGBSY_A::Free,
            true => CFGBSY_A::Busy,
        }
    }
    ///Checks if the value of the field is `Free`
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == CFGBSY_A::Free
    }
    ///Checks if the value of the field is `Busy`
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == CFGBSY_A::Busy
    }
}
///Field `PESD` reader - PESD
pub type PESD_R = crate::BitReader<PESD_A>;
///PESD
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PESD_A {
    ///0: Flash program and erase operations granted
    Granted = 0,
    ///1: Any new Flash program and erase operation is suspended until this bit is cleared. This bit is set when at least one PES bit in FLASH_ACR or FLASH_C2ACR is set.
    Suspended = 1,
}
impl From<PESD_A> for bool {
    #[inline(always)]
    fn from(variant: PESD_A) -> Self {
        variant as u8 != 0
    }
}
impl PESD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PESD_A {
        match self.bits {
            false => PESD_A::Granted,
            true => PESD_A::Suspended,
        }
    }
    ///Checks if the value of the field is `Granted`
    #[inline(always)]
    pub fn is_granted(&self) -> bool {
        *self == PESD_A::Granted
    }
    ///Checks if the value of the field is `Suspended`
    #[inline(always)]
    pub fn is_suspended(&self) -> bool {
        *self == PESD_A::Suspended
    }
}
impl R {
    ///Bit 0 - End of operation
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Operation error
    #[inline(always)]
    pub fn operr(&self) -> OPERR_R {
        OPERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Programming error
    #[inline(always)]
    pub fn progerr(&self) -> PROGERR_R {
        PROGERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - WRPERR
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PGAERR
    #[inline(always)]
    pub fn pgaerr(&self) -> PGAERR_R {
        PGAERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Size error
    #[inline(always)]
    pub fn sizerr(&self) -> SIZERR_R {
        SIZERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Programming sequence error
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Fast programming data miss error
    #[inline(always)]
    pub fn misserr(&self) -> MISSERR_R {
        MISSERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Fast programming error
    #[inline(always)]
    pub fn fasterr(&self) -> FASTERR_R {
        FASTERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 14 - PCROP read error
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - BSY
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - CFGBSY
    #[inline(always)]
    pub fn cfgbsy(&self) -> CFGBSY_R {
        CFGBSY_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - PESD
    #[inline(always)]
    pub fn pesd(&self) -> PESD_R {
        PESD_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - End of operation
    #[inline(always)]
    #[must_use]
    pub fn eop(&mut self) -> EOP_W<0> {
        EOP_W::new(self)
    }
    ///Bit 1 - Operation error
    #[inline(always)]
    #[must_use]
    pub fn operr(&mut self) -> OPERR_W<1> {
        OPERR_W::new(self)
    }
    ///Bit 3 - Programming error
    #[inline(always)]
    #[must_use]
    pub fn progerr(&mut self) -> PROGERR_W<3> {
        PROGERR_W::new(self)
    }
    ///Bit 4 - WRPERR
    #[inline(always)]
    #[must_use]
    pub fn wrperr(&mut self) -> WRPERR_W<4> {
        WRPERR_W::new(self)
    }
    ///Bit 5 - PGAERR
    #[inline(always)]
    #[must_use]
    pub fn pgaerr(&mut self) -> PGAERR_W<5> {
        PGAERR_W::new(self)
    }
    ///Bit 6 - Size error
    #[inline(always)]
    #[must_use]
    pub fn sizerr(&mut self) -> SIZERR_W<6> {
        SIZERR_W::new(self)
    }
    ///Bit 7 - Programming sequence error
    #[inline(always)]
    #[must_use]
    pub fn pgserr(&mut self) -> PGSERR_W<7> {
        PGSERR_W::new(self)
    }
    ///Bit 8 - Fast programming data miss error
    #[inline(always)]
    #[must_use]
    pub fn misserr(&mut self) -> MISSERR_W<8> {
        MISSERR_W::new(self)
    }
    ///Bit 9 - Fast programming error
    #[inline(always)]
    #[must_use]
    pub fn fasterr(&mut self) -> FASTERR_W<9> {
        FASTERR_W::new(self)
    }
    ///Bit 14 - PCROP read error
    #[inline(always)]
    #[must_use]
    pub fn rderr(&mut self) -> RDERR_W<14> {
        RDERR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flash CPU2 status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2sr](index.html) module
pub struct C2SR_SPEC;
impl crate::RegisterSpec for C2SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c2sr::R](R) reader structure
impl crate::Readable for C2SR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c2sr::W](W) writer structure
impl crate::Writable for C2SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C2SR to value 0
impl crate::Resettable for C2SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
