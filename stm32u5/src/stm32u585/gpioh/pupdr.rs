///Register `PUPDR` reader
pub struct R(crate::R<PUPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUPDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PUPDR` writer
pub struct W(crate::W<PUPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUPDR_SPEC>;
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
impl From<crate::W<PUPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUPDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PUPD0` reader - Port x configuration bits (y = 0..15)
pub type PUPD0_R = crate::FieldReader<u8, u8>;
///Field `PUPD0` writer - Port x configuration bits (y = 0..15)
pub type PUPD0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
///Field `PUPD1` reader - Port x configuration bits (y = 0..15)
pub type PUPD1_R = crate::FieldReader<u8, u8>;
///Field `PUPD1` writer - Port x configuration bits (y = 0..15)
pub type PUPD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
///Field `PUPD2` reader - Port x configuration bits (y = 0..15)
pub type PUPD2_R = crate::FieldReader<u8, u8>;
///Field `PUPD2` writer - Port x configuration bits (y = 0..15)
pub type PUPD2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
///Field `PUPD3` reader - Port x configuration bits (y = 0..15)
pub type PUPD3_R = crate::FieldReader<u8, u8>;
///Field `PUPD3` writer - Port x configuration bits (y = 0..15)
pub type PUPD3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
///Field `PUPD4` reader - Port x configuration bits (y = 0..15)
pub type PUPD4_R = crate::FieldReader<u8, u8>;
///Field `PUPD4` writer - Port x configuration bits (y = 0..15)
pub type PUPD4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
///Field `PUPD5` reader - Port x configuration bits (y = 0..15)
pub type PUPD5_R = crate::FieldReader<u8, u8>;
///Field `PUPD5` writer - Port x configuration bits (y = 0..15)
pub type PUPD5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
///Field `PUPD6` reader - Port x configuration bits (y = 0..15)
pub type PUPD6_R = crate::FieldReader<u8, u8>;
///Field `PUPD6` writer - Port x configuration bits (y = 0..15)
pub type PUPD6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
///Field `PUPD7` reader - Port x configuration bits (y = 0..15)
pub type PUPD7_R = crate::FieldReader<u8, u8>;
///Field `PUPD7` writer - Port x configuration bits (y = 0..15)
pub type PUPD7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
///Field `PUPD8` reader - Port x configuration bits (y = 0..15)
pub type PUPD8_R = crate::FieldReader<u8, u8>;
///Field `PUPD8` writer - Port x configuration bits (y = 0..15)
pub type PUPD8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
///Field `PUPD9` reader - Port x configuration bits (y = 0..15)
pub type PUPD9_R = crate::FieldReader<u8, u8>;
///Field `PUPD9` writer - Port x configuration bits (y = 0..15)
pub type PUPD9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
///Field `PUPD10` reader - Port x configuration bits (y = 0..15)
pub type PUPD10_R = crate::FieldReader<u8, u8>;
///Field `PUPD10` writer - Port x configuration bits (y = 0..15)
pub type PUPD10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
///Field `PUPD11` reader - Port x configuration bits (y = 0..15)
pub type PUPD11_R = crate::FieldReader<u8, u8>;
///Field `PUPD11` writer - Port x configuration bits (y = 0..15)
pub type PUPD11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
///Field `PUPD12` reader - Port x configuration bits (y = 0..15)
pub type PUPD12_R = crate::FieldReader<u8, u8>;
///Field `PUPD12` writer - Port x configuration bits (y = 0..15)
pub type PUPD12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
///Field `PUPD13` reader - Port x configuration bits (y = 0..15)
pub type PUPD13_R = crate::FieldReader<u8, u8>;
///Field `PUPD13` writer - Port x configuration bits (y = 0..15)
pub type PUPD13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
///Field `PUPD14` reader - Port x configuration bits (y = 0..15)
pub type PUPD14_R = crate::FieldReader<u8, u8>;
///Field `PUPD14` writer - Port x configuration bits (y = 0..15)
pub type PUPD14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
///Field `PUPD15` reader - Port x configuration bits (y = 0..15)
pub type PUPD15_R = crate::FieldReader<u8, u8>;
///Field `PUPD15` writer - Port x configuration bits (y = 0..15)
pub type PUPD15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PUPDR_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupd0(&self) -> PUPD0_R {
        PUPD0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupd1(&self) -> PUPD1_R {
        PUPD1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupd2(&self) -> PUPD2_R {
        PUPD2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupd3(&self) -> PUPD3_R {
        PUPD3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupd4(&self) -> PUPD4_R {
        PUPD4_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupd5(&self) -> PUPD5_R {
        PUPD5_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupd6(&self) -> PUPD6_R {
        PUPD6_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupd7(&self) -> PUPD7_R {
        PUPD7_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupd8(&self) -> PUPD8_R {
        PUPD8_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupd9(&self) -> PUPD9_R {
        PUPD9_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupd10(&self) -> PUPD10_R {
        PUPD10_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupd11(&self) -> PUPD11_R {
        PUPD11_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupd12(&self) -> PUPD12_R {
        PUPD12_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupd13(&self) -> PUPD13_R {
        PUPD13_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupd14(&self) -> PUPD14_R {
        PUPD14_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn pupd15(&self) -> PUPD15_R {
        PUPD15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupd0(&mut self) -> PUPD0_W<0> {
        PUPD0_W::new(self)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupd1(&mut self) -> PUPD1_W<2> {
        PUPD1_W::new(self)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupd2(&mut self) -> PUPD2_W<4> {
        PUPD2_W::new(self)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupd3(&mut self) -> PUPD3_W<6> {
        PUPD3_W::new(self)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupd4(&mut self) -> PUPD4_W<8> {
        PUPD4_W::new(self)
    }
    ///Bits 10:11 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupd5(&mut self) -> PUPD5_W<10> {
        PUPD5_W::new(self)
    }
    ///Bits 12:13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupd6(&mut self) -> PUPD6_W<12> {
        PUPD6_W::new(self)
    }
    ///Bits 14:15 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupd7(&mut self) -> PUPD7_W<14> {
        PUPD7_W::new(self)
    }
    ///Bits 16:17 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupd8(&mut self) -> PUPD8_W<16> {
        PUPD8_W::new(self)
    }
    ///Bits 18:19 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupd9(&mut self) -> PUPD9_W<18> {
        PUPD9_W::new(self)
    }
    ///Bits 20:21 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupd10(&mut self) -> PUPD10_W<20> {
        PUPD10_W::new(self)
    }
    ///Bits 22:23 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupd11(&mut self) -> PUPD11_W<22> {
        PUPD11_W::new(self)
    }
    ///Bits 24:25 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupd12(&mut self) -> PUPD12_W<24> {
        PUPD12_W::new(self)
    }
    ///Bits 26:27 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupd13(&mut self) -> PUPD13_W<26> {
        PUPD13_W::new(self)
    }
    ///Bits 28:29 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupd14(&mut self) -> PUPD14_W<28> {
        PUPD14_W::new(self)
    }
    ///Bits 30:31 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn pupd15(&mut self) -> PUPD15_W<30> {
        PUPD15_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO port pull-up/pull-down register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pupdr](index.html) module
pub struct PUPDR_SPEC;
impl crate::RegisterSpec for PUPDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pupdr::R](R) reader structure
impl crate::Readable for PUPDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pupdr::W](W) writer structure
impl crate::Writable for PUPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PUPDR to value 0
impl crate::Resettable for PUPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
