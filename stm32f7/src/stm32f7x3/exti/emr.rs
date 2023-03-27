///Register `EMR` reader
pub struct R(crate::R<EMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EMR` writer
pub struct W(crate::W<EMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMR_SPEC>;
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
impl From<crate::W<EMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EM0` reader - Event Mask on line 0
pub type EM0_R = crate::BitReader<EM0_A>;
///Event Mask on line 0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EM0_A {
    ///0: Interrupt request line is masked
    Masked = 0,
    ///1: Interrupt request line is unmasked
    Unmasked = 1,
}
impl From<EM0_A> for bool {
    #[inline(always)]
    fn from(variant: EM0_A) -> Self {
        variant as u8 != 0
    }
}
impl EM0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EM0_A {
        match self.bits {
            false => EM0_A::Masked,
            true => EM0_A::Unmasked,
        }
    }
    ///Checks if the value of the field is `Masked`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == EM0_A::Masked
    }
    ///Checks if the value of the field is `Unmasked`
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == EM0_A::Unmasked
    }
}
///Field `EM0` writer - Event Mask on line 0
pub type EM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, EM0_A, O>;
impl<'a, const O: u8> EM0_W<'a, O> {
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(EM0_A::Masked)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(EM0_A::Unmasked)
    }
}
///Field `EM1` reader - Event Mask on line 1
pub use EM0_R as EM1_R;
///Field `EM2` reader - Event Mask on line 2
pub use EM0_R as EM2_R;
///Field `EM3` reader - Event Mask on line 3
pub use EM0_R as EM3_R;
///Field `EM4` reader - Event Mask on line 4
pub use EM0_R as EM4_R;
///Field `EM5` reader - Event Mask on line 5
pub use EM0_R as EM5_R;
///Field `EM6` reader - Event Mask on line 6
pub use EM0_R as EM6_R;
///Field `EM7` reader - Event Mask on line 7
pub use EM0_R as EM7_R;
///Field `EM8` reader - Event Mask on line 8
pub use EM0_R as EM8_R;
///Field `EM9` reader - Event Mask on line 9
pub use EM0_R as EM9_R;
///Field `EM10` reader - Event Mask on line 10
pub use EM0_R as EM10_R;
///Field `EM11` reader - Event Mask on line 11
pub use EM0_R as EM11_R;
///Field `EM12` reader - Event Mask on line 12
pub use EM0_R as EM12_R;
///Field `EM13` reader - Event Mask on line 13
pub use EM0_R as EM13_R;
///Field `EM14` reader - Event Mask on line 14
pub use EM0_R as EM14_R;
///Field `EM15` reader - Event Mask on line 15
pub use EM0_R as EM15_R;
///Field `EM16` reader - Event Mask on line 16
pub use EM0_R as EM16_R;
///Field `EM17` reader - Event Mask on line 17
pub use EM0_R as EM17_R;
///Field `EM18` reader - Event Mask on line 18
pub use EM0_R as EM18_R;
///Field `EM19` reader - Event Mask on line 19
pub use EM0_R as EM19_R;
///Field `EM20` reader - Event Mask on line 20
pub use EM0_R as EM20_R;
///Field `EM21` reader - Event Mask on line 21
pub use EM0_R as EM21_R;
///Field `EM22` reader - Event Mask on line 22
pub use EM0_R as EM22_R;
///Field `EM23` reader - Event Mask on line 23
pub use EM0_R as EM23_R;
///Field `EM1` writer - Event Mask on line 1
pub use EM0_W as EM1_W;
///Field `EM2` writer - Event Mask on line 2
pub use EM0_W as EM2_W;
///Field `EM3` writer - Event Mask on line 3
pub use EM0_W as EM3_W;
///Field `EM4` writer - Event Mask on line 4
pub use EM0_W as EM4_W;
///Field `EM5` writer - Event Mask on line 5
pub use EM0_W as EM5_W;
///Field `EM6` writer - Event Mask on line 6
pub use EM0_W as EM6_W;
///Field `EM7` writer - Event Mask on line 7
pub use EM0_W as EM7_W;
///Field `EM8` writer - Event Mask on line 8
pub use EM0_W as EM8_W;
///Field `EM9` writer - Event Mask on line 9
pub use EM0_W as EM9_W;
///Field `EM10` writer - Event Mask on line 10
pub use EM0_W as EM10_W;
///Field `EM11` writer - Event Mask on line 11
pub use EM0_W as EM11_W;
///Field `EM12` writer - Event Mask on line 12
pub use EM0_W as EM12_W;
///Field `EM13` writer - Event Mask on line 13
pub use EM0_W as EM13_W;
///Field `EM14` writer - Event Mask on line 14
pub use EM0_W as EM14_W;
///Field `EM15` writer - Event Mask on line 15
pub use EM0_W as EM15_W;
///Field `EM16` writer - Event Mask on line 16
pub use EM0_W as EM16_W;
///Field `EM17` writer - Event Mask on line 17
pub use EM0_W as EM17_W;
///Field `EM18` writer - Event Mask on line 18
pub use EM0_W as EM18_W;
///Field `EM19` writer - Event Mask on line 19
pub use EM0_W as EM19_W;
///Field `EM20` writer - Event Mask on line 20
pub use EM0_W as EM20_W;
///Field `EM21` writer - Event Mask on line 21
pub use EM0_W as EM21_W;
///Field `EM22` writer - Event Mask on line 22
pub use EM0_W as EM22_W;
///Field `EM23` writer - Event Mask on line 23
pub use EM0_W as EM23_W;
impl R {
    ///Bit 0 - Event Mask on line 0
    #[inline(always)]
    pub fn em0(&self) -> EM0_R {
        EM0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Event Mask on line 1
    #[inline(always)]
    pub fn em1(&self) -> EM1_R {
        EM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Event Mask on line 2
    #[inline(always)]
    pub fn em2(&self) -> EM2_R {
        EM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Event Mask on line 3
    #[inline(always)]
    pub fn em3(&self) -> EM3_R {
        EM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Event Mask on line 4
    #[inline(always)]
    pub fn em4(&self) -> EM4_R {
        EM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Event Mask on line 5
    #[inline(always)]
    pub fn em5(&self) -> EM5_R {
        EM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Event Mask on line 6
    #[inline(always)]
    pub fn em6(&self) -> EM6_R {
        EM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Event Mask on line 7
    #[inline(always)]
    pub fn em7(&self) -> EM7_R {
        EM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Event Mask on line 8
    #[inline(always)]
    pub fn em8(&self) -> EM8_R {
        EM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Event Mask on line 9
    #[inline(always)]
    pub fn em9(&self) -> EM9_R {
        EM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Event Mask on line 10
    #[inline(always)]
    pub fn em10(&self) -> EM10_R {
        EM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Event Mask on line 11
    #[inline(always)]
    pub fn em11(&self) -> EM11_R {
        EM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Event Mask on line 12
    #[inline(always)]
    pub fn em12(&self) -> EM12_R {
        EM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Event Mask on line 13
    #[inline(always)]
    pub fn em13(&self) -> EM13_R {
        EM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Event Mask on line 14
    #[inline(always)]
    pub fn em14(&self) -> EM14_R {
        EM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Event Mask on line 15
    #[inline(always)]
    pub fn em15(&self) -> EM15_R {
        EM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Event Mask on line 16
    #[inline(always)]
    pub fn em16(&self) -> EM16_R {
        EM16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Event Mask on line 17
    #[inline(always)]
    pub fn em17(&self) -> EM17_R {
        EM17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Event Mask on line 18
    #[inline(always)]
    pub fn em18(&self) -> EM18_R {
        EM18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Event Mask on line 19
    #[inline(always)]
    pub fn em19(&self) -> EM19_R {
        EM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Event Mask on line 20
    #[inline(always)]
    pub fn em20(&self) -> EM20_R {
        EM20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Event Mask on line 21
    #[inline(always)]
    pub fn em21(&self) -> EM21_R {
        EM21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Event Mask on line 22
    #[inline(always)]
    pub fn em22(&self) -> EM22_R {
        EM22_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Event Mask on line 23
    #[inline(always)]
    pub fn em23(&self) -> EM23_R {
        EM23_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Event Mask on line 0
    #[inline(always)]
    #[must_use]
    pub fn em0(&mut self) -> EM0_W<0> {
        EM0_W::new(self)
    }
    ///Bit 1 - Event Mask on line 1
    #[inline(always)]
    #[must_use]
    pub fn em1(&mut self) -> EM1_W<1> {
        EM1_W::new(self)
    }
    ///Bit 2 - Event Mask on line 2
    #[inline(always)]
    #[must_use]
    pub fn em2(&mut self) -> EM2_W<2> {
        EM2_W::new(self)
    }
    ///Bit 3 - Event Mask on line 3
    #[inline(always)]
    #[must_use]
    pub fn em3(&mut self) -> EM3_W<3> {
        EM3_W::new(self)
    }
    ///Bit 4 - Event Mask on line 4
    #[inline(always)]
    #[must_use]
    pub fn em4(&mut self) -> EM4_W<4> {
        EM4_W::new(self)
    }
    ///Bit 5 - Event Mask on line 5
    #[inline(always)]
    #[must_use]
    pub fn em5(&mut self) -> EM5_W<5> {
        EM5_W::new(self)
    }
    ///Bit 6 - Event Mask on line 6
    #[inline(always)]
    #[must_use]
    pub fn em6(&mut self) -> EM6_W<6> {
        EM6_W::new(self)
    }
    ///Bit 7 - Event Mask on line 7
    #[inline(always)]
    #[must_use]
    pub fn em7(&mut self) -> EM7_W<7> {
        EM7_W::new(self)
    }
    ///Bit 8 - Event Mask on line 8
    #[inline(always)]
    #[must_use]
    pub fn em8(&mut self) -> EM8_W<8> {
        EM8_W::new(self)
    }
    ///Bit 9 - Event Mask on line 9
    #[inline(always)]
    #[must_use]
    pub fn em9(&mut self) -> EM9_W<9> {
        EM9_W::new(self)
    }
    ///Bit 10 - Event Mask on line 10
    #[inline(always)]
    #[must_use]
    pub fn em10(&mut self) -> EM10_W<10> {
        EM10_W::new(self)
    }
    ///Bit 11 - Event Mask on line 11
    #[inline(always)]
    #[must_use]
    pub fn em11(&mut self) -> EM11_W<11> {
        EM11_W::new(self)
    }
    ///Bit 12 - Event Mask on line 12
    #[inline(always)]
    #[must_use]
    pub fn em12(&mut self) -> EM12_W<12> {
        EM12_W::new(self)
    }
    ///Bit 13 - Event Mask on line 13
    #[inline(always)]
    #[must_use]
    pub fn em13(&mut self) -> EM13_W<13> {
        EM13_W::new(self)
    }
    ///Bit 14 - Event Mask on line 14
    #[inline(always)]
    #[must_use]
    pub fn em14(&mut self) -> EM14_W<14> {
        EM14_W::new(self)
    }
    ///Bit 15 - Event Mask on line 15
    #[inline(always)]
    #[must_use]
    pub fn em15(&mut self) -> EM15_W<15> {
        EM15_W::new(self)
    }
    ///Bit 16 - Event Mask on line 16
    #[inline(always)]
    #[must_use]
    pub fn em16(&mut self) -> EM16_W<16> {
        EM16_W::new(self)
    }
    ///Bit 17 - Event Mask on line 17
    #[inline(always)]
    #[must_use]
    pub fn em17(&mut self) -> EM17_W<17> {
        EM17_W::new(self)
    }
    ///Bit 18 - Event Mask on line 18
    #[inline(always)]
    #[must_use]
    pub fn em18(&mut self) -> EM18_W<18> {
        EM18_W::new(self)
    }
    ///Bit 19 - Event Mask on line 19
    #[inline(always)]
    #[must_use]
    pub fn em19(&mut self) -> EM19_W<19> {
        EM19_W::new(self)
    }
    ///Bit 20 - Event Mask on line 20
    #[inline(always)]
    #[must_use]
    pub fn em20(&mut self) -> EM20_W<20> {
        EM20_W::new(self)
    }
    ///Bit 21 - Event Mask on line 21
    #[inline(always)]
    #[must_use]
    pub fn em21(&mut self) -> EM21_W<21> {
        EM21_W::new(self)
    }
    ///Bit 22 - Event Mask on line 22
    #[inline(always)]
    #[must_use]
    pub fn em22(&mut self) -> EM22_W<22> {
        EM22_W::new(self)
    }
    ///Bit 23 - Event Mask on line 23
    #[inline(always)]
    #[must_use]
    pub fn em23(&mut self) -> EM23_W<23> {
        EM23_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Event mask register (EXTI_EMR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [emr](index.html) module
pub struct EMR_SPEC;
impl crate::RegisterSpec for EMR_SPEC {
    type Ux = u32;
}
///`read()` method returns [emr::R](R) reader structure
impl crate::Readable for EMR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [emr::W](W) writer structure
impl crate::Writable for EMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EMR to value 0
impl crate::Resettable for EMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
