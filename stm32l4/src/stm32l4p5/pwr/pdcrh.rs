///Register `PDCRH` reader
pub struct R(crate::R<PDCRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDCRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDCRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDCRH_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PDCRH` writer
pub struct W(crate::W<PDCRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDCRH_SPEC>;
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
impl From<crate::W<PDCRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDCRH_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PD0` reader - Port H pull-down bit y (y=0..15)
pub type PD0_R = crate::BitReader<PD0_A>;
///Port H pull-down bit y (y=0..15)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD0_A {
    ///0: Pull-Down on Pxx is disabled
    Disabled = 0,
    ///1: Pull-Down on Pxx is enabled
    Enabled = 1,
}
impl From<PD0_A> for bool {
    #[inline(always)]
    fn from(variant: PD0_A) -> Self {
        variant as u8 != 0
    }
}
impl PD0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PD0_A {
        match self.bits {
            false => PD0_A::Disabled,
            true => PD0_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PD0_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PD0_A::Enabled
    }
}
///Field `PD0` writer - Port H pull-down bit y (y=0..15)
pub type PD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRH_SPEC, PD0_A, O>;
impl<'a, const O: u8> PD0_W<'a, O> {
    ///Pull-Down on Pxx is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD0_A::Disabled)
    }
    ///Pull-Down on Pxx is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD0_A::Enabled)
    }
}
///Field `PD1` reader - Port H pull-down bit y (y=0..15)
pub use PD0_R as PD1_R;
///Field `PD2` reader - Port H pull-down bit y (y=0..15)
pub use PD0_R as PD2_R;
///Field `PD3` reader - Port H pull-down bit y (y=0..15)
pub use PD0_R as PD3_R;
///Field `PD4` reader - Port H pull-down bit y (y=0..15)
pub use PD0_R as PD4_R;
///Field `PD5` reader - Port H pull-down bit y (y=0..15)
pub use PD0_R as PD5_R;
///Field `PD6` reader - Port H pull-down bit y (y=0..15)
pub use PD0_R as PD6_R;
///Field `PD7` reader - Port H pull-down bit y (y=0..15)
pub use PD0_R as PD7_R;
///Field `PD8` reader - Port H pull-down bit y (y=0..15)
pub use PD0_R as PD8_R;
///Field `PD9` reader - Port H pull-down bit y (y=0..15)
pub use PD0_R as PD9_R;
///Field `PD10` reader - Port H pull-down bit y (y=0..15)
pub use PD0_R as PD10_R;
///Field `PD11` reader - Port H pull-down bit y (y=0..15)
pub use PD0_R as PD11_R;
///Field `PD12` reader - Port H pull-down bit y (y=0..15)
pub use PD0_R as PD12_R;
///Field `PD13` reader - Port H pull-down bit y (y=0..15)
pub use PD0_R as PD13_R;
///Field `PD14` reader - Port H pull-down bit y (y=0..15)
pub use PD0_R as PD14_R;
///Field `PD15` reader - Port H pull-down bit y (y=0..15)
pub use PD0_R as PD15_R;
///Field `PD1` writer - Port H pull-down bit y (y=0..15)
pub use PD0_W as PD1_W;
///Field `PD2` writer - Port H pull-down bit y (y=0..15)
pub use PD0_W as PD2_W;
///Field `PD3` writer - Port H pull-down bit y (y=0..15)
pub use PD0_W as PD3_W;
///Field `PD4` writer - Port H pull-down bit y (y=0..15)
pub use PD0_W as PD4_W;
///Field `PD5` writer - Port H pull-down bit y (y=0..15)
pub use PD0_W as PD5_W;
///Field `PD6` writer - Port H pull-down bit y (y=0..15)
pub use PD0_W as PD6_W;
///Field `PD7` writer - Port H pull-down bit y (y=0..15)
pub use PD0_W as PD7_W;
///Field `PD8` writer - Port H pull-down bit y (y=0..15)
pub use PD0_W as PD8_W;
///Field `PD9` writer - Port H pull-down bit y (y=0..15)
pub use PD0_W as PD9_W;
///Field `PD10` writer - Port H pull-down bit y (y=0..15)
pub use PD0_W as PD10_W;
///Field `PD11` writer - Port H pull-down bit y (y=0..15)
pub use PD0_W as PD11_W;
///Field `PD12` writer - Port H pull-down bit y (y=0..15)
pub use PD0_W as PD12_W;
///Field `PD13` writer - Port H pull-down bit y (y=0..15)
pub use PD0_W as PD13_W;
///Field `PD14` writer - Port H pull-down bit y (y=0..15)
pub use PD0_W as PD14_W;
///Field `PD15` writer - Port H pull-down bit y (y=0..15)
pub use PD0_W as PD15_W;
impl R {
    ///Bit 0 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd8(&self) -> PD8_R {
        PD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd9(&self) -> PD9_R {
        PD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd10(&self) -> PD10_R {
        PD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd11(&self) -> PD11_R {
        PD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd12(&self) -> PD12_R {
        PD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd13(&self) -> PD13_R {
        PD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd14(&self) -> PD14_R {
        PD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd15(&self) -> PD15_R {
        PD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> PD0_W<0> {
        PD0_W::new(self)
    }
    ///Bit 1 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> PD1_W<1> {
        PD1_W::new(self)
    }
    ///Bit 2 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> PD2_W<2> {
        PD2_W::new(self)
    }
    ///Bit 3 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> PD3_W<3> {
        PD3_W::new(self)
    }
    ///Bit 4 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd4(&mut self) -> PD4_W<4> {
        PD4_W::new(self)
    }
    ///Bit 5 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd5(&mut self) -> PD5_W<5> {
        PD5_W::new(self)
    }
    ///Bit 6 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd6(&mut self) -> PD6_W<6> {
        PD6_W::new(self)
    }
    ///Bit 7 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd7(&mut self) -> PD7_W<7> {
        PD7_W::new(self)
    }
    ///Bit 8 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd8(&mut self) -> PD8_W<8> {
        PD8_W::new(self)
    }
    ///Bit 9 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd9(&mut self) -> PD9_W<9> {
        PD9_W::new(self)
    }
    ///Bit 10 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd10(&mut self) -> PD10_W<10> {
        PD10_W::new(self)
    }
    ///Bit 11 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd11(&mut self) -> PD11_W<11> {
        PD11_W::new(self)
    }
    ///Bit 12 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd12(&mut self) -> PD12_W<12> {
        PD12_W::new(self)
    }
    ///Bit 13 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd13(&mut self) -> PD13_W<13> {
        PD13_W::new(self)
    }
    ///Bit 14 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd14(&mut self) -> PD14_W<14> {
        PD14_W::new(self)
    }
    ///Bit 15 - Port H pull-down bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pd15(&mut self) -> PD15_W<15> {
        PD15_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Power Port H pull-down control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pdcrh](index.html) module
pub struct PDCRH_SPEC;
impl crate::RegisterSpec for PDCRH_SPEC {
    type Ux = u32;
}
///`read()` method returns [pdcrh::R](R) reader structure
impl crate::Readable for PDCRH_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pdcrh::W](W) writer structure
impl crate::Writable for PDCRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PDCRH to value 0
impl crate::Resettable for PDCRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
