///Register `PWR_WUCR2` reader
pub struct R(crate::R<PWR_WUCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_WUCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_WUCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_WUCR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PWR_WUCR2` writer
pub struct W(crate::W<PWR_WUCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_WUCR2_SPEC>;
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
impl From<crate::W<PWR_WUCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_WUCR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WUPP1` reader - Wakeup pin WKUP1 polarity. This bit must be configured when WUPEN1 = 0.
pub type WUPP1_R = crate::BitReader<bool>;
///Field `WUPP1` writer - Wakeup pin WKUP1 polarity. This bit must be configured when WUPEN1 = 0.
pub type WUPP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WUCR2_SPEC, bool, O>;
///Field `WUPP2` reader - Wakeup pin WKUP2 polarity This bit must be configured when WUPEN2 = 0.
pub type WUPP2_R = crate::BitReader<bool>;
///Field `WUPP2` writer - Wakeup pin WKUP2 polarity This bit must be configured when WUPEN2 = 0.
pub type WUPP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WUCR2_SPEC, bool, O>;
///Field `WUPP3` reader - Wakeup pin WKUP3 polarity This bit must be configured when WUPEN3 = 0.
pub type WUPP3_R = crate::BitReader<bool>;
///Field `WUPP3` writer - Wakeup pin WKUP3 polarity This bit must be configured when WUPEN3 = 0.
pub type WUPP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WUCR2_SPEC, bool, O>;
///Field `WUPP4` reader - Wakeup pin WKUP4 polarity This bit must be configured when WUPEN4 = 0.
pub type WUPP4_R = crate::BitReader<bool>;
///Field `WUPP4` writer - Wakeup pin WKUP4 polarity This bit must be configured when WUPEN4 = 0.
pub type WUPP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WUCR2_SPEC, bool, O>;
///Field `WUPP5` reader - Wakeup pin WKUP5 polarity This bit must be configured when WUPEN5 = 0.
pub type WUPP5_R = crate::BitReader<bool>;
///Field `WUPP5` writer - Wakeup pin WKUP5 polarity This bit must be configured when WUPEN5 = 0.
pub type WUPP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WUCR2_SPEC, bool, O>;
///Field `WUPP6` reader - Wakeup pin WKUP6 polarity This bit must be configured when WUPEN6 = 0.
pub type WUPP6_R = crate::BitReader<bool>;
///Field `WUPP6` writer - Wakeup pin WKUP6 polarity This bit must be configured when WUPEN6 = 0.
pub type WUPP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WUCR2_SPEC, bool, O>;
///Field `WUPP7` reader - Wakeup pin WKUP7 polarity This bit must be configured when WUPEN7 = 0.
pub type WUPP7_R = crate::BitReader<bool>;
///Field `WUPP7` writer - Wakeup pin WKUP7 polarity This bit must be configured when WUPEN7 = 0.
pub type WUPP7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WUCR2_SPEC, bool, O>;
///Field `WUPP8` reader - Wakeup pin WKUP8 polarity This bit must be configured when WUPEN8 = 0.
pub type WUPP8_R = crate::BitReader<bool>;
///Field `WUPP8` writer - Wakeup pin WKUP8 polarity This bit must be configured when WUPEN8 = 0.
pub type WUPP8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_WUCR2_SPEC, bool, O>;
impl R {
    ///Bit 0 - Wakeup pin WKUP1 polarity. This bit must be configured when WUPEN1 = 0.
    #[inline(always)]
    pub fn wupp1(&self) -> WUPP1_R {
        WUPP1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wakeup pin WKUP2 polarity This bit must be configured when WUPEN2 = 0.
    #[inline(always)]
    pub fn wupp2(&self) -> WUPP2_R {
        WUPP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup pin WKUP3 polarity This bit must be configured when WUPEN3 = 0.
    #[inline(always)]
    pub fn wupp3(&self) -> WUPP3_R {
        WUPP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Wakeup pin WKUP4 polarity This bit must be configured when WUPEN4 = 0.
    #[inline(always)]
    pub fn wupp4(&self) -> WUPP4_R {
        WUPP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Wakeup pin WKUP5 polarity This bit must be configured when WUPEN5 = 0.
    #[inline(always)]
    pub fn wupp5(&self) -> WUPP5_R {
        WUPP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Wakeup pin WKUP6 polarity This bit must be configured when WUPEN6 = 0.
    #[inline(always)]
    pub fn wupp6(&self) -> WUPP6_R {
        WUPP6_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Wakeup pin WKUP7 polarity This bit must be configured when WUPEN7 = 0.
    #[inline(always)]
    pub fn wupp7(&self) -> WUPP7_R {
        WUPP7_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Wakeup pin WKUP8 polarity This bit must be configured when WUPEN8 = 0.
    #[inline(always)]
    pub fn wupp8(&self) -> WUPP8_R {
        WUPP8_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Wakeup pin WKUP1 polarity. This bit must be configured when WUPEN1 = 0.
    #[inline(always)]
    #[must_use]
    pub fn wupp1(&mut self) -> WUPP1_W<0> {
        WUPP1_W::new(self)
    }
    ///Bit 1 - Wakeup pin WKUP2 polarity This bit must be configured when WUPEN2 = 0.
    #[inline(always)]
    #[must_use]
    pub fn wupp2(&mut self) -> WUPP2_W<1> {
        WUPP2_W::new(self)
    }
    ///Bit 2 - Wakeup pin WKUP3 polarity This bit must be configured when WUPEN3 = 0.
    #[inline(always)]
    #[must_use]
    pub fn wupp3(&mut self) -> WUPP3_W<2> {
        WUPP3_W::new(self)
    }
    ///Bit 3 - Wakeup pin WKUP4 polarity This bit must be configured when WUPEN4 = 0.
    #[inline(always)]
    #[must_use]
    pub fn wupp4(&mut self) -> WUPP4_W<3> {
        WUPP4_W::new(self)
    }
    ///Bit 4 - Wakeup pin WKUP5 polarity This bit must be configured when WUPEN5 = 0.
    #[inline(always)]
    #[must_use]
    pub fn wupp5(&mut self) -> WUPP5_W<4> {
        WUPP5_W::new(self)
    }
    ///Bit 5 - Wakeup pin WKUP6 polarity This bit must be configured when WUPEN6 = 0.
    #[inline(always)]
    #[must_use]
    pub fn wupp6(&mut self) -> WUPP6_W<5> {
        WUPP6_W::new(self)
    }
    ///Bit 6 - Wakeup pin WKUP7 polarity This bit must be configured when WUPEN7 = 0.
    #[inline(always)]
    #[must_use]
    pub fn wupp7(&mut self) -> WUPP7_W<6> {
        WUPP7_W::new(self)
    }
    ///Bit 7 - Wakeup pin WKUP8 polarity This bit must be configured when WUPEN8 = 0.
    #[inline(always)]
    #[must_use]
    pub fn wupp8(&mut self) -> WUPP8_W<7> {
        WUPP8_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PWR wakeup control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pwr_wucr2](index.html) module
pub struct PWR_WUCR2_SPEC;
impl crate::RegisterSpec for PWR_WUCR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [pwr_wucr2::R](R) reader structure
impl crate::Readable for PWR_WUCR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pwr_wucr2::W](W) writer structure
impl crate::Writable for PWR_WUCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PWR_WUCR2 to value 0
impl crate::Resettable for PWR_WUCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
