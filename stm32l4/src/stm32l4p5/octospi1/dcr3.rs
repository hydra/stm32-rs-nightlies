///Register `DCR3` reader
pub struct R(crate::R<DCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DCR3` writer
pub struct W(crate::W<DCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCR3_SPEC>;
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
impl From<crate::W<DCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MAXTRAN` reader - Maximum transfer
pub type MAXTRAN_R = crate::FieldReader<u8, u8>;
///Field `MAXTRAN` writer - Maximum transfer
pub type MAXTRAN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DCR3_SPEC, u8, u8, 8, O>;
///Field `CSBOUND` reader - CS boundary
pub type CSBOUND_R = crate::FieldReader<u8, u8>;
///Field `CSBOUND` writer - CS boundary
pub type CSBOUND_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DCR3_SPEC, u8, u8, 5, O>;
impl R {
    ///Bits 0:7 - Maximum transfer
    #[inline(always)]
    pub fn maxtran(&self) -> MAXTRAN_R {
        MAXTRAN_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:20 - CS boundary
    #[inline(always)]
    pub fn csbound(&self) -> CSBOUND_R {
        CSBOUND_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:7 - Maximum transfer
    #[inline(always)]
    #[must_use]
    pub fn maxtran(&mut self) -> MAXTRAN_W<0> {
        MAXTRAN_W::new(self)
    }
    ///Bits 16:20 - CS boundary
    #[inline(always)]
    #[must_use]
    pub fn csbound(&mut self) -> CSBOUND_W<16> {
        CSBOUND_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///device configuration register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dcr3](index.html) module
pub struct DCR3_SPEC;
impl crate::RegisterSpec for DCR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [dcr3::R](R) reader structure
impl crate::Readable for DCR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dcr3::W](W) writer structure
impl crate::Writable for DCR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DCR3 to value 0
impl crate::Resettable for DCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
