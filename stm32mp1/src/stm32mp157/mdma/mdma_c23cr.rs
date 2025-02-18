///Register `MDMA_C23CR` reader
pub struct R(crate::R<MDMA_C23CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDMA_C23CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDMA_C23CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDMA_C23CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MDMA_C23CR` writer
pub struct W(crate::W<MDMA_C23CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDMA_C23CR_SPEC>;
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
impl From<crate::W<MDMA_C23CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDMA_C23CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EN` reader - EN
pub type EN_R = crate::BitReader<bool>;
///Field `EN` writer - EN
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMA_C23CR_SPEC, bool, O>;
///Field `TEIE` reader - TEIE
pub type TEIE_R = crate::BitReader<bool>;
///Field `TEIE` writer - TEIE
pub type TEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMA_C23CR_SPEC, bool, O>;
///Field `CTCIE` reader - CTCIE
pub type CTCIE_R = crate::BitReader<bool>;
///Field `CTCIE` writer - CTCIE
pub type CTCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMA_C23CR_SPEC, bool, O>;
///Field `BRTIE` reader - BRTIE
pub type BRTIE_R = crate::BitReader<bool>;
///Field `BRTIE` writer - BRTIE
pub type BRTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMA_C23CR_SPEC, bool, O>;
///Field `BTIE` reader - BTIE
pub type BTIE_R = crate::BitReader<bool>;
///Field `BTIE` writer - BTIE
pub type BTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMA_C23CR_SPEC, bool, O>;
///Field `TCIE` reader - TCIE
pub type TCIE_R = crate::BitReader<bool>;
///Field `TCIE` writer - TCIE
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMA_C23CR_SPEC, bool, O>;
///Field `PL` reader - PL
pub type PL_R = crate::FieldReader<u8, u8>;
///Field `PL` writer - PL
pub type PL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDMA_C23CR_SPEC, u8, u8, 2, O>;
///Field `BEX` reader - BEX
pub type BEX_R = crate::BitReader<bool>;
///Field `BEX` writer - BEX
pub type BEX_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMA_C23CR_SPEC, bool, O>;
///Field `HEX` reader - HEX
pub type HEX_R = crate::BitReader<bool>;
///Field `HEX` writer - HEX
pub type HEX_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMA_C23CR_SPEC, bool, O>;
///Field `WEX` reader - WEX
pub type WEX_R = crate::BitReader<bool>;
///Field `WEX` writer - WEX
pub type WEX_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMA_C23CR_SPEC, bool, O>;
///Field `SWRQ` writer - SWRQ
pub type SWRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMA_C23CR_SPEC, bool, O>;
impl R {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TEIE
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CTCIE
    #[inline(always)]
    pub fn ctcie(&self) -> CTCIE_R {
        CTCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - BRTIE
    #[inline(always)]
    pub fn brtie(&self) -> BRTIE_R {
        BRTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - BTIE
    #[inline(always)]
    pub fn btie(&self) -> BTIE_R {
        BTIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TCIE
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - PL
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 12 - BEX
    #[inline(always)]
    pub fn bex(&self) -> BEX_R {
        BEX_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - HEX
    #[inline(always)]
    pub fn hex(&self) -> HEX_R {
        HEX_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - WEX
    #[inline(always)]
    pub fn wex(&self) -> WEX_R {
        WEX_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - EN
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    ///Bit 1 - TEIE
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<1> {
        TEIE_W::new(self)
    }
    ///Bit 2 - CTCIE
    #[inline(always)]
    #[must_use]
    pub fn ctcie(&mut self) -> CTCIE_W<2> {
        CTCIE_W::new(self)
    }
    ///Bit 3 - BRTIE
    #[inline(always)]
    #[must_use]
    pub fn brtie(&mut self) -> BRTIE_W<3> {
        BRTIE_W::new(self)
    }
    ///Bit 4 - BTIE
    #[inline(always)]
    #[must_use]
    pub fn btie(&mut self) -> BTIE_W<4> {
        BTIE_W::new(self)
    }
    ///Bit 5 - TCIE
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<5> {
        TCIE_W::new(self)
    }
    ///Bits 6:7 - PL
    #[inline(always)]
    #[must_use]
    pub fn pl(&mut self) -> PL_W<6> {
        PL_W::new(self)
    }
    ///Bit 12 - BEX
    #[inline(always)]
    #[must_use]
    pub fn bex(&mut self) -> BEX_W<12> {
        BEX_W::new(self)
    }
    ///Bit 13 - HEX
    #[inline(always)]
    #[must_use]
    pub fn hex(&mut self) -> HEX_W<13> {
        HEX_W::new(self)
    }
    ///Bit 14 - WEX
    #[inline(always)]
    #[must_use]
    pub fn wex(&mut self) -> WEX_W<14> {
        WEX_W::new(self)
    }
    ///Bit 16 - SWRQ
    #[inline(always)]
    #[must_use]
    pub fn swrq(&mut self) -> SWRQ_W<16> {
        SWRQ_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to control the concerned channel.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdma_c23cr](index.html) module
pub struct MDMA_C23CR_SPEC;
impl crate::RegisterSpec for MDMA_C23CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdma_c23cr::R](R) reader structure
impl crate::Readable for MDMA_C23CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mdma_c23cr::W](W) writer structure
impl crate::Writable for MDMA_C23CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MDMA_C23CR to value 0
impl crate::Resettable for MDMA_C23CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
