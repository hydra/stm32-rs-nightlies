///Register `FDCAN_CCCR` reader
pub struct R(crate::R<FDCAN_CCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_CCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_CCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_CCCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FDCAN_CCCR` writer
pub struct W(crate::W<FDCAN_CCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_CCCR_SPEC>;
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
impl From<crate::W<FDCAN_CCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_CCCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `INIT` reader - INIT
pub type INIT_R = crate::BitReader<bool>;
///Field `INIT` writer - INIT
pub type INIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_CCCR_SPEC, bool, O>;
///Field `CCE` reader - CCE
pub type CCE_R = crate::BitReader<bool>;
///Field `CCE` writer - CCE
pub type CCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_CCCR_SPEC, bool, O>;
///Field `ASM` reader - ASM
pub type ASM_R = crate::BitReader<bool>;
///Field `ASM` writer - ASM
pub type ASM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_CCCR_SPEC, bool, O>;
///Field `CSA` reader - CSA
pub type CSA_R = crate::BitReader<bool>;
///Field `CSR` reader - CSR
pub type CSR_R = crate::BitReader<bool>;
///Field `CSR` writer - CSR
pub type CSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_CCCR_SPEC, bool, O>;
///Field `MON` reader - MON
pub type MON_R = crate::BitReader<bool>;
///Field `MON` writer - MON
pub type MON_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_CCCR_SPEC, bool, O>;
///Field `DAR` reader - DAR
pub type DAR_R = crate::BitReader<bool>;
///Field `DAR` writer - DAR
pub type DAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_CCCR_SPEC, bool, O>;
///Field `TEST` reader - TEST
pub type TEST_R = crate::BitReader<bool>;
///Field `TEST` writer - TEST
pub type TEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_CCCR_SPEC, bool, O>;
///Field `FDOE` reader - FDOE
pub type FDOE_R = crate::BitReader<bool>;
///Field `FDOE` writer - FDOE
pub type FDOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_CCCR_SPEC, bool, O>;
///Field `BRSE` reader - BRSE
pub type BRSE_R = crate::BitReader<bool>;
///Field `BRSE` writer - BRSE
pub type BRSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_CCCR_SPEC, bool, O>;
///Field `PXHD` reader - PXHD
pub type PXHD_R = crate::BitReader<bool>;
///Field `PXHD` writer - PXHD
pub type PXHD_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_CCCR_SPEC, bool, O>;
///Field `EFBI` reader - EFBI
pub type EFBI_R = crate::BitReader<bool>;
///Field `EFBI` writer - EFBI
pub type EFBI_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_CCCR_SPEC, bool, O>;
///Field `TXP` reader - TXP
pub type TXP_R = crate::BitReader<bool>;
///Field `TXP` writer - TXP
pub type TXP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_CCCR_SPEC, bool, O>;
///Field `NISO` reader - NISO
pub type NISO_R = crate::BitReader<bool>;
///Field `NISO` writer - NISO
pub type NISO_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_CCCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - INIT
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CCE
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ASM
    #[inline(always)]
    pub fn asm(&self) -> ASM_R {
        ASM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CSA
    #[inline(always)]
    pub fn csa(&self) -> CSA_R {
        CSA_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CSR
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MON
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DAR
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TEST
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - FDOE
    #[inline(always)]
    pub fn fdoe(&self) -> FDOE_R {
        FDOE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - BRSE
    #[inline(always)]
    pub fn brse(&self) -> BRSE_R {
        BRSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 12 - PXHD
    #[inline(always)]
    pub fn pxhd(&self) -> PXHD_R {
        PXHD_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - EFBI
    #[inline(always)]
    pub fn efbi(&self) -> EFBI_R {
        EFBI_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - TXP
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - NISO
    #[inline(always)]
    pub fn niso(&self) -> NISO_R {
        NISO_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - INIT
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<0> {
        INIT_W::new(self)
    }
    ///Bit 1 - CCE
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self) -> CCE_W<1> {
        CCE_W::new(self)
    }
    ///Bit 2 - ASM
    #[inline(always)]
    #[must_use]
    pub fn asm(&mut self) -> ASM_W<2> {
        ASM_W::new(self)
    }
    ///Bit 4 - CSR
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CSR_W<4> {
        CSR_W::new(self)
    }
    ///Bit 5 - MON
    #[inline(always)]
    #[must_use]
    pub fn mon(&mut self) -> MON_W<5> {
        MON_W::new(self)
    }
    ///Bit 6 - DAR
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DAR_W<6> {
        DAR_W::new(self)
    }
    ///Bit 7 - TEST
    #[inline(always)]
    #[must_use]
    pub fn test(&mut self) -> TEST_W<7> {
        TEST_W::new(self)
    }
    ///Bit 8 - FDOE
    #[inline(always)]
    #[must_use]
    pub fn fdoe(&mut self) -> FDOE_W<8> {
        FDOE_W::new(self)
    }
    ///Bit 9 - BRSE
    #[inline(always)]
    #[must_use]
    pub fn brse(&mut self) -> BRSE_W<9> {
        BRSE_W::new(self)
    }
    ///Bit 12 - PXHD
    #[inline(always)]
    #[must_use]
    pub fn pxhd(&mut self) -> PXHD_W<12> {
        PXHD_W::new(self)
    }
    ///Bit 13 - EFBI
    #[inline(always)]
    #[must_use]
    pub fn efbi(&mut self) -> EFBI_W<13> {
        EFBI_W::new(self)
    }
    ///Bit 14 - TXP
    #[inline(always)]
    #[must_use]
    pub fn txp(&mut self) -> TXP_W<14> {
        TXP_W::new(self)
    }
    ///Bit 15 - NISO
    #[inline(always)]
    #[must_use]
    pub fn niso(&mut self) -> NISO_W<15> {
        NISO_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///For details about setting and resetting of single bits see Software initialization.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_cccr](index.html) module
pub struct FDCAN_CCCR_SPEC;
impl crate::RegisterSpec for FDCAN_CCCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_cccr::R](R) reader structure
impl crate::Readable for FDCAN_CCCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fdcan_cccr::W](W) writer structure
impl crate::Writable for FDCAN_CCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FDCAN_CCCR to value 0x01
impl crate::Resettable for FDCAN_CCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
