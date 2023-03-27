///Register `FPR1` reader
pub struct R(crate::R<FPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FPR1` writer
pub struct W(crate::W<FPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPR1_SPEC>;
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
impl From<crate::W<FPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FPIF0` reader - configurable event inputs x falling edge pending bit.
pub type FPIF0_R = crate::BitReader<FPIF0R_A>;
///configurable event inputs x falling edge pending bit.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF0R_A {
    ///0: No trigger request occurred
    NotPending = 0,
    ///1: Selected trigger request occurred
    Pending = 1,
}
impl From<FPIF0R_A> for bool {
    #[inline(always)]
    fn from(variant: FPIF0R_A) -> Self {
        variant as u8 != 0
    }
}
impl FPIF0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FPIF0R_A {
        match self.bits {
            false => FPIF0R_A::NotPending,
            true => FPIF0R_A::Pending,
        }
    }
    ///Checks if the value of the field is `NotPending`
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == FPIF0R_A::NotPending
    }
    ///Checks if the value of the field is `Pending`
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == FPIF0R_A::Pending
    }
}
///configurable event inputs x falling edge pending bit.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPIF0W_AW {
    ///1: Clears pending bit
    Clear = 1,
}
impl From<FPIF0W_AW> for bool {
    #[inline(always)]
    fn from(variant: FPIF0W_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `FPIF0` writer - configurable event inputs x falling edge pending bit.
pub type FPIF0_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, FPR1_SPEC, FPIF0W_AW, O>;
impl<'a, const O: u8> FPIF0_W<'a, O> {
    ///Clears pending bit
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FPIF0W_AW::Clear)
    }
}
///Field `FPIF1` reader - configurable event inputs x falling edge pending bit.
pub use FPIF0_R as FPIF1_R;
///Field `FPIF2` reader - configurable event inputs x falling edge pending bit.
pub use FPIF0_R as FPIF2_R;
///Field `FPIF3` reader - configurable event inputs x falling edge pending bit.
pub use FPIF0_R as FPIF3_R;
///Field `FPIF4` reader - configurable event inputs x falling edge pending bit.
pub use FPIF0_R as FPIF4_R;
///Field `FPIF5` reader - configurable event inputs x falling edge pending bit.
pub use FPIF0_R as FPIF5_R;
///Field `FPIF6` reader - configurable event inputs x falling edge pending bit.
pub use FPIF0_R as FPIF6_R;
///Field `FPIF7` reader - configurable event inputs x falling edge pending bit.
pub use FPIF0_R as FPIF7_R;
///Field `FPIF8` reader - configurable event inputs x falling edge pending bit.
pub use FPIF0_R as FPIF8_R;
///Field `FPIF9` reader - configurable event inputs x falling edge pending bit.
pub use FPIF0_R as FPIF9_R;
///Field `FPIF10` reader - configurable event inputs x falling edge pending bit.
pub use FPIF0_R as FPIF10_R;
///Field `FPIF11` reader - configurable event inputs x falling edge pending bit.
pub use FPIF0_R as FPIF11_R;
///Field `FPIF12` reader - configurable event inputs x falling edge pending bit.
pub use FPIF0_R as FPIF12_R;
///Field `FPIF13` reader - configurable event inputs x falling edge pending bit.
pub use FPIF0_R as FPIF13_R;
///Field `FPIF14` reader - configurable event inputs x falling edge pending bit.
pub use FPIF0_R as FPIF14_R;
///Field `FPIF15` reader - configurable event inputs x falling edge pending bit.
pub use FPIF0_R as FPIF15_R;
///Field `FPIF16` reader - configurable event inputs x falling edge pending bit.
pub use FPIF0_R as FPIF16_R;
///Field `FPIF1` writer - configurable event inputs x falling edge pending bit.
pub use FPIF0_W as FPIF1_W;
///Field `FPIF2` writer - configurable event inputs x falling edge pending bit.
pub use FPIF0_W as FPIF2_W;
///Field `FPIF3` writer - configurable event inputs x falling edge pending bit.
pub use FPIF0_W as FPIF3_W;
///Field `FPIF4` writer - configurable event inputs x falling edge pending bit.
pub use FPIF0_W as FPIF4_W;
///Field `FPIF5` writer - configurable event inputs x falling edge pending bit.
pub use FPIF0_W as FPIF5_W;
///Field `FPIF6` writer - configurable event inputs x falling edge pending bit.
pub use FPIF0_W as FPIF6_W;
///Field `FPIF7` writer - configurable event inputs x falling edge pending bit.
pub use FPIF0_W as FPIF7_W;
///Field `FPIF8` writer - configurable event inputs x falling edge pending bit.
pub use FPIF0_W as FPIF8_W;
///Field `FPIF9` writer - configurable event inputs x falling edge pending bit.
pub use FPIF0_W as FPIF9_W;
///Field `FPIF10` writer - configurable event inputs x falling edge pending bit.
pub use FPIF0_W as FPIF10_W;
///Field `FPIF11` writer - configurable event inputs x falling edge pending bit.
pub use FPIF0_W as FPIF11_W;
///Field `FPIF12` writer - configurable event inputs x falling edge pending bit.
pub use FPIF0_W as FPIF12_W;
///Field `FPIF13` writer - configurable event inputs x falling edge pending bit.
pub use FPIF0_W as FPIF13_W;
///Field `FPIF14` writer - configurable event inputs x falling edge pending bit.
pub use FPIF0_W as FPIF14_W;
///Field `FPIF15` writer - configurable event inputs x falling edge pending bit.
pub use FPIF0_W as FPIF15_W;
///Field `FPIF16` writer - configurable event inputs x falling edge pending bit.
pub use FPIF0_W as FPIF16_W;
impl R {
    ///Bit 0 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif0(&self) -> FPIF0_R {
        FPIF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif1(&self) -> FPIF1_R {
        FPIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif2(&self) -> FPIF2_R {
        FPIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif3(&self) -> FPIF3_R {
        FPIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif4(&self) -> FPIF4_R {
        FPIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif5(&self) -> FPIF5_R {
        FPIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif6(&self) -> FPIF6_R {
        FPIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif7(&self) -> FPIF7_R {
        FPIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif8(&self) -> FPIF8_R {
        FPIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif9(&self) -> FPIF9_R {
        FPIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif10(&self) -> FPIF10_R {
        FPIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif11(&self) -> FPIF11_R {
        FPIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif12(&self) -> FPIF12_R {
        FPIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif13(&self) -> FPIF13_R {
        FPIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif14(&self) -> FPIF14_R {
        FPIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif15(&self) -> FPIF15_R {
        FPIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif16(&self) -> FPIF16_R {
        FPIF16_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif0(&mut self) -> FPIF0_W<0> {
        FPIF0_W::new(self)
    }
    ///Bit 1 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif1(&mut self) -> FPIF1_W<1> {
        FPIF1_W::new(self)
    }
    ///Bit 2 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif2(&mut self) -> FPIF2_W<2> {
        FPIF2_W::new(self)
    }
    ///Bit 3 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif3(&mut self) -> FPIF3_W<3> {
        FPIF3_W::new(self)
    }
    ///Bit 4 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif4(&mut self) -> FPIF4_W<4> {
        FPIF4_W::new(self)
    }
    ///Bit 5 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif5(&mut self) -> FPIF5_W<5> {
        FPIF5_W::new(self)
    }
    ///Bit 6 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif6(&mut self) -> FPIF6_W<6> {
        FPIF6_W::new(self)
    }
    ///Bit 7 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif7(&mut self) -> FPIF7_W<7> {
        FPIF7_W::new(self)
    }
    ///Bit 8 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif8(&mut self) -> FPIF8_W<8> {
        FPIF8_W::new(self)
    }
    ///Bit 9 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif9(&mut self) -> FPIF9_W<9> {
        FPIF9_W::new(self)
    }
    ///Bit 10 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif10(&mut self) -> FPIF10_W<10> {
        FPIF10_W::new(self)
    }
    ///Bit 11 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif11(&mut self) -> FPIF11_W<11> {
        FPIF11_W::new(self)
    }
    ///Bit 12 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif12(&mut self) -> FPIF12_W<12> {
        FPIF12_W::new(self)
    }
    ///Bit 13 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif13(&mut self) -> FPIF13_W<13> {
        FPIF13_W::new(self)
    }
    ///Bit 14 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif14(&mut self) -> FPIF14_W<14> {
        FPIF14_W::new(self)
    }
    ///Bit 15 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif15(&mut self) -> FPIF15_W<15> {
        FPIF15_W::new(self)
    }
    ///Bit 16 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif16(&mut self) -> FPIF16_W<16> {
        FPIF16_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI falling edge pending register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fpr1](index.html) module
pub struct FPR1_SPEC;
impl crate::RegisterSpec for FPR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [fpr1::R](R) reader structure
impl crate::Readable for FPR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fpr1::W](W) writer structure
impl crate::Writable for FPR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0001_ffff;
}
///`reset()` method sets FPR1 to value 0
impl crate::Resettable for FPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
