///Register `DAC_AUTOCR` reader
pub struct R(crate::R<DAC_AUTOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_AUTOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_AUTOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_AUTOCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DAC_AUTOCR` writer
pub struct W(crate::W<DAC_AUTOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_AUTOCR_SPEC>;
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
impl From<crate::W<DAC_AUTOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_AUTOCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AUTOMODE` reader - DAC Autonomous mode
pub type AUTOMODE_R = crate::BitReader<bool>;
///Field `AUTOMODE` writer - DAC Autonomous mode
pub type AUTOMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAC_AUTOCR_SPEC, bool, O>;
impl R {
    ///Bit 22 - DAC Autonomous mode
    #[inline(always)]
    pub fn automode(&self) -> AUTOMODE_R {
        AUTOMODE_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    ///Bit 22 - DAC Autonomous mode
    #[inline(always)]
    #[must_use]
    pub fn automode(&mut self) -> AUTOMODE_W<22> {
        AUTOMODE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Autonomous mode control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dac_autocr](index.html) module
pub struct DAC_AUTOCR_SPEC;
impl crate::RegisterSpec for DAC_AUTOCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dac_autocr::R](R) reader structure
impl crate::Readable for DAC_AUTOCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dac_autocr::W](W) writer structure
impl crate::Writable for DAC_AUTOCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DAC_AUTOCR to value 0
impl crate::Resettable for DAC_AUTOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
