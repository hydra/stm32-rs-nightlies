///Register `DFSDM_FLT4AWLTR` reader
pub struct R(crate::R<DFSDM_FLT4AWLTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_FLT4AWLTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_FLT4AWLTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_FLT4AWLTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DFSDM_FLT4AWLTR` writer
pub struct W(crate::W<DFSDM_FLT4AWLTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSDM_FLT4AWLTR_SPEC>;
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
impl From<crate::W<DFSDM_FLT4AWLTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSDM_FLT4AWLTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BKAWL` reader - BKAWL
pub type BKAWL_R = crate::FieldReader<u8, u8>;
///Field `BKAWL` writer - BKAWL
pub type BKAWL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFSDM_FLT4AWLTR_SPEC, u8, u8, 4, O>;
///Field `AWLT` reader - AWLT
pub type AWLT_R = crate::FieldReader<u32, u32>;
///Field `AWLT` writer - AWLT
pub type AWLT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFSDM_FLT4AWLTR_SPEC, u32, u32, 24, O>;
impl R {
    ///Bits 0:3 - BKAWL
    #[inline(always)]
    pub fn bkawl(&self) -> BKAWL_R {
        BKAWL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:31 - AWLT
    #[inline(always)]
    pub fn awlt(&self) -> AWLT_R {
        AWLT_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    ///Bits 0:3 - BKAWL
    #[inline(always)]
    #[must_use]
    pub fn bkawl(&mut self) -> BKAWL_W<0> {
        BKAWL_W::new(self)
    }
    ///Bits 8:31 - AWLT
    #[inline(always)]
    #[must_use]
    pub fn awlt(&mut self) -> AWLT_W<8> {
        AWLT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DFSDM filter 4 analog watchdog low threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dfsdm_flt4awltr](index.html) module
pub struct DFSDM_FLT4AWLTR_SPEC;
impl crate::RegisterSpec for DFSDM_FLT4AWLTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dfsdm_flt4awltr::R](R) reader structure
impl crate::Readable for DFSDM_FLT4AWLTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dfsdm_flt4awltr::W](W) writer structure
impl crate::Writable for DFSDM_FLT4AWLTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DFSDM_FLT4AWLTR to value 0
impl crate::Resettable for DFSDM_FLT4AWLTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
