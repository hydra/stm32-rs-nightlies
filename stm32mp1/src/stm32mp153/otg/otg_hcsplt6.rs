///Register `OTG_HCSPLT6` reader
pub struct R(crate::R<OTG_HCSPLT6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HCSPLT6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HCSPLT6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HCSPLT6_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OTG_HCSPLT6` writer
pub struct W(crate::W<OTG_HCSPLT6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_HCSPLT6_SPEC>;
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
impl From<crate::W<OTG_HCSPLT6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_HCSPLT6_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PRTADDR` reader - PRTADDR
pub type PRTADDR_R = crate::FieldReader<u8, u8>;
///Field `PRTADDR` writer - PRTADDR
pub type PRTADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_HCSPLT6_SPEC, u8, u8, 7, O>;
///Field `HUBADDR` reader - HUBADDR
pub type HUBADDR_R = crate::FieldReader<u8, u8>;
///Field `HUBADDR` writer - HUBADDR
pub type HUBADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_HCSPLT6_SPEC, u8, u8, 7, O>;
///Field `XACTPOS` reader - XACTPOS
pub type XACTPOS_R = crate::FieldReader<u8, u8>;
///Field `XACTPOS` writer - XACTPOS
pub type XACTPOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_HCSPLT6_SPEC, u8, u8, 2, O>;
///Field `COMPLSPLT` reader - COMPLSPLT
pub type COMPLSPLT_R = crate::BitReader<bool>;
///Field `COMPLSPLT` writer - COMPLSPLT
pub type COMPLSPLT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCSPLT6_SPEC, bool, O>;
///Field `SPLITEN` reader - SPLITEN
pub type SPLITEN_R = crate::BitReader<bool>;
///Field `SPLITEN` writer - SPLITEN
pub type SPLITEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HCSPLT6_SPEC, bool, O>;
impl R {
    ///Bits 0:6 - PRTADDR
    #[inline(always)]
    pub fn prtaddr(&self) -> PRTADDR_R {
        PRTADDR_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 7:13 - HUBADDR
    #[inline(always)]
    pub fn hubaddr(&self) -> HUBADDR_R {
        HUBADDR_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    ///Bits 14:15 - XACTPOS
    #[inline(always)]
    pub fn xactpos(&self) -> XACTPOS_R {
        XACTPOS_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - COMPLSPLT
    #[inline(always)]
    pub fn complsplt(&self) -> COMPLSPLT_R {
        COMPLSPLT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 31 - SPLITEN
    #[inline(always)]
    pub fn spliten(&self) -> SPLITEN_R {
        SPLITEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:6 - PRTADDR
    #[inline(always)]
    #[must_use]
    pub fn prtaddr(&mut self) -> PRTADDR_W<0> {
        PRTADDR_W::new(self)
    }
    ///Bits 7:13 - HUBADDR
    #[inline(always)]
    #[must_use]
    pub fn hubaddr(&mut self) -> HUBADDR_W<7> {
        HUBADDR_W::new(self)
    }
    ///Bits 14:15 - XACTPOS
    #[inline(always)]
    #[must_use]
    pub fn xactpos(&mut self) -> XACTPOS_W<14> {
        XACTPOS_W::new(self)
    }
    ///Bit 16 - COMPLSPLT
    #[inline(always)]
    #[must_use]
    pub fn complsplt(&mut self) -> COMPLSPLT_W<16> {
        COMPLSPLT_W::new(self)
    }
    ///Bit 31 - SPLITEN
    #[inline(always)]
    #[must_use]
    pub fn spliten(&mut self) -> SPLITEN_W<31> {
        SPLITEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTG host channel 6 split control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_hcsplt6](index.html) module
pub struct OTG_HCSPLT6_SPEC;
impl crate::RegisterSpec for OTG_HCSPLT6_SPEC {
    type Ux = u32;
}
///`read()` method returns [otg_hcsplt6::R](R) reader structure
impl crate::Readable for OTG_HCSPLT6_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [otg_hcsplt6::W](W) writer structure
impl crate::Writable for OTG_HCSPLT6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OTG_HCSPLT6 to value 0
impl crate::Resettable for OTG_HCSPLT6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
