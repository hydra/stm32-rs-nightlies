///Register `ICR` reader
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ICR` writer
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CCRCFAILC` reader - CCRCFAIL flag clear bit Set by software to clear the CCRCFAIL flag.
pub type CCRCFAILC_R = crate::BitReader<bool>;
///Field `CCRCFAILC` writer - CCRCFAIL flag clear bit Set by software to clear the CCRCFAIL flag.
pub type CCRCFAILC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `DCRCFAILC` reader - DCRCFAIL flag clear bit Set by software to clear the DCRCFAIL flag.
pub type DCRCFAILC_R = crate::BitReader<bool>;
///Field `DCRCFAILC` writer - DCRCFAIL flag clear bit Set by software to clear the DCRCFAIL flag.
pub type DCRCFAILC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `CTIMEOUTC` reader - CTIMEOUT flag clear bit Set by software to clear the CTIMEOUT flag.
pub type CTIMEOUTC_R = crate::BitReader<bool>;
///Field `CTIMEOUTC` writer - CTIMEOUT flag clear bit Set by software to clear the CTIMEOUT flag.
pub type CTIMEOUTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `DTIMEOUTC` reader - DTIMEOUT flag clear bit Set by software to clear the DTIMEOUT flag.
pub type DTIMEOUTC_R = crate::BitReader<bool>;
///Field `DTIMEOUTC` writer - DTIMEOUT flag clear bit Set by software to clear the DTIMEOUT flag.
pub type DTIMEOUTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `TXUNDERRC` reader - TXUNDERR flag clear bit Set by software to clear TXUNDERR flag.
pub type TXUNDERRC_R = crate::BitReader<bool>;
///Field `TXUNDERRC` writer - TXUNDERR flag clear bit Set by software to clear TXUNDERR flag.
pub type TXUNDERRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `RXOVERRC` reader - RXOVERR flag clear bit Set by software to clear the RXOVERR flag.
pub type RXOVERRC_R = crate::BitReader<bool>;
///Field `RXOVERRC` writer - RXOVERR flag clear bit Set by software to clear the RXOVERR flag.
pub type RXOVERRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `CMDRENDC` reader - CMDREND flag clear bit Set by software to clear the CMDREND flag.
pub type CMDRENDC_R = crate::BitReader<bool>;
///Field `CMDRENDC` writer - CMDREND flag clear bit Set by software to clear the CMDREND flag.
pub type CMDRENDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `CMDSENTC` reader - CMDSENT flag clear bit Set by software to clear the CMDSENT flag.
pub type CMDSENTC_R = crate::BitReader<bool>;
///Field `CMDSENTC` writer - CMDSENT flag clear bit Set by software to clear the CMDSENT flag.
pub type CMDSENTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `DATAENDC` reader - DATAEND flag clear bit Set by software to clear the DATAEND flag.
pub type DATAENDC_R = crate::BitReader<bool>;
///Field `DATAENDC` writer - DATAEND flag clear bit Set by software to clear the DATAEND flag.
pub type DATAENDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `DHOLDC` reader - DHOLD flag clear bit Set by software to clear the DHOLD flag.
pub type DHOLDC_R = crate::BitReader<bool>;
///Field `DHOLDC` writer - DHOLD flag clear bit Set by software to clear the DHOLD flag.
pub type DHOLDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `DBCKENDC` reader - DBCKEND flag clear bit Set by software to clear the DBCKEND flag.
pub type DBCKENDC_R = crate::BitReader<bool>;
///Field `DBCKENDC` writer - DBCKEND flag clear bit Set by software to clear the DBCKEND flag.
pub type DBCKENDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `DABORTC` reader - DABORT flag clear bit Set by software to clear the DABORT flag.
pub type DABORTC_R = crate::BitReader<bool>;
///Field `DABORTC` writer - DABORT flag clear bit Set by software to clear the DABORT flag.
pub type DABORTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `BUSYD0ENDC` reader - BUSYD0END flag clear bit Set by software to clear the BUSYD0END flag.
pub type BUSYD0ENDC_R = crate::BitReader<bool>;
///Field `BUSYD0ENDC` writer - BUSYD0END flag clear bit Set by software to clear the BUSYD0END flag.
pub type BUSYD0ENDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `SDIOITC` reader - SDIOIT flag clear bit Set by software to clear the SDIOIT flag.
pub type SDIOITC_R = crate::BitReader<bool>;
///Field `SDIOITC` writer - SDIOIT flag clear bit Set by software to clear the SDIOIT flag.
pub type SDIOITC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `ACKFAILC` reader - ACKFAIL flag clear bit Set by software to clear the ACKFAIL flag.
pub type ACKFAILC_R = crate::BitReader<bool>;
///Field `ACKFAILC` writer - ACKFAIL flag clear bit Set by software to clear the ACKFAIL flag.
pub type ACKFAILC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `ACKTIMEOUTC` reader - ACKTIMEOUT flag clear bit Set by software to clear the ACKTIMEOUT flag.
pub type ACKTIMEOUTC_R = crate::BitReader<bool>;
///Field `ACKTIMEOUTC` writer - ACKTIMEOUT flag clear bit Set by software to clear the ACKTIMEOUT flag.
pub type ACKTIMEOUTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `VSWENDC` reader - VSWEND flag clear bit Set by software to clear the VSWEND flag.
pub type VSWENDC_R = crate::BitReader<bool>;
///Field `VSWENDC` writer - VSWEND flag clear bit Set by software to clear the VSWEND flag.
pub type VSWENDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `CKSTOPC` reader - CKSTOP flag clear bit Set by software to clear the CKSTOP flag.
pub type CKSTOPC_R = crate::BitReader<bool>;
///Field `CKSTOPC` writer - CKSTOP flag clear bit Set by software to clear the CKSTOP flag.
pub type CKSTOPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `IDMATEC` reader - IDMA transfer error clear bit Set by software to clear the IDMATE flag.
pub type IDMATEC_R = crate::BitReader<bool>;
///Field `IDMATEC` writer - IDMA transfer error clear bit Set by software to clear the IDMATE flag.
pub type IDMATEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `IDMABTCC` reader - IDMA buffer transfer complete clear bit Set by software to clear the IDMABTC flag.
pub type IDMABTCC_R = crate::BitReader<bool>;
///Field `IDMABTCC` writer - IDMA buffer transfer complete clear bit Set by software to clear the IDMABTC flag.
pub type IDMABTCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
impl R {
    ///Bit 0 - CCRCFAIL flag clear bit Set by software to clear the CCRCFAIL flag.
    #[inline(always)]
    pub fn ccrcfailc(&self) -> CCRCFAILC_R {
        CCRCFAILC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DCRCFAIL flag clear bit Set by software to clear the DCRCFAIL flag.
    #[inline(always)]
    pub fn dcrcfailc(&self) -> DCRCFAILC_R {
        DCRCFAILC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CTIMEOUT flag clear bit Set by software to clear the CTIMEOUT flag.
    #[inline(always)]
    pub fn ctimeoutc(&self) -> CTIMEOUTC_R {
        CTIMEOUTC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DTIMEOUT flag clear bit Set by software to clear the DTIMEOUT flag.
    #[inline(always)]
    pub fn dtimeoutc(&self) -> DTIMEOUTC_R {
        DTIMEOUTC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TXUNDERR flag clear bit Set by software to clear TXUNDERR flag.
    #[inline(always)]
    pub fn txunderrc(&self) -> TXUNDERRC_R {
        TXUNDERRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXOVERR flag clear bit Set by software to clear the RXOVERR flag.
    #[inline(always)]
    pub fn rxoverrc(&self) -> RXOVERRC_R {
        RXOVERRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CMDREND flag clear bit Set by software to clear the CMDREND flag.
    #[inline(always)]
    pub fn cmdrendc(&self) -> CMDRENDC_R {
        CMDRENDC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CMDSENT flag clear bit Set by software to clear the CMDSENT flag.
    #[inline(always)]
    pub fn cmdsentc(&self) -> CMDSENTC_R {
        CMDSENTC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - DATAEND flag clear bit Set by software to clear the DATAEND flag.
    #[inline(always)]
    pub fn dataendc(&self) -> DATAENDC_R {
        DATAENDC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DHOLD flag clear bit Set by software to clear the DHOLD flag.
    #[inline(always)]
    pub fn dholdc(&self) -> DHOLDC_R {
        DHOLDC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DBCKEND flag clear bit Set by software to clear the DBCKEND flag.
    #[inline(always)]
    pub fn dbckendc(&self) -> DBCKENDC_R {
        DBCKENDC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DABORT flag clear bit Set by software to clear the DABORT flag.
    #[inline(always)]
    pub fn dabortc(&self) -> DABORTC_R {
        DABORTC_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 21 - BUSYD0END flag clear bit Set by software to clear the BUSYD0END flag.
    #[inline(always)]
    pub fn busyd0endc(&self) -> BUSYD0ENDC_R {
        BUSYD0ENDC_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SDIOIT flag clear bit Set by software to clear the SDIOIT flag.
    #[inline(always)]
    pub fn sdioitc(&self) -> SDIOITC_R {
        SDIOITC_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ACKFAIL flag clear bit Set by software to clear the ACKFAIL flag.
    #[inline(always)]
    pub fn ackfailc(&self) -> ACKFAILC_R {
        ACKFAILC_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - ACKTIMEOUT flag clear bit Set by software to clear the ACKTIMEOUT flag.
    #[inline(always)]
    pub fn acktimeoutc(&self) -> ACKTIMEOUTC_R {
        ACKTIMEOUTC_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - VSWEND flag clear bit Set by software to clear the VSWEND flag.
    #[inline(always)]
    pub fn vswendc(&self) -> VSWENDC_R {
        VSWENDC_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - CKSTOP flag clear bit Set by software to clear the CKSTOP flag.
    #[inline(always)]
    pub fn ckstopc(&self) -> CKSTOPC_R {
        CKSTOPC_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - IDMA transfer error clear bit Set by software to clear the IDMATE flag.
    #[inline(always)]
    pub fn idmatec(&self) -> IDMATEC_R {
        IDMATEC_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - IDMA buffer transfer complete clear bit Set by software to clear the IDMABTC flag.
    #[inline(always)]
    pub fn idmabtcc(&self) -> IDMABTCC_R {
        IDMABTCC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CCRCFAIL flag clear bit Set by software to clear the CCRCFAIL flag.
    #[inline(always)]
    #[must_use]
    pub fn ccrcfailc(&mut self) -> CCRCFAILC_W<0> {
        CCRCFAILC_W::new(self)
    }
    ///Bit 1 - DCRCFAIL flag clear bit Set by software to clear the DCRCFAIL flag.
    #[inline(always)]
    #[must_use]
    pub fn dcrcfailc(&mut self) -> DCRCFAILC_W<1> {
        DCRCFAILC_W::new(self)
    }
    ///Bit 2 - CTIMEOUT flag clear bit Set by software to clear the CTIMEOUT flag.
    #[inline(always)]
    #[must_use]
    pub fn ctimeoutc(&mut self) -> CTIMEOUTC_W<2> {
        CTIMEOUTC_W::new(self)
    }
    ///Bit 3 - DTIMEOUT flag clear bit Set by software to clear the DTIMEOUT flag.
    #[inline(always)]
    #[must_use]
    pub fn dtimeoutc(&mut self) -> DTIMEOUTC_W<3> {
        DTIMEOUTC_W::new(self)
    }
    ///Bit 4 - TXUNDERR flag clear bit Set by software to clear TXUNDERR flag.
    #[inline(always)]
    #[must_use]
    pub fn txunderrc(&mut self) -> TXUNDERRC_W<4> {
        TXUNDERRC_W::new(self)
    }
    ///Bit 5 - RXOVERR flag clear bit Set by software to clear the RXOVERR flag.
    #[inline(always)]
    #[must_use]
    pub fn rxoverrc(&mut self) -> RXOVERRC_W<5> {
        RXOVERRC_W::new(self)
    }
    ///Bit 6 - CMDREND flag clear bit Set by software to clear the CMDREND flag.
    #[inline(always)]
    #[must_use]
    pub fn cmdrendc(&mut self) -> CMDRENDC_W<6> {
        CMDRENDC_W::new(self)
    }
    ///Bit 7 - CMDSENT flag clear bit Set by software to clear the CMDSENT flag.
    #[inline(always)]
    #[must_use]
    pub fn cmdsentc(&mut self) -> CMDSENTC_W<7> {
        CMDSENTC_W::new(self)
    }
    ///Bit 8 - DATAEND flag clear bit Set by software to clear the DATAEND flag.
    #[inline(always)]
    #[must_use]
    pub fn dataendc(&mut self) -> DATAENDC_W<8> {
        DATAENDC_W::new(self)
    }
    ///Bit 9 - DHOLD flag clear bit Set by software to clear the DHOLD flag.
    #[inline(always)]
    #[must_use]
    pub fn dholdc(&mut self) -> DHOLDC_W<9> {
        DHOLDC_W::new(self)
    }
    ///Bit 10 - DBCKEND flag clear bit Set by software to clear the DBCKEND flag.
    #[inline(always)]
    #[must_use]
    pub fn dbckendc(&mut self) -> DBCKENDC_W<10> {
        DBCKENDC_W::new(self)
    }
    ///Bit 11 - DABORT flag clear bit Set by software to clear the DABORT flag.
    #[inline(always)]
    #[must_use]
    pub fn dabortc(&mut self) -> DABORTC_W<11> {
        DABORTC_W::new(self)
    }
    ///Bit 21 - BUSYD0END flag clear bit Set by software to clear the BUSYD0END flag.
    #[inline(always)]
    #[must_use]
    pub fn busyd0endc(&mut self) -> BUSYD0ENDC_W<21> {
        BUSYD0ENDC_W::new(self)
    }
    ///Bit 22 - SDIOIT flag clear bit Set by software to clear the SDIOIT flag.
    #[inline(always)]
    #[must_use]
    pub fn sdioitc(&mut self) -> SDIOITC_W<22> {
        SDIOITC_W::new(self)
    }
    ///Bit 23 - ACKFAIL flag clear bit Set by software to clear the ACKFAIL flag.
    #[inline(always)]
    #[must_use]
    pub fn ackfailc(&mut self) -> ACKFAILC_W<23> {
        ACKFAILC_W::new(self)
    }
    ///Bit 24 - ACKTIMEOUT flag clear bit Set by software to clear the ACKTIMEOUT flag.
    #[inline(always)]
    #[must_use]
    pub fn acktimeoutc(&mut self) -> ACKTIMEOUTC_W<24> {
        ACKTIMEOUTC_W::new(self)
    }
    ///Bit 25 - VSWEND flag clear bit Set by software to clear the VSWEND flag.
    #[inline(always)]
    #[must_use]
    pub fn vswendc(&mut self) -> VSWENDC_W<25> {
        VSWENDC_W::new(self)
    }
    ///Bit 26 - CKSTOP flag clear bit Set by software to clear the CKSTOP flag.
    #[inline(always)]
    #[must_use]
    pub fn ckstopc(&mut self) -> CKSTOPC_W<26> {
        CKSTOPC_W::new(self)
    }
    ///Bit 27 - IDMA transfer error clear bit Set by software to clear the IDMATE flag.
    #[inline(always)]
    #[must_use]
    pub fn idmatec(&mut self) -> IDMATEC_W<27> {
        IDMATEC_W::new(self)
    }
    ///Bit 28 - IDMA buffer transfer complete clear bit Set by software to clear the IDMABTC flag.
    #[inline(always)]
    #[must_use]
    pub fn idmabtcc(&mut self) -> IDMABTCC_W<28> {
        IDMABTCC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icr](index.html) module
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
///`read()` method returns [icr::R](R) reader structure
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [icr::W](W) writer structure
impl crate::Writable for ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
