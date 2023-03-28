///Register `ICR_output` writer
pub struct W(crate::W<ICR_OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_OUTPUT_SPEC>;
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
impl From<crate::W<ICR_OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CC1CF` writer - Capture/compare 1 clear flag Writing 1 to this bit clears the CC1IF flag in the LPTIM_ISR register.
pub type CC1CF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_OUTPUT_SPEC, bool, O>;
///Field `ARRMCF` writer - Autoreload match clear flag Writing 1 to this bit clears the ARRM flag in the LPTIM_ISR register
pub type ARRMCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_OUTPUT_SPEC, bool, O>;
///Field `EXTTRIGCF` writer - External trigger valid edge clear flag Writing 1 to this bit clears the EXTTRIG flag in the LPTIM_ISR register
pub type EXTTRIGCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_OUTPUT_SPEC, bool, O>;
///Field `CMP1OKCF` writer - Compare register 1 update OK clear flag Writing 1 to this bit clears the CMP1OK flag in the LPTIM_ISR register.
pub type CMP1OKCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_OUTPUT_SPEC, bool, O>;
///Field `ARROKCF` writer - Autoreload register update OK clear flag Writing 1 to this bit clears the ARROK flag in the LPTIM_ISR register
pub type ARROKCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_OUTPUT_SPEC, bool, O>;
///Field `UPCF` writer - Direction change to UP clear flag Writing 1 to this bit clear the UP flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .
pub type UPCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_OUTPUT_SPEC, bool, O>;
///Field `DOWNCF` writer - Direction change to down clear flag Writing 1 to this bit clear the DOWN flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .
pub type DOWNCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_OUTPUT_SPEC, bool, O>;
///Field `UECF` writer - Update event clear flag Writing 1 to this bit clear the UE flag in the LPTIM_ISR register.
pub type UECF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_OUTPUT_SPEC, bool, O>;
///Field `REPOKCF` writer - Repetition register update OK clear flag Writing 1 to this bit clears the REPOK flag in the LPTIM_ISR register.
pub type REPOKCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_OUTPUT_SPEC, bool, O>;
///Field `DIEROKCF` writer - Interrupt enable register update OK clear flag Writing 1 to this bit clears the DIEROK flag in the LPTIM_ISR register.
pub type DIEROKCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_OUTPUT_SPEC, bool, O>;
impl W {
    ///Bit 0 - Capture/compare 1 clear flag Writing 1 to this bit clears the CC1IF flag in the LPTIM_ISR register.
    #[inline(always)]
    #[must_use]
    pub fn cc1cf(&mut self) -> CC1CF_W<0> {
        CC1CF_W::new(self)
    }
    ///Bit 1 - Autoreload match clear flag Writing 1 to this bit clears the ARRM flag in the LPTIM_ISR register
    #[inline(always)]
    #[must_use]
    pub fn arrmcf(&mut self) -> ARRMCF_W<1> {
        ARRMCF_W::new(self)
    }
    ///Bit 2 - External trigger valid edge clear flag Writing 1 to this bit clears the EXTTRIG flag in the LPTIM_ISR register
    #[inline(always)]
    #[must_use]
    pub fn exttrigcf(&mut self) -> EXTTRIGCF_W<2> {
        EXTTRIGCF_W::new(self)
    }
    ///Bit 3 - Compare register 1 update OK clear flag Writing 1 to this bit clears the CMP1OK flag in the LPTIM_ISR register.
    #[inline(always)]
    #[must_use]
    pub fn cmp1okcf(&mut self) -> CMP1OKCF_W<3> {
        CMP1OKCF_W::new(self)
    }
    ///Bit 4 - Autoreload register update OK clear flag Writing 1 to this bit clears the ARROK flag in the LPTIM_ISR register
    #[inline(always)]
    #[must_use]
    pub fn arrokcf(&mut self) -> ARROKCF_W<4> {
        ARROKCF_W::new(self)
    }
    ///Bit 5 - Direction change to UP clear flag Writing 1 to this bit clear the UP flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .
    #[inline(always)]
    #[must_use]
    pub fn upcf(&mut self) -> UPCF_W<5> {
        UPCF_W::new(self)
    }
    ///Bit 6 - Direction change to down clear flag Writing 1 to this bit clear the DOWN flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .
    #[inline(always)]
    #[must_use]
    pub fn downcf(&mut self) -> DOWNCF_W<6> {
        DOWNCF_W::new(self)
    }
    ///Bit 7 - Update event clear flag Writing 1 to this bit clear the UE flag in the LPTIM_ISR register.
    #[inline(always)]
    #[must_use]
    pub fn uecf(&mut self) -> UECF_W<7> {
        UECF_W::new(self)
    }
    ///Bit 8 - Repetition register update OK clear flag Writing 1 to this bit clears the REPOK flag in the LPTIM_ISR register.
    #[inline(always)]
    #[must_use]
    pub fn repokcf(&mut self) -> REPOKCF_W<8> {
        REPOKCF_W::new(self)
    }
    ///Bit 24 - Interrupt enable register update OK clear flag Writing 1 to this bit clears the DIEROK flag in the LPTIM_ISR register.
    #[inline(always)]
    #[must_use]
    pub fn dierokcf(&mut self) -> DIEROKCF_W<24> {
        DIEROKCF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LPTIM interrupt clear register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icr_output](index.html) module
pub struct ICR_OUTPUT_SPEC;
impl crate::RegisterSpec for ICR_OUTPUT_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [icr_output::W](W) writer structure
impl crate::Writable for ICR_OUTPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ICR_output to value 0
impl crate::Resettable for ICR_OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
