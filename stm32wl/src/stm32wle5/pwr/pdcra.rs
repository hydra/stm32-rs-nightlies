///Register `PDCRA` reader
pub struct R(crate::R<PDCRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDCRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDCRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDCRA_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PDCRA` writer
pub struct W(crate::W<PDCRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDCRA_SPEC>;
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
impl From<crate::W<PDCRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDCRA_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PD0` reader - PD0
pub type PD0_R = crate::BitReader<PD0_A>;
///PD0
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD0_A {
    ///0: Disable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    Disabled = 0,
    ///1: Enable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
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
///Field `PD0` writer - PD0
pub type PD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRA_SPEC, PD0_A, O>;
impl<'a, const O: u8> PD0_W<'a, O> {
    ///Disable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD0_A::Disabled)
    }
    ///Enable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD0_A::Enabled)
    }
}
///Field `PD1` reader - PD1
pub use PD0_R as PD1_R;
///Field `PD2` reader - PD2
pub use PD0_R as PD2_R;
///Field `PD3` reader - PD3
pub use PD0_R as PD3_R;
///Field `PD4` reader - PD4
pub use PD0_R as PD4_R;
///Field `PD5` reader - PD5
pub use PD0_R as PD5_R;
///Field `PD6` reader - PD6
pub use PD0_R as PD6_R;
///Field `PD7` reader - PD7
pub use PD0_R as PD7_R;
///Field `PD8` reader - PD8
pub use PD0_R as PD8_R;
///Field `PD9` reader - PD9
pub use PD0_R as PD9_R;
///Field `PD1` writer - PD1
pub use PD0_W as PD1_W;
///Field `PD2` writer - PD2
pub use PD0_W as PD2_W;
///Field `PD3` writer - PD3
pub use PD0_W as PD3_W;
///Field `PD4` writer - PD4
pub use PD0_W as PD4_W;
///Field `PD5` writer - PD5
pub use PD0_W as PD5_W;
///Field `PD6` writer - PD6
pub use PD0_W as PD6_W;
///Field `PD7` writer - PD7
pub use PD0_W as PD7_W;
///Field `PD8` writer - PD8
pub use PD0_W as PD8_W;
///Field `PD9` writer - PD9
pub use PD0_W as PD9_W;
///Field `PD10` reader - PD10
pub type PD10_R = crate::BitReader<PD10_A>;
///PD10
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD10_A {
    ///0: Disable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    Disabled = 0,
    ///1: Enable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    Enabled = 1,
}
impl From<PD10_A> for bool {
    #[inline(always)]
    fn from(variant: PD10_A) -> Self {
        variant as u8 != 0
    }
}
impl PD10_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PD10_A {
        match self.bits {
            false => PD10_A::Disabled,
            true => PD10_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PD10_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PD10_A::Enabled
    }
}
///Field `PD10` writer - PD10
pub type PD10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRA_SPEC, PD10_A, O>;
impl<'a, const O: u8> PD10_W<'a, O> {
    ///Disable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PD10_A::Disabled)
    }
    ///Enable the pull-down on PA\[y\]
    ///when both APC bits are set in PWR control register 3 (PWR_CR3)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PD10_A::Enabled)
    }
}
///Field `PD11` reader - PD11
pub use PD10_R as PD11_R;
///Field `PD12` reader - Port PA\[y\]
///pull-down (y=0 to 12)
pub use PD10_R as PD12_R;
///Field `PD13` reader - PD13
pub use PD10_R as PD13_R;
///Field `PD14` reader - ull-down
pub use PD10_R as PD14_R;
///Field `PD15` reader - PD15
pub use PD10_R as PD15_R;
///Field `PD11` writer - PD11
pub use PD10_W as PD11_W;
///Field `PD12` writer - Port PA\[y\]
///pull-down (y=0 to 12)
pub use PD10_W as PD12_W;
///Field `PD13` writer - PD13
pub use PD10_W as PD13_W;
///Field `PD14` writer - ull-down
pub use PD10_W as PD14_W;
///Field `PD15` writer - PD15
pub use PD10_W as PD15_W;
impl R {
    ///Bit 0 - PD0
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PD1
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PD2
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - PD3
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PD4
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PD5
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PD6
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - PD7
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - PD8
    #[inline(always)]
    pub fn pd8(&self) -> PD8_R {
        PD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - PD9
    #[inline(always)]
    pub fn pd9(&self) -> PD9_R {
        PD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PD10
    #[inline(always)]
    pub fn pd10(&self) -> PD10_R {
        PD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - PD11
    #[inline(always)]
    pub fn pd11(&self) -> PD11_R {
        PD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port PA\[y\]
    ///pull-down (y=0 to 12)
    #[inline(always)]
    pub fn pd12(&self) -> PD12_R {
        PD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - PD13
    #[inline(always)]
    pub fn pd13(&self) -> PD13_R {
        PD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ull-down
    #[inline(always)]
    pub fn pd14(&self) -> PD14_R {
        PD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - PD15
    #[inline(always)]
    pub fn pd15(&self) -> PD15_R {
        PD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - PD0
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> PD0_W<0> {
        PD0_W::new(self)
    }
    ///Bit 1 - PD1
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> PD1_W<1> {
        PD1_W::new(self)
    }
    ///Bit 2 - PD2
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> PD2_W<2> {
        PD2_W::new(self)
    }
    ///Bit 3 - PD3
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> PD3_W<3> {
        PD3_W::new(self)
    }
    ///Bit 4 - PD4
    #[inline(always)]
    #[must_use]
    pub fn pd4(&mut self) -> PD4_W<4> {
        PD4_W::new(self)
    }
    ///Bit 5 - PD5
    #[inline(always)]
    #[must_use]
    pub fn pd5(&mut self) -> PD5_W<5> {
        PD5_W::new(self)
    }
    ///Bit 6 - PD6
    #[inline(always)]
    #[must_use]
    pub fn pd6(&mut self) -> PD6_W<6> {
        PD6_W::new(self)
    }
    ///Bit 7 - PD7
    #[inline(always)]
    #[must_use]
    pub fn pd7(&mut self) -> PD7_W<7> {
        PD7_W::new(self)
    }
    ///Bit 8 - PD8
    #[inline(always)]
    #[must_use]
    pub fn pd8(&mut self) -> PD8_W<8> {
        PD8_W::new(self)
    }
    ///Bit 9 - PD9
    #[inline(always)]
    #[must_use]
    pub fn pd9(&mut self) -> PD9_W<9> {
        PD9_W::new(self)
    }
    ///Bit 10 - PD10
    #[inline(always)]
    #[must_use]
    pub fn pd10(&mut self) -> PD10_W<10> {
        PD10_W::new(self)
    }
    ///Bit 11 - PD11
    #[inline(always)]
    #[must_use]
    pub fn pd11(&mut self) -> PD11_W<11> {
        PD11_W::new(self)
    }
    ///Bit 12 - Port PA\[y\]
    ///pull-down (y=0 to 12)
    #[inline(always)]
    #[must_use]
    pub fn pd12(&mut self) -> PD12_W<12> {
        PD12_W::new(self)
    }
    ///Bit 13 - PD13
    #[inline(always)]
    #[must_use]
    pub fn pd13(&mut self) -> PD13_W<13> {
        PD13_W::new(self)
    }
    ///Bit 14 - ull-down
    #[inline(always)]
    #[must_use]
    pub fn pd14(&mut self) -> PD14_W<14> {
        PD14_W::new(self)
    }
    ///Bit 15 - PD15
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
///Power Port A pull-down control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pdcra](index.html) module
pub struct PDCRA_SPEC;
impl crate::RegisterSpec for PDCRA_SPEC {
    type Ux = u32;
}
///`read()` method returns [pdcra::R](R) reader structure
impl crate::Readable for PDCRA_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pdcra::W](W) writer structure
impl crate::Writable for PDCRA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PDCRA to value 0
impl crate::Resettable for PDCRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
