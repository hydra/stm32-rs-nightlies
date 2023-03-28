///Register `SWPR2` reader
pub struct R(crate::R<SWPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWPR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SWPR2` writer
pub struct W(crate::W<SWPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWPR2_SPEC>;
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
impl From<crate::W<SWPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWPR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `P32WP` reader - SRAM2 page x write protection
pub type P32WP_R = crate::BitReader<P32WP_A>;
///SRAM2 page x write protection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P32WP_A {
    ///0: Write protection of SRAM2 page x is disabled
    Disabled = 0,
    ///1: Write protection of SRAM2 page x is enabled
    Enabled = 1,
}
impl From<P32WP_A> for bool {
    #[inline(always)]
    fn from(variant: P32WP_A) -> Self {
        variant as u8 != 0
    }
}
impl P32WP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> P32WP_A {
        match self.bits {
            false => P32WP_A::Disabled,
            true => P32WP_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == P32WP_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == P32WP_A::Enabled
    }
}
///Field `P32WP` writer - SRAM2 page x write protection
pub type P32WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWPR2_SPEC, P32WP_A, O>;
impl<'a, const O: u8> P32WP_W<'a, O> {
    ///Write protection of SRAM2 page x is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(P32WP_A::Disabled)
    }
    ///Write protection of SRAM2 page x is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(P32WP_A::Enabled)
    }
}
///Field `P33WP` reader - SRAM2 page x write protection
pub use P32WP_R as P33WP_R;
///Field `P34WP` reader - SRAM2 page x write protection
pub use P32WP_R as P34WP_R;
///Field `P35WP` reader - SRAM2 page x write protection
pub use P32WP_R as P35WP_R;
///Field `P36WP` reader - SRAM2 page x write protection
pub use P32WP_R as P36WP_R;
///Field `P37WP` reader - SRAM2 page x write protection
pub use P32WP_R as P37WP_R;
///Field `P38WP` reader - SRAM2 page x write protection
pub use P32WP_R as P38WP_R;
///Field `P39WP` reader - SRAM2 page x write protection
pub use P32WP_R as P39WP_R;
///Field `P40WP` reader - SRAM2 page x write protection
pub use P32WP_R as P40WP_R;
///Field `P41WP` reader - SRAM2 page x write protection
pub use P32WP_R as P41WP_R;
///Field `P42WP` reader - SRAM2 page x write protection
pub use P32WP_R as P42WP_R;
///Field `P43WP` reader - SRAM2 page x write protection
pub use P32WP_R as P43WP_R;
///Field `P44WP` reader - SRAM2 page x write protection
pub use P32WP_R as P44WP_R;
///Field `P45WP` reader - SRAM2 page x write protection
pub use P32WP_R as P45WP_R;
///Field `P46WP` reader - SRAM2 page x write protection
pub use P32WP_R as P46WP_R;
///Field `P47WP` reader - SRAM2 page x write protection
pub use P32WP_R as P47WP_R;
///Field `P48WP` reader - SRAM2 page x write protection
pub use P32WP_R as P48WP_R;
///Field `P49WP` reader - SRAM2 page x write protection
pub use P32WP_R as P49WP_R;
///Field `P50WP` reader - SRAM2 page x write protection
pub use P32WP_R as P50WP_R;
///Field `P51WP` reader - SRAM2 page x write protection
pub use P32WP_R as P51WP_R;
///Field `P52WP` reader - SRAM2 page x write protection
pub use P32WP_R as P52WP_R;
///Field `P53WP` reader - SRAM2 page x write protection
pub use P32WP_R as P53WP_R;
///Field `P54WP` reader - SRAM2 page x write protection
pub use P32WP_R as P54WP_R;
///Field `P55WP` reader - SRAM2 page x write protection
pub use P32WP_R as P55WP_R;
///Field `P56WP` reader - SRAM2 page x write protection
pub use P32WP_R as P56WP_R;
///Field `P57WP` reader - SRAM2 page x write protection
pub use P32WP_R as P57WP_R;
///Field `P58WP` reader - SRAM2 page x write protection
pub use P32WP_R as P58WP_R;
///Field `P59WP` reader - SRAM2 page x write protection
pub use P32WP_R as P59WP_R;
///Field `P60WP` reader - SRAM2 page x write protection
pub use P32WP_R as P60WP_R;
///Field `P61WP` reader - SRAM2 page x write protection
pub use P32WP_R as P61WP_R;
///Field `P62WP` reader - SRAM2 page x write protection
pub use P32WP_R as P62WP_R;
///Field `P63WP` reader - SRAM2 page x write protection
pub use P32WP_R as P63WP_R;
///Field `P33WP` writer - SRAM2 page x write protection
pub use P32WP_W as P33WP_W;
///Field `P34WP` writer - SRAM2 page x write protection
pub use P32WP_W as P34WP_W;
///Field `P35WP` writer - SRAM2 page x write protection
pub use P32WP_W as P35WP_W;
///Field `P36WP` writer - SRAM2 page x write protection
pub use P32WP_W as P36WP_W;
///Field `P37WP` writer - SRAM2 page x write protection
pub use P32WP_W as P37WP_W;
///Field `P38WP` writer - SRAM2 page x write protection
pub use P32WP_W as P38WP_W;
///Field `P39WP` writer - SRAM2 page x write protection
pub use P32WP_W as P39WP_W;
///Field `P40WP` writer - SRAM2 page x write protection
pub use P32WP_W as P40WP_W;
///Field `P41WP` writer - SRAM2 page x write protection
pub use P32WP_W as P41WP_W;
///Field `P42WP` writer - SRAM2 page x write protection
pub use P32WP_W as P42WP_W;
///Field `P43WP` writer - SRAM2 page x write protection
pub use P32WP_W as P43WP_W;
///Field `P44WP` writer - SRAM2 page x write protection
pub use P32WP_W as P44WP_W;
///Field `P45WP` writer - SRAM2 page x write protection
pub use P32WP_W as P45WP_W;
///Field `P46WP` writer - SRAM2 page x write protection
pub use P32WP_W as P46WP_W;
///Field `P47WP` writer - SRAM2 page x write protection
pub use P32WP_W as P47WP_W;
///Field `P48WP` writer - SRAM2 page x write protection
pub use P32WP_W as P48WP_W;
///Field `P49WP` writer - SRAM2 page x write protection
pub use P32WP_W as P49WP_W;
///Field `P50WP` writer - SRAM2 page x write protection
pub use P32WP_W as P50WP_W;
///Field `P51WP` writer - SRAM2 page x write protection
pub use P32WP_W as P51WP_W;
///Field `P52WP` writer - SRAM2 page x write protection
pub use P32WP_W as P52WP_W;
///Field `P53WP` writer - SRAM2 page x write protection
pub use P32WP_W as P53WP_W;
///Field `P54WP` writer - SRAM2 page x write protection
pub use P32WP_W as P54WP_W;
///Field `P55WP` writer - SRAM2 page x write protection
pub use P32WP_W as P55WP_W;
///Field `P56WP` writer - SRAM2 page x write protection
pub use P32WP_W as P56WP_W;
///Field `P57WP` writer - SRAM2 page x write protection
pub use P32WP_W as P57WP_W;
///Field `P58WP` writer - SRAM2 page x write protection
pub use P32WP_W as P58WP_W;
///Field `P59WP` writer - SRAM2 page x write protection
pub use P32WP_W as P59WP_W;
///Field `P60WP` writer - SRAM2 page x write protection
pub use P32WP_W as P60WP_W;
///Field `P61WP` writer - SRAM2 page x write protection
pub use P32WP_W as P61WP_W;
///Field `P62WP` writer - SRAM2 page x write protection
pub use P32WP_W as P62WP_W;
///Field `P63WP` writer - SRAM2 page x write protection
pub use P32WP_W as P63WP_W;
impl R {
    ///Bit 0 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p32wp(&self) -> P32WP_R {
        P32WP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p33wp(&self) -> P33WP_R {
        P33WP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p34wp(&self) -> P34WP_R {
        P34WP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p35wp(&self) -> P35WP_R {
        P35WP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p36wp(&self) -> P36WP_R {
        P36WP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p37wp(&self) -> P37WP_R {
        P37WP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p38wp(&self) -> P38WP_R {
        P38WP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p39wp(&self) -> P39WP_R {
        P39WP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p40wp(&self) -> P40WP_R {
        P40WP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p41wp(&self) -> P41WP_R {
        P41WP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p42wp(&self) -> P42WP_R {
        P42WP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p43wp(&self) -> P43WP_R {
        P43WP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p44wp(&self) -> P44WP_R {
        P44WP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p45wp(&self) -> P45WP_R {
        P45WP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p46wp(&self) -> P46WP_R {
        P46WP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p47wp(&self) -> P47WP_R {
        P47WP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p48wp(&self) -> P48WP_R {
        P48WP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p49wp(&self) -> P49WP_R {
        P49WP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p50wp(&self) -> P50WP_R {
        P50WP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p51wp(&self) -> P51WP_R {
        P51WP_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p52wp(&self) -> P52WP_R {
        P52WP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p53wp(&self) -> P53WP_R {
        P53WP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p54wp(&self) -> P54WP_R {
        P54WP_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p55wp(&self) -> P55WP_R {
        P55WP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p56wp(&self) -> P56WP_R {
        P56WP_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p57wp(&self) -> P57WP_R {
        P57WP_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p58wp(&self) -> P58WP_R {
        P58WP_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p59wp(&self) -> P59WP_R {
        P59WP_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p60wp(&self) -> P60WP_R {
        P60WP_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p61wp(&self) -> P61WP_R {
        P61WP_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p62wp(&self) -> P62WP_R {
        P62WP_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SRAM2 page x write protection
    #[inline(always)]
    pub fn p63wp(&self) -> P63WP_R {
        P63WP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p32wp(&mut self) -> P32WP_W<0> {
        P32WP_W::new(self)
    }
    ///Bit 1 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p33wp(&mut self) -> P33WP_W<1> {
        P33WP_W::new(self)
    }
    ///Bit 2 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p34wp(&mut self) -> P34WP_W<2> {
        P34WP_W::new(self)
    }
    ///Bit 3 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p35wp(&mut self) -> P35WP_W<3> {
        P35WP_W::new(self)
    }
    ///Bit 4 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p36wp(&mut self) -> P36WP_W<4> {
        P36WP_W::new(self)
    }
    ///Bit 5 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p37wp(&mut self) -> P37WP_W<5> {
        P37WP_W::new(self)
    }
    ///Bit 6 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p38wp(&mut self) -> P38WP_W<6> {
        P38WP_W::new(self)
    }
    ///Bit 7 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p39wp(&mut self) -> P39WP_W<7> {
        P39WP_W::new(self)
    }
    ///Bit 8 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p40wp(&mut self) -> P40WP_W<8> {
        P40WP_W::new(self)
    }
    ///Bit 9 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p41wp(&mut self) -> P41WP_W<9> {
        P41WP_W::new(self)
    }
    ///Bit 10 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p42wp(&mut self) -> P42WP_W<10> {
        P42WP_W::new(self)
    }
    ///Bit 11 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p43wp(&mut self) -> P43WP_W<11> {
        P43WP_W::new(self)
    }
    ///Bit 12 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p44wp(&mut self) -> P44WP_W<12> {
        P44WP_W::new(self)
    }
    ///Bit 13 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p45wp(&mut self) -> P45WP_W<13> {
        P45WP_W::new(self)
    }
    ///Bit 14 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p46wp(&mut self) -> P46WP_W<14> {
        P46WP_W::new(self)
    }
    ///Bit 15 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p47wp(&mut self) -> P47WP_W<15> {
        P47WP_W::new(self)
    }
    ///Bit 16 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p48wp(&mut self) -> P48WP_W<16> {
        P48WP_W::new(self)
    }
    ///Bit 17 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p49wp(&mut self) -> P49WP_W<17> {
        P49WP_W::new(self)
    }
    ///Bit 18 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p50wp(&mut self) -> P50WP_W<18> {
        P50WP_W::new(self)
    }
    ///Bit 19 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p51wp(&mut self) -> P51WP_W<19> {
        P51WP_W::new(self)
    }
    ///Bit 20 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p52wp(&mut self) -> P52WP_W<20> {
        P52WP_W::new(self)
    }
    ///Bit 21 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p53wp(&mut self) -> P53WP_W<21> {
        P53WP_W::new(self)
    }
    ///Bit 22 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p54wp(&mut self) -> P54WP_W<22> {
        P54WP_W::new(self)
    }
    ///Bit 23 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p55wp(&mut self) -> P55WP_W<23> {
        P55WP_W::new(self)
    }
    ///Bit 24 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p56wp(&mut self) -> P56WP_W<24> {
        P56WP_W::new(self)
    }
    ///Bit 25 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p57wp(&mut self) -> P57WP_W<25> {
        P57WP_W::new(self)
    }
    ///Bit 26 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p58wp(&mut self) -> P58WP_W<26> {
        P58WP_W::new(self)
    }
    ///Bit 27 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p59wp(&mut self) -> P59WP_W<27> {
        P59WP_W::new(self)
    }
    ///Bit 28 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p60wp(&mut self) -> P60WP_W<28> {
        P60WP_W::new(self)
    }
    ///Bit 29 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p61wp(&mut self) -> P61WP_W<29> {
        P61WP_W::new(self)
    }
    ///Bit 30 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p62wp(&mut self) -> P62WP_W<30> {
        P62WP_W::new(self)
    }
    ///Bit 31 - SRAM2 page x write protection
    #[inline(always)]
    #[must_use]
    pub fn p63wp(&mut self) -> P63WP_W<31> {
        P63WP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///write protection register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [swpr2](index.html) module
pub struct SWPR2_SPEC;
impl crate::RegisterSpec for SWPR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [swpr2::R](R) reader structure
impl crate::Readable for SWPR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [swpr2::W](W) writer structure
impl crate::Writable for SWPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SWPR2 to value 0
impl crate::Resettable for SWPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
