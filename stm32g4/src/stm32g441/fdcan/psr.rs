///Register `PSR` reader
pub struct R(crate::R<PSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PSR` writer
pub struct W(crate::W<PSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSR_SPEC>;
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
impl From<crate::W<PSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LEC` reader - LEC
pub type LEC_R = crate::FieldReader<u8, u8>;
///Field `LEC` writer - LEC
pub type LEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSR_SPEC, u8, u8, 3, O>;
///Field `ACT` writer - ACT
pub type ACT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSR_SPEC, u8, u8, 2, O>;
///Field `EP` reader - EP
pub type EP_R = crate::BitReader<bool>;
///Field `EP` writer - EP
pub type EP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
///Field `EW` reader - EW
pub type EW_R = crate::BitReader<bool>;
///Field `EW` writer - EW
pub type EW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
///Field `BO` reader - BO
pub type BO_R = crate::BitReader<bool>;
///Field `BO` writer - BO
pub type BO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
///Field `DLEC` writer - DLEC
pub type DLEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSR_SPEC, u8, u8, 3, O>;
///Field `RESI` reader - RESI
pub type RESI_R = crate::BitReader<bool>;
///Field `RESI` writer - RESI
pub type RESI_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
///Field `RBRS` reader - RBRS
pub type RBRS_R = crate::BitReader<bool>;
///Field `RBRS` writer - RBRS
pub type RBRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
///Field `REDL` reader - REDL
pub type REDL_R = crate::BitReader<bool>;
///Field `REDL` writer - REDL
pub type REDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
///Field `PXE` reader - PXE
pub type PXE_R = crate::BitReader<bool>;
///Field `PXE` writer - PXE
pub type PXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
///Field `TDCV` reader - TDCV
pub type TDCV_R = crate::FieldReader<u8, u8>;
///Field `TDCV` writer - TDCV
pub type TDCV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSR_SPEC, u8, u8, 7, O>;
impl R {
    ///Bits 0:2 - LEC
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 7) as u8)
    }
    ///Bit 5 - EP
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - EW
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - BO
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - RESI
    #[inline(always)]
    pub fn resi(&self) -> RESI_R {
        RESI_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - RBRS
    #[inline(always)]
    pub fn rbrs(&self) -> RBRS_R {
        RBRS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - REDL
    #[inline(always)]
    pub fn redl(&self) -> REDL_R {
        REDL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - PXE
    #[inline(always)]
    pub fn pxe(&self) -> PXE_R {
        PXE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bits 16:22 - TDCV
    #[inline(always)]
    pub fn tdcv(&self) -> TDCV_R {
        TDCV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:2 - LEC
    #[inline(always)]
    #[must_use]
    pub fn lec(&mut self) -> LEC_W<0> {
        LEC_W::new(self)
    }
    ///Bits 3:4 - ACT
    #[inline(always)]
    #[must_use]
    pub fn act(&mut self) -> ACT_W<3> {
        ACT_W::new(self)
    }
    ///Bit 5 - EP
    #[inline(always)]
    #[must_use]
    pub fn ep(&mut self) -> EP_W<5> {
        EP_W::new(self)
    }
    ///Bit 6 - EW
    #[inline(always)]
    #[must_use]
    pub fn ew(&mut self) -> EW_W<6> {
        EW_W::new(self)
    }
    ///Bit 7 - BO
    #[inline(always)]
    #[must_use]
    pub fn bo(&mut self) -> BO_W<7> {
        BO_W::new(self)
    }
    ///Bits 8:10 - DLEC
    #[inline(always)]
    #[must_use]
    pub fn dlec(&mut self) -> DLEC_W<8> {
        DLEC_W::new(self)
    }
    ///Bit 11 - RESI
    #[inline(always)]
    #[must_use]
    pub fn resi(&mut self) -> RESI_W<11> {
        RESI_W::new(self)
    }
    ///Bit 12 - RBRS
    #[inline(always)]
    #[must_use]
    pub fn rbrs(&mut self) -> RBRS_W<12> {
        RBRS_W::new(self)
    }
    ///Bit 13 - REDL
    #[inline(always)]
    #[must_use]
    pub fn redl(&mut self) -> REDL_W<13> {
        REDL_W::new(self)
    }
    ///Bit 14 - PXE
    #[inline(always)]
    #[must_use]
    pub fn pxe(&mut self) -> PXE_W<14> {
        PXE_W::new(self)
    }
    ///Bits 16:22 - TDCV
    #[inline(always)]
    #[must_use]
    pub fn tdcv(&mut self) -> TDCV_W<16> {
        TDCV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Protocol Status Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [psr](index.html) module
pub struct PSR_SPEC;
impl crate::RegisterSpec for PSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [psr::R](R) reader structure
impl crate::Readable for PSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [psr::W](W) writer structure
impl crate::Writable for PSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PSR to value 0x0707
impl crate::Resettable for PSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0707;
}
