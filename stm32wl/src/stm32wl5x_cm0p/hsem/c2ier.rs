///Register `C2IER` reader
pub struct R(crate::R<C2IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2IER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C2IER` writer
pub struct W(crate::W<C2IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2IER_SPEC>;
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
impl From<crate::W<C2IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2IER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ISE0` reader - Interrupt semaphore n enable bit
pub type ISE0_R = crate::BitReader<ISE0_A>;
///Interrupt semaphore n enable bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISE0_A {
    ///0: Interrupt generation disabled
    Disabled = 0,
    ///1: Interrupt generation enabled
    Enabled = 1,
}
impl From<ISE0_A> for bool {
    #[inline(always)]
    fn from(variant: ISE0_A) -> Self {
        variant as u8 != 0
    }
}
impl ISE0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ISE0_A {
        match self.bits {
            false => ISE0_A::Disabled,
            true => ISE0_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ISE0_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ISE0_A::Enabled
    }
}
///Field `ISE0` writer - Interrupt semaphore n enable bit
pub type ISE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IER_SPEC, ISE0_A, O>;
impl<'a, const O: u8> ISE0_W<'a, O> {
    ///Interrupt generation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ISE0_A::Disabled)
    }
    ///Interrupt generation enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ISE0_A::Enabled)
    }
}
///Field `ISE1` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE1_R;
///Field `ISE2` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE2_R;
///Field `ISE3` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE3_R;
///Field `ISE4` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE4_R;
///Field `ISE5` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE5_R;
///Field `ISE6` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE6_R;
///Field `ISE7` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE7_R;
///Field `ISE8` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE8_R;
///Field `ISE9` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE9_R;
///Field `ISE10` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE10_R;
///Field `ISE11` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE11_R;
///Field `ISE12` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE12_R;
///Field `ISE13` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE13_R;
///Field `ISE14` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE14_R;
///Field `ISE15` reader - Interrupt semaphore n enable bit
pub use ISE0_R as ISE15_R;
///Field `ISE1` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE1_W;
///Field `ISE2` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE2_W;
///Field `ISE3` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE3_W;
///Field `ISE4` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE4_W;
///Field `ISE5` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE5_W;
///Field `ISE6` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE6_W;
///Field `ISE7` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE7_W;
///Field `ISE8` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE8_W;
///Field `ISE9` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE9_W;
///Field `ISE10` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE10_W;
///Field `ISE11` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE11_W;
///Field `ISE12` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE12_W;
///Field `ISE13` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE13_W;
///Field `ISE14` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE14_W;
///Field `ISE15` writer - Interrupt semaphore n enable bit
pub use ISE0_W as ISE15_W;
impl R {
    ///Bit 0 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise0(&self) -> ISE0_R {
        ISE0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise1(&self) -> ISE1_R {
        ISE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise2(&self) -> ISE2_R {
        ISE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise3(&self) -> ISE3_R {
        ISE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise4(&self) -> ISE4_R {
        ISE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise5(&self) -> ISE5_R {
        ISE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise6(&self) -> ISE6_R {
        ISE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise7(&self) -> ISE7_R {
        ISE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise8(&self) -> ISE8_R {
        ISE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise9(&self) -> ISE9_R {
        ISE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise10(&self) -> ISE10_R {
        ISE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise11(&self) -> ISE11_R {
        ISE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise12(&self) -> ISE12_R {
        ISE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise13(&self) -> ISE13_R {
        ISE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise14(&self) -> ISE14_R {
        ISE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Interrupt semaphore n enable bit
    #[inline(always)]
    pub fn ise15(&self) -> ISE15_R {
        ISE15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Interrupt semaphore n enable bit
    #[inline(always)]
    #[must_use]
    pub fn ise0(&mut self) -> ISE0_W<0> {
        ISE0_W::new(self)
    }
    ///Bit 1 - Interrupt semaphore n enable bit
    #[inline(always)]
    #[must_use]
    pub fn ise1(&mut self) -> ISE1_W<1> {
        ISE1_W::new(self)
    }
    ///Bit 2 - Interrupt semaphore n enable bit
    #[inline(always)]
    #[must_use]
    pub fn ise2(&mut self) -> ISE2_W<2> {
        ISE2_W::new(self)
    }
    ///Bit 3 - Interrupt semaphore n enable bit
    #[inline(always)]
    #[must_use]
    pub fn ise3(&mut self) -> ISE3_W<3> {
        ISE3_W::new(self)
    }
    ///Bit 4 - Interrupt semaphore n enable bit
    #[inline(always)]
    #[must_use]
    pub fn ise4(&mut self) -> ISE4_W<4> {
        ISE4_W::new(self)
    }
    ///Bit 5 - Interrupt semaphore n enable bit
    #[inline(always)]
    #[must_use]
    pub fn ise5(&mut self) -> ISE5_W<5> {
        ISE5_W::new(self)
    }
    ///Bit 6 - Interrupt semaphore n enable bit
    #[inline(always)]
    #[must_use]
    pub fn ise6(&mut self) -> ISE6_W<6> {
        ISE6_W::new(self)
    }
    ///Bit 7 - Interrupt semaphore n enable bit
    #[inline(always)]
    #[must_use]
    pub fn ise7(&mut self) -> ISE7_W<7> {
        ISE7_W::new(self)
    }
    ///Bit 8 - Interrupt semaphore n enable bit
    #[inline(always)]
    #[must_use]
    pub fn ise8(&mut self) -> ISE8_W<8> {
        ISE8_W::new(self)
    }
    ///Bit 9 - Interrupt semaphore n enable bit
    #[inline(always)]
    #[must_use]
    pub fn ise9(&mut self) -> ISE9_W<9> {
        ISE9_W::new(self)
    }
    ///Bit 10 - Interrupt semaphore n enable bit
    #[inline(always)]
    #[must_use]
    pub fn ise10(&mut self) -> ISE10_W<10> {
        ISE10_W::new(self)
    }
    ///Bit 11 - Interrupt semaphore n enable bit
    #[inline(always)]
    #[must_use]
    pub fn ise11(&mut self) -> ISE11_W<11> {
        ISE11_W::new(self)
    }
    ///Bit 12 - Interrupt semaphore n enable bit
    #[inline(always)]
    #[must_use]
    pub fn ise12(&mut self) -> ISE12_W<12> {
        ISE12_W::new(self)
    }
    ///Bit 13 - Interrupt semaphore n enable bit
    #[inline(always)]
    #[must_use]
    pub fn ise13(&mut self) -> ISE13_W<13> {
        ISE13_W::new(self)
    }
    ///Bit 14 - Interrupt semaphore n enable bit
    #[inline(always)]
    #[must_use]
    pub fn ise14(&mut self) -> ISE14_W<14> {
        ISE14_W::new(self)
    }
    ///Bit 15 - Interrupt semaphore n enable bit
    #[inline(always)]
    #[must_use]
    pub fn ise15(&mut self) -> ISE15_W<15> {
        ISE15_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///HSEM Interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2ier](index.html) module
pub struct C2IER_SPEC;
impl crate::RegisterSpec for C2IER_SPEC {
    type Ux = u32;
}
///`read()` method returns [c2ier::R](R) reader structure
impl crate::Readable for C2IER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c2ier::W](W) writer structure
impl crate::Writable for C2IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C2IER to value 0
impl crate::Resettable for C2IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
