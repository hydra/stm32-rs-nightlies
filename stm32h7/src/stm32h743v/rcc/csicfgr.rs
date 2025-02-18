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
///Field `CSICAL` reader - CSI clock calibration
pub type CSICAL_R = crate::FieldReader<u16, u16>;
///Field `CSICAL` writer - CSI clock calibration
pub type CSICAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSICFGR_SPEC, u16, u16, 9, O>;
///Field `CSITRIM` reader - CSI clock trimming
pub type CSITRIM_R = crate::FieldReader<u8, u8>;
///Field `CSITRIM` writer - CSI clock trimming
pub type CSITRIM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CSICFGR_SPEC, u8, u8, 6, O>;
impl R {
    ///Bits 0:8 - CSI clock calibration
    #[inline(always)]
    pub fn csical(&self) -> CSICAL_R {
        CSICAL_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 24:29 - CSI clock trimming
    #[inline(always)]
    pub fn csitrim(&self) -> CSITRIM_R {
        CSITRIM_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    ///Bits 0:8 - CSI clock calibration
    #[inline(always)]
    #[must_use]
    pub fn csical(&mut self) -> CSICAL_W<0> {
        CSICAL_W::new(self)
    }
    ///Bits 24:29 - CSI clock trimming
    #[inline(always)]
    #[must_use]
    pub fn csitrim(&mut self) -> CSITRIM_W<24> {
        CSITRIM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC CSI configuration register
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
///`reset()` method sets CSICFGR to value 0
impl crate::Resettable for CSICFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
