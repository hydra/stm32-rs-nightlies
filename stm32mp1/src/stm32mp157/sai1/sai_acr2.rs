///Register `SAI_ACR2` reader
pub struct R(crate::R<SAI_ACR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_ACR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_ACR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_ACR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SAI_ACR2` writer
pub struct W(crate::W<SAI_ACR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_ACR2_SPEC>;
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
impl From<crate::W<SAI_ACR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_ACR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FTH` reader - FTH
pub type FTH_R = crate::FieldReader<u8, u8>;
///Field `FTH` writer - FTH
pub type FTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_ACR2_SPEC, u8, u8, 3, O>;
///Field `FFLUSH` writer - FFLUSH
pub type FFLUSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR2_SPEC, bool, O>;
///Field `TRIS` reader - TRIS
pub type TRIS_R = crate::BitReader<bool>;
///Field `TRIS` writer - TRIS
pub type TRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR2_SPEC, bool, O>;
///Field `MUTE` reader - MUTE
pub type MUTE_R = crate::BitReader<bool>;
///Field `MUTE` writer - MUTE
pub type MUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR2_SPEC, bool, O>;
///Field `MUTEVAL` reader - MUTEVAL
pub type MUTEVAL_R = crate::BitReader<bool>;
///Field `MUTEVAL` writer - MUTEVAL
pub type MUTEVAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR2_SPEC, bool, O>;
///Field `MUTECNT` reader - MUTECNT
pub type MUTECNT_R = crate::FieldReader<u8, u8>;
///Field `MUTECNT` writer - MUTECNT
pub type MUTECNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_ACR2_SPEC, u8, u8, 6, O>;
///Field `CPL` reader - CPL
pub type CPL_R = crate::BitReader<bool>;
///Field `CPL` writer - CPL
pub type CPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACR2_SPEC, bool, O>;
///Field `COMP` reader - COMP
pub type COMP_R = crate::FieldReader<u8, u8>;
///Field `COMP` writer - COMP
pub type COMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_ACR2_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:2 - FTH
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 7) as u8)
    }
    ///Bit 4 - TRIS
    #[inline(always)]
    pub fn tris(&self) -> TRIS_R {
        TRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MUTE
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MUTEVAL
    #[inline(always)]
    pub fn muteval(&self) -> MUTEVAL_R {
        MUTEVAL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:12 - MUTECNT
    #[inline(always)]
    pub fn mutecnt(&self) -> MUTECNT_R {
        MUTECNT_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    ///Bit 13 - CPL
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - COMP
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    ///Bits 0:2 - FTH
    #[inline(always)]
    #[must_use]
    pub fn fth(&mut self) -> FTH_W<0> {
        FTH_W::new(self)
    }
    ///Bit 3 - FFLUSH
    #[inline(always)]
    #[must_use]
    pub fn fflush(&mut self) -> FFLUSH_W<3> {
        FFLUSH_W::new(self)
    }
    ///Bit 4 - TRIS
    #[inline(always)]
    #[must_use]
    pub fn tris(&mut self) -> TRIS_W<4> {
        TRIS_W::new(self)
    }
    ///Bit 5 - MUTE
    #[inline(always)]
    #[must_use]
    pub fn mute(&mut self) -> MUTE_W<5> {
        MUTE_W::new(self)
    }
    ///Bit 6 - MUTEVAL
    #[inline(always)]
    #[must_use]
    pub fn muteval(&mut self) -> MUTEVAL_W<6> {
        MUTEVAL_W::new(self)
    }
    ///Bits 7:12 - MUTECNT
    #[inline(always)]
    #[must_use]
    pub fn mutecnt(&mut self) -> MUTECNT_W<7> {
        MUTECNT_W::new(self)
    }
    ///Bit 13 - CPL
    #[inline(always)]
    #[must_use]
    pub fn cpl(&mut self) -> CPL_W<13> {
        CPL_W::new(self)
    }
    ///Bits 14:15 - COMP
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<14> {
        COMP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sai_acr2](index.html) module
pub struct SAI_ACR2_SPEC;
impl crate::RegisterSpec for SAI_ACR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [sai_acr2::R](R) reader structure
impl crate::Readable for SAI_ACR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sai_acr2::W](W) writer structure
impl crate::Writable for SAI_ACR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SAI_ACR2 to value 0
impl crate::Resettable for SAI_ACR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
