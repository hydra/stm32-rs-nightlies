///Register `MCR` reader
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MCR` writer
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `INRQ` reader - INRQ
pub type INRQ_R = crate::BitReader<bool>;
///Field `INRQ` writer - INRQ
pub type INRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
///Field `SLEEP` reader - SLEEP
pub type SLEEP_R = crate::BitReader<bool>;
///Field `SLEEP` writer - SLEEP
pub type SLEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
///Field `TXFP` reader - TXFP
pub type TXFP_R = crate::BitReader<bool>;
///Field `TXFP` writer - TXFP
pub type TXFP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
///Field `RFLM` reader - RFLM
pub type RFLM_R = crate::BitReader<bool>;
///Field `RFLM` writer - RFLM
pub type RFLM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
///Field `NART` reader - NART
pub type NART_R = crate::BitReader<bool>;
///Field `NART` writer - NART
pub type NART_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
///Field `AWUM` reader - AWUM
pub type AWUM_R = crate::BitReader<bool>;
///Field `AWUM` writer - AWUM
pub type AWUM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
///Field `ABOM` reader - ABOM
pub type ABOM_R = crate::BitReader<bool>;
///Field `ABOM` writer - ABOM
pub type ABOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
///Field `TTCM` reader - TTCM
pub type TTCM_R = crate::BitReader<bool>;
///Field `TTCM` writer - TTCM
pub type TTCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
///Field `RESET` reader - RESET
pub type RESET_R = crate::BitReader<bool>;
///Field `RESET` writer - RESET
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
///Field `DBF` reader - DBF
pub type DBF_R = crate::BitReader<bool>;
///Field `DBF` writer - DBF
pub type DBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - INRQ
    #[inline(always)]
    pub fn inrq(&self) -> INRQ_R {
        INRQ_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SLEEP
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TXFP
    #[inline(always)]
    pub fn txfp(&self) -> TXFP_R {
        TXFP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - RFLM
    #[inline(always)]
    pub fn rflm(&self) -> RFLM_R {
        RFLM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NART
    #[inline(always)]
    pub fn nart(&self) -> NART_R {
        NART_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - AWUM
    #[inline(always)]
    pub fn awum(&self) -> AWUM_R {
        AWUM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - ABOM
    #[inline(always)]
    pub fn abom(&self) -> ABOM_R {
        ABOM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TTCM
    #[inline(always)]
    pub fn ttcm(&self) -> TTCM_R {
        TTCM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 15 - RESET
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - DBF
    #[inline(always)]
    pub fn dbf(&self) -> DBF_R {
        DBF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - INRQ
    #[inline(always)]
    #[must_use]
    pub fn inrq(&mut self) -> INRQ_W<0> {
        INRQ_W::new(self)
    }
    ///Bit 1 - SLEEP
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<1> {
        SLEEP_W::new(self)
    }
    ///Bit 2 - TXFP
    #[inline(always)]
    #[must_use]
    pub fn txfp(&mut self) -> TXFP_W<2> {
        TXFP_W::new(self)
    }
    ///Bit 3 - RFLM
    #[inline(always)]
    #[must_use]
    pub fn rflm(&mut self) -> RFLM_W<3> {
        RFLM_W::new(self)
    }
    ///Bit 4 - NART
    #[inline(always)]
    #[must_use]
    pub fn nart(&mut self) -> NART_W<4> {
        NART_W::new(self)
    }
    ///Bit 5 - AWUM
    #[inline(always)]
    #[must_use]
    pub fn awum(&mut self) -> AWUM_W<5> {
        AWUM_W::new(self)
    }
    ///Bit 6 - ABOM
    #[inline(always)]
    #[must_use]
    pub fn abom(&mut self) -> ABOM_W<6> {
        ABOM_W::new(self)
    }
    ///Bit 7 - TTCM
    #[inline(always)]
    #[must_use]
    pub fn ttcm(&mut self) -> TTCM_W<7> {
        TTCM_W::new(self)
    }
    ///Bit 15 - RESET
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<15> {
        RESET_W::new(self)
    }
    ///Bit 16 - DBF
    #[inline(always)]
    #[must_use]
    pub fn dbf(&mut self) -> DBF_W<16> {
        DBF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CAN_MCR
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mcr](index.html) module
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mcr::R](R) reader structure
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mcr::W](W) writer structure
impl crate::Writable for MCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MCR to value 0
impl crate::Resettable for MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
