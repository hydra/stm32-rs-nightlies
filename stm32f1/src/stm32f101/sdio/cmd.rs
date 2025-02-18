///Register `CMD` reader
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CMD` writer
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CMDINDEX` reader - CMDINDEX
pub type CMDINDEX_R = crate::FieldReader<u8, u8>;
///Field `CMDINDEX` writer - CMDINDEX
pub type CMDINDEX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD_SPEC, u8, u8, 6, O>;
///Field `WAITRESP` reader - WAITRESP
pub type WAITRESP_R = crate::FieldReader<u8, u8>;
///Field `WAITRESP` writer - WAITRESP
pub type WAITRESP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD_SPEC, u8, u8, 2, O>;
///Field `WAITINT` reader - WAITINT
pub type WAITINT_R = crate::BitReader<bool>;
///Field `WAITINT` writer - WAITINT
pub type WAITINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
///Field `WAITPEND` reader - WAITPEND
pub type WAITPEND_R = crate::BitReader<bool>;
///Field `WAITPEND` writer - WAITPEND
pub type WAITPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
///Field `CPSMEN` reader - CPSMEN
pub type CPSMEN_R = crate::BitReader<bool>;
///Field `CPSMEN` writer - CPSMEN
pub type CPSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
///Field `SDIOSuspend` reader - SDIOSuspend
pub type SDIOSUSPEND_R = crate::BitReader<bool>;
///Field `SDIOSuspend` writer - SDIOSuspend
pub type SDIOSUSPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
///Field `ENCMDcompl` reader - ENCMDcompl
pub type ENCMDCOMPL_R = crate::BitReader<bool>;
///Field `ENCMDcompl` writer - ENCMDcompl
pub type ENCMDCOMPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
///Field `nIEN` reader - nIEN
pub type N_IEN_R = crate::BitReader<bool>;
///Field `nIEN` writer - nIEN
pub type N_IEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
///Field `CE_ATACMD` reader - CE_ATACMD
pub type CE_ATACMD_R = crate::BitReader<bool>;
///Field `CE_ATACMD` writer - CE_ATACMD
pub type CE_ATACMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
impl R {
    ///Bits 0:5 - CMDINDEX
    #[inline(always)]
    pub fn cmdindex(&self) -> CMDINDEX_R {
        CMDINDEX_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:7 - WAITRESP
    #[inline(always)]
    pub fn waitresp(&self) -> WAITRESP_R {
        WAITRESP_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - WAITINT
    #[inline(always)]
    pub fn waitint(&self) -> WAITINT_R {
        WAITINT_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - WAITPEND
    #[inline(always)]
    pub fn waitpend(&self) -> WAITPEND_R {
        WAITPEND_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CPSMEN
    #[inline(always)]
    pub fn cpsmen(&self) -> CPSMEN_R {
        CPSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SDIOSuspend
    #[inline(always)]
    pub fn sdiosuspend(&self) -> SDIOSUSPEND_R {
        SDIOSUSPEND_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - ENCMDcompl
    #[inline(always)]
    pub fn encmdcompl(&self) -> ENCMDCOMPL_R {
        ENCMDCOMPL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - nIEN
    #[inline(always)]
    pub fn n_ien(&self) -> N_IEN_R {
        N_IEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CE_ATACMD
    #[inline(always)]
    pub fn ce_atacmd(&self) -> CE_ATACMD_R {
        CE_ATACMD_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bits 0:5 - CMDINDEX
    #[inline(always)]
    #[must_use]
    pub fn cmdindex(&mut self) -> CMDINDEX_W<0> {
        CMDINDEX_W::new(self)
    }
    ///Bits 6:7 - WAITRESP
    #[inline(always)]
    #[must_use]
    pub fn waitresp(&mut self) -> WAITRESP_W<6> {
        WAITRESP_W::new(self)
    }
    ///Bit 8 - WAITINT
    #[inline(always)]
    #[must_use]
    pub fn waitint(&mut self) -> WAITINT_W<8> {
        WAITINT_W::new(self)
    }
    ///Bit 9 - WAITPEND
    #[inline(always)]
    #[must_use]
    pub fn waitpend(&mut self) -> WAITPEND_W<9> {
        WAITPEND_W::new(self)
    }
    ///Bit 10 - CPSMEN
    #[inline(always)]
    #[must_use]
    pub fn cpsmen(&mut self) -> CPSMEN_W<10> {
        CPSMEN_W::new(self)
    }
    ///Bit 11 - SDIOSuspend
    #[inline(always)]
    #[must_use]
    pub fn sdiosuspend(&mut self) -> SDIOSUSPEND_W<11> {
        SDIOSUSPEND_W::new(self)
    }
    ///Bit 12 - ENCMDcompl
    #[inline(always)]
    #[must_use]
    pub fn encmdcompl(&mut self) -> ENCMDCOMPL_W<12> {
        ENCMDCOMPL_W::new(self)
    }
    ///Bit 13 - nIEN
    #[inline(always)]
    #[must_use]
    pub fn n_ien(&mut self) -> N_IEN_W<13> {
        N_IEN_W::new(self)
    }
    ///Bit 14 - CE_ATACMD
    #[inline(always)]
    #[must_use]
    pub fn ce_atacmd(&mut self) -> CE_ATACMD_W<14> {
        CE_ATACMD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SDIO command register (SDIO_CMD)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmd](index.html) module
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
///`read()` method returns [cmd::R](R) reader structure
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cmd::W](W) writer structure
impl crate::Writable for CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CMD to value 0
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
