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
///Field `EOP` reader - End of operation
pub type EOP_R = crate::BitReader<EOPW_A>;
///End of operation
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOPW_A {
    ///1: Clear error flag
    Clear = 1,
}
impl From<EOPW_A> for bool {
    #[inline(always)]
    fn from(variant: EOPW_A) -> Self {
        variant as u8 != 0
    }
}
impl EOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<EOPW_A> {
        match self.bits {
            true => Some(EOPW_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == EOPW_A::Clear
    }
}
///Field `EOP` writer - End of operation
pub type EOP_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SR_SPEC, EOPW_A, O>;
impl<'a, const O: u8> EOP_W<'a, O> {
    ///Clear error flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOPW_A::Clear)
    }
}
///Field `OPERR` reader - Operation error
pub type OPERR_R = crate::BitReader<OPERRW_A>;
///Operation error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPERRW_A {
    ///1: Clear error flag
    Clear = 1,
}
impl From<OPERRW_A> for bool {
    #[inline(always)]
    fn from(variant: OPERRW_A) -> Self {
        variant as u8 != 0
    }
}
impl OPERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<OPERRW_A> {
        match self.bits {
            true => Some(OPERRW_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == OPERRW_A::Clear
    }
}
///Field `OPERR` writer - Operation error
pub type OPERR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SR_SPEC, OPERRW_A, O>;
impl<'a, const O: u8> OPERR_W<'a, O> {
    ///Clear error flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OPERRW_A::Clear)
    }
}
///Field `WRPERR` reader - Write protection error
pub type WRPERR_R = crate::BitReader<WRPERRW_A>;
///Write protection error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRPERRW_A {
    ///1: Clear error flag
    Clear = 1,
}
impl From<WRPERRW_A> for bool {
    #[inline(always)]
    fn from(variant: WRPERRW_A) -> Self {
        variant as u8 != 0
    }
}
impl WRPERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<WRPERRW_A> {
        match self.bits {
            true => Some(WRPERRW_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == WRPERRW_A::Clear
    }
}
///Field `WRPERR` writer - Write protection error
pub type WRPERR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SR_SPEC, WRPERRW_A, O>;
impl<'a, const O: u8> WRPERR_W<'a, O> {
    ///Clear error flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(WRPERRW_A::Clear)
    }
}
///Field `PGAERR` reader - Programming alignment error
pub type PGAERR_R = crate::BitReader<PGAERRW_A>;
///Programming alignment error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGAERRW_A {
    ///1: Clear error flag
    Clear = 1,
}
impl From<PGAERRW_A> for bool {
    #[inline(always)]
    fn from(variant: PGAERRW_A) -> Self {
        variant as u8 != 0
    }
}
impl PGAERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PGAERRW_A> {
        match self.bits {
            true => Some(PGAERRW_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == PGAERRW_A::Clear
    }
}
///Field `PGAERR` writer - Programming alignment error
pub type PGAERR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SR_SPEC, PGAERRW_A, O>;
impl<'a, const O: u8> PGAERR_W<'a, O> {
    ///Clear error flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PGAERRW_A::Clear)
    }
}
///Field `PGPERR` reader - Programming parallelism error
pub type PGPERR_R = crate::BitReader<PGPERRW_A>;
///Programming parallelism error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGPERRW_A {
    ///1: Clear error flag
    Clear = 1,
}
impl From<PGPERRW_A> for bool {
    #[inline(always)]
    fn from(variant: PGPERRW_A) -> Self {
        variant as u8 != 0
    }
}
impl PGPERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PGPERRW_A> {
        match self.bits {
            true => Some(PGPERRW_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == PGPERRW_A::Clear
    }
}
///Field `PGPERR` writer - Programming parallelism error
pub type PGPERR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SR_SPEC, PGPERRW_A, O>;
impl<'a, const O: u8> PGPERR_W<'a, O> {
    ///Clear error flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PGPERRW_A::Clear)
    }
}
///Field `PGSERR` reader - Programming sequence error
pub type PGSERR_R = crate::BitReader<PGSERRW_A>;
///Programming sequence error
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PGSERRW_A {
    ///1: Clear error flag
    Clear = 1,
}
impl From<PGSERRW_A> for bool {
    #[inline(always)]
    fn from(variant: PGSERRW_A) -> Self {
        variant as u8 != 0
    }
}
impl PGSERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PGSERRW_A> {
        match self.bits {
            true => Some(PGSERRW_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == PGSERRW_A::Clear
    }
}
///Field `PGSERR` writer - Programming sequence error
pub type PGSERR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SR_SPEC, PGSERRW_A, O>;
impl<'a, const O: u8> PGSERR_W<'a, O> {
    ///Clear error flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PGSERRW_A::Clear)
    }
}
///Field `RDERR` reader - Proprietary readout protection (PCROP) error
pub type RDERR_R = crate::BitReader<bool>;
///Field `RDERR` writer - Proprietary readout protection (PCROP) error
pub type RDERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `BSY` reader - Busy
pub type BSY_R = crate::BitReader<BSYR_A>;
///Busy
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSYR_A {
    ///0: no Flash memory operation ongoing
    NotBusy = 0,
    ///1: Flash memory operation ongoing
    Busy = 1,
}
impl From<BSYR_A> for bool {
    #[inline(always)]
    fn from(variant: BSYR_A) -> Self {
        variant as u8 != 0
    }
}
impl BSY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BSYR_A {
        match self.bits {
            false => BSYR_A::NotBusy,
            true => BSYR_A::Busy,
        }
    }
    ///Checks if the value of the field is `NotBusy`
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BSYR_A::NotBusy
    }
    ///Checks if the value of the field is `Busy`
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BSYR_A::Busy
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
    ///Bit 4 - Write protection error
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Programming alignment error
    #[inline(always)]
    pub fn pgaerr(&self) -> PGAERR_R {
        PGAERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Programming parallelism error
    #[inline(always)]
    pub fn pgperr(&self) -> PGPERR_R {
        PGPERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Programming sequence error
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Proprietary readout protection (PCROP) error
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - Busy
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 16) & 1) != 0)
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
    ///Bit 4 - Write protection error
    #[inline(always)]
    #[must_use]
    pub fn wrperr(&mut self) -> WRPERR_W<4> {
        WRPERR_W::new(self)
    }
    ///Bit 5 - Programming alignment error
    #[inline(always)]
    #[must_use]
    pub fn pgaerr(&mut self) -> PGAERR_W<5> {
        PGAERR_W::new(self)
    }
    ///Bit 6 - Programming parallelism error
    #[inline(always)]
    #[must_use]
    pub fn pgperr(&mut self) -> PGPERR_W<6> {
        PGPERR_W::new(self)
    }
    ///Bit 7 - Programming sequence error
    #[inline(always)]
    #[must_use]
    pub fn pgserr(&mut self) -> PGSERR_W<7> {
        PGSERR_W::new(self)
    }
    ///Bit 8 - Proprietary readout protection (PCROP) error
    #[inline(always)]
    #[must_use]
    pub fn rderr(&mut self) -> RDERR_W<8> {
        RDERR_W::new(self)
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xf3;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
