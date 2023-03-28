///Register `NBTP` reader
pub struct R(crate::R<NBTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NBTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NBTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NBTP_SPEC>) -> Self {
        R(reader)
    }
}
///Register `NBTP` writer
pub struct W(crate::W<NBTP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NBTP_SPEC>;
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
impl From<crate::W<NBTP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NBTP_SPEC>) -> Self {
        W(writer)
    }
}
///Field `NTSEG2` reader - TSEG2
pub type NTSEG2_R = crate::FieldReader<u8, u8>;
///Field `NTSEG2` writer - TSEG2
pub type NTSEG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NBTP_SPEC, u8, u8, 7, O>;
///Field `NTSEG1` reader - NTSEG1
pub type NTSEG1_R = crate::FieldReader<u8, u8>;
///Field `NTSEG1` writer - NTSEG1
pub type NTSEG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NBTP_SPEC, u8, u8, 8, O>;
///Field `NBRP` reader - NBRP
pub type NBRP_R = crate::FieldReader<u16, u16>;
///Field `NBRP` writer - NBRP
pub type NBRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NBTP_SPEC, u16, u16, 9, O>;
///Field `NSJW` reader - NSJW
pub type NSJW_R = crate::FieldReader<u8, u8>;
///Field `NSJW` writer - NSJW
pub type NSJW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NBTP_SPEC, u8, u8, 7, O>;
impl R {
    ///Bits 0:6 - TSEG2
    #[inline(always)]
    pub fn ntseg2(&self) -> NTSEG2_R {
        NTSEG2_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:15 - NTSEG1
    #[inline(always)]
    pub fn ntseg1(&self) -> NTSEG1_R {
        NTSEG1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:24 - NBRP
    #[inline(always)]
    pub fn nbrp(&self) -> NBRP_R {
        NBRP_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    ///Bits 25:31 - NSJW
    #[inline(always)]
    pub fn nsjw(&self) -> NSJW_R {
        NSJW_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:6 - TSEG2
    #[inline(always)]
    #[must_use]
    pub fn ntseg2(&mut self) -> NTSEG2_W<0> {
        NTSEG2_W::new(self)
    }
    ///Bits 8:15 - NTSEG1
    #[inline(always)]
    #[must_use]
    pub fn ntseg1(&mut self) -> NTSEG1_W<8> {
        NTSEG1_W::new(self)
    }
    ///Bits 16:24 - NBRP
    #[inline(always)]
    #[must_use]
    pub fn nbrp(&mut self) -> NBRP_W<16> {
        NBRP_W::new(self)
    }
    ///Bits 25:31 - NSJW
    #[inline(always)]
    #[must_use]
    pub fn nsjw(&mut self) -> NSJW_W<25> {
        NSJW_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN_NBTP
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [nbtp](index.html) module
pub struct NBTP_SPEC;
impl crate::RegisterSpec for NBTP_SPEC {
    type Ux = u32;
}
///`read()` method returns [nbtp::R](R) reader structure
impl crate::Readable for NBTP_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [nbtp::W](W) writer structure
impl crate::Writable for NBTP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets NBTP to value 0x0a33
impl crate::Resettable for NBTP_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a33;
}
