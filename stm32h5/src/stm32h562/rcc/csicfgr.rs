///Register `CSICFGR` reader
pub struct R(crate::R<CSICFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSICFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSICFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSICFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSICFGR` writer
pub struct W(crate::W<CSICFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSICFGR_SPEC>;
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
impl From<crate::W<CSICFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSICFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CSICAL` reader - CSI clock calibration Set by hardware by option byte loading during system reset NRESET. Adjusted by software through trimming bits CSITRIM. This field represents the sum of engineering option byte calibration value and CSITRIM bits value.
pub type CSICAL_R = crate::FieldReader<u8, u8>;
///Field `CSICAL` writer - CSI clock calibration Set by hardware by option byte loading during system reset NRESET. Adjusted by software through trimming bits CSITRIM. This field represents the sum of engineering option byte calibration value and CSITRIM bits value.
pub type CSICAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSICFGR_SPEC, u8, u8, 8, O>;
///Field `CSITRIM` reader - CSI clock trimming Set by software to adjust calibration. CSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_CSI_OPT) in order to form the calibration trimming value. CSICAL = CSITRIM + FLASH_CSI_OPT. Note: The reset value of the field is 0x20.
pub type CSITRIM_R = crate::FieldReader<u8, u8>;
///Field `CSITRIM` writer - CSI clock trimming Set by software to adjust calibration. CSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_CSI_OPT) in order to form the calibration trimming value. CSICAL = CSITRIM + FLASH_CSI_OPT. Note: The reset value of the field is 0x20.
pub type CSITRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSICFGR_SPEC, u8, u8, 6, O>;
impl R {
    ///Bits 0:7 - CSI clock calibration Set by hardware by option byte loading during system reset NRESET. Adjusted by software through trimming bits CSITRIM. This field represents the sum of engineering option byte calibration value and CSITRIM bits value.
    #[inline(always)]
    pub fn csical(&self) -> CSICAL_R {
        CSICAL_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:21 - CSI clock trimming Set by software to adjust calibration. CSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_CSI_OPT) in order to form the calibration trimming value. CSICAL = CSITRIM + FLASH_CSI_OPT. Note: The reset value of the field is 0x20.
    #[inline(always)]
    pub fn csitrim(&self) -> CSITRIM_R {
        CSITRIM_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    ///Bits 0:7 - CSI clock calibration Set by hardware by option byte loading during system reset NRESET. Adjusted by software through trimming bits CSITRIM. This field represents the sum of engineering option byte calibration value and CSITRIM bits value.
    #[inline(always)]
    #[must_use]
    pub fn csical(&mut self) -> CSICAL_W<0> {
        CSICAL_W::new(self)
    }
    ///Bits 16:21 - CSI clock trimming Set by software to adjust calibration. CSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_CSI_OPT) in order to form the calibration trimming value. CSICAL = CSITRIM + FLASH_CSI_OPT. Note: The reset value of the field is 0x20.
    #[inline(always)]
    #[must_use]
    pub fn csitrim(&mut self) -> CSITRIM_W<16> {
        CSITRIM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC CSI calibration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csicfgr](index.html) module
pub struct CSICFGR_SPEC;
impl crate::RegisterSpec for CSICFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [csicfgr::R](R) reader structure
impl crate::Readable for CSICFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csicfgr::W](W) writer structure
impl crate::Writable for CSICFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSICFGR to value 0x0020_0000
impl crate::Resettable for CSICFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0020_0000;
}
