///Register `SPLT` reader
pub struct R(crate::R<SPLT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPLT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPLT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPLT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SPLT` writer
pub struct W(crate::W<SPLT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPLT_SPEC>;
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
impl From<crate::W<SPLT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPLT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PRTADDR` reader - Port address
pub type PRTADDR_R = crate::FieldReader<u8, u8>;
///Field `PRTADDR` writer - Port address
pub type PRTADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPLT_SPEC, u8, u8, 7, O>;
///Field `HUBADDR` reader - Hub address
pub type HUBADDR_R = crate::FieldReader<u8, u8>;
///Field `HUBADDR` writer - Hub address
pub type HUBADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPLT_SPEC, u8, u8, 7, O>;
///Field `XACTPOS` reader - XACTPOS
pub type XACTPOS_R = crate::FieldReader<u8, u8>;
///Field `XACTPOS` writer - XACTPOS
pub type XACTPOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPLT_SPEC, u8, u8, 2, O>;
///Field `COMPLSPLT` reader - Do complete split
pub type COMPLSPLT_R = crate::BitReader<bool>;
///Field `COMPLSPLT` writer - Do complete split
pub type COMPLSPLT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPLT_SPEC, bool, O>;
///Field `SPLITEN` reader - Split enable
pub type SPLITEN_R = crate::BitReader<bool>;
///Field `SPLITEN` writer - Split enable
pub type SPLITEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPLT_SPEC, bool, O>;
impl R {
    ///Bits 0:6 - Port address
    #[inline(always)]
    pub fn prtaddr(&self) -> PRTADDR_R {
        PRTADDR_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 7:13 - Hub address
    #[inline(always)]
    pub fn hubaddr(&self) -> HUBADDR_R {
        HUBADDR_R::new(((self.bits >> 7) & 0x7f) as u8)
    }
    ///Bits 14:15 - XACTPOS
    #[inline(always)]
    pub fn xactpos(&self) -> XACTPOS_R {
        XACTPOS_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - Do complete split
    #[inline(always)]
    pub fn complsplt(&self) -> COMPLSPLT_R {
        COMPLSPLT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 31 - Split enable
    #[inline(always)]
    pub fn spliten(&self) -> SPLITEN_R {
        SPLITEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:6 - Port address
    #[inline(always)]
    #[must_use]
    pub fn prtaddr(&mut self) -> PRTADDR_W<0> {
        PRTADDR_W::new(self)
    }
    ///Bits 7:13 - Hub address
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
    ///Bit 16 - Do complete split
    #[inline(always)]
    #[must_use]
    pub fn complsplt(&mut self) -> COMPLSPLT_W<16> {
        COMPLSPLT_W::new(self)
    }
    ///Bit 31 - Split enable
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
///OTG_HS host channel-0 split control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [splt](index.html) module
pub struct SPLT_SPEC;
impl crate::RegisterSpec for SPLT_SPEC {
    type Ux = u32;
}
///`read()` method returns [splt::R](R) reader structure
impl crate::Readable for SPLT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [splt::W](W) writer structure
impl crate::Writable for SPLT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SPLT to value 0
impl crate::Resettable for SPLT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
