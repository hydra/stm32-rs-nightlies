///Register `DMABMR` reader
pub struct R(crate::R<DMABMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMABMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMABMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMABMR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMABMR` writer
pub struct W(crate::W<DMABMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMABMR_SPEC>;
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
impl From<crate::W<DMABMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMABMR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SR` reader - SR
pub type SR_R = crate::BitReader<bool>;
///Field `SR` writer - SR
pub type SR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABMR_SPEC, bool, O>;
///Field `DA` reader - DA
pub type DA_R = crate::BitReader<bool>;
///Field `DA` writer - DA
pub type DA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABMR_SPEC, bool, O>;
///Field `DSL` reader - DSL
pub type DSL_R = crate::FieldReader<u8, u8>;
///Field `DSL` writer - DSL
pub type DSL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMABMR_SPEC, u8, u8, 5, O>;
///Field `EDFE` reader - EDFE
pub type EDFE_R = crate::BitReader<bool>;
///Field `EDFE` writer - EDFE
pub type EDFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABMR_SPEC, bool, O>;
///Field `PBL` reader - PBL
pub type PBL_R = crate::FieldReader<u8, u8>;
///Field `PBL` writer - PBL
pub type PBL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMABMR_SPEC, u8, u8, 6, O>;
///Field `RTPR` reader - RTPR
pub type RTPR_R = crate::FieldReader<u8, u8>;
///Field `RTPR` writer - RTPR
pub type RTPR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMABMR_SPEC, u8, u8, 2, O>;
///Field `FB` reader - FB
pub type FB_R = crate::BitReader<bool>;
///Field `FB` writer - FB
pub type FB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABMR_SPEC, bool, O>;
///Field `RDP` reader - RDP
pub type RDP_R = crate::FieldReader<u8, u8>;
///Field `RDP` writer - RDP
pub type RDP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMABMR_SPEC, u8, u8, 6, O>;
///Field `USP` reader - USP
pub type USP_R = crate::BitReader<bool>;
///Field `USP` writer - USP
pub type USP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABMR_SPEC, bool, O>;
///Field `FPM` reader - FPM
pub type FPM_R = crate::BitReader<bool>;
///Field `FPM` writer - FPM
pub type FPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABMR_SPEC, bool, O>;
///Field `AAB` reader - AAB
pub type AAB_R = crate::BitReader<bool>;
///Field `AAB` writer - AAB
pub type AAB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABMR_SPEC, bool, O>;
///Field `MB` reader - MB
pub type MB_R = crate::BitReader<bool>;
///Field `MB` writer - MB
pub type MB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABMR_SPEC, bool, O>;
impl R {
    ///Bit 0 - SR
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DA
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:6 - DSL
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    ///Bit 7 - EDFE
    #[inline(always)]
    pub fn edfe(&self) -> EDFE_R {
        EDFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:13 - PBL
    #[inline(always)]
    pub fn pbl(&self) -> PBL_R {
        PBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 14:15 - RTPR
    #[inline(always)]
    pub fn rtpr(&self) -> RTPR_R {
        RTPR_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - FB
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:22 - RDP
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    ///Bit 23 - USP
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - FPM
    #[inline(always)]
    pub fn fpm(&self) -> FPM_R {
        FPM_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - AAB
    #[inline(always)]
    pub fn aab(&self) -> AAB_R {
        AAB_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - MB
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SR
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SR_W<0> {
        SR_W::new(self)
    }
    ///Bit 1 - DA
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<1> {
        DA_W::new(self)
    }
    ///Bits 2:6 - DSL
    #[inline(always)]
    #[must_use]
    pub fn dsl(&mut self) -> DSL_W<2> {
        DSL_W::new(self)
    }
    ///Bit 7 - EDFE
    #[inline(always)]
    #[must_use]
    pub fn edfe(&mut self) -> EDFE_W<7> {
        EDFE_W::new(self)
    }
    ///Bits 8:13 - PBL
    #[inline(always)]
    #[must_use]
    pub fn pbl(&mut self) -> PBL_W<8> {
        PBL_W::new(self)
    }
    ///Bits 14:15 - RTPR
    #[inline(always)]
    #[must_use]
    pub fn rtpr(&mut self) -> RTPR_W<14> {
        RTPR_W::new(self)
    }
    ///Bit 16 - FB
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FB_W<16> {
        FB_W::new(self)
    }
    ///Bits 17:22 - RDP
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<17> {
        RDP_W::new(self)
    }
    ///Bit 23 - USP
    #[inline(always)]
    #[must_use]
    pub fn usp(&mut self) -> USP_W<23> {
        USP_W::new(self)
    }
    ///Bit 24 - FPM
    #[inline(always)]
    #[must_use]
    pub fn fpm(&mut self) -> FPM_W<24> {
        FPM_W::new(self)
    }
    ///Bit 25 - AAB
    #[inline(always)]
    #[must_use]
    pub fn aab(&mut self) -> AAB_W<25> {
        AAB_W::new(self)
    }
    ///Bit 26 - MB
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MB_W<26> {
        MB_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet DMA bus mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmabmr](index.html) module
pub struct DMABMR_SPEC;
impl crate::RegisterSpec for DMABMR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmabmr::R](R) reader structure
impl crate::Readable for DMABMR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dmabmr::W](W) writer structure
impl crate::Writable for DMABMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMABMR to value 0x2101
impl crate::Resettable for DMABMR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2101;
}
