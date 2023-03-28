///Register `TIMEICR` writer
pub struct W(crate::W<TIMEICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMEICR_SPEC>;
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
impl From<crate::W<TIMEICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMEICR_SPEC>) -> Self {
        W(writer)
    }
}
///Compare 1 Interrupt flag Clear
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP1C_AW {
    ///1: Clears associated flag in ISR register
    Clear = 1,
}
impl From<CMP1C_AW> for bool {
    #[inline(always)]
    fn from(variant: CMP1C_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CMP1C` writer - Compare 1 Interrupt flag Clear
pub type CMP1C_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMEICR_SPEC, CMP1C_AW, O>;
impl<'a, const O: u8> CMP1C_W<'a, O> {
    ///Clears associated flag in ISR register
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMP1C_AW::Clear)
    }
}
///Field `CMP2C` writer - Compare 2 Interrupt flag Clear
pub use CMP1C_W as CMP2C_W;
///Field `CMP3C` writer - Compare 3 Interrupt flag Clear
pub use CMP1C_W as CMP3C_W;
///Field `CMP4C` writer - Compare 4 Interrupt flag Clear
pub use CMP1C_W as CMP4C_W;
///Field `REPC` writer - Repetition Interrupt flag Clear
pub use CMP1C_W as REPC_W;
///Field `UPDC` writer - Update Interrupt flag Clear
pub use CMP1C_W as UPDC_W;
///Field `CPT1C` writer - Capture1 Interrupt flag Clear
pub use CMP1C_W as CPT1C_W;
///Field `CPT2C` writer - Capture2 Interrupt flag Clear
pub use CMP1C_W as CPT2C_W;
///Field `SET1xC` writer - Output 1 Set flag Clear
pub use CMP1C_W as SET1X_C_W;
///Field `RSTx1C` writer - Output 1 Reset flag Clear
pub use CMP1C_W as RSTX1C_W;
///Field `SET2xC` writer - Output 2 Set flag Clear
pub use CMP1C_W as SET2X_C_W;
///Field `RSTx2C` writer - Output 2 Reset flag Clear
pub use CMP1C_W as RSTX2C_W;
///Field `RSTC` writer - Reset Interrupt flag Clear
pub use CMP1C_W as RSTC_W;
///Field `DLYPRTC` writer - Delayed Protection Flag Clear
pub use CMP1C_W as DLYPRTC_W;
impl W {
    ///Bit 0 - Compare 1 Interrupt flag Clear
    #[inline(always)]
    #[must_use]
    pub fn cmp1c(&mut self) -> CMP1C_W<0> {
        CMP1C_W::new(self)
    }
    ///Bit 1 - Compare 2 Interrupt flag Clear
    #[inline(always)]
    #[must_use]
    pub fn cmp2c(&mut self) -> CMP2C_W<1> {
        CMP2C_W::new(self)
    }
    ///Bit 2 - Compare 3 Interrupt flag Clear
    #[inline(always)]
    #[must_use]
    pub fn cmp3c(&mut self) -> CMP3C_W<2> {
        CMP3C_W::new(self)
    }
    ///Bit 3 - Compare 4 Interrupt flag Clear
    #[inline(always)]
    #[must_use]
    pub fn cmp4c(&mut self) -> CMP4C_W<3> {
        CMP4C_W::new(self)
    }
    ///Bit 4 - Repetition Interrupt flag Clear
    #[inline(always)]
    #[must_use]
    pub fn repc(&mut self) -> REPC_W<4> {
        REPC_W::new(self)
    }
    ///Bit 6 - Update Interrupt flag Clear
    #[inline(always)]
    #[must_use]
    pub fn updc(&mut self) -> UPDC_W<6> {
        UPDC_W::new(self)
    }
    ///Bit 7 - Capture1 Interrupt flag Clear
    #[inline(always)]
    #[must_use]
    pub fn cpt1c(&mut self) -> CPT1C_W<7> {
        CPT1C_W::new(self)
    }
    ///Bit 8 - Capture2 Interrupt flag Clear
    #[inline(always)]
    #[must_use]
    pub fn cpt2c(&mut self) -> CPT2C_W<8> {
        CPT2C_W::new(self)
    }
    ///Bit 9 - Output 1 Set flag Clear
    #[inline(always)]
    #[must_use]
    pub fn set1x_c(&mut self) -> SET1X_C_W<9> {
        SET1X_C_W::new(self)
    }
    ///Bit 10 - Output 1 Reset flag Clear
    #[inline(always)]
    #[must_use]
    pub fn rstx1c(&mut self) -> RSTX1C_W<10> {
        RSTX1C_W::new(self)
    }
    ///Bit 11 - Output 2 Set flag Clear
    #[inline(always)]
    #[must_use]
    pub fn set2x_c(&mut self) -> SET2X_C_W<11> {
        SET2X_C_W::new(self)
    }
    ///Bit 12 - Output 2 Reset flag Clear
    #[inline(always)]
    #[must_use]
    pub fn rstx2c(&mut self) -> RSTX2C_W<12> {
        RSTX2C_W::new(self)
    }
    ///Bit 13 - Reset Interrupt flag Clear
    #[inline(always)]
    #[must_use]
    pub fn rstc(&mut self) -> RSTC_W<13> {
        RSTC_W::new(self)
    }
    ///Bit 14 - Delayed Protection Flag Clear
    #[inline(always)]
    #[must_use]
    pub fn dlyprtc(&mut self) -> DLYPRTC_W<14> {
        DLYPRTC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timerx Interrupt Clear Register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timeicr](index.html) module
pub struct TIMEICR_SPEC;
impl crate::RegisterSpec for TIMEICR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [timeicr::W](W) writer structure
impl crate::Writable for TIMEICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIMEICR to value 0
impl crate::Resettable for TIMEICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
