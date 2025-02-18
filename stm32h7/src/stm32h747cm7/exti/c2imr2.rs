///Register `C2IMR2` reader
pub struct R(crate::R<C2IMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2IMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2IMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2IMR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C2IMR2` writer
pub struct W(crate::W<C2IMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2IMR2_SPEC>;
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
impl From<crate::W<C2IMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2IMR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MR32` reader - CPU2 interrupt Mask on Direct Event input x+32
pub type MR32_R = crate::BitReader<MR32_A>;
///CPU2 interrupt Mask on Direct Event input x+32
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR32_A {
    ///0: Interrupt request line is masked
    Masked = 0,
    ///1: Interrupt request line is unmasked
    Unmasked = 1,
}
impl From<MR32_A> for bool {
    #[inline(always)]
    fn from(variant: MR32_A) -> Self {
        variant as u8 != 0
    }
}
impl MR32_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MR32_A {
        match self.bits {
            false => MR32_A::Masked,
            true => MR32_A::Unmasked,
        }
    }
    ///Checks if the value of the field is `Masked`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == MR32_A::Masked
    }
    ///Checks if the value of the field is `Unmasked`
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == MR32_A::Unmasked
    }
}
///Field `MR32` writer - CPU2 interrupt Mask on Direct Event input x+32
pub type MR32_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, MR32_A, O>;
impl<'a, const O: u8> MR32_W<'a, O> {
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR32_A::Masked)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR32_A::Unmasked)
    }
}
///Field `MR33` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR33_R;
///Field `MR34` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR34_R;
///Field `MR35` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR35_R;
///Field `MR36` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR36_R;
///Field `MR37` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR37_R;
///Field `MR38` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR38_R;
///Field `MR39` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR39_R;
///Field `MR40` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR40_R;
///Field `MR41` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR41_R;
///Field `MR42` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR42_R;
///Field `MR43` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR43_R;
///Field `MR44` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR44_R;
///Field `MR46` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR46_R;
///Field `MR47` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR47_R;
///Field `MR48` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR48_R;
///Field `MR49` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR49_R;
///Field `MR50` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR50_R;
///Field `MR51` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR51_R;
///Field `MR52` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR52_R;
///Field `MR53` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR53_R;
///Field `MR54` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR54_R;
///Field `MR55` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR55_R;
///Field `MR56` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR56_R;
///Field `MR57` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR57_R;
///Field `MR58` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR58_R;
///Field `MR59` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR59_R;
///Field `MR60` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR60_R;
///Field `MR61` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR61_R;
///Field `MR62` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR62_R;
///Field `MR63` reader - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_R as MR63_R;
///Field `MR33` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR33_W;
///Field `MR34` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR34_W;
///Field `MR35` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR35_W;
///Field `MR36` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR36_W;
///Field `MR37` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR37_W;
///Field `MR38` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR38_W;
///Field `MR39` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR39_W;
///Field `MR40` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR40_W;
///Field `MR41` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR41_W;
///Field `MR42` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR42_W;
///Field `MR43` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR43_W;
///Field `MR44` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR44_W;
///Field `MR46` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR46_W;
///Field `MR47` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR47_W;
///Field `MR48` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR48_W;
///Field `MR49` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR49_W;
///Field `MR50` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR50_W;
///Field `MR51` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR51_W;
///Field `MR52` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR52_W;
///Field `MR53` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR53_W;
///Field `MR54` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR54_W;
///Field `MR55` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR55_W;
///Field `MR56` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR56_W;
///Field `MR57` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR57_W;
///Field `MR58` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR58_W;
///Field `MR59` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR59_W;
///Field `MR60` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR60_W;
///Field `MR61` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR61_W;
///Field `MR62` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR62_W;
///Field `MR63` writer - CPU2 interrupt Mask on Direct Event input x+32
pub use MR32_W as MR63_W;
impl R {
    ///Bit 0 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr32(&self) -> MR32_R {
        MR32_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr33(&self) -> MR33_R {
        MR33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr34(&self) -> MR34_R {
        MR34_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr35(&self) -> MR35_R {
        MR35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr36(&self) -> MR36_R {
        MR36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr37(&self) -> MR37_R {
        MR37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr38(&self) -> MR38_R {
        MR38_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr39(&self) -> MR39_R {
        MR39_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr40(&self) -> MR40_R {
        MR40_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr41(&self) -> MR41_R {
        MR41_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr42(&self) -> MR42_R {
        MR42_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr43(&self) -> MR43_R {
        MR43_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr44(&self) -> MR44_R {
        MR44_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr46(&self) -> MR46_R {
        MR46_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr47(&self) -> MR47_R {
        MR47_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr48(&self) -> MR48_R {
        MR48_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr49(&self) -> MR49_R {
        MR49_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr50(&self) -> MR50_R {
        MR50_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr51(&self) -> MR51_R {
        MR51_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr52(&self) -> MR52_R {
        MR52_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr53(&self) -> MR53_R {
        MR53_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr54(&self) -> MR54_R {
        MR54_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr55(&self) -> MR55_R {
        MR55_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr56(&self) -> MR56_R {
        MR56_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr57(&self) -> MR57_R {
        MR57_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr58(&self) -> MR58_R {
        MR58_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr59(&self) -> MR59_R {
        MR59_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr60(&self) -> MR60_R {
        MR60_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr61(&self) -> MR61_R {
        MR61_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr62(&self) -> MR62_R {
        MR62_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    pub fn mr63(&self) -> MR63_R {
        MR63_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr32(&mut self) -> MR32_W<0> {
        MR32_W::new(self)
    }
    ///Bit 1 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr33(&mut self) -> MR33_W<1> {
        MR33_W::new(self)
    }
    ///Bit 2 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr34(&mut self) -> MR34_W<2> {
        MR34_W::new(self)
    }
    ///Bit 3 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr35(&mut self) -> MR35_W<3> {
        MR35_W::new(self)
    }
    ///Bit 4 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr36(&mut self) -> MR36_W<4> {
        MR36_W::new(self)
    }
    ///Bit 5 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr37(&mut self) -> MR37_W<5> {
        MR37_W::new(self)
    }
    ///Bit 6 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr38(&mut self) -> MR38_W<6> {
        MR38_W::new(self)
    }
    ///Bit 7 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr39(&mut self) -> MR39_W<7> {
        MR39_W::new(self)
    }
    ///Bit 8 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr40(&mut self) -> MR40_W<8> {
        MR40_W::new(self)
    }
    ///Bit 9 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr41(&mut self) -> MR41_W<9> {
        MR41_W::new(self)
    }
    ///Bit 10 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr42(&mut self) -> MR42_W<10> {
        MR42_W::new(self)
    }
    ///Bit 11 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr43(&mut self) -> MR43_W<11> {
        MR43_W::new(self)
    }
    ///Bit 12 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr44(&mut self) -> MR44_W<12> {
        MR44_W::new(self)
    }
    ///Bit 14 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr46(&mut self) -> MR46_W<14> {
        MR46_W::new(self)
    }
    ///Bit 15 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr47(&mut self) -> MR47_W<15> {
        MR47_W::new(self)
    }
    ///Bit 16 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr48(&mut self) -> MR48_W<16> {
        MR48_W::new(self)
    }
    ///Bit 17 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr49(&mut self) -> MR49_W<17> {
        MR49_W::new(self)
    }
    ///Bit 18 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr50(&mut self) -> MR50_W<18> {
        MR50_W::new(self)
    }
    ///Bit 19 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr51(&mut self) -> MR51_W<19> {
        MR51_W::new(self)
    }
    ///Bit 20 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr52(&mut self) -> MR52_W<20> {
        MR52_W::new(self)
    }
    ///Bit 21 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr53(&mut self) -> MR53_W<21> {
        MR53_W::new(self)
    }
    ///Bit 22 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr54(&mut self) -> MR54_W<22> {
        MR54_W::new(self)
    }
    ///Bit 23 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr55(&mut self) -> MR55_W<23> {
        MR55_W::new(self)
    }
    ///Bit 24 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr56(&mut self) -> MR56_W<24> {
        MR56_W::new(self)
    }
    ///Bit 25 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr57(&mut self) -> MR57_W<25> {
        MR57_W::new(self)
    }
    ///Bit 26 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr58(&mut self) -> MR58_W<26> {
        MR58_W::new(self)
    }
    ///Bit 27 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr59(&mut self) -> MR59_W<27> {
        MR59_W::new(self)
    }
    ///Bit 28 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr60(&mut self) -> MR60_W<28> {
        MR60_W::new(self)
    }
    ///Bit 29 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr61(&mut self) -> MR61_W<29> {
        MR61_W::new(self)
    }
    ///Bit 30 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr62(&mut self) -> MR62_W<30> {
        MR62_W::new(self)
    }
    ///Bit 31 - CPU2 interrupt Mask on Direct Event input x+32
    #[inline(always)]
    #[must_use]
    pub fn mr63(&mut self) -> MR63_W<31> {
        MR63_W::new(self)
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
///For information about available fields see [c2imr2](index.html) module
pub struct C2IMR2_SPEC;
impl crate::RegisterSpec for C2IMR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [c2imr2::R](R) reader structure
impl crate::Readable for C2IMR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c2imr2::W](W) writer structure
impl crate::Writable for C2IMR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C2IMR2 to value 0xfff5_ffff
impl crate::Resettable for C2IMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0xfff5_ffff;
}
