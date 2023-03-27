///Register `MASK` reader
pub struct R(crate::R<MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MASK` writer
pub struct W(crate::W<MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK_SPEC>;
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
impl From<crate::W<MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CCRCFAILIE` reader - CCRCFAILIE
pub type CCRCFAILIE_R = crate::BitReader<bool>;
///Field `CCRCFAILIE` writer - CCRCFAILIE
pub type CCRCFAILIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
///Field `DCRCFAILIE` reader - DCRCFAILIE
pub type DCRCFAILIE_R = crate::BitReader<bool>;
///Field `DCRCFAILIE` writer - DCRCFAILIE
pub type DCRCFAILIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
///Field `CTIMEOUTIE` reader - CTIMEOUTIE
pub type CTIMEOUTIE_R = crate::BitReader<bool>;
///Field `CTIMEOUTIE` writer - CTIMEOUTIE
pub type CTIMEOUTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
///Field `DTIMEOUTIE` reader - DTIMEOUTIE
pub type DTIMEOUTIE_R = crate::BitReader<bool>;
///Field `DTIMEOUTIE` writer - DTIMEOUTIE
pub type DTIMEOUTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
///Field `TXUNDERRIE` reader - TXUNDERRIE
pub type TXUNDERRIE_R = crate::BitReader<bool>;
///Field `TXUNDERRIE` writer - TXUNDERRIE
pub type TXUNDERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
///Field `RXOVERRIE` reader - RXOVERRIE
pub type RXOVERRIE_R = crate::BitReader<bool>;
///Field `RXOVERRIE` writer - RXOVERRIE
pub type RXOVERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
///Field `CMDRENDIE` reader - CMDRENDIE
pub type CMDRENDIE_R = crate::BitReader<bool>;
///Field `CMDRENDIE` writer - CMDRENDIE
pub type CMDRENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
///Field `CMDSENTIE` reader - CMDSENTIE
pub type CMDSENTIE_R = crate::BitReader<bool>;
///Field `CMDSENTIE` writer - CMDSENTIE
pub type CMDSENTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
///Field `DATAENDIE` reader - DATAENDIE
pub type DATAENDIE_R = crate::BitReader<bool>;
///Field `DATAENDIE` writer - DATAENDIE
pub type DATAENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
///Field `STBITERRIE` reader - STBITERRIE
pub type STBITERRIE_R = crate::BitReader<bool>;
///Field `STBITERRIE` writer - STBITERRIE
pub type STBITERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
///Field `DBACKENDIE` reader - DBACKENDIE
pub type DBACKENDIE_R = crate::BitReader<bool>;
///Field `DBACKENDIE` writer - DBACKENDIE
pub type DBACKENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
///Field `CMDACTIE` reader - CMDACTIE
pub type CMDACTIE_R = crate::BitReader<bool>;
///Field `CMDACTIE` writer - CMDACTIE
pub type CMDACTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
///Field `TXACTIE` reader - TXACTIE
pub type TXACTIE_R = crate::BitReader<bool>;
///Field `TXACTIE` writer - TXACTIE
pub type TXACTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
///Field `RXACTIE` reader - RXACTIE
pub type RXACTIE_R = crate::BitReader<bool>;
///Field `RXACTIE` writer - RXACTIE
pub type RXACTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
///Field `TXFIFOHEIE` reader - TXFIFOHEIE
pub type TXFIFOHEIE_R = crate::BitReader<bool>;
///Field `TXFIFOHEIE` writer - TXFIFOHEIE
pub type TXFIFOHEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
///Field `RXFIFOHFIE` reader - RXFIFOHFIE
pub type RXFIFOHFIE_R = crate::BitReader<bool>;
///Field `RXFIFOHFIE` writer - RXFIFOHFIE
pub type RXFIFOHFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
///Field `TXFIFOFIE` reader - TXFIFOFIE
pub type TXFIFOFIE_R = crate::BitReader<bool>;
///Field `TXFIFOFIE` writer - TXFIFOFIE
pub type TXFIFOFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
///Field `RXFIFOFIE` reader - RXFIFOFIE
pub type RXFIFOFIE_R = crate::BitReader<bool>;
///Field `RXFIFOFIE` writer - RXFIFOFIE
pub type RXFIFOFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
///Field `TXFIFOEIE` reader - TXFIFOEIE
pub type TXFIFOEIE_R = crate::BitReader<bool>;
///Field `TXFIFOEIE` writer - TXFIFOEIE
pub type TXFIFOEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
///Field `RXFIFOEIE` reader - RXFIFOEIE
pub type RXFIFOEIE_R = crate::BitReader<bool>;
///Field `RXFIFOEIE` writer - RXFIFOEIE
pub type RXFIFOEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
///Field `TXDAVLIE` reader - TXDAVLIE
pub type TXDAVLIE_R = crate::BitReader<bool>;
///Field `TXDAVLIE` writer - TXDAVLIE
pub type TXDAVLIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
///Field `RXDAVLIE` reader - RXDAVLIE
pub type RXDAVLIE_R = crate::BitReader<bool>;
///Field `RXDAVLIE` writer - RXDAVLIE
pub type RXDAVLIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
///Field `SDIOITIE` reader - SDIOITIE
pub type SDIOITIE_R = crate::BitReader<bool>;
///Field `SDIOITIE` writer - SDIOITIE
pub type SDIOITIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
///Field `CEATENDIE` reader - CEATENDIE
pub type CEATENDIE_R = crate::BitReader<bool>;
///Field `CEATENDIE` writer - CEATENDIE
pub type CEATENDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASK_SPEC, bool, O>;
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
    ///Bit 9 - STBITERRIE
    #[inline(always)]
    pub fn stbiterrie(&self) -> STBITERRIE_R {
        STBITERRIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DBACKENDIE
    #[inline(always)]
    pub fn dbackendie(&self) -> DBACKENDIE_R {
        DBACKENDIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CMDACTIE
    #[inline(always)]
    pub fn cmdactie(&self) -> CMDACTIE_R {
        CMDACTIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - TXACTIE
    #[inline(always)]
    pub fn txactie(&self) -> TXACTIE_R {
        TXACTIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - RXACTIE
    #[inline(always)]
    pub fn rxactie(&self) -> RXACTIE_R {
        RXACTIE_R::new(((self.bits >> 13) & 1) != 0)
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
    ///Bit 16 - TXFIFOFIE
    #[inline(always)]
    pub fn txfifofie(&self) -> TXFIFOFIE_R {
        TXFIFOFIE_R::new(((self.bits >> 16) & 1) != 0)
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
    ///Bit 19 - RXFIFOEIE
    #[inline(always)]
    pub fn rxfifoeie(&self) -> RXFIFOEIE_R {
        RXFIFOEIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - TXDAVLIE
    #[inline(always)]
    pub fn txdavlie(&self) -> TXDAVLIE_R {
        TXDAVLIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - RXDAVLIE
    #[inline(always)]
    pub fn rxdavlie(&self) -> RXDAVLIE_R {
        RXDAVLIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SDIOITIE
    #[inline(always)]
    pub fn sdioitie(&self) -> SDIOITIE_R {
        SDIOITIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - CEATENDIE
    #[inline(always)]
    pub fn ceatendie(&self) -> CEATENDIE_R {
        CEATENDIE_R::new(((self.bits >> 23) & 1) != 0)
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
    ///Bit 9 - STBITERRIE
    #[inline(always)]
    #[must_use]
    pub fn stbiterrie(&mut self) -> STBITERRIE_W<9> {
        STBITERRIE_W::new(self)
    }
    ///Bit 10 - DBACKENDIE
    #[inline(always)]
    #[must_use]
    pub fn dbackendie(&mut self) -> DBACKENDIE_W<10> {
        DBACKENDIE_W::new(self)
    }
    ///Bit 11 - CMDACTIE
    #[inline(always)]
    #[must_use]
    pub fn cmdactie(&mut self) -> CMDACTIE_W<11> {
        CMDACTIE_W::new(self)
    }
    ///Bit 12 - TXACTIE
    #[inline(always)]
    #[must_use]
    pub fn txactie(&mut self) -> TXACTIE_W<12> {
        TXACTIE_W::new(self)
    }
    ///Bit 13 - RXACTIE
    #[inline(always)]
    #[must_use]
    pub fn rxactie(&mut self) -> RXACTIE_W<13> {
        RXACTIE_W::new(self)
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
    ///Bit 16 - TXFIFOFIE
    #[inline(always)]
    #[must_use]
    pub fn txfifofie(&mut self) -> TXFIFOFIE_W<16> {
        TXFIFOFIE_W::new(self)
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
    ///Bit 19 - RXFIFOEIE
    #[inline(always)]
    #[must_use]
    pub fn rxfifoeie(&mut self) -> RXFIFOEIE_W<19> {
        RXFIFOEIE_W::new(self)
    }
    ///Bit 20 - TXDAVLIE
    #[inline(always)]
    #[must_use]
    pub fn txdavlie(&mut self) -> TXDAVLIE_W<20> {
        TXDAVLIE_W::new(self)
    }
    ///Bit 21 - RXDAVLIE
    #[inline(always)]
    #[must_use]
    pub fn rxdavlie(&mut self) -> RXDAVLIE_W<21> {
        RXDAVLIE_W::new(self)
    }
    ///Bit 22 - SDIOITIE
    #[inline(always)]
    #[must_use]
    pub fn sdioitie(&mut self) -> SDIOITIE_W<22> {
        SDIOITIE_W::new(self)
    }
    ///Bit 23 - CEATENDIE
    #[inline(always)]
    #[must_use]
    pub fn ceatendie(&mut self) -> CEATENDIE_W<23> {
        CEATENDIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SDIO mask register (SDIO_MASK)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mask](index.html) module
pub struct MASK_SPEC;
impl crate::RegisterSpec for MASK_SPEC {
    type Ux = u32;
}
///`read()` method returns [mask::R](R) reader structure
impl crate::Readable for MASK_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mask::W](W) writer structure
impl crate::Writable for MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MASK to value 0
impl crate::Resettable for MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
