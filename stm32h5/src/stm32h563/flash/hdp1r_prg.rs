///Register `HDP1R_PRG` reader
pub struct R(crate::R<HDP1R_PRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HDP1R_PRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HDP1R_PRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HDP1R_PRG_SPEC>) -> Self {
        R(reader)
    }
}
///Register `HDP1R_PRG` writer
pub struct W(crate::W<HDP1R_PRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HDP1R_PRG_SPEC>;
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
impl From<crate::W<HDP1R_PRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HDP1R_PRG_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HDP1_STRT` reader - HDPL barrier start set in number of 8-Kbyte sectors
pub type HDP1_STRT_R = crate::FieldReader<u8, u8>;
///Field `HDP1_STRT` writer - HDPL barrier start set in number of 8-Kbyte sectors
pub type HDP1_STRT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HDP1R_PRG_SPEC, u8, u8, 7, O>;
///Field `HDP1_END` reader - HDPL barrier end set in number of 8-Kbyte sectors
pub type HDP1_END_R = crate::FieldReader<u8, u8>;
///Field `HDP1_END` writer - HDPL barrier end set in number of 8-Kbyte sectors
pub type HDP1_END_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HDP1R_PRG_SPEC, u8, u8, 7, O>;
impl R {
    ///Bits 0:6 - HDPL barrier start set in number of 8-Kbyte sectors
    #[inline(always)]
    pub fn hdp1_strt(&self) -> HDP1_STRT_R {
        HDP1_STRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - HDPL barrier end set in number of 8-Kbyte sectors
    #[inline(always)]
    pub fn hdp1_end(&self) -> HDP1_END_R {
        HDP1_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:6 - HDPL barrier start set in number of 8-Kbyte sectors
    #[inline(always)]
    #[must_use]
    pub fn hdp1_strt(&mut self) -> HDP1_STRT_W<0> {
        HDP1_STRT_W::new(self)
    }
    ///Bits 16:22 - HDPL barrier end set in number of 8-Kbyte sectors
    #[inline(always)]
    #[must_use]
    pub fn hdp1_end(&mut self) -> HDP1_END_W<16> {
        HDP1_END_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH HDP Bank 1 configuration
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hdp1r_prg](index.html) module
pub struct HDP1R_PRG_SPEC;
impl crate::RegisterSpec for HDP1R_PRG_SPEC {
    type Ux = u32;
}
///`read()` method returns [hdp1r_prg::R](R) reader structure
impl crate::Readable for HDP1R_PRG_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [hdp1r_prg::W](W) writer structure
impl crate::Writable for HDP1R_PRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets HDP1R_PRG to value 0
impl crate::Resettable for HDP1R_PRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
