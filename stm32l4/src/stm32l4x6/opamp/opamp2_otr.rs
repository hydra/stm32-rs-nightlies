///Register `OPAMP2_OTR` reader
pub struct R(crate::R<OPAMP2_OTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPAMP2_OTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPAMP2_OTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPAMP2_OTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OPAMP2_OTR` writer
pub struct W(crate::W<OPAMP2_OTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPAMP2_OTR_SPEC>;
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
impl From<crate::W<OPAMP2_OTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPAMP2_OTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TRIMOFFSETN` reader - Trim for NMOS differential pairs
pub type TRIMOFFSETN_R = crate::FieldReader<u8, u8>;
///Field `TRIMOFFSETN` writer - Trim for NMOS differential pairs
pub type TRIMOFFSETN_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OPAMP2_OTR_SPEC, u8, u8, 5, O>;
///Field `TRIMOFFSETP` reader - Trim for PMOS differential pairs
pub type TRIMOFFSETP_R = crate::FieldReader<u8, u8>;
///Field `TRIMOFFSETP` writer - Trim for PMOS differential pairs
pub type TRIMOFFSETP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, OPAMP2_OTR_SPEC, u8, u8, 5, O>;
impl R {
    ///Bits 0:4 - Trim for NMOS differential pairs
    #[inline(always)]
    pub fn trimoffsetn(&self) -> TRIMOFFSETN_R {
        TRIMOFFSETN_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:12 - Trim for PMOS differential pairs
    #[inline(always)]
    pub fn trimoffsetp(&self) -> TRIMOFFSETP_R {
        TRIMOFFSETP_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:4 - Trim for NMOS differential pairs
    #[inline(always)]
    #[must_use]
    pub fn trimoffsetn(&mut self) -> TRIMOFFSETN_W<0> {
        TRIMOFFSETN_W::new(self)
    }
    ///Bits 8:12 - Trim for PMOS differential pairs
    #[inline(always)]
    #[must_use]
    pub fn trimoffsetp(&mut self) -> TRIMOFFSETP_W<8> {
        TRIMOFFSETP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OPAMP2 offset trimming register in normal mode
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [opamp2_otr](index.html) module
pub struct OPAMP2_OTR_SPEC;
impl crate::RegisterSpec for OPAMP2_OTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [opamp2_otr::R](R) reader structure
impl crate::Readable for OPAMP2_OTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [opamp2_otr::W](W) writer structure
impl crate::Writable for OPAMP2_OTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OPAMP2_OTR to value 0
impl crate::Resettable for OPAMP2_OTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
