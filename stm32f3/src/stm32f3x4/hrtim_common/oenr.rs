///Register `OENR` reader
pub struct R(crate::R<OENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OENR` writer
pub struct W(crate::W<OENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OENR_SPEC>;
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
impl From<crate::W<OENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TA1OEN` reader - Timer A Output 1 Enable
pub type TA1OEN_R = crate::BitReader<TA1OENR_A>;
///Timer A Output 1 Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TA1OENR_A {
    ///0: Output disabled
    Disabled = 0,
    ///1: Output enabled
    Enabled = 1,
}
impl From<TA1OENR_A> for bool {
    #[inline(always)]
    fn from(variant: TA1OENR_A) -> Self {
        variant as u8 != 0
    }
}
impl TA1OEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TA1OENR_A {
        match self.bits {
            false => TA1OENR_A::Disabled,
            true => TA1OENR_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TA1OENR_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TA1OENR_A::Enabled
    }
}
///Timer A Output 1 Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TA1OENW_AW {
    ///1: Enable output
    Enable = 1,
}
impl From<TA1OENW_AW> for bool {
    #[inline(always)]
    fn from(variant: TA1OENW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `TA1OEN` writer - Timer A Output 1 Enable
pub type TA1OEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OENR_SPEC, TA1OENW_AW, O>;
impl<'a, const O: u8> TA1OEN_W<'a, O> {
    ///Enable output
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TA1OENW_AW::Enable)
    }
}
///Field `TA2OEN` reader - Timer A Output 2 Enable
pub use TA1OEN_R as TA2OEN_R;
///Field `TB1OEN` reader - Timer B Output 1 Enable
pub use TA1OEN_R as TB1OEN_R;
///Field `TB2OEN` reader - Timer B Output 2 Enable
pub use TA1OEN_R as TB2OEN_R;
///Field `TC1OEN` reader - Timer C Output 1 Enable
pub use TA1OEN_R as TC1OEN_R;
///Field `TC2OEN` reader - Timer C Output 2 Enable
pub use TA1OEN_R as TC2OEN_R;
///Field `TD1OEN` reader - Timer D Output 1 Enable
pub use TA1OEN_R as TD1OEN_R;
///Field `TD2OEN` reader - Timer D Output 2 Enable
pub use TA1OEN_R as TD2OEN_R;
///Field `TE1OEN` reader - Timer E Output 1 Enable
pub use TA1OEN_R as TE1OEN_R;
///Field `TE2OEN` reader - Timer E Output 2 Enable
pub use TA1OEN_R as TE2OEN_R;
///Field `TA2OEN` writer - Timer A Output 2 Enable
pub use TA1OEN_W as TA2OEN_W;
///Field `TB1OEN` writer - Timer B Output 1 Enable
pub use TA1OEN_W as TB1OEN_W;
///Field `TB2OEN` writer - Timer B Output 2 Enable
pub use TA1OEN_W as TB2OEN_W;
///Field `TC1OEN` writer - Timer C Output 1 Enable
pub use TA1OEN_W as TC1OEN_W;
///Field `TC2OEN` writer - Timer C Output 2 Enable
pub use TA1OEN_W as TC2OEN_W;
///Field `TD1OEN` writer - Timer D Output 1 Enable
pub use TA1OEN_W as TD1OEN_W;
///Field `TD2OEN` writer - Timer D Output 2 Enable
pub use TA1OEN_W as TD2OEN_W;
///Field `TE1OEN` writer - Timer E Output 1 Enable
pub use TA1OEN_W as TE1OEN_W;
///Field `TE2OEN` writer - Timer E Output 2 Enable
pub use TA1OEN_W as TE2OEN_W;
impl R {
    ///Bit 0 - Timer A Output 1 Enable
    #[inline(always)]
    pub fn ta1oen(&self) -> TA1OEN_R {
        TA1OEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timer A Output 2 Enable
    #[inline(always)]
    pub fn ta2oen(&self) -> TA2OEN_R {
        TA2OEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Timer B Output 1 Enable
    #[inline(always)]
    pub fn tb1oen(&self) -> TB1OEN_R {
        TB1OEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timer B Output 2 Enable
    #[inline(always)]
    pub fn tb2oen(&self) -> TB2OEN_R {
        TB2OEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timer C Output 1 Enable
    #[inline(always)]
    pub fn tc1oen(&self) -> TC1OEN_R {
        TC1OEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Timer C Output 2 Enable
    #[inline(always)]
    pub fn tc2oen(&self) -> TC2OEN_R {
        TC2OEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Timer D Output 1 Enable
    #[inline(always)]
    pub fn td1oen(&self) -> TD1OEN_R {
        TD1OEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Timer D Output 2 Enable
    #[inline(always)]
    pub fn td2oen(&self) -> TD2OEN_R {
        TD2OEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Timer E Output 1 Enable
    #[inline(always)]
    pub fn te1oen(&self) -> TE1OEN_R {
        TE1OEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Timer E Output 2 Enable
    #[inline(always)]
    pub fn te2oen(&self) -> TE2OEN_R {
        TE2OEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Timer A Output 1 Enable
    #[inline(always)]
    #[must_use]
    pub fn ta1oen(&mut self) -> TA1OEN_W<0> {
        TA1OEN_W::new(self)
    }
    ///Bit 1 - Timer A Output 2 Enable
    #[inline(always)]
    #[must_use]
    pub fn ta2oen(&mut self) -> TA2OEN_W<1> {
        TA2OEN_W::new(self)
    }
    ///Bit 2 - Timer B Output 1 Enable
    #[inline(always)]
    #[must_use]
    pub fn tb1oen(&mut self) -> TB1OEN_W<2> {
        TB1OEN_W::new(self)
    }
    ///Bit 3 - Timer B Output 2 Enable
    #[inline(always)]
    #[must_use]
    pub fn tb2oen(&mut self) -> TB2OEN_W<3> {
        TB2OEN_W::new(self)
    }
    ///Bit 4 - Timer C Output 1 Enable
    #[inline(always)]
    #[must_use]
    pub fn tc1oen(&mut self) -> TC1OEN_W<4> {
        TC1OEN_W::new(self)
    }
    ///Bit 5 - Timer C Output 2 Enable
    #[inline(always)]
    #[must_use]
    pub fn tc2oen(&mut self) -> TC2OEN_W<5> {
        TC2OEN_W::new(self)
    }
    ///Bit 6 - Timer D Output 1 Enable
    #[inline(always)]
    #[must_use]
    pub fn td1oen(&mut self) -> TD1OEN_W<6> {
        TD1OEN_W::new(self)
    }
    ///Bit 7 - Timer D Output 2 Enable
    #[inline(always)]
    #[must_use]
    pub fn td2oen(&mut self) -> TD2OEN_W<7> {
        TD2OEN_W::new(self)
    }
    ///Bit 8 - Timer E Output 1 Enable
    #[inline(always)]
    #[must_use]
    pub fn te1oen(&mut self) -> TE1OEN_W<8> {
        TE1OEN_W::new(self)
    }
    ///Bit 9 - Timer E Output 2 Enable
    #[inline(always)]
    #[must_use]
    pub fn te2oen(&mut self) -> TE2OEN_W<9> {
        TE2OEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Output Enable Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [oenr](index.html) module
pub struct OENR_SPEC;
impl crate::RegisterSpec for OENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [oenr::R](R) reader structure
impl crate::Readable for OENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [oenr::W](W) writer structure
impl crate::Writable for OENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OENR to value 0
impl crate::Resettable for OENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
