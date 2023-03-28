///Register `DAC_SWTRGR` writer
pub struct W(crate::W<DAC_SWTRGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_SWTRGR_SPEC>;
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
impl From<crate::W<DAC_SWTRGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_SWTRGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SWTRIG1` writer - SWTRIG1
pub type SWTRIG1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_SWTRGR_SPEC, bool, O>;
///Field `SWTRIG2` writer - SWTRIG2
pub type SWTRIG2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_SWTRGR_SPEC, bool, O>;
impl W {
    ///Bit 0 - SWTRIG1
    #[inline(always)]
    #[must_use]
    pub fn swtrig1(&mut self) -> SWTRIG1_W<0> {
        SWTRIG1_W::new(self)
    }
    ///Bit 1 - SWTRIG2
    #[inline(always)]
    #[must_use]
    pub fn swtrig2(&mut self) -> SWTRIG2_W<1> {
        SWTRIG2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DAC software trigger register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dac_swtrgr](index.html) module
pub struct DAC_SWTRGR_SPEC;
impl crate::RegisterSpec for DAC_SWTRGR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [dac_swtrgr::W](W) writer structure
impl crate::Writable for DAC_SWTRGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DAC_SWTRGR to value 0
impl crate::Resettable for DAC_SWTRGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
