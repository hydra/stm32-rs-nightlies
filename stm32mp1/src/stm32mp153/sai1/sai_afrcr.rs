///Register `SAI_AFRCR` reader
pub struct R(crate::R<SAI_AFRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_AFRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_AFRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_AFRCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SAI_AFRCR` writer
pub struct W(crate::W<SAI_AFRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_AFRCR_SPEC>;
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
impl From<crate::W<SAI_AFRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_AFRCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FRL` reader - FRL
pub type FRL_R = crate::FieldReader<u8, u8>;
///Field `FRL` writer - FRL
pub type FRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_AFRCR_SPEC, u8, u8, 8, O>;
///Field `FSALL` reader - FSALL
pub type FSALL_R = crate::FieldReader<u8, u8>;
///Field `FSALL` writer - FSALL
pub type FSALL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAI_AFRCR_SPEC, u8, u8, 7, O>;
///Field `FSDEF` reader - FSDEF
pub type FSDEF_R = crate::BitReader<bool>;
///Field `FSPOL` reader - FSPOL
pub type FSPOL_R = crate::BitReader<bool>;
///Field `FSPOL` writer - FSPOL
pub type FSPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_AFRCR_SPEC, bool, O>;
///Field `FSOFF` reader - FSOFF
pub type FSOFF_R = crate::BitReader<bool>;
///Field `FSOFF` writer - FSOFF
pub type FSOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_AFRCR_SPEC, bool, O>;
impl R {
    ///Bits 0:7 - FRL
    #[inline(always)]
    pub fn frl(&self) -> FRL_R {
        FRL_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:14 - FSALL
    #[inline(always)]
    pub fn fsall(&self) -> FSALL_R {
        FSALL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 16 - FSDEF
    #[inline(always)]
    pub fn fsdef(&self) -> FSDEF_R {
        FSDEF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - FSPOL
    #[inline(always)]
    pub fn fspol(&self) -> FSPOL_R {
        FSPOL_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - FSOFF
    #[inline(always)]
    pub fn fsoff(&self) -> FSOFF_R {
        FSOFF_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - FRL
    #[inline(always)]
    #[must_use]
    pub fn frl(&mut self) -> FRL_W<0> {
        FRL_W::new(self)
    }
    ///Bits 8:14 - FSALL
    #[inline(always)]
    #[must_use]
    pub fn fsall(&mut self) -> FSALL_W<8> {
        FSALL_W::new(self)
    }
    ///Bit 17 - FSPOL
    #[inline(always)]
    #[must_use]
    pub fn fspol(&mut self) -> FSPOL_W<17> {
        FSPOL_W::new(self)
    }
    ///Bit 18 - FSOFF
    #[inline(always)]
    #[must_use]
    pub fn fsoff(&mut self) -> FSOFF_W<18> {
        FSOFF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register has no meaning in and SPDIF audio protocol
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sai_afrcr](index.html) module
pub struct SAI_AFRCR_SPEC;
impl crate::RegisterSpec for SAI_AFRCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sai_afrcr::R](R) reader structure
impl crate::Readable for SAI_AFRCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sai_afrcr::W](W) writer structure
impl crate::Writable for SAI_AFRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SAI_AFRCR to value 0x07
impl crate::Resettable for SAI_AFRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
