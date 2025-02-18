///Register `CALIB` reader
pub struct R(crate::R<CALIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALIB_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CALIB` writer
pub struct W(crate::W<CALIB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALIB_SPEC>;
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
impl From<crate::W<CALIB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALIB_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TENMS` reader - Calibration value
pub type TENMS_R = crate::FieldReader<u32, u32>;
///Field `TENMS` writer - Calibration value
pub type TENMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALIB_SPEC, u32, u32, 24, O>;
///Field `SKEW` reader - SKEW flag: Indicates whether the TENMS value is exact
pub type SKEW_R = crate::BitReader<bool>;
///Field `SKEW` writer - SKEW flag: Indicates whether the TENMS value is exact
pub type SKEW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALIB_SPEC, bool, O>;
///Field `NOREF` reader - NOREF flag. Reads as zero
pub type NOREF_R = crate::BitReader<bool>;
///Field `NOREF` writer - NOREF flag. Reads as zero
pub type NOREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALIB_SPEC, bool, O>;
impl R {
    ///Bits 0:23 - Calibration value
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new(self.bits & 0x00ff_ffff)
    }
    ///Bit 30 - SKEW flag: Indicates whether the TENMS value is exact
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - NOREF flag. Reads as zero
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:23 - Calibration value
    #[inline(always)]
    #[must_use]
    pub fn tenms(&mut self) -> TENMS_W<0> {
        TENMS_W::new(self)
    }
    ///Bit 30 - SKEW flag: Indicates whether the TENMS value is exact
    #[inline(always)]
    #[must_use]
    pub fn skew(&mut self) -> SKEW_W<30> {
        SKEW_W::new(self)
    }
    ///Bit 31 - NOREF flag. Reads as zero
    #[inline(always)]
    #[must_use]
    pub fn noref(&mut self) -> NOREF_W<31> {
        NOREF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SysTick calibration value register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [calib](index.html) module
pub struct CALIB_SPEC;
impl crate::RegisterSpec for CALIB_SPEC {
    type Ux = u32;
}
///`read()` method returns [calib::R](R) reader structure
impl crate::Readable for CALIB_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [calib::W](W) writer structure
impl crate::Writable for CALIB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CALIB to value 0
impl crate::Resettable for CALIB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
