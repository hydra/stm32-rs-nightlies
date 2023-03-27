///Register `GLPMCFG` reader
pub struct R(crate::R<GLPMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLPMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLPMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLPMCFG_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GLPMCFG` writer
pub struct W(crate::W<GLPMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLPMCFG_SPEC>;
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
impl From<crate::W<GLPMCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLPMCFG_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LPMEN` reader - LPM support enable
pub type LPMEN_R = crate::BitReader<bool>;
///Field `LPMEN` writer - LPM support enable
pub type LPMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLPMCFG_SPEC, bool, O>;
///Field `LPMACK` reader - LPM token acknowledge enable
pub type LPMACK_R = crate::BitReader<bool>;
///Field `LPMACK` writer - LPM token acknowledge enable
pub type LPMACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLPMCFG_SPEC, bool, O>;
///Field `BESL` reader - Best effort service latency
pub type BESL_R = crate::FieldReader<u8, u8>;
///Field `BESL` writer - Best effort service latency
pub type BESL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GLPMCFG_SPEC, u8, u8, 4, O>;
///Field `REMWAKE` reader - bRemoteWake value
pub type REMWAKE_R = crate::BitReader<bool>;
///Field `REMWAKE` writer - bRemoteWake value
pub type REMWAKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLPMCFG_SPEC, bool, O>;
///Field `L1SSEN` reader - L1 Shallow Sleep enable
pub type L1SSEN_R = crate::BitReader<bool>;
///Field `L1SSEN` writer - L1 Shallow Sleep enable
pub type L1SSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLPMCFG_SPEC, bool, O>;
///Field `BESLTHRS` reader - BESL threshold
pub type BESLTHRS_R = crate::FieldReader<u8, u8>;
///Field `BESLTHRS` writer - BESL threshold
pub type BESLTHRS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GLPMCFG_SPEC, u8, u8, 4, O>;
///Field `L1DSEN` reader - L1 deep sleep enable
pub type L1DSEN_R = crate::BitReader<bool>;
///Field `L1DSEN` writer - L1 deep sleep enable
pub type L1DSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLPMCFG_SPEC, bool, O>;
///Field `LPMRSP` reader - LPM response
pub type LPMRSP_R = crate::FieldReader<u8, u8>;
///Field `LPMRSP` writer - LPM response
pub type LPMRSP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GLPMCFG_SPEC, u8, u8, 2, O>;
///Field `SLPSTS` reader - Port sleep status
pub type SLPSTS_R = crate::BitReader<bool>;
///Field `SLPSTS` writer - Port sleep status
pub type SLPSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLPMCFG_SPEC, bool, O>;
///Field `L1RSMOK` reader - Sleep state resume OK
pub type L1RSMOK_R = crate::BitReader<bool>;
///Field `L1RSMOK` writer - Sleep state resume OK
pub type L1RSMOK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLPMCFG_SPEC, bool, O>;
///Field `LPMCHIDX` reader - LPM Channel Index
pub type LPMCHIDX_R = crate::FieldReader<u8, u8>;
///Field `LPMCHIDX` writer - LPM Channel Index
pub type LPMCHIDX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GLPMCFG_SPEC, u8, u8, 4, O>;
///Field `PMRCNT` reader - LPM retry count
pub type PMRCNT_R = crate::FieldReader<u8, u8>;
///Field `PMRCNT` writer - LPM retry count
pub type PMRCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GLPMCFG_SPEC, u8, u8, 3, O>;
///Field `SNDLPM` reader - Send LPM transaction
pub type SNDLPM_R = crate::BitReader<bool>;
///Field `SNDLPM` writer - Send LPM transaction
pub type SNDLPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLPMCFG_SPEC, bool, O>;
///Field `LPMRCNTSTS` reader - LPM retry count status
pub type LPMRCNTSTS_R = crate::FieldReader<u8, u8>;
///Field `LPMRCNTSTS` writer - LPM retry count status
pub type LPMRCNTSTS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GLPMCFG_SPEC, u8, u8, 3, O>;
///Field `ENBESL` reader - Enable best effort service latency
pub type ENBESL_R = crate::BitReader<bool>;
///Field `ENBESL` writer - Enable best effort service latency
pub type ENBESL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GLPMCFG_SPEC, bool, O>;
impl R {
    ///Bit 0 - LPM support enable
    #[inline(always)]
    pub fn lpmen(&self) -> LPMEN_R {
        LPMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LPM token acknowledge enable
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:5 - Best effort service latency
    #[inline(always)]
    pub fn besl(&self) -> BESL_R {
        BESL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bit 6 - bRemoteWake value
    #[inline(always)]
    pub fn remwake(&self) -> REMWAKE_R {
        REMWAKE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - L1 Shallow Sleep enable
    #[inline(always)]
    pub fn l1ssen(&self) -> L1SSEN_R {
        L1SSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - BESL threshold
    #[inline(always)]
    pub fn beslthrs(&self) -> BESLTHRS_R {
        BESLTHRS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - L1 deep sleep enable
    #[inline(always)]
    pub fn l1dsen(&self) -> L1DSEN_R {
        L1DSEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:14 - LPM response
    #[inline(always)]
    pub fn lpmrsp(&self) -> LPMRSP_R {
        LPMRSP_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - Port sleep status
    #[inline(always)]
    pub fn slpsts(&self) -> SLPSTS_R {
        SLPSTS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Sleep state resume OK
    #[inline(always)]
    pub fn l1rsmok(&self) -> L1RSMOK_R {
        L1RSMOK_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:20 - LPM Channel Index
    #[inline(always)]
    pub fn lpmchidx(&self) -> LPMCHIDX_R {
        LPMCHIDX_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    ///Bits 21:23 - LPM retry count
    #[inline(always)]
    pub fn pmrcnt(&self) -> PMRCNT_R {
        PMRCNT_R::new(((self.bits >> 21) & 7) as u8)
    }
    ///Bit 24 - Send LPM transaction
    #[inline(always)]
    pub fn sndlpm(&self) -> SNDLPM_R {
        SNDLPM_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:27 - LPM retry count status
    #[inline(always)]
    pub fn lpmrcntsts(&self) -> LPMRCNTSTS_R {
        LPMRCNTSTS_R::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bit 28 - Enable best effort service latency
    #[inline(always)]
    pub fn enbesl(&self) -> ENBESL_R {
        ENBESL_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LPM support enable
    #[inline(always)]
    #[must_use]
    pub fn lpmen(&mut self) -> LPMEN_W<0> {
        LPMEN_W::new(self)
    }
    ///Bit 1 - LPM token acknowledge enable
    #[inline(always)]
    #[must_use]
    pub fn lpmack(&mut self) -> LPMACK_W<1> {
        LPMACK_W::new(self)
    }
    ///Bits 2:5 - Best effort service latency
    #[inline(always)]
    #[must_use]
    pub fn besl(&mut self) -> BESL_W<2> {
        BESL_W::new(self)
    }
    ///Bit 6 - bRemoteWake value
    #[inline(always)]
    #[must_use]
    pub fn remwake(&mut self) -> REMWAKE_W<6> {
        REMWAKE_W::new(self)
    }
    ///Bit 7 - L1 Shallow Sleep enable
    #[inline(always)]
    #[must_use]
    pub fn l1ssen(&mut self) -> L1SSEN_W<7> {
        L1SSEN_W::new(self)
    }
    ///Bits 8:11 - BESL threshold
    #[inline(always)]
    #[must_use]
    pub fn beslthrs(&mut self) -> BESLTHRS_W<8> {
        BESLTHRS_W::new(self)
    }
    ///Bit 12 - L1 deep sleep enable
    #[inline(always)]
    #[must_use]
    pub fn l1dsen(&mut self) -> L1DSEN_W<12> {
        L1DSEN_W::new(self)
    }
    ///Bits 13:14 - LPM response
    #[inline(always)]
    #[must_use]
    pub fn lpmrsp(&mut self) -> LPMRSP_W<13> {
        LPMRSP_W::new(self)
    }
    ///Bit 15 - Port sleep status
    #[inline(always)]
    #[must_use]
    pub fn slpsts(&mut self) -> SLPSTS_W<15> {
        SLPSTS_W::new(self)
    }
    ///Bit 16 - Sleep state resume OK
    #[inline(always)]
    #[must_use]
    pub fn l1rsmok(&mut self) -> L1RSMOK_W<16> {
        L1RSMOK_W::new(self)
    }
    ///Bits 17:20 - LPM Channel Index
    #[inline(always)]
    #[must_use]
    pub fn lpmchidx(&mut self) -> LPMCHIDX_W<17> {
        LPMCHIDX_W::new(self)
    }
    ///Bits 21:23 - LPM retry count
    #[inline(always)]
    #[must_use]
    pub fn pmrcnt(&mut self) -> PMRCNT_W<21> {
        PMRCNT_W::new(self)
    }
    ///Bit 24 - Send LPM transaction
    #[inline(always)]
    #[must_use]
    pub fn sndlpm(&mut self) -> SNDLPM_W<24> {
        SNDLPM_W::new(self)
    }
    ///Bits 25:27 - LPM retry count status
    #[inline(always)]
    #[must_use]
    pub fn lpmrcntsts(&mut self) -> LPMRCNTSTS_W<25> {
        LPMRCNTSTS_W::new(self)
    }
    ///Bit 28 - Enable best effort service latency
    #[inline(always)]
    #[must_use]
    pub fn enbesl(&mut self) -> ENBESL_W<28> {
        ENBESL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTG core LPM configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [glpmcfg](index.html) module
pub struct GLPMCFG_SPEC;
impl crate::RegisterSpec for GLPMCFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [glpmcfg::R](R) reader structure
impl crate::Readable for GLPMCFG_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [glpmcfg::W](W) writer structure
impl crate::Writable for GLPMCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GLPMCFG to value 0
impl crate::Resettable for GLPMCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
