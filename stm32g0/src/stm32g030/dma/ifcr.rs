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
///Clear channel 1 global interrupt flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CGIF1_AW {
    ///1: Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register
    Clear = 1,
}
impl From<CGIF1_AW> for bool {
    #[inline(always)]
    fn from(variant: CGIF1_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CGIF1` writer - Clear channel 1 global interrupt flag
pub type CGIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, CGIF1_AW, O>;
impl<'a, const O: u8> CGIF1_W<'a, O> {
    ///Clears the GIF, TEIF, HTIF, TCIF flags in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CGIF1_AW::Clear)
    }
}
///Clear channel 1 transfer complete flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTCIF1_AW {
    ///1: Clears the TCIF flag in the ISR register
    Clear = 1,
}
impl From<CTCIF1_AW> for bool {
    #[inline(always)]
    fn from(variant: CTCIF1_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CTCIF1` writer - Clear channel 1 transfer complete flag
pub type CTCIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, CTCIF1_AW, O>;
impl<'a, const O: u8> CTCIF1_W<'a, O> {
    ///Clears the TCIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTCIF1_AW::Clear)
    }
}
///Clear channel 1 half transfer flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHTIF1_AW {
    ///1: Clears the HTIF flag in the ISR register
    Clear = 1,
}
impl From<CHTIF1_AW> for bool {
    #[inline(always)]
    fn from(variant: CHTIF1_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CHTIF1` writer - Clear channel 1 half transfer flag
pub type CHTIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, CHTIF1_AW, O>;
impl<'a, const O: u8> CHTIF1_W<'a, O> {
    ///Clears the HTIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CHTIF1_AW::Clear)
    }
}
///Clear channel 1 transfer error flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTEIF1_AW {
    ///1: Clears the TEIF flag in the ISR register
    Clear = 1,
}
impl From<CTEIF1_AW> for bool {
    #[inline(always)]
    fn from(variant: CTEIF1_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CTEIF1` writer - Clear channel 1 transfer error flag
pub type CTEIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFCR_SPEC, CTEIF1_AW, O>;
impl<'a, const O: u8> CTEIF1_W<'a, O> {
    ///Clears the TEIF flag in the ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTEIF1_AW::Clear)
    }
}
///Field `CGIF2` writer - Clear channel 2 global interrupt flag
pub use CGIF1_W as CGIF2_W;
///Field `CGIF3` writer - Clear channel 3 global interrupt flag
pub use CGIF1_W as CGIF3_W;
///Field `CGIF4` writer - Clear channel 4 global interrupt flag
pub use CGIF1_W as CGIF4_W;
///Field `CGIF5` writer - Clear channel 5 global interrupt flag
pub use CGIF1_W as CGIF5_W;
///Field `CHTIF2` writer - Clear channel 2 half transfer flag
pub use CHTIF1_W as CHTIF2_W;
///Field `CHTIF3` writer - Clear channel 3 half transfer flag
pub use CHTIF1_W as CHTIF3_W;
///Field `CHTIF4` writer - Clear channel 4 half transfer flag
pub use CHTIF1_W as CHTIF4_W;
///Field `CHTIF5` writer - Clear channel 5 half transfer flag
pub use CHTIF1_W as CHTIF5_W;
///Field `CTCIF2` writer - Clear channel 2 transfer complete flag
pub use CTCIF1_W as CTCIF2_W;
///Field `CTCIF3` writer - Clear channel 3 transfer complete flag
pub use CTCIF1_W as CTCIF3_W;
///Field `CTCIF4` writer - Clear channel 4 transfer complete flag
pub use CTCIF1_W as CTCIF4_W;
///Field `CTCIF5` writer - Clear channel 5 transfer complete flag
pub use CTCIF1_W as CTCIF5_W;
///Field `CTEIF2` writer - Clear channel 2 transfer error flag
pub use CTEIF1_W as CTEIF2_W;
///Field `CTEIF3` writer - Clear channel 3 transfer error flag
pub use CTEIF1_W as CTEIF3_W;
///Field `CTEIF4` writer - Clear channel 4 transfer error flag
pub use CTEIF1_W as CTEIF4_W;
///Field `CTEIF5` writer - Clear channel 5 transfer error flag
pub use CTEIF1_W as CTEIF5_W;
impl W {
    ///Bit 0 - Clear channel 1 global interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn cgif1(&mut self) -> CGIF1_W<0> {
        CGIF1_W::new(self)
    }
    ///Bit 1 - Clear channel 1 transfer complete flag
    #[inline(always)]
    #[must_use]
    pub fn ctcif1(&mut self) -> CTCIF1_W<1> {
        CTCIF1_W::new(self)
    }
    ///Bit 2 - Clear channel 1 half transfer flag
    #[inline(always)]
    #[must_use]
    pub fn chtif1(&mut self) -> CHTIF1_W<2> {
        CHTIF1_W::new(self)
    }
    ///Bit 3 - Clear channel 1 transfer error flag
    #[inline(always)]
    #[must_use]
    pub fn cteif1(&mut self) -> CTEIF1_W<3> {
        CTEIF1_W::new(self)
    }
    ///Bit 4 - Clear channel 2 global interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn cgif2(&mut self) -> CGIF2_W<4> {
        CGIF2_W::new(self)
    }
    ///Bit 5 - Clear channel 2 transfer complete flag
    #[inline(always)]
    #[must_use]
    pub fn ctcif2(&mut self) -> CTCIF2_W<5> {
        CTCIF2_W::new(self)
    }
    ///Bit 6 - Clear channel 2 half transfer flag
    #[inline(always)]
    #[must_use]
    pub fn chtif2(&mut self) -> CHTIF2_W<6> {
        CHTIF2_W::new(self)
    }
    ///Bit 7 - Clear channel 2 transfer error flag
    #[inline(always)]
    #[must_use]
    pub fn cteif2(&mut self) -> CTEIF2_W<7> {
        CTEIF2_W::new(self)
    }
    ///Bit 8 - Clear channel 3 global interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn cgif3(&mut self) -> CGIF3_W<8> {
        CGIF3_W::new(self)
    }
    ///Bit 9 - Clear channel 3 transfer complete flag
    #[inline(always)]
    #[must_use]
    pub fn ctcif3(&mut self) -> CTCIF3_W<9> {
        CTCIF3_W::new(self)
    }
    ///Bit 10 - Clear channel 3 half transfer flag
    #[inline(always)]
    #[must_use]
    pub fn chtif3(&mut self) -> CHTIF3_W<10> {
        CHTIF3_W::new(self)
    }
    ///Bit 11 - Clear channel 3 transfer error flag
    #[inline(always)]
    #[must_use]
    pub fn cteif3(&mut self) -> CTEIF3_W<11> {
        CTEIF3_W::new(self)
    }
    ///Bit 12 - Clear channel 4 global interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn cgif4(&mut self) -> CGIF4_W<12> {
        CGIF4_W::new(self)
    }
    ///Bit 13 - Clear channel 4 transfer complete flag
    #[inline(always)]
    #[must_use]
    pub fn ctcif4(&mut self) -> CTCIF4_W<13> {
        CTCIF4_W::new(self)
    }
    ///Bit 14 - Clear channel 4 half transfer flag
    #[inline(always)]
    #[must_use]
    pub fn chtif4(&mut self) -> CHTIF4_W<14> {
        CHTIF4_W::new(self)
    }
    ///Bit 15 - Clear channel 4 transfer error flag
    #[inline(always)]
    #[must_use]
    pub fn cteif4(&mut self) -> CTEIF4_W<15> {
        CTEIF4_W::new(self)
    }
    ///Bit 16 - Clear channel 5 global interrupt flag
    #[inline(always)]
    #[must_use]
    pub fn cgif5(&mut self) -> CGIF5_W<16> {
        CGIF5_W::new(self)
    }
    ///Bit 17 - Clear channel 5 transfer complete flag
    #[inline(always)]
    #[must_use]
    pub fn ctcif5(&mut self) -> CTCIF5_W<17> {
        CTCIF5_W::new(self)
    }
    ///Bit 18 - Clear channel 5 half transfer flag
    #[inline(always)]
    #[must_use]
    pub fn chtif5(&mut self) -> CHTIF5_W<18> {
        CHTIF5_W::new(self)
    }
    ///Bit 19 - Clear channel 5 transfer error flag
    #[inline(always)]
    #[must_use]
    pub fn cteif5(&mut self) -> CTEIF5_W<19> {
        CTEIF5_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DMA interrupt flag clear register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ifcr](index.html) module
pub struct IFCR_SPEC;
impl crate::RegisterSpec for IFCR_SPEC {
    type Ux = u32;
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
