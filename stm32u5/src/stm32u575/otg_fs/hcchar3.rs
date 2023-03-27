///Register `HCCHAR3` reader
pub struct R(crate::R<HCCHAR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCCHAR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCCHAR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCCHAR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `HCCHAR3` writer
pub struct W(crate::W<HCCHAR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCCHAR3_SPEC>;
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
impl From<crate::W<HCCHAR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCCHAR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MPSIZ` reader - MPSIZ
pub type MPSIZ_R = crate::FieldReader<u16, u16>;
///Field `MPSIZ` writer - MPSIZ
pub type MPSIZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCCHAR3_SPEC, u16, u16, 11, O>;
///Field `EPNUM` reader - EPNUM
pub type EPNUM_R = crate::FieldReader<u8, u8>;
///Field `EPNUM` writer - EPNUM
pub type EPNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCCHAR3_SPEC, u8, u8, 4, O>;
///Field `EPDIR` reader - EPDIR
pub type EPDIR_R = crate::BitReader<bool>;
///Field `EPDIR` writer - EPDIR
pub type EPDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCHAR3_SPEC, bool, O>;
///Field `LSDEV` reader - LSDEV
pub type LSDEV_R = crate::BitReader<bool>;
///Field `LSDEV` writer - LSDEV
pub type LSDEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCHAR3_SPEC, bool, O>;
///Field `EPTYP` reader - EPTYP
pub type EPTYP_R = crate::FieldReader<u8, u8>;
///Field `EPTYP` writer - EPTYP
pub type EPTYP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCCHAR3_SPEC, u8, u8, 2, O>;
///Field `MCNT` reader - MCNT
pub type MCNT_R = crate::FieldReader<u8, u8>;
///Field `MCNT` writer - MCNT
pub type MCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCCHAR3_SPEC, u8, u8, 2, O>;
///Field `DAD` reader - DAD
pub type DAD_R = crate::FieldReader<u8, u8>;
///Field `DAD` writer - DAD
pub type DAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCCHAR3_SPEC, u8, u8, 7, O>;
///Field `ODDFRM` reader - ODDFRM
pub type ODDFRM_R = crate::BitReader<bool>;
///Field `ODDFRM` writer - ODDFRM
pub type ODDFRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCHAR3_SPEC, bool, O>;
///Field `CHDIS` reader - CHDIS
pub type CHDIS_R = crate::BitReader<bool>;
///Field `CHDIS` writer - CHDIS
pub type CHDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCHAR3_SPEC, bool, O>;
///Field `CHENA` reader - CHENA
pub type CHENA_R = crate::BitReader<bool>;
///Field `CHENA` writer - CHENA
pub type CHENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCHAR3_SPEC, bool, O>;
impl R {
    ///Bits 0:10 - MPSIZ
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 11:14 - EPNUM
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    ///Bit 15 - EPDIR
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - LSDEV
    #[inline(always)]
    pub fn lsdev(&self) -> LSDEV_R {
        LSDEV_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - EPTYP
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - MCNT
    #[inline(always)]
    pub fn mcnt(&self) -> MCNT_R {
        MCNT_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:28 - DAD
    #[inline(always)]
    pub fn dad(&self) -> DAD_R {
        DAD_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    ///Bit 29 - ODDFRM
    #[inline(always)]
    pub fn oddfrm(&self) -> ODDFRM_R {
        ODDFRM_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - CHDIS
    #[inline(always)]
    pub fn chdis(&self) -> CHDIS_R {
        CHDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - CHENA
    #[inline(always)]
    pub fn chena(&self) -> CHENA_R {
        CHENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:10 - MPSIZ
    #[inline(always)]
    #[must_use]
    pub fn mpsiz(&mut self) -> MPSIZ_W<0> {
        MPSIZ_W::new(self)
    }
    ///Bits 11:14 - EPNUM
    #[inline(always)]
    #[must_use]
    pub fn epnum(&mut self) -> EPNUM_W<11> {
        EPNUM_W::new(self)
    }
    ///Bit 15 - EPDIR
    #[inline(always)]
    #[must_use]
    pub fn epdir(&mut self) -> EPDIR_W<15> {
        EPDIR_W::new(self)
    }
    ///Bit 17 - LSDEV
    #[inline(always)]
    #[must_use]
    pub fn lsdev(&mut self) -> LSDEV_W<17> {
        LSDEV_W::new(self)
    }
    ///Bits 18:19 - EPTYP
    #[inline(always)]
    #[must_use]
    pub fn eptyp(&mut self) -> EPTYP_W<18> {
        EPTYP_W::new(self)
    }
    ///Bits 20:21 - MCNT
    #[inline(always)]
    #[must_use]
    pub fn mcnt(&mut self) -> MCNT_W<20> {
        MCNT_W::new(self)
    }
    ///Bits 22:28 - DAD
    #[inline(always)]
    #[must_use]
    pub fn dad(&mut self) -> DAD_W<22> {
        DAD_W::new(self)
    }
    ///Bit 29 - ODDFRM
    #[inline(always)]
    #[must_use]
    pub fn oddfrm(&mut self) -> ODDFRM_W<29> {
        ODDFRM_W::new(self)
    }
    ///Bit 30 - CHDIS
    #[inline(always)]
    #[must_use]
    pub fn chdis(&mut self) -> CHDIS_W<30> {
        CHDIS_W::new(self)
    }
    ///Bit 31 - CHENA
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
///OTG host channel 3 characteristics register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcchar3](index.html) module
pub struct HCCHAR3_SPEC;
impl crate::RegisterSpec for HCCHAR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [hcchar3::R](R) reader structure
impl crate::Readable for HCCHAR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [hcchar3::W](W) writer structure
impl crate::Writable for HCCHAR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets HCCHAR3 to value 0
impl crate::Resettable for HCCHAR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
