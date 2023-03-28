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
///Field `CNT1PRIV` reader - Monotonic counter 1 privilege protection
pub type CNT1PRIV_R = crate::BitReader<bool>;
///Field `CNT1PRIV` writer - Monotonic counter 1 privilege protection
pub type CNT1PRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR_SPEC, bool, O>;
///Field `BKPRWPRIV` reader - Backup registers zone 1 privilege protection
pub type BKPRWPRIV_R = crate::BitReader<bool>;
///Field `BKPRWPRIV` writer - Backup registers zone 1 privilege protection
pub type BKPRWPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR_SPEC, bool, O>;
///Field `BKPWPRIV` reader - Backup registers zone 2 privilege protection
pub type BKPWPRIV_R = crate::BitReader<bool>;
///Field `BKPWPRIV` writer - Backup registers zone 2 privilege protection
pub type BKPWPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR_SPEC, bool, O>;
///Field `TAMPPRIV` reader - Tamper privilege protection (excluding backup registers) Note: Refer to Section 32.3.6: TAMP privilege protection modes for details on the read protection.
pub type TAMPPRIV_R = crate::BitReader<bool>;
///Field `TAMPPRIV` writer - Tamper privilege protection (excluding backup registers) Note: Refer to Section 32.3.6: TAMP privilege protection modes for details on the read protection.
pub type TAMPPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRIVCFGR_SPEC, bool, O>;
impl R {
    ///Bit 15 - Monotonic counter 1 privilege protection
    #[inline(always)]
    pub fn cnt1priv(&self) -> CNT1PRIV_R {
        CNT1PRIV_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 29 - Backup registers zone 1 privilege protection
    #[inline(always)]
    pub fn bkprwpriv(&self) -> BKPRWPRIV_R {
        BKPRWPRIV_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Backup registers zone 2 privilege protection
    #[inline(always)]
    pub fn bkpwpriv(&self) -> BKPWPRIV_R {
        BKPWPRIV_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Tamper privilege protection (excluding backup registers) Note: Refer to Section 32.3.6: TAMP privilege protection modes for details on the read protection.
    #[inline(always)]
    pub fn tamppriv(&self) -> TAMPPRIV_R {
        TAMPPRIV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 15 - Monotonic counter 1 privilege protection
    #[inline(always)]
    #[must_use]
    pub fn cnt1priv(&mut self) -> CNT1PRIV_W<15> {
        CNT1PRIV_W::new(self)
    }
    ///Bit 29 - Backup registers zone 1 privilege protection
    #[inline(always)]
    #[must_use]
    pub fn bkprwpriv(&mut self) -> BKPRWPRIV_W<29> {
        BKPRWPRIV_W::new(self)
    }
    ///Bit 30 - Backup registers zone 2 privilege protection
    #[inline(always)]
    #[must_use]
    pub fn bkpwpriv(&mut self) -> BKPWPRIV_W<30> {
        BKPWPRIV_W::new(self)
    }
    ///Bit 31 - Tamper privilege protection (excluding backup registers) Note: Refer to Section 32.3.6: TAMP privilege protection modes for details on the read protection.
    #[inline(always)]
    #[must_use]
    pub fn tamppriv(&mut self) -> TAMPPRIV_W<31> {
        TAMPPRIV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TAMP privilege configuration register
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
