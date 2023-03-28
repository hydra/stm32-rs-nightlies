///Register `C2IMR1` reader
pub struct R(crate::R<C2IMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2IMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2IMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2IMR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C2IMR1` writer
pub struct W(crate::W<C2IMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2IMR1_SPEC>;
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
impl From<crate::W<C2IMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2IMR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MR0` reader - CPU2 interrupt mask on Configurable Event input
pub type MR0_R = crate::BitReader<MR0_A>;
///CPU2 interrupt mask on Configurable Event input
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
///Field `MR0` writer - CPU2 interrupt mask on Configurable Event input
pub type MR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR1_SPEC, MR0_A, O>;
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
///Field `MR1` reader - CPU2 interrupt mask on Configurable Event input
pub use MR0_R as MR1_R;
///Field `MR2` reader - CPU2 interrupt mask on Configurable Event input
pub use MR0_R as MR2_R;
///Field `MR3` reader - CPU2 interrupt mask on Configurable Event input
pub use MR0_R as MR3_R;
///Field `MR4` reader - CPU2 interrupt mask on Configurable Event input
pub use MR0_R as MR4_R;
///Field `MR5` reader - CPU2 interrupt mask on Configurable Event input
pub use MR0_R as MR5_R;
///Field `MR6` reader - CPU2 interrupt mask on Configurable Event input
pub use MR0_R as MR6_R;
///Field `MR7` reader - CPU2 interrupt mask on Configurable Event input
pub use MR0_R as MR7_R;
///Field `MR8` reader - CPU2 interrupt mask on Configurable Event input
pub use MR0_R as MR8_R;
///Field `MR9` reader - CPU2 interrupt mask on Configurable Event input
pub use MR0_R as MR9_R;
///Field `MR10` reader - CPU2 interrupt mask on Configurable Event input
pub use MR0_R as MR10_R;
///Field `MR11` reader - CPU2 interrupt mask on Configurable Event input
pub use MR0_R as MR11_R;
///Field `MR12` reader - CPU2 interrupt mask on Configurable Event input
pub use MR0_R as MR12_R;
///Field `MR13` reader - CPU2 interrupt mask on Configurable Event input
pub use MR0_R as MR13_R;
///Field `MR14` reader - CPU2 interrupt mask on Configurable Event input
pub use MR0_R as MR14_R;
///Field `MR15` reader - CPU2 interrupt mask on Configurable Event input
pub use MR0_R as MR15_R;
///Field `MR16` reader - CPU2 interrupt mask on Configurable Event input
pub use MR0_R as MR16_R;
///Field `MR17` reader - CPU2 interrupt mask on Configurable Event input
pub use MR0_R as MR17_R;
///Field `MR18` reader - CPU2 interrupt mask on Configurable Event input
pub use MR0_R as MR18_R;
///Field `MR19` reader - CPU2 interrupt mask on Configurable Event input
pub use MR0_R as MR19_R;
///Field `MR20` reader - CPU2 interrupt mask on Configurable Event input
pub use MR0_R as MR20_R;
///Field `MR21` reader - CPU2 interrupt mask on Configurable Event input
pub use MR0_R as MR21_R;
///Field `MR22` reader - CPU2 interrupt mask on Direct Event input
pub use MR0_R as MR22_R;
///Field `MR23` reader - CPU2 interrupt mask on Direct Event input
pub use MR0_R as MR23_R;
///Field `MR24` reader - CPU2 interrupt mask on Direct Event input
pub use MR0_R as MR24_R;
///Field `MR25` reader - CPU2 interrupt mask on Direct Event input
pub use MR0_R as MR25_R;
///Field `MR26` reader - CPU2 interrupt mask on Direct Event input
pub use MR0_R as MR26_R;
///Field `MR27` reader - CPU2 interrupt mask on Direct Event input
pub use MR0_R as MR27_R;
///Field `MR28` reader - CPU2 interrupt mask on Direct Event input
pub use MR0_R as MR28_R;
///Field `MR29` reader - CPU2 interrupt mask on Direct Event input
pub use MR0_R as MR29_R;
///Field `MR30` reader - CPU2 interrupt mask on Direct Event input
pub use MR0_R as MR30_R;
///Field `MR31` reader - CPU2 interrupt mask on Direct Event input
pub use MR0_R as MR31_R;
///Field `MR1` writer - CPU2 interrupt mask on Configurable Event input
pub use MR0_W as MR1_W;
///Field `MR2` writer - CPU2 interrupt mask on Configurable Event input
pub use MR0_W as MR2_W;
///Field `MR3` writer - CPU2 interrupt mask on Configurable Event input
pub use MR0_W as MR3_W;
///Field `MR4` writer - CPU2 interrupt mask on Configurable Event input
pub use MR0_W as MR4_W;
///Field `MR5` writer - CPU2 interrupt mask on Configurable Event input
pub use MR0_W as MR5_W;
///Field `MR6` writer - CPU2 interrupt mask on Configurable Event input
pub use MR0_W as MR6_W;
///Field `MR7` writer - CPU2 interrupt mask on Configurable Event input
pub use MR0_W as MR7_W;
///Field `MR8` writer - CPU2 interrupt mask on Configurable Event input
pub use MR0_W as MR8_W;
///Field `MR9` writer - CPU2 interrupt mask on Configurable Event input
pub use MR0_W as MR9_W;
///Field `MR10` writer - CPU2 interrupt mask on Configurable Event input
pub use MR0_W as MR10_W;
///Field `MR11` writer - CPU2 interrupt mask on Configurable Event input
pub use MR0_W as MR11_W;
///Field `MR12` writer - CPU2 interrupt mask on Configurable Event input
pub use MR0_W as MR12_W;
///Field `MR13` writer - CPU2 interrupt mask on Configurable Event input
pub use MR0_W as MR13_W;
///Field `MR14` writer - CPU2 interrupt mask on Configurable Event input
pub use MR0_W as MR14_W;
///Field `MR15` writer - CPU2 interrupt mask on Configurable Event input
pub use MR0_W as MR15_W;
///Field `MR16` writer - CPU2 interrupt mask on Configurable Event input
pub use MR0_W as MR16_W;
///Field `MR17` writer - CPU2 interrupt mask on Configurable Event input
pub use MR0_W as MR17_W;
///Field `MR18` writer - CPU2 interrupt mask on Configurable Event input
pub use MR0_W as MR18_W;
///Field `MR19` writer - CPU2 interrupt mask on Configurable Event input
pub use MR0_W as MR19_W;
///Field `MR20` writer - CPU2 interrupt mask on Configurable Event input
pub use MR0_W as MR20_W;
///Field `MR21` writer - CPU2 interrupt mask on Configurable Event input
pub use MR0_W as MR21_W;
///Field `MR22` writer - CPU2 interrupt mask on Direct Event input
pub use MR0_W as MR22_W;
///Field `MR23` writer - CPU2 interrupt mask on Direct Event input
pub use MR0_W as MR23_W;
///Field `MR24` writer - CPU2 interrupt mask on Direct Event input
pub use MR0_W as MR24_W;
///Field `MR25` writer - CPU2 interrupt mask on Direct Event input
pub use MR0_W as MR25_W;
///Field `MR26` writer - CPU2 interrupt mask on Direct Event input
pub use MR0_W as MR26_W;
///Field `MR27` writer - CPU2 interrupt mask on Direct Event input
pub use MR0_W as MR27_W;
///Field `MR28` writer - CPU2 interrupt mask on Direct Event input
pub use MR0_W as MR28_W;
///Field `MR29` writer - CPU2 interrupt mask on Direct Event input
pub use MR0_W as MR29_W;
///Field `MR30` writer - CPU2 interrupt mask on Direct Event input
pub use MR0_W as MR30_W;
///Field `MR31` writer - CPU2 interrupt mask on Direct Event input
pub use MR0_W as MR31_W;
impl R {
    ///Bit 0 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    pub fn mr0(&self) -> MR0_R {
        MR0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    pub fn mr1(&self) -> MR1_R {
        MR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    pub fn mr2(&self) -> MR2_R {
        MR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    pub fn mr3(&self) -> MR3_R {
        MR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    pub fn mr4(&self) -> MR4_R {
        MR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    pub fn mr5(&self) -> MR5_R {
        MR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    pub fn mr6(&self) -> MR6_R {
        MR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    pub fn mr7(&self) -> MR7_R {
        MR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    pub fn mr8(&self) -> MR8_R {
        MR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    pub fn mr9(&self) -> MR9_R {
        MR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    pub fn mr10(&self) -> MR10_R {
        MR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    pub fn mr11(&self) -> MR11_R {
        MR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    pub fn mr12(&self) -> MR12_R {
        MR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    pub fn mr13(&self) -> MR13_R {
        MR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    pub fn mr14(&self) -> MR14_R {
        MR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    pub fn mr15(&self) -> MR15_R {
        MR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    pub fn mr16(&self) -> MR16_R {
        MR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    pub fn mr17(&self) -> MR17_R {
        MR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    pub fn mr18(&self) -> MR18_R {
        MR18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    pub fn mr19(&self) -> MR19_R {
        MR19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    pub fn mr20(&self) -> MR20_R {
        MR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    pub fn mr21(&self) -> MR21_R {
        MR21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - CPU2 interrupt mask on Direct Event input
    #[inline(always)]
    pub fn mr22(&self) -> MR22_R {
        MR22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - CPU2 interrupt mask on Direct Event input
    #[inline(always)]
    pub fn mr23(&self) -> MR23_R {
        MR23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CPU2 interrupt mask on Direct Event input
    #[inline(always)]
    pub fn mr24(&self) -> MR24_R {
        MR24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - CPU2 interrupt mask on Direct Event input
    #[inline(always)]
    pub fn mr25(&self) -> MR25_R {
        MR25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - CPU2 interrupt mask on Direct Event input
    #[inline(always)]
    pub fn mr26(&self) -> MR26_R {
        MR26_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - CPU2 interrupt mask on Direct Event input
    #[inline(always)]
    pub fn mr27(&self) -> MR27_R {
        MR27_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - CPU2 interrupt mask on Direct Event input
    #[inline(always)]
    pub fn mr28(&self) -> MR28_R {
        MR28_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - CPU2 interrupt mask on Direct Event input
    #[inline(always)]
    pub fn mr29(&self) -> MR29_R {
        MR29_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - CPU2 interrupt mask on Direct Event input
    #[inline(always)]
    pub fn mr30(&self) -> MR30_R {
        MR30_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - CPU2 interrupt mask on Direct Event input
    #[inline(always)]
    pub fn mr31(&self) -> MR31_R {
        MR31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr0(&mut self) -> MR0_W<0> {
        MR0_W::new(self)
    }
    ///Bit 1 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr1(&mut self) -> MR1_W<1> {
        MR1_W::new(self)
    }
    ///Bit 2 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr2(&mut self) -> MR2_W<2> {
        MR2_W::new(self)
    }
    ///Bit 3 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr3(&mut self) -> MR3_W<3> {
        MR3_W::new(self)
    }
    ///Bit 4 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr4(&mut self) -> MR4_W<4> {
        MR4_W::new(self)
    }
    ///Bit 5 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr5(&mut self) -> MR5_W<5> {
        MR5_W::new(self)
    }
    ///Bit 6 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr6(&mut self) -> MR6_W<6> {
        MR6_W::new(self)
    }
    ///Bit 7 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr7(&mut self) -> MR7_W<7> {
        MR7_W::new(self)
    }
    ///Bit 8 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr8(&mut self) -> MR8_W<8> {
        MR8_W::new(self)
    }
    ///Bit 9 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr9(&mut self) -> MR9_W<9> {
        MR9_W::new(self)
    }
    ///Bit 10 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr10(&mut self) -> MR10_W<10> {
        MR10_W::new(self)
    }
    ///Bit 11 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr11(&mut self) -> MR11_W<11> {
        MR11_W::new(self)
    }
    ///Bit 12 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr12(&mut self) -> MR12_W<12> {
        MR12_W::new(self)
    }
    ///Bit 13 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr13(&mut self) -> MR13_W<13> {
        MR13_W::new(self)
    }
    ///Bit 14 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr14(&mut self) -> MR14_W<14> {
        MR14_W::new(self)
    }
    ///Bit 15 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr15(&mut self) -> MR15_W<15> {
        MR15_W::new(self)
    }
    ///Bit 16 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr16(&mut self) -> MR16_W<16> {
        MR16_W::new(self)
    }
    ///Bit 17 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr17(&mut self) -> MR17_W<17> {
        MR17_W::new(self)
    }
    ///Bit 18 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr18(&mut self) -> MR18_W<18> {
        MR18_W::new(self)
    }
    ///Bit 19 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr19(&mut self) -> MR19_W<19> {
        MR19_W::new(self)
    }
    ///Bit 20 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr20(&mut self) -> MR20_W<20> {
        MR20_W::new(self)
    }
    ///Bit 21 - CPU2 interrupt mask on Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn mr21(&mut self) -> MR21_W<21> {
        MR21_W::new(self)
    }
    ///Bit 22 - CPU2 interrupt mask on Direct Event input
    #[inline(always)]
    #[must_use]
    pub fn mr22(&mut self) -> MR22_W<22> {
        MR22_W::new(self)
    }
    ///Bit 23 - CPU2 interrupt mask on Direct Event input
    #[inline(always)]
    #[must_use]
    pub fn mr23(&mut self) -> MR23_W<23> {
        MR23_W::new(self)
    }
    ///Bit 24 - CPU2 interrupt mask on Direct Event input
    #[inline(always)]
    #[must_use]
    pub fn mr24(&mut self) -> MR24_W<24> {
        MR24_W::new(self)
    }
    ///Bit 25 - CPU2 interrupt mask on Direct Event input
    #[inline(always)]
    #[must_use]
    pub fn mr25(&mut self) -> MR25_W<25> {
        MR25_W::new(self)
    }
    ///Bit 26 - CPU2 interrupt mask on Direct Event input
    #[inline(always)]
    #[must_use]
    pub fn mr26(&mut self) -> MR26_W<26> {
        MR26_W::new(self)
    }
    ///Bit 27 - CPU2 interrupt mask on Direct Event input
    #[inline(always)]
    #[must_use]
    pub fn mr27(&mut self) -> MR27_W<27> {
        MR27_W::new(self)
    }
    ///Bit 28 - CPU2 interrupt mask on Direct Event input
    #[inline(always)]
    #[must_use]
    pub fn mr28(&mut self) -> MR28_W<28> {
        MR28_W::new(self)
    }
    ///Bit 29 - CPU2 interrupt mask on Direct Event input
    #[inline(always)]
    #[must_use]
    pub fn mr29(&mut self) -> MR29_W<29> {
        MR29_W::new(self)
    }
    ///Bit 30 - CPU2 interrupt mask on Direct Event input
    #[inline(always)]
    #[must_use]
    pub fn mr30(&mut self) -> MR30_W<30> {
        MR30_W::new(self)
    }
    ///Bit 31 - CPU2 interrupt mask on Direct Event input
    #[inline(always)]
    #[must_use]
    pub fn mr31(&mut self) -> MR31_W<31> {
        MR31_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CPU2 EXTI interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2imr1](index.html) module
pub struct C2IMR1_SPEC;
impl crate::RegisterSpec for C2IMR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [c2imr1::R](R) reader structure
impl crate::Readable for C2IMR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c2imr1::W](W) writer structure
impl crate::Writable for C2IMR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C2IMR1 to value 0xffc0_0000
impl crate::Resettable for C2IMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffc0_0000;
}
