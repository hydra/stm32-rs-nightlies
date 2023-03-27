///Register `HSICFGR` reader
pub struct R(crate::R<HSICFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSICFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSICFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSICFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `HSICFGR` writer
pub struct W(crate::W<HSICFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSICFGR_SPEC>;
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
impl From<crate::W<HSICFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSICFGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HSICAL` reader - HSI clock calibration
pub type HSICAL_R = crate::FieldReader<u16, u16>;
///Field `HSICAL` writer - HSI clock calibration
pub type HSICAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSICFGR_SPEC, u16, u16, 12, O>;
///Field `HSITRIM` reader - HSI clock trimming
pub type HSITRIM_R = crate::FieldReader<u8, u8>;
///Field `HSITRIM` writer - HSI clock trimming
pub type HSITRIM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, HSICFGR_SPEC, u8, u8, 7, O>;
impl R {
    ///Bits 0:11 - HSI clock calibration
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 24:30 - HSI clock trimming
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:11 - HSI clock calibration
    #[inline(always)]
    #[must_use]
    pub fn hsical(&mut self) -> HSICAL_W<0> {
        HSICAL_W::new(self)
    }
    ///Bits 24:30 - HSI clock trimming
    #[inline(always)]
    #[must_use]
    pub fn hsitrim(&mut self) -> HSITRIM_W<24> {
        HSITRIM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RCC HSI configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hsicfgr](index.html) module
pub struct HSICFGR_SPEC;
impl crate::RegisterSpec for HSICFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [hsicfgr::R](R) reader structure
impl crate::Readable for HSICFGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [hsicfgr::W](W) writer structure
impl crate::Writable for HSICFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets HSICFGR to value 0
impl crate::Resettable for HSICFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
