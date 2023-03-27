///Register `FDCAN_TURCF` reader
pub struct R(crate::R<FDCAN_TURCF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TURCF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TURCF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TURCF_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FDCAN_TURCF` writer
pub struct W(crate::W<FDCAN_TURCF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TURCF_SPEC>;
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
impl From<crate::W<FDCAN_TURCF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TURCF_SPEC>) -> Self {
        W(writer)
    }
}
///Field `NCL` reader - Numerator Configuration Low.
pub type NCL_R = crate::FieldReader<u16, u16>;
///Field `NCL` writer - Numerator Configuration Low.
pub type NCL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TURCF_SPEC, u16, u16, 16, O>;
///Field `DC` reader - Denominator Configuration.
pub type DC_R = crate::FieldReader<u16, u16>;
///Field `DC` writer - Denominator Configuration.
pub type DC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TURCF_SPEC, u16, u16, 14, O>;
///Field `ELT` reader - Enable Local Time
pub type ELT_R = crate::BitReader<bool>;
///Field `ELT` writer - Enable Local Time
pub type ELT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TURCF_SPEC, bool, O>;
impl R {
    ///Bits 0:15 - Numerator Configuration Low.
    #[inline(always)]
    pub fn ncl(&self) -> NCL_R {
        NCL_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:29 - Denominator Configuration.
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    ///Bit 31 - Enable Local Time
    #[inline(always)]
    pub fn elt(&self) -> ELT_R {
        ELT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:15 - Numerator Configuration Low.
    #[inline(always)]
    #[must_use]
    pub fn ncl(&mut self) -> NCL_W<0> {
        NCL_W::new(self)
    }
    ///Bits 16:29 - Denominator Configuration.
    #[inline(always)]
    #[must_use]
    pub fn dc(&mut self) -> DC_W<16> {
        DC_W::new(self)
    }
    ///Bit 31 - Enable Local Time
    #[inline(always)]
    #[must_use]
    pub fn elt(&mut self) -> ELT_W<31> {
        ELT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN TUR Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_turcf](index.html) module
pub struct FDCAN_TURCF_SPEC;
impl crate::RegisterSpec for FDCAN_TURCF_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_turcf::R](R) reader structure
impl crate::Readable for FDCAN_TURCF_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fdcan_turcf::W](W) writer structure
impl crate::Writable for FDCAN_TURCF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FDCAN_TURCF to value 0
impl crate::Resettable for FDCAN_TURCF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
