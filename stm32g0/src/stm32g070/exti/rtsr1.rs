///Register `RTSR1` reader
pub struct R(crate::R<RTSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTSR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RTSR1` writer
pub struct W(crate::W<RTSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTSR1_SPEC>;
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
impl From<crate::W<RTSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTSR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TR0` reader - Rising trigger event configuration bit of Configurable Event input
pub type TR0_R = crate::BitReader<TR0_A>;
///Rising trigger event configuration bit of Configurable Event input
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TR0_A {
    ///0: Rising edge trigger is disabled
    Disabled = 0,
    ///1: Rising edge trigger is enabled
    Enabled = 1,
}
impl From<TR0_A> for bool {
    #[inline(always)]
    fn from(variant: TR0_A) -> Self {
        variant as u8 != 0
    }
}
impl TR0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TR0_A {
        match self.bits {
            false => TR0_A::Disabled,
            true => TR0_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TR0_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TR0_A::Enabled
    }
}
///Field `TR0` writer - Rising trigger event configuration bit of Configurable Event input
pub type TR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTSR1_SPEC, TR0_A, O>;
impl<'a, const O: u8> TR0_W<'a, O> {
    ///Rising edge trigger is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TR0_A::Disabled)
    }
    ///Rising edge trigger is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TR0_A::Enabled)
    }
}
///Field `TR1` reader - Rising trigger event configuration bit of Configurable Event input
pub use TR0_R as TR1_R;
///Field `TR2` reader - Rising trigger event configuration bit of Configurable Event input
pub use TR0_R as TR2_R;
///Field `TR3` reader - Rising trigger event configuration bit of Configurable Event input
pub use TR0_R as TR3_R;
///Field `TR4` reader - Rising trigger event configuration bit of Configurable Event input
pub use TR0_R as TR4_R;
///Field `TR5` reader - Rising trigger event configuration bit of Configurable Event input
pub use TR0_R as TR5_R;
///Field `TR6` reader - Rising trigger event configuration bit of Configurable Event input
pub use TR0_R as TR6_R;
///Field `TR7` reader - Rising trigger event configuration bit of Configurable Event input
pub use TR0_R as TR7_R;
///Field `TR8` reader - Rising trigger event configuration bit of Configurable Event input
pub use TR0_R as TR8_R;
///Field `TR9` reader - Rising trigger event configuration bit of Configurable Event input
pub use TR0_R as TR9_R;
///Field `TR10` reader - Rising trigger event configuration bit of Configurable Event input
pub use TR0_R as TR10_R;
///Field `TR11` reader - Rising trigger event configuration bit of Configurable Event input
pub use TR0_R as TR11_R;
///Field `TR12` reader - Rising trigger event configuration bit of Configurable Event input
pub use TR0_R as TR12_R;
///Field `TR13` reader - Rising trigger event configuration bit of Configurable Event input
pub use TR0_R as TR13_R;
///Field `TR14` reader - Rising trigger event configuration bit of Configurable Event input
pub use TR0_R as TR14_R;
///Field `TR15` reader - Rising trigger event configuration bit of Configurable Event input
pub use TR0_R as TR15_R;
///Field `TR16` reader - Rising trigger event configuration bit of Configurable Event input
pub use TR0_R as TR16_R;
///Field `TR17` reader - Rising trigger event configuration bit of Configurable Event input
pub use TR0_R as TR17_R;
///Field `TR18` reader - Rising trigger event configuration bit of Configurable Event input
pub use TR0_R as TR18_R;
///Field `TR1` writer - Rising trigger event configuration bit of Configurable Event input
pub use TR0_W as TR1_W;
///Field `TR2` writer - Rising trigger event configuration bit of Configurable Event input
pub use TR0_W as TR2_W;
///Field `TR3` writer - Rising trigger event configuration bit of Configurable Event input
pub use TR0_W as TR3_W;
///Field `TR4` writer - Rising trigger event configuration bit of Configurable Event input
pub use TR0_W as TR4_W;
///Field `TR5` writer - Rising trigger event configuration bit of Configurable Event input
pub use TR0_W as TR5_W;
///Field `TR6` writer - Rising trigger event configuration bit of Configurable Event input
pub use TR0_W as TR6_W;
///Field `TR7` writer - Rising trigger event configuration bit of Configurable Event input
pub use TR0_W as TR7_W;
///Field `TR8` writer - Rising trigger event configuration bit of Configurable Event input
pub use TR0_W as TR8_W;
///Field `TR9` writer - Rising trigger event configuration bit of Configurable Event input
pub use TR0_W as TR9_W;
///Field `TR10` writer - Rising trigger event configuration bit of Configurable Event input
pub use TR0_W as TR10_W;
///Field `TR11` writer - Rising trigger event configuration bit of Configurable Event input
pub use TR0_W as TR11_W;
///Field `TR12` writer - Rising trigger event configuration bit of Configurable Event input
pub use TR0_W as TR12_W;
///Field `TR13` writer - Rising trigger event configuration bit of Configurable Event input
pub use TR0_W as TR13_W;
///Field `TR14` writer - Rising trigger event configuration bit of Configurable Event input
pub use TR0_W as TR14_W;
///Field `TR15` writer - Rising trigger event configuration bit of Configurable Event input
pub use TR0_W as TR15_W;
///Field `TR16` writer - Rising trigger event configuration bit of Configurable Event input
pub use TR0_W as TR16_W;
///Field `TR17` writer - Rising trigger event configuration bit of Configurable Event input
pub use TR0_W as TR17_W;
///Field `TR18` writer - Rising trigger event configuration bit of Configurable Event input
pub use TR0_W as TR18_W;
impl R {
    ///Bit 0 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn tr0(&self) -> TR0_R {
        TR0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn tr1(&self) -> TR1_R {
        TR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn tr2(&self) -> TR2_R {
        TR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn tr3(&self) -> TR3_R {
        TR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn tr4(&self) -> TR4_R {
        TR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn tr5(&self) -> TR5_R {
        TR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn tr6(&self) -> TR6_R {
        TR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn tr7(&self) -> TR7_R {
        TR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn tr8(&self) -> TR8_R {
        TR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn tr9(&self) -> TR9_R {
        TR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn tr10(&self) -> TR10_R {
        TR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn tr11(&self) -> TR11_R {
        TR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn tr12(&self) -> TR12_R {
        TR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn tr13(&self) -> TR13_R {
        TR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn tr14(&self) -> TR14_R {
        TR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn tr15(&self) -> TR15_R {
        TR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn tr16(&self) -> TR16_R {
        TR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn tr17(&self) -> TR17_R {
        TR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn tr18(&self) -> TR18_R {
        TR18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn tr0(&mut self) -> TR0_W<0> {
        TR0_W::new(self)
    }
    ///Bit 1 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn tr1(&mut self) -> TR1_W<1> {
        TR1_W::new(self)
    }
    ///Bit 2 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn tr2(&mut self) -> TR2_W<2> {
        TR2_W::new(self)
    }
    ///Bit 3 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn tr3(&mut self) -> TR3_W<3> {
        TR3_W::new(self)
    }
    ///Bit 4 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn tr4(&mut self) -> TR4_W<4> {
        TR4_W::new(self)
    }
    ///Bit 5 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn tr5(&mut self) -> TR5_W<5> {
        TR5_W::new(self)
    }
    ///Bit 6 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn tr6(&mut self) -> TR6_W<6> {
        TR6_W::new(self)
    }
    ///Bit 7 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn tr7(&mut self) -> TR7_W<7> {
        TR7_W::new(self)
    }
    ///Bit 8 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn tr8(&mut self) -> TR8_W<8> {
        TR8_W::new(self)
    }
    ///Bit 9 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn tr9(&mut self) -> TR9_W<9> {
        TR9_W::new(self)
    }
    ///Bit 10 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn tr10(&mut self) -> TR10_W<10> {
        TR10_W::new(self)
    }
    ///Bit 11 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn tr11(&mut self) -> TR11_W<11> {
        TR11_W::new(self)
    }
    ///Bit 12 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn tr12(&mut self) -> TR12_W<12> {
        TR12_W::new(self)
    }
    ///Bit 13 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn tr13(&mut self) -> TR13_W<13> {
        TR13_W::new(self)
    }
    ///Bit 14 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn tr14(&mut self) -> TR14_W<14> {
        TR14_W::new(self)
    }
    ///Bit 15 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn tr15(&mut self) -> TR15_W<15> {
        TR15_W::new(self)
    }
    ///Bit 16 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn tr16(&mut self) -> TR16_W<16> {
        TR16_W::new(self)
    }
    ///Bit 17 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn tr17(&mut self) -> TR17_W<17> {
        TR17_W::new(self)
    }
    ///Bit 18 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    #[must_use]
    pub fn tr18(&mut self) -> TR18_W<18> {
        TR18_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI rising trigger selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rtsr1](index.html) module
pub struct RTSR1_SPEC;
impl crate::RegisterSpec for RTSR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [rtsr1::R](R) reader structure
impl crate::Readable for RTSR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rtsr1::W](W) writer structure
impl crate::Writable for RTSR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RTSR1 to value 0
impl crate::Resettable for RTSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
