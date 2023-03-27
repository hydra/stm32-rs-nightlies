///Register `SDMMC_MASKR` reader
pub struct R(crate::R<SDMMC_MASKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_MASKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_MASKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_MASKR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SDMMC_MASKR` writer
pub struct W(crate::W<SDMMC_MASKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_MASKR_SPEC>;
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
impl From<crate::W<SDMMC_MASKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_MASKR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CCRCFAILIE` reader - CCRCFAILIE
pub type CCRCFAILIE_R = crate::BitReader<bool>;
///Field `CCRCFAILIE` writer - CCRCFAILIE
pub type CCRCFAILIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_MASKR_SPEC, bool, O>;
///Field `DCRCFAILIE` reader - DCRCFAILIE
pub type DCRCFAILIE_R = crate::BitReader<bool>;
///Field `DCRCFAILIE` writer - DCRCFAILIE
pub type DCRCFAILIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_MASKR_SPEC, bool, O>;
///Field `CTIMEOUTIE` reader - CTIMEOUTIE
pub type CTIMEOUTIE_R = crate::BitReader<bool>;
///Field `CTIMEOUTIE` writer - CTIMEOUTIE
pub type CTIMEOUTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_MASKR_SPEC, bool, O>;
///Field `DTIMEOUTIE` reader - DTIMEOUTIE
pub type DTIMEOUTIE_R = crate::BitReader<bool>;
///Field `DTIMEOUTIE` writer - DTIMEOUTIE
pub type DTIMEOUTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_MASKR_SPEC, bool, O>;
///Field `TXUNDERRIE` reader - TXUNDERRIE
pub type TXUNDERRIE_R = crate::BitReader<bool>;
///Field `TXUNDERRIE` writer - TXUNDERRIE
pub type TXUNDERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_MASKR_SPEC, bool, O>;
///Field `RXOVERRIE` reader - RXOVERRIE
pub type RXOVERRIE_R = crate::BitReader<bool>;
///Field `RXOVERRIE` writer - RXOVERRIE
pub type RXOVERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_MASKR_SPEC, bool, O>;
///Field `CMDRENDIE` reader - CMDRENDIE
pub type CMDRENDIE_R = crate::BitReader<bool>;
///Field `CMDRENDIE` writer - CMDRENDIE
pub type CMDRENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_MASKR_SPEC, bool, O>;
///Field `CMDSENTIE` reader - CMDSENTIE
pub type CMDSENTIE_R = crate::BitReader<bool>;
///Field `CMDSENTIE` writer - CMDSENTIE
pub type CMDSENTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_MASKR_SPEC, bool, O>;
///Field `DATAENDIE` reader - DATAENDIE
pub type DATAENDIE_R = crate::BitReader<bool>;
///Field `DATAENDIE` writer - DATAENDIE
pub type DATAENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_MASKR_SPEC, bool, O>;
///Field `DHOLDIE` reader - DHOLDIE
pub type DHOLDIE_R = crate::BitReader<bool>;
///Field `DHOLDIE` writer - DHOLDIE
pub type DHOLDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_MASKR_SPEC, bool, O>;
///Field `DBCKENDIE` reader - DBCKENDIE
pub type DBCKENDIE_R = crate::BitReader<bool>;
///Field `DBCKENDIE` writer - DBCKENDIE
pub type DBCKENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_MASKR_SPEC, bool, O>;
///Field `DABORTIE` reader - DABORTIE
pub type DABORTIE_R = crate::BitReader<bool>;
///Field `DABORTIE` writer - DABORTIE
pub type DABORTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_MASKR_SPEC, bool, O>;
///Field `TXFIFOHEIE` reader - TXFIFOHEIE
pub type TXFIFOHEIE_R = crate::BitReader<bool>;
///Field `TXFIFOHEIE` writer - TXFIFOHEIE
pub type TXFIFOHEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_MASKR_SPEC, bool, O>;
///Field `RXFIFOHFIE` reader - RXFIFOHFIE
pub type RXFIFOHFIE_R = crate::BitReader<bool>;
///Field `RXFIFOHFIE` writer - RXFIFOHFIE
pub type RXFIFOHFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_MASKR_SPEC, bool, O>;
///Field `RXFIFOFIE` reader - RXFIFOFIE
pub type RXFIFOFIE_R = crate::BitReader<bool>;
///Field `RXFIFOFIE` writer - RXFIFOFIE
pub type RXFIFOFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_MASKR_SPEC, bool, O>;
///Field `TXFIFOEIE` reader - TXFIFOEIE
pub type TXFIFOEIE_R = crate::BitReader<bool>;
///Field `TXFIFOEIE` writer - TXFIFOEIE
pub type TXFIFOEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_MASKR_SPEC, bool, O>;
///Field `BUSYD0ENDIE` reader - BUSYD0ENDIE
pub type BUSYD0ENDIE_R = crate::BitReader<bool>;
///Field `BUSYD0ENDIE` writer - BUSYD0ENDIE
pub type BUSYD0ENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_MASKR_SPEC, bool, O>;
///Field `SDIOITIE` reader - SDIOITIE
pub type SDIOITIE_R = crate::BitReader<bool>;
///Field `SDIOITIE` writer - SDIOITIE
pub type SDIOITIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_MASKR_SPEC, bool, O>;
///Field `ACKFAILIE` reader - ACKFAILIE
pub type ACKFAILIE_R = crate::BitReader<bool>;
///Field `ACKFAILIE` writer - ACKFAILIE
pub type ACKFAILIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_MASKR_SPEC, bool, O>;
///Field `ACKTIMEOUTIE` reader - ACKTIMEOUTIE
pub type ACKTIMEOUTIE_R = crate::BitReader<bool>;
///Field `ACKTIMEOUTIE` writer - ACKTIMEOUTIE
pub type ACKTIMEOUTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_MASKR_SPEC, bool, O>;
///Field `VSWENDIE` reader - VSWENDIE
pub type VSWENDIE_R = crate::BitReader<bool>;
///Field `VSWENDIE` writer - VSWENDIE
pub type VSWENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_MASKR_SPEC, bool, O>;
///Field `CKSTOPIE` reader - CKSTOPIE
pub type CKSTOPIE_R = crate::BitReader<bool>;
///Field `CKSTOPIE` writer - CKSTOPIE
pub type CKSTOPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_MASKR_SPEC, bool, O>;
///Field `IDMABTCIE` reader - IDMABTCIE
pub type IDMABTCIE_R = crate::BitReader<bool>;
///Field `IDMABTCIE` writer - IDMABTCIE
pub type IDMABTCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_MASKR_SPEC, bool, O>;
impl R {
    ///Bit 0 - CCRCFAILIE
    #[inline(always)]
    pub fn ccrcfailie(&self) -> CCRCFAILIE_R {
        CCRCFAILIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DCRCFAILIE
    #[inline(always)]
    pub fn dcrcfailie(&self) -> DCRCFAILIE_R {
        DCRCFAILIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CTIMEOUTIE
    #[inline(always)]
    pub fn ctimeoutie(&self) -> CTIMEOUTIE_R {
        CTIMEOUTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DTIMEOUTIE
    #[inline(always)]
    pub fn dtimeoutie(&self) -> DTIMEOUTIE_R {
        DTIMEOUTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TXUNDERRIE
    #[inline(always)]
    pub fn txunderrie(&self) -> TXUNDERRIE_R {
        TXUNDERRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXOVERRIE
    #[inline(always)]
    pub fn rxoverrie(&self) -> RXOVERRIE_R {
        RXOVERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CMDRENDIE
    #[inline(always)]
    pub fn cmdrendie(&self) -> CMDRENDIE_R {
        CMDRENDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CMDSENTIE
    #[inline(always)]
    pub fn cmdsentie(&self) -> CMDSENTIE_R {
        CMDSENTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - DATAENDIE
    #[inline(always)]
    pub fn dataendie(&self) -> DATAENDIE_R {
        DATAENDIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DHOLDIE
    #[inline(always)]
    pub fn dholdie(&self) -> DHOLDIE_R {
        DHOLDIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DBCKENDIE
    #[inline(always)]
    pub fn dbckendie(&self) -> DBCKENDIE_R {
        DBCKENDIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DABORTIE
    #[inline(always)]
    pub fn dabortie(&self) -> DABORTIE_R {
        DABORTIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - TXFIFOHEIE
    #[inline(always)]
    pub fn txfifoheie(&self) -> TXFIFOHEIE_R {
        TXFIFOHEIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - RXFIFOHFIE
    #[inline(always)]
    pub fn rxfifohfie(&self) -> RXFIFOHFIE_R {
        RXFIFOHFIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - RXFIFOFIE
    #[inline(always)]
    pub fn rxfifofie(&self) -> RXFIFOFIE_R {
        RXFIFOFIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TXFIFOEIE
    #[inline(always)]
    pub fn txfifoeie(&self) -> TXFIFOEIE_R {
        TXFIFOEIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 21 - BUSYD0ENDIE
    #[inline(always)]
    pub fn busyd0endie(&self) -> BUSYD0ENDIE_R {
        BUSYD0ENDIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SDIOITIE
    #[inline(always)]
    pub fn sdioitie(&self) -> SDIOITIE_R {
        SDIOITIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ACKFAILIE
    #[inline(always)]
    pub fn ackfailie(&self) -> ACKFAILIE_R {
        ACKFAILIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - ACKTIMEOUTIE
    #[inline(always)]
    pub fn acktimeoutie(&self) -> ACKTIMEOUTIE_R {
        ACKTIMEOUTIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - VSWENDIE
    #[inline(always)]
    pub fn vswendie(&self) -> VSWENDIE_R {
        VSWENDIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - CKSTOPIE
    #[inline(always)]
    pub fn ckstopie(&self) -> CKSTOPIE_R {
        CKSTOPIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28 - IDMABTCIE
    #[inline(always)]
    pub fn idmabtcie(&self) -> IDMABTCIE_R {
        IDMABTCIE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CCRCFAILIE
    #[inline(always)]
    #[must_use]
    pub fn ccrcfailie(&mut self) -> CCRCFAILIE_W<0> {
        CCRCFAILIE_W::new(self)
    }
    ///Bit 1 - DCRCFAILIE
    #[inline(always)]
    #[must_use]
    pub fn dcrcfailie(&mut self) -> DCRCFAILIE_W<1> {
        DCRCFAILIE_W::new(self)
    }
    ///Bit 2 - CTIMEOUTIE
    #[inline(always)]
    #[must_use]
    pub fn ctimeoutie(&mut self) -> CTIMEOUTIE_W<2> {
        CTIMEOUTIE_W::new(self)
    }
    ///Bit 3 - DTIMEOUTIE
    #[inline(always)]
    #[must_use]
    pub fn dtimeoutie(&mut self) -> DTIMEOUTIE_W<3> {
        DTIMEOUTIE_W::new(self)
    }
    ///Bit 4 - TXUNDERRIE
    #[inline(always)]
    #[must_use]
    pub fn txunderrie(&mut self) -> TXUNDERRIE_W<4> {
        TXUNDERRIE_W::new(self)
    }
    ///Bit 5 - RXOVERRIE
    #[inline(always)]
    #[must_use]
    pub fn rxoverrie(&mut self) -> RXOVERRIE_W<5> {
        RXOVERRIE_W::new(self)
    }
    ///Bit 6 - CMDRENDIE
    #[inline(always)]
    #[must_use]
    pub fn cmdrendie(&mut self) -> CMDRENDIE_W<6> {
        CMDRENDIE_W::new(self)
    }
    ///Bit 7 - CMDSENTIE
    #[inline(always)]
    #[must_use]
    pub fn cmdsentie(&mut self) -> CMDSENTIE_W<7> {
        CMDSENTIE_W::new(self)
    }
    ///Bit 8 - DATAENDIE
    #[inline(always)]
    #[must_use]
    pub fn dataendie(&mut self) -> DATAENDIE_W<8> {
        DATAENDIE_W::new(self)
    }
    ///Bit 9 - DHOLDIE
    #[inline(always)]
    #[must_use]
    pub fn dholdie(&mut self) -> DHOLDIE_W<9> {
        DHOLDIE_W::new(self)
    }
    ///Bit 10 - DBCKENDIE
    #[inline(always)]
    #[must_use]
    pub fn dbckendie(&mut self) -> DBCKENDIE_W<10> {
        DBCKENDIE_W::new(self)
    }
    ///Bit 11 - DABORTIE
    #[inline(always)]
    #[must_use]
    pub fn dabortie(&mut self) -> DABORTIE_W<11> {
        DABORTIE_W::new(self)
    }
    ///Bit 14 - TXFIFOHEIE
    #[inline(always)]
    #[must_use]
    pub fn txfifoheie(&mut self) -> TXFIFOHEIE_W<14> {
        TXFIFOHEIE_W::new(self)
    }
    ///Bit 15 - RXFIFOHFIE
    #[inline(always)]
    #[must_use]
    pub fn rxfifohfie(&mut self) -> RXFIFOHFIE_W<15> {
        RXFIFOHFIE_W::new(self)
    }
    ///Bit 17 - RXFIFOFIE
    #[inline(always)]
    #[must_use]
    pub fn rxfifofie(&mut self) -> RXFIFOFIE_W<17> {
        RXFIFOFIE_W::new(self)
    }
    ///Bit 18 - TXFIFOEIE
    #[inline(always)]
    #[must_use]
    pub fn txfifoeie(&mut self) -> TXFIFOEIE_W<18> {
        TXFIFOEIE_W::new(self)
    }
    ///Bit 21 - BUSYD0ENDIE
    #[inline(always)]
    #[must_use]
    pub fn busyd0endie(&mut self) -> BUSYD0ENDIE_W<21> {
        BUSYD0ENDIE_W::new(self)
    }
    ///Bit 22 - SDIOITIE
    #[inline(always)]
    #[must_use]
    pub fn sdioitie(&mut self) -> SDIOITIE_W<22> {
        SDIOITIE_W::new(self)
    }
    ///Bit 23 - ACKFAILIE
    #[inline(always)]
    #[must_use]
    pub fn ackfailie(&mut self) -> ACKFAILIE_W<23> {
        ACKFAILIE_W::new(self)
    }
    ///Bit 24 - ACKTIMEOUTIE
    #[inline(always)]
    #[must_use]
    pub fn acktimeoutie(&mut self) -> ACKTIMEOUTIE_W<24> {
        ACKTIMEOUTIE_W::new(self)
    }
    ///Bit 25 - VSWENDIE
    #[inline(always)]
    #[must_use]
    pub fn vswendie(&mut self) -> VSWENDIE_W<25> {
        VSWENDIE_W::new(self)
    }
    ///Bit 26 - CKSTOPIE
    #[inline(always)]
    #[must_use]
    pub fn ckstopie(&mut self) -> CKSTOPIE_W<26> {
        CKSTOPIE_W::new(self)
    }
    ///Bit 28 - IDMABTCIE
    #[inline(always)]
    #[must_use]
    pub fn idmabtcie(&mut self) -> IDMABTCIE_W<28> {
        IDMABTCIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sdmmc_maskr](index.html) module
pub struct SDMMC_MASKR_SPEC;
impl crate::RegisterSpec for SDMMC_MASKR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sdmmc_maskr::R](R) reader structure
impl crate::Readable for SDMMC_MASKR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sdmmc_maskr::W](W) writer structure
impl crate::Writable for SDMMC_MASKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SDMMC_MASKR to value 0
impl crate::Resettable for SDMMC_MASKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
