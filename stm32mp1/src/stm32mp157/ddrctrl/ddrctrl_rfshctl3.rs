///Register `DDRCTRL_RFSHCTL3` reader
pub struct R(crate::R<DDRCTRL_RFSHCTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_RFSHCTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_RFSHCTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_RFSHCTL3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DDRCTRL_RFSHCTL3` writer
pub struct W(crate::W<DDRCTRL_RFSHCTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_RFSHCTL3_SPEC>;
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
impl From<crate::W<DDRCTRL_RFSHCTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_RFSHCTL3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DIS_AUTO_REFRESH` reader - DIS_AUTO_REFRESH
pub type DIS_AUTO_REFRESH_R = crate::BitReader<bool>;
///Field `DIS_AUTO_REFRESH` writer - DIS_AUTO_REFRESH
pub type DIS_AUTO_REFRESH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_RFSHCTL3_SPEC, bool, O>;
///Field `REFRESH_UPDATE_LEVEL` reader - REFRESH_UPDATE_LEVEL
pub type REFRESH_UPDATE_LEVEL_R = crate::BitReader<bool>;
///Field `REFRESH_UPDATE_LEVEL` writer - REFRESH_UPDATE_LEVEL
pub type REFRESH_UPDATE_LEVEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DDRCTRL_RFSHCTL3_SPEC, bool, O>;
impl R {
    ///Bit 0 - DIS_AUTO_REFRESH
    #[inline(always)]
    pub fn dis_auto_refresh(&self) -> DIS_AUTO_REFRESH_R {
        DIS_AUTO_REFRESH_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - REFRESH_UPDATE_LEVEL
    #[inline(always)]
    pub fn refresh_update_level(&self) -> REFRESH_UPDATE_LEVEL_R {
        REFRESH_UPDATE_LEVEL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DIS_AUTO_REFRESH
    #[inline(always)]
    #[must_use]
    pub fn dis_auto_refresh(&mut self) -> DIS_AUTO_REFRESH_W<0> {
        DIS_AUTO_REFRESH_W::new(self)
    }
    ///Bit 1 - REFRESH_UPDATE_LEVEL
    #[inline(always)]
    #[must_use]
    pub fn refresh_update_level(&mut self) -> REFRESH_UPDATE_LEVEL_W<1> {
        REFRESH_UPDATE_LEVEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DDRCTRL refresh control register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrctrl_rfshctl3](index.html) module
pub struct DDRCTRL_RFSHCTL3_SPEC;
impl crate::RegisterSpec for DDRCTRL_RFSHCTL3_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrctrl_rfshctl3::R](R) reader structure
impl crate::Readable for DDRCTRL_RFSHCTL3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ddrctrl_rfshctl3::W](W) writer structure
impl crate::Writable for DDRCTRL_RFSHCTL3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DDRCTRL_RFSHCTL3 to value 0
impl crate::Resettable for DDRCTRL_RFSHCTL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
