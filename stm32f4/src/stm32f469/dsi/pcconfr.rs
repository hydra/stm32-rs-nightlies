///Register `PCCONFR` reader
pub struct R(crate::R<PCCONFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCCONFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCCONFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCCONFR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PCCONFR` writer
pub struct W(crate::W<PCCONFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCCONFR_SPEC>;
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
impl From<crate::W<PCCONFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCCONFR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `NL` reader - NL
pub type NL_R = crate::FieldReader<u8, u8>;
///Field `NL` writer - NL
pub type NL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCCONFR_SPEC, u8, u8, 2, O>;
///Field `SW_TIME` reader - SW_TIME
pub type SW_TIME_R = crate::FieldReader<u8, u8>;
///Field `SW_TIME` writer - SW_TIME
pub type SW_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCCONFR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:1 - NL
    #[inline(always)]
    pub fn nl(&self) -> NL_R {
        NL_R::new((self.bits & 3) as u8)
    }
    ///Bits 8:15 - SW_TIME
    #[inline(always)]
    pub fn sw_time(&self) -> SW_TIME_R {
        SW_TIME_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:1 - NL
    #[inline(always)]
    #[must_use]
    pub fn nl(&mut self) -> NL_W<0> {
        NL_W::new(self)
    }
    ///Bits 8:15 - SW_TIME
    #[inline(always)]
    #[must_use]
    pub fn sw_time(&mut self) -> SW_TIME_W<8> {
        SW_TIME_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host PHY Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pcconfr](index.html) module
pub struct PCCONFR_SPEC;
impl crate::RegisterSpec for PCCONFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pcconfr::R](R) reader structure
impl crate::Readable for PCCONFR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pcconfr::W](W) writer structure
impl crate::Writable for PCCONFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PCCONFR to value 0x3133_302a
impl crate::Resettable for PCCONFR_SPEC {
    const RESET_VALUE: Self::Ux = 0x3133_302a;
}
