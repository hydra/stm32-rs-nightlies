///Register `ODR` reader
pub struct R(crate::R<ODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ODR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ODR` writer
pub struct W(crate::W<ODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ODR_SPEC>;
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
impl From<crate::W<ODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ODR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ODR0` reader - Port output data (y = 0..15)
pub type ODR0_R = crate::BitReader<bool>;
///Field `ODR0` writer - Port output data (y = 0..15)
pub type ODR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
///Field `ODR1` reader - Port output data (y = 0..15)
pub type ODR1_R = crate::BitReader<bool>;
///Field `ODR1` writer - Port output data (y = 0..15)
pub type ODR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
///Field `ODR2` reader - Port output data (y = 0..15)
pub type ODR2_R = crate::BitReader<bool>;
///Field `ODR2` writer - Port output data (y = 0..15)
pub type ODR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
///Field `ODR3` reader - Port output data (y = 0..15)
pub type ODR3_R = crate::BitReader<bool>;
///Field `ODR3` writer - Port output data (y = 0..15)
pub type ODR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
///Field `ODR4` reader - Port output data (y = 0..15)
pub type ODR4_R = crate::BitReader<bool>;
///Field `ODR4` writer - Port output data (y = 0..15)
pub type ODR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ODR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr0(&self) -> ODR0_R {
        ODR0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr1(&self) -> ODR1_R {
        ODR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr2(&self) -> ODR2_R {
        ODR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr3(&self) -> ODR3_R {
        ODR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port output data (y = 0..15)
    #[inline(always)]
    pub fn odr4(&self) -> ODR4_R {
        ODR4_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Port output data (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn odr0(&mut self) -> ODR0_W<0> {
        ODR0_W::new(self)
    }
    ///Bit 1 - Port output data (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn odr1(&mut self) -> ODR1_W<1> {
        ODR1_W::new(self)
    }
    ///Bit 2 - Port output data (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn odr2(&mut self) -> ODR2_W<2> {
        ODR2_W::new(self)
    }
    ///Bit 3 - Port output data (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn odr3(&mut self) -> ODR3_W<3> {
        ODR3_W::new(self)
    }
    ///Bit 4 - Port output data (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn odr4(&mut self) -> ODR4_W<4> {
        ODR4_W::new(self)
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
///For information about available fields see [odr](index.html) module
pub struct ODR_SPEC;
impl crate::RegisterSpec for ODR_SPEC {
    type Ux = u32;
}
///`read()` method returns [odr::R](R) reader structure
impl crate::Readable for ODR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [odr::W](W) writer structure
impl crate::Writable for ODR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ODR to value 0
impl crate::Resettable for ODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
