///Register `ODR` reader
pub struct R(crate::R<ODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ODR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ODR` writer
pub struct W(crate::W<ODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ODR_SPEC>;
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
impl From<crate::W<ODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ODR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ODR0` reader - Port output data (y = 0..15)
pub type ODR0_R = crate::BitReader<ODR0_A>;
///Port output data (y = 0..15)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ODR0_A {
    ///0: Set output to logic low
    Low = 0,
    ///1: Set output to logic high
    High = 1,
}
impl From<ODR0_A> for bool {
    #[inline(always)]
    fn from(variant: ODR0_A) -> Self {
        variant as u8 != 0
    }
}
impl ODR0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ODR0_A {
        match self.bits {
            false => ODR0_A::Low,
            true => ODR0_A::High,
        }
    }
    ///Checks if the value of the field is `Low`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == ODR0_A::Low
    }
    ///Checks if the value of the field is `High`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == ODR0_A::High
    }
}
///Field `ODR0` writer - Port output data (y = 0..15)
pub type ODR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, ODR0_A, O>;
impl<'a, const O: u8> ODR0_W<'a, O> {
    ///Set output to logic low
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(ODR0_A::Low)
    }
    ///Set output to logic high
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(ODR0_A::High)
    }
}
///Field `ODR1` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR1_R;
///Field `ODR2` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR2_R;
///Field `ODR3` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR3_R;
///Field `ODR4` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR4_R;
///Field `ODR5` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR5_R;
///Field `ODR6` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR6_R;
///Field `ODR13` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR13_R;
///Field `ODR14` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR14_R;
///Field `ODR15` reader - Port output data (y = 0..15)
pub use ODR0_R as ODR15_R;
///Field `ODR1` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR1_W;
///Field `ODR2` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR2_W;
///Field `ODR3` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR3_W;
///Field `ODR4` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR4_W;
///Field `ODR5` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR5_W;
///Field `ODR6` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR6_W;
///Field `ODR13` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR13_W;
///Field `ODR14` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR14_W;
///Field `ODR15` writer - Port output data (y = 0..15)
pub use ODR0_W as ODR15_W;
impl R {
    ///Bit 0 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr0(&self) -> ODR0_R {
        ODR0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr1(&self) -> ODR1_R {
        ODR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr2(&self) -> ODR2_R {
        ODR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr3(&self) -> ODR3_R {
        ODR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr4(&self) -> ODR4_R {
        ODR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr5(&self) -> ODR5_R {
        ODR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr6(&self) -> ODR6_R {
        ODR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 13 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr13(&self) -> ODR13_R {
        ODR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr14(&self) -> ODR14_R {
        ODR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr15(&self) -> ODR15_R {
        ODR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Port output data (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn odr0(&mut self) -> ODR0_W<0> {
        ODR0_W::new(self)
    }
    ///Bit 1 - Port output data (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn odr1(&mut self) -> ODR1_W<1> {
        ODR1_W::new(self)
    }
    ///Bit 2 - Port output data (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn odr2(&mut self) -> ODR2_W<2> {
        ODR2_W::new(self)
    }
    ///Bit 3 - Port output data (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn odr3(&mut self) -> ODR3_W<3> {
        ODR3_W::new(self)
    }
    ///Bit 4 - Port output data (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn odr4(&mut self) -> ODR4_W<4> {
        ODR4_W::new(self)
    }
    ///Bit 5 - Port output data (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn odr5(&mut self) -> ODR5_W<5> {
        ODR5_W::new(self)
    }
    ///Bit 6 - Port output data (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn odr6(&mut self) -> ODR6_W<6> {
        ODR6_W::new(self)
    }
    ///Bit 13 - Port output data (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn odr13(&mut self) -> ODR13_W<13> {
        ODR13_W::new(self)
    }
    ///Bit 14 - Port output data (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn odr14(&mut self) -> ODR14_W<14> {
        ODR14_W::new(self)
    }
    ///Bit 15 - Port output data (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn odr15(&mut self) -> ODR15_W<15> {
        ODR15_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO port output data register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [odr](index.html) module
pub struct ODR_SPEC;
impl crate::RegisterSpec for ODR_SPEC {
    type Ux = u32;
}
///`read()` method returns [odr::R](R) reader structure
impl crate::Readable for ODR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [odr::W](W) writer structure
impl crate::Writable for ODR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ODR to value 0
impl crate::Resettable for ODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
