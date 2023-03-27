///Register `PRIVCFGR1` reader
pub struct R(crate::R<PRIVCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIVCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIVCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIVCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PRIVCFGR1` writer
pub struct W(crate::W<PRIVCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIVCFGR1_SPEC>;
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
impl From<crate::W<PRIVCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIVCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AESPRIV` reader - AESPRIV
pub type AESPRIV_R = crate::BitReader<bool>;
///Field `AESPRIV` writer - AESPRIV
pub type AESPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `RNGPRIV` reader - RNGPRIV
pub type RNGPRIV_R = crate::BitReader<bool>;
///Field `RNGPRIV` writer - RNGPRIV
pub type RNGPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `SUBGHZSPIPRIV` reader - SUBGHZSPIPRIV
pub type SUBGHZSPIPRIV_R = crate::BitReader<bool>;
///Field `SUBGHZSPIPRIV` writer - SUBGHZSPIPRIV
pub type SUBGHZSPIPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
///Field `PKAPRIV` reader - PKAPRIV
pub type PKAPRIV_R = crate::BitReader<bool>;
///Field `PKAPRIV` writer - PKAPRIV
pub type PKAPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR1_SPEC, bool, O>;
impl R {
    ///Bit 2 - AESPRIV
    #[inline(always)]
    pub fn aespriv(&self) -> AESPRIV_R {
        AESPRIV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RNGPRIV
    #[inline(always)]
    pub fn rngpriv(&self) -> RNGPRIV_R {
        RNGPRIV_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - SUBGHZSPIPRIV
    #[inline(always)]
    pub fn subghzspipriv(&self) -> SUBGHZSPIPRIV_R {
        SUBGHZSPIPRIV_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 13 - PKAPRIV
    #[inline(always)]
    pub fn pkapriv(&self) -> PKAPRIV_R {
        PKAPRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - AESPRIV
    #[inline(always)]
    #[must_use]
    pub fn aespriv(&mut self) -> AESPRIV_W<2> {
        AESPRIV_W::new(self)
    }
    ///Bit 3 - RNGPRIV
    #[inline(always)]
    #[must_use]
    pub fn rngpriv(&mut self) -> RNGPRIV_W<3> {
        RNGPRIV_W::new(self)
    }
    ///Bit 4 - SUBGHZSPIPRIV
    #[inline(always)]
    #[must_use]
    pub fn subghzspipriv(&mut self) -> SUBGHZSPIPRIV_W<4> {
        SUBGHZSPIPRIV_W::new(self)
    }
    ///Bit 13 - PKAPRIV
    #[inline(always)]
    #[must_use]
    pub fn pkapriv(&mut self) -> PKAPRIV_W<13> {
        PKAPRIV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TZSC privilege configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [privcfgr1](index.html) module
pub struct PRIVCFGR1_SPEC;
impl crate::RegisterSpec for PRIVCFGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [privcfgr1::R](R) reader structure
impl crate::Readable for PRIVCFGR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [privcfgr1::W](W) writer structure
impl crate::Writable for PRIVCFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PRIVCFGR1 to value 0
impl crate::Resettable for PRIVCFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
