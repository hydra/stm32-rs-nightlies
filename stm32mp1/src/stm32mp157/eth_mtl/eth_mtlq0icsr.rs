///Register `ETH_MTLQ0ICSR` reader
pub struct R(crate::R<ETH_MTLQ0ICSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MTLQ0ICSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MTLQ0ICSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MTLQ0ICSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ETH_MTLQ0ICSR` writer
pub struct W(crate::W<ETH_MTLQ0ICSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MTLQ0ICSR_SPEC>;
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
impl From<crate::W<ETH_MTLQ0ICSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MTLQ0ICSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TXUNFIS` reader - TXUNFIS
pub type TXUNFIS_R = crate::BitReader<bool>;
///Field `ABPSIS` reader - ABPSIS
pub type ABPSIS_R = crate::BitReader<bool>;
///Field `ABPSIS` writer - ABPSIS
pub type ABPSIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLQ0ICSR_SPEC, bool, O>;
///Field `TXUIE` reader - TXUIE
pub type TXUIE_R = crate::BitReader<bool>;
///Field `TXUIE` writer - TXUIE
pub type TXUIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLQ0ICSR_SPEC, bool, O>;
///Field `ABPSIE` reader - ABPSIE
pub type ABPSIE_R = crate::BitReader<bool>;
///Field `ABPSIE` writer - ABPSIE
pub type ABPSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLQ0ICSR_SPEC, bool, O>;
///Field `RXOVFIS` reader - RXOVFIS
pub type RXOVFIS_R = crate::BitReader<bool>;
///Field `RXOVFIS` writer - RXOVFIS
pub type RXOVFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLQ0ICSR_SPEC, bool, O>;
///Field `RXOIE` reader - RXOIE
pub type RXOIE_R = crate::BitReader<bool>;
///Field `RXOIE` writer - RXOIE
pub type RXOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_MTLQ0ICSR_SPEC, bool, O>;
impl R {
    ///Bit 0 - TXUNFIS
    #[inline(always)]
    pub fn txunfis(&self) -> TXUNFIS_R {
        TXUNFIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ABPSIS
    #[inline(always)]
    pub fn abpsis(&self) -> ABPSIS_R {
        ABPSIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - TXUIE
    #[inline(always)]
    pub fn txuie(&self) -> TXUIE_R {
        TXUIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ABPSIE
    #[inline(always)]
    pub fn abpsie(&self) -> ABPSIE_R {
        ABPSIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - RXOVFIS
    #[inline(always)]
    pub fn rxovfis(&self) -> RXOVFIS_R {
        RXOVFIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - RXOIE
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bit 1 - ABPSIS
    #[inline(always)]
    #[must_use]
    pub fn abpsis(&mut self) -> ABPSIS_W<1> {
        ABPSIS_W::new(self)
    }
    ///Bit 8 - TXUIE
    #[inline(always)]
    #[must_use]
    pub fn txuie(&mut self) -> TXUIE_W<8> {
        TXUIE_W::new(self)
    }
    ///Bit 9 - ABPSIE
    #[inline(always)]
    #[must_use]
    pub fn abpsie(&mut self) -> ABPSIE_W<9> {
        ABPSIE_W::new(self)
    }
    ///Bit 16 - RXOVFIS
    #[inline(always)]
    #[must_use]
    pub fn rxovfis(&mut self) -> RXOVFIS_W<16> {
        RXOVFIS_W::new(self)
    }
    ///Bit 24 - RXOIE
    #[inline(always)]
    #[must_use]
    pub fn rxoie(&mut self) -> RXOIE_W<24> {
        RXOIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Queue 0 interrupt control status Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_mtlq0icsr](index.html) module
pub struct ETH_MTLQ0ICSR_SPEC;
impl crate::RegisterSpec for ETH_MTLQ0ICSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_mtlq0icsr::R](R) reader structure
impl crate::Readable for ETH_MTLQ0ICSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [eth_mtlq0icsr::W](W) writer structure
impl crate::Writable for ETH_MTLQ0ICSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ETH_MTLQ0ICSR to value 0
impl crate::Resettable for ETH_MTLQ0ICSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
