///Register `IFCR` reader
pub struct R(crate::R<IFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IFCR` writer
pub struct W(crate::W<IFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFCR_SPEC>;
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
impl From<crate::W<IFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CTEIF` reader - Clear Transfer error interrupt flag Programming this bit to 1 clears the TEIF flag in the DMA2D_ISR register
pub type CTEIF_R = crate::BitReader<CTEIF_A>;
///Clear Transfer error interrupt flag Programming this bit to 1 clears the TEIF flag in the DMA2D_ISR register
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTEIF_A {
    ///1: Clear the TEIF flag in the ISR register
    Clear = 1,
}
impl From<CTEIF_A> for bool {
    #[inline(always)]
    fn from(variant: CTEIF_A) -> Self {
        variant as u8 != 0
    }
}
impl CTEIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CTEIF_A> {
        match self.bits {
            true => Some(CTEIF_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CTEIF_A::Clear
    }
}
///Field `CTEIF` writer - Clear Transfer error interrupt flag Programming this bit to 1 clears the TEIF flag in the DMA2D_ISR register
pub type CTEIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, CTEIF_A, O>;
impl<'a, const O: u8> CTEIF_W<'a, O> {
    ///Clear the TEIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF_A::Clear)
    }
}
///Field `CTCIF` reader - Clear transfer complete interrupt flag Programming this bit to 1 clears the TCIF flag in the DMA2D_ISR register
pub type CTCIF_R = crate::BitReader<CTCIF_A>;
///Clear transfer complete interrupt flag Programming this bit to 1 clears the TCIF flag in the DMA2D_ISR register
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTCIF_A {
    ///1: Clear the TCIF flag in the ISR register
    Clear = 1,
}
impl From<CTCIF_A> for bool {
    #[inline(always)]
    fn from(variant: CTCIF_A) -> Self {
        variant as u8 != 0
    }
}
impl CTCIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CTCIF_A> {
        match self.bits {
            true => Some(CTCIF_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CTCIF_A::Clear
    }
}
///Field `CTCIF` writer - Clear transfer complete interrupt flag Programming this bit to 1 clears the TCIF flag in the DMA2D_ISR register
pub type CTCIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, CTCIF_A, O>;
impl<'a, const O: u8> CTCIF_W<'a, O> {
    ///Clear the TCIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF_A::Clear)
    }
}
///Field `CTWIF` reader - Clear transfer watermark interrupt flag Programming this bit to 1 clears the TWIF flag in the DMA2D_ISR register
pub type CTWIF_R = crate::BitReader<CTWIF_A>;
///Clear transfer watermark interrupt flag Programming this bit to 1 clears the TWIF flag in the DMA2D_ISR register
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTWIF_A {
    ///1: Clear the TWIF flag in the ISR register
    Clear = 1,
}
impl From<CTWIF_A> for bool {
    #[inline(always)]
    fn from(variant: CTWIF_A) -> Self {
        variant as u8 != 0
    }
}
impl CTWIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CTWIF_A> {
        match self.bits {
            true => Some(CTWIF_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CTWIF_A::Clear
    }
}
///Field `CTWIF` writer - Clear transfer watermark interrupt flag Programming this bit to 1 clears the TWIF flag in the DMA2D_ISR register
pub type CTWIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, CTWIF_A, O>;
impl<'a, const O: u8> CTWIF_W<'a, O> {
    ///Clear the TWIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTWIF_A::Clear)
    }
}
///Field `CAECIF` reader - Clear CLUT access error interrupt flag Programming this bit to 1 clears the CAEIF flag in the DMA2D_ISR register
pub type CAECIF_R = crate::BitReader<CAECIF_A>;
///Clear CLUT access error interrupt flag Programming this bit to 1 clears the CAEIF flag in the DMA2D_ISR register
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAECIF_A {
    ///1: Clear the CAEIF flag in the ISR register
    Clear = 1,
}
impl From<CAECIF_A> for bool {
    #[inline(always)]
    fn from(variant: CAECIF_A) -> Self {
        variant as u8 != 0
    }
}
impl CAECIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CAECIF_A> {
        match self.bits {
            true => Some(CAECIF_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CAECIF_A::Clear
    }
}
///Field `CAECIF` writer - Clear CLUT access error interrupt flag Programming this bit to 1 clears the CAEIF flag in the DMA2D_ISR register
pub type CAECIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, CAECIF_A, O>;
impl<'a, const O: u8> CAECIF_W<'a, O> {
    ///Clear the CAEIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CAECIF_A::Clear)
    }
}
///Field `CCTCIF` reader - Clear CLUT transfer complete interrupt flag Programming this bit to 1 clears the CTCIF flag in the DMA2D_ISR register
pub type CCTCIF_R = crate::BitReader<CCTCIF_A>;
///Clear CLUT transfer complete interrupt flag Programming this bit to 1 clears the CTCIF flag in the DMA2D_ISR register
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCTCIF_A {
    ///1: Clear the CTCIF flag in the ISR register
    Clear = 1,
}
impl From<CCTCIF_A> for bool {
    #[inline(always)]
    fn from(variant: CCTCIF_A) -> Self {
        variant as u8 != 0
    }
}
impl CCTCIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CCTCIF_A> {
        match self.bits {
            true => Some(CCTCIF_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CCTCIF_A::Clear
    }
}
///Field `CCTCIF` writer - Clear CLUT transfer complete interrupt flag Programming this bit to 1 clears the CTCIF flag in the DMA2D_ISR register
pub type CCTCIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, CCTCIF_A, O>;
impl<'a, const O: u8> CCTCIF_W<'a, O> {
    ///Clear the CTCIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCTCIF_A::Clear)
    }
}
///Field `CCEIF` reader - Clear configuration error interrupt flag Programming this bit to 1 clears the CEIF flag in the DMA2D_ISR register
pub type CCEIF_R = crate::BitReader<CCEIF_A>;
///Clear configuration error interrupt flag Programming this bit to 1 clears the CEIF flag in the DMA2D_ISR register
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCEIF_A {
    ///1: Clear the CEIF flag in the ISR register
    Clear = 1,
}
impl From<CCEIF_A> for bool {
    #[inline(always)]
    fn from(variant: CCEIF_A) -> Self {
        variant as u8 != 0
    }
}
impl CCEIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<CCEIF_A> {
        match self.bits {
            true => Some(CCEIF_A::Clear),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Clear`
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CCEIF_A::Clear
    }
}
///Field `CCEIF` writer - Clear configuration error interrupt flag Programming this bit to 1 clears the CEIF flag in the DMA2D_ISR register
pub type CCEIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, CCEIF_A, O>;
impl<'a, const O: u8> CCEIF_W<'a, O> {
    ///Clear the CEIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCEIF_A::Clear)
    }
}
impl R {
    ///Bit 0 - Clear Transfer error interrupt flag Programming this bit to 1 clears the TEIF flag in the DMA2D_ISR register
    #[inline(always)]
    pub fn cteif(&self) -> CTEIF_R {
        CTEIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clear transfer complete interrupt flag Programming this bit to 1 clears the TCIF flag in the DMA2D_ISR register
    #[inline(always)]
    pub fn ctcif(&self) -> CTCIF_R {
        CTCIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clear transfer watermark interrupt flag Programming this bit to 1 clears the TWIF flag in the DMA2D_ISR register
    #[inline(always)]
    pub fn ctwif(&self) -> CTWIF_R {
        CTWIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Clear CLUT access error interrupt flag Programming this bit to 1 clears the CAEIF flag in the DMA2D_ISR register
    #[inline(always)]
    pub fn caecif(&self) -> CAECIF_R {
        CAECIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Clear CLUT transfer complete interrupt flag Programming this bit to 1 clears the CTCIF flag in the DMA2D_ISR register
    #[inline(always)]
    pub fn cctcif(&self) -> CCTCIF_R {
        CCTCIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Clear configuration error interrupt flag Programming this bit to 1 clears the CEIF flag in the DMA2D_ISR register
    #[inline(always)]
    pub fn cceif(&self) -> CCEIF_R {
        CCEIF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Clear Transfer error interrupt flag Programming this bit to 1 clears the TEIF flag in the DMA2D_ISR register
    #[inline(always)]
    #[must_use]
    pub fn cteif(&mut self) -> CTEIF_W<0> {
        CTEIF_W::new(self)
    }
    ///Bit 1 - Clear transfer complete interrupt flag Programming this bit to 1 clears the TCIF flag in the DMA2D_ISR register
    #[inline(always)]
    #[must_use]
    pub fn ctcif(&mut self) -> CTCIF_W<1> {
        CTCIF_W::new(self)
    }
    ///Bit 2 - Clear transfer watermark interrupt flag Programming this bit to 1 clears the TWIF flag in the DMA2D_ISR register
    #[inline(always)]
    #[must_use]
    pub fn ctwif(&mut self) -> CTWIF_W<2> {
        CTWIF_W::new(self)
    }
    ///Bit 3 - Clear CLUT access error interrupt flag Programming this bit to 1 clears the CAEIF flag in the DMA2D_ISR register
    #[inline(always)]
    #[must_use]
    pub fn caecif(&mut self) -> CAECIF_W<3> {
        CAECIF_W::new(self)
    }
    ///Bit 4 - Clear CLUT transfer complete interrupt flag Programming this bit to 1 clears the CTCIF flag in the DMA2D_ISR register
    #[inline(always)]
    #[must_use]
    pub fn cctcif(&mut self) -> CCTCIF_W<4> {
        CCTCIF_W::new(self)
    }
    ///Bit 5 - Clear configuration error interrupt flag Programming this bit to 1 clears the CEIF flag in the DMA2D_ISR register
    #[inline(always)]
    #[must_use]
    pub fn cceif(&mut self) -> CCEIF_W<5> {
        CCEIF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DMA2D interrupt flag clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ifcr](index.html) module
pub struct IFCR_SPEC;
impl crate::RegisterSpec for IFCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ifcr::R](R) reader structure
impl crate::Readable for IFCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ifcr::W](W) writer structure
impl crate::Writable for IFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IFCR to value 0
impl crate::Resettable for IFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
