///Register `CONFR1` reader
pub struct R(crate::R<CONFR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CONFR1` writer
pub struct W(crate::W<CONFR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFR1_SPEC>;
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
impl From<crate::W<CONFR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `NF` reader - Number of color components This field defines the number of color components minus 1.
pub type NF_R = crate::FieldReader<u8, u8>;
///Field `NF` writer - Number of color components This field defines the number of color components minus 1.
pub type NF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFR1_SPEC, u8, u8, 2, O>;
///Field `DE` reader - Decoding Enable This bit selects the coding or decoding process
pub type DE_R = crate::BitReader<bool>;
///Field `DE` writer - Decoding Enable This bit selects the coding or decoding process
pub type DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFR1_SPEC, bool, O>;
///Field `COLORSPACE` reader - Color Space This filed defines the number of quantization tables minus 1 to insert in the output stream.
pub type COLORSPACE_R = crate::FieldReader<u8, u8>;
///Field `COLORSPACE` writer - Color Space This filed defines the number of quantization tables minus 1 to insert in the output stream.
pub type COLORSPACE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFR1_SPEC, u8, u8, 2, O>;
///Field `NS` reader - Number of components for Scan This field defines the number of components minus 1 for scan header marker segment.
pub type NS_R = crate::FieldReader<u8, u8>;
///Field `NS` writer - Number of components for Scan This field defines the number of components minus 1 for scan header marker segment.
pub type NS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFR1_SPEC, u8, u8, 2, O>;
///Field `HDR` reader - Header Processing This bit enable the header processing (generation/parsing).
pub type HDR_R = crate::BitReader<bool>;
///Field `HDR` writer - Header Processing This bit enable the header processing (generation/parsing).
pub type HDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFR1_SPEC, bool, O>;
///Field `YSIZE` reader - Y Size This field defines the number of lines in source image.
pub type YSIZE_R = crate::FieldReader<u16, u16>;
///Field `YSIZE` writer - Y Size This field defines the number of lines in source image.
pub type YSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFR1_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:1 - Number of color components This field defines the number of color components minus 1.
    #[inline(always)]
    pub fn nf(&self) -> NF_R {
        NF_R::new((self.bits & 3) as u8)
    }
    ///Bit 3 - Decoding Enable This bit selects the coding or decoding process
    #[inline(always)]
    pub fn de(&self) -> DE_R {
        DE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - Color Space This filed defines the number of quantization tables minus 1 to insert in the output stream.
    #[inline(always)]
    pub fn colorspace(&self) -> COLORSPACE_R {
        COLORSPACE_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Number of components for Scan This field defines the number of components minus 1 for scan header marker segment.
    #[inline(always)]
    pub fn ns(&self) -> NS_R {
        NS_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - Header Processing This bit enable the header processing (generation/parsing).
    #[inline(always)]
    pub fn hdr(&self) -> HDR_R {
        HDR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 16:31 - Y Size This field defines the number of lines in source image.
    #[inline(always)]
    pub fn ysize(&self) -> YSIZE_R {
        YSIZE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:1 - Number of color components This field defines the number of color components minus 1.
    #[inline(always)]
    #[must_use]
    pub fn nf(&mut self) -> NF_W<0> {
        NF_W::new(self)
    }
    ///Bit 3 - Decoding Enable This bit selects the coding or decoding process
    #[inline(always)]
    #[must_use]
    pub fn de(&mut self) -> DE_W<3> {
        DE_W::new(self)
    }
    ///Bits 4:5 - Color Space This filed defines the number of quantization tables minus 1 to insert in the output stream.
    #[inline(always)]
    #[must_use]
    pub fn colorspace(&mut self) -> COLORSPACE_W<4> {
        COLORSPACE_W::new(self)
    }
    ///Bits 6:7 - Number of components for Scan This field defines the number of components minus 1 for scan header marker segment.
    #[inline(always)]
    #[must_use]
    pub fn ns(&mut self) -> NS_W<6> {
        NS_W::new(self)
    }
    ///Bit 8 - Header Processing This bit enable the header processing (generation/parsing).
    #[inline(always)]
    #[must_use]
    pub fn hdr(&mut self) -> HDR_W<8> {
        HDR_W::new(self)
    }
    ///Bits 16:31 - Y Size This field defines the number of lines in source image.
    #[inline(always)]
    #[must_use]
    pub fn ysize(&mut self) -> YSIZE_W<16> {
        YSIZE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///JPEG codec configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [confr1](index.html) module
pub struct CONFR1_SPEC;
impl crate::RegisterSpec for CONFR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [confr1::R](R) reader structure
impl crate::Readable for CONFR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [confr1::W](W) writer structure
impl crate::Writable for CONFR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CONFR1 to value 0
impl crate::Resettable for CONFR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
