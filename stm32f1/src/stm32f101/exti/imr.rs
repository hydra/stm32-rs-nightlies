///Register `IMR` reader
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IMR` writer
pub struct W(crate::W<IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR_SPEC>;
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
impl From<crate::W<IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MR0` reader - Interrupt Mask on line 0
pub type MR0_R = crate::BitReader<MR0_A>;
///Interrupt Mask on line 0
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
///Field `MR0` writer - Interrupt Mask on line 0
pub type MR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, MR0_A, O>;
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
///Field `MR1` reader - Interrupt Mask on line 1
pub use MR0_R as MR1_R;
///Field `MR2` reader - Interrupt Mask on line 2
pub use MR0_R as MR2_R;
///Field `MR3` reader - Interrupt Mask on line 3
pub use MR0_R as MR3_R;
///Field `MR4` reader - Interrupt Mask on line 4
pub use MR0_R as MR4_R;
///Field `MR5` reader - Interrupt Mask on line 5
pub use MR0_R as MR5_R;
///Field `MR6` reader - Interrupt Mask on line 6
pub use MR0_R as MR6_R;
///Field `MR7` reader - Interrupt Mask on line 7
pub use MR0_R as MR7_R;
///Field `MR8` reader - Interrupt Mask on line 8
pub use MR0_R as MR8_R;
///Field `MR9` reader - Interrupt Mask on line 9
pub use MR0_R as MR9_R;
///Field `MR10` reader - Interrupt Mask on line 10
pub use MR0_R as MR10_R;
///Field `MR11` reader - Interrupt Mask on line 11
pub use MR0_R as MR11_R;
///Field `MR12` reader - Interrupt Mask on line 12
pub use MR0_R as MR12_R;
///Field `MR13` reader - Interrupt Mask on line 13
pub use MR0_R as MR13_R;
///Field `MR14` reader - Interrupt Mask on line 14
pub use MR0_R as MR14_R;
///Field `MR15` reader - Interrupt Mask on line 15
pub use MR0_R as MR15_R;
///Field `MR16` reader - Interrupt Mask on line 16
pub use MR0_R as MR16_R;
///Field `MR17` reader - Interrupt Mask on line 17
pub use MR0_R as MR17_R;
///Field `MR18` reader - Interrupt Mask on line 18
pub use MR0_R as MR18_R;
///Field `MR1` writer - Interrupt Mask on line 1
pub use MR0_W as MR1_W;
///Field `MR2` writer - Interrupt Mask on line 2
pub use MR0_W as MR2_W;
///Field `MR3` writer - Interrupt Mask on line 3
pub use MR0_W as MR3_W;
///Field `MR4` writer - Interrupt Mask on line 4
pub use MR0_W as MR4_W;
///Field `MR5` writer - Interrupt Mask on line 5
pub use MR0_W as MR5_W;
///Field `MR6` writer - Interrupt Mask on line 6
pub use MR0_W as MR6_W;
///Field `MR7` writer - Interrupt Mask on line 7
pub use MR0_W as MR7_W;
///Field `MR8` writer - Interrupt Mask on line 8
pub use MR0_W as MR8_W;
///Field `MR9` writer - Interrupt Mask on line 9
pub use MR0_W as MR9_W;
///Field `MR10` writer - Interrupt Mask on line 10
pub use MR0_W as MR10_W;
///Field `MR11` writer - Interrupt Mask on line 11
pub use MR0_W as MR11_W;
///Field `MR12` writer - Interrupt Mask on line 12
pub use MR0_W as MR12_W;
///Field `MR13` writer - Interrupt Mask on line 13
pub use MR0_W as MR13_W;
///Field `MR14` writer - Interrupt Mask on line 14
pub use MR0_W as MR14_W;
///Field `MR15` writer - Interrupt Mask on line 15
pub use MR0_W as MR15_W;
///Field `MR16` writer - Interrupt Mask on line 16
pub use MR0_W as MR16_W;
///Field `MR17` writer - Interrupt Mask on line 17
pub use MR0_W as MR17_W;
///Field `MR18` writer - Interrupt Mask on line 18
pub use MR0_W as MR18_W;
impl R {
    ///Bit 0 - Interrupt Mask on line 0
    #[inline(always)]
    pub fn mr0(&self) -> MR0_R {
        MR0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt Mask on line 1
    #[inline(always)]
    pub fn mr1(&self) -> MR1_R {
        MR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt Mask on line 2
    #[inline(always)]
    pub fn mr2(&self) -> MR2_R {
        MR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt Mask on line 3
    #[inline(always)]
    pub fn mr3(&self) -> MR3_R {
        MR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Interrupt Mask on line 4
    #[inline(always)]
    pub fn mr4(&self) -> MR4_R {
        MR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Interrupt Mask on line 5
    #[inline(always)]
    pub fn mr5(&self) -> MR5_R {
        MR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Interrupt Mask on line 6
    #[inline(always)]
    pub fn mr6(&self) -> MR6_R {
        MR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Interrupt Mask on line 7
    #[inline(always)]
    pub fn mr7(&self) -> MR7_R {
        MR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Interrupt Mask on line 8
    #[inline(always)]
    pub fn mr8(&self) -> MR8_R {
        MR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Interrupt Mask on line 9
    #[inline(always)]
    pub fn mr9(&self) -> MR9_R {
        MR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Interrupt Mask on line 10
    #[inline(always)]
    pub fn mr10(&self) -> MR10_R {
        MR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Interrupt Mask on line 11
    #[inline(always)]
    pub fn mr11(&self) -> MR11_R {
        MR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Interrupt Mask on line 12
    #[inline(always)]
    pub fn mr12(&self) -> MR12_R {
        MR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Interrupt Mask on line 13
    #[inline(always)]
    pub fn mr13(&self) -> MR13_R {
        MR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Interrupt Mask on line 14
    #[inline(always)]
    pub fn mr14(&self) -> MR14_R {
        MR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Interrupt Mask on line 15
    #[inline(always)]
    pub fn mr15(&self) -> MR15_R {
        MR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Interrupt Mask on line 16
    #[inline(always)]
    pub fn mr16(&self) -> MR16_R {
        MR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Interrupt Mask on line 17
    #[inline(always)]
    pub fn mr17(&self) -> MR17_R {
        MR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Interrupt Mask on line 18
    #[inline(always)]
    pub fn mr18(&self) -> MR18_R {
        MR18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Interrupt Mask on line 0
    #[inline(always)]
    #[must_use]
    pub fn mr0(&mut self) -> MR0_W<0> {
        MR0_W::new(self)
    }
    ///Bit 1 - Interrupt Mask on line 1
    #[inline(always)]
    #[must_use]
    pub fn mr1(&mut self) -> MR1_W<1> {
        MR1_W::new(self)
    }
    ///Bit 2 - Interrupt Mask on line 2
    #[inline(always)]
    #[must_use]
    pub fn mr2(&mut self) -> MR2_W<2> {
        MR2_W::new(self)
    }
    ///Bit 3 - Interrupt Mask on line 3
    #[inline(always)]
    #[must_use]
    pub fn mr3(&mut self) -> MR3_W<3> {
        MR3_W::new(self)
    }
    ///Bit 4 - Interrupt Mask on line 4
    #[inline(always)]
    #[must_use]
    pub fn mr4(&mut self) -> MR4_W<4> {
        MR4_W::new(self)
    }
    ///Bit 5 - Interrupt Mask on line 5
    #[inline(always)]
    #[must_use]
    pub fn mr5(&mut self) -> MR5_W<5> {
        MR5_W::new(self)
    }
    ///Bit 6 - Interrupt Mask on line 6
    #[inline(always)]
    #[must_use]
    pub fn mr6(&mut self) -> MR6_W<6> {
        MR6_W::new(self)
    }
    ///Bit 7 - Interrupt Mask on line 7
    #[inline(always)]
    #[must_use]
    pub fn mr7(&mut self) -> MR7_W<7> {
        MR7_W::new(self)
    }
    ///Bit 8 - Interrupt Mask on line 8
    #[inline(always)]
    #[must_use]
    pub fn mr8(&mut self) -> MR8_W<8> {
        MR8_W::new(self)
    }
    ///Bit 9 - Interrupt Mask on line 9
    #[inline(always)]
    #[must_use]
    pub fn mr9(&mut self) -> MR9_W<9> {
        MR9_W::new(self)
    }
    ///Bit 10 - Interrupt Mask on line 10
    #[inline(always)]
    #[must_use]
    pub fn mr10(&mut self) -> MR10_W<10> {
        MR10_W::new(self)
    }
    ///Bit 11 - Interrupt Mask on line 11
    #[inline(always)]
    #[must_use]
    pub fn mr11(&mut self) -> MR11_W<11> {
        MR11_W::new(self)
    }
    ///Bit 12 - Interrupt Mask on line 12
    #[inline(always)]
    #[must_use]
    pub fn mr12(&mut self) -> MR12_W<12> {
        MR12_W::new(self)
    }
    ///Bit 13 - Interrupt Mask on line 13
    #[inline(always)]
    #[must_use]
    pub fn mr13(&mut self) -> MR13_W<13> {
        MR13_W::new(self)
    }
    ///Bit 14 - Interrupt Mask on line 14
    #[inline(always)]
    #[must_use]
    pub fn mr14(&mut self) -> MR14_W<14> {
        MR14_W::new(self)
    }
    ///Bit 15 - Interrupt Mask on line 15
    #[inline(always)]
    #[must_use]
    pub fn mr15(&mut self) -> MR15_W<15> {
        MR15_W::new(self)
    }
    ///Bit 16 - Interrupt Mask on line 16
    #[inline(always)]
    #[must_use]
    pub fn mr16(&mut self) -> MR16_W<16> {
        MR16_W::new(self)
    }
    ///Bit 17 - Interrupt Mask on line 17
    #[inline(always)]
    #[must_use]
    pub fn mr17(&mut self) -> MR17_W<17> {
        MR17_W::new(self)
    }
    ///Bit 18 - Interrupt Mask on line 18
    #[inline(always)]
    #[must_use]
    pub fn mr18(&mut self) -> MR18_W<18> {
        MR18_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Interrupt mask register (EXTI_IMR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [imr](index.html) module
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
///`read()` method returns [imr::R](R) reader structure
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [imr::W](W) writer structure
impl crate::Writable for IMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IMR to value 0
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
