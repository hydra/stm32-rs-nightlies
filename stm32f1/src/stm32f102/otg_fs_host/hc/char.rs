///Register `CHAR` reader
pub struct R(crate::R<CHAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHAR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CHAR` writer
pub struct W(crate::W<CHAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHAR_SPEC>;
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
impl From<crate::W<CHAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHAR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MPSIZ` reader - Maximum packet size
pub type MPSIZ_R = crate::FieldReader<u16, u16>;
///Field `MPSIZ` writer - Maximum packet size
pub type MPSIZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHAR_SPEC, u16, u16, 11, O>;
///Field `EPNUM` reader - Endpoint number
pub type EPNUM_R = crate::FieldReader<u8, u8>;
///Field `EPNUM` writer - Endpoint number
pub type EPNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHAR_SPEC, u8, u8, 4, O>;
///Field `EPDIR` reader - Endpoint direction
pub type EPDIR_R = crate::BitReader<bool>;
///Field `EPDIR` writer - Endpoint direction
pub type EPDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHAR_SPEC, bool, O>;
///Field `LSDEV` reader - Low-speed device
pub type LSDEV_R = crate::BitReader<bool>;
///Field `LSDEV` writer - Low-speed device
pub type LSDEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHAR_SPEC, bool, O>;
///Field `EPTYP` reader - Endpoint type
pub type EPTYP_R = crate::FieldReader<u8, u8>;
///Field `EPTYP` writer - Endpoint type
pub type EPTYP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHAR_SPEC, u8, u8, 2, O>;
///Field `MCNT` reader - Multicount
pub type MCNT_R = crate::FieldReader<u8, u8>;
///Field `MCNT` writer - Multicount
pub type MCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHAR_SPEC, u8, u8, 2, O>;
///Field `DAD` reader - Device address
pub type DAD_R = crate::FieldReader<u8, u8>;
///Field `DAD` writer - Device address
pub type DAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHAR_SPEC, u8, u8, 7, O>;
///Field `ODDFRM` reader - Odd frame
pub type ODDFRM_R = crate::BitReader<bool>;
///Field `ODDFRM` writer - Odd frame
pub type ODDFRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHAR_SPEC, bool, O>;
///Field `CHDIS` reader - Channel disable
pub type CHDIS_R = crate::BitReader<bool>;
///Field `CHDIS` writer - Channel disable
pub type CHDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHAR_SPEC, bool, O>;
///Field `CHENA` reader - Channel enable
pub type CHENA_R = crate::BitReader<bool>;
///Field `CHENA` writer - Channel enable
pub type CHENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHAR_SPEC, bool, O>;
impl R {
    ///Bits 0:10 - Maximum packet size
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 11:14 - Endpoint number
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    ///Bit 15 - Endpoint direction
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - Low-speed device
    #[inline(always)]
    pub fn lsdev(&self) -> LSDEV_R {
        LSDEV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - Endpoint type
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Multicount
    #[inline(always)]
    pub fn mcnt(&self) -> MCNT_R {
        MCNT_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:28 - Device address
    #[inline(always)]
    pub fn dad(&self) -> DAD_R {
        DAD_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    ///Bit 29 - Odd frame
    #[inline(always)]
    pub fn oddfrm(&self) -> ODDFRM_R {
        ODDFRM_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Channel disable
    #[inline(always)]
    pub fn chdis(&self) -> CHDIS_R {
        CHDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Channel enable
    #[inline(always)]
    pub fn chena(&self) -> CHENA_R {
        CHENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:10 - Maximum packet size
    #[inline(always)]
    #[must_use]
    pub fn mpsiz(&mut self) -> MPSIZ_W<0> {
        MPSIZ_W::new(self)
    }
    ///Bits 11:14 - Endpoint number
    #[inline(always)]
    #[must_use]
    pub fn epnum(&mut self) -> EPNUM_W<11> {
        EPNUM_W::new(self)
    }
    ///Bit 15 - Endpoint direction
    #[inline(always)]
    #[must_use]
    pub fn epdir(&mut self) -> EPDIR_W<15> {
        EPDIR_W::new(self)
    }
    ///Bit 17 - Low-speed device
    #[inline(always)]
    #[must_use]
    pub fn lsdev(&mut self) -> LSDEV_W<17> {
        LSDEV_W::new(self)
    }
    ///Bits 18:19 - Endpoint type
    #[inline(always)]
    #[must_use]
    pub fn eptyp(&mut self) -> EPTYP_W<18> {
        EPTYP_W::new(self)
    }
    ///Bits 20:21 - Multicount
    #[inline(always)]
    #[must_use]
    pub fn mcnt(&mut self) -> MCNT_W<20> {
        MCNT_W::new(self)
    }
    ///Bits 22:28 - Device address
    #[inline(always)]
    #[must_use]
    pub fn dad(&mut self) -> DAD_W<22> {
        DAD_W::new(self)
    }
    ///Bit 29 - Odd frame
    #[inline(always)]
    #[must_use]
    pub fn oddfrm(&mut self) -> ODDFRM_W<29> {
        ODDFRM_W::new(self)
    }
    ///Bit 30 - Channel disable
    #[inline(always)]
    #[must_use]
    pub fn chdis(&mut self) -> CHDIS_W<30> {
        CHDIS_W::new(self)
    }
    ///Bit 31 - Channel enable
    #[inline(always)]
    #[must_use]
    pub fn chena(&mut self) -> CHENA_W<31> {
        CHENA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [char](index.html) module
pub struct CHAR_SPEC;
impl crate::RegisterSpec for CHAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [char::R](R) reader structure
impl crate::Readable for CHAR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [char::W](W) writer structure
impl crate::Writable for CHAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CHAR to value 0
impl crate::Resettable for CHAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
