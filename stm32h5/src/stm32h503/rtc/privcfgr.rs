///Register `PRIVCFGR` reader
pub struct R(crate::R<PRIVCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIVCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIVCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIVCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PRIVCFGR` writer
pub struct W(crate::W<PRIVCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIVCFGR_SPEC>;
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
impl From<crate::W<PRIVCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIVCFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ALRAPRIV` reader - Alarm A and SSR underflow privilege protection
pub type ALRAPRIV_R = crate::BitReader<bool>;
///Field `ALRAPRIV` writer - Alarm A and SSR underflow privilege protection
pub type ALRAPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR_SPEC, bool, O>;
///Field `ALRBPRIV` reader - Alarm B privilege protection
pub type ALRBPRIV_R = crate::BitReader<bool>;
///Field `ALRBPRIV` writer - Alarm B privilege protection
pub type ALRBPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR_SPEC, bool, O>;
///Field `WUTPRIV` reader - Wakeup timer privilege protection
pub type WUTPRIV_R = crate::BitReader<bool>;
///Field `WUTPRIV` writer - Wakeup timer privilege protection
pub type WUTPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR_SPEC, bool, O>;
///Field `TSPRIV` reader - Timestamp privilege protection
pub type TSPRIV_R = crate::BitReader<bool>;
///Field `TSPRIV` writer - Timestamp privilege protection
pub type TSPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR_SPEC, bool, O>;
///Field `CALPRIV` reader - Shift register, Delight saving, calibration and reference clock privilege protection
pub type CALPRIV_R = crate::BitReader<bool>;
///Field `CALPRIV` writer - Shift register, Delight saving, calibration and reference clock privilege protection
pub type CALPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR_SPEC, bool, O>;
///Field `INITPRIV` reader - Initialization privilege protection
pub type INITPRIV_R = crate::BitReader<bool>;
///Field `INITPRIV` writer - Initialization privilege protection
pub type INITPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR_SPEC, bool, O>;
///Field `PRIV` reader - RTC privilege protection
pub type PRIV_R = crate::BitReader<bool>;
///Field `PRIV` writer - RTC privilege protection
pub type PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Alarm A and SSR underflow privilege protection
    #[inline(always)]
    pub fn alrapriv(&self) -> ALRAPRIV_R {
        ALRAPRIV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Alarm B privilege protection
    #[inline(always)]
    pub fn alrbpriv(&self) -> ALRBPRIV_R {
        ALRBPRIV_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup timer privilege protection
    #[inline(always)]
    pub fn wutpriv(&self) -> WUTPRIV_R {
        WUTPRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timestamp privilege protection
    #[inline(always)]
    pub fn tspriv(&self) -> TSPRIV_R {
        TSPRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 13 - Shift register, Delight saving, calibration and reference clock privilege protection
    #[inline(always)]
    pub fn calpriv(&self) -> CALPRIV_R {
        CALPRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Initialization privilege protection
    #[inline(always)]
    pub fn initpriv(&self) -> INITPRIV_R {
        INITPRIV_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - RTC privilege protection
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Alarm A and SSR underflow privilege protection
    #[inline(always)]
    #[must_use]
    pub fn alrapriv(&mut self) -> ALRAPRIV_W<0> {
        ALRAPRIV_W::new(self)
    }
    ///Bit 1 - Alarm B privilege protection
    #[inline(always)]
    #[must_use]
    pub fn alrbpriv(&mut self) -> ALRBPRIV_W<1> {
        ALRBPRIV_W::new(self)
    }
    ///Bit 2 - Wakeup timer privilege protection
    #[inline(always)]
    #[must_use]
    pub fn wutpriv(&mut self) -> WUTPRIV_W<2> {
        WUTPRIV_W::new(self)
    }
    ///Bit 3 - Timestamp privilege protection
    #[inline(always)]
    #[must_use]
    pub fn tspriv(&mut self) -> TSPRIV_W<3> {
        TSPRIV_W::new(self)
    }
    ///Bit 13 - Shift register, Delight saving, calibration and reference clock privilege protection
    #[inline(always)]
    #[must_use]
    pub fn calpriv(&mut self) -> CALPRIV_W<13> {
        CALPRIV_W::new(self)
    }
    ///Bit 14 - Initialization privilege protection
    #[inline(always)]
    #[must_use]
    pub fn initpriv(&mut self) -> INITPRIV_W<14> {
        INITPRIV_W::new(self)
    }
    ///Bit 15 - RTC privilege protection
    #[inline(always)]
    #[must_use]
    pub fn priv_(&mut self) -> PRIV_W<15> {
        PRIV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RTC privilege mode control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [privcfgr](index.html) module
pub struct PRIVCFGR_SPEC;
impl crate::RegisterSpec for PRIVCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [privcfgr::R](R) reader structure
impl crate::Readable for PRIVCFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [privcfgr::W](W) writer structure
impl crate::Writable for PRIVCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PRIVCFGR to value 0
impl crate::Resettable for PRIVCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
