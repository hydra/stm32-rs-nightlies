///Register `SWIER1` reader
pub struct R(crate::R<SWIER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWIER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWIER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWIER1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SWIER1` writer
pub struct W(crate::W<SWIER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWIER1_SPEC>;
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
impl From<crate::W<SWIER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWIER1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SWI0` reader - Software rising edge event trigger on line
pub type SWI0_R = crate::BitReader<SWI0W_A>;
///Software rising edge event trigger on line
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWI0W_A {
    ///1: Generates an interrupt request
    Pend = 1,
}
impl From<SWI0W_A> for bool {
    #[inline(always)]
    fn from(variant: SWI0W_A) -> Self {
        variant as u8 != 0
    }
}
impl SWI0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SWI0W_A> {
        match self.bits {
            true => Some(SWI0W_A::Pend),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Pend`
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWI0W_A::Pend
    }
}
///Field `SWI0` writer - Software rising edge event trigger on line
pub type SWI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER1_SPEC, SWI0W_A, O>;
impl<'a, const O: u8> SWI0_W<'a, O> {
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI0W_A::Pend)
    }
}
///Field `SWI1` reader - Software rising edge event trigger on line
pub use SWI0_R as SWI1_R;
///Field `SWI2` reader - Software rising edge event trigger on line
pub use SWI0_R as SWI2_R;
///Field `SWI3` reader - Software rising edge event trigger on line
pub use SWI0_R as SWI3_R;
///Field `SWI4` reader - Software rising edge event trigger on line
pub use SWI0_R as SWI4_R;
///Field `SWI5` reader - Software rising edge event trigger on line
pub use SWI0_R as SWI5_R;
///Field `SWI6` reader - Software rising edge event trigger on line
pub use SWI0_R as SWI6_R;
///Field `SWI7` reader - Software rising edge event trigger on line
pub use SWI0_R as SWI7_R;
///Field `SWI8` reader - Software rising edge event trigger on line
pub use SWI0_R as SWI8_R;
///Field `SWI9` reader - Software rising edge event trigger on line
pub use SWI0_R as SWI9_R;
///Field `SWI10` reader - Software rising edge event trigger on line
pub use SWI0_R as SWI10_R;
///Field `SWI11` reader - Software rising edge event trigger on line
pub use SWI0_R as SWI11_R;
///Field `SWI12` reader - Software rising edge event trigger on line
pub use SWI0_R as SWI12_R;
///Field `SWI13` reader - Software rising edge event trigger on line
pub use SWI0_R as SWI13_R;
///Field `SWI14` reader - Software rising edge event trigger on line
pub use SWI0_R as SWI14_R;
///Field `SWI15` reader - Software rising edge event trigger on line
pub use SWI0_R as SWI15_R;
///Field `SWI16` reader - Software rising edge event trigger on line
pub use SWI0_R as SWI16_R;
///Field `SWI17` reader - Software rising edge event trigger on line
pub use SWI0_R as SWI17_R;
///Field `SWI18` reader - Software rising edge event trigger on line
pub use SWI0_R as SWI18_R;
///Field `SWI20` reader - Software rising edge event trigger on line
pub use SWI0_R as SWI20_R;
///Field `SWI1` writer - Software rising edge event trigger on line
pub use SWI0_W as SWI1_W;
///Field `SWI2` writer - Software rising edge event trigger on line
pub use SWI0_W as SWI2_W;
///Field `SWI3` writer - Software rising edge event trigger on line
pub use SWI0_W as SWI3_W;
///Field `SWI4` writer - Software rising edge event trigger on line
pub use SWI0_W as SWI4_W;
///Field `SWI5` writer - Software rising edge event trigger on line
pub use SWI0_W as SWI5_W;
///Field `SWI6` writer - Software rising edge event trigger on line
pub use SWI0_W as SWI6_W;
///Field `SWI7` writer - Software rising edge event trigger on line
pub use SWI0_W as SWI7_W;
///Field `SWI8` writer - Software rising edge event trigger on line
pub use SWI0_W as SWI8_W;
///Field `SWI9` writer - Software rising edge event trigger on line
pub use SWI0_W as SWI9_W;
///Field `SWI10` writer - Software rising edge event trigger on line
pub use SWI0_W as SWI10_W;
///Field `SWI11` writer - Software rising edge event trigger on line
pub use SWI0_W as SWI11_W;
///Field `SWI12` writer - Software rising edge event trigger on line
pub use SWI0_W as SWI12_W;
///Field `SWI13` writer - Software rising edge event trigger on line
pub use SWI0_W as SWI13_W;
///Field `SWI14` writer - Software rising edge event trigger on line
pub use SWI0_W as SWI14_W;
///Field `SWI15` writer - Software rising edge event trigger on line
pub use SWI0_W as SWI15_W;
///Field `SWI16` writer - Software rising edge event trigger on line
pub use SWI0_W as SWI16_W;
///Field `SWI17` writer - Software rising edge event trigger on line
pub use SWI0_W as SWI17_W;
///Field `SWI18` writer - Software rising edge event trigger on line
pub use SWI0_W as SWI18_W;
///Field `SWI20` writer - Software rising edge event trigger on line
pub use SWI0_W as SWI20_W;
impl R {
    ///Bit 0 - Software rising edge event trigger on line
    #[inline(always)]
    pub fn swi0(&self) -> SWI0_R {
        SWI0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Software rising edge event trigger on line
    #[inline(always)]
    pub fn swi1(&self) -> SWI1_R {
        SWI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Software rising edge event trigger on line
    #[inline(always)]
    pub fn swi2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Software rising edge event trigger on line
    #[inline(always)]
    pub fn swi3(&self) -> SWI3_R {
        SWI3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Software rising edge event trigger on line
    #[inline(always)]
    pub fn swi4(&self) -> SWI4_R {
        SWI4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Software rising edge event trigger on line
    #[inline(always)]
    pub fn swi5(&self) -> SWI5_R {
        SWI5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Software rising edge event trigger on line
    #[inline(always)]
    pub fn swi6(&self) -> SWI6_R {
        SWI6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Software rising edge event trigger on line
    #[inline(always)]
    pub fn swi7(&self) -> SWI7_R {
        SWI7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Software rising edge event trigger on line
    #[inline(always)]
    pub fn swi8(&self) -> SWI8_R {
        SWI8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Software rising edge event trigger on line
    #[inline(always)]
    pub fn swi9(&self) -> SWI9_R {
        SWI9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Software rising edge event trigger on line
    #[inline(always)]
    pub fn swi10(&self) -> SWI10_R {
        SWI10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Software rising edge event trigger on line
    #[inline(always)]
    pub fn swi11(&self) -> SWI11_R {
        SWI11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Software rising edge event trigger on line
    #[inline(always)]
    pub fn swi12(&self) -> SWI12_R {
        SWI12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Software rising edge event trigger on line
    #[inline(always)]
    pub fn swi13(&self) -> SWI13_R {
        SWI13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Software rising edge event trigger on line
    #[inline(always)]
    pub fn swi14(&self) -> SWI14_R {
        SWI14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Software rising edge event trigger on line
    #[inline(always)]
    pub fn swi15(&self) -> SWI15_R {
        SWI15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Software rising edge event trigger on line
    #[inline(always)]
    pub fn swi16(&self) -> SWI16_R {
        SWI16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Software rising edge event trigger on line
    #[inline(always)]
    pub fn swi17(&self) -> SWI17_R {
        SWI17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Software rising edge event trigger on line
    #[inline(always)]
    pub fn swi18(&self) -> SWI18_R {
        SWI18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - Software rising edge event trigger on line
    #[inline(always)]
    pub fn swi20(&self) -> SWI20_R {
        SWI20_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Software rising edge event trigger on line
    #[inline(always)]
    #[must_use]
    pub fn swi0(&mut self) -> SWI0_W<0> {
        SWI0_W::new(self)
    }
    ///Bit 1 - Software rising edge event trigger on line
    #[inline(always)]
    #[must_use]
    pub fn swi1(&mut self) -> SWI1_W<1> {
        SWI1_W::new(self)
    }
    ///Bit 2 - Software rising edge event trigger on line
    #[inline(always)]
    #[must_use]
    pub fn swi2(&mut self) -> SWI2_W<2> {
        SWI2_W::new(self)
    }
    ///Bit 3 - Software rising edge event trigger on line
    #[inline(always)]
    #[must_use]
    pub fn swi3(&mut self) -> SWI3_W<3> {
        SWI3_W::new(self)
    }
    ///Bit 4 - Software rising edge event trigger on line
    #[inline(always)]
    #[must_use]
    pub fn swi4(&mut self) -> SWI4_W<4> {
        SWI4_W::new(self)
    }
    ///Bit 5 - Software rising edge event trigger on line
    #[inline(always)]
    #[must_use]
    pub fn swi5(&mut self) -> SWI5_W<5> {
        SWI5_W::new(self)
    }
    ///Bit 6 - Software rising edge event trigger on line
    #[inline(always)]
    #[must_use]
    pub fn swi6(&mut self) -> SWI6_W<6> {
        SWI6_W::new(self)
    }
    ///Bit 7 - Software rising edge event trigger on line
    #[inline(always)]
    #[must_use]
    pub fn swi7(&mut self) -> SWI7_W<7> {
        SWI7_W::new(self)
    }
    ///Bit 8 - Software rising edge event trigger on line
    #[inline(always)]
    #[must_use]
    pub fn swi8(&mut self) -> SWI8_W<8> {
        SWI8_W::new(self)
    }
    ///Bit 9 - Software rising edge event trigger on line
    #[inline(always)]
    #[must_use]
    pub fn swi9(&mut self) -> SWI9_W<9> {
        SWI9_W::new(self)
    }
    ///Bit 10 - Software rising edge event trigger on line
    #[inline(always)]
    #[must_use]
    pub fn swi10(&mut self) -> SWI10_W<10> {
        SWI10_W::new(self)
    }
    ///Bit 11 - Software rising edge event trigger on line
    #[inline(always)]
    #[must_use]
    pub fn swi11(&mut self) -> SWI11_W<11> {
        SWI11_W::new(self)
    }
    ///Bit 12 - Software rising edge event trigger on line
    #[inline(always)]
    #[must_use]
    pub fn swi12(&mut self) -> SWI12_W<12> {
        SWI12_W::new(self)
    }
    ///Bit 13 - Software rising edge event trigger on line
    #[inline(always)]
    #[must_use]
    pub fn swi13(&mut self) -> SWI13_W<13> {
        SWI13_W::new(self)
    }
    ///Bit 14 - Software rising edge event trigger on line
    #[inline(always)]
    #[must_use]
    pub fn swi14(&mut self) -> SWI14_W<14> {
        SWI14_W::new(self)
    }
    ///Bit 15 - Software rising edge event trigger on line
    #[inline(always)]
    #[must_use]
    pub fn swi15(&mut self) -> SWI15_W<15> {
        SWI15_W::new(self)
    }
    ///Bit 16 - Software rising edge event trigger on line
    #[inline(always)]
    #[must_use]
    pub fn swi16(&mut self) -> SWI16_W<16> {
        SWI16_W::new(self)
    }
    ///Bit 17 - Software rising edge event trigger on line
    #[inline(always)]
    #[must_use]
    pub fn swi17(&mut self) -> SWI17_W<17> {
        SWI17_W::new(self)
    }
    ///Bit 18 - Software rising edge event trigger on line
    #[inline(always)]
    #[must_use]
    pub fn swi18(&mut self) -> SWI18_W<18> {
        SWI18_W::new(self)
    }
    ///Bit 20 - Software rising edge event trigger on line
    #[inline(always)]
    #[must_use]
    pub fn swi20(&mut self) -> SWI20_W<20> {
        SWI20_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI software interrupt event register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [swier1](index.html) module
pub struct SWIER1_SPEC;
impl crate::RegisterSpec for SWIER1_SPEC {
    type Ux = u32;
}
///`read()` method returns [swier1::R](R) reader structure
impl crate::Readable for SWIER1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [swier1::W](W) writer structure
impl crate::Writable for SWIER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SWIER1 to value 0
impl crate::Resettable for SWIER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
