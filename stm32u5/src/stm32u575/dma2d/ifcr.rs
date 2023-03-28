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
///Field `CTEIF` reader - Clear Transfer error interrupt flag
pub type CTEIF_R = crate::BitReader<bool>;
///Field `CTEIF` writer - Clear Transfer error interrupt flag
pub type CTEIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
///Field `CTCIF` reader - Clear transfer complete interrupt flag
pub type CTCIF_R = crate::BitReader<bool>;
///Field `CTCIF` writer - Clear transfer complete interrupt flag
pub type CTCIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
///Field `CTWIF` reader - Clear transfer watermark interrupt flag
pub type CTWIF_R = crate::BitReader<bool>;
///Field `CTWIF` writer - Clear transfer watermark interrupt flag
pub type CTWIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
///Field `CAECIF` reader - Clear CLUT access error interrupt flag
pub type CAECIF_R = crate::BitReader<bool>;
///Field `CAECIF` writer - Clear CLUT access error interrupt flag
pub type CAECIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
///Field `CCTCIF` reader - Clear CLUT transfer complete interrupt flag
pub type CCTCIF_R = crate::BitReader<bool>;
///Field `CCTCIF` writer - Clear CLUT transfer complete interrupt flag
pub type CCTCIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
///Field `CCEIF` reader - Clear configuration error interrupt flag
pub type CCEIF_R = crate::BitReader<bool>;
///Field `CCEIF` writer - Clear configuration error interrupt flag
pub type CCEIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Clear Transfer error interrupt flag
    #[inline(always)]
    pub fn cteif(&self) -> CTEIF_R {
        CTEIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clear transfer complete interrupt flag
    #[inline(always)]
    pub fn ctcif(&self) -> CTCIF_R {
        CTCIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clear transfer watermark interrupt flag
    #[inline(always)]
    pub fn ctwif(&self) -> CTWIF_R {
        CTWIF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Clear CLUT access error interrupt flag
    #[inline(always)]
    pub fn caecif(&self) -> CAECIF_R {
        CAECIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Clear CLUT transfer complete interrupt flag
    #[inline(always)]
    pub fn cctcif(&self) -> CCTCIF_R {
        CCTCIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Clear configuration error interrupt flag
    #[inline(always)]
    pub fn cceif(&self) -> CCEIF_R {
        CCEIF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Clear Transfer error interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn cteif(&mut self) -> CTEIF_W<0> {
        CTEIF_W::new(self)
    }
    ///Bit 1 - Clear transfer complete interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn ctcif(&mut self) -> CTCIF_W<1> {
        CTCIF_W::new(self)
    }
    ///Bit 2 - Clear transfer watermark interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn ctwif(&mut self) -> CTWIF_W<2> {
        CTWIF_W::new(self)
    }
    ///Bit 3 - Clear CLUT access error interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn caecif(&mut self) -> CAECIF_W<3> {
        CAECIF_W::new(self)
    }
    ///Bit 4 - Clear CLUT transfer complete interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn cctcif(&mut self) -> CCTCIF_W<4> {
        CCTCIF_W::new(self)
    }
    ///Bit 5 - Clear configuration error interrupt flag
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
///interrupt flag clear register
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
