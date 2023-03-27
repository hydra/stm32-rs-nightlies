///Register `PR` reader
pub struct R(crate::R<PR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PR` writer
pub struct W(crate::W<PR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR_SPEC>;
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
impl From<crate::W<PR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PIF0` reader - Pending bit 0
pub type PIF0_R = crate::BitReader<PIF0R_A>;
///Pending bit 0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIF0R_A {
    ///0: No trigger request occurred
    NotPending = 0,
    ///1: Selected trigger request occurred
    Pending = 1,
}
impl From<PIF0R_A> for bool {
    #[inline(always)]
    fn from(variant: PIF0R_A) -> Self {
        variant as u8 != 0
    }
}
impl PIF0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PIF0R_A {
        match self.bits {
            false => PIF0R_A::NotPending,
            true => PIF0R_A::Pending,
        }
    }
    ///Checks if the value of the field is `NotPending`
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == PIF0R_A::NotPending
    }
    ///Checks if the value of the field is `Pending`
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == PIF0R_A::Pending
    }
}
///Pending bit 0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIF0W_AW {
    ///1: Clears pending bit
    Clear = 1,
}
impl From<PIF0W_AW> for bool {
    #[inline(always)]
    fn from(variant: PIF0W_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `PIF0` writer - Pending bit 0
pub type PIF0_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, PR_SPEC, PIF0W_AW, O>;
impl<'a, const O: u8> PIF0_W<'a, O> {
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF0W_AW::Clear)
    }
}
///Field `PIF1` reader - Pending bit 1
pub use PIF0_R as PIF1_R;
///Field `PIF2` reader - Pending bit 2
pub use PIF0_R as PIF2_R;
///Field `PIF3` reader - Pending bit 3
pub use PIF0_R as PIF3_R;
///Field `PIF4` reader - Pending bit 4
pub use PIF0_R as PIF4_R;
///Field `PIF5` reader - Pending bit 5
pub use PIF0_R as PIF5_R;
///Field `PIF6` reader - Pending bit 6
pub use PIF0_R as PIF6_R;
///Field `PIF7` reader - Pending bit 7
pub use PIF0_R as PIF7_R;
///Field `PIF8` reader - Pending bit 8
pub use PIF0_R as PIF8_R;
///Field `PIF9` reader - Pending bit 9
pub use PIF0_R as PIF9_R;
///Field `PIF10` reader - Pending bit 10
pub use PIF0_R as PIF10_R;
///Field `PIF11` reader - Pending bit 11
pub use PIF0_R as PIF11_R;
///Field `PIF12` reader - Pending bit 12
pub use PIF0_R as PIF12_R;
///Field `PIF13` reader - Pending bit 13
pub use PIF0_R as PIF13_R;
///Field `PIF14` reader - Pending bit 14
pub use PIF0_R as PIF14_R;
///Field `PIF15` reader - Pending bit 15
pub use PIF0_R as PIF15_R;
///Field `PIF16` reader - Pending bit 16
pub use PIF0_R as PIF16_R;
///Field `PIF17` reader - Pending bit 17
pub use PIF0_R as PIF17_R;
///Field `PIF19` reader - Pending bit 19
pub use PIF0_R as PIF19_R;
///Field `PIF20` reader - Pending bit 20
pub use PIF0_R as PIF20_R;
///Field `PIF21` reader - Pending bit 21
pub use PIF0_R as PIF21_R;
///Field `PIF22` reader - Pending bit 22
pub use PIF0_R as PIF22_R;
///Field `PIF1` writer - Pending bit 1
pub use PIF0_W as PIF1_W;
///Field `PIF2` writer - Pending bit 2
pub use PIF0_W as PIF2_W;
///Field `PIF3` writer - Pending bit 3
pub use PIF0_W as PIF3_W;
///Field `PIF4` writer - Pending bit 4
pub use PIF0_W as PIF4_W;
///Field `PIF5` writer - Pending bit 5
pub use PIF0_W as PIF5_W;
///Field `PIF6` writer - Pending bit 6
pub use PIF0_W as PIF6_W;
///Field `PIF7` writer - Pending bit 7
pub use PIF0_W as PIF7_W;
///Field `PIF8` writer - Pending bit 8
pub use PIF0_W as PIF8_W;
///Field `PIF9` writer - Pending bit 9
pub use PIF0_W as PIF9_W;
///Field `PIF10` writer - Pending bit 10
pub use PIF0_W as PIF10_W;
///Field `PIF11` writer - Pending bit 11
pub use PIF0_W as PIF11_W;
///Field `PIF12` writer - Pending bit 12
pub use PIF0_W as PIF12_W;
///Field `PIF13` writer - Pending bit 13
pub use PIF0_W as PIF13_W;
///Field `PIF14` writer - Pending bit 14
pub use PIF0_W as PIF14_W;
///Field `PIF15` writer - Pending bit 15
pub use PIF0_W as PIF15_W;
///Field `PIF16` writer - Pending bit 16
pub use PIF0_W as PIF16_W;
///Field `PIF17` writer - Pending bit 17
pub use PIF0_W as PIF17_W;
///Field `PIF19` writer - Pending bit 19
pub use PIF0_W as PIF19_W;
///Field `PIF20` writer - Pending bit 20
pub use PIF0_W as PIF20_W;
///Field `PIF21` writer - Pending bit 21
pub use PIF0_W as PIF21_W;
///Field `PIF22` writer - Pending bit 22
pub use PIF0_W as PIF22_W;
impl R {
    ///Bit 0 - Pending bit 0
    #[inline(always)]
    pub fn pif0(&self) -> PIF0_R {
        PIF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Pending bit 1
    #[inline(always)]
    pub fn pif1(&self) -> PIF1_R {
        PIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Pending bit 2
    #[inline(always)]
    pub fn pif2(&self) -> PIF2_R {
        PIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Pending bit 3
    #[inline(always)]
    pub fn pif3(&self) -> PIF3_R {
        PIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Pending bit 4
    #[inline(always)]
    pub fn pif4(&self) -> PIF4_R {
        PIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Pending bit 5
    #[inline(always)]
    pub fn pif5(&self) -> PIF5_R {
        PIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Pending bit 6
    #[inline(always)]
    pub fn pif6(&self) -> PIF6_R {
        PIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Pending bit 7
    #[inline(always)]
    pub fn pif7(&self) -> PIF7_R {
        PIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Pending bit 8
    #[inline(always)]
    pub fn pif8(&self) -> PIF8_R {
        PIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Pending bit 9
    #[inline(always)]
    pub fn pif9(&self) -> PIF9_R {
        PIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Pending bit 10
    #[inline(always)]
    pub fn pif10(&self) -> PIF10_R {
        PIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Pending bit 11
    #[inline(always)]
    pub fn pif11(&self) -> PIF11_R {
        PIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Pending bit 12
    #[inline(always)]
    pub fn pif12(&self) -> PIF12_R {
        PIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Pending bit 13
    #[inline(always)]
    pub fn pif13(&self) -> PIF13_R {
        PIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Pending bit 14
    #[inline(always)]
    pub fn pif14(&self) -> PIF14_R {
        PIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Pending bit 15
    #[inline(always)]
    pub fn pif15(&self) -> PIF15_R {
        PIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Pending bit 16
    #[inline(always)]
    pub fn pif16(&self) -> PIF16_R {
        PIF16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Pending bit 17
    #[inline(always)]
    pub fn pif17(&self) -> PIF17_R {
        PIF17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - Pending bit 19
    #[inline(always)]
    pub fn pif19(&self) -> PIF19_R {
        PIF19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Pending bit 20
    #[inline(always)]
    pub fn pif20(&self) -> PIF20_R {
        PIF20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Pending bit 21
    #[inline(always)]
    pub fn pif21(&self) -> PIF21_R {
        PIF21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Pending bit 22
    #[inline(always)]
    pub fn pif22(&self) -> PIF22_R {
        PIF22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Pending bit 0
    #[inline(always)]
    #[must_use]
    pub fn pif0(&mut self) -> PIF0_W<0> {
        PIF0_W::new(self)
    }
    ///Bit 1 - Pending bit 1
    #[inline(always)]
    #[must_use]
    pub fn pif1(&mut self) -> PIF1_W<1> {
        PIF1_W::new(self)
    }
    ///Bit 2 - Pending bit 2
    #[inline(always)]
    #[must_use]
    pub fn pif2(&mut self) -> PIF2_W<2> {
        PIF2_W::new(self)
    }
    ///Bit 3 - Pending bit 3
    #[inline(always)]
    #[must_use]
    pub fn pif3(&mut self) -> PIF3_W<3> {
        PIF3_W::new(self)
    }
    ///Bit 4 - Pending bit 4
    #[inline(always)]
    #[must_use]
    pub fn pif4(&mut self) -> PIF4_W<4> {
        PIF4_W::new(self)
    }
    ///Bit 5 - Pending bit 5
    #[inline(always)]
    #[must_use]
    pub fn pif5(&mut self) -> PIF5_W<5> {
        PIF5_W::new(self)
    }
    ///Bit 6 - Pending bit 6
    #[inline(always)]
    #[must_use]
    pub fn pif6(&mut self) -> PIF6_W<6> {
        PIF6_W::new(self)
    }
    ///Bit 7 - Pending bit 7
    #[inline(always)]
    #[must_use]
    pub fn pif7(&mut self) -> PIF7_W<7> {
        PIF7_W::new(self)
    }
    ///Bit 8 - Pending bit 8
    #[inline(always)]
    #[must_use]
    pub fn pif8(&mut self) -> PIF8_W<8> {
        PIF8_W::new(self)
    }
    ///Bit 9 - Pending bit 9
    #[inline(always)]
    #[must_use]
    pub fn pif9(&mut self) -> PIF9_W<9> {
        PIF9_W::new(self)
    }
    ///Bit 10 - Pending bit 10
    #[inline(always)]
    #[must_use]
    pub fn pif10(&mut self) -> PIF10_W<10> {
        PIF10_W::new(self)
    }
    ///Bit 11 - Pending bit 11
    #[inline(always)]
    #[must_use]
    pub fn pif11(&mut self) -> PIF11_W<11> {
        PIF11_W::new(self)
    }
    ///Bit 12 - Pending bit 12
    #[inline(always)]
    #[must_use]
    pub fn pif12(&mut self) -> PIF12_W<12> {
        PIF12_W::new(self)
    }
    ///Bit 13 - Pending bit 13
    #[inline(always)]
    #[must_use]
    pub fn pif13(&mut self) -> PIF13_W<13> {
        PIF13_W::new(self)
    }
    ///Bit 14 - Pending bit 14
    #[inline(always)]
    #[must_use]
    pub fn pif14(&mut self) -> PIF14_W<14> {
        PIF14_W::new(self)
    }
    ///Bit 15 - Pending bit 15
    #[inline(always)]
    #[must_use]
    pub fn pif15(&mut self) -> PIF15_W<15> {
        PIF15_W::new(self)
    }
    ///Bit 16 - Pending bit 16
    #[inline(always)]
    #[must_use]
    pub fn pif16(&mut self) -> PIF16_W<16> {
        PIF16_W::new(self)
    }
    ///Bit 17 - Pending bit 17
    #[inline(always)]
    #[must_use]
    pub fn pif17(&mut self) -> PIF17_W<17> {
        PIF17_W::new(self)
    }
    ///Bit 19 - Pending bit 19
    #[inline(always)]
    #[must_use]
    pub fn pif19(&mut self) -> PIF19_W<19> {
        PIF19_W::new(self)
    }
    ///Bit 20 - Pending bit 20
    #[inline(always)]
    #[must_use]
    pub fn pif20(&mut self) -> PIF20_W<20> {
        PIF20_W::new(self)
    }
    ///Bit 21 - Pending bit 21
    #[inline(always)]
    #[must_use]
    pub fn pif21(&mut self) -> PIF21_W<21> {
        PIF21_W::new(self)
    }
    ///Bit 22 - Pending bit 22
    #[inline(always)]
    #[must_use]
    pub fn pif22(&mut self) -> PIF22_W<22> {
        PIF22_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Pending register (EXTI_PR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pr](index.html) module
pub struct PR_SPEC;
impl crate::RegisterSpec for PR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pr::R](R) reader structure
impl crate::Readable for PR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pr::W](W) writer structure
impl crate::Writable for PR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x007b_ffff;
}
///`reset()` method sets PR to value 0
impl crate::Resettable for PR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
