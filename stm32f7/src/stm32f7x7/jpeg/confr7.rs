///Register `CONFR7` reader
pub struct R(crate::R<CONFR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFR7_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CONFR7` writer
pub struct W(crate::W<CONFR7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFR7_SPEC>;
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
impl From<crate::W<CONFR7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFR7_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HD` reader - Huffman DC
pub type HD_R = crate::BitReader<bool>;
///Field `HD` writer - Huffman DC
pub type HD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFR7_SPEC, bool, O>;
///Field `HA` reader - Huffman AC
pub type HA_R = crate::BitReader<bool>;
///Field `HA` writer - Huffman AC
pub type HA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFR7_SPEC, bool, O>;
///Field `QT` reader - Quantization Table
pub type QT_R = crate::FieldReader<u8, u8>;
///Field `QT` writer - Quantization Table
pub type QT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFR7_SPEC, u8, u8, 2, O>;
///Field `NB` reader - Number of Block
pub type NB_R = crate::FieldReader<u8, u8>;
///Field `NB` writer - Number of Block
pub type NB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFR7_SPEC, u8, u8, 4, O>;
///Field `VSF` reader - Vertical Sampling Factor
pub type VSF_R = crate::FieldReader<u8, u8>;
///Field `VSF` writer - Vertical Sampling Factor
pub type VSF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFR7_SPEC, u8, u8, 4, O>;
///Field `HSF` reader - Horizontal Sampling Factor
pub type HSF_R = crate::FieldReader<u8, u8>;
///Field `HSF` writer - Horizontal Sampling Factor
pub type HSF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFR7_SPEC, u8, u8, 4, O>;
impl R {
    ///Bit 0 - Huffman DC
    #[inline(always)]
    pub fn hd(&self) -> HD_R {
        HD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Huffman AC
    #[inline(always)]
    pub fn ha(&self) -> HA_R {
        HA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Quantization Table
    #[inline(always)]
    pub fn qt(&self) -> QT_R {
        QT_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - Number of Block
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Vertical Sampling Factor
    #[inline(always)]
    pub fn vsf(&self) -> VSF_R {
        VSF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Horizontal Sampling Factor
    #[inline(always)]
    pub fn hsf(&self) -> HSF_R {
        HSF_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - Huffman DC
    #[inline(always)]
    #[must_use]
    pub fn hd(&mut self) -> HD_W<0> {
        HD_W::new(self)
    }
    ///Bit 1 - Huffman AC
    #[inline(always)]
    #[must_use]
    pub fn ha(&mut self) -> HA_W<1> {
        HA_W::new(self)
    }
    ///Bits 2:3 - Quantization Table
    #[inline(always)]
    #[must_use]
    pub fn qt(&mut self) -> QT_W<2> {
        QT_W::new(self)
    }
    ///Bits 4:7 - Number of Block
    #[inline(always)]
    #[must_use]
    pub fn nb(&mut self) -> NB_W<4> {
        NB_W::new(self)
    }
    ///Bits 8:11 - Vertical Sampling Factor
    #[inline(always)]
    #[must_use]
    pub fn vsf(&mut self) -> VSF_W<8> {
        VSF_W::new(self)
    }
    ///Bits 12:15 - Horizontal Sampling Factor
    #[inline(always)]
    #[must_use]
    pub fn hsf(&mut self) -> HSF_W<12> {
        HSF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///JPEG codec configuration register 7
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [confr7](index.html) module
pub struct CONFR7_SPEC;
impl crate::RegisterSpec for CONFR7_SPEC {
    type Ux = u32;
}
///`read()` method returns [confr7::R](R) reader structure
impl crate::Readable for CONFR7_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [confr7::W](W) writer structure
impl crate::Writable for CONFR7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CONFR7 to value 0
impl crate::Resettable for CONFR7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
