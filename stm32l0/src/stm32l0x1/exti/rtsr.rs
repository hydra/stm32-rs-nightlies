///Register `RTSR` reader
pub struct R(crate::R<RTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RTSR` writer
pub struct W(crate::W<RTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTSR_SPEC>;
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
impl From<crate::W<RTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RT0` reader - Rising trigger event configuration of line 0
pub type RT0_R = crate::BitReader<RT0_A>;
///Rising trigger event configuration of line 0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RT0_A {
    ///0: Rising edge trigger is disabled
    Disabled = 0,
    ///1: Rising edge trigger is enabled
    Enabled = 1,
}
impl From<RT0_A> for bool {
    #[inline(always)]
    fn from(variant: RT0_A) -> Self {
        variant as u8 != 0
    }
}
impl RT0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RT0_A {
        match self.bits {
            false => RT0_A::Disabled,
            true => RT0_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RT0_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RT0_A::Enabled
    }
}
///Field `RT0` writer - Rising trigger event configuration of line 0
pub type RT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTSR_SPEC, RT0_A, O>;
impl<'a, const O: u8> RT0_W<'a, O> {
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RT0_A::Disabled)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RT0_A::Enabled)
    }
}
///Field `RT1` reader - Rising trigger event configuration of line 1
pub use RT0_R as RT1_R;
///Field `RT2` reader - Rising trigger event configuration of line 2
pub use RT0_R as RT2_R;
///Field `RT3` reader - Rising trigger event configuration of line 3
pub use RT0_R as RT3_R;
///Field `RT4` reader - Rising trigger event configuration of line 4
pub use RT0_R as RT4_R;
///Field `RT5` reader - Rising trigger event configuration of line 5
pub use RT0_R as RT5_R;
///Field `RT6` reader - Rising trigger event configuration of line 6
pub use RT0_R as RT6_R;
///Field `RT7` reader - Rising trigger event configuration of line 7
pub use RT0_R as RT7_R;
///Field `RT8` reader - Rising trigger event configuration of line 8
pub use RT0_R as RT8_R;
///Field `RT9` reader - Rising trigger event configuration of line 9
pub use RT0_R as RT9_R;
///Field `RT10` reader - Rising trigger event configuration of line 10
pub use RT0_R as RT10_R;
///Field `RT11` reader - Rising trigger event configuration of line 11
pub use RT0_R as RT11_R;
///Field `RT12` reader - Rising trigger event configuration of line 12
pub use RT0_R as RT12_R;
///Field `RT13` reader - Rising trigger event configuration of line 13
pub use RT0_R as RT13_R;
///Field `RT14` reader - Rising trigger event configuration of line 14
pub use RT0_R as RT14_R;
///Field `RT15` reader - Rising trigger event configuration of line 15
pub use RT0_R as RT15_R;
///Field `RT16` reader - Rising trigger event configuration of line 16
pub use RT0_R as RT16_R;
///Field `RT17` reader - Rising trigger event configuration of line 17
pub use RT0_R as RT17_R;
///Field `RT19` reader - Rising trigger event configuration of line 19
pub use RT0_R as RT19_R;
///Field `RT20` reader - Rising trigger event configuration of line 20
pub use RT0_R as RT20_R;
///Field `RT21` reader - Rising trigger event configuration of line 21
pub use RT0_R as RT21_R;
///Field `RT22` reader - Rising trigger event configuration of line 22
pub use RT0_R as RT22_R;
///Field `RT1` writer - Rising trigger event configuration of line 1
pub use RT0_W as RT1_W;
///Field `RT2` writer - Rising trigger event configuration of line 2
pub use RT0_W as RT2_W;
///Field `RT3` writer - Rising trigger event configuration of line 3
pub use RT0_W as RT3_W;
///Field `RT4` writer - Rising trigger event configuration of line 4
pub use RT0_W as RT4_W;
///Field `RT5` writer - Rising trigger event configuration of line 5
pub use RT0_W as RT5_W;
///Field `RT6` writer - Rising trigger event configuration of line 6
pub use RT0_W as RT6_W;
///Field `RT7` writer - Rising trigger event configuration of line 7
pub use RT0_W as RT7_W;
///Field `RT8` writer - Rising trigger event configuration of line 8
pub use RT0_W as RT8_W;
///Field `RT9` writer - Rising trigger event configuration of line 9
pub use RT0_W as RT9_W;
///Field `RT10` writer - Rising trigger event configuration of line 10
pub use RT0_W as RT10_W;
///Field `RT11` writer - Rising trigger event configuration of line 11
pub use RT0_W as RT11_W;
///Field `RT12` writer - Rising trigger event configuration of line 12
pub use RT0_W as RT12_W;
///Field `RT13` writer - Rising trigger event configuration of line 13
pub use RT0_W as RT13_W;
///Field `RT14` writer - Rising trigger event configuration of line 14
pub use RT0_W as RT14_W;
///Field `RT15` writer - Rising trigger event configuration of line 15
pub use RT0_W as RT15_W;
///Field `RT16` writer - Rising trigger event configuration of line 16
pub use RT0_W as RT16_W;
///Field `RT17` writer - Rising trigger event configuration of line 17
pub use RT0_W as RT17_W;
///Field `RT19` writer - Rising trigger event configuration of line 19
pub use RT0_W as RT19_W;
///Field `RT20` writer - Rising trigger event configuration of line 20
pub use RT0_W as RT20_W;
///Field `RT21` writer - Rising trigger event configuration of line 21
pub use RT0_W as RT21_W;
///Field `RT22` writer - Rising trigger event configuration of line 22
pub use RT0_W as RT22_W;
impl R {
    ///Bit 0 - Rising trigger event configuration of line 0
    #[inline(always)]
    pub fn rt0(&self) -> RT0_R {
        RT0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Rising trigger event configuration of line 1
    #[inline(always)]
    pub fn rt1(&self) -> RT1_R {
        RT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rising trigger event configuration of line 2
    #[inline(always)]
    pub fn rt2(&self) -> RT2_R {
        RT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Rising trigger event configuration of line 3
    #[inline(always)]
    pub fn rt3(&self) -> RT3_R {
        RT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rising trigger event configuration of line 4
    #[inline(always)]
    pub fn rt4(&self) -> RT4_R {
        RT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rising trigger event configuration of line 5
    #[inline(always)]
    pub fn rt5(&self) -> RT5_R {
        RT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rising trigger event configuration of line 6
    #[inline(always)]
    pub fn rt6(&self) -> RT6_R {
        RT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Rising trigger event configuration of line 7
    #[inline(always)]
    pub fn rt7(&self) -> RT7_R {
        RT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Rising trigger event configuration of line 8
    #[inline(always)]
    pub fn rt8(&self) -> RT8_R {
        RT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Rising trigger event configuration of line 9
    #[inline(always)]
    pub fn rt9(&self) -> RT9_R {
        RT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Rising trigger event configuration of line 10
    #[inline(always)]
    pub fn rt10(&self) -> RT10_R {
        RT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Rising trigger event configuration of line 11
    #[inline(always)]
    pub fn rt11(&self) -> RT11_R {
        RT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Rising trigger event configuration of line 12
    #[inline(always)]
    pub fn rt12(&self) -> RT12_R {
        RT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Rising trigger event configuration of line 13
    #[inline(always)]
    pub fn rt13(&self) -> RT13_R {
        RT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Rising trigger event configuration of line 14
    #[inline(always)]
    pub fn rt14(&self) -> RT14_R {
        RT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Rising trigger event configuration of line 15
    #[inline(always)]
    pub fn rt15(&self) -> RT15_R {
        RT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Rising trigger event configuration of line 16
    #[inline(always)]
    pub fn rt16(&self) -> RT16_R {
        RT16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Rising trigger event configuration of line 17
    #[inline(always)]
    pub fn rt17(&self) -> RT17_R {
        RT17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - Rising trigger event configuration of line 19
    #[inline(always)]
    pub fn rt19(&self) -> RT19_R {
        RT19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Rising trigger event configuration of line 20
    #[inline(always)]
    pub fn rt20(&self) -> RT20_R {
        RT20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Rising trigger event configuration of line 21
    #[inline(always)]
    pub fn rt21(&self) -> RT21_R {
        RT21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Rising trigger event configuration of line 22
    #[inline(always)]
    pub fn rt22(&self) -> RT22_R {
        RT22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Rising trigger event configuration of line 0
    #[inline(always)]
    #[must_use]
    pub fn rt0(&mut self) -> RT0_W<0> {
        RT0_W::new(self)
    }
    ///Bit 1 - Rising trigger event configuration of line 1
    #[inline(always)]
    #[must_use]
    pub fn rt1(&mut self) -> RT1_W<1> {
        RT1_W::new(self)
    }
    ///Bit 2 - Rising trigger event configuration of line 2
    #[inline(always)]
    #[must_use]
    pub fn rt2(&mut self) -> RT2_W<2> {
        RT2_W::new(self)
    }
    ///Bit 3 - Rising trigger event configuration of line 3
    #[inline(always)]
    #[must_use]
    pub fn rt3(&mut self) -> RT3_W<3> {
        RT3_W::new(self)
    }
    ///Bit 4 - Rising trigger event configuration of line 4
    #[inline(always)]
    #[must_use]
    pub fn rt4(&mut self) -> RT4_W<4> {
        RT4_W::new(self)
    }
    ///Bit 5 - Rising trigger event configuration of line 5
    #[inline(always)]
    #[must_use]
    pub fn rt5(&mut self) -> RT5_W<5> {
        RT5_W::new(self)
    }
    ///Bit 6 - Rising trigger event configuration of line 6
    #[inline(always)]
    #[must_use]
    pub fn rt6(&mut self) -> RT6_W<6> {
        RT6_W::new(self)
    }
    ///Bit 7 - Rising trigger event configuration of line 7
    #[inline(always)]
    #[must_use]
    pub fn rt7(&mut self) -> RT7_W<7> {
        RT7_W::new(self)
    }
    ///Bit 8 - Rising trigger event configuration of line 8
    #[inline(always)]
    #[must_use]
    pub fn rt8(&mut self) -> RT8_W<8> {
        RT8_W::new(self)
    }
    ///Bit 9 - Rising trigger event configuration of line 9
    #[inline(always)]
    #[must_use]
    pub fn rt9(&mut self) -> RT9_W<9> {
        RT9_W::new(self)
    }
    ///Bit 10 - Rising trigger event configuration of line 10
    #[inline(always)]
    #[must_use]
    pub fn rt10(&mut self) -> RT10_W<10> {
        RT10_W::new(self)
    }
    ///Bit 11 - Rising trigger event configuration of line 11
    #[inline(always)]
    #[must_use]
    pub fn rt11(&mut self) -> RT11_W<11> {
        RT11_W::new(self)
    }
    ///Bit 12 - Rising trigger event configuration of line 12
    #[inline(always)]
    #[must_use]
    pub fn rt12(&mut self) -> RT12_W<12> {
        RT12_W::new(self)
    }
    ///Bit 13 - Rising trigger event configuration of line 13
    #[inline(always)]
    #[must_use]
    pub fn rt13(&mut self) -> RT13_W<13> {
        RT13_W::new(self)
    }
    ///Bit 14 - Rising trigger event configuration of line 14
    #[inline(always)]
    #[must_use]
    pub fn rt14(&mut self) -> RT14_W<14> {
        RT14_W::new(self)
    }
    ///Bit 15 - Rising trigger event configuration of line 15
    #[inline(always)]
    #[must_use]
    pub fn rt15(&mut self) -> RT15_W<15> {
        RT15_W::new(self)
    }
    ///Bit 16 - Rising trigger event configuration of line 16
    #[inline(always)]
    #[must_use]
    pub fn rt16(&mut self) -> RT16_W<16> {
        RT16_W::new(self)
    }
    ///Bit 17 - Rising trigger event configuration of line 17
    #[inline(always)]
    #[must_use]
    pub fn rt17(&mut self) -> RT17_W<17> {
        RT17_W::new(self)
    }
    ///Bit 19 - Rising trigger event configuration of line 19
    #[inline(always)]
    #[must_use]
    pub fn rt19(&mut self) -> RT19_W<19> {
        RT19_W::new(self)
    }
    ///Bit 20 - Rising trigger event configuration of line 20
    #[inline(always)]
    #[must_use]
    pub fn rt20(&mut self) -> RT20_W<20> {
        RT20_W::new(self)
    }
    ///Bit 21 - Rising trigger event configuration of line 21
    #[inline(always)]
    #[must_use]
    pub fn rt21(&mut self) -> RT21_W<21> {
        RT21_W::new(self)
    }
    ///Bit 22 - Rising trigger event configuration of line 22
    #[inline(always)]
    #[must_use]
    pub fn rt22(&mut self) -> RT22_W<22> {
        RT22_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Rising Trigger selection register (EXTI_RTSR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rtsr](index.html) module
pub struct RTSR_SPEC;
impl crate::RegisterSpec for RTSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rtsr::R](R) reader structure
impl crate::Readable for RTSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rtsr::W](W) writer structure
impl crate::Writable for RTSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RTSR to value 0
impl crate::Resettable for RTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
