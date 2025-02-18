///Register `D3PMR1` reader
pub struct R(crate::R<D3PMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D3PMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D3PMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D3PMR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `D3PMR1` writer
pub struct W(crate::W<D3PMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D3PMR1_SPEC>;
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
impl From<crate::W<D3PMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D3PMR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MR0` reader - Rising trigger event configuration bit of Configurable Event input
pub type MR0_R = crate::BitReader<MR0_A>;
///Rising trigger event configuration bit of Configurable Event input
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR0_A {
    ///0: Interrupt request line is masked
    Masked = 0,
    ///1: Interrupt request line is unmasked
    Unmasked = 1,
}
impl From<MR0_A> for bool {
    #[inline(always)]
    fn from(variant: MR0_A) -> Self {
        variant as u8 != 0
    }
}
impl MR0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MR0_A {
        match self.bits {
            false => MR0_A::Masked,
            true => MR0_A::Unmasked,
        }
    }
    ///Checks if the value of the field is `Masked`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == MR0_A::Masked
    }
    ///Checks if the value of the field is `Unmasked`
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == MR0_A::Unmasked
    }
}
///Field `MR0` writer - Rising trigger event configuration bit of Configurable Event input
pub type MR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, D3PMR1_SPEC, MR0_A, O>;
impl<'a, const O: u8> MR0_W<'a, O> {
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR0_A::Masked)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR0_A::Unmasked)
    }
}
///Field `MR1` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR1_R;
///Field `MR2` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR2_R;
///Field `MR3` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR3_R;
///Field `MR4` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR4_R;
///Field `MR5` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR5_R;
///Field `MR6` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR6_R;
///Field `MR7` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR7_R;
///Field `MR8` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR8_R;
///Field `MR9` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR9_R;
///Field `MR10` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR10_R;
///Field `MR11` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR11_R;
///Field `MR12` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR12_R;
///Field `MR13` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR13_R;
///Field `MR14` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR14_R;
///Field `MR15` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR15_R;
///Field `MR19` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR19_R;
///Field `MR20` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR20_R;
///Field `MR21` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR21_R;
///Field `MR25` reader - Rising trigger event configuration bit of Configurable Event input
pub use MR0_R as MR25_R;
///Field `MR1` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR1_W;
///Field `MR2` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR2_W;
///Field `MR3` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR3_W;
///Field `MR4` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR4_W;
///Field `MR5` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR5_W;
///Field `MR6` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR6_W;
///Field `MR7` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR7_W;
///Field `MR8` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR8_W;
///Field `MR9` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR9_W;
///Field `MR10` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR10_W;
///Field `MR11` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR11_W;
///Field `MR12` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR12_W;
///Field `MR13` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR13_W;
///Field `MR14` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR14_W;
///Field `MR15` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR15_W;
///Field `MR19` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR19_W;
///Field `MR20` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR20_W;
///Field `MR21` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR21_W;
///Field `MR25` writer - Rising trigger event configuration bit of Configurable Event input
pub use MR0_W as MR25_W;
impl R {
    ///Bit 0 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr0(&self) -> MR0_R {
        MR0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr1(&self) -> MR1_R {
        MR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr2(&self) -> MR2_R {
        MR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr3(&self) -> MR3_R {
        MR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr4(&self) -> MR4_R {
        MR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr5(&self) -> MR5_R {
        MR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr6(&self) -> MR6_R {
        MR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr7(&self) -> MR7_R {
        MR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr8(&self) -> MR8_R {
        MR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr9(&self) -> MR9_R {
        MR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr10(&self) -> MR10_R {
        MR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr11(&self) -> MR11_R {
        MR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr12(&self) -> MR12_R {
        MR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr13(&self) -> MR13_R {
        MR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr14(&self) -> MR14_R {
        MR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr15(&self) -> MR15_R {
        MR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 19 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr19(&self) -> MR19_R {
        MR19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr20(&self) -> MR20_R {
        MR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr21(&self) -> MR21_R {
        MR21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 25 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn mr25(&self) -> MR25_R {
        MR25_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr0(&mut self) -> MR0_W<0> {
        MR0_W::new(self)
    }
    ///Bit 1 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr1(&mut self) -> MR1_W<1> {
        MR1_W::new(self)
    }
    ///Bit 2 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr2(&mut self) -> MR2_W<2> {
        MR2_W::new(self)
    }
    ///Bit 3 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr3(&mut self) -> MR3_W<3> {
        MR3_W::new(self)
    }
    ///Bit 4 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr4(&mut self) -> MR4_W<4> {
        MR4_W::new(self)
    }
    ///Bit 5 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr5(&mut self) -> MR5_W<5> {
        MR5_W::new(self)
    }
    ///Bit 6 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr6(&mut self) -> MR6_W<6> {
        MR6_W::new(self)
    }
    ///Bit 7 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr7(&mut self) -> MR7_W<7> {
        MR7_W::new(self)
    }
    ///Bit 8 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr8(&mut self) -> MR8_W<8> {
        MR8_W::new(self)
    }
    ///Bit 9 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr9(&mut self) -> MR9_W<9> {
        MR9_W::new(self)
    }
    ///Bit 10 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr10(&mut self) -> MR10_W<10> {
        MR10_W::new(self)
    }
    ///Bit 11 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr11(&mut self) -> MR11_W<11> {
        MR11_W::new(self)
    }
    ///Bit 12 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr12(&mut self) -> MR12_W<12> {
        MR12_W::new(self)
    }
    ///Bit 13 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr13(&mut self) -> MR13_W<13> {
        MR13_W::new(self)
    }
    ///Bit 14 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr14(&mut self) -> MR14_W<14> {
        MR14_W::new(self)
    }
    ///Bit 15 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr15(&mut self) -> MR15_W<15> {
        MR15_W::new(self)
    }
    ///Bit 19 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr19(&mut self) -> MR19_W<19> {
        MR19_W::new(self)
    }
    ///Bit 20 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr20(&mut self) -> MR20_W<20> {
        MR20_W::new(self)
    }
    ///Bit 21 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr21(&mut self) -> MR21_W<21> {
        MR21_W::new(self)
    }
    ///Bit 25 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr25(&mut self) -> MR25_W<25> {
        MR25_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI D3 pending mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [d3pmr1](index.html) module
pub struct D3PMR1_SPEC;
impl crate::RegisterSpec for D3PMR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [d3pmr1::R](R) reader structure
impl crate::Readable for D3PMR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [d3pmr1::W](W) writer structure
impl crate::Writable for D3PMR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets D3PMR1 to value 0
impl crate::Resettable for D3PMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
