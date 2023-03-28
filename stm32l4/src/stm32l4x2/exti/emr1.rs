///Register `EMR1` reader
pub struct R(crate::R<EMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EMR1` writer
pub struct W(crate::W<EMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMR1_SPEC>;
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
impl From<crate::W<EMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MR0` reader - Event Mask on line 0
pub type MR0_R = crate::BitReader<MR0_A>;
///Event Mask on line 0
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
///Field `MR0` writer - Event Mask on line 0
pub type MR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR1_SPEC, MR0_A, O>;
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
///Field `MR1` reader - Event Mask on line 1
pub use MR0_R as MR1_R;
///Field `MR2` reader - Event Mask on line 2
pub use MR0_R as MR2_R;
///Field `MR3` reader - Event Mask on line 3
pub use MR0_R as MR3_R;
///Field `MR4` reader - Event Mask on line 4
pub use MR0_R as MR4_R;
///Field `MR5` reader - Event Mask on line 5
pub use MR0_R as MR5_R;
///Field `MR6` reader - Event Mask on line 6
pub use MR0_R as MR6_R;
///Field `MR7` reader - Event Mask on line 7
pub use MR0_R as MR7_R;
///Field `MR8` reader - Event Mask on line 8
pub use MR0_R as MR8_R;
///Field `MR9` reader - Event Mask on line 9
pub use MR0_R as MR9_R;
///Field `MR10` reader - Event Mask on line 10
pub use MR0_R as MR10_R;
///Field `MR11` reader - Event Mask on line 11
pub use MR0_R as MR11_R;
///Field `MR12` reader - Event Mask on line 12
pub use MR0_R as MR12_R;
///Field `MR13` reader - Event Mask on line 13
pub use MR0_R as MR13_R;
///Field `MR14` reader - Event Mask on line 14
pub use MR0_R as MR14_R;
///Field `MR15` reader - Event Mask on line 15
pub use MR0_R as MR15_R;
///Field `MR16` reader - Event Mask on line 16
pub use MR0_R as MR16_R;
///Field `MR17` reader - Event Mask on line 17
pub use MR0_R as MR17_R;
///Field `MR18` reader - Event Mask on line 18
pub use MR0_R as MR18_R;
///Field `MR19` reader - Event Mask on line 19
pub use MR0_R as MR19_R;
///Field `MR20` reader - Event Mask on line 20
pub use MR0_R as MR20_R;
///Field `MR21` reader - Event Mask on line 21
pub use MR0_R as MR21_R;
///Field `MR22` reader - Event Mask on line 22
pub use MR0_R as MR22_R;
///Field `MR23` reader - Event Mask on line 23
pub use MR0_R as MR23_R;
///Field `MR24` reader - Event Mask on line 24
pub use MR0_R as MR24_R;
///Field `MR25` reader - Event Mask on line 25
pub use MR0_R as MR25_R;
///Field `MR26` reader - Event Mask on line 26
pub use MR0_R as MR26_R;
///Field `MR27` reader - Event Mask on line 27
pub use MR0_R as MR27_R;
///Field `MR28` reader - Event Mask on line 28
pub use MR0_R as MR28_R;
///Field `MR29` reader - Event Mask on line 29
pub use MR0_R as MR29_R;
///Field `MR30` reader - Event Mask on line 30
pub use MR0_R as MR30_R;
///Field `MR31` reader - Event Mask on line 31
pub use MR0_R as MR31_R;
///Field `MR1` writer - Event Mask on line 1
pub use MR0_W as MR1_W;
///Field `MR2` writer - Event Mask on line 2
pub use MR0_W as MR2_W;
///Field `MR3` writer - Event Mask on line 3
pub use MR0_W as MR3_W;
///Field `MR4` writer - Event Mask on line 4
pub use MR0_W as MR4_W;
///Field `MR5` writer - Event Mask on line 5
pub use MR0_W as MR5_W;
///Field `MR6` writer - Event Mask on line 6
pub use MR0_W as MR6_W;
///Field `MR7` writer - Event Mask on line 7
pub use MR0_W as MR7_W;
///Field `MR8` writer - Event Mask on line 8
pub use MR0_W as MR8_W;
///Field `MR9` writer - Event Mask on line 9
pub use MR0_W as MR9_W;
///Field `MR10` writer - Event Mask on line 10
pub use MR0_W as MR10_W;
///Field `MR11` writer - Event Mask on line 11
pub use MR0_W as MR11_W;
///Field `MR12` writer - Event Mask on line 12
pub use MR0_W as MR12_W;
///Field `MR13` writer - Event Mask on line 13
pub use MR0_W as MR13_W;
///Field `MR14` writer - Event Mask on line 14
pub use MR0_W as MR14_W;
///Field `MR15` writer - Event Mask on line 15
pub use MR0_W as MR15_W;
///Field `MR16` writer - Event Mask on line 16
pub use MR0_W as MR16_W;
///Field `MR17` writer - Event Mask on line 17
pub use MR0_W as MR17_W;
///Field `MR18` writer - Event Mask on line 18
pub use MR0_W as MR18_W;
///Field `MR19` writer - Event Mask on line 19
pub use MR0_W as MR19_W;
///Field `MR20` writer - Event Mask on line 20
pub use MR0_W as MR20_W;
///Field `MR21` writer - Event Mask on line 21
pub use MR0_W as MR21_W;
///Field `MR22` writer - Event Mask on line 22
pub use MR0_W as MR22_W;
///Field `MR23` writer - Event Mask on line 23
pub use MR0_W as MR23_W;
///Field `MR24` writer - Event Mask on line 24
pub use MR0_W as MR24_W;
///Field `MR25` writer - Event Mask on line 25
pub use MR0_W as MR25_W;
///Field `MR26` writer - Event Mask on line 26
pub use MR0_W as MR26_W;
///Field `MR27` writer - Event Mask on line 27
pub use MR0_W as MR27_W;
///Field `MR28` writer - Event Mask on line 28
pub use MR0_W as MR28_W;
///Field `MR29` writer - Event Mask on line 29
pub use MR0_W as MR29_W;
///Field `MR30` writer - Event Mask on line 30
pub use MR0_W as MR30_W;
///Field `MR31` writer - Event Mask on line 31
pub use MR0_W as MR31_W;
impl R {
    ///Bit 0 - Event Mask on line 0
    #[inline(always)]
    pub fn mr0(&self) -> MR0_R {
        MR0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Event Mask on line 1
    #[inline(always)]
    pub fn mr1(&self) -> MR1_R {
        MR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Event Mask on line 2
    #[inline(always)]
    pub fn mr2(&self) -> MR2_R {
        MR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Event Mask on line 3
    #[inline(always)]
    pub fn mr3(&self) -> MR3_R {
        MR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Event Mask on line 4
    #[inline(always)]
    pub fn mr4(&self) -> MR4_R {
        MR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Event Mask on line 5
    #[inline(always)]
    pub fn mr5(&self) -> MR5_R {
        MR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Event Mask on line 6
    #[inline(always)]
    pub fn mr6(&self) -> MR6_R {
        MR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Event Mask on line 7
    #[inline(always)]
    pub fn mr7(&self) -> MR7_R {
        MR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Event Mask on line 8
    #[inline(always)]
    pub fn mr8(&self) -> MR8_R {
        MR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Event Mask on line 9
    #[inline(always)]
    pub fn mr9(&self) -> MR9_R {
        MR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Event Mask on line 10
    #[inline(always)]
    pub fn mr10(&self) -> MR10_R {
        MR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Event Mask on line 11
    #[inline(always)]
    pub fn mr11(&self) -> MR11_R {
        MR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Event Mask on line 12
    #[inline(always)]
    pub fn mr12(&self) -> MR12_R {
        MR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Event Mask on line 13
    #[inline(always)]
    pub fn mr13(&self) -> MR13_R {
        MR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Event Mask on line 14
    #[inline(always)]
    pub fn mr14(&self) -> MR14_R {
        MR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Event Mask on line 15
    #[inline(always)]
    pub fn mr15(&self) -> MR15_R {
        MR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Event Mask on line 16
    #[inline(always)]
    pub fn mr16(&self) -> MR16_R {
        MR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Event Mask on line 17
    #[inline(always)]
    pub fn mr17(&self) -> MR17_R {
        MR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Event Mask on line 18
    #[inline(always)]
    pub fn mr18(&self) -> MR18_R {
        MR18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Event Mask on line 19
    #[inline(always)]
    pub fn mr19(&self) -> MR19_R {
        MR19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Event Mask on line 20
    #[inline(always)]
    pub fn mr20(&self) -> MR20_R {
        MR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Event Mask on line 21
    #[inline(always)]
    pub fn mr21(&self) -> MR21_R {
        MR21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Event Mask on line 22
    #[inline(always)]
    pub fn mr22(&self) -> MR22_R {
        MR22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Event Mask on line 23
    #[inline(always)]
    pub fn mr23(&self) -> MR23_R {
        MR23_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Event Mask on line 24
    #[inline(always)]
    pub fn mr24(&self) -> MR24_R {
        MR24_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Event Mask on line 25
    #[inline(always)]
    pub fn mr25(&self) -> MR25_R {
        MR25_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Event Mask on line 26
    #[inline(always)]
    pub fn mr26(&self) -> MR26_R {
        MR26_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Event Mask on line 27
    #[inline(always)]
    pub fn mr27(&self) -> MR27_R {
        MR27_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Event Mask on line 28
    #[inline(always)]
    pub fn mr28(&self) -> MR28_R {
        MR28_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Event Mask on line 29
    #[inline(always)]
    pub fn mr29(&self) -> MR29_R {
        MR29_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Event Mask on line 30
    #[inline(always)]
    pub fn mr30(&self) -> MR30_R {
        MR30_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Event Mask on line 31
    #[inline(always)]
    pub fn mr31(&self) -> MR31_R {
        MR31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Event Mask on line 0
    #[inline(always)]
    #[must_use]
    pub fn mr0(&mut self) -> MR0_W<0> {
        MR0_W::new(self)
    }
    ///Bit 1 - Event Mask on line 1
    #[inline(always)]
    #[must_use]
    pub fn mr1(&mut self) -> MR1_W<1> {
        MR1_W::new(self)
    }
    ///Bit 2 - Event Mask on line 2
    #[inline(always)]
    #[must_use]
    pub fn mr2(&mut self) -> MR2_W<2> {
        MR2_W::new(self)
    }
    ///Bit 3 - Event Mask on line 3
    #[inline(always)]
    #[must_use]
    pub fn mr3(&mut self) -> MR3_W<3> {
        MR3_W::new(self)
    }
    ///Bit 4 - Event Mask on line 4
    #[inline(always)]
    #[must_use]
    pub fn mr4(&mut self) -> MR4_W<4> {
        MR4_W::new(self)
    }
    ///Bit 5 - Event Mask on line 5
    #[inline(always)]
    #[must_use]
    pub fn mr5(&mut self) -> MR5_W<5> {
        MR5_W::new(self)
    }
    ///Bit 6 - Event Mask on line 6
    #[inline(always)]
    #[must_use]
    pub fn mr6(&mut self) -> MR6_W<6> {
        MR6_W::new(self)
    }
    ///Bit 7 - Event Mask on line 7
    #[inline(always)]
    #[must_use]
    pub fn mr7(&mut self) -> MR7_W<7> {
        MR7_W::new(self)
    }
    ///Bit 8 - Event Mask on line 8
    #[inline(always)]
    #[must_use]
    pub fn mr8(&mut self) -> MR8_W<8> {
        MR8_W::new(self)
    }
    ///Bit 9 - Event Mask on line 9
    #[inline(always)]
    #[must_use]
    pub fn mr9(&mut self) -> MR9_W<9> {
        MR9_W::new(self)
    }
    ///Bit 10 - Event Mask on line 10
    #[inline(always)]
    #[must_use]
    pub fn mr10(&mut self) -> MR10_W<10> {
        MR10_W::new(self)
    }
    ///Bit 11 - Event Mask on line 11
    #[inline(always)]
    #[must_use]
    pub fn mr11(&mut self) -> MR11_W<11> {
        MR11_W::new(self)
    }
    ///Bit 12 - Event Mask on line 12
    #[inline(always)]
    #[must_use]
    pub fn mr12(&mut self) -> MR12_W<12> {
        MR12_W::new(self)
    }
    ///Bit 13 - Event Mask on line 13
    #[inline(always)]
    #[must_use]
    pub fn mr13(&mut self) -> MR13_W<13> {
        MR13_W::new(self)
    }
    ///Bit 14 - Event Mask on line 14
    #[inline(always)]
    #[must_use]
    pub fn mr14(&mut self) -> MR14_W<14> {
        MR14_W::new(self)
    }
    ///Bit 15 - Event Mask on line 15
    #[inline(always)]
    #[must_use]
    pub fn mr15(&mut self) -> MR15_W<15> {
        MR15_W::new(self)
    }
    ///Bit 16 - Event Mask on line 16
    #[inline(always)]
    #[must_use]
    pub fn mr16(&mut self) -> MR16_W<16> {
        MR16_W::new(self)
    }
    ///Bit 17 - Event Mask on line 17
    #[inline(always)]
    #[must_use]
    pub fn mr17(&mut self) -> MR17_W<17> {
        MR17_W::new(self)
    }
    ///Bit 18 - Event Mask on line 18
    #[inline(always)]
    #[must_use]
    pub fn mr18(&mut self) -> MR18_W<18> {
        MR18_W::new(self)
    }
    ///Bit 19 - Event Mask on line 19
    #[inline(always)]
    #[must_use]
    pub fn mr19(&mut self) -> MR19_W<19> {
        MR19_W::new(self)
    }
    ///Bit 20 - Event Mask on line 20
    #[inline(always)]
    #[must_use]
    pub fn mr20(&mut self) -> MR20_W<20> {
        MR20_W::new(self)
    }
    ///Bit 21 - Event Mask on line 21
    #[inline(always)]
    #[must_use]
    pub fn mr21(&mut self) -> MR21_W<21> {
        MR21_W::new(self)
    }
    ///Bit 22 - Event Mask on line 22
    #[inline(always)]
    #[must_use]
    pub fn mr22(&mut self) -> MR22_W<22> {
        MR22_W::new(self)
    }
    ///Bit 23 - Event Mask on line 23
    #[inline(always)]
    #[must_use]
    pub fn mr23(&mut self) -> MR23_W<23> {
        MR23_W::new(self)
    }
    ///Bit 24 - Event Mask on line 24
    #[inline(always)]
    #[must_use]
    pub fn mr24(&mut self) -> MR24_W<24> {
        MR24_W::new(self)
    }
    ///Bit 25 - Event Mask on line 25
    #[inline(always)]
    #[must_use]
    pub fn mr25(&mut self) -> MR25_W<25> {
        MR25_W::new(self)
    }
    ///Bit 26 - Event Mask on line 26
    #[inline(always)]
    #[must_use]
    pub fn mr26(&mut self) -> MR26_W<26> {
        MR26_W::new(self)
    }
    ///Bit 27 - Event Mask on line 27
    #[inline(always)]
    #[must_use]
    pub fn mr27(&mut self) -> MR27_W<27> {
        MR27_W::new(self)
    }
    ///Bit 28 - Event Mask on line 28
    #[inline(always)]
    #[must_use]
    pub fn mr28(&mut self) -> MR28_W<28> {
        MR28_W::new(self)
    }
    ///Bit 29 - Event Mask on line 29
    #[inline(always)]
    #[must_use]
    pub fn mr29(&mut self) -> MR29_W<29> {
        MR29_W::new(self)
    }
    ///Bit 30 - Event Mask on line 30
    #[inline(always)]
    #[must_use]
    pub fn mr30(&mut self) -> MR30_W<30> {
        MR30_W::new(self)
    }
    ///Bit 31 - Event Mask on line 31
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
///Event mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [emr1](index.html) module
pub struct EMR1_SPEC;
impl crate::RegisterSpec for EMR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [emr1::R](R) reader structure
impl crate::Readable for EMR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [emr1::W](W) writer structure
impl crate::Writable for EMR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EMR1 to value 0
impl crate::Resettable for EMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
