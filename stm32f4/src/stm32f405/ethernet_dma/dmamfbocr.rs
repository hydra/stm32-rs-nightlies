///Register `DMAMFBOCR` reader
pub struct R(crate::R<DMAMFBOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAMFBOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAMFBOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAMFBOCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMAMFBOCR` writer
pub struct W(crate::W<DMAMFBOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAMFBOCR_SPEC>;
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
impl From<crate::W<DMAMFBOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAMFBOCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MFC` reader - MFC
pub type MFC_R = crate::FieldReader<u16, u16>;
///Field `MFC` writer - MFC
pub type MFC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMAMFBOCR_SPEC, u16, u16, 16, O>;
///Field `OMFC` reader - OMFC
pub type OMFC_R = crate::BitReader<bool>;
///Field `OMFC` writer - OMFC
pub type OMFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMFBOCR_SPEC, bool, O>;
///Field `MFA` reader - MFA
pub type MFA_R = crate::FieldReader<u16, u16>;
///Field `MFA` writer - MFA
pub type MFA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMAMFBOCR_SPEC, u16, u16, 11, O>;
///Field `OFOC` reader - OFOC
pub type OFOC_R = crate::BitReader<bool>;
///Field `OFOC` writer - OFOC
pub type OFOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMFBOCR_SPEC, bool, O>;
impl R {
    ///Bits 0:15 - MFC
    #[inline(always)]
    pub fn mfc(&self) -> MFC_R {
        MFC_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - OMFC
    #[inline(always)]
    pub fn omfc(&self) -> OMFC_R {
        OMFC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:27 - MFA
    #[inline(always)]
    pub fn mfa(&self) -> MFA_R {
        MFA_R::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    ///Bit 28 - OFOC
    #[inline(always)]
    pub fn ofoc(&self) -> OFOC_R {
        OFOC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    ///Bits 0:15 - MFC
    #[inline(always)]
    #[must_use]
    pub fn mfc(&mut self) -> MFC_W<0> {
        MFC_W::new(self)
    }
    ///Bit 16 - OMFC
    #[inline(always)]
    #[must_use]
    pub fn omfc(&mut self) -> OMFC_W<16> {
        OMFC_W::new(self)
    }
    ///Bits 17:27 - MFA
    #[inline(always)]
    #[must_use]
    pub fn mfa(&mut self) -> MFA_W<17> {
        MFA_W::new(self)
    }
    ///Bit 28 - OFOC
    #[inline(always)]
    #[must_use]
    pub fn ofoc(&mut self) -> OFOC_W<28> {
        OFOC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet DMA missed frame and buffer overflow counter register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmamfbocr](index.html) module
pub struct DMAMFBOCR_SPEC;
impl crate::RegisterSpec for DMAMFBOCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmamfbocr::R](R) reader structure
impl crate::Readable for DMAMFBOCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dmamfbocr::W](W) writer structure
impl crate::Writable for DMAMFBOCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMAMFBOCR to value 0
impl crate::Resettable for DMAMFBOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
