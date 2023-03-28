///Register `STA` reader
pub struct R(crate::R<STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STA_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CCRCFAIL` reader - CCRCFAIL
pub type CCRCFAIL_R = crate::BitReader<bool>;
///Field `DCRCFAIL` reader - DCRCFAIL
pub type DCRCFAIL_R = crate::BitReader<bool>;
///Field `CTIMEOUT` reader - CTIMEOUT
pub type CTIMEOUT_R = crate::BitReader<bool>;
///Field `DTIMEOUT` reader - DTIMEOUT
pub type DTIMEOUT_R = crate::BitReader<bool>;
///Field `TXUNDERR` reader - TXUNDERR
pub type TXUNDERR_R = crate::BitReader<bool>;
///Field `RXOVERR` reader - RXOVERR
pub type RXOVERR_R = crate::BitReader<bool>;
///Field `CMDREND` reader - CMDREND
pub type CMDREND_R = crate::BitReader<bool>;
///Field `CMDSENT` reader - CMDSENT
pub type CMDSENT_R = crate::BitReader<bool>;
///Field `DATAEND` reader - DATAEND
pub type DATAEND_R = crate::BitReader<bool>;
///Field `STBITERR` reader - STBITERR
pub type STBITERR_R = crate::BitReader<bool>;
///Field `DBCKEND` reader - DBCKEND
pub type DBCKEND_R = crate::BitReader<bool>;
///Field `CMDACT` reader - CMDACT
pub type CMDACT_R = crate::BitReader<bool>;
///Field `TXACT` reader - TXACT
pub type TXACT_R = crate::BitReader<bool>;
///Field `RXACT` reader - RXACT
pub type RXACT_R = crate::BitReader<bool>;
///Field `TXFIFOHE` reader - TXFIFOHE
pub type TXFIFOHE_R = crate::BitReader<bool>;
///Field `RXFIFOHF` reader - RXFIFOHF
pub type RXFIFOHF_R = crate::BitReader<bool>;
///Field `TXFIFOF` reader - TXFIFOF
pub type TXFIFOF_R = crate::BitReader<bool>;
///Field `RXFIFOF` reader - RXFIFOF
pub type RXFIFOF_R = crate::BitReader<bool>;
///Field `TXFIFOE` reader - TXFIFOE
pub type TXFIFOE_R = crate::BitReader<bool>;
///Field `RXFIFOE` reader - RXFIFOE
pub type RXFIFOE_R = crate::BitReader<bool>;
///Field `TXDAVL` reader - TXDAVL
pub type TXDAVL_R = crate::BitReader<bool>;
///Field `RXDAVL` reader - RXDAVL
pub type RXDAVL_R = crate::BitReader<bool>;
///Field `SDIOIT` reader - SDIOIT
pub type SDIOIT_R = crate::BitReader<bool>;
///Field `CEATAEND` reader - CEATAEND
pub type CEATAEND_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - CCRCFAIL
    #[inline(always)]
    pub fn ccrcfail(&self) -> CCRCFAIL_R {
        CCRCFAIL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DCRCFAIL
    #[inline(always)]
    pub fn dcrcfail(&self) -> DCRCFAIL_R {
        DCRCFAIL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CTIMEOUT
    #[inline(always)]
    pub fn ctimeout(&self) -> CTIMEOUT_R {
        CTIMEOUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DTIMEOUT
    #[inline(always)]
    pub fn dtimeout(&self) -> DTIMEOUT_R {
        DTIMEOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TXUNDERR
    #[inline(always)]
    pub fn txunderr(&self) -> TXUNDERR_R {
        TXUNDERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - RXOVERR
    #[inline(always)]
    pub fn rxoverr(&self) -> RXOVERR_R {
        RXOVERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CMDREND
    #[inline(always)]
    pub fn cmdrend(&self) -> CMDREND_R {
        CMDREND_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CMDSENT
    #[inline(always)]
    pub fn cmdsent(&self) -> CMDSENT_R {
        CMDSENT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - DATAEND
    #[inline(always)]
    pub fn dataend(&self) -> DATAEND_R {
        DATAEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - STBITERR
    #[inline(always)]
    pub fn stbiterr(&self) -> STBITERR_R {
        STBITERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DBCKEND
    #[inline(always)]
    pub fn dbckend(&self) -> DBCKEND_R {
        DBCKEND_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CMDACT
    #[inline(always)]
    pub fn cmdact(&self) -> CMDACT_R {
        CMDACT_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TXACT
    #[inline(always)]
    pub fn txact(&self) -> TXACT_R {
        TXACT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - RXACT
    #[inline(always)]
    pub fn rxact(&self) -> RXACT_R {
        RXACT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - TXFIFOHE
    #[inline(always)]
    pub fn txfifohe(&self) -> TXFIFOHE_R {
        TXFIFOHE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - RXFIFOHF
    #[inline(always)]
    pub fn rxfifohf(&self) -> RXFIFOHF_R {
        RXFIFOHF_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - TXFIFOF
    #[inline(always)]
    pub fn txfifof(&self) -> TXFIFOF_R {
        TXFIFOF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - RXFIFOF
    #[inline(always)]
    pub fn rxfifof(&self) -> RXFIFOF_R {
        RXFIFOF_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TXFIFOE
    #[inline(always)]
    pub fn txfifoe(&self) -> TXFIFOE_R {
        TXFIFOE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - RXFIFOE
    #[inline(always)]
    pub fn rxfifoe(&self) -> RXFIFOE_R {
        RXFIFOE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - TXDAVL
    #[inline(always)]
    pub fn txdavl(&self) -> TXDAVL_R {
        TXDAVL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - RXDAVL
    #[inline(always)]
    pub fn rxdavl(&self) -> RXDAVL_R {
        RXDAVL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SDIOIT
    #[inline(always)]
    pub fn sdioit(&self) -> SDIOIT_R {
        SDIOIT_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - CEATAEND
    #[inline(always)]
    pub fn ceataend(&self) -> CEATAEND_R {
        CEATAEND_R::new(((self.bits >> 23) & 1) != 0)
    }
}
///SDIO status register (SDIO_STA)
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sta](index.html) module
pub struct STA_SPEC;
impl crate::RegisterSpec for STA_SPEC {
    type Ux = u32;
}
///`read()` method returns [sta::R](R) reader structure
impl crate::Readable for STA_SPEC {
    type Reader = R;
}
///`reset()` method sets STA to value 0
impl crate::Resettable for STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
