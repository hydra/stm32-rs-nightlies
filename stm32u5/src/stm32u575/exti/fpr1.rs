///Register `FPR1` reader
pub struct R(crate::R<FPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FPR1` writer
pub struct W(crate::W<FPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPR1_SPEC>;
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
impl From<crate::W<FPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FPIF0` reader - configurable event inputs x falling edge pending bit.
pub type FPIF0_R = crate::BitReader<bool>;
///Field `FPIF0` writer - configurable event inputs x falling edge pending bit.
pub type FPIF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR1_SPEC, bool, O>;
///Field `FPIF1` reader - configurable event inputs x falling edge pending bit.
pub type FPIF1_R = crate::BitReader<bool>;
///Field `FPIF1` writer - configurable event inputs x falling edge pending bit.
pub type FPIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR1_SPEC, bool, O>;
///Field `FPIF2` reader - configurable event inputs x falling edge pending bit.
pub type FPIF2_R = crate::BitReader<bool>;
///Field `FPIF2` writer - configurable event inputs x falling edge pending bit.
pub type FPIF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR1_SPEC, bool, O>;
///Field `FPIF3` reader - configurable event inputs x falling edge pending bit.
pub type FPIF3_R = crate::BitReader<bool>;
///Field `FPIF3` writer - configurable event inputs x falling edge pending bit.
pub type FPIF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR1_SPEC, bool, O>;
///Field `FPIF4` reader - configurable event inputs x falling edge pending bit.
pub type FPIF4_R = crate::BitReader<bool>;
///Field `FPIF4` writer - configurable event inputs x falling edge pending bit.
pub type FPIF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR1_SPEC, bool, O>;
///Field `FPIF5` reader - configurable event inputs x falling edge pending bit.
pub type FPIF5_R = crate::BitReader<bool>;
///Field `FPIF5` writer - configurable event inputs x falling edge pending bit.
pub type FPIF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR1_SPEC, bool, O>;
///Field `FPIF6` reader - configurable event inputs x falling edge pending bit.
pub type FPIF6_R = crate::BitReader<bool>;
///Field `FPIF6` writer - configurable event inputs x falling edge pending bit.
pub type FPIF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR1_SPEC, bool, O>;
///Field `FPIF7` reader - configurable event inputs x falling edge pending bit.
pub type FPIF7_R = crate::BitReader<bool>;
///Field `FPIF7` writer - configurable event inputs x falling edge pending bit.
pub type FPIF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR1_SPEC, bool, O>;
///Field `FPIF8` reader - configurable event inputs x falling edge pending bit.
pub type FPIF8_R = crate::BitReader<bool>;
///Field `FPIF8` writer - configurable event inputs x falling edge pending bit.
pub type FPIF8_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR1_SPEC, bool, O>;
///Field `FPIF9` reader - configurable event inputs x falling edge pending bit.
pub type FPIF9_R = crate::BitReader<bool>;
///Field `FPIF9` writer - configurable event inputs x falling edge pending bit.
pub type FPIF9_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR1_SPEC, bool, O>;
///Field `FPIF10` reader - configurable event inputs x falling edge pending bit.
pub type FPIF10_R = crate::BitReader<bool>;
///Field `FPIF10` writer - configurable event inputs x falling edge pending bit.
pub type FPIF10_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR1_SPEC, bool, O>;
///Field `FPIF11` reader - configurable event inputs x falling edge pending bit.
pub type FPIF11_R = crate::BitReader<bool>;
///Field `FPIF11` writer - configurable event inputs x falling edge pending bit.
pub type FPIF11_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR1_SPEC, bool, O>;
///Field `FPIF12` reader - configurable event inputs x falling edge pending bit.
pub type FPIF12_R = crate::BitReader<bool>;
///Field `FPIF12` writer - configurable event inputs x falling edge pending bit.
pub type FPIF12_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR1_SPEC, bool, O>;
///Field `FPIF13` reader - configurable event inputs x falling edge pending bit.
pub type FPIF13_R = crate::BitReader<bool>;
///Field `FPIF13` writer - configurable event inputs x falling edge pending bit.
pub type FPIF13_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR1_SPEC, bool, O>;
///Field `FPIF14` reader - configurable event inputs x falling edge pending bit.
pub type FPIF14_R = crate::BitReader<bool>;
///Field `FPIF14` writer - configurable event inputs x falling edge pending bit.
pub type FPIF14_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR1_SPEC, bool, O>;
///Field `FPIF15` reader - configurable event inputs x falling edge pending bit.
pub type FPIF15_R = crate::BitReader<bool>;
///Field `FPIF15` writer - configurable event inputs x falling edge pending bit.
pub type FPIF15_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR1_SPEC, bool, O>;
///Field `FPIF16` reader - configurable event inputs x falling edge pending bit.
pub type FPIF16_R = crate::BitReader<bool>;
///Field `FPIF16` writer - configurable event inputs x falling edge pending bit.
pub type FPIF16_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR1_SPEC, bool, O>;
///Field `FPIF17` reader - configurable event inputs x falling edge pending bit.
pub type FPIF17_R = crate::BitReader<bool>;
///Field `FPIF17` writer - configurable event inputs x falling edge pending bit.
pub type FPIF17_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR1_SPEC, bool, O>;
///Field `FPIF18` reader - configurable event inputs x falling edge pending bit.
pub type FPIF18_R = crate::BitReader<bool>;
///Field `FPIF18` writer - configurable event inputs x falling edge pending bit.
pub type FPIF18_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR1_SPEC, bool, O>;
///Field `FPIF19` reader - configurable event inputs x falling edge pending bit.
pub type FPIF19_R = crate::BitReader<bool>;
///Field `FPIF19` writer - configurable event inputs x falling edge pending bit.
pub type FPIF19_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR1_SPEC, bool, O>;
///Field `FPIF20` reader - configurable event inputs x falling edge pending bit.
pub type FPIF20_R = crate::BitReader<bool>;
///Field `FPIF20` writer - configurable event inputs x falling edge pending bit.
pub type FPIF20_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR1_SPEC, bool, O>;
///Field `FPIF21` reader - configurable event inputs x falling edge pending bit.
pub type FPIF21_R = crate::BitReader<bool>;
///Field `FPIF21` writer - configurable event inputs x falling edge pending bit.
pub type FPIF21_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR1_SPEC, bool, O>;
///Field `FPIF22` reader - configurable event inputs x falling edge pending bit.
pub type FPIF22_R = crate::BitReader<bool>;
///Field `FPIF22` writer - configurable event inputs x falling edge pending bit.
pub type FPIF22_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif0(&self) -> FPIF0_R {
        FPIF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif1(&self) -> FPIF1_R {
        FPIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif2(&self) -> FPIF2_R {
        FPIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif3(&self) -> FPIF3_R {
        FPIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif4(&self) -> FPIF4_R {
        FPIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif5(&self) -> FPIF5_R {
        FPIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif6(&self) -> FPIF6_R {
        FPIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif7(&self) -> FPIF7_R {
        FPIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif8(&self) -> FPIF8_R {
        FPIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif9(&self) -> FPIF9_R {
        FPIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif10(&self) -> FPIF10_R {
        FPIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif11(&self) -> FPIF11_R {
        FPIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif12(&self) -> FPIF12_R {
        FPIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif13(&self) -> FPIF13_R {
        FPIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif14(&self) -> FPIF14_R {
        FPIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif15(&self) -> FPIF15_R {
        FPIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif16(&self) -> FPIF16_R {
        FPIF16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif17(&self) -> FPIF17_R {
        FPIF17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif18(&self) -> FPIF18_R {
        FPIF18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif19(&self) -> FPIF19_R {
        FPIF19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif20(&self) -> FPIF20_R {
        FPIF20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif21(&self) -> FPIF21_R {
        FPIF21_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    pub fn fpif22(&self) -> FPIF22_R {
        FPIF22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif0(&mut self) -> FPIF0_W<0> {
        FPIF0_W::new(self)
    }
    ///Bit 1 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif1(&mut self) -> FPIF1_W<1> {
        FPIF1_W::new(self)
    }
    ///Bit 2 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif2(&mut self) -> FPIF2_W<2> {
        FPIF2_W::new(self)
    }
    ///Bit 3 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif3(&mut self) -> FPIF3_W<3> {
        FPIF3_W::new(self)
    }
    ///Bit 4 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif4(&mut self) -> FPIF4_W<4> {
        FPIF4_W::new(self)
    }
    ///Bit 5 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif5(&mut self) -> FPIF5_W<5> {
        FPIF5_W::new(self)
    }
    ///Bit 6 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif6(&mut self) -> FPIF6_W<6> {
        FPIF6_W::new(self)
    }
    ///Bit 7 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif7(&mut self) -> FPIF7_W<7> {
        FPIF7_W::new(self)
    }
    ///Bit 8 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif8(&mut self) -> FPIF8_W<8> {
        FPIF8_W::new(self)
    }
    ///Bit 9 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif9(&mut self) -> FPIF9_W<9> {
        FPIF9_W::new(self)
    }
    ///Bit 10 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif10(&mut self) -> FPIF10_W<10> {
        FPIF10_W::new(self)
    }
    ///Bit 11 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif11(&mut self) -> FPIF11_W<11> {
        FPIF11_W::new(self)
    }
    ///Bit 12 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif12(&mut self) -> FPIF12_W<12> {
        FPIF12_W::new(self)
    }
    ///Bit 13 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif13(&mut self) -> FPIF13_W<13> {
        FPIF13_W::new(self)
    }
    ///Bit 14 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif14(&mut self) -> FPIF14_W<14> {
        FPIF14_W::new(self)
    }
    ///Bit 15 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif15(&mut self) -> FPIF15_W<15> {
        FPIF15_W::new(self)
    }
    ///Bit 16 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif16(&mut self) -> FPIF16_W<16> {
        FPIF16_W::new(self)
    }
    ///Bit 17 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif17(&mut self) -> FPIF17_W<17> {
        FPIF17_W::new(self)
    }
    ///Bit 18 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif18(&mut self) -> FPIF18_W<18> {
        FPIF18_W::new(self)
    }
    ///Bit 19 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif19(&mut self) -> FPIF19_W<19> {
        FPIF19_W::new(self)
    }
    ///Bit 20 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif20(&mut self) -> FPIF20_W<20> {
        FPIF20_W::new(self)
    }
    ///Bit 21 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif21(&mut self) -> FPIF21_W<21> {
        FPIF21_W::new(self)
    }
    ///Bit 22 - configurable event inputs x falling edge pending bit.
    #[inline(always)]
    #[must_use]
    pub fn fpif22(&mut self) -> FPIF22_W<22> {
        FPIF22_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI falling edge pending register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fpr1](index.html) module
pub struct FPR1_SPEC;
impl crate::RegisterSpec for FPR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [fpr1::R](R) reader structure
impl crate::Readable for FPR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fpr1::W](W) writer structure
impl crate::Writable for FPR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FPR1 to value 0
impl crate::Resettable for FPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
