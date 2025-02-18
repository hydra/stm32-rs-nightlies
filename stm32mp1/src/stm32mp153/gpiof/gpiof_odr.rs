///Register `GPIOF_ODR` reader
pub struct R(crate::R<GPIOF_ODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOF_ODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOF_ODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOF_ODR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GPIOF_ODR` writer
pub struct W(crate::W<GPIOF_ODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOF_ODR_SPEC>;
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
impl From<crate::W<GPIOF_ODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOF_ODR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ODR0` reader - ODR0
pub type ODR0_R = crate::BitReader<bool>;
///Field `ODR0` writer - ODR0
pub type ODR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOF_ODR_SPEC, bool, O>;
///Field `ODR1` reader - ODR1
pub type ODR1_R = crate::BitReader<bool>;
///Field `ODR1` writer - ODR1
pub type ODR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOF_ODR_SPEC, bool, O>;
///Field `ODR2` reader - ODR2
pub type ODR2_R = crate::BitReader<bool>;
///Field `ODR2` writer - ODR2
pub type ODR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOF_ODR_SPEC, bool, O>;
///Field `ODR3` reader - ODR3
pub type ODR3_R = crate::BitReader<bool>;
///Field `ODR3` writer - ODR3
pub type ODR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOF_ODR_SPEC, bool, O>;
///Field `ODR4` reader - ODR4
pub type ODR4_R = crate::BitReader<bool>;
///Field `ODR4` writer - ODR4
pub type ODR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOF_ODR_SPEC, bool, O>;
///Field `ODR5` reader - ODR5
pub type ODR5_R = crate::BitReader<bool>;
///Field `ODR5` writer - ODR5
pub type ODR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOF_ODR_SPEC, bool, O>;
///Field `ODR6` reader - ODR6
pub type ODR6_R = crate::BitReader<bool>;
///Field `ODR6` writer - ODR6
pub type ODR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOF_ODR_SPEC, bool, O>;
///Field `ODR7` reader - ODR7
pub type ODR7_R = crate::BitReader<bool>;
///Field `ODR7` writer - ODR7
pub type ODR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOF_ODR_SPEC, bool, O>;
///Field `ODR8` reader - ODR8
pub type ODR8_R = crate::BitReader<bool>;
///Field `ODR8` writer - ODR8
pub type ODR8_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOF_ODR_SPEC, bool, O>;
///Field `ODR9` reader - ODR9
pub type ODR9_R = crate::BitReader<bool>;
///Field `ODR9` writer - ODR9
pub type ODR9_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOF_ODR_SPEC, bool, O>;
///Field `ODR10` reader - ODR10
pub type ODR10_R = crate::BitReader<bool>;
///Field `ODR10` writer - ODR10
pub type ODR10_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOF_ODR_SPEC, bool, O>;
///Field `ODR11` reader - ODR11
pub type ODR11_R = crate::BitReader<bool>;
///Field `ODR11` writer - ODR11
pub type ODR11_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOF_ODR_SPEC, bool, O>;
///Field `ODR12` reader - ODR12
pub type ODR12_R = crate::BitReader<bool>;
///Field `ODR12` writer - ODR12
pub type ODR12_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOF_ODR_SPEC, bool, O>;
///Field `ODR13` reader - ODR13
pub type ODR13_R = crate::BitReader<bool>;
///Field `ODR13` writer - ODR13
pub type ODR13_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOF_ODR_SPEC, bool, O>;
///Field `ODR14` reader - ODR14
pub type ODR14_R = crate::BitReader<bool>;
///Field `ODR14` writer - ODR14
pub type ODR14_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOF_ODR_SPEC, bool, O>;
///Field `ODR15` reader - ODR15
pub type ODR15_R = crate::BitReader<bool>;
///Field `ODR15` writer - ODR15
pub type ODR15_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIOF_ODR_SPEC, bool, O>;
impl R {
    ///Bit 0 - ODR0
    #[inline(always)]
    pub fn odr0(&self) -> ODR0_R {
        ODR0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ODR1
    #[inline(always)]
    pub fn odr1(&self) -> ODR1_R {
        ODR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ODR2
    #[inline(always)]
    pub fn odr2(&self) -> ODR2_R {
        ODR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ODR3
    #[inline(always)]
    pub fn odr3(&self) -> ODR3_R {
        ODR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ODR4
    #[inline(always)]
    pub fn odr4(&self) -> ODR4_R {
        ODR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ODR5
    #[inline(always)]
    pub fn odr5(&self) -> ODR5_R {
        ODR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - ODR6
    #[inline(always)]
    pub fn odr6(&self) -> ODR6_R {
        ODR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - ODR7
    #[inline(always)]
    pub fn odr7(&self) -> ODR7_R {
        ODR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ODR8
    #[inline(always)]
    pub fn odr8(&self) -> ODR8_R {
        ODR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ODR9
    #[inline(always)]
    pub fn odr9(&self) -> ODR9_R {
        ODR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ODR10
    #[inline(always)]
    pub fn odr10(&self) -> ODR10_R {
        ODR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - ODR11
    #[inline(always)]
    pub fn odr11(&self) -> ODR11_R {
        ODR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - ODR12
    #[inline(always)]
    pub fn odr12(&self) -> ODR12_R {
        ODR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ODR13
    #[inline(always)]
    pub fn odr13(&self) -> ODR13_R {
        ODR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ODR14
    #[inline(always)]
    pub fn odr14(&self) -> ODR14_R {
        ODR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - ODR15
    #[inline(always)]
    pub fn odr15(&self) -> ODR15_R {
        ODR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - ODR0
    #[inline(always)]
    #[must_use]
    pub fn odr0(&mut self) -> ODR0_W<0> {
        ODR0_W::new(self)
    }
    ///Bit 1 - ODR1
    #[inline(always)]
    #[must_use]
    pub fn odr1(&mut self) -> ODR1_W<1> {
        ODR1_W::new(self)
    }
    ///Bit 2 - ODR2
    #[inline(always)]
    #[must_use]
    pub fn odr2(&mut self) -> ODR2_W<2> {
        ODR2_W::new(self)
    }
    ///Bit 3 - ODR3
    #[inline(always)]
    #[must_use]
    pub fn odr3(&mut self) -> ODR3_W<3> {
        ODR3_W::new(self)
    }
    ///Bit 4 - ODR4
    #[inline(always)]
    #[must_use]
    pub fn odr4(&mut self) -> ODR4_W<4> {
        ODR4_W::new(self)
    }
    ///Bit 5 - ODR5
    #[inline(always)]
    #[must_use]
    pub fn odr5(&mut self) -> ODR5_W<5> {
        ODR5_W::new(self)
    }
    ///Bit 6 - ODR6
    #[inline(always)]
    #[must_use]
    pub fn odr6(&mut self) -> ODR6_W<6> {
        ODR6_W::new(self)
    }
    ///Bit 7 - ODR7
    #[inline(always)]
    #[must_use]
    pub fn odr7(&mut self) -> ODR7_W<7> {
        ODR7_W::new(self)
    }
    ///Bit 8 - ODR8
    #[inline(always)]
    #[must_use]
    pub fn odr8(&mut self) -> ODR8_W<8> {
        ODR8_W::new(self)
    }
    ///Bit 9 - ODR9
    #[inline(always)]
    #[must_use]
    pub fn odr9(&mut self) -> ODR9_W<9> {
        ODR9_W::new(self)
    }
    ///Bit 10 - ODR10
    #[inline(always)]
    #[must_use]
    pub fn odr10(&mut self) -> ODR10_W<10> {
        ODR10_W::new(self)
    }
    ///Bit 11 - ODR11
    #[inline(always)]
    #[must_use]
    pub fn odr11(&mut self) -> ODR11_W<11> {
        ODR11_W::new(self)
    }
    ///Bit 12 - ODR12
    #[inline(always)]
    #[must_use]
    pub fn odr12(&mut self) -> ODR12_W<12> {
        ODR12_W::new(self)
    }
    ///Bit 13 - ODR13
    #[inline(always)]
    #[must_use]
    pub fn odr13(&mut self) -> ODR13_W<13> {
        ODR13_W::new(self)
    }
    ///Bit 14 - ODR14
    #[inline(always)]
    #[must_use]
    pub fn odr14(&mut self) -> ODR14_W<14> {
        ODR14_W::new(self)
    }
    ///Bit 15 - ODR15
    #[inline(always)]
    #[must_use]
    pub fn odr15(&mut self) -> ODR15_W<15> {
        ODR15_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO port output data register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [gpiof_odr](index.html) module
pub struct GPIOF_ODR_SPEC;
impl crate::RegisterSpec for GPIOF_ODR_SPEC {
    type Ux = u32;
}
///`read()` method returns [gpiof_odr::R](R) reader structure
impl crate::Readable for GPIOF_ODR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [gpiof_odr::W](W) writer structure
impl crate::Writable for GPIOF_ODR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GPIOF_ODR to value 0
impl crate::Resettable for GPIOF_ODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
