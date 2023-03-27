///Register `SCR` writer
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
///CTAMP1F
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTAMP1FW_AW {
    ///1: Clear tamper flag
    Clear = 1,
}
impl From<CTAMP1FW_AW> for bool {
    #[inline(always)]
    fn from(variant: CTAMP1FW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CTAMP1F` writer - CTAMP1F
pub type CTAMP1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, CTAMP1FW_AW, O>;
impl<'a, const O: u8> CTAMP1F_W<'a, O> {
    ///Clear tamper flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTAMP1FW_AW::Clear)
    }
}
///Field `CTAMP2F` writer - CTAMP2F
pub use CTAMP1F_W as CTAMP2F_W;
///Field `CTAMP3F` writer - CTAMP3F
pub use CTAMP1F_W as CTAMP3F_W;
///CITAMP3F
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CITAMP3FW_AW {
    ///1: Clear tamper flag
    Clear = 1,
}
impl From<CITAMP3FW_AW> for bool {
    #[inline(always)]
    fn from(variant: CITAMP3FW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CITAMP3F` writer - CITAMP3F
pub type CITAMP3F_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, CITAMP3FW_AW, O>;
impl<'a, const O: u8> CITAMP3F_W<'a, O> {
    ///Clear tamper flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CITAMP3FW_AW::Clear)
    }
}
///Field `CITAMP5F` writer - CITAMP5F
pub use CITAMP3F_W as CITAMP5F_W;
///Field `CITAMP6F` writer - CITAMP6F
pub use CITAMP3F_W as CITAMP6F_W;
///Field `CITAMP8F` writer - CITAMP8F
pub use CITAMP3F_W as CITAMP8F_W;
impl W {
    ///Bit 0 - CTAMP1F
    #[inline(always)]
    #[must_use]
    pub fn ctamp1f(&mut self) -> CTAMP1F_W<0> {
        CTAMP1F_W::new(self)
    }
    ///Bit 1 - CTAMP2F
    #[inline(always)]
    #[must_use]
    pub fn ctamp2f(&mut self) -> CTAMP2F_W<1> {
        CTAMP2F_W::new(self)
    }
    ///Bit 2 - CTAMP3F
    #[inline(always)]
    #[must_use]
    pub fn ctamp3f(&mut self) -> CTAMP3F_W<2> {
        CTAMP3F_W::new(self)
    }
    ///Bit 18 - CITAMP3F
    #[inline(always)]
    #[must_use]
    pub fn citamp3f(&mut self) -> CITAMP3F_W<18> {
        CITAMP3F_W::new(self)
    }
    ///Bit 20 - CITAMP5F
    #[inline(always)]
    #[must_use]
    pub fn citamp5f(&mut self) -> CITAMP5F_W<20> {
        CITAMP5F_W::new(self)
    }
    ///Bit 21 - CITAMP6F
    #[inline(always)]
    #[must_use]
    pub fn citamp6f(&mut self) -> CITAMP6F_W<21> {
        CITAMP6F_W::new(self)
    }
    ///Bit 23 - CITAMP8F
    #[inline(always)]
    #[must_use]
    pub fn citamp8f(&mut self) -> CITAMP8F_W<23> {
        CITAMP8F_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TAMP status clear register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [scr](index.html) module
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [scr::W](W) writer structure
impl crate::Writable for SCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SCR to value 0
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
