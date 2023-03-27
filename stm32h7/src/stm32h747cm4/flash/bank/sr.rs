///Register `SR` reader
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SR` writer
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BSY` reader - Bank 1 busy flag
pub type BSY_R = crate::BitReader<bool>;
///Field `BSY` writer - Bank 1 busy flag
pub type BSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `WBNE` reader - Bank 1 write buffer not empty flag
pub type WBNE_R = crate::BitReader<bool>;
///Field `WBNE` writer - Bank 1 write buffer not empty flag
pub type WBNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `QW` reader - Bank 1 wait queue flag
pub type QW_R = crate::BitReader<bool>;
///Field `QW` writer - Bank 1 wait queue flag
pub type QW_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `CRC_BUSY` reader - Bank 1 CRC busy flag
pub type CRC_BUSY_R = crate::BitReader<bool>;
///Field `CRC_BUSY` writer - Bank 1 CRC busy flag
pub type CRC_BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `EOP` reader - Bank 1 end-of-program flag
pub type EOP_R = crate::BitReader<bool>;
///Field `EOP` writer - Bank 1 end-of-program flag
pub type EOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `WRPERR` reader - Bank 1 write protection error flag
pub type WRPERR_R = crate::BitReader<bool>;
///Field `WRPERR` writer - Bank 1 write protection error flag
pub type WRPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `PGSERR` reader - Bank 1 programming sequence error flag
pub type PGSERR_R = crate::BitReader<bool>;
///Field `PGSERR` writer - Bank 1 programming sequence error flag
pub type PGSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `STRBERR` reader - Bank 1 strobe error flag
pub type STRBERR_R = crate::BitReader<bool>;
///Field `STRBERR` writer - Bank 1 strobe error flag
pub type STRBERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `INCERR` reader - Bank 1 inconsistency error flag
pub type INCERR_R = crate::BitReader<bool>;
///Field `INCERR` writer - Bank 1 inconsistency error flag
pub type INCERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `OPERR` reader - Bank 1 write
pub type OPERR_R = crate::BitReader<bool>;
///Field `OPERR` writer - Bank 1 write
pub type OPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `RDPERR` reader - Bank 1 read protection error flag
pub type RDPERR_R = crate::BitReader<bool>;
///Field `RDPERR` writer - Bank 1 read protection error flag
pub type RDPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `RDSERR` reader - Bank 1 secure error flag
pub type RDSERR_R = crate::BitReader<bool>;
///Field `RDSERR` writer - Bank 1 secure error flag
pub type RDSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `SNECCERR` reader - Bank 1 single correction error flag
pub type SNECCERR_R = crate::BitReader<bool>;
///Field `SNECCERR` writer - Bank 1 single correction error flag
pub type SNECCERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `DBECCERR` reader - Bank 1 ECC double detection error flag
pub type DBECCERR_R = crate::BitReader<bool>;
///Field `DBECCERR` writer - Bank 1 ECC double detection error flag
pub type DBECCERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `CRCEND` reader - Bank 1 CRC end of calculation flag
pub type CRCEND_R = crate::BitReader<bool>;
///Field `CRCEND` writer - Bank 1 CRC end of calculation flag
pub type CRCEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `CRCRDERR` reader - Bank 1 CRC read error flag
pub type CRCRDERR_R = crate::BitReader<bool>;
///Field `CRCRDERR` writer - Bank 1 CRC read error flag
pub type CRCRDERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Bank 1 busy flag
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Bank 1 write buffer not empty flag
    #[inline(always)]
    pub fn wbne(&self) -> WBNE_R {
        WBNE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Bank 1 wait queue flag
    #[inline(always)]
    pub fn qw(&self) -> QW_R {
        QW_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Bank 1 CRC busy flag
    #[inline(always)]
    pub fn crc_busy(&self) -> CRC_BUSY_R {
        CRC_BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 16 - Bank 1 end-of-program flag
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Bank 1 write protection error flag
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Bank 1 programming sequence error flag
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Bank 1 strobe error flag
    #[inline(always)]
    pub fn strberr(&self) -> STRBERR_R {
        STRBERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 21 - Bank 1 inconsistency error flag
    #[inline(always)]
    pub fn incerr(&self) -> INCERR_R {
        INCERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Bank 1 write
    #[inline(always)]
    pub fn operr(&self) -> OPERR_R {
        OPERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Bank 1 read protection error flag
    #[inline(always)]
    pub fn rdperr(&self) -> RDPERR_R {
        RDPERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Bank 1 secure error flag
    #[inline(always)]
    pub fn rdserr(&self) -> RDSERR_R {
        RDSERR_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Bank 1 single correction error flag
    #[inline(always)]
    pub fn sneccerr(&self) -> SNECCERR_R {
        SNECCERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Bank 1 ECC double detection error flag
    #[inline(always)]
    pub fn dbeccerr(&self) -> DBECCERR_R {
        DBECCERR_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Bank 1 CRC end of calculation flag
    #[inline(always)]
    pub fn crcend(&self) -> CRCEND_R {
        CRCEND_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Bank 1 CRC read error flag
    #[inline(always)]
    pub fn crcrderr(&self) -> CRCRDERR_R {
        CRCRDERR_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Bank 1 busy flag
    #[inline(always)]
    #[must_use]
    pub fn bsy(&mut self) -> BSY_W<0> {
        BSY_W::new(self)
    }
    ///Bit 1 - Bank 1 write buffer not empty flag
    #[inline(always)]
    #[must_use]
    pub fn wbne(&mut self) -> WBNE_W<1> {
        WBNE_W::new(self)
    }
    ///Bit 2 - Bank 1 wait queue flag
    #[inline(always)]
    #[must_use]
    pub fn qw(&mut self) -> QW_W<2> {
        QW_W::new(self)
    }
    ///Bit 3 - Bank 1 CRC busy flag
    #[inline(always)]
    #[must_use]
    pub fn crc_busy(&mut self) -> CRC_BUSY_W<3> {
        CRC_BUSY_W::new(self)
    }
    ///Bit 16 - Bank 1 end-of-program flag
    #[inline(always)]
    #[must_use]
    pub fn eop(&mut self) -> EOP_W<16> {
        EOP_W::new(self)
    }
    ///Bit 17 - Bank 1 write protection error flag
    #[inline(always)]
    #[must_use]
    pub fn wrperr(&mut self) -> WRPERR_W<17> {
        WRPERR_W::new(self)
    }
    ///Bit 18 - Bank 1 programming sequence error flag
    #[inline(always)]
    #[must_use]
    pub fn pgserr(&mut self) -> PGSERR_W<18> {
        PGSERR_W::new(self)
    }
    ///Bit 19 - Bank 1 strobe error flag
    #[inline(always)]
    #[must_use]
    pub fn strberr(&mut self) -> STRBERR_W<19> {
        STRBERR_W::new(self)
    }
    ///Bit 21 - Bank 1 inconsistency error flag
    #[inline(always)]
    #[must_use]
    pub fn incerr(&mut self) -> INCERR_W<21> {
        INCERR_W::new(self)
    }
    ///Bit 22 - Bank 1 write
    #[inline(always)]
    #[must_use]
    pub fn operr(&mut self) -> OPERR_W<22> {
        OPERR_W::new(self)
    }
    ///Bit 23 - Bank 1 read protection error flag
    #[inline(always)]
    #[must_use]
    pub fn rdperr(&mut self) -> RDPERR_W<23> {
        RDPERR_W::new(self)
    }
    ///Bit 24 - Bank 1 secure error flag
    #[inline(always)]
    #[must_use]
    pub fn rdserr(&mut self) -> RDSERR_W<24> {
        RDSERR_W::new(self)
    }
    ///Bit 25 - Bank 1 single correction error flag
    #[inline(always)]
    #[must_use]
    pub fn sneccerr(&mut self) -> SNECCERR_W<25> {
        SNECCERR_W::new(self)
    }
    ///Bit 26 - Bank 1 ECC double detection error flag
    #[inline(always)]
    #[must_use]
    pub fn dbeccerr(&mut self) -> DBECCERR_W<26> {
        DBECCERR_W::new(self)
    }
    ///Bit 27 - Bank 1 CRC end of calculation flag
    #[inline(always)]
    #[must_use]
    pub fn crcend(&mut self) -> CRCEND_W<27> {
        CRCEND_W::new(self)
    }
    ///Bit 28 - Bank 1 CRC read error flag
    #[inline(always)]
    #[must_use]
    pub fn crcrderr(&mut self) -> CRCRDERR_W<28> {
        CRCRDERR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH status register for bank 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](index.html) module
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr::R](R) reader structure
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sr::W](W) writer structure
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
