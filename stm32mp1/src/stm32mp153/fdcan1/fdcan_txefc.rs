///Register `FDCAN_TXEFC` reader
pub struct R(crate::R<FDCAN_TXEFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TXEFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TXEFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TXEFC_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FDCAN_TXEFC` writer
pub struct W(crate::W<FDCAN_TXEFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TXEFC_SPEC>;
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
impl From<crate::W<FDCAN_TXEFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TXEFC_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EFSA` reader - EFSA
pub type EFSA_R = crate::FieldReader<u16, u16>;
///Field `EFSA` writer - EFSA
pub type EFSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TXEFC_SPEC, u16, u16, 14, O>;
///Field `EFS` reader - EFS
pub type EFS_R = crate::FieldReader<u8, u8>;
///Field `EFS` writer - EFS
pub type EFS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TXEFC_SPEC, u8, u8, 6, O>;
///Field `EFWM` reader - EFWM
pub type EFWM_R = crate::FieldReader<u8, u8>;
///Field `EFWM` writer - EFWM
pub type EFWM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TXEFC_SPEC, u8, u8, 6, O>;
impl R {
    ///Bits 2:15 - EFSA
    #[inline(always)]
    pub fn efsa(&self) -> EFSA_R {
        EFSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    ///Bits 16:21 - EFS
    #[inline(always)]
    pub fn efs(&self) -> EFS_R {
        EFS_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:29 - EFWM
    #[inline(always)]
    pub fn efwm(&self) -> EFWM_R {
        EFWM_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    ///Bits 2:15 - EFSA
    #[inline(always)]
    #[must_use]
    pub fn efsa(&mut self) -> EFSA_W<2> {
        EFSA_W::new(self)
    }
    ///Bits 16:21 - EFS
    #[inline(always)]
    #[must_use]
    pub fn efs(&mut self) -> EFS_W<16> {
        EFS_W::new(self)
    }
    ///Bits 24:29 - EFWM
    #[inline(always)]
    #[must_use]
    pub fn efwm(&mut self) -> EFWM_W<24> {
        EFWM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Tx event FIFO configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_txefc](index.html) module
pub struct FDCAN_TXEFC_SPEC;
impl crate::RegisterSpec for FDCAN_TXEFC_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_txefc::R](R) reader structure
impl crate::Readable for FDCAN_TXEFC_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fdcan_txefc::W](W) writer structure
impl crate::Writable for FDCAN_TXEFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FDCAN_TXEFC to value 0
impl crate::Resettable for FDCAN_TXEFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
