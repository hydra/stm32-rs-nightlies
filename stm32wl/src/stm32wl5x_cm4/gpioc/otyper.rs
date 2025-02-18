///Register `OTYPER` reader
pub struct R(crate::R<OTYPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTYPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTYPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTYPER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OTYPER` writer
pub struct W(crate::W<OTYPER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTYPER_SPEC>;
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
impl From<crate::W<OTYPER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTYPER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OT0` reader - Port x configuration bits (y = 0..15)
pub type OT0_R = crate::BitReader<OT0_A>;
///Port x configuration bits (y = 0..15)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OT0_A {
    ///0: Output push-pull (reset state)
    PushPull = 0,
    ///1: Output open-drain
    OpenDrain = 1,
}
impl From<OT0_A> for bool {
    #[inline(always)]
    fn from(variant: OT0_A) -> Self {
        variant as u8 != 0
    }
}
impl OT0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OT0_A {
        match self.bits {
            false => OT0_A::PushPull,
            true => OT0_A::OpenDrain,
        }
    }
    ///Checks if the value of the field is `PushPull`
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        *self == OT0_A::PushPull
    }
    ///Checks if the value of the field is `OpenDrain`
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == OT0_A::OpenDrain
    }
}
///Field `OT0` writer - Port x configuration bits (y = 0..15)
pub type OT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTYPER_SPEC, OT0_A, O>;
impl<'a, const O: u8> OT0_W<'a, O> {
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut W {
        self.variant(OT0_A::PushPull)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(OT0_A::OpenDrain)
    }
}
///Field `OT1` reader - Port x configuration bits (y = 0..15)
pub use OT0_R as OT1_R;
///Field `OT2` reader - Port x configuration bits (y = 0..15)
pub use OT0_R as OT2_R;
///Field `OT3` reader - Port x configuration bits (y = 0..15)
pub use OT0_R as OT3_R;
///Field `OT4` reader - Port x configuration bits (y = 0..15)
pub use OT0_R as OT4_R;
///Field `OT5` reader - Port x configuration bits (y = 0..15)
pub use OT0_R as OT5_R;
///Field `OT6` reader - Port x configuration bits (y = 0..15)
pub use OT0_R as OT6_R;
///Field `OT13` reader - Port x configuration bits (y = 0..15)
pub use OT0_R as OT13_R;
///Field `OT14` reader - Port x configuration bits (y = 0..15)
pub use OT0_R as OT14_R;
///Field `OT15` reader - Port x configuration bits (y = 0..15)
pub use OT0_R as OT15_R;
///Field `OT1` writer - Port x configuration bits (y = 0..15)
pub use OT0_W as OT1_W;
///Field `OT2` writer - Port x configuration bits (y = 0..15)
pub use OT0_W as OT2_W;
///Field `OT3` writer - Port x configuration bits (y = 0..15)
pub use OT0_W as OT3_W;
///Field `OT4` writer - Port x configuration bits (y = 0..15)
pub use OT0_W as OT4_W;
///Field `OT5` writer - Port x configuration bits (y = 0..15)
pub use OT0_W as OT5_W;
///Field `OT6` writer - Port x configuration bits (y = 0..15)
pub use OT0_W as OT6_W;
///Field `OT13` writer - Port x configuration bits (y = 0..15)
pub use OT0_W as OT13_W;
///Field `OT14` writer - Port x configuration bits (y = 0..15)
pub use OT0_W as OT14_W;
///Field `OT15` writer - Port x configuration bits (y = 0..15)
pub use OT0_W as OT15_W;
impl R {
    ///Bit 0 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot0(&self) -> OT0_R {
        OT0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot1(&self) -> OT1_R {
        OT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot2(&self) -> OT2_R {
        OT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot3(&self) -> OT3_R {
        OT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot4(&self) -> OT4_R {
        OT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot5(&self) -> OT5_R {
        OT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot6(&self) -> OT6_R {
        OT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot13(&self) -> OT13_R {
        OT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot14(&self) -> OT14_R {
        OT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot15(&self) -> OT15_R {
        OT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn ot0(&mut self) -> OT0_W<0> {
        OT0_W::new(self)
    }
    ///Bit 1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn ot1(&mut self) -> OT1_W<1> {
        OT1_W::new(self)
    }
    ///Bit 2 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn ot2(&mut self) -> OT2_W<2> {
        OT2_W::new(self)
    }
    ///Bit 3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn ot3(&mut self) -> OT3_W<3> {
        OT3_W::new(self)
    }
    ///Bit 4 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn ot4(&mut self) -> OT4_W<4> {
        OT4_W::new(self)
    }
    ///Bit 5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn ot5(&mut self) -> OT5_W<5> {
        OT5_W::new(self)
    }
    ///Bit 6 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn ot6(&mut self) -> OT6_W<6> {
        OT6_W::new(self)
    }
    ///Bit 13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn ot13(&mut self) -> OT13_W<13> {
        OT13_W::new(self)
    }
    ///Bit 14 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn ot14(&mut self) -> OT14_W<14> {
        OT14_W::new(self)
    }
    ///Bit 15 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn ot15(&mut self) -> OT15_W<15> {
        OT15_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO port output type register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otyper](index.html) module
pub struct OTYPER_SPEC;
impl crate::RegisterSpec for OTYPER_SPEC {
    type Ux = u32;
}
///`read()` method returns [otyper::R](R) reader structure
impl crate::Readable for OTYPER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [otyper::W](W) writer structure
impl crate::Writable for OTYPER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OTYPER to value 0
impl crate::Resettable for OTYPER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
