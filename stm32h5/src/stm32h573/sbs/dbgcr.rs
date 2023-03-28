///Register `DBGCR` reader
pub struct R(crate::R<DBGCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBGCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBGCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBGCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DBGCR` writer
pub struct W(crate::W<DBGCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBGCR_SPEC>;
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
impl From<crate::W<DBGCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBGCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AP_UNLOCK` reader - access port unlock Write 0xB4 to this bitfield to open the device access port.
pub type AP_UNLOCK_R = crate::FieldReader<u8, u8>;
///Field `AP_UNLOCK` writer - access port unlock Write 0xB4 to this bitfield to open the device access port.
pub type AP_UNLOCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DBGCR_SPEC, u8, u8, 8, O>;
///Field `DBG_UNLOCK` reader - debug unlock when DBG_AUTH_HDPL is reached Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register.
pub type DBG_UNLOCK_R = crate::FieldReader<u8, u8>;
///Field `DBG_UNLOCK` writer - debug unlock when DBG_AUTH_HDPL is reached Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register.
pub type DBG_UNLOCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DBGCR_SPEC, u8, u8, 8, O>;
///Field `DBG_AUTH_HDPL` reader - authenticated debug temporal isolation level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the debug never opens.
pub type DBG_AUTH_HDPL_R = crate::FieldReader<u8, u8>;
///Field `DBG_AUTH_HDPL` writer - authenticated debug temporal isolation level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the debug never opens.
pub type DBG_AUTH_HDPL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DBGCR_SPEC, u8, u8, 8, O>;
///Field `DBG_AUTH_SEC` reader - control debug opening secure/non-secure Write 0xB4 to this bitfield to open debug for secure and non-secure. Writing any other values only open non-secure.
pub type DBG_AUTH_SEC_R = crate::FieldReader<u8, u8>;
///Field `DBG_AUTH_SEC` writer - control debug opening secure/non-secure Write 0xB4 to this bitfield to open debug for secure and non-secure. Writing any other values only open non-secure.
pub type DBG_AUTH_SEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DBGCR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - access port unlock Write 0xB4 to this bitfield to open the device access port.
    #[inline(always)]
    pub fn ap_unlock(&self) -> AP_UNLOCK_R {
        AP_UNLOCK_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - debug unlock when DBG_AUTH_HDPL is reached Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register.
    #[inline(always)]
    pub fn dbg_unlock(&self) -> DBG_UNLOCK_R {
        DBG_UNLOCK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - authenticated debug temporal isolation level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the debug never opens.
    #[inline(always)]
    pub fn dbg_auth_hdpl(&self) -> DBG_AUTH_HDPL_R {
        DBG_AUTH_HDPL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - control debug opening secure/non-secure Write 0xB4 to this bitfield to open debug for secure and non-secure. Writing any other values only open non-secure.
    #[inline(always)]
    pub fn dbg_auth_sec(&self) -> DBG_AUTH_SEC_R {
        DBG_AUTH_SEC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - access port unlock Write 0xB4 to this bitfield to open the device access port.
    #[inline(always)]
    #[must_use]
    pub fn ap_unlock(&mut self) -> AP_UNLOCK_W<0> {
        AP_UNLOCK_W::new(self)
    }
    ///Bits 8:15 - debug unlock when DBG_AUTH_HDPL is reached Write 0xB4 to this bitfield to open the debug when HDPL in SBS_HDPLSR equals to DBG_AUTH_HDPL in this register.
    #[inline(always)]
    #[must_use]
    pub fn dbg_unlock(&mut self) -> DBG_UNLOCK_W<8> {
        DBG_UNLOCK_W::new(self)
    }
    ///Bits 16:23 - authenticated debug temporal isolation level Writing to this bitfield defines at which HDPL the authenticated debug opens. Note: Writing any other values is ignored. Reading any other value means the debug never opens.
    #[inline(always)]
    #[must_use]
    pub fn dbg_auth_hdpl(&mut self) -> DBG_AUTH_HDPL_W<16> {
        DBG_AUTH_HDPL_W::new(self)
    }
    ///Bits 24:31 - control debug opening secure/non-secure Write 0xB4 to this bitfield to open debug for secure and non-secure. Writing any other values only open non-secure.
    #[inline(always)]
    #[must_use]
    pub fn dbg_auth_sec(&mut self) -> DBG_AUTH_SEC_W<24> {
        DBG_AUTH_SEC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SBS debug control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dbgcr](index.html) module
pub struct DBGCR_SPEC;
impl crate::RegisterSpec for DBGCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dbgcr::R](R) reader structure
impl crate::Readable for DBGCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dbgcr::W](W) writer structure
impl crate::Writable for DBGCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DBGCR to value 0
impl crate::Resettable for DBGCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
