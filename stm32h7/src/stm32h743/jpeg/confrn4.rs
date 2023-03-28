///Register `CONFRN4` reader
pub struct R(crate::R<CONFRN4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFRN4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFRN4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFRN4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CONFRN4` writer
pub struct W(crate::W<CONFRN4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFRN4_SPEC>;
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
impl From<crate::W<CONFRN4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFRN4_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HD` reader - Huffman DC Selects the Huffman table for encoding the DC coefficients.
pub type HD_R = crate::BitReader<bool>;
///Field `HD` writer - Huffman DC Selects the Huffman table for encoding the DC coefficients.
pub type HD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFRN4_SPEC, bool, O>;
///Field `HA` reader - Huffman AC Selects the Huffman table for encoding the AC coefficients.
pub type HA_R = crate::BitReader<bool>;
///Field `HA` writer - Huffman AC Selects the Huffman table for encoding the AC coefficients.
pub type HA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFRN4_SPEC, bool, O>;
///Field `QT` reader - Quantization Table Selects quantization table associated with a color component.
pub type QT_R = crate::FieldReader<u8, u8>;
///Field `QT` writer - Quantization Table Selects quantization table associated with a color component.
pub type QT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFRN4_SPEC, u8, u8, 2, O>;
///Field `NB` reader - Number of Block Number of data units minus 1 that belong to a particular color in the MCU.
pub type NB_R = crate::FieldReader<u8, u8>;
///Field `NB` writer - Number of Block Number of data units minus 1 that belong to a particular color in the MCU.
pub type NB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFRN4_SPEC, u8, u8, 4, O>;
///Field `VSF` reader - Vertical Sampling Factor Vertical sampling factor for component i.
pub type VSF_R = crate::FieldReader<u8, u8>;
///Field `VSF` writer - Vertical Sampling Factor Vertical sampling factor for component i.
pub type VSF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFRN4_SPEC, u8, u8, 4, O>;
///Field `HSF` reader - Horizontal Sampling Factor Horizontal sampling factor for component i.
pub type HSF_R = crate::FieldReader<u8, u8>;
///Field `HSF` writer - Horizontal Sampling Factor Horizontal sampling factor for component i.
pub type HSF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFRN4_SPEC, u8, u8, 4, O>;
impl R {
    ///Bit 0 - Huffman DC Selects the Huffman table for encoding the DC coefficients.
    #[inline(always)]
    pub fn hd(&self) -> HD_R {
        HD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Huffman AC Selects the Huffman table for encoding the AC coefficients.
    #[inline(always)]
    pub fn ha(&self) -> HA_R {
        HA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Quantization Table Selects quantization table associated with a color component.
    #[inline(always)]
    pub fn qt(&self) -> QT_R {
        QT_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - Number of Block Number of data units minus 1 that belong to a particular color in the MCU.
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Vertical Sampling Factor Vertical sampling factor for component i.
    #[inline(always)]
    pub fn vsf(&self) -> VSF_R {
        VSF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Horizontal Sampling Factor Horizontal sampling factor for component i.
    #[inline(always)]
    pub fn hsf(&self) -> HSF_R {
        HSF_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    ///Bit 0 - Huffman DC Selects the Huffman table for encoding the DC coefficients.
    #[inline(always)]
    #[must_use]
    pub fn hd(&mut self) -> HD_W<0> {
        HD_W::new(self)
    }
    ///Bit 1 - Huffman AC Selects the Huffman table for encoding the AC coefficients.
    #[inline(always)]
    #[must_use]
    pub fn ha(&mut self) -> HA_W<1> {
        HA_W::new(self)
    }
    ///Bits 2:3 - Quantization Table Selects quantization table associated with a color component.
    #[inline(always)]
    #[must_use]
    pub fn qt(&mut self) -> QT_W<2> {
        QT_W::new(self)
    }
    ///Bits 4:7 - Number of Block Number of data units minus 1 that belong to a particular color in the MCU.
    #[inline(always)]
    #[must_use]
    pub fn nb(&mut self) -> NB_W<4> {
        NB_W::new(self)
    }
    ///Bits 8:11 - Vertical Sampling Factor Vertical sampling factor for component i.
    #[inline(always)]
    #[must_use]
    pub fn vsf(&mut self) -> VSF_W<8> {
        VSF_W::new(self)
    }
    ///Bits 12:15 - Horizontal Sampling Factor Horizontal sampling factor for component i.
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
///JPEG codec configuration register 4-7
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [confrn4](index.html) module
pub struct CONFRN4_SPEC;
impl crate::RegisterSpec for CONFRN4_SPEC {
    type Ux = u32;
}
///`read()` method returns [confrn4::R](R) reader structure
impl crate::Readable for CONFRN4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [confrn4::W](W) writer structure
impl crate::Writable for CONFRN4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CONFRN4 to value 0
impl crate::Resettable for CONFRN4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
