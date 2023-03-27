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
///Field `SR` reader - Software reset
pub type SR_R = crate::BitReader<bool>;
///Field `SR` writer - Software reset
pub type SR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABMR_SPEC, bool, O>;
///Field `DA` reader - DMA Arbitration
pub type DA_R = crate::BitReader<bool>;
///Field `DA` writer - DMA Arbitration
pub type DA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABMR_SPEC, bool, O>;
///Field `DSL` reader - Descriptor skip length
pub type DSL_R = crate::FieldReader<u8, u8>;
///Field `DSL` writer - Descriptor skip length
pub type DSL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMABMR_SPEC, u8, u8, 5, O>;
///Field `EDFE` reader - Enhanced descriptor format enable
pub type EDFE_R = crate::BitReader<bool>;
///Field `EDFE` writer - Enhanced descriptor format enable
pub type EDFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABMR_SPEC, bool, O>;
///Field `PBL` reader - Programmable burst length
pub type PBL_R = crate::FieldReader<u8, u8>;
///Field `PBL` writer - Programmable burst length
pub type PBL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMABMR_SPEC, u8, u8, 6, O>;
///Field `PM` reader - Rx Tx priority ratio
pub type PM_R = crate::FieldReader<u8, u8>;
///Field `PM` writer - Rx Tx priority ratio
pub type PM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMABMR_SPEC, u8, u8, 2, O>;
///Field `FB` reader - Fixed burst
pub type FB_R = crate::BitReader<bool>;
///Field `FB` writer - Fixed burst
pub type FB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABMR_SPEC, bool, O>;
///Field `RDP` reader - Rx DMA PBL
pub type RDP_R = crate::FieldReader<u8, u8>;
///Field `RDP` writer - Rx DMA PBL
pub type RDP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMABMR_SPEC, u8, u8, 6, O>;
///Field `USP` reader - Use separate PBL
pub type USP_R = crate::BitReader<bool>;
///Field `USP` writer - Use separate PBL
pub type USP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABMR_SPEC, bool, O>;
///Field `FPM` reader - 4xPBL mode
pub type FPM_R = crate::BitReader<bool>;
///Field `FPM` writer - 4xPBL mode
pub type FPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABMR_SPEC, bool, O>;
///Field `AAB` reader - Address-aligned beats
pub type AAB_R = crate::BitReader<bool>;
///Field `AAB` writer - Address-aligned beats
pub type AAB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABMR_SPEC, bool, O>;
///Field `MB` reader - Mixed burst
pub type MB_R = crate::BitReader<bool>;
///Field `MB` writer - Mixed burst
pub type MB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMABMR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Software reset
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA Arbitration
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:6 - Descriptor skip length
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    ///Bit 7 - Enhanced descriptor format enable
    #[inline(always)]
    pub fn edfe(&self) -> EDFE_R {
        EDFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:13 - Programmable burst length
    #[inline(always)]
    pub fn pbl(&self) -> PBL_R {
        PBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 14:15 - Rx Tx priority ratio
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - Fixed burst
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:22 - Rx DMA PBL
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    ///Bit 23 - Use separate PBL
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - 4xPBL mode
    #[inline(always)]
    pub fn fpm(&self) -> FPM_R {
        FPM_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Address-aligned beats
    #[inline(always)]
    pub fn aab(&self) -> AAB_R {
        AAB_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Mixed burst
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Software reset
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SR_W<0> {
        SR_W::new(self)
    }
    ///Bit 1 - DMA Arbitration
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<1> {
        DA_W::new(self)
    }
    ///Bits 2:6 - Descriptor skip length
    #[inline(always)]
    #[must_use]
    pub fn dsl(&mut self) -> DSL_W<2> {
        DSL_W::new(self)
    }
    ///Bit 7 - Enhanced descriptor format enable
    #[inline(always)]
    #[must_use]
    pub fn edfe(&mut self) -> EDFE_W<7> {
        EDFE_W::new(self)
    }
    ///Bits 8:13 - Programmable burst length
    #[inline(always)]
    #[must_use]
    pub fn pbl(&mut self) -> PBL_W<8> {
        PBL_W::new(self)
    }
    ///Bits 14:15 - Rx Tx priority ratio
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<14> {
        PM_W::new(self)
    }
    ///Bit 16 - Fixed burst
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FB_W<16> {
        FB_W::new(self)
    }
    ///Bits 17:22 - Rx DMA PBL
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<17> {
        RDP_W::new(self)
    }
    ///Bit 23 - Use separate PBL
    #[inline(always)]
    #[must_use]
    pub fn usp(&mut self) -> USP_W<23> {
        USP_W::new(self)
    }
    ///Bit 24 - 4xPBL mode
    #[inline(always)]
    #[must_use]
    pub fn fpm(&mut self) -> FPM_W<24> {
        FPM_W::new(self)
    }
    ///Bit 25 - Address-aligned beats
    #[inline(always)]
    #[must_use]
    pub fn aab(&mut self) -> AAB_W<25> {
        AAB_W::new(self)
    }
    ///Bit 26 - Mixed burst
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
///`reset()` method sets DMABMR to value 0x0002_0101
impl crate::Resettable for DMABMR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0101;
}
