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
///Field `WUF1` writer - Clear wakeup flag 1
pub type WUF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
///Field `WUF2` writer - Clear wakeup flag 2
pub type WUF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
///Field `WUF3` writer - Clear wakeup flag 3
pub type WUF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
///Field `WUF4` writer - Clear wakeup flag 4
pub type WUF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
///Field `WUF5` writer - Clear wakeup flag 5
pub type WUF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
///Field `SBF` writer - Clear standby flag
pub type SBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
impl W {
    ///Bit 0 - Clear wakeup flag 1
    #[inline(always)]
    #[must_use]
    pub fn wuf1(&mut self) -> WUF1_W<0> {
        WUF1_W::new(self)
    }
    ///Bit 1 - Clear wakeup flag 2
    #[inline(always)]
    #[must_use]
    pub fn wuf2(&mut self) -> WUF2_W<1> {
        WUF2_W::new(self)
    }
    ///Bit 2 - Clear wakeup flag 3
    #[inline(always)]
    #[must_use]
    pub fn wuf3(&mut self) -> WUF3_W<2> {
        WUF3_W::new(self)
    }
    ///Bit 3 - Clear wakeup flag 4
    #[inline(always)]
    #[must_use]
    pub fn wuf4(&mut self) -> WUF4_W<3> {
        WUF4_W::new(self)
    }
    ///Bit 4 - Clear wakeup flag 5
    #[inline(always)]
    #[must_use]
    pub fn wuf5(&mut self) -> WUF5_W<4> {
        WUF5_W::new(self)
    }
    ///Bit 8 - Clear standby flag
    #[inline(always)]
    #[must_use]
    pub fn sbf(&mut self) -> SBF_W<8> {
        SBF_W::new(self)
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
