///Register `PWR_WUCR3` reader
pub struct R(crate::R<PWR_WUCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_WUCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_WUCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_WUCR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PWR_WUCR3` writer
pub struct W(crate::W<PWR_WUCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_WUCR3_SPEC>;
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
impl From<crate::W<PWR_WUCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_WUCR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WUSEL1` reader - Wakeup pin WKUP1 selection This field must be configured when WUPEN1 = 0.
pub type WUSEL1_R = crate::FieldReader<u8, u8>;
///Field `WUSEL1` writer - Wakeup pin WKUP1 selection This field must be configured when WUPEN1 = 0.
pub type WUSEL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_WUCR3_SPEC, u8, u8, 2, O>;
///Field `WUSEL2` reader - Wakeup pin WKUP2 selection This field must be configured when WUPEN2 = 0.
pub type WUSEL2_R = crate::FieldReader<u8, u8>;
///Field `WUSEL2` writer - Wakeup pin WKUP2 selection This field must be configured when WUPEN2 = 0.
pub type WUSEL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_WUCR3_SPEC, u8, u8, 2, O>;
///Field `WUSEL3` reader - Wakeup pin WKUP3 selection This field must be configured when WUPEN3 = 0.
pub type WUSEL3_R = crate::FieldReader<u8, u8>;
///Field `WUSEL3` writer - Wakeup pin WKUP3 selection This field must be configured when WUPEN3 = 0.
pub type WUSEL3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_WUCR3_SPEC, u8, u8, 2, O>;
///Field `WUSEL4` reader - Wakeup pin WKUP4 selection This field must be configured when WUPEN4 = 0.
pub type WUSEL4_R = crate::FieldReader<u8, u8>;
///Field `WUSEL4` writer - Wakeup pin WKUP4 selection This field must be configured when WUPEN4 = 0.
pub type WUSEL4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_WUCR3_SPEC, u8, u8, 2, O>;
///Field `WUSEL5` reader - Wakeup pin WKUP5 selection This field must be configured when WUPEN5 = 0.
pub type WUSEL5_R = crate::FieldReader<u8, u8>;
///Field `WUSEL5` writer - Wakeup pin WKUP5 selection This field must be configured when WUPEN5 = 0.
pub type WUSEL5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_WUCR3_SPEC, u8, u8, 2, O>;
///Field `WUSEL6` reader - Wakeup pin WKUP6 selection This field must be configured when WUPEN6 = 0.
pub type WUSEL6_R = crate::FieldReader<u8, u8>;
///Field `WUSEL6` writer - Wakeup pin WKUP6 selection This field must be configured when WUPEN6 = 0.
pub type WUSEL6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_WUCR3_SPEC, u8, u8, 2, O>;
///Field `WUSEL7` reader - Wakeup pin WKUP7 selection This field must be configured when WUPEN7 = 0.
pub type WUSEL7_R = crate::FieldReader<u8, u8>;
///Field `WUSEL7` writer - Wakeup pin WKUP7 selection This field must be configured when WUPEN7 = 0.
pub type WUSEL7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_WUCR3_SPEC, u8, u8, 2, O>;
///Field `WUSEL8` reader - Wakeup pin WKUP8 selection This field must be configured when WUPEN8 = 0.
pub type WUSEL8_R = crate::FieldReader<u8, u8>;
///Field `WUSEL8` writer - Wakeup pin WKUP8 selection This field must be configured when WUPEN8 = 0.
pub type WUSEL8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_WUCR3_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:1 - Wakeup pin WKUP1 selection This field must be configured when WUPEN1 = 0.
    #[inline(always)]
    pub fn wusel1(&self) -> WUSEL1_R {
        WUSEL1_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Wakeup pin WKUP2 selection This field must be configured when WUPEN2 = 0.
    #[inline(always)]
    pub fn wusel2(&self) -> WUSEL2_R {
        WUSEL2_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Wakeup pin WKUP3 selection This field must be configured when WUPEN3 = 0.
    #[inline(always)]
    pub fn wusel3(&self) -> WUSEL3_R {
        WUSEL3_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Wakeup pin WKUP4 selection This field must be configured when WUPEN4 = 0.
    #[inline(always)]
    pub fn wusel4(&self) -> WUSEL4_R {
        WUSEL4_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Wakeup pin WKUP5 selection This field must be configured when WUPEN5 = 0.
    #[inline(always)]
    pub fn wusel5(&self) -> WUSEL5_R {
        WUSEL5_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Wakeup pin WKUP6 selection This field must be configured when WUPEN6 = 0.
    #[inline(always)]
    pub fn wusel6(&self) -> WUSEL6_R {
        WUSEL6_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Wakeup pin WKUP7 selection This field must be configured when WUPEN7 = 0.
    #[inline(always)]
    pub fn wusel7(&self) -> WUSEL7_R {
        WUSEL7_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Wakeup pin WKUP8 selection This field must be configured when WUPEN8 = 0.
    #[inline(always)]
    pub fn wusel8(&self) -> WUSEL8_R {
        WUSEL8_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - Wakeup pin WKUP1 selection This field must be configured when WUPEN1 = 0.
    #[inline(always)]
    #[must_use]
    pub fn wusel1(&mut self) -> WUSEL1_W<0> {
        WUSEL1_W::new(self)
    }
    ///Bits 2:3 - Wakeup pin WKUP2 selection This field must be configured when WUPEN2 = 0.
    #[inline(always)]
    #[must_use]
    pub fn wusel2(&mut self) -> WUSEL2_W<2> {
        WUSEL2_W::new(self)
    }
    ///Bits 4:5 - Wakeup pin WKUP3 selection This field must be configured when WUPEN3 = 0.
    #[inline(always)]
    #[must_use]
    pub fn wusel3(&mut self) -> WUSEL3_W<4> {
        WUSEL3_W::new(self)
    }
    ///Bits 6:7 - Wakeup pin WKUP4 selection This field must be configured when WUPEN4 = 0.
    #[inline(always)]
    #[must_use]
    pub fn wusel4(&mut self) -> WUSEL4_W<6> {
        WUSEL4_W::new(self)
    }
    ///Bits 8:9 - Wakeup pin WKUP5 selection This field must be configured when WUPEN5 = 0.
    #[inline(always)]
    #[must_use]
    pub fn wusel5(&mut self) -> WUSEL5_W<8> {
        WUSEL5_W::new(self)
    }
    ///Bits 10:11 - Wakeup pin WKUP6 selection This field must be configured when WUPEN6 = 0.
    #[inline(always)]
    #[must_use]
    pub fn wusel6(&mut self) -> WUSEL6_W<10> {
        WUSEL6_W::new(self)
    }
    ///Bits 12:13 - Wakeup pin WKUP7 selection This field must be configured when WUPEN7 = 0.
    #[inline(always)]
    #[must_use]
    pub fn wusel7(&mut self) -> WUSEL7_W<12> {
        WUSEL7_W::new(self)
    }
    ///Bits 14:15 - Wakeup pin WKUP8 selection This field must be configured when WUPEN8 = 0.
    #[inline(always)]
    #[must_use]
    pub fn wusel8(&mut self) -> WUSEL8_W<14> {
        WUSEL8_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PWR wakeup control register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pwr_wucr3](index.html) module
pub struct PWR_WUCR3_SPEC;
impl crate::RegisterSpec for PWR_WUCR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [pwr_wucr3::R](R) reader structure
impl crate::Readable for PWR_WUCR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pwr_wucr3::W](W) writer structure
impl crate::Writable for PWR_WUCR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PWR_WUCR3 to value 0
impl crate::Resettable for PWR_WUCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
