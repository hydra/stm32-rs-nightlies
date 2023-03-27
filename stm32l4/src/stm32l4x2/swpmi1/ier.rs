///Register `IER` reader
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IER` writer
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RXBFIE` reader - Receive buffer full interrupt enable
pub type RXBFIE_R = crate::BitReader<bool>;
///Field `RXBFIE` writer - Receive buffer full interrupt enable
pub type RXBFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `TXBEIE` reader - Transmit buffer empty interrupt enable
pub type TXBEIE_R = crate::BitReader<bool>;
///Field `TXBEIE` writer - Transmit buffer empty interrupt enable
pub type TXBEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `RXBERIE` reader - Receive CRC error interrupt enable
pub type RXBERIE_R = crate::BitReader<bool>;
///Field `RXBERIE` writer - Receive CRC error interrupt enable
pub type RXBERIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `RXOVRIE` reader - Receive overrun error interrupt enable
pub type RXOVRIE_R = crate::BitReader<bool>;
///Field `RXOVRIE` writer - Receive overrun error interrupt enable
pub type RXOVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `TXUNRIE` reader - Transmit underrun error interrupt enable
pub type TXUNRIE_R = crate::BitReader<bool>;
///Field `TXUNRIE` writer - Transmit underrun error interrupt enable
pub type TXUNRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `RIE` reader - Receive interrupt enable
pub type RIE_R = crate::BitReader<bool>;
///Field `RIE` writer - Receive interrupt enable
pub type RIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `TIE` reader - Transmit interrupt enable
pub type TIE_R = crate::BitReader<bool>;
///Field `TIE` writer - Transmit interrupt enable
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `TCIE` reader - Transmit complete interrupt enable
pub type TCIE_R = crate::BitReader<bool>;
///Field `TCIE` writer - Transmit complete interrupt enable
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `SRIE` reader - Slave resume interrupt enable
pub type SRIE_R = crate::BitReader<bool>;
///Field `SRIE` writer - Slave resume interrupt enable
pub type SRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
impl R {
    ///Bit 0 - Receive buffer full interrupt enable
    #[inline(always)]
    pub fn rxbfie(&self) -> RXBFIE_R {
        RXBFIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit buffer empty interrupt enable
    #[inline(always)]
    pub fn txbeie(&self) -> TXBEIE_R {
        TXBEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receive CRC error interrupt enable
    #[inline(always)]
    pub fn rxberie(&self) -> RXBERIE_R {
        RXBERIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Receive overrun error interrupt enable
    #[inline(always)]
    pub fn rxovrie(&self) -> RXOVRIE_R {
        RXOVRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Transmit underrun error interrupt enable
    #[inline(always)]
    pub fn txunrie(&self) -> TXUNRIE_R {
        TXUNRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Receive interrupt enable
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transmit interrupt enable
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmit complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Slave resume interrupt enable
    #[inline(always)]
    pub fn srie(&self) -> SRIE_R {
        SRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Receive buffer full interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rxbfie(&mut self) -> RXBFIE_W<0> {
        RXBFIE_W::new(self)
    }
    ///Bit 1 - Transmit buffer empty interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn txbeie(&mut self) -> TXBEIE_W<1> {
        TXBEIE_W::new(self)
    }
    ///Bit 2 - Receive CRC error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rxberie(&mut self) -> RXBERIE_W<2> {
        RXBERIE_W::new(self)
    }
    ///Bit 3 - Receive overrun error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rxovrie(&mut self) -> RXOVRIE_W<3> {
        RXOVRIE_W::new(self)
    }
    ///Bit 4 - Transmit underrun error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn txunrie(&mut self) -> TXUNRIE_W<4> {
        TXUNRIE_W::new(self)
    }
    ///Bit 5 - Receive interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<5> {
        RIE_W::new(self)
    }
    ///Bit 6 - Transmit interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<6> {
        TIE_W::new(self)
    }
    ///Bit 7 - Transmit complete interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<7> {
        TCIE_W::new(self)
    }
    ///Bit 8 - Slave resume interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn srie(&mut self) -> SRIE_W<8> {
        SRIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SWPMI Interrupt Enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ier](index.html) module
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
///`read()` method returns [ier::R](R) reader structure
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ier::W](W) writer structure
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
