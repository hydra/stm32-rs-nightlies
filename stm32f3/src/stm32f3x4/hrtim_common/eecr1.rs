///Register `EECR1` reader
pub struct R(crate::R<EECR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EECR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EECR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EECR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EECR1` writer
pub struct W(crate::W<EECR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EECR1_SPEC>;
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
impl From<crate::W<EECR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EECR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EE1SRC` reader - External Event 1 Source
pub type EE1SRC_R = crate::FieldReader<u8, EE1SRC_A>;
///External Event 1 Source
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EE1SRC_A {
    ///0: Source 1
    Src1 = 0,
    ///1: Source 2
    Src2 = 1,
    ///2: Source 3
    Src3 = 2,
    ///3: Source 4
    Src4 = 3,
}
impl From<EE1SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: EE1SRC_A) -> Self {
        variant as _
    }
}
impl EE1SRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EE1SRC_A {
        match self.bits {
            0 => EE1SRC_A::Src1,
            1 => EE1SRC_A::Src2,
            2 => EE1SRC_A::Src3,
            3 => EE1SRC_A::Src4,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Src1`
    #[inline(always)]
    pub fn is_src1(&self) -> bool {
        *self == EE1SRC_A::Src1
    }
    ///Checks if the value of the field is `Src2`
    #[inline(always)]
    pub fn is_src2(&self) -> bool {
        *self == EE1SRC_A::Src2
    }
    ///Checks if the value of the field is `Src3`
    #[inline(always)]
    pub fn is_src3(&self) -> bool {
        *self == EE1SRC_A::Src3
    }
    ///Checks if the value of the field is `Src4`
    #[inline(always)]
    pub fn is_src4(&self) -> bool {
        *self == EE1SRC_A::Src4
    }
}
///Field `EE1SRC` writer - External Event 1 Source
pub type EE1SRC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EECR1_SPEC, u8, EE1SRC_A, 2, O>;
impl<'a, const O: u8> EE1SRC_W<'a, O> {
    ///Source 1
    #[inline(always)]
    pub fn src1(self) -> &'a mut W {
        self.variant(EE1SRC_A::Src1)
    }
    ///Source 2
    #[inline(always)]
    pub fn src2(self) -> &'a mut W {
        self.variant(EE1SRC_A::Src2)
    }
    ///Source 3
    #[inline(always)]
    pub fn src3(self) -> &'a mut W {
        self.variant(EE1SRC_A::Src3)
    }
    ///Source 4
    #[inline(always)]
    pub fn src4(self) -> &'a mut W {
        self.variant(EE1SRC_A::Src4)
    }
}
///Field `EE1POL` reader - External Event 1 Polarity
pub type EE1POL_R = crate::BitReader<EE1POL_A>;
///External Event 1 Polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EE1POL_A {
    ///0: External event is active high
    ActiveHigh = 0,
    ///1: External event is active low
    ActiveLow = 1,
}
impl From<EE1POL_A> for bool {
    #[inline(always)]
    fn from(variant: EE1POL_A) -> Self {
        variant as u8 != 0
    }
}
impl EE1POL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EE1POL_A {
        match self.bits {
            false => EE1POL_A::ActiveHigh,
            true => EE1POL_A::ActiveLow,
        }
    }
    ///Checks if the value of the field is `ActiveHigh`
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == EE1POL_A::ActiveHigh
    }
    ///Checks if the value of the field is `ActiveLow`
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == EE1POL_A::ActiveLow
    }
}
///Field `EE1POL` writer - External Event 1 Polarity
pub type EE1POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EECR1_SPEC, EE1POL_A, O>;
impl<'a, const O: u8> EE1POL_W<'a, O> {
    ///External event is active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(EE1POL_A::ActiveHigh)
    }
    ///External event is active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(EE1POL_A::ActiveLow)
    }
}
///Field `EE1SNS` reader - External Event 1 Sensitivity
pub type EE1SNS_R = crate::FieldReader<u8, EE1SNS_A>;
///External Event 1 Sensitivity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EE1SNS_A {
    ///0: On active level defined by EExPOL bit
    Active = 0,
    ///1: Rising edge
    Rising = 1,
    ///2: Falling edge
    Falling = 2,
    ///3: Both edges
    Both = 3,
}
impl From<EE1SNS_A> for u8 {
    #[inline(always)]
    fn from(variant: EE1SNS_A) -> Self {
        variant as _
    }
}
impl EE1SNS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EE1SNS_A {
        match self.bits {
            0 => EE1SNS_A::Active,
            1 => EE1SNS_A::Rising,
            2 => EE1SNS_A::Falling,
            3 => EE1SNS_A::Both,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Active`
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == EE1SNS_A::Active
    }
    ///Checks if the value of the field is `Rising`
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == EE1SNS_A::Rising
    }
    ///Checks if the value of the field is `Falling`
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == EE1SNS_A::Falling
    }
    ///Checks if the value of the field is `Both`
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == EE1SNS_A::Both
    }
}
///Field `EE1SNS` writer - External Event 1 Sensitivity
pub type EE1SNS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EECR1_SPEC, u8, EE1SNS_A, 2, O>;
impl<'a, const O: u8> EE1SNS_W<'a, O> {
    ///On active level defined by EExPOL bit
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(EE1SNS_A::Active)
    }
    ///Rising edge
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(EE1SNS_A::Rising)
    }
    ///Falling edge
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(EE1SNS_A::Falling)
    }
    ///Both edges
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(EE1SNS_A::Both)
    }
}
///Field `EE1FAST` reader - External Event 1 Fast mode
pub type EE1FAST_R = crate::BitReader<EE1FAST_A>;
///External Event 1 Fast mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EE1FAST_A {
    ///0: External event is re-synchronised by the HRTIM logic before acting on outputs
    Resynchronized = 0,
    ///1: External event is acting asynchronously on outputs (low-latency mode)
    Asynchronous = 1,
}
impl From<EE1FAST_A> for bool {
    #[inline(always)]
    fn from(variant: EE1FAST_A) -> Self {
        variant as u8 != 0
    }
}
impl EE1FAST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EE1FAST_A {
        match self.bits {
            false => EE1FAST_A::Resynchronized,
            true => EE1FAST_A::Asynchronous,
        }
    }
    ///Checks if the value of the field is `Resynchronized`
    #[inline(always)]
    pub fn is_resynchronized(&self) -> bool {
        *self == EE1FAST_A::Resynchronized
    }
    ///Checks if the value of the field is `Asynchronous`
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == EE1FAST_A::Asynchronous
    }
}
///Field `EE1FAST` writer - External Event 1 Fast mode
pub type EE1FAST_W<'a, const O: u8> = crate::BitWriter<'a, u32, EECR1_SPEC, EE1FAST_A, O>;
impl<'a, const O: u8> EE1FAST_W<'a, O> {
    ///External event is re-synchronised by the HRTIM logic before acting on outputs
    #[inline(always)]
    pub fn resynchronized(self) -> &'a mut W {
        self.variant(EE1FAST_A::Resynchronized)
    }
    ///External event is acting asynchronously on outputs (low-latency mode)
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(EE1FAST_A::Asynchronous)
    }
}
///Field `EE2FAST` reader - External Event 2 Fast mode
pub use EE1FAST_R as EE2FAST_R;
///Field `EE3FAST` reader - External Event 3 Fast mode
pub use EE1FAST_R as EE3FAST_R;
///Field `EE4FAST` reader - External Event 4 Fast mode
pub use EE1FAST_R as EE4FAST_R;
///Field `EE5FAST` reader - External Event 5 Fast mode
pub use EE1FAST_R as EE5FAST_R;
///Field `EE2FAST` writer - External Event 2 Fast mode
pub use EE1FAST_W as EE2FAST_W;
///Field `EE3FAST` writer - External Event 3 Fast mode
pub use EE1FAST_W as EE3FAST_W;
///Field `EE4FAST` writer - External Event 4 Fast mode
pub use EE1FAST_W as EE4FAST_W;
///Field `EE5FAST` writer - External Event 5 Fast mode
pub use EE1FAST_W as EE5FAST_W;
///Field `EE2POL` reader - External Event 2 Polarity
pub use EE1POL_R as EE2POL_R;
///Field `EE3POL` reader - External Event 3 Polarity
pub use EE1POL_R as EE3POL_R;
///Field `EE4POL` reader - External Event 4 Polarity
pub use EE1POL_R as EE4POL_R;
///Field `EE5POL` reader - External Event 5 Polarity
pub use EE1POL_R as EE5POL_R;
///Field `EE2POL` writer - External Event 2 Polarity
pub use EE1POL_W as EE2POL_W;
///Field `EE3POL` writer - External Event 3 Polarity
pub use EE1POL_W as EE3POL_W;
///Field `EE4POL` writer - External Event 4 Polarity
pub use EE1POL_W as EE4POL_W;
///Field `EE5POL` writer - External Event 5 Polarity
pub use EE1POL_W as EE5POL_W;
///Field `EE2SNS` reader - External Event 2 Sensitivity
pub use EE1SNS_R as EE2SNS_R;
///Field `EE3SNS` reader - External Event 3 Sensitivity
pub use EE1SNS_R as EE3SNS_R;
///Field `EE4SNS` reader - External Event 4 Sensitivity
pub use EE1SNS_R as EE4SNS_R;
///Field `EE5SNS` reader - External Event 5 Sensitivity
pub use EE1SNS_R as EE5SNS_R;
///Field `EE2SNS` writer - External Event 2 Sensitivity
pub use EE1SNS_W as EE2SNS_W;
///Field `EE3SNS` writer - External Event 3 Sensitivity
pub use EE1SNS_W as EE3SNS_W;
///Field `EE4SNS` writer - External Event 4 Sensitivity
pub use EE1SNS_W as EE4SNS_W;
///Field `EE5SNS` writer - External Event 5 Sensitivity
pub use EE1SNS_W as EE5SNS_W;
///Field `EE2SRC` reader - External Event 2 Source
pub use EE1SRC_R as EE2SRC_R;
///Field `EE3SRC` reader - External Event 3 Source
pub use EE1SRC_R as EE3SRC_R;
///Field `EE4SRC` reader - External Event 4 Source
pub use EE1SRC_R as EE4SRC_R;
///Field `EE5SRC` reader - External Event 5 Source
pub use EE1SRC_R as EE5SRC_R;
///Field `EE2SRC` writer - External Event 2 Source
pub use EE1SRC_W as EE2SRC_W;
///Field `EE3SRC` writer - External Event 3 Source
pub use EE1SRC_W as EE3SRC_W;
///Field `EE4SRC` writer - External Event 4 Source
pub use EE1SRC_W as EE4SRC_W;
///Field `EE5SRC` writer - External Event 5 Source
pub use EE1SRC_W as EE5SRC_W;
impl R {
    ///Bits 0:1 - External Event 1 Source
    #[inline(always)]
    pub fn ee1src(&self) -> EE1SRC_R {
        EE1SRC_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - External Event 1 Polarity
    #[inline(always)]
    pub fn ee1pol(&self) -> EE1POL_R {
        EE1POL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - External Event 1 Sensitivity
    #[inline(always)]
    pub fn ee1sns(&self) -> EE1SNS_R {
        EE1SNS_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - External Event 1 Fast mode
    #[inline(always)]
    pub fn ee1fast(&self) -> EE1FAST_R {
        EE1FAST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - External Event 2 Source
    #[inline(always)]
    pub fn ee2src(&self) -> EE2SRC_R {
        EE2SRC_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - External Event 2 Polarity
    #[inline(always)]
    pub fn ee2pol(&self) -> EE2POL_R {
        EE2POL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - External Event 2 Sensitivity
    #[inline(always)]
    pub fn ee2sns(&self) -> EE2SNS_R {
        EE2SNS_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bit 11 - External Event 2 Fast mode
    #[inline(always)]
    pub fn ee2fast(&self) -> EE2FAST_R {
        EE2FAST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - External Event 3 Source
    #[inline(always)]
    pub fn ee3src(&self) -> EE3SRC_R {
        EE3SRC_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - External Event 3 Polarity
    #[inline(always)]
    pub fn ee3pol(&self) -> EE3POL_R {
        EE3POL_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 15:16 - External Event 3 Sensitivity
    #[inline(always)]
    pub fn ee3sns(&self) -> EE3SNS_R {
        EE3SNS_R::new(((self.bits >> 15) & 3) as u8)
    }
    ///Bit 17 - External Event 3 Fast mode
    #[inline(always)]
    pub fn ee3fast(&self) -> EE3FAST_R {
        EE3FAST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - External Event 4 Source
    #[inline(always)]
    pub fn ee4src(&self) -> EE4SRC_R {
        EE4SRC_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 20 - External Event 4 Polarity
    #[inline(always)]
    pub fn ee4pol(&self) -> EE4POL_R {
        EE4POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - External Event 4 Sensitivity
    #[inline(always)]
    pub fn ee4sns(&self) -> EE4SNS_R {
        EE4SNS_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 23 - External Event 4 Fast mode
    #[inline(always)]
    pub fn ee4fast(&self) -> EE4FAST_R {
        EE4FAST_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:25 - External Event 5 Source
    #[inline(always)]
    pub fn ee5src(&self) -> EE5SRC_R {
        EE5SRC_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bit 26 - External Event 5 Polarity
    #[inline(always)]
    pub fn ee5pol(&self) -> EE5POL_R {
        EE5POL_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 27:28 - External Event 5 Sensitivity
    #[inline(always)]
    pub fn ee5sns(&self) -> EE5SNS_R {
        EE5SNS_R::new(((self.bits >> 27) & 3) as u8)
    }
    ///Bit 29 - External Event 5 Fast mode
    #[inline(always)]
    pub fn ee5fast(&self) -> EE5FAST_R {
        EE5FAST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - External Event 1 Source
    #[inline(always)]
    #[must_use]
    pub fn ee1src(&mut self) -> EE1SRC_W<0> {
        EE1SRC_W::new(self)
    }
    ///Bit 2 - External Event 1 Polarity
    #[inline(always)]
    #[must_use]
    pub fn ee1pol(&mut self) -> EE1POL_W<2> {
        EE1POL_W::new(self)
    }
    ///Bits 3:4 - External Event 1 Sensitivity
    #[inline(always)]
    #[must_use]
    pub fn ee1sns(&mut self) -> EE1SNS_W<3> {
        EE1SNS_W::new(self)
    }
    ///Bit 5 - External Event 1 Fast mode
    #[inline(always)]
    #[must_use]
    pub fn ee1fast(&mut self) -> EE1FAST_W<5> {
        EE1FAST_W::new(self)
    }
    ///Bits 6:7 - External Event 2 Source
    #[inline(always)]
    #[must_use]
    pub fn ee2src(&mut self) -> EE2SRC_W<6> {
        EE2SRC_W::new(self)
    }
    ///Bit 8 - External Event 2 Polarity
    #[inline(always)]
    #[must_use]
    pub fn ee2pol(&mut self) -> EE2POL_W<8> {
        EE2POL_W::new(self)
    }
    ///Bits 9:10 - External Event 2 Sensitivity
    #[inline(always)]
    #[must_use]
    pub fn ee2sns(&mut self) -> EE2SNS_W<9> {
        EE2SNS_W::new(self)
    }
    ///Bit 11 - External Event 2 Fast mode
    #[inline(always)]
    #[must_use]
    pub fn ee2fast(&mut self) -> EE2FAST_W<11> {
        EE2FAST_W::new(self)
    }
    ///Bits 12:13 - External Event 3 Source
    #[inline(always)]
    #[must_use]
    pub fn ee3src(&mut self) -> EE3SRC_W<12> {
        EE3SRC_W::new(self)
    }
    ///Bit 14 - External Event 3 Polarity
    #[inline(always)]
    #[must_use]
    pub fn ee3pol(&mut self) -> EE3POL_W<14> {
        EE3POL_W::new(self)
    }
    ///Bits 15:16 - External Event 3 Sensitivity
    #[inline(always)]
    #[must_use]
    pub fn ee3sns(&mut self) -> EE3SNS_W<15> {
        EE3SNS_W::new(self)
    }
    ///Bit 17 - External Event 3 Fast mode
    #[inline(always)]
    #[must_use]
    pub fn ee3fast(&mut self) -> EE3FAST_W<17> {
        EE3FAST_W::new(self)
    }
    ///Bits 18:19 - External Event 4 Source
    #[inline(always)]
    #[must_use]
    pub fn ee4src(&mut self) -> EE4SRC_W<18> {
        EE4SRC_W::new(self)
    }
    ///Bit 20 - External Event 4 Polarity
    #[inline(always)]
    #[must_use]
    pub fn ee4pol(&mut self) -> EE4POL_W<20> {
        EE4POL_W::new(self)
    }
    ///Bits 21:22 - External Event 4 Sensitivity
    #[inline(always)]
    #[must_use]
    pub fn ee4sns(&mut self) -> EE4SNS_W<21> {
        EE4SNS_W::new(self)
    }
    ///Bit 23 - External Event 4 Fast mode
    #[inline(always)]
    #[must_use]
    pub fn ee4fast(&mut self) -> EE4FAST_W<23> {
        EE4FAST_W::new(self)
    }
    ///Bits 24:25 - External Event 5 Source
    #[inline(always)]
    #[must_use]
    pub fn ee5src(&mut self) -> EE5SRC_W<24> {
        EE5SRC_W::new(self)
    }
    ///Bit 26 - External Event 5 Polarity
    #[inline(always)]
    #[must_use]
    pub fn ee5pol(&mut self) -> EE5POL_W<26> {
        EE5POL_W::new(self)
    }
    ///Bits 27:28 - External Event 5 Sensitivity
    #[inline(always)]
    #[must_use]
    pub fn ee5sns(&mut self) -> EE5SNS_W<27> {
        EE5SNS_W::new(self)
    }
    ///Bit 29 - External Event 5 Fast mode
    #[inline(always)]
    #[must_use]
    pub fn ee5fast(&mut self) -> EE5FAST_W<29> {
        EE5FAST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timer External Event Control Register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eecr1](index.html) module
pub struct EECR1_SPEC;
impl crate::RegisterSpec for EECR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [eecr1::R](R) reader structure
impl crate::Readable for EECR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [eecr1::W](W) writer structure
impl crate::Writable for EECR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EECR1 to value 0
impl crate::Resettable for EECR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
