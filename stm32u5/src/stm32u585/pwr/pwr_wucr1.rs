///Register `PWR_WUCR1` reader
pub struct R(crate::R<PWR_WUCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_WUCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_WUCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_WUCR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PWR_WUCR1` writer
pub struct W(crate::W<PWR_WUCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_WUCR1_SPEC>;
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
impl From<crate::W<PWR_WUCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_WUCR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WUPEN1` reader - Wakeup pin WKUP1 enable
pub type WUPEN1_R = crate::BitReader<bool>;
///Field `WUPEN1` writer - Wakeup pin WKUP1 enable
pub type WUPEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WUCR1_SPEC, bool, O>;
///Field `WUPEN2` reader - Wakeup pin WKUP2 enable
pub type WUPEN2_R = crate::BitReader<bool>;
///Field `WUPEN2` writer - Wakeup pin WKUP2 enable
pub type WUPEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WUCR1_SPEC, bool, O>;
///Field `WUPEN3` reader - Wakeup pin WKUP3 enable
pub type WUPEN3_R = crate::BitReader<bool>;
///Field `WUPEN3` writer - Wakeup pin WKUP3 enable
pub type WUPEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WUCR1_SPEC, bool, O>;
///Field `WUPEN4` reader - Wakeup pin WKUP4 enable
pub type WUPEN4_R = crate::BitReader<bool>;
///Field `WUPEN4` writer - Wakeup pin WKUP4 enable
pub type WUPEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WUCR1_SPEC, bool, O>;
///Field `WUPEN5` reader - Wakeup pin WKUP5 enable
pub type WUPEN5_R = crate::BitReader<bool>;
///Field `WUPEN5` writer - Wakeup pin WKUP5 enable
pub type WUPEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WUCR1_SPEC, bool, O>;
///Field `WUPEN6` reader - Wakeup pin WKUP6 enable
pub type WUPEN6_R = crate::BitReader<bool>;
///Field `WUPEN6` writer - Wakeup pin WKUP6 enable
pub type WUPEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WUCR1_SPEC, bool, O>;
///Field `WUPEN7` reader - Wakeup pin WKUP7 enable
pub type WUPEN7_R = crate::BitReader<bool>;
///Field `WUPEN7` writer - Wakeup pin WKUP7 enable
pub type WUPEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WUCR1_SPEC, bool, O>;
///Field `WUPEN8` reader - Wakeup pin WKUP8 enable
pub type WUPEN8_R = crate::BitReader<bool>;
///Field `WUPEN8` writer - Wakeup pin WKUP8 enable
pub type WUPEN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WUCR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - Wakeup pin WKUP1 enable
    #[inline(always)]
    pub fn wupen1(&self) -> WUPEN1_R {
        WUPEN1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wakeup pin WKUP2 enable
    #[inline(always)]
    pub fn wupen2(&self) -> WUPEN2_R {
        WUPEN2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup pin WKUP3 enable
    #[inline(always)]
    pub fn wupen3(&self) -> WUPEN3_R {
        WUPEN3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Wakeup pin WKUP4 enable
    #[inline(always)]
    pub fn wupen4(&self) -> WUPEN4_R {
        WUPEN4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Wakeup pin WKUP5 enable
    #[inline(always)]
    pub fn wupen5(&self) -> WUPEN5_R {
        WUPEN5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Wakeup pin WKUP6 enable
    #[inline(always)]
    pub fn wupen6(&self) -> WUPEN6_R {
        WUPEN6_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Wakeup pin WKUP7 enable
    #[inline(always)]
    pub fn wupen7(&self) -> WUPEN7_R {
        WUPEN7_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Wakeup pin WKUP8 enable
    #[inline(always)]
    pub fn wupen8(&self) -> WUPEN8_R {
        WUPEN8_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Wakeup pin WKUP1 enable
    #[inline(always)]
    #[must_use]
    pub fn wupen1(&mut self) -> WUPEN1_W<0> {
        WUPEN1_W::new(self)
    }
    ///Bit 1 - Wakeup pin WKUP2 enable
    #[inline(always)]
    #[must_use]
    pub fn wupen2(&mut self) -> WUPEN2_W<1> {
        WUPEN2_W::new(self)
    }
    ///Bit 2 - Wakeup pin WKUP3 enable
    #[inline(always)]
    #[must_use]
    pub fn wupen3(&mut self) -> WUPEN3_W<2> {
        WUPEN3_W::new(self)
    }
    ///Bit 3 - Wakeup pin WKUP4 enable
    #[inline(always)]
    #[must_use]
    pub fn wupen4(&mut self) -> WUPEN4_W<3> {
        WUPEN4_W::new(self)
    }
    ///Bit 4 - Wakeup pin WKUP5 enable
    #[inline(always)]
    #[must_use]
    pub fn wupen5(&mut self) -> WUPEN5_W<4> {
        WUPEN5_W::new(self)
    }
    ///Bit 5 - Wakeup pin WKUP6 enable
    #[inline(always)]
    #[must_use]
    pub fn wupen6(&mut self) -> WUPEN6_W<5> {
        WUPEN6_W::new(self)
    }
    ///Bit 6 - Wakeup pin WKUP7 enable
    #[inline(always)]
    #[must_use]
    pub fn wupen7(&mut self) -> WUPEN7_W<6> {
        WUPEN7_W::new(self)
    }
    ///Bit 7 - Wakeup pin WKUP8 enable
    #[inline(always)]
    #[must_use]
    pub fn wupen8(&mut self) -> WUPEN8_W<7> {
        WUPEN8_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PWR wakeup control register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pwr_wucr1](index.html) module
pub struct PWR_WUCR1_SPEC;
impl crate::RegisterSpec for PWR_WUCR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [pwr_wucr1::R](R) reader structure
impl crate::Readable for PWR_WUCR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pwr_wucr1::W](W) writer structure
impl crate::Writable for PWR_WUCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PWR_WUCR1 to value 0
impl crate::Resettable for PWR_WUCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
