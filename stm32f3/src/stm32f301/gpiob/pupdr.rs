///Register `PUPDR` reader
pub struct R(crate::R<PUPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUPDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PUPDR` writer
pub struct W(crate::W<PUPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUPDR_SPEC>;
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
impl From<crate::W<PUPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUPDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PUPDR0` reader - Port x configuration bits (y = 0..15)
pub type PUPDR0_R = crate::FieldReader<u8, PUPDR0_A>;
///Port x configuration bits (y = 0..15)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PUPDR0_A {
    ///0: No pull-up, pull-down
    Floating = 0,
    ///1: Pull-up
    PullUp = 1,
    ///2: Pull-down
    PullDown = 2,
}
impl From<PUPDR0_A> for u8 {
    #[inline(always)]
    fn from(variant: PUPDR0_A) -> Self {
        variant as _
    }
}
impl PUPDR0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PUPDR0_A> {
        match self.bits {
            0 => Some(PUPDR0_A::Floating),
            1 => Some(PUPDR0_A::PullUp),
            2 => Some(PUPDR0_A::PullDown),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Floating`
    #[inline(always)]
    pub fn is_floating(&self) -> bool {
        *self == PUPDR0_A::Floating
    }
    ///Checks if the value of the field is `PullUp`
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == PUPDR0_A::PullUp
    }
    ///Checks if the value of the field is `PullDown`
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == PUPDR0_A::PullDown
    }
}
///Field `PUPDR0` writer - Port x configuration bits (y = 0..15)
pub type PUPDR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, PUPDR0_A, 2, O>;
impl<'a, const O: u8> PUPDR0_W<'a, O> {
    ///No pull-up, pull-down
    #[inline(always)]
    pub fn floating(self) -> &'a mut W {
        self.variant(PUPDR0_A::Floating)
    }
    ///Pull-up
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(PUPDR0_A::PullUp)
    }
    ///Pull-down
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(PUPDR0_A::PullDown)
    }
}
///Field `PUPDR1` reader - Port x configuration bits (y = 0..15)
pub use PUPDR0_R as PUPDR1_R;
///Field `PUPDR2` reader - Port x configuration bits (y = 0..15)
pub use PUPDR0_R as PUPDR2_R;
///Field `PUPDR3` reader - Port x configuration bits (y = 0..15)
pub use PUPDR0_R as PUPDR3_R;
///Field `PUPDR4` reader - Port x configuration bits (y = 0..15)
pub use PUPDR0_R as PUPDR4_R;
///Field `PUPDR5` reader - Port x configuration bits (y = 0..15)
pub use PUPDR0_R as PUPDR5_R;
///Field `PUPDR6` reader - Port x configuration bits (y = 0..15)
pub use PUPDR0_R as PUPDR6_R;
///Field `PUPDR7` reader - Port x configuration bits (y = 0..15)
pub use PUPDR0_R as PUPDR7_R;
///Field `PUPDR8` reader - Port x configuration bits (y = 0..15)
pub use PUPDR0_R as PUPDR8_R;
///Field `PUPDR9` reader - Port x configuration bits (y = 0..15)
pub use PUPDR0_R as PUPDR9_R;
///Field `PUPDR10` reader - Port x configuration bits (y = 0..15)
pub use PUPDR0_R as PUPDR10_R;
///Field `PUPDR11` reader - Port x configuration bits (y = 0..15)
pub use PUPDR0_R as PUPDR11_R;
///Field `PUPDR12` reader - Port x configuration bits (y = 0..15)
pub use PUPDR0_R as PUPDR12_R;
///Field `PUPDR13` reader - Port x configuration bits (y = 0..15)
pub use PUPDR0_R as PUPDR13_R;
///Field `PUPDR14` reader - Port x configuration bits (y = 0..15)
pub use PUPDR0_R as PUPDR14_R;
///Field `PUPDR15` reader - Port x configuration bits (y = 0..15)
pub use PUPDR0_R as PUPDR15_R;
///Field `PUPDR1` writer - Port x configuration bits (y = 0..15)
pub use PUPDR0_W as PUPDR1_W;
///Field `PUPDR2` writer - Port x configuration bits (y = 0..15)
pub use PUPDR0_W as PUPDR2_W;
///Field `PUPDR3` writer - Port x configuration bits (y = 0..15)
pub use PUPDR0_W as PUPDR3_W;
///Field `PUPDR4` writer - Port x configuration bits (y = 0..15)
pub use PUPDR0_W as PUPDR4_W;
///Field `PUPDR5` writer - Port x configuration bits (y = 0..15)
pub use PUPDR0_W as PUPDR5_W;
///Field `PUPDR6` writer - Port x configuration bits (y = 0..15)
pub use PUPDR0_W as PUPDR6_W;
///Field `PUPDR7` writer - Port x configuration bits (y = 0..15)
pub use PUPDR0_W as PUPDR7_W;
///Field `PUPDR8` writer - Port x configuration bits (y = 0..15)
pub use PUPDR0_W as PUPDR8_W;
///Field `PUPDR9` writer - Port x configuration bits (y = 0..15)
pub use PUPDR0_W as PUPDR9_W;
///Field `PUPDR10` writer - Port x configuration bits (y = 0..15)
pub use PUPDR0_W as PUPDR10_W;
///Field `PUPDR11` writer - Port x configuration bits (y = 0..15)
pub use PUPDR0_W as PUPDR11_W;
///Field `PUPDR12` writer - Port x configuration bits (y = 0..15)
pub use PUPDR0_W as PUPDR12_W;
///Field `PUPDR13` writer - Port x configuration bits (y = 0..15)
pub use PUPDR0_W as PUPDR13_W;
///Field `PUPDR14` writer - Port x configuration bits (y = 0..15)
pub use PUPDR0_W as PUPDR14_W;
///Field `PUPDR15` writer - Port x configuration bits (y = 0..15)
pub use PUPDR0_W as PUPDR15_W;
impl R {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr0(&self) -> PUPDR0_R {
        PUPDR0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr1(&self) -> PUPDR1_R {
        PUPDR1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr2(&self) -> PUPDR2_R {
        PUPDR2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr3(&self) -> PUPDR3_R {
        PUPDR3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr4(&self) -> PUPDR4_R {
        PUPDR4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr5(&self) -> PUPDR5_R {
        PUPDR5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr6(&self) -> PUPDR6_R {
        PUPDR6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr7(&self) -> PUPDR7_R {
        PUPDR7_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr8(&self) -> PUPDR8_R {
        PUPDR8_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr9(&self) -> PUPDR9_R {
        PUPDR9_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr10(&self) -> PUPDR10_R {
        PUPDR10_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr11(&self) -> PUPDR11_R {
        PUPDR11_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr12(&self) -> PUPDR12_R {
        PUPDR12_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr13(&self) -> PUPDR13_R {
        PUPDR13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr14(&self) -> PUPDR14_R {
        PUPDR14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupdr15(&self) -> PUPDR15_R {
        PUPDR15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupdr0(&mut self) -> PUPDR0_W<0> {
        PUPDR0_W::new(self)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupdr1(&mut self) -> PUPDR1_W<2> {
        PUPDR1_W::new(self)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupdr2(&mut self) -> PUPDR2_W<4> {
        PUPDR2_W::new(self)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupdr3(&mut self) -> PUPDR3_W<6> {
        PUPDR3_W::new(self)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupdr4(&mut self) -> PUPDR4_W<8> {
        PUPDR4_W::new(self)
    }
    ///Bits 10:11 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupdr5(&mut self) -> PUPDR5_W<10> {
        PUPDR5_W::new(self)
    }
    ///Bits 12:13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupdr6(&mut self) -> PUPDR6_W<12> {
        PUPDR6_W::new(self)
    }
    ///Bits 14:15 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupdr7(&mut self) -> PUPDR7_W<14> {
        PUPDR7_W::new(self)
    }
    ///Bits 16:17 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupdr8(&mut self) -> PUPDR8_W<16> {
        PUPDR8_W::new(self)
    }
    ///Bits 18:19 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupdr9(&mut self) -> PUPDR9_W<18> {
        PUPDR9_W::new(self)
    }
    ///Bits 20:21 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupdr10(&mut self) -> PUPDR10_W<20> {
        PUPDR10_W::new(self)
    }
    ///Bits 22:23 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupdr11(&mut self) -> PUPDR11_W<22> {
        PUPDR11_W::new(self)
    }
    ///Bits 24:25 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupdr12(&mut self) -> PUPDR12_W<24> {
        PUPDR12_W::new(self)
    }
    ///Bits 26:27 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupdr13(&mut self) -> PUPDR13_W<26> {
        PUPDR13_W::new(self)
    }
    ///Bits 28:29 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupdr14(&mut self) -> PUPDR14_W<28> {
        PUPDR14_W::new(self)
    }
    ///Bits 30:31 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupdr15(&mut self) -> PUPDR15_W<30> {
        PUPDR15_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO port pull-up/pull-down register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pupdr](index.html) module
pub struct PUPDR_SPEC;
impl crate::RegisterSpec for PUPDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pupdr::R](R) reader structure
impl crate::Readable for PUPDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pupdr::W](W) writer structure
impl crate::Writable for PUPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PUPDR to value 0x0100
impl crate::Resettable for PUPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
