///Register `C1EMR3` reader
pub struct R(crate::R<C1EMR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1EMR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1EMR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1EMR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C1EMR3` writer
pub struct W(crate::W<C1EMR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1EMR3_SPEC>;
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
impl From<crate::W<C1EMR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1EMR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MR64` reader - CPU Event mask on Event input x+64
pub type MR64_R = crate::BitReader<MR64_A>;
///CPU Event mask on Event input x+64
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MR64_A {
    ///0: Interrupt request line is masked
    Masked = 0,
    ///1: Interrupt request line is unmasked
    Unmasked = 1,
}
impl From<MR64_A> for bool {
    #[inline(always)]
    fn from(variant: MR64_A) -> Self {
        variant as u8 != 0
    }
}
impl MR64_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MR64_A {
        match self.bits {
            false => MR64_A::Masked,
            true => MR64_A::Unmasked,
        }
    }
    ///Checks if the value of the field is `Masked`
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == MR64_A::Masked
    }
    ///Checks if the value of the field is `Unmasked`
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == MR64_A::Unmasked
    }
}
///Field `MR64` writer - CPU Event mask on Event input x+64
pub type MR64_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1EMR3_SPEC, MR64_A, O>;
impl<'a, const O: u8> MR64_W<'a, O> {
    ///Interrupt request line is masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MR64_A::Masked)
    }
    ///Interrupt request line is unmasked
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(MR64_A::Unmasked)
    }
}
///Field `MR65` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR65_R;
///Field `MR66` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR66_R;
///Field `MR67` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR67_R;
///Field `MR68` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR68_R;
///Field `MR69` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR69_R;
///Field `MR70` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR70_R;
///Field `MR71` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR71_R;
///Field `MR72` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR72_R;
///Field `MR73` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR73_R;
///Field `MR74` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR74_R;
///Field `MR75` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR75_R;
///Field `MR76` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR76_R;
///Field `MR77` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR77_R;
///Field `MR78` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR78_R;
///Field `MR79` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR79_R;
///Field `MR80` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR80_R;
///Field `MR82` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR82_R;
///Field `MR84` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR84_R;
///Field `MR85` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR85_R;
///Field `MR86` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR86_R;
///Field `MR87` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR87_R;
///Field `MR88` reader - CPU Event mask on Event input x+64
pub use MR64_R as MR88_R;
///Field `MR65` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR65_W;
///Field `MR66` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR66_W;
///Field `MR67` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR67_W;
///Field `MR68` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR68_W;
///Field `MR69` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR69_W;
///Field `MR70` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR70_W;
///Field `MR71` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR71_W;
///Field `MR72` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR72_W;
///Field `MR73` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR73_W;
///Field `MR74` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR74_W;
///Field `MR75` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR75_W;
///Field `MR76` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR76_W;
///Field `MR77` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR77_W;
///Field `MR78` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR78_W;
///Field `MR79` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR79_W;
///Field `MR80` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR80_W;
///Field `MR82` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR82_W;
///Field `MR84` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR84_W;
///Field `MR85` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR85_W;
///Field `MR86` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR86_W;
///Field `MR87` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR87_W;
///Field `MR88` writer - CPU Event mask on Event input x+64
pub use MR64_W as MR88_W;
impl R {
    ///Bit 0 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr64(&self) -> MR64_R {
        MR64_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr65(&self) -> MR65_R {
        MR65_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr66(&self) -> MR66_R {
        MR66_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr67(&self) -> MR67_R {
        MR67_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr68(&self) -> MR68_R {
        MR68_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr69(&self) -> MR69_R {
        MR69_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr70(&self) -> MR70_R {
        MR70_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr71(&self) -> MR71_R {
        MR71_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr72(&self) -> MR72_R {
        MR72_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr73(&self) -> MR73_R {
        MR73_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr74(&self) -> MR74_R {
        MR74_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr75(&self) -> MR75_R {
        MR75_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr76(&self) -> MR76_R {
        MR76_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr77(&self) -> MR77_R {
        MR77_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr78(&self) -> MR78_R {
        MR78_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr79(&self) -> MR79_R {
        MR79_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr80(&self) -> MR80_R {
        MR80_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr82(&self) -> MR82_R {
        MR82_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr84(&self) -> MR84_R {
        MR84_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr85(&self) -> MR85_R {
        MR85_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr86(&self) -> MR86_R {
        MR86_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr87(&self) -> MR87_R {
        MR87_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CPU Event mask on Event input x+64
    #[inline(always)]
    pub fn mr88(&self) -> MR88_R {
        MR88_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CPU Event mask on Event input x+64
    #[inline(always)]
    #[must_use]
    pub fn mr64(&mut self) -> MR64_W<0> {
        MR64_W::new(self)
    }
    ///Bit 1 - CPU Event mask on Event input x+64
    #[inline(always)]
    #[must_use]
    pub fn mr65(&mut self) -> MR65_W<1> {
        MR65_W::new(self)
    }
    ///Bit 2 - CPU Event mask on Event input x+64
    #[inline(always)]
    #[must_use]
    pub fn mr66(&mut self) -> MR66_W<2> {
        MR66_W::new(self)
    }
    ///Bit 3 - CPU Event mask on Event input x+64
    #[inline(always)]
    #[must_use]
    pub fn mr67(&mut self) -> MR67_W<3> {
        MR67_W::new(self)
    }
    ///Bit 4 - CPU Event mask on Event input x+64
    #[inline(always)]
    #[must_use]
    pub fn mr68(&mut self) -> MR68_W<4> {
        MR68_W::new(self)
    }
    ///Bit 5 - CPU Event mask on Event input x+64
    #[inline(always)]
    #[must_use]
    pub fn mr69(&mut self) -> MR69_W<5> {
        MR69_W::new(self)
    }
    ///Bit 6 - CPU Event mask on Event input x+64
    #[inline(always)]
    #[must_use]
    pub fn mr70(&mut self) -> MR70_W<6> {
        MR70_W::new(self)
    }
    ///Bit 7 - CPU Event mask on Event input x+64
    #[inline(always)]
    #[must_use]
    pub fn mr71(&mut self) -> MR71_W<7> {
        MR71_W::new(self)
    }
    ///Bit 8 - CPU Event mask on Event input x+64
    #[inline(always)]
    #[must_use]
    pub fn mr72(&mut self) -> MR72_W<8> {
        MR72_W::new(self)
    }
    ///Bit 9 - CPU Event mask on Event input x+64
    #[inline(always)]
    #[must_use]
    pub fn mr73(&mut self) -> MR73_W<9> {
        MR73_W::new(self)
    }
    ///Bit 10 - CPU Event mask on Event input x+64
    #[inline(always)]
    #[must_use]
    pub fn mr74(&mut self) -> MR74_W<10> {
        MR74_W::new(self)
    }
    ///Bit 11 - CPU Event mask on Event input x+64
    #[inline(always)]
    #[must_use]
    pub fn mr75(&mut self) -> MR75_W<11> {
        MR75_W::new(self)
    }
    ///Bit 12 - CPU Event mask on Event input x+64
    #[inline(always)]
    #[must_use]
    pub fn mr76(&mut self) -> MR76_W<12> {
        MR76_W::new(self)
    }
    ///Bit 13 - CPU Event mask on Event input x+64
    #[inline(always)]
    #[must_use]
    pub fn mr77(&mut self) -> MR77_W<13> {
        MR77_W::new(self)
    }
    ///Bit 14 - CPU Event mask on Event input x+64
    #[inline(always)]
    #[must_use]
    pub fn mr78(&mut self) -> MR78_W<14> {
        MR78_W::new(self)
    }
    ///Bit 15 - CPU Event mask on Event input x+64
    #[inline(always)]
    #[must_use]
    pub fn mr79(&mut self) -> MR79_W<15> {
        MR79_W::new(self)
    }
    ///Bit 16 - CPU Event mask on Event input x+64
    #[inline(always)]
    #[must_use]
    pub fn mr80(&mut self) -> MR80_W<16> {
        MR80_W::new(self)
    }
    ///Bit 18 - CPU Event mask on Event input x+64
    #[inline(always)]
    #[must_use]
    pub fn mr82(&mut self) -> MR82_W<18> {
        MR82_W::new(self)
    }
    ///Bit 20 - CPU Event mask on Event input x+64
    #[inline(always)]
    #[must_use]
    pub fn mr84(&mut self) -> MR84_W<20> {
        MR84_W::new(self)
    }
    ///Bit 21 - CPU Event mask on Event input x+64
    #[inline(always)]
    #[must_use]
    pub fn mr85(&mut self) -> MR85_W<21> {
        MR85_W::new(self)
    }
    ///Bit 22 - CPU Event mask on Event input x+64
    #[inline(always)]
    #[must_use]
    pub fn mr86(&mut self) -> MR86_W<22> {
        MR86_W::new(self)
    }
    ///Bit 23 - CPU Event mask on Event input x+64
    #[inline(always)]
    #[must_use]
    pub fn mr87(&mut self) -> MR87_W<23> {
        MR87_W::new(self)
    }
    ///Bit 24 - CPU Event mask on Event input x+64
    #[inline(always)]
    #[must_use]
    pub fn mr88(&mut self) -> MR88_W<24> {
        MR88_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI event mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c1emr3](index.html) module
pub struct C1EMR3_SPEC;
impl crate::RegisterSpec for C1EMR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [c1emr3::R](R) reader structure
impl crate::Readable for C1EMR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c1emr3::W](W) writer structure
impl crate::Writable for C1EMR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C1EMR3 to value 0
impl crate::Resettable for C1EMR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
