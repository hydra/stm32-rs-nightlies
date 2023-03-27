///Register `SDMMC_ICR` reader
pub struct R(crate::R<SDMMC_ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMMC_ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMMC_ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMMC_ICR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SDMMC_ICR` writer
pub struct W(crate::W<SDMMC_ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMMC_ICR_SPEC>;
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
impl From<crate::W<SDMMC_ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMMC_ICR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CCRCFAILC` reader - CCRCFAILC
pub type CCRCFAILC_R = crate::BitReader<bool>;
///Field `CCRCFAILC` writer - CCRCFAILC
pub type CCRCFAILC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_ICR_SPEC, bool, O>;
///Field `DCRCFAILC` reader - DCRCFAILC
pub type DCRCFAILC_R = crate::BitReader<bool>;
///Field `DCRCFAILC` writer - DCRCFAILC
pub type DCRCFAILC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_ICR_SPEC, bool, O>;
///Field `CTIMEOUTC` reader - CTIMEOUTC
pub type CTIMEOUTC_R = crate::BitReader<bool>;
///Field `CTIMEOUTC` writer - CTIMEOUTC
pub type CTIMEOUTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_ICR_SPEC, bool, O>;
///Field `DTIMEOUTC` reader - DTIMEOUTC
pub type DTIMEOUTC_R = crate::BitReader<bool>;
///Field `DTIMEOUTC` writer - DTIMEOUTC
pub type DTIMEOUTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_ICR_SPEC, bool, O>;
///Field `TXUNDERRC` reader - TXUNDERRC
pub type TXUNDERRC_R = crate::BitReader<bool>;
///Field `TXUNDERRC` writer - TXUNDERRC
pub type TXUNDERRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_ICR_SPEC, bool, O>;
///Field `RXOVERRC` reader - RXOVERRC
pub type RXOVERRC_R = crate::BitReader<bool>;
///Field `RXOVERRC` writer - RXOVERRC
pub type RXOVERRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_ICR_SPEC, bool, O>;
///Field `CMDRENDC` reader - CMDRENDC
pub type CMDRENDC_R = crate::BitReader<bool>;
///Field `CMDRENDC` writer - CMDRENDC
pub type CMDRENDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_ICR_SPEC, bool, O>;
///Field `CMDSENTC` reader - CMDSENTC
pub type CMDSENTC_R = crate::BitReader<bool>;
///Field `CMDSENTC` writer - CMDSENTC
pub type CMDSENTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_ICR_SPEC, bool, O>;
///Field `DATAENDC` reader - DATAENDC
pub type DATAENDC_R = crate::BitReader<bool>;
///Field `DATAENDC` writer - DATAENDC
pub type DATAENDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_ICR_SPEC, bool, O>;
///Field `DHOLDC` reader - DHOLDC
pub type DHOLDC_R = crate::BitReader<bool>;
///Field `DHOLDC` writer - DHOLDC
pub type DHOLDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_ICR_SPEC, bool, O>;
///Field `DBCKENDC` reader - DBCKENDC
pub type DBCKENDC_R = crate::BitReader<bool>;
///Field `DBCKENDC` writer - DBCKENDC
pub type DBCKENDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_ICR_SPEC, bool, O>;
///Field `DABORTC` reader - DABORTC
pub type DABORTC_R = crate::BitReader<bool>;
///Field `DABORTC` writer - DABORTC
pub type DABORTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_ICR_SPEC, bool, O>;
///Field `BUSYD0ENDC` reader - BUSYD0ENDC
pub type BUSYD0ENDC_R = crate::BitReader<bool>;
///Field `BUSYD0ENDC` writer - BUSYD0ENDC
pub type BUSYD0ENDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_ICR_SPEC, bool, O>;
///Field `SDIOITC` reader - SDIOITC
pub type SDIOITC_R = crate::BitReader<bool>;
///Field `SDIOITC` writer - SDIOITC
pub type SDIOITC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_ICR_SPEC, bool, O>;
///Field `ACKFAILC` reader - ACKFAILC
pub type ACKFAILC_R = crate::BitReader<bool>;
///Field `ACKFAILC` writer - ACKFAILC
pub type ACKFAILC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_ICR_SPEC, bool, O>;
///Field `ACKTIMEOUTC` reader - ACKTIMEOUTC
pub type ACKTIMEOUTC_R = crate::BitReader<bool>;
///Field `ACKTIMEOUTC` writer - ACKTIMEOUTC
pub type ACKTIMEOUTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_ICR_SPEC, bool, O>;
///Field `VSWENDC` reader - VSWENDC
pub type VSWENDC_R = crate::BitReader<bool>;
///Field `VSWENDC` writer - VSWENDC
pub type VSWENDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_ICR_SPEC, bool, O>;
///Field `CKSTOPC` reader - CKSTOPC
pub type CKSTOPC_R = crate::BitReader<bool>;
///Field `CKSTOPC` writer - CKSTOPC
pub type CKSTOPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_ICR_SPEC, bool, O>;
///Field `IDMATEC` reader - IDMATEC
pub type IDMATEC_R = crate::BitReader<bool>;
///Field `IDMATEC` writer - IDMATEC
pub type IDMATEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_ICR_SPEC, bool, O>;
///Field `IDMABTCC` reader - IDMABTCC
pub type IDMABTCC_R = crate::BitReader<bool>;
///Field `IDMABTCC` writer - IDMABTCC
pub type IDMABTCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDMMC_ICR_SPEC, bool, O>;
impl R {
    ///Bit 0 - CCRCFAILC
    #[inline(always)]
    pub fn ccrcfailc(&self) -> CCRCFAILC_R {
        CCRCFAILC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DCRCFAILC
    #[inline(always)]
    pub fn dcrcfailc(&self) -> DCRCFAILC_R {
        DCRCFAILC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CTIMEOUTC
    #[inline(always)]
    pub fn ctimeoutc(&self) -> CTIMEOUTC_R {
        CTIMEOUTC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DTIMEOUTC
    #[inline(always)]
    pub fn dtimeoutc(&self) -> DTIMEOUTC_R {
        DTIMEOUTC_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TXUNDERRC
    #[inline(always)]
    pub fn txunderrc(&self) -> TXUNDERRC_R {
        TXUNDERRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXOVERRC
    #[inline(always)]
    pub fn rxoverrc(&self) -> RXOVERRC_R {
        RXOVERRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CMDRENDC
    #[inline(always)]
    pub fn cmdrendc(&self) -> CMDRENDC_R {
        CMDRENDC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CMDSENTC
    #[inline(always)]
    pub fn cmdsentc(&self) -> CMDSENTC_R {
        CMDSENTC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - DATAENDC
    #[inline(always)]
    pub fn dataendc(&self) -> DATAENDC_R {
        DATAENDC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - DHOLDC
    #[inline(always)]
    pub fn dholdc(&self) -> DHOLDC_R {
        DHOLDC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DBCKENDC
    #[inline(always)]
    pub fn dbckendc(&self) -> DBCKENDC_R {
        DBCKENDC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DABORTC
    #[inline(always)]
    pub fn dabortc(&self) -> DABORTC_R {
        DABORTC_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 21 - BUSYD0ENDC
    #[inline(always)]
    pub fn busyd0endc(&self) -> BUSYD0ENDC_R {
        BUSYD0ENDC_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SDIOITC
    #[inline(always)]
    pub fn sdioitc(&self) -> SDIOITC_R {
        SDIOITC_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ACKFAILC
    #[inline(always)]
    pub fn ackfailc(&self) -> ACKFAILC_R {
        ACKFAILC_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - ACKTIMEOUTC
    #[inline(always)]
    pub fn acktimeoutc(&self) -> ACKTIMEOUTC_R {
        ACKTIMEOUTC_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - VSWENDC
    #[inline(always)]
    pub fn vswendc(&self) -> VSWENDC_R {
        VSWENDC_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - CKSTOPC
    #[inline(always)]
    pub fn ckstopc(&self) -> CKSTOPC_R {
        CKSTOPC_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - IDMATEC
    #[inline(always)]
    pub fn idmatec(&self) -> IDMATEC_R {
        IDMATEC_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - IDMABTCC
    #[inline(always)]
    pub fn idmabtcc(&self) -> IDMABTCC_R {
        IDMABTCC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CCRCFAILC
    #[inline(always)]
    #[must_use]
    pub fn ccrcfailc(&mut self) -> CCRCFAILC_W<0> {
        CCRCFAILC_W::new(self)
    }
    ///Bit 1 - DCRCFAILC
    #[inline(always)]
    #[must_use]
    pub fn dcrcfailc(&mut self) -> DCRCFAILC_W<1> {
        DCRCFAILC_W::new(self)
    }
    ///Bit 2 - CTIMEOUTC
    #[inline(always)]
    #[must_use]
    pub fn ctimeoutc(&mut self) -> CTIMEOUTC_W<2> {
        CTIMEOUTC_W::new(self)
    }
    ///Bit 3 - DTIMEOUTC
    #[inline(always)]
    #[must_use]
    pub fn dtimeoutc(&mut self) -> DTIMEOUTC_W<3> {
        DTIMEOUTC_W::new(self)
    }
    ///Bit 4 - TXUNDERRC
    #[inline(always)]
    #[must_use]
    pub fn txunderrc(&mut self) -> TXUNDERRC_W<4> {
        TXUNDERRC_W::new(self)
    }
    ///Bit 5 - RXOVERRC
    #[inline(always)]
    #[must_use]
    pub fn rxoverrc(&mut self) -> RXOVERRC_W<5> {
        RXOVERRC_W::new(self)
    }
    ///Bit 6 - CMDRENDC
    #[inline(always)]
    #[must_use]
    pub fn cmdrendc(&mut self) -> CMDRENDC_W<6> {
        CMDRENDC_W::new(self)
    }
    ///Bit 7 - CMDSENTC
    #[inline(always)]
    #[must_use]
    pub fn cmdsentc(&mut self) -> CMDSENTC_W<7> {
        CMDSENTC_W::new(self)
    }
    ///Bit 8 - DATAENDC
    #[inline(always)]
    #[must_use]
    pub fn dataendc(&mut self) -> DATAENDC_W<8> {
        DATAENDC_W::new(self)
    }
    ///Bit 9 - DHOLDC
    #[inline(always)]
    #[must_use]
    pub fn dholdc(&mut self) -> DHOLDC_W<9> {
        DHOLDC_W::new(self)
    }
    ///Bit 10 - DBCKENDC
    #[inline(always)]
    #[must_use]
    pub fn dbckendc(&mut self) -> DBCKENDC_W<10> {
        DBCKENDC_W::new(self)
    }
    ///Bit 11 - DABORTC
    #[inline(always)]
    #[must_use]
    pub fn dabortc(&mut self) -> DABORTC_W<11> {
        DABORTC_W::new(self)
    }
    ///Bit 21 - BUSYD0ENDC
    #[inline(always)]
    #[must_use]
    pub fn busyd0endc(&mut self) -> BUSYD0ENDC_W<21> {
        BUSYD0ENDC_W::new(self)
    }
    ///Bit 22 - SDIOITC
    #[inline(always)]
    #[must_use]
    pub fn sdioitc(&mut self) -> SDIOITC_W<22> {
        SDIOITC_W::new(self)
    }
    ///Bit 23 - ACKFAILC
    #[inline(always)]
    #[must_use]
    pub fn ackfailc(&mut self) -> ACKFAILC_W<23> {
        ACKFAILC_W::new(self)
    }
    ///Bit 24 - ACKTIMEOUTC
    #[inline(always)]
    #[must_use]
    pub fn acktimeoutc(&mut self) -> ACKTIMEOUTC_W<24> {
        ACKTIMEOUTC_W::new(self)
    }
    ///Bit 25 - VSWENDC
    #[inline(always)]
    #[must_use]
    pub fn vswendc(&mut self) -> VSWENDC_W<25> {
        VSWENDC_W::new(self)
    }
    ///Bit 26 - CKSTOPC
    #[inline(always)]
    #[must_use]
    pub fn ckstopc(&mut self) -> CKSTOPC_W<26> {
        CKSTOPC_W::new(self)
    }
    ///Bit 27 - IDMATEC
    #[inline(always)]
    #[must_use]
    pub fn idmatec(&mut self) -> IDMATEC_W<27> {
        IDMATEC_W::new(self)
    }
    ///Bit 28 - IDMABTCC
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
///For information about available fields see [sdmmc_icr](index.html) module
pub struct SDMMC_ICR_SPEC;
impl crate::RegisterSpec for SDMMC_ICR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sdmmc_icr::R](R) reader structure
impl crate::Readable for SDMMC_ICR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sdmmc_icr::W](W) writer structure
impl crate::Writable for SDMMC_ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SDMMC_ICR to value 0
impl crate::Resettable for SDMMC_ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
