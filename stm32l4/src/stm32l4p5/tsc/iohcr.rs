///Register `IOHCR` reader
pub struct R(crate::R<IOHCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOHCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOHCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOHCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IOHCR` writer
pub struct W(crate::W<IOHCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOHCR_SPEC>;
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
impl From<crate::W<IOHCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOHCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `G1_IO1` reader - G1_IO1
pub type G1_IO1_R = crate::BitReader<G1_IO1_A>;
///G1_IO1
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum G1_IO1_A {
    ///0: Gx_IOy Schmitt trigger hysteresis disabled
    Disabled = 0,
    ///1: Gx_IOy Schmitt trigger hysteresis enabled
    Enabled = 1,
}
impl From<G1_IO1_A> for bool {
    #[inline(always)]
    fn from(variant: G1_IO1_A) -> Self {
        variant as u8 != 0
    }
}
impl G1_IO1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> G1_IO1_A {
        match self.bits {
            false => G1_IO1_A::Disabled,
            true => G1_IO1_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == G1_IO1_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == G1_IO1_A::Enabled
    }
}
///Field `G1_IO1` writer - G1_IO1
pub type G1_IO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOHCR_SPEC, G1_IO1_A, O>;
impl<'a, const O: u8> G1_IO1_W<'a, O> {
    ///Gx_IOy Schmitt trigger hysteresis disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(G1_IO1_A::Disabled)
    }
    ///Gx_IOy Schmitt trigger hysteresis enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(G1_IO1_A::Enabled)
    }
}
///Field `G1_IO2` reader - G1_IO2
pub use G1_IO1_R as G1_IO2_R;
///Field `G1_IO3` reader - G1_IO3
pub use G1_IO1_R as G1_IO3_R;
///Field `G1_IO4` reader - G1_IO4
pub use G1_IO1_R as G1_IO4_R;
///Field `G2_IO1` reader - G2_IO1
pub use G1_IO1_R as G2_IO1_R;
///Field `G2_IO2` reader - G2_IO2
pub use G1_IO1_R as G2_IO2_R;
///Field `G2_IO3` reader - G2_IO3
pub use G1_IO1_R as G2_IO3_R;
///Field `G2_IO4` reader - G2_IO4
pub use G1_IO1_R as G2_IO4_R;
///Field `G3_IO1` reader - G3_IO1
pub use G1_IO1_R as G3_IO1_R;
///Field `G3_IO2` reader - G3_IO2
pub use G1_IO1_R as G3_IO2_R;
///Field `G3_IO3` reader - G3_IO3
pub use G1_IO1_R as G3_IO3_R;
///Field `G3_IO4` reader - G3_IO4
pub use G1_IO1_R as G3_IO4_R;
///Field `G4_IO1` reader - G4_IO1
pub use G1_IO1_R as G4_IO1_R;
///Field `G4_IO2` reader - G4_IO2
pub use G1_IO1_R as G4_IO2_R;
///Field `G4_IO3` reader - G4_IO3
pub use G1_IO1_R as G4_IO3_R;
///Field `G4_IO4` reader - G4_IO4
pub use G1_IO1_R as G4_IO4_R;
///Field `G5_IO1` reader - G5_IO1
pub use G1_IO1_R as G5_IO1_R;
///Field `G5_IO2` reader - G5_IO2
pub use G1_IO1_R as G5_IO2_R;
///Field `G5_IO3` reader - G5_IO3
pub use G1_IO1_R as G5_IO3_R;
///Field `G5_IO4` reader - G5_IO4
pub use G1_IO1_R as G5_IO4_R;
///Field `G6_IO1` reader - G6_IO1
pub use G1_IO1_R as G6_IO1_R;
///Field `G6_IO2` reader - G6_IO2
pub use G1_IO1_R as G6_IO2_R;
///Field `G6_IO3` reader - G6_IO3
pub use G1_IO1_R as G6_IO3_R;
///Field `G6_IO4` reader - G6_IO4
pub use G1_IO1_R as G6_IO4_R;
///Field `G7_IO1` reader - G7_IO1
pub use G1_IO1_R as G7_IO1_R;
///Field `G7_IO2` reader - G7_IO2
pub use G1_IO1_R as G7_IO2_R;
///Field `G7_IO3` reader - G7_IO3
pub use G1_IO1_R as G7_IO3_R;
///Field `G7_IO4` reader - G7_IO4
pub use G1_IO1_R as G7_IO4_R;
///Field `G8_IO1` reader - G8_IO1
pub use G1_IO1_R as G8_IO1_R;
///Field `G8_IO2` reader - G8_IO2
pub use G1_IO1_R as G8_IO2_R;
///Field `G8_IO3` reader - G8_IO3
pub use G1_IO1_R as G8_IO3_R;
///Field `G8_IO4` reader - G8_IO4
pub use G1_IO1_R as G8_IO4_R;
///Field `G1_IO2` writer - G1_IO2
pub use G1_IO1_W as G1_IO2_W;
///Field `G1_IO3` writer - G1_IO3
pub use G1_IO1_W as G1_IO3_W;
///Field `G1_IO4` writer - G1_IO4
pub use G1_IO1_W as G1_IO4_W;
///Field `G2_IO1` writer - G2_IO1
pub use G1_IO1_W as G2_IO1_W;
///Field `G2_IO2` writer - G2_IO2
pub use G1_IO1_W as G2_IO2_W;
///Field `G2_IO3` writer - G2_IO3
pub use G1_IO1_W as G2_IO3_W;
///Field `G2_IO4` writer - G2_IO4
pub use G1_IO1_W as G2_IO4_W;
///Field `G3_IO1` writer - G3_IO1
pub use G1_IO1_W as G3_IO1_W;
///Field `G3_IO2` writer - G3_IO2
pub use G1_IO1_W as G3_IO2_W;
///Field `G3_IO3` writer - G3_IO3
pub use G1_IO1_W as G3_IO3_W;
///Field `G3_IO4` writer - G3_IO4
pub use G1_IO1_W as G3_IO4_W;
///Field `G4_IO1` writer - G4_IO1
pub use G1_IO1_W as G4_IO1_W;
///Field `G4_IO2` writer - G4_IO2
pub use G1_IO1_W as G4_IO2_W;
///Field `G4_IO3` writer - G4_IO3
pub use G1_IO1_W as G4_IO3_W;
///Field `G4_IO4` writer - G4_IO4
pub use G1_IO1_W as G4_IO4_W;
///Field `G5_IO1` writer - G5_IO1
pub use G1_IO1_W as G5_IO1_W;
///Field `G5_IO2` writer - G5_IO2
pub use G1_IO1_W as G5_IO2_W;
///Field `G5_IO3` writer - G5_IO3
pub use G1_IO1_W as G5_IO3_W;
///Field `G5_IO4` writer - G5_IO4
pub use G1_IO1_W as G5_IO4_W;
///Field `G6_IO1` writer - G6_IO1
pub use G1_IO1_W as G6_IO1_W;
///Field `G6_IO2` writer - G6_IO2
pub use G1_IO1_W as G6_IO2_W;
///Field `G6_IO3` writer - G6_IO3
pub use G1_IO1_W as G6_IO3_W;
///Field `G6_IO4` writer - G6_IO4
pub use G1_IO1_W as G6_IO4_W;
///Field `G7_IO1` writer - G7_IO1
pub use G1_IO1_W as G7_IO1_W;
///Field `G7_IO2` writer - G7_IO2
pub use G1_IO1_W as G7_IO2_W;
///Field `G7_IO3` writer - G7_IO3
pub use G1_IO1_W as G7_IO3_W;
///Field `G7_IO4` writer - G7_IO4
pub use G1_IO1_W as G7_IO4_W;
///Field `G8_IO1` writer - G8_IO1
pub use G1_IO1_W as G8_IO1_W;
///Field `G8_IO2` writer - G8_IO2
pub use G1_IO1_W as G8_IO2_W;
///Field `G8_IO3` writer - G8_IO3
pub use G1_IO1_W as G8_IO3_W;
///Field `G8_IO4` writer - G8_IO4
pub use G1_IO1_W as G8_IO4_W;
impl R {
    ///Bit 0 - G1_IO1
    #[inline(always)]
    pub fn g1_io1(&self) -> G1_IO1_R {
        G1_IO1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - G1_IO2
    #[inline(always)]
    pub fn g1_io2(&self) -> G1_IO2_R {
        G1_IO2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - G1_IO3
    #[inline(always)]
    pub fn g1_io3(&self) -> G1_IO3_R {
        G1_IO3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - G1_IO4
    #[inline(always)]
    pub fn g1_io4(&self) -> G1_IO4_R {
        G1_IO4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - G2_IO1
    #[inline(always)]
    pub fn g2_io1(&self) -> G2_IO1_R {
        G2_IO1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - G2_IO2
    #[inline(always)]
    pub fn g2_io2(&self) -> G2_IO2_R {
        G2_IO2_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - G2_IO3
    #[inline(always)]
    pub fn g2_io3(&self) -> G2_IO3_R {
        G2_IO3_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - G2_IO4
    #[inline(always)]
    pub fn g2_io4(&self) -> G2_IO4_R {
        G2_IO4_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - G3_IO1
    #[inline(always)]
    pub fn g3_io1(&self) -> G3_IO1_R {
        G3_IO1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - G3_IO2
    #[inline(always)]
    pub fn g3_io2(&self) -> G3_IO2_R {
        G3_IO2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - G3_IO3
    #[inline(always)]
    pub fn g3_io3(&self) -> G3_IO3_R {
        G3_IO3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - G3_IO4
    #[inline(always)]
    pub fn g3_io4(&self) -> G3_IO4_R {
        G3_IO4_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - G4_IO1
    #[inline(always)]
    pub fn g4_io1(&self) -> G4_IO1_R {
        G4_IO1_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - G4_IO2
    #[inline(always)]
    pub fn g4_io2(&self) -> G4_IO2_R {
        G4_IO2_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - G4_IO3
    #[inline(always)]
    pub fn g4_io3(&self) -> G4_IO3_R {
        G4_IO3_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - G4_IO4
    #[inline(always)]
    pub fn g4_io4(&self) -> G4_IO4_R {
        G4_IO4_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - G5_IO1
    #[inline(always)]
    pub fn g5_io1(&self) -> G5_IO1_R {
        G5_IO1_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - G5_IO2
    #[inline(always)]
    pub fn g5_io2(&self) -> G5_IO2_R {
        G5_IO2_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - G5_IO3
    #[inline(always)]
    pub fn g5_io3(&self) -> G5_IO3_R {
        G5_IO3_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - G5_IO4
    #[inline(always)]
    pub fn g5_io4(&self) -> G5_IO4_R {
        G5_IO4_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - G6_IO1
    #[inline(always)]
    pub fn g6_io1(&self) -> G6_IO1_R {
        G6_IO1_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - G6_IO2
    #[inline(always)]
    pub fn g6_io2(&self) -> G6_IO2_R {
        G6_IO2_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - G6_IO3
    #[inline(always)]
    pub fn g6_io3(&self) -> G6_IO3_R {
        G6_IO3_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - G6_IO4
    #[inline(always)]
    pub fn g6_io4(&self) -> G6_IO4_R {
        G6_IO4_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - G7_IO1
    #[inline(always)]
    pub fn g7_io1(&self) -> G7_IO1_R {
        G7_IO1_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - G7_IO2
    #[inline(always)]
    pub fn g7_io2(&self) -> G7_IO2_R {
        G7_IO2_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - G7_IO3
    #[inline(always)]
    pub fn g7_io3(&self) -> G7_IO3_R {
        G7_IO3_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - G7_IO4
    #[inline(always)]
    pub fn g7_io4(&self) -> G7_IO4_R {
        G7_IO4_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - G8_IO1
    #[inline(always)]
    pub fn g8_io1(&self) -> G8_IO1_R {
        G8_IO1_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - G8_IO2
    #[inline(always)]
    pub fn g8_io2(&self) -> G8_IO2_R {
        G8_IO2_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - G8_IO3
    #[inline(always)]
    pub fn g8_io3(&self) -> G8_IO3_R {
        G8_IO3_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - G8_IO4
    #[inline(always)]
    pub fn g8_io4(&self) -> G8_IO4_R {
        G8_IO4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - G1_IO1
    #[inline(always)]
    #[must_use]
    pub fn g1_io1(&mut self) -> G1_IO1_W<0> {
        G1_IO1_W::new(self)
    }
    ///Bit 1 - G1_IO2
    #[inline(always)]
    #[must_use]
    pub fn g1_io2(&mut self) -> G1_IO2_W<1> {
        G1_IO2_W::new(self)
    }
    ///Bit 2 - G1_IO3
    #[inline(always)]
    #[must_use]
    pub fn g1_io3(&mut self) -> G1_IO3_W<2> {
        G1_IO3_W::new(self)
    }
    ///Bit 3 - G1_IO4
    #[inline(always)]
    #[must_use]
    pub fn g1_io4(&mut self) -> G1_IO4_W<3> {
        G1_IO4_W::new(self)
    }
    ///Bit 4 - G2_IO1
    #[inline(always)]
    #[must_use]
    pub fn g2_io1(&mut self) -> G2_IO1_W<4> {
        G2_IO1_W::new(self)
    }
    ///Bit 5 - G2_IO2
    #[inline(always)]
    #[must_use]
    pub fn g2_io2(&mut self) -> G2_IO2_W<5> {
        G2_IO2_W::new(self)
    }
    ///Bit 6 - G2_IO3
    #[inline(always)]
    #[must_use]
    pub fn g2_io3(&mut self) -> G2_IO3_W<6> {
        G2_IO3_W::new(self)
    }
    ///Bit 7 - G2_IO4
    #[inline(always)]
    #[must_use]
    pub fn g2_io4(&mut self) -> G2_IO4_W<7> {
        G2_IO4_W::new(self)
    }
    ///Bit 8 - G3_IO1
    #[inline(always)]
    #[must_use]
    pub fn g3_io1(&mut self) -> G3_IO1_W<8> {
        G3_IO1_W::new(self)
    }
    ///Bit 9 - G3_IO2
    #[inline(always)]
    #[must_use]
    pub fn g3_io2(&mut self) -> G3_IO2_W<9> {
        G3_IO2_W::new(self)
    }
    ///Bit 10 - G3_IO3
    #[inline(always)]
    #[must_use]
    pub fn g3_io3(&mut self) -> G3_IO3_W<10> {
        G3_IO3_W::new(self)
    }
    ///Bit 11 - G3_IO4
    #[inline(always)]
    #[must_use]
    pub fn g3_io4(&mut self) -> G3_IO4_W<11> {
        G3_IO4_W::new(self)
    }
    ///Bit 12 - G4_IO1
    #[inline(always)]
    #[must_use]
    pub fn g4_io1(&mut self) -> G4_IO1_W<12> {
        G4_IO1_W::new(self)
    }
    ///Bit 13 - G4_IO2
    #[inline(always)]
    #[must_use]
    pub fn g4_io2(&mut self) -> G4_IO2_W<13> {
        G4_IO2_W::new(self)
    }
    ///Bit 14 - G4_IO3
    #[inline(always)]
    #[must_use]
    pub fn g4_io3(&mut self) -> G4_IO3_W<14> {
        G4_IO3_W::new(self)
    }
    ///Bit 15 - G4_IO4
    #[inline(always)]
    #[must_use]
    pub fn g4_io4(&mut self) -> G4_IO4_W<15> {
        G4_IO4_W::new(self)
    }
    ///Bit 16 - G5_IO1
    #[inline(always)]
    #[must_use]
    pub fn g5_io1(&mut self) -> G5_IO1_W<16> {
        G5_IO1_W::new(self)
    }
    ///Bit 17 - G5_IO2
    #[inline(always)]
    #[must_use]
    pub fn g5_io2(&mut self) -> G5_IO2_W<17> {
        G5_IO2_W::new(self)
    }
    ///Bit 18 - G5_IO3
    #[inline(always)]
    #[must_use]
    pub fn g5_io3(&mut self) -> G5_IO3_W<18> {
        G5_IO3_W::new(self)
    }
    ///Bit 19 - G5_IO4
    #[inline(always)]
    #[must_use]
    pub fn g5_io4(&mut self) -> G5_IO4_W<19> {
        G5_IO4_W::new(self)
    }
    ///Bit 20 - G6_IO1
    #[inline(always)]
    #[must_use]
    pub fn g6_io1(&mut self) -> G6_IO1_W<20> {
        G6_IO1_W::new(self)
    }
    ///Bit 21 - G6_IO2
    #[inline(always)]
    #[must_use]
    pub fn g6_io2(&mut self) -> G6_IO2_W<21> {
        G6_IO2_W::new(self)
    }
    ///Bit 22 - G6_IO3
    #[inline(always)]
    #[must_use]
    pub fn g6_io3(&mut self) -> G6_IO3_W<22> {
        G6_IO3_W::new(self)
    }
    ///Bit 23 - G6_IO4
    #[inline(always)]
    #[must_use]
    pub fn g6_io4(&mut self) -> G6_IO4_W<23> {
        G6_IO4_W::new(self)
    }
    ///Bit 24 - G7_IO1
    #[inline(always)]
    #[must_use]
    pub fn g7_io1(&mut self) -> G7_IO1_W<24> {
        G7_IO1_W::new(self)
    }
    ///Bit 25 - G7_IO2
    #[inline(always)]
    #[must_use]
    pub fn g7_io2(&mut self) -> G7_IO2_W<25> {
        G7_IO2_W::new(self)
    }
    ///Bit 26 - G7_IO3
    #[inline(always)]
    #[must_use]
    pub fn g7_io3(&mut self) -> G7_IO3_W<26> {
        G7_IO3_W::new(self)
    }
    ///Bit 27 - G7_IO4
    #[inline(always)]
    #[must_use]
    pub fn g7_io4(&mut self) -> G7_IO4_W<27> {
        G7_IO4_W::new(self)
    }
    ///Bit 28 - G8_IO1
    #[inline(always)]
    #[must_use]
    pub fn g8_io1(&mut self) -> G8_IO1_W<28> {
        G8_IO1_W::new(self)
    }
    ///Bit 29 - G8_IO2
    #[inline(always)]
    #[must_use]
    pub fn g8_io2(&mut self) -> G8_IO2_W<29> {
        G8_IO2_W::new(self)
    }
    ///Bit 30 - G8_IO3
    #[inline(always)]
    #[must_use]
    pub fn g8_io3(&mut self) -> G8_IO3_W<30> {
        G8_IO3_W::new(self)
    }
    ///Bit 31 - G8_IO4
    #[inline(always)]
    #[must_use]
    pub fn g8_io4(&mut self) -> G8_IO4_W<31> {
        G8_IO4_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///I/O hysteresis control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [iohcr](index.html) module
pub struct IOHCR_SPEC;
impl crate::RegisterSpec for IOHCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [iohcr::R](R) reader structure
impl crate::Readable for IOHCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [iohcr::W](W) writer structure
impl crate::Writable for IOHCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IOHCR to value 0xffff_ffff
impl crate::Resettable for IOHCR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
