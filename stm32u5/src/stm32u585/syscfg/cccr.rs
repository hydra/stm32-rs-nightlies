///Register `CCCR` reader
pub struct R(crate::R<CCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCCR` writer
pub struct W(crate::W<CCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCCR_SPEC>;
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
impl From<crate::W<CCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `NCC1` reader - NCC1
pub type NCC1_R = crate::FieldReader<u8, u8>;
///Field `NCC1` writer - NCC1
pub type NCC1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCCR_SPEC, u8, u8, 4, O>;
///Field `PCC1` reader - PCC1
pub type PCC1_R = crate::FieldReader<u8, u8>;
///Field `PCC1` writer - PCC1
pub type PCC1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCCR_SPEC, u8, u8, 4, O>;
///Field `NCC2` reader - NCC2
pub type NCC2_R = crate::FieldReader<u8, u8>;
///Field `NCC2` writer - NCC2
pub type NCC2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCCR_SPEC, u8, u8, 4, O>;
///Field `PCC2` reader - PCC2
pub type PCC2_R = crate::FieldReader<u8, u8>;
///Field `PCC2` writer - PCC2
pub type PCC2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCCR_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:3 - NCC1
    #[inline(always)]
    pub fn ncc1(&self) -> NCC1_R {
        NCC1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - PCC1
    #[inline(always)]
    pub fn pcc1(&self) -> PCC1_R {
        PCC1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - NCC2
    #[inline(always)]
    pub fn ncc2(&self) -> NCC2_R {
        NCC2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - PCC2
    #[inline(always)]
    pub fn pcc2(&self) -> PCC2_R {
        PCC2_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - NCC1
    #[inline(always)]
    #[must_use]
    pub fn ncc1(&mut self) -> NCC1_W<0> {
        NCC1_W::new(self)
    }
    ///Bits 4:7 - PCC1
    #[inline(always)]
    #[must_use]
    pub fn pcc1(&mut self) -> PCC1_W<4> {
        PCC1_W::new(self)
    }
    ///Bits 8:11 - NCC2
    #[inline(always)]
    #[must_use]
    pub fn ncc2(&mut self) -> NCC2_W<8> {
        NCC2_W::new(self)
    }
    ///Bits 12:15 - PCC2
    #[inline(always)]
    #[must_use]
    pub fn pcc2(&mut self) -> PCC2_W<12> {
        PCC2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///compensation cell code register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cccr](index.html) module
pub struct CCCR_SPEC;
impl crate::RegisterSpec for CCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cccr::R](R) reader structure
impl crate::Readable for CCCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cccr::W](W) writer structure
impl crate::Writable for CCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCCR to value 0x7878
impl crate::Resettable for CCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x7878;
}
