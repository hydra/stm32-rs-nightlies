///Register `FDCAN_TEST` reader
pub struct R(crate::R<FDCAN_TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TEST_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FDCAN_TEST` writer
pub struct W(crate::W<FDCAN_TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TEST_SPEC>;
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
impl From<crate::W<FDCAN_TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TEST_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LBCK` reader - Loop back mode
pub type LBCK_R = crate::BitReader<bool>;
///Field `LBCK` writer - Loop back mode
pub type LBCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TEST_SPEC, bool, O>;
///Field `TX` reader - Control of transmit pin
pub type TX_R = crate::FieldReader<u8, u8>;
///Field `TX` writer - Control of transmit pin
pub type TX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TEST_SPEC, u8, u8, 2, O>;
///Field `RX` reader - Receive pin Monitors the actual value of pin FDCANx_RX
pub type RX_R = crate::BitReader<bool>;
impl R {
    ///Bit 4 - Loop back mode
    #[inline(always)]
    pub fn lbck(&self) -> LBCK_R {
        LBCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - Control of transmit pin
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - Receive pin Monitors the actual value of pin FDCANx_RX
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 4 - Loop back mode
    #[inline(always)]
    #[must_use]
    pub fn lbck(&mut self) -> LBCK_W<4> {
        LBCK_W::new(self)
    }
    ///Bits 5:6 - Control of transmit pin
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<5> {
        TX_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN test register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_test](index.html) module
pub struct FDCAN_TEST_SPEC;
impl crate::RegisterSpec for FDCAN_TEST_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_test::R](R) reader structure
impl crate::Readable for FDCAN_TEST_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fdcan_test::W](W) writer structure
impl crate::Writable for FDCAN_TEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FDCAN_TEST to value 0
impl crate::Resettable for FDCAN_TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
