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
///Clear wakeup flag 1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWUF1_AW {
    ///1: Setting this bit clears the WUFx flag in the PWR_SR1 register
    Clear = 1,
}
impl From<CWUF1_AW> for bool {
    #[inline(always)]
    fn from(variant: CWUF1_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CWUF1` writer - Clear wakeup flag 1
pub type CWUF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, CWUF1_AW, O>;
impl<'a, const O: u8> CWUF1_W<'a, O> {
    ///Setting this bit clears the WUFx flag in the PWR_SR1 register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CWUF1_AW::Clear)
    }
}
///Field `CWUF2` writer - Clear wakeup flag 2
pub use CWUF1_W as CWUF2_W;
///Field `CWUF3` writer - Clear wakeup flag 3
pub use CWUF1_W as CWUF3_W;
///Field `CWUF4` writer - Clear wakeup flag 4
pub use CWUF1_W as CWUF4_W;
///Field `CWUF5` writer - Clear wakeup flag 5
pub use CWUF1_W as CWUF5_W;
///Clear standby flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CSBF_AW {
    ///1: Setting this bit clears the SBF flag in the PWR_SR1 register
    Clear = 1,
}
impl From<CSBF_AW> for bool {
    #[inline(always)]
    fn from(variant: CSBF_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CSBF` writer - Clear standby flag
pub type CSBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, CSBF_AW, O>;
impl<'a, const O: u8> CSBF_W<'a, O> {
    ///Setting this bit clears the SBF flag in the PWR_SR1 register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CSBF_AW::Clear)
    }
}
impl W {
    ///Bit 0 - Clear wakeup flag 1
    #[inline(always)]
    #[must_use]
    pub fn cwuf1(&mut self) -> CWUF1_W<0> {
        CWUF1_W::new(self)
    }
    ///Bit 1 - Clear wakeup flag 2
    #[inline(always)]
    #[must_use]
    pub fn cwuf2(&mut self) -> CWUF2_W<1> {
        CWUF2_W::new(self)
    }
    ///Bit 2 - Clear wakeup flag 3
    #[inline(always)]
    #[must_use]
    pub fn cwuf3(&mut self) -> CWUF3_W<2> {
        CWUF3_W::new(self)
    }
    ///Bit 3 - Clear wakeup flag 4
    #[inline(always)]
    #[must_use]
    pub fn cwuf4(&mut self) -> CWUF4_W<3> {
        CWUF4_W::new(self)
    }
    ///Bit 4 - Clear wakeup flag 5
    #[inline(always)]
    #[must_use]
    pub fn cwuf5(&mut self) -> CWUF5_W<4> {
        CWUF5_W::new(self)
    }
    ///Bit 8 - Clear standby flag
    #[inline(always)]
    #[must_use]
    pub fn csbf(&mut self) -> CSBF_W<8> {
        CSBF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Power status clear register
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
