///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CAPTURE` reader - Capture enable
pub type CAPTURE_R = crate::BitReader<bool>;
///Field `CAPTURE` writer - Capture enable
pub type CAPTURE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `CM` reader - Capture mode
pub type CM_R = crate::BitReader<bool>;
///Field `CM` writer - Capture mode
pub type CM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `CROP` reader - Crop feature
pub type CROP_R = crate::BitReader<bool>;
///Field `CROP` writer - Crop feature
pub type CROP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `JPEG` reader - JPEG format
pub type JPEG_R = crate::BitReader<bool>;
///Field `JPEG` writer - JPEG format
pub type JPEG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `ESS` reader - Embedded synchronization select
pub type ESS_R = crate::BitReader<bool>;
///Field `ESS` writer - Embedded synchronization select
pub type ESS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `PCKPOL` reader - Pixel clock polarity
pub type PCKPOL_R = crate::BitReader<bool>;
///Field `PCKPOL` writer - Pixel clock polarity
pub type PCKPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `HSPOL` reader - Horizontal synchronization polarity
pub type HSPOL_R = crate::BitReader<bool>;
///Field `HSPOL` writer - Horizontal synchronization polarity
pub type HSPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `VSPOL` reader - Vertical synchronization polarity
pub type VSPOL_R = crate::BitReader<bool>;
///Field `VSPOL` writer - Vertical synchronization polarity
pub type VSPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `FCRC` reader - Frame capture rate control
pub type FCRC_R = crate::FieldReader<u8, u8>;
///Field `FCRC` writer - Frame capture rate control
pub type FCRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
///Field `EDM` reader - Extended data mode
pub type EDM_R = crate::FieldReader<u8, u8>;
///Field `EDM` writer - Extended data mode
pub type EDM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
///Field `ENABLE` reader - DCMI enable
pub type ENABLE_R = crate::BitReader<bool>;
///Field `ENABLE` writer - DCMI enable
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `BSM` reader - Byte Select mode
pub type BSM_R = crate::FieldReader<u8, u8>;
///Field `BSM` writer - Byte Select mode
pub type BSM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
///Field `OEBS` reader - Odd/Even Byte Select (Byte Select Start)
pub type OEBS_R = crate::BitReader<bool>;
///Field `OEBS` writer - Odd/Even Byte Select (Byte Select Start)
pub type OEBS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `LSM` reader - Line Select mode
pub type LSM_R = crate::BitReader<bool>;
///Field `LSM` writer - Line Select mode
pub type LSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `OELS` reader - Odd/Even Line Select (Line Select Start)
pub type OELS_R = crate::BitReader<bool>;
///Field `OELS` writer - Odd/Even Line Select (Line Select Start)
pub type OELS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Capture enable
    #[inline(always)]
    pub fn capture(&self) -> CAPTURE_R {
        CAPTURE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture mode
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Crop feature
    #[inline(always)]
    pub fn crop(&self) -> CROP_R {
        CROP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - JPEG format
    #[inline(always)]
    pub fn jpeg(&self) -> JPEG_R {
        JPEG_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Embedded synchronization select
    #[inline(always)]
    pub fn ess(&self) -> ESS_R {
        ESS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Pixel clock polarity
    #[inline(always)]
    pub fn pckpol(&self) -> PCKPOL_R {
        PCKPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Horizontal synchronization polarity
    #[inline(always)]
    pub fn hspol(&self) -> HSPOL_R {
        HSPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Vertical synchronization polarity
    #[inline(always)]
    pub fn vspol(&self) -> VSPOL_R {
        VSPOL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Frame capture rate control
    #[inline(always)]
    pub fn fcrc(&self) -> FCRC_R {
        FCRC_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Extended data mode
    #[inline(always)]
    pub fn edm(&self) -> EDM_R {
        EDM_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 14 - DCMI enable
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:17 - Byte Select mode
    #[inline(always)]
    pub fn bsm(&self) -> BSM_R {
        BSM_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - Odd/Even Byte Select (Byte Select Start)
    #[inline(always)]
    pub fn oebs(&self) -> OEBS_R {
        OEBS_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Line Select mode
    #[inline(always)]
    pub fn lsm(&self) -> LSM_R {
        LSM_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Odd/Even Line Select (Line Select Start)
    #[inline(always)]
    pub fn oels(&self) -> OELS_R {
        OELS_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Capture enable
    #[inline(always)]
    #[must_use]
    pub fn capture(&mut self) -> CAPTURE_W<0> {
        CAPTURE_W::new(self)
    }
    ///Bit 1 - Capture mode
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<1> {
        CM_W::new(self)
    }
    ///Bit 2 - Crop feature
    #[inline(always)]
    #[must_use]
    pub fn crop(&mut self) -> CROP_W<2> {
        CROP_W::new(self)
    }
    ///Bit 3 - JPEG format
    #[inline(always)]
    #[must_use]
    pub fn jpeg(&mut self) -> JPEG_W<3> {
        JPEG_W::new(self)
    }
    ///Bit 4 - Embedded synchronization select
    #[inline(always)]
    #[must_use]
    pub fn ess(&mut self) -> ESS_W<4> {
        ESS_W::new(self)
    }
    ///Bit 5 - Pixel clock polarity
    #[inline(always)]
    #[must_use]
    pub fn pckpol(&mut self) -> PCKPOL_W<5> {
        PCKPOL_W::new(self)
    }
    ///Bit 6 - Horizontal synchronization polarity
    #[inline(always)]
    #[must_use]
    pub fn hspol(&mut self) -> HSPOL_W<6> {
        HSPOL_W::new(self)
    }
    ///Bit 7 - Vertical synchronization polarity
    #[inline(always)]
    #[must_use]
    pub fn vspol(&mut self) -> VSPOL_W<7> {
        VSPOL_W::new(self)
    }
    ///Bits 8:9 - Frame capture rate control
    #[inline(always)]
    #[must_use]
    pub fn fcrc(&mut self) -> FCRC_W<8> {
        FCRC_W::new(self)
    }
    ///Bits 10:11 - Extended data mode
    #[inline(always)]
    #[must_use]
    pub fn edm(&mut self) -> EDM_W<10> {
        EDM_W::new(self)
    }
    ///Bit 14 - DCMI enable
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<14> {
        ENABLE_W::new(self)
    }
    ///Bits 16:17 - Byte Select mode
    #[inline(always)]
    #[must_use]
    pub fn bsm(&mut self) -> BSM_W<16> {
        BSM_W::new(self)
    }
    ///Bit 18 - Odd/Even Byte Select (Byte Select Start)
    #[inline(always)]
    #[must_use]
    pub fn oebs(&mut self) -> OEBS_W<18> {
        OEBS_W::new(self)
    }
    ///Bit 19 - Line Select mode
    #[inline(always)]
    #[must_use]
    pub fn lsm(&mut self) -> LSM_W<19> {
        LSM_W::new(self)
    }
    ///Bit 20 - Odd/Even Line Select (Line Select Start)
    #[inline(always)]
    #[must_use]
    pub fn oels(&mut self) -> OELS_W<20> {
        OELS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
