///Register `FLTER` reader
pub struct R(crate::R<FLTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLTER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLTER` writer
pub struct W(crate::W<FLTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLTER_SPEC>;
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
impl From<crate::W<FLTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLTER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FLT1EN` reader - Fault 1 enable
pub type FLT1EN_R = crate::BitReader<FLT1EN_A>;
///Fault 1 enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLT1EN_A {
    ///0: Fault input ignored
    Ignored = 0,
    ///1: Fault input is active and can disable HRTIM outputs
    Active = 1,
}
impl From<FLT1EN_A> for bool {
    #[inline(always)]
    fn from(variant: FLT1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FLT1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FLT1EN_A {
        match self.bits {
            false => FLT1EN_A::Ignored,
            true => FLT1EN_A::Active,
        }
    }
    ///Checks if the value of the field is `Ignored`
    #[inline(always)]
    pub fn is_ignored(&self) -> bool {
        *self == FLT1EN_A::Ignored
    }
    ///Checks if the value of the field is `Active`
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == FLT1EN_A::Active
    }
}
///Field `FLT1EN` writer - Fault 1 enable
pub type FLT1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTER_SPEC, FLT1EN_A, O>;
impl<'a, const O: u8> FLT1EN_W<'a, O> {
    ///Fault input ignored
    #[inline(always)]
    pub fn ignored(self) -> &'a mut W {
        self.variant(FLT1EN_A::Ignored)
    }
    ///Fault input is active and can disable HRTIM outputs
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(FLT1EN_A::Active)
    }
}
///Field `FLT2EN` reader - Fault 2 enable
pub use FLT1EN_R as FLT2EN_R;
///Field `FLT3EN` reader - Fault 3 enable
pub use FLT1EN_R as FLT3EN_R;
///Field `FLT4EN` reader - Fault 4 enable
pub use FLT1EN_R as FLT4EN_R;
///Field `FLT5EN` reader - Fault 5 enable
pub use FLT1EN_R as FLT5EN_R;
///Field `FLT2EN` writer - Fault 2 enable
pub use FLT1EN_W as FLT2EN_W;
///Field `FLT3EN` writer - Fault 3 enable
pub use FLT1EN_W as FLT3EN_W;
///Field `FLT4EN` writer - Fault 4 enable
pub use FLT1EN_W as FLT4EN_W;
///Field `FLT5EN` writer - Fault 5 enable
pub use FLT1EN_W as FLT5EN_W;
///Field `FLTLCK` reader - Fault sources Lock
pub type FLTLCK_R = crate::BitReader<FLTLCK_A>;
///Fault sources Lock
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLTLCK_A {
    ///0: FLT1EN..FLT5EN bits are read/write
    Unlocked = 0,
    ///1: FLT1EN..FLT5EN bits are read only
    Locked = 1,
}
impl From<FLTLCK_A> for bool {
    #[inline(always)]
    fn from(variant: FLTLCK_A) -> Self {
        variant as u8 != 0
    }
}
impl FLTLCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FLTLCK_A {
        match self.bits {
            false => FLTLCK_A::Unlocked,
            true => FLTLCK_A::Locked,
        }
    }
    ///Checks if the value of the field is `Unlocked`
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == FLTLCK_A::Unlocked
    }
    ///Checks if the value of the field is `Locked`
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == FLTLCK_A::Locked
    }
}
///Field `FLTLCK` writer - Fault sources Lock
pub type FLTLCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLTER_SPEC, FLTLCK_A, O>;
impl<'a, const O: u8> FLTLCK_W<'a, O> {
    ///FLT1EN..FLT5EN bits are read/write
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(FLTLCK_A::Unlocked)
    }
    ///FLT1EN..FLT5EN bits are read only
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(FLTLCK_A::Locked)
    }
}
impl R {
    ///Bit 0 - Fault 1 enable
    #[inline(always)]
    pub fn flt1en(&self) -> FLT1EN_R {
        FLT1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Fault 2 enable
    #[inline(always)]
    pub fn flt2en(&self) -> FLT2EN_R {
        FLT2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Fault 3 enable
    #[inline(always)]
    pub fn flt3en(&self) -> FLT3EN_R {
        FLT3EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Fault 4 enable
    #[inline(always)]
    pub fn flt4en(&self) -> FLT4EN_R {
        FLT4EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Fault 5 enable
    #[inline(always)]
    pub fn flt5en(&self) -> FLT5EN_R {
        FLT5EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 31 - Fault sources Lock
    #[inline(always)]
    pub fn fltlck(&self) -> FLTLCK_R {
        FLTLCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Fault 1 enable
    #[inline(always)]
    #[must_use]
    pub fn flt1en(&mut self) -> FLT1EN_W<0> {
        FLT1EN_W::new(self)
    }
    ///Bit 1 - Fault 2 enable
    #[inline(always)]
    #[must_use]
    pub fn flt2en(&mut self) -> FLT2EN_W<1> {
        FLT2EN_W::new(self)
    }
    ///Bit 2 - Fault 3 enable
    #[inline(always)]
    #[must_use]
    pub fn flt3en(&mut self) -> FLT3EN_W<2> {
        FLT3EN_W::new(self)
    }
    ///Bit 3 - Fault 4 enable
    #[inline(always)]
    #[must_use]
    pub fn flt4en(&mut self) -> FLT4EN_W<3> {
        FLT4EN_W::new(self)
    }
    ///Bit 4 - Fault 5 enable
    #[inline(always)]
    #[must_use]
    pub fn flt5en(&mut self) -> FLT5EN_W<4> {
        FLT5EN_W::new(self)
    }
    ///Bit 31 - Fault sources Lock
    #[inline(always)]
    #[must_use]
    pub fn fltlck(&mut self) -> FLTLCK_W<31> {
        FLTLCK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timerx Fault Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flter](index.html) module
pub struct FLTER_SPEC;
impl crate::RegisterSpec for FLTER_SPEC {
    type Ux = u32;
}
///`read()` method returns [flter::R](R) reader structure
impl crate::Readable for FLTER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [flter::W](W) writer structure
impl crate::Writable for FLTER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLTER to value 0
impl crate::Resettable for FLTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
