///Register `MMCCR` reader
pub struct R(crate::R<MMCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MMCCR` writer
pub struct W(crate::W<MMCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMCCR_SPEC>;
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
impl From<crate::W<MMCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMCCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CR` reader - CR
pub type CR_R = crate::BitReader<bool>;
///Field `CR` writer - CR
pub type CR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCCR_SPEC, bool, O>;
///Field `CSR` reader - CSR
pub type CSR_R = crate::BitReader<bool>;
///Field `CSR` writer - CSR
pub type CSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCCR_SPEC, bool, O>;
///Field `ROR` reader - ROR
pub type ROR_R = crate::BitReader<bool>;
///Field `ROR` writer - ROR
pub type ROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCCR_SPEC, bool, O>;
///Field `MCF` reader - MCF
pub type MCF_R = crate::BitReader<bool>;
///Field `MCF` writer - MCF
pub type MCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCCR_SPEC, bool, O>;
///Field `MCP` reader - MCP
pub type MCP_R = crate::BitReader<bool>;
///Field `MCP` writer - MCP
pub type MCP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCCR_SPEC, bool, O>;
///Field `MCFHP` reader - MCFHP
pub type MCFHP_R = crate::BitReader<bool>;
///Field `MCFHP` writer - MCFHP
pub type MCFHP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - CR
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CSR
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ROR
    #[inline(always)]
    pub fn ror(&self) -> ROR_R {
        ROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MCF
    #[inline(always)]
    pub fn mcf(&self) -> MCF_R {
        MCF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MCP
    #[inline(always)]
    pub fn mcp(&self) -> MCP_R {
        MCP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MCFHP
    #[inline(always)]
    pub fn mcfhp(&self) -> MCFHP_R {
        MCFHP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CR
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<0> {
        CR_W::new(self)
    }
    ///Bit 1 - CSR
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CSR_W<1> {
        CSR_W::new(self)
    }
    ///Bit 2 - ROR
    #[inline(always)]
    #[must_use]
    pub fn ror(&mut self) -> ROR_W<2> {
        ROR_W::new(self)
    }
    ///Bit 3 - MCF
    #[inline(always)]
    #[must_use]
    pub fn mcf(&mut self) -> MCF_W<3> {
        MCF_W::new(self)
    }
    ///Bit 4 - MCP
    #[inline(always)]
    #[must_use]
    pub fn mcp(&mut self) -> MCP_W<4> {
        MCP_W::new(self)
    }
    ///Bit 5 - MCFHP
    #[inline(always)]
    #[must_use]
    pub fn mcfhp(&mut self) -> MCFHP_W<5> {
        MCFHP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet MMC control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mmccr](index.html) module
pub struct MMCCR_SPEC;
impl crate::RegisterSpec for MMCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mmccr::R](R) reader structure
impl crate::Readable for MMCCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mmccr::W](W) writer structure
impl crate::Writable for MMCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MMCCR to value 0
impl crate::Resettable for MMCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
