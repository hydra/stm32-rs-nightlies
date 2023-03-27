///Register `PR1` reader
pub struct R(crate::R<PR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PR1` writer
pub struct W(crate::W<PR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR1_SPEC>;
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
impl From<crate::W<PR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PIF0` reader - Configurable event inputs Pending bit
pub type PIF0_R = crate::BitReader<PIF0R_A>;
///Configurable event inputs Pending bit
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
///Configurable event inputs Pending bit
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
///Field `PIF0` writer - Configurable event inputs Pending bit
pub type PIF0_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, PR1_SPEC, PIF0W_AW, O>;
impl<'a, const O: u8> PIF0_W<'a, O> {
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PIF0W_AW::Clear)
    }
}
///Field `PIF1` reader - Configurable event inputs Pending bit
pub use PIF0_R as PIF1_R;
///Field `PIF2` reader - Configurable event inputs Pending bit
pub use PIF0_R as PIF2_R;
///Field `PIF3` reader - Configurable event inputs Pending bit
pub use PIF0_R as PIF3_R;
///Field `PIF4` reader - Configurable event inputs Pending bit
pub use PIF0_R as PIF4_R;
///Field `PIF5` reader - Configurable event inputs Pending bit
pub use PIF0_R as PIF5_R;
///Field `PIF6` reader - Configurable event inputs Pending bit
pub use PIF0_R as PIF6_R;
///Field `PIF7` reader - Configurable event inputs Pending bit
pub use PIF0_R as PIF7_R;
///Field `PIF8` reader - Configurable event inputs Pending bit
pub use PIF0_R as PIF8_R;
///Field `PIF9` reader - Configurable event inputs Pending bit
pub use PIF0_R as PIF9_R;
///Field `PIF10` reader - Configurable event inputs Pending bit
pub use PIF0_R as PIF10_R;
///Field `PIF11` reader - Configurable event inputs Pending bit
pub use PIF0_R as PIF11_R;
///Field `PIF12` reader - Configurable event inputs Pending bit
pub use PIF0_R as PIF12_R;
///Field `PIF13` reader - Configurable event inputs Pending bit
pub use PIF0_R as PIF13_R;
///Field `PIF14` reader - Configurable event inputs Pending bit
pub use PIF0_R as PIF14_R;
///Field `PIF15` reader - Configurable event inputs Pending bit
pub use PIF0_R as PIF15_R;
///Field `PIF16` reader - Configurable event inputs Pending bit
pub use PIF0_R as PIF16_R;
///Field `PIF21` reader - Configurable event inputs Pending bit
pub use PIF0_R as PIF21_R;
///Field `PIF22` reader - Configurable event inputs Pending bit
pub use PIF0_R as PIF22_R;
///Field `PIF1` writer - Configurable event inputs Pending bit
pub use PIF0_W as PIF1_W;
///Field `PIF2` writer - Configurable event inputs Pending bit
pub use PIF0_W as PIF2_W;
///Field `PIF3` writer - Configurable event inputs Pending bit
pub use PIF0_W as PIF3_W;
///Field `PIF4` writer - Configurable event inputs Pending bit
pub use PIF0_W as PIF4_W;
///Field `PIF5` writer - Configurable event inputs Pending bit
pub use PIF0_W as PIF5_W;
///Field `PIF6` writer - Configurable event inputs Pending bit
pub use PIF0_W as PIF6_W;
///Field `PIF7` writer - Configurable event inputs Pending bit
pub use PIF0_W as PIF7_W;
///Field `PIF8` writer - Configurable event inputs Pending bit
pub use PIF0_W as PIF8_W;
///Field `PIF9` writer - Configurable event inputs Pending bit
pub use PIF0_W as PIF9_W;
///Field `PIF10` writer - Configurable event inputs Pending bit
pub use PIF0_W as PIF10_W;
///Field `PIF11` writer - Configurable event inputs Pending bit
pub use PIF0_W as PIF11_W;
///Field `PIF12` writer - Configurable event inputs Pending bit
pub use PIF0_W as PIF12_W;
///Field `PIF13` writer - Configurable event inputs Pending bit
pub use PIF0_W as PIF13_W;
///Field `PIF14` writer - Configurable event inputs Pending bit
pub use PIF0_W as PIF14_W;
///Field `PIF15` writer - Configurable event inputs Pending bit
pub use PIF0_W as PIF15_W;
///Field `PIF16` writer - Configurable event inputs Pending bit
pub use PIF0_W as PIF16_W;
///Field `PIF21` writer - Configurable event inputs Pending bit
pub use PIF0_W as PIF21_W;
///Field `PIF22` writer - Configurable event inputs Pending bit
pub use PIF0_W as PIF22_W;
impl R {
    ///Bit 0 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif0(&self) -> PIF0_R {
        PIF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif1(&self) -> PIF1_R {
        PIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif2(&self) -> PIF2_R {
        PIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif3(&self) -> PIF3_R {
        PIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif4(&self) -> PIF4_R {
        PIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif5(&self) -> PIF5_R {
        PIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif6(&self) -> PIF6_R {
        PIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif7(&self) -> PIF7_R {
        PIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif8(&self) -> PIF8_R {
        PIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif9(&self) -> PIF9_R {
        PIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif10(&self) -> PIF10_R {
        PIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif11(&self) -> PIF11_R {
        PIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif12(&self) -> PIF12_R {
        PIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif13(&self) -> PIF13_R {
        PIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif14(&self) -> PIF14_R {
        PIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif15(&self) -> PIF15_R {
        PIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif16(&self) -> PIF16_R {
        PIF16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 21 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif21(&self) -> PIF21_R {
        PIF21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Configurable event inputs Pending bit
    #[inline(always)]
    pub fn pif22(&self) -> PIF22_R {
        PIF22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Configurable event inputs Pending bit
    #[inline(always)]
    #[must_use]
    pub fn pif0(&mut self) -> PIF0_W<0> {
        PIF0_W::new(self)
    }
    ///Bit 1 - Configurable event inputs Pending bit
    #[inline(always)]
    #[must_use]
    pub fn pif1(&mut self) -> PIF1_W<1> {
        PIF1_W::new(self)
    }
    ///Bit 2 - Configurable event inputs Pending bit
    #[inline(always)]
    #[must_use]
    pub fn pif2(&mut self) -> PIF2_W<2> {
        PIF2_W::new(self)
    }
    ///Bit 3 - Configurable event inputs Pending bit
    #[inline(always)]
    #[must_use]
    pub fn pif3(&mut self) -> PIF3_W<3> {
        PIF3_W::new(self)
    }
    ///Bit 4 - Configurable event inputs Pending bit
    #[inline(always)]
    #[must_use]
    pub fn pif4(&mut self) -> PIF4_W<4> {
        PIF4_W::new(self)
    }
    ///Bit 5 - Configurable event inputs Pending bit
    #[inline(always)]
    #[must_use]
    pub fn pif5(&mut self) -> PIF5_W<5> {
        PIF5_W::new(self)
    }
    ///Bit 6 - Configurable event inputs Pending bit
    #[inline(always)]
    #[must_use]
    pub fn pif6(&mut self) -> PIF6_W<6> {
        PIF6_W::new(self)
    }
    ///Bit 7 - Configurable event inputs Pending bit
    #[inline(always)]
    #[must_use]
    pub fn pif7(&mut self) -> PIF7_W<7> {
        PIF7_W::new(self)
    }
    ///Bit 8 - Configurable event inputs Pending bit
    #[inline(always)]
    #[must_use]
    pub fn pif8(&mut self) -> PIF8_W<8> {
        PIF8_W::new(self)
    }
    ///Bit 9 - Configurable event inputs Pending bit
    #[inline(always)]
    #[must_use]
    pub fn pif9(&mut self) -> PIF9_W<9> {
        PIF9_W::new(self)
    }
    ///Bit 10 - Configurable event inputs Pending bit
    #[inline(always)]
    #[must_use]
    pub fn pif10(&mut self) -> PIF10_W<10> {
        PIF10_W::new(self)
    }
    ///Bit 11 - Configurable event inputs Pending bit
    #[inline(always)]
    #[must_use]
    pub fn pif11(&mut self) -> PIF11_W<11> {
        PIF11_W::new(self)
    }
    ///Bit 12 - Configurable event inputs Pending bit
    #[inline(always)]
    #[must_use]
    pub fn pif12(&mut self) -> PIF12_W<12> {
        PIF12_W::new(self)
    }
    ///Bit 13 - Configurable event inputs Pending bit
    #[inline(always)]
    #[must_use]
    pub fn pif13(&mut self) -> PIF13_W<13> {
        PIF13_W::new(self)
    }
    ///Bit 14 - Configurable event inputs Pending bit
    #[inline(always)]
    #[must_use]
    pub fn pif14(&mut self) -> PIF14_W<14> {
        PIF14_W::new(self)
    }
    ///Bit 15 - Configurable event inputs Pending bit
    #[inline(always)]
    #[must_use]
    pub fn pif15(&mut self) -> PIF15_W<15> {
        PIF15_W::new(self)
    }
    ///Bit 16 - Configurable event inputs Pending bit
    #[inline(always)]
    #[must_use]
    pub fn pif16(&mut self) -> PIF16_W<16> {
        PIF16_W::new(self)
    }
    ///Bit 21 - Configurable event inputs Pending bit
    #[inline(always)]
    #[must_use]
    pub fn pif21(&mut self) -> PIF21_W<21> {
        PIF21_W::new(self)
    }
    ///Bit 22 - Configurable event inputs Pending bit
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
///EXTI pending register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pr1](index.html) module
pub struct PR1_SPEC;
impl crate::RegisterSpec for PR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [pr1::R](R) reader structure
impl crate::Readable for PR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pr1::W](W) writer structure
impl crate::Writable for PR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0061_ffff;
}
///`reset()` method sets PR1 to value 0
impl crate::Resettable for PR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
