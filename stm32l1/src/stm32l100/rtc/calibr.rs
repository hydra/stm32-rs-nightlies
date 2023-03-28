///Register `CALIBR` reader
pub struct R(crate::R<CALIBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALIBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALIBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALIBR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CALIBR` writer
pub struct W(crate::W<CALIBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALIBR_SPEC>;
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
impl From<crate::W<CALIBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALIBR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DC` reader - Digital calibration
pub type DC_R = crate::FieldReader<u8, u8>;
///Field `DC` writer - Digital calibration
pub type DC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALIBR_SPEC, u8, u8, 5, O>;
///Field `DCS` reader - Digital calibration sign
pub type DCS_R = crate::BitReader<bool>;
///Field `DCS` writer - Digital calibration sign
pub type DCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALIBR_SPEC, bool, O>;
impl R {
    ///Bits 0:4 - Digital calibration
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 7 - Digital calibration sign
    #[inline(always)]
    pub fn dcs(&self) -> DCS_R {
        DCS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:4 - Digital calibration
    #[inline(always)]
    #[must_use]
    pub fn dc(&mut self) -> DC_W<0> {
        DC_W::new(self)
    }
    ///Bit 7 - Digital calibration sign
    #[inline(always)]
    #[must_use]
    pub fn dcs(&mut self) -> DCS_W<7> {
        DCS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///calibration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [calibr](index.html) module
pub struct CALIBR_SPEC;
impl crate::RegisterSpec for CALIBR_SPEC {
    type Ux = u32;
}
///`read()` method returns [calibr::R](R) reader structure
impl crate::Readable for CALIBR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [calibr::W](W) writer structure
impl crate::Writable for CALIBR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CALIBR to value 0
impl crate::Resettable for CALIBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
