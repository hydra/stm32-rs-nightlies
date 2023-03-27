///Register `BGPFCCR` reader
pub struct R(crate::R<BGPFCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BGPFCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BGPFCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BGPFCCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BGPFCCR` writer
pub struct W(crate::W<BGPFCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BGPFCCR_SPEC>;
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
impl From<crate::W<BGPFCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BGPFCCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CM` reader - Color mode
pub type CM_R = crate::FieldReader<u8, u8>;
///Field `CM` writer - Color mode
pub type CM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BGPFCCR_SPEC, u8, u8, 4, O>;
///Field `CCM` reader - CLUT Color mode
pub type CCM_R = crate::BitReader<bool>;
///Field `CCM` writer - CLUT Color mode
pub type CCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BGPFCCR_SPEC, bool, O>;
///Field `START` reader - Start
pub type START_R = crate::BitReader<bool>;
///Field `START` writer - Start
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, BGPFCCR_SPEC, bool, O>;
///Field `CS` reader - CLUT size
pub type CS_R = crate::FieldReader<u8, u8>;
///Field `CS` writer - CLUT size
pub type CS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BGPFCCR_SPEC, u8, u8, 8, O>;
///Field `AM` reader - Alpha mode
pub type AM_R = crate::FieldReader<u8, u8>;
///Field `AM` writer - Alpha mode
pub type AM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BGPFCCR_SPEC, u8, u8, 2, O>;
///Field `ALPHA` reader - Alpha value
pub type ALPHA_R = crate::FieldReader<u8, u8>;
///Field `ALPHA` writer - Alpha value
pub type ALPHA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BGPFCCR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:3 - Color mode
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - CLUT Color mode
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Start
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:15 - CLUT size
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:17 - Alpha mode
    #[inline(always)]
    pub fn am(&self) -> AM_R {
        AM_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 24:31 - Alpha value
    #[inline(always)]
    pub fn alpha(&self) -> ALPHA_R {
        ALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:3 - Color mode
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<0> {
        CM_W::new(self)
    }
    ///Bit 4 - CLUT Color mode
    #[inline(always)]
    #[must_use]
    pub fn ccm(&mut self) -> CCM_W<4> {
        CCM_W::new(self)
    }
    ///Bit 5 - Start
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<5> {
        START_W::new(self)
    }
    ///Bits 8:15 - CLUT size
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CS_W<8> {
        CS_W::new(self)
    }
    ///Bits 16:17 - Alpha mode
    #[inline(always)]
    #[must_use]
    pub fn am(&mut self) -> AM_W<16> {
        AM_W::new(self)
    }
    ///Bits 24:31 - Alpha value
    #[inline(always)]
    #[must_use]
    pub fn alpha(&mut self) -> ALPHA_W<24> {
        ALPHA_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///background PFC control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bgpfccr](index.html) module
pub struct BGPFCCR_SPEC;
impl crate::RegisterSpec for BGPFCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [bgpfccr::R](R) reader structure
impl crate::Readable for BGPFCCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bgpfccr::W](W) writer structure
impl crate::Writable for BGPFCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BGPFCCR to value 0
impl crate::Resettable for BGPFCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
