///Register `PUCRA` reader
pub struct R(crate::R<PUCRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCRA_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PUCRA` writer
pub struct W(crate::W<PUCRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCRA_SPEC>;
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
impl From<crate::W<PUCRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCRA_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PU0` reader - Port A pull-up bit y (y=0..15)
pub type PU0_R = crate::BitReader<PU0_A>;
///Port A pull-up bit y (y=0..15)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PU0_A {
    ///0: Pull-Up on Pxx is disabled
    Disabled = 0,
    ///1: Pull-Up on Pxx is enabled
    Enabled = 1,
}
impl From<PU0_A> for bool {
    #[inline(always)]
    fn from(variant: PU0_A) -> Self {
        variant as u8 != 0
    }
}
impl PU0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PU0_A {
        match self.bits {
            false => PU0_A::Disabled,
            true => PU0_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PU0_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PU0_A::Enabled
    }
}
///Field `PU0` writer - Port A pull-up bit y (y=0..15)
pub type PU0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRA_SPEC, PU0_A, O>;
impl<'a, const O: u8> PU0_W<'a, O> {
    ///Pull-Up on Pxx is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PU0_A::Disabled)
    }
    ///Pull-Up on Pxx is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PU0_A::Enabled)
    }
}
///Field `PU1` reader - Port A pull-up bit y (y=0..15)
pub use PU0_R as PU1_R;
///Field `PU2` reader - Port A pull-up bit y (y=0..15)
pub use PU0_R as PU2_R;
///Field `PU3` reader - Port A pull-up bit y (y=0..15)
pub use PU0_R as PU3_R;
///Field `PU4` reader - Port A pull-up bit y (y=0..15)
pub use PU0_R as PU4_R;
///Field `PU5` reader - Port A pull-up bit y (y=0..15)
pub use PU0_R as PU5_R;
///Field `PU6` reader - Port A pull-up bit y (y=0..15)
pub use PU0_R as PU6_R;
///Field `PU7` reader - Port A pull-up bit y (y=0..15)
pub use PU0_R as PU7_R;
///Field `PU8` reader - Port A pull-up bit y (y=0..15)
pub use PU0_R as PU8_R;
///Field `PU9` reader - Port A pull-up bit y (y=0..15)
pub use PU0_R as PU9_R;
///Field `PU10` reader - Port A pull-up bit y (y=0..15)
pub use PU0_R as PU10_R;
///Field `PU11` reader - Port A pull-up bit y (y=0..15)
pub use PU0_R as PU11_R;
///Field `PU12` reader - Port A pull-up bit y (y=0..15)
pub use PU0_R as PU12_R;
///Field `PU13` reader - Port A pull-up bit y (y=0..15)
pub use PU0_R as PU13_R;
///Field `PU15` reader - Port A pull-up bit y (y=0..15)
pub use PU0_R as PU15_R;
///Field `PU1` writer - Port A pull-up bit y (y=0..15)
pub use PU0_W as PU1_W;
///Field `PU2` writer - Port A pull-up bit y (y=0..15)
pub use PU0_W as PU2_W;
///Field `PU3` writer - Port A pull-up bit y (y=0..15)
pub use PU0_W as PU3_W;
///Field `PU4` writer - Port A pull-up bit y (y=0..15)
pub use PU0_W as PU4_W;
///Field `PU5` writer - Port A pull-up bit y (y=0..15)
pub use PU0_W as PU5_W;
///Field `PU6` writer - Port A pull-up bit y (y=0..15)
pub use PU0_W as PU6_W;
///Field `PU7` writer - Port A pull-up bit y (y=0..15)
pub use PU0_W as PU7_W;
///Field `PU8` writer - Port A pull-up bit y (y=0..15)
pub use PU0_W as PU8_W;
///Field `PU9` writer - Port A pull-up bit y (y=0..15)
pub use PU0_W as PU9_W;
///Field `PU10` writer - Port A pull-up bit y (y=0..15)
pub use PU0_W as PU10_W;
///Field `PU11` writer - Port A pull-up bit y (y=0..15)
pub use PU0_W as PU11_W;
///Field `PU12` writer - Port A pull-up bit y (y=0..15)
pub use PU0_W as PU12_W;
///Field `PU13` writer - Port A pull-up bit y (y=0..15)
pub use PU0_W as PU13_W;
///Field `PU15` writer - Port A pull-up bit y (y=0..15)
pub use PU0_W as PU15_W;
impl R {
    ///Bit 0 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu4(&self) -> PU4_R {
        PU4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu5(&self) -> PU5_R {
        PU5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu6(&self) -> PU6_R {
        PU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu7(&self) -> PU7_R {
        PU7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu8(&self) -> PU8_R {
        PU8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu9(&self) -> PU9_R {
        PU9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu10(&self) -> PU10_R {
        PU10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu11(&self) -> PU11_R {
        PU11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu12(&self) -> PU12_R {
        PU12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu13(&self) -> PU13_R {
        PU13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu15(&self) -> PU15_R {
        PU15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pu0(&mut self) -> PU0_W<0> {
        PU0_W::new(self)
    }
    ///Bit 1 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pu1(&mut self) -> PU1_W<1> {
        PU1_W::new(self)
    }
    ///Bit 2 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pu2(&mut self) -> PU2_W<2> {
        PU2_W::new(self)
    }
    ///Bit 3 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pu3(&mut self) -> PU3_W<3> {
        PU3_W::new(self)
    }
    ///Bit 4 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pu4(&mut self) -> PU4_W<4> {
        PU4_W::new(self)
    }
    ///Bit 5 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pu5(&mut self) -> PU5_W<5> {
        PU5_W::new(self)
    }
    ///Bit 6 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pu6(&mut self) -> PU6_W<6> {
        PU6_W::new(self)
    }
    ///Bit 7 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pu7(&mut self) -> PU7_W<7> {
        PU7_W::new(self)
    }
    ///Bit 8 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pu8(&mut self) -> PU8_W<8> {
        PU8_W::new(self)
    }
    ///Bit 9 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pu9(&mut self) -> PU9_W<9> {
        PU9_W::new(self)
    }
    ///Bit 10 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pu10(&mut self) -> PU10_W<10> {
        PU10_W::new(self)
    }
    ///Bit 11 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pu11(&mut self) -> PU11_W<11> {
        PU11_W::new(self)
    }
    ///Bit 12 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pu12(&mut self) -> PU12_W<12> {
        PU12_W::new(self)
    }
    ///Bit 13 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pu13(&mut self) -> PU13_W<13> {
        PU13_W::new(self)
    }
    ///Bit 15 - Port A pull-up bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pu15(&mut self) -> PU15_W<15> {
        PU15_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Power Port A pull-up control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pucra](index.html) module
pub struct PUCRA_SPEC;
impl crate::RegisterSpec for PUCRA_SPEC {
    type Ux = u32;
}
///`read()` method returns [pucra::R](R) reader structure
impl crate::Readable for PUCRA_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pucra::W](W) writer structure
impl crate::Writable for PUCRA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PUCRA to value 0
impl crate::Resettable for PUCRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
