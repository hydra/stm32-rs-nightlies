///Register `L1CLUTWR` writer
pub struct W(crate::W<L1CLUTWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1CLUTWR_SPEC>;
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
impl From<crate::W<L1CLUTWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1CLUTWR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BLUE` writer - Blue value
pub type BLUE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, L1CLUTWR_SPEC, u8, u8, 8, O>;
///Field `GREEN` writer - Green value
pub type GREEN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, L1CLUTWR_SPEC, u8, u8, 8, O>;
///Field `RED` writer - Red value
pub type RED_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, L1CLUTWR_SPEC, u8, u8, 8, O>;
///Field `CLUTADD` writer - CLUT Address
pub type CLUTADD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, L1CLUTWR_SPEC, u8, u8, 8, O>;
impl W {
    ///Bits 0:7 - Blue value
    #[inline(always)]
    #[must_use]
    pub fn blue(&mut self) -> BLUE_W<0> {
        BLUE_W::new(self)
    }
    ///Bits 8:15 - Green value
    #[inline(always)]
    #[must_use]
    pub fn green(&mut self) -> GREEN_W<8> {
        GREEN_W::new(self)
    }
    ///Bits 16:23 - Red value
    #[inline(always)]
    #[must_use]
    pub fn red(&mut self) -> RED_W<16> {
        RED_W::new(self)
    }
    ///Bits 24:31 - CLUT Address
    #[inline(always)]
    #[must_use]
    pub fn clutadd(&mut self) -> CLUTADD_W<24> {
        CLUTADD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LTDC Layerx CLUT Write Register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [l1clutwr](index.html) module
pub struct L1CLUTWR_SPEC;
impl crate::RegisterSpec for L1CLUTWR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [l1clutwr::W](W) writer structure
impl crate::Writable for L1CLUTWR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets L1CLUTWR to value 0
impl crate::Resettable for L1CLUTWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
