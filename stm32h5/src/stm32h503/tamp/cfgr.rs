///Register `CFGR` reader
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR` writer
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BKPRW` reader - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRW-1, from 0 to 128). Note: If BKPRW = 0: there is no protection zone 1. Note: If BKPRWPRIV is set, BKPRW\[7:0\]
///can be written only in privileged mode.
pub type BKPRW_R = crate::FieldReader<u8, u8>;
///Field `BKPRW` writer - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRW-1, from 0 to 128). Note: If BKPRW = 0: there is no protection zone 1. Note: If BKPRWPRIV is set, BKPRW\[7:0\]
///can be written only in privileged mode.
pub type BKPRW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 8, O>;
///Field `BKPW` reader - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRW, from 0 to 128) to TAMP_BKPzR (z = BKPW-1, from 0 to 128, BKPW ≥ BKPRW): Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPW, from 0 to 127). Note: If BKPW = 0 or if BKPW ≤ BKPRW: there is no protection zone 2. Note: If BKPWPRIV is set, BKPRW\[7:0\]
///can be written only in privileged mode.
pub type BKPW_R = crate::FieldReader<u8, u8>;
///Field `BKPW` writer - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRW, from 0 to 128) to TAMP_BKPzR (z = BKPW-1, from 0 to 128, BKPW ≥ BKPRW): Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPW, from 0 to 127). Note: If BKPW = 0 or if BKPW ≤ BKPRW: there is no protection zone 2. Note: If BKPWPRIV is set, BKPRW\[7:0\]
///can be written only in privileged mode.
pub type BKPW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRW-1, from 0 to 128). Note: If BKPRW = 0: there is no protection zone 1. Note: If BKPRWPRIV is set, BKPRW\[7:0\]
    ///can be written only in privileged mode.
    #[inline(always)]
    pub fn bkprw(&self) -> BKPRW_R {
        BKPRW_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRW, from 0 to 128) to TAMP_BKPzR (z = BKPW-1, from 0 to 128, BKPW ≥ BKPRW): Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPW, from 0 to 127). Note: If BKPW = 0 or if BKPW ≤ BKPRW: there is no protection zone 2. Note: If BKPWPRIV is set, BKPRW\[7:0\]
    ///can be written only in privileged mode.
    #[inline(always)]
    pub fn bkpw(&self) -> BKPW_R {
        BKPW_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRW-1, from 0 to 128). Note: If BKPRW = 0: there is no protection zone 1. Note: If BKPRWPRIV is set, BKPRW\[7:0\]
    ///can be written only in privileged mode.
    #[inline(always)]
    #[must_use]
    pub fn bkprw(&mut self) -> BKPRW_W<0> {
        BKPRW_W::new(self)
    }
    ///Bits 16:23 - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRW, from 0 to 128) to TAMP_BKPzR (z = BKPW-1, from 0 to 128, BKPW ≥ BKPRW): Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPW, from 0 to 127). Note: If BKPW = 0 or if BKPW ≤ BKPRW: there is no protection zone 2. Note: If BKPWPRIV is set, BKPRW\[7:0\]
    ///can be written only in privileged mode.
    #[inline(always)]
    #[must_use]
    pub fn bkpw(&mut self) -> BKPW_W<16> {
        BKPW_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TAMP configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr](index.html) module
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr::R](R) reader structure
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr::W](W) writer structure
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CFGR to value 0
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
