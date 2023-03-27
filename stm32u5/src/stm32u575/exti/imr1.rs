///Register `IMR1` reader
pub struct R(crate::R<IMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IMR1` writer
pub struct W(crate::W<IMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR1_SPEC>;
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
impl From<crate::W<IMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IM0` reader - CPU wakeup with interrupt mask on event input
pub type IM0_R = crate::BitReader<bool>;
///Field `IM0` writer - CPU wakeup with interrupt mask on event input
pub type IM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
///Field `IM1` reader - CPU wakeup with interrupt mask on event input
pub type IM1_R = crate::BitReader<bool>;
///Field `IM1` writer - CPU wakeup with interrupt mask on event input
pub type IM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
///Field `IM2` reader - CPU wakeup with interrupt mask on event input
pub type IM2_R = crate::BitReader<bool>;
///Field `IM2` writer - CPU wakeup with interrupt mask on event input
pub type IM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
///Field `IM3` reader - CPU wakeup with interrupt mask on event input
pub type IM3_R = crate::BitReader<bool>;
///Field `IM3` writer - CPU wakeup with interrupt mask on event input
pub type IM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
///Field `IM4` reader - CPU wakeup with interrupt mask on event input
pub type IM4_R = crate::BitReader<bool>;
///Field `IM4` writer - CPU wakeup with interrupt mask on event input
pub type IM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
///Field `IM5` reader - CPU wakeup with interrupt mask on event input
pub type IM5_R = crate::BitReader<bool>;
///Field `IM5` writer - CPU wakeup with interrupt mask on event input
pub type IM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
///Field `IM6` reader - CPU wakeup with interrupt mask on event input
pub type IM6_R = crate::BitReader<bool>;
///Field `IM6` writer - CPU wakeup with interrupt mask on event input
pub type IM6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
///Field `IM7` reader - CPU wakeup with interrupt mask on event input
pub type IM7_R = crate::BitReader<bool>;
///Field `IM7` writer - CPU wakeup with interrupt mask on event input
pub type IM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
///Field `IM8` reader - CPU wakeup with interrupt mask on event input
pub type IM8_R = crate::BitReader<bool>;
///Field `IM8` writer - CPU wakeup with interrupt mask on event input
pub type IM8_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
///Field `IM9` reader - CPU wakeup with interrupt mask on event input
pub type IM9_R = crate::BitReader<bool>;
///Field `IM9` writer - CPU wakeup with interrupt mask on event input
pub type IM9_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
///Field `IM10` reader - CPU wakeup with interrupt mask on event input
pub type IM10_R = crate::BitReader<bool>;
///Field `IM10` writer - CPU wakeup with interrupt mask on event input
pub type IM10_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
///Field `IM11` reader - CPU wakeup with interrupt mask on event input
pub type IM11_R = crate::BitReader<bool>;
///Field `IM11` writer - CPU wakeup with interrupt mask on event input
pub type IM11_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
///Field `IM12` reader - CPU wakeup with interrupt mask on event input
pub type IM12_R = crate::BitReader<bool>;
///Field `IM12` writer - CPU wakeup with interrupt mask on event input
pub type IM12_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
///Field `IM13` reader - CPU wakeup with interrupt mask on event input
pub type IM13_R = crate::BitReader<bool>;
///Field `IM13` writer - CPU wakeup with interrupt mask on event input
pub type IM13_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
///Field `IM14` reader - CPU wakeup with interrupt mask on event input
pub type IM14_R = crate::BitReader<bool>;
///Field `IM14` writer - CPU wakeup with interrupt mask on event input
pub type IM14_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
///Field `IM15` reader - CPU wakeup with interrupt mask on event input
pub type IM15_R = crate::BitReader<bool>;
///Field `IM15` writer - CPU wakeup with interrupt mask on event input
pub type IM15_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
///Field `IM16` reader - CPU wakeup with interrupt mask on event input
pub type IM16_R = crate::BitReader<bool>;
///Field `IM16` writer - CPU wakeup with interrupt mask on event input
pub type IM16_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
///Field `IM17` reader - CPU wakeup with interrupt mask on event input
pub type IM17_R = crate::BitReader<bool>;
///Field `IM17` writer - CPU wakeup with interrupt mask on event input
pub type IM17_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
///Field `IM18` reader - CPU wakeup with interrupt mask on event input
pub type IM18_R = crate::BitReader<bool>;
///Field `IM18` writer - CPU wakeup with interrupt mask on event input
pub type IM18_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
///Field `IM19` reader - CPU wakeup with interrupt mask on event input
pub type IM19_R = crate::BitReader<bool>;
///Field `IM19` writer - CPU wakeup with interrupt mask on event input
pub type IM19_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
///Field `IM20` reader - CPU wakeup with interrupt mask on event input
pub type IM20_R = crate::BitReader<bool>;
///Field `IM20` writer - CPU wakeup with interrupt mask on event input
pub type IM20_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
///Field `IM21` reader - CPU wakeup with interrupt mask on event input
pub type IM21_R = crate::BitReader<bool>;
///Field `IM21` writer - CPU wakeup with interrupt mask on event input
pub type IM21_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
///Field `IM22` reader - CPU wakeup with interrupt mask on event input
pub type IM22_R = crate::BitReader<bool>;
///Field `IM22` writer - CPU wakeup with interrupt mask on event input
pub type IM22_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im0(&self) -> IM0_R {
        IM0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im1(&self) -> IM1_R {
        IM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im2(&self) -> IM2_R {
        IM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im3(&self) -> IM3_R {
        IM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im4(&self) -> IM4_R {
        IM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im5(&self) -> IM5_R {
        IM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im6(&self) -> IM6_R {
        IM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im7(&self) -> IM7_R {
        IM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im8(&self) -> IM8_R {
        IM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im9(&self) -> IM9_R {
        IM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im10(&self) -> IM10_R {
        IM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im11(&self) -> IM11_R {
        IM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im12(&self) -> IM12_R {
        IM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im13(&self) -> IM13_R {
        IM13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im14(&self) -> IM14_R {
        IM14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im15(&self) -> IM15_R {
        IM15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im16(&self) -> IM16_R {
        IM16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im17(&self) -> IM17_R {
        IM17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im18(&self) -> IM18_R {
        IM18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im19(&self) -> IM19_R {
        IM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im20(&self) -> IM20_R {
        IM20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im21(&self) -> IM21_R {
        IM21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    pub fn im22(&self) -> IM22_R {
        IM22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im0(&mut self) -> IM0_W<0> {
        IM0_W::new(self)
    }
    ///Bit 1 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im1(&mut self) -> IM1_W<1> {
        IM1_W::new(self)
    }
    ///Bit 2 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im2(&mut self) -> IM2_W<2> {
        IM2_W::new(self)
    }
    ///Bit 3 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im3(&mut self) -> IM3_W<3> {
        IM3_W::new(self)
    }
    ///Bit 4 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im4(&mut self) -> IM4_W<4> {
        IM4_W::new(self)
    }
    ///Bit 5 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im5(&mut self) -> IM5_W<5> {
        IM5_W::new(self)
    }
    ///Bit 6 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im6(&mut self) -> IM6_W<6> {
        IM6_W::new(self)
    }
    ///Bit 7 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im7(&mut self) -> IM7_W<7> {
        IM7_W::new(self)
    }
    ///Bit 8 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im8(&mut self) -> IM8_W<8> {
        IM8_W::new(self)
    }
    ///Bit 9 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im9(&mut self) -> IM9_W<9> {
        IM9_W::new(self)
    }
    ///Bit 10 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im10(&mut self) -> IM10_W<10> {
        IM10_W::new(self)
    }
    ///Bit 11 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im11(&mut self) -> IM11_W<11> {
        IM11_W::new(self)
    }
    ///Bit 12 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im12(&mut self) -> IM12_W<12> {
        IM12_W::new(self)
    }
    ///Bit 13 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im13(&mut self) -> IM13_W<13> {
        IM13_W::new(self)
    }
    ///Bit 14 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im14(&mut self) -> IM14_W<14> {
        IM14_W::new(self)
    }
    ///Bit 15 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im15(&mut self) -> IM15_W<15> {
        IM15_W::new(self)
    }
    ///Bit 16 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im16(&mut self) -> IM16_W<16> {
        IM16_W::new(self)
    }
    ///Bit 17 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im17(&mut self) -> IM17_W<17> {
        IM17_W::new(self)
    }
    ///Bit 18 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im18(&mut self) -> IM18_W<18> {
        IM18_W::new(self)
    }
    ///Bit 19 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im19(&mut self) -> IM19_W<19> {
        IM19_W::new(self)
    }
    ///Bit 20 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im20(&mut self) -> IM20_W<20> {
        IM20_W::new(self)
    }
    ///Bit 21 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im21(&mut self) -> IM21_W<21> {
        IM21_W::new(self)
    }
    ///Bit 22 - CPU wakeup with interrupt mask on event input
    #[inline(always)]
    #[must_use]
    pub fn im22(&mut self) -> IM22_W<22> {
        IM22_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI CPU wakeup with interrupt mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [imr1](index.html) module
pub struct IMR1_SPEC;
impl crate::RegisterSpec for IMR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [imr1::R](R) reader structure
impl crate::Readable for IMR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [imr1::W](W) writer structure
impl crate::Writable for IMR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IMR1 to value 0xff9e_0000
impl crate::Resettable for IMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0xff9e_0000;
}
