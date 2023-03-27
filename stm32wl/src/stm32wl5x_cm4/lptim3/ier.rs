///Register `IER` reader
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IER` writer
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CMPMIE` reader - Compare match Interrupt Enable
pub type CMPMIE_R = crate::BitReader<CMPMIE_A>;
///Compare match Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPMIE_A {
    ///0: CMPM interrupt disabled
    Disabled = 0,
    ///1: CMPM interrupt enabled
    Enabled = 1,
}
impl From<CMPMIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPMIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPMIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CMPMIE_A {
        match self.bits {
            false => CMPMIE_A::Disabled,
            true => CMPMIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMPMIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMPMIE_A::Enabled
    }
}
///Field `CMPMIE` writer - Compare match Interrupt Enable
pub type CMPMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, CMPMIE_A, O>;
impl<'a, const O: u8> CMPMIE_W<'a, O> {
    ///CMPM interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMPMIE_A::Disabled)
    }
    ///CMPM interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMPMIE_A::Enabled)
    }
}
///Field `ARRMIE` reader - Autoreload match Interrupt Enable
pub type ARRMIE_R = crate::BitReader<ARRMIE_A>;
///Autoreload match Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARRMIE_A {
    ///0: ARRM interrupt disabled
    Disabled = 0,
    ///1: ARRM interrupt enabled
    Enabled = 1,
}
impl From<ARRMIE_A> for bool {
    #[inline(always)]
    fn from(variant: ARRMIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ARRMIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ARRMIE_A {
        match self.bits {
            false => ARRMIE_A::Disabled,
            true => ARRMIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARRMIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARRMIE_A::Enabled
    }
}
///Field `ARRMIE` writer - Autoreload match Interrupt Enable
pub type ARRMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, ARRMIE_A, O>;
impl<'a, const O: u8> ARRMIE_W<'a, O> {
    ///ARRM interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ARRMIE_A::Disabled)
    }
    ///ARRM interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ARRMIE_A::Enabled)
    }
}
///Field `EXTTRIGIE` reader - External trigger valid edge Interrupt Enable
pub type EXTTRIGIE_R = crate::BitReader<EXTTRIGIE_A>;
///External trigger valid edge Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTTRIGIE_A {
    ///0: EXTTRIG interrupt disabled
    Disabled = 0,
    ///1: EXTTRIG interrupt enabled
    Enabled = 1,
}
impl From<EXTTRIGIE_A> for bool {
    #[inline(always)]
    fn from(variant: EXTTRIGIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTTRIGIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EXTTRIGIE_A {
        match self.bits {
            false => EXTTRIGIE_A::Disabled,
            true => EXTTRIGIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTTRIGIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EXTTRIGIE_A::Enabled
    }
}
///Field `EXTTRIGIE` writer - External trigger valid edge Interrupt Enable
pub type EXTTRIGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, EXTTRIGIE_A, O>;
impl<'a, const O: u8> EXTTRIGIE_W<'a, O> {
    ///EXTTRIG interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTTRIGIE_A::Disabled)
    }
    ///EXTTRIG interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EXTTRIGIE_A::Enabled)
    }
}
///Field `CMPOKIE` reader - Compare register update OK Interrupt Enable
pub type CMPOKIE_R = crate::BitReader<CMPOKIE_A>;
///Compare register update OK Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPOKIE_A {
    ///0: CMPOK interrupt disabled
    Disabled = 0,
    ///1: CMPOK interrupt enabled
    Enabled = 1,
}
impl From<CMPOKIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPOKIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPOKIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CMPOKIE_A {
        match self.bits {
            false => CMPOKIE_A::Disabled,
            true => CMPOKIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMPOKIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMPOKIE_A::Enabled
    }
}
///Field `CMPOKIE` writer - Compare register update OK Interrupt Enable
pub type CMPOKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, CMPOKIE_A, O>;
impl<'a, const O: u8> CMPOKIE_W<'a, O> {
    ///CMPOK interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMPOKIE_A::Disabled)
    }
    ///CMPOK interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMPOKIE_A::Enabled)
    }
}
///Field `ARROKIE` reader - Autoreload register update OK Interrupt Enable
pub type ARROKIE_R = crate::BitReader<ARROKIE_A>;
///Autoreload register update OK Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARROKIE_A {
    ///0: ARROK interrupt disabled
    Disabled = 0,
    ///1: ARROK interrupt enabled
    Enabled = 1,
}
impl From<ARROKIE_A> for bool {
    #[inline(always)]
    fn from(variant: ARROKIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ARROKIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ARROKIE_A {
        match self.bits {
            false => ARROKIE_A::Disabled,
            true => ARROKIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARROKIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARROKIE_A::Enabled
    }
}
///Field `ARROKIE` writer - Autoreload register update OK Interrupt Enable
pub type ARROKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, ARROKIE_A, O>;
impl<'a, const O: u8> ARROKIE_W<'a, O> {
    ///ARROK interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ARROKIE_A::Disabled)
    }
    ///ARROK interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ARROKIE_A::Enabled)
    }
}
///Field `UPIE` reader - Direction change to UP Interrupt Enable
pub type UPIE_R = crate::BitReader<UPIE_A>;
///Direction change to UP Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPIE_A {
    ///0: UP interrupt disabled
    Disabled = 0,
    ///1: UP interrupt enabled
    Enabled = 1,
}
impl From<UPIE_A> for bool {
    #[inline(always)]
    fn from(variant: UPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl UPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UPIE_A {
        match self.bits {
            false => UPIE_A::Disabled,
            true => UPIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UPIE_A::Enabled
    }
}
///Field `UPIE` writer - Direction change to UP Interrupt Enable
pub type UPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, UPIE_A, O>;
impl<'a, const O: u8> UPIE_W<'a, O> {
    ///UP interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UPIE_A::Disabled)
    }
    ///UP interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UPIE_A::Enabled)
    }
}
///Field `DOWNIE` reader - Direction change to down Interrupt Enable
pub type DOWNIE_R = crate::BitReader<DOWNIE_A>;
///Direction change to down Interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOWNIE_A {
    ///0: DOWN interrupt disabled
    Disabled = 0,
    ///1: DOWN interrupt enabled
    Enabled = 1,
}
impl From<DOWNIE_A> for bool {
    #[inline(always)]
    fn from(variant: DOWNIE_A) -> Self {
        variant as u8 != 0
    }
}
impl DOWNIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DOWNIE_A {
        match self.bits {
            false => DOWNIE_A::Disabled,
            true => DOWNIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DOWNIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DOWNIE_A::Enabled
    }
}
///Field `DOWNIE` writer - Direction change to down Interrupt Enable
pub type DOWNIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, DOWNIE_A, O>;
impl<'a, const O: u8> DOWNIE_W<'a, O> {
    ///DOWN interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DOWNIE_A::Disabled)
    }
    ///DOWN interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DOWNIE_A::Enabled)
    }
}
///Field `UEIE` reader - Update event interrupt enable
pub type UEIE_R = crate::BitReader<UEIE_A>;
///Update event interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UEIE_A {
    ///0: Update event interrupt disabled
    Disabled = 0,
    ///1: Update event interrupt enabled
    Enabled = 1,
}
impl From<UEIE_A> for bool {
    #[inline(always)]
    fn from(variant: UEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl UEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UEIE_A {
        match self.bits {
            false => UEIE_A::Disabled,
            true => UEIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UEIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UEIE_A::Enabled
    }
}
///Field `UEIE` writer - Update event interrupt enable
pub type UEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, UEIE_A, O>;
impl<'a, const O: u8> UEIE_W<'a, O> {
    ///Update event interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UEIE_A::Disabled)
    }
    ///Update event interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UEIE_A::Enabled)
    }
}
///Field `REPOKIE` reader - Repetition register update OK interrupt Enable
pub type REPOKIE_R = crate::BitReader<REPOKIE_A>;
///Repetition register update OK interrupt Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REPOKIE_A {
    ///0: Repetition register update OK interrupt disabled
    Disabled = 0,
    ///1: Repetition register update OK interrupt enabled
    Enabled = 1,
}
impl From<REPOKIE_A> for bool {
    #[inline(always)]
    fn from(variant: REPOKIE_A) -> Self {
        variant as u8 != 0
    }
}
impl REPOKIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> REPOKIE_A {
        match self.bits {
            false => REPOKIE_A::Disabled,
            true => REPOKIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REPOKIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REPOKIE_A::Enabled
    }
}
///Field `REPOKIE` writer - Repetition register update OK interrupt Enable
pub type REPOKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, REPOKIE_A, O>;
impl<'a, const O: u8> REPOKIE_W<'a, O> {
    ///Repetition register update OK interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REPOKIE_A::Disabled)
    }
    ///Repetition register update OK interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REPOKIE_A::Enabled)
    }
}
impl R {
    ///Bit 0 - Compare match Interrupt Enable
    #[inline(always)]
    pub fn cmpmie(&self) -> CMPMIE_R {
        CMPMIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Autoreload match Interrupt Enable
    #[inline(always)]
    pub fn arrmie(&self) -> ARRMIE_R {
        ARRMIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - External trigger valid edge Interrupt Enable
    #[inline(always)]
    pub fn exttrigie(&self) -> EXTTRIGIE_R {
        EXTTRIGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Compare register update OK Interrupt Enable
    #[inline(always)]
    pub fn cmpokie(&self) -> CMPOKIE_R {
        CMPOKIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Autoreload register update OK Interrupt Enable
    #[inline(always)]
    pub fn arrokie(&self) -> ARROKIE_R {
        ARROKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Direction change to UP Interrupt Enable
    #[inline(always)]
    pub fn upie(&self) -> UPIE_R {
        UPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Direction change to down Interrupt Enable
    #[inline(always)]
    pub fn downie(&self) -> DOWNIE_R {
        DOWNIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Update event interrupt enable
    #[inline(always)]
    pub fn ueie(&self) -> UEIE_R {
        UEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Repetition register update OK interrupt Enable
    #[inline(always)]
    pub fn repokie(&self) -> REPOKIE_R {
        REPOKIE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Compare match Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn cmpmie(&mut self) -> CMPMIE_W<0> {
        CMPMIE_W::new(self)
    }
    ///Bit 1 - Autoreload match Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn arrmie(&mut self) -> ARRMIE_W<1> {
        ARRMIE_W::new(self)
    }
    ///Bit 2 - External trigger valid edge Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn exttrigie(&mut self) -> EXTTRIGIE_W<2> {
        EXTTRIGIE_W::new(self)
    }
    ///Bit 3 - Compare register update OK Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn cmpokie(&mut self) -> CMPOKIE_W<3> {
        CMPOKIE_W::new(self)
    }
    ///Bit 4 - Autoreload register update OK Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn arrokie(&mut self) -> ARROKIE_W<4> {
        ARROKIE_W::new(self)
    }
    ///Bit 5 - Direction change to UP Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn upie(&mut self) -> UPIE_W<5> {
        UPIE_W::new(self)
    }
    ///Bit 6 - Direction change to down Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn downie(&mut self) -> DOWNIE_W<6> {
        DOWNIE_W::new(self)
    }
    ///Bit 7 - Update event interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn ueie(&mut self) -> UEIE_W<7> {
        UEIE_W::new(self)
    }
    ///Bit 8 - Repetition register update OK interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn repokie(&mut self) -> REPOKIE_W<8> {
        REPOKIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ier](index.html) module
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
///`read()` method returns [ier::R](R) reader structure
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ier::W](W) writer structure
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
