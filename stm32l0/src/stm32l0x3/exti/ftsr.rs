///Register `FTSR` reader
pub struct R(crate::R<FTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FTSR` writer
pub struct W(crate::W<FTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTSR_SPEC>;
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
impl From<crate::W<FTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FT0` reader - Falling trigger event configuration of line 0
pub type FT0_R = crate::BitReader<FT0_A>;
///Falling trigger event configuration of line 0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FT0_A {
    ///0: Falling edge trigger is disabled
    Disabled = 0,
    ///1: Falling edge trigger is enabled
    Enabled = 1,
}
impl From<FT0_A> for bool {
    #[inline(always)]
    fn from(variant: FT0_A) -> Self {
        variant as u8 != 0
    }
}
impl FT0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FT0_A {
        match self.bits {
            false => FT0_A::Disabled,
            true => FT0_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FT0_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FT0_A::Enabled
    }
}
///Field `FT0` writer - Falling trigger event configuration of line 0
pub type FT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FTSR_SPEC, FT0_A, O>;
impl<'a, const O: u8> FT0_W<'a, O> {
    ///Falling edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FT0_A::Disabled)
    }
    ///Falling edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FT0_A::Enabled)
    }
}
///Field `FT1` reader - Falling trigger event configuration of line 1
pub use FT0_R as FT1_R;
///Field `FT2` reader - Falling trigger event configuration of line 2
pub use FT0_R as FT2_R;
///Field `FT3` reader - Falling trigger event configuration of line 3
pub use FT0_R as FT3_R;
///Field `FT4` reader - Falling trigger event configuration of line 4
pub use FT0_R as FT4_R;
///Field `FT5` reader - Falling trigger event configuration of line 5
pub use FT0_R as FT5_R;
///Field `FT6` reader - Falling trigger event configuration of line 6
pub use FT0_R as FT6_R;
///Field `FT7` reader - Falling trigger event configuration of line 7
pub use FT0_R as FT7_R;
///Field `FT8` reader - Falling trigger event configuration of line 8
pub use FT0_R as FT8_R;
///Field `FT9` reader - Falling trigger event configuration of line 9
pub use FT0_R as FT9_R;
///Field `FT10` reader - Falling trigger event configuration of line 10
pub use FT0_R as FT10_R;
///Field `FT11` reader - Falling trigger event configuration of line 11
pub use FT0_R as FT11_R;
///Field `FT12` reader - Falling trigger event configuration of line 12
pub use FT0_R as FT12_R;
///Field `FT13` reader - Falling trigger event configuration of line 13
pub use FT0_R as FT13_R;
///Field `FT14` reader - Falling trigger event configuration of line 14
pub use FT0_R as FT14_R;
///Field `FT15` reader - Falling trigger event configuration of line 15
pub use FT0_R as FT15_R;
///Field `FT16` reader - Falling trigger event configuration of line 16
pub use FT0_R as FT16_R;
///Field `FT17` reader - Falling trigger event configuration of line 17
pub use FT0_R as FT17_R;
///Field `FT19` reader - Falling trigger event configuration of line 19
pub use FT0_R as FT19_R;
///Field `FT20` reader - Falling trigger event configuration of line 20
pub use FT0_R as FT20_R;
///Field `FT21` reader - Falling trigger event configuration of line 21
pub use FT0_R as FT21_R;
///Field `FT22` reader - Falling trigger event configuration of line 22
pub use FT0_R as FT22_R;
///Field `FT1` writer - Falling trigger event configuration of line 1
pub use FT0_W as FT1_W;
///Field `FT2` writer - Falling trigger event configuration of line 2
pub use FT0_W as FT2_W;
///Field `FT3` writer - Falling trigger event configuration of line 3
pub use FT0_W as FT3_W;
///Field `FT4` writer - Falling trigger event configuration of line 4
pub use FT0_W as FT4_W;
///Field `FT5` writer - Falling trigger event configuration of line 5
pub use FT0_W as FT5_W;
///Field `FT6` writer - Falling trigger event configuration of line 6
pub use FT0_W as FT6_W;
///Field `FT7` writer - Falling trigger event configuration of line 7
pub use FT0_W as FT7_W;
///Field `FT8` writer - Falling trigger event configuration of line 8
pub use FT0_W as FT8_W;
///Field `FT9` writer - Falling trigger event configuration of line 9
pub use FT0_W as FT9_W;
///Field `FT10` writer - Falling trigger event configuration of line 10
pub use FT0_W as FT10_W;
///Field `FT11` writer - Falling trigger event configuration of line 11
pub use FT0_W as FT11_W;
///Field `FT12` writer - Falling trigger event configuration of line 12
pub use FT0_W as FT12_W;
///Field `FT13` writer - Falling trigger event configuration of line 13
pub use FT0_W as FT13_W;
///Field `FT14` writer - Falling trigger event configuration of line 14
pub use FT0_W as FT14_W;
///Field `FT15` writer - Falling trigger event configuration of line 15
pub use FT0_W as FT15_W;
///Field `FT16` writer - Falling trigger event configuration of line 16
pub use FT0_W as FT16_W;
///Field `FT17` writer - Falling trigger event configuration of line 17
pub use FT0_W as FT17_W;
///Field `FT19` writer - Falling trigger event configuration of line 19
pub use FT0_W as FT19_W;
///Field `FT20` writer - Falling trigger event configuration of line 20
pub use FT0_W as FT20_W;
///Field `FT21` writer - Falling trigger event configuration of line 21
pub use FT0_W as FT21_W;
///Field `FT22` writer - Falling trigger event configuration of line 22
pub use FT0_W as FT22_W;
impl R {
    ///Bit 0 - Falling trigger event configuration of line 0
    #[inline(always)]
    pub fn ft0(&self) -> FT0_R {
        FT0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Falling trigger event configuration of line 1
    #[inline(always)]
    pub fn ft1(&self) -> FT1_R {
        FT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Falling trigger event configuration of line 2
    #[inline(always)]
    pub fn ft2(&self) -> FT2_R {
        FT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Falling trigger event configuration of line 3
    #[inline(always)]
    pub fn ft3(&self) -> FT3_R {
        FT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Falling trigger event configuration of line 4
    #[inline(always)]
    pub fn ft4(&self) -> FT4_R {
        FT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Falling trigger event configuration of line 5
    #[inline(always)]
    pub fn ft5(&self) -> FT5_R {
        FT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Falling trigger event configuration of line 6
    #[inline(always)]
    pub fn ft6(&self) -> FT6_R {
        FT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Falling trigger event configuration of line 7
    #[inline(always)]
    pub fn ft7(&self) -> FT7_R {
        FT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Falling trigger event configuration of line 8
    #[inline(always)]
    pub fn ft8(&self) -> FT8_R {
        FT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Falling trigger event configuration of line 9
    #[inline(always)]
    pub fn ft9(&self) -> FT9_R {
        FT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Falling trigger event configuration of line 10
    #[inline(always)]
    pub fn ft10(&self) -> FT10_R {
        FT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Falling trigger event configuration of line 11
    #[inline(always)]
    pub fn ft11(&self) -> FT11_R {
        FT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Falling trigger event configuration of line 12
    #[inline(always)]
    pub fn ft12(&self) -> FT12_R {
        FT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Falling trigger event configuration of line 13
    #[inline(always)]
    pub fn ft13(&self) -> FT13_R {
        FT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Falling trigger event configuration of line 14
    #[inline(always)]
    pub fn ft14(&self) -> FT14_R {
        FT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Falling trigger event configuration of line 15
    #[inline(always)]
    pub fn ft15(&self) -> FT15_R {
        FT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Falling trigger event configuration of line 16
    #[inline(always)]
    pub fn ft16(&self) -> FT16_R {
        FT16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Falling trigger event configuration of line 17
    #[inline(always)]
    pub fn ft17(&self) -> FT17_R {
        FT17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - Falling trigger event configuration of line 19
    #[inline(always)]
    pub fn ft19(&self) -> FT19_R {
        FT19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Falling trigger event configuration of line 20
    #[inline(always)]
    pub fn ft20(&self) -> FT20_R {
        FT20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Falling trigger event configuration of line 21
    #[inline(always)]
    pub fn ft21(&self) -> FT21_R {
        FT21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Falling trigger event configuration of line 22
    #[inline(always)]
    pub fn ft22(&self) -> FT22_R {
        FT22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Falling trigger event configuration of line 0
    #[inline(always)]
    #[must_use]
    pub fn ft0(&mut self) -> FT0_W<0> {
        FT0_W::new(self)
    }
    ///Bit 1 - Falling trigger event configuration of line 1
    #[inline(always)]
    #[must_use]
    pub fn ft1(&mut self) -> FT1_W<1> {
        FT1_W::new(self)
    }
    ///Bit 2 - Falling trigger event configuration of line 2
    #[inline(always)]
    #[must_use]
    pub fn ft2(&mut self) -> FT2_W<2> {
        FT2_W::new(self)
    }
    ///Bit 3 - Falling trigger event configuration of line 3
    #[inline(always)]
    #[must_use]
    pub fn ft3(&mut self) -> FT3_W<3> {
        FT3_W::new(self)
    }
    ///Bit 4 - Falling trigger event configuration of line 4
    #[inline(always)]
    #[must_use]
    pub fn ft4(&mut self) -> FT4_W<4> {
        FT4_W::new(self)
    }
    ///Bit 5 - Falling trigger event configuration of line 5
    #[inline(always)]
    #[must_use]
    pub fn ft5(&mut self) -> FT5_W<5> {
        FT5_W::new(self)
    }
    ///Bit 6 - Falling trigger event configuration of line 6
    #[inline(always)]
    #[must_use]
    pub fn ft6(&mut self) -> FT6_W<6> {
        FT6_W::new(self)
    }
    ///Bit 7 - Falling trigger event configuration of line 7
    #[inline(always)]
    #[must_use]
    pub fn ft7(&mut self) -> FT7_W<7> {
        FT7_W::new(self)
    }
    ///Bit 8 - Falling trigger event configuration of line 8
    #[inline(always)]
    #[must_use]
    pub fn ft8(&mut self) -> FT8_W<8> {
        FT8_W::new(self)
    }
    ///Bit 9 - Falling trigger event configuration of line 9
    #[inline(always)]
    #[must_use]
    pub fn ft9(&mut self) -> FT9_W<9> {
        FT9_W::new(self)
    }
    ///Bit 10 - Falling trigger event configuration of line 10
    #[inline(always)]
    #[must_use]
    pub fn ft10(&mut self) -> FT10_W<10> {
        FT10_W::new(self)
    }
    ///Bit 11 - Falling trigger event configuration of line 11
    #[inline(always)]
    #[must_use]
    pub fn ft11(&mut self) -> FT11_W<11> {
        FT11_W::new(self)
    }
    ///Bit 12 - Falling trigger event configuration of line 12
    #[inline(always)]
    #[must_use]
    pub fn ft12(&mut self) -> FT12_W<12> {
        FT12_W::new(self)
    }
    ///Bit 13 - Falling trigger event configuration of line 13
    #[inline(always)]
    #[must_use]
    pub fn ft13(&mut self) -> FT13_W<13> {
        FT13_W::new(self)
    }
    ///Bit 14 - Falling trigger event configuration of line 14
    #[inline(always)]
    #[must_use]
    pub fn ft14(&mut self) -> FT14_W<14> {
        FT14_W::new(self)
    }
    ///Bit 15 - Falling trigger event configuration of line 15
    #[inline(always)]
    #[must_use]
    pub fn ft15(&mut self) -> FT15_W<15> {
        FT15_W::new(self)
    }
    ///Bit 16 - Falling trigger event configuration of line 16
    #[inline(always)]
    #[must_use]
    pub fn ft16(&mut self) -> FT16_W<16> {
        FT16_W::new(self)
    }
    ///Bit 17 - Falling trigger event configuration of line 17
    #[inline(always)]
    #[must_use]
    pub fn ft17(&mut self) -> FT17_W<17> {
        FT17_W::new(self)
    }
    ///Bit 19 - Falling trigger event configuration of line 19
    #[inline(always)]
    #[must_use]
    pub fn ft19(&mut self) -> FT19_W<19> {
        FT19_W::new(self)
    }
    ///Bit 20 - Falling trigger event configuration of line 20
    #[inline(always)]
    #[must_use]
    pub fn ft20(&mut self) -> FT20_W<20> {
        FT20_W::new(self)
    }
    ///Bit 21 - Falling trigger event configuration of line 21
    #[inline(always)]
    #[must_use]
    pub fn ft21(&mut self) -> FT21_W<21> {
        FT21_W::new(self)
    }
    ///Bit 22 - Falling trigger event configuration of line 22
    #[inline(always)]
    #[must_use]
    pub fn ft22(&mut self) -> FT22_W<22> {
        FT22_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Falling Trigger selection register (EXTI_FTSR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ftsr](index.html) module
pub struct FTSR_SPEC;
impl crate::RegisterSpec for FTSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ftsr::R](R) reader structure
impl crate::Readable for FTSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ftsr::W](W) writer structure
impl crate::Writable for FTSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FTSR to value 0
impl crate::Resettable for FTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
