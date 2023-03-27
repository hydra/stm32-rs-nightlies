///Register `GHCR` reader
pub struct R(crate::R<GHCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GHCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GHCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GHCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GHCR` writer
pub struct W(crate::W<GHCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GHCR_SPEC>;
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
impl From<crate::W<GHCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GHCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DT` reader - Type
pub type DT_R = crate::FieldReader<u8, u8>;
///Field `DT` writer - Type
pub type DT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GHCR_SPEC, u8, u8, 6, O>;
///Field `VCID` reader - Channel
pub type VCID_R = crate::FieldReader<u8, u8>;
///Field `VCID` writer - Channel
pub type VCID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GHCR_SPEC, u8, u8, 2, O>;
///Field `WCLSB` reader - WordCount LSB
pub type WCLSB_R = crate::FieldReader<u8, u8>;
///Field `WCLSB` writer - WordCount LSB
pub type WCLSB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GHCR_SPEC, u8, u8, 8, O>;
///Field `WCMSB` reader - WordCount MSB
pub type WCMSB_R = crate::FieldReader<u8, u8>;
///Field `WCMSB` writer - WordCount MSB
pub type WCMSB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GHCR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:5 - Type
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:7 - Channel
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:15 - WordCount LSB
    #[inline(always)]
    pub fn wclsb(&self) -> WCLSB_R {
        WCLSB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - WordCount MSB
    #[inline(always)]
    pub fn wcmsb(&self) -> WCMSB_R {
        WCMSB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:5 - Type
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<0> {
        DT_W::new(self)
    }
    ///Bits 6:7 - Channel
    #[inline(always)]
    #[must_use]
    pub fn vcid(&mut self) -> VCID_W<6> {
        VCID_W::new(self)
    }
    ///Bits 8:15 - WordCount LSB
    #[inline(always)]
    #[must_use]
    pub fn wclsb(&mut self) -> WCLSB_W<8> {
        WCLSB_W::new(self)
    }
    ///Bits 16:23 - WordCount MSB
    #[inline(always)]
    #[must_use]
    pub fn wcmsb(&mut self) -> WCMSB_W<16> {
        WCMSB_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host generic header configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ghcr](index.html) module
pub struct GHCR_SPEC;
impl crate::RegisterSpec for GHCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ghcr::R](R) reader structure
impl crate::Readable for GHCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ghcr::W](W) writer structure
impl crate::Writable for GHCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GHCR to value 0
impl crate::Resettable for GHCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
