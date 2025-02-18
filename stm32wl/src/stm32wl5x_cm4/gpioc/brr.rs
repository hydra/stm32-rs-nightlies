///Register `BRR` reader
pub struct R(crate::R<BRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BRR` writer
pub struct W(crate::W<BRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRR_SPEC>;
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
impl From<crate::W<BRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BR0` reader - Port Reset bit
pub type BR0_R = crate::BitReader<BR0W_A>;
///Port Reset bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR0W_A {
    ///0: No action on the corresponding ODx bit
    NoAction = 0,
    ///1: Reset the ODx bit
    Reset = 1,
}
impl From<BR0W_A> for bool {
    #[inline(always)]
    fn from(variant: BR0W_A) -> Self {
        variant as u8 != 0
    }
}
impl BR0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BR0W_A {
        match self.bits {
            false => BR0W_A::NoAction,
            true => BR0W_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoAction`
    #[inline(always)]
    pub fn is_no_action(&self) -> bool {
        *self == BR0W_A::NoAction
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == BR0W_A::Reset
    }
}
///Field `BR0` writer - Port Reset bit
pub type BR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, BR0W_A, O>;
impl<'a, const O: u8> BR0_W<'a, O> {
    ///No action on the corresponding ODx bit
    #[inline(always)]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR0W_A::NoAction)
    }
    ///Reset the ODx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR0W_A::Reset)
    }
}
///Field `BR1` reader - Port Reset bit
pub use BR0_R as BR1_R;
///Field `BR2` reader - Port Reset bit
pub use BR0_R as BR2_R;
///Field `BR3` reader - Port Reset bit
pub use BR0_R as BR3_R;
///Field `BR4` reader - Port Reset bit
pub use BR0_R as BR4_R;
///Field `BR5` reader - Port Reset bit
pub use BR0_R as BR5_R;
///Field `BR6` reader - Port Reset bit
pub use BR0_R as BR6_R;
///Field `BR13` reader - Port Reset bit
pub use BR0_R as BR13_R;
///Field `BR14` reader - Port Reset bit
pub use BR0_R as BR14_R;
///Field `BR15` reader - Port Reset bit
pub use BR0_R as BR15_R;
///Field `BR1` writer - Port Reset bit
pub use BR0_W as BR1_W;
///Field `BR2` writer - Port Reset bit
pub use BR0_W as BR2_W;
///Field `BR3` writer - Port Reset bit
pub use BR0_W as BR3_W;
///Field `BR4` writer - Port Reset bit
pub use BR0_W as BR4_W;
///Field `BR5` writer - Port Reset bit
pub use BR0_W as BR5_W;
///Field `BR6` writer - Port Reset bit
pub use BR0_W as BR6_W;
///Field `BR13` writer - Port Reset bit
pub use BR0_W as BR13_W;
///Field `BR14` writer - Port Reset bit
pub use BR0_W as BR14_W;
///Field `BR15` writer - Port Reset bit
pub use BR0_W as BR15_W;
impl R {
    ///Bit 0 - Port Reset bit
    #[inline(always)]
    pub fn br0(&self) -> BR0_R {
        BR0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port Reset bit
    #[inline(always)]
    pub fn br1(&self) -> BR1_R {
        BR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port Reset bit
    #[inline(always)]
    pub fn br2(&self) -> BR2_R {
        BR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port Reset bit
    #[inline(always)]
    pub fn br3(&self) -> BR3_R {
        BR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port Reset bit
    #[inline(always)]
    pub fn br4(&self) -> BR4_R {
        BR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port Reset bit
    #[inline(always)]
    pub fn br5(&self) -> BR5_R {
        BR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port Reset bit
    #[inline(always)]
    pub fn br6(&self) -> BR6_R {
        BR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 13 - Port Reset bit
    #[inline(always)]
    pub fn br13(&self) -> BR13_R {
        BR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port Reset bit
    #[inline(always)]
    pub fn br14(&self) -> BR14_R {
        BR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port Reset bit
    #[inline(always)]
    pub fn br15(&self) -> BR15_R {
        BR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Port Reset bit
    #[inline(always)]
    #[must_use]
    pub fn br0(&mut self) -> BR0_W<0> {
        BR0_W::new(self)
    }
    ///Bit 1 - Port Reset bit
    #[inline(always)]
    #[must_use]
    pub fn br1(&mut self) -> BR1_W<1> {
        BR1_W::new(self)
    }
    ///Bit 2 - Port Reset bit
    #[inline(always)]
    #[must_use]
    pub fn br2(&mut self) -> BR2_W<2> {
        BR2_W::new(self)
    }
    ///Bit 3 - Port Reset bit
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<3> {
        BR3_W::new(self)
    }
    ///Bit 4 - Port Reset bit
    #[inline(always)]
    #[must_use]
    pub fn br4(&mut self) -> BR4_W<4> {
        BR4_W::new(self)
    }
    ///Bit 5 - Port Reset bit
    #[inline(always)]
    #[must_use]
    pub fn br5(&mut self) -> BR5_W<5> {
        BR5_W::new(self)
    }
    ///Bit 6 - Port Reset bit
    #[inline(always)]
    #[must_use]
    pub fn br6(&mut self) -> BR6_W<6> {
        BR6_W::new(self)
    }
    ///Bit 13 - Port Reset bit
    #[inline(always)]
    #[must_use]
    pub fn br13(&mut self) -> BR13_W<13> {
        BR13_W::new(self)
    }
    ///Bit 14 - Port Reset bit
    #[inline(always)]
    #[must_use]
    pub fn br14(&mut self) -> BR14_W<14> {
        BR14_W::new(self)
    }
    ///Bit 15 - Port Reset bit
    #[inline(always)]
    #[must_use]
    pub fn br15(&mut self) -> BR15_W<15> {
        BR15_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO port bit reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [brr](index.html) module
pub struct BRR_SPEC;
impl crate::RegisterSpec for BRR_SPEC {
    type Ux = u32;
}
///`read()` method returns [brr::R](R) reader structure
impl crate::Readable for BRR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [brr::W](W) writer structure
impl crate::Writable for BRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BRR to value 0
impl crate::Resettable for BRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
