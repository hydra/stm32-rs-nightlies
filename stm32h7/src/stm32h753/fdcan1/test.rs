///Register `TEST` reader
pub struct R(crate::R<TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TEST` writer
pub struct W(crate::W<TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_SPEC>;
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
impl From<crate::W<TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LBCK` reader - Loop Back mode
pub type LBCK_R = crate::BitReader<bool>;
///Field `LBCK` writer - Loop Back mode
pub type LBCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_SPEC, bool, O>;
///Field `TX` reader - Loop Back mode
pub type TX_R = crate::FieldReader<u8, u8>;
///Field `TX` writer - Loop Back mode
pub type TX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEST_SPEC, u8, u8, 2, O>;
///Field `RX` reader - Control of Transmit Pin
pub type RX_R = crate::BitReader<bool>;
///Field `RX` writer - Control of Transmit Pin
pub type RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_SPEC, bool, O>;
impl R {
    ///Bit 4 - Loop Back mode
    #[inline(always)]
    pub fn lbck(&self) -> LBCK_R {
        LBCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:6 - Loop Back mode
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - Control of Transmit Pin
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 4 - Loop Back mode
    #[inline(always)]
    #[must_use]
    pub fn lbck(&mut self) -> LBCK_W<4> {
        LBCK_W::new(self)
    }
    ///Bits 5:6 - Loop Back mode
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<5> {
        TX_W::new(self)
    }
    ///Bit 7 - Control of Transmit Pin
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RX_W<7> {
        RX_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Test Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [test](index.html) module
pub struct TEST_SPEC;
impl crate::RegisterSpec for TEST_SPEC {
    type Ux = u32;
}
///`read()` method returns [test::R](R) reader structure
impl crate::Readable for TEST_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [test::W](W) writer structure
impl crate::Writable for TEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TEST to value 0
impl crate::Resettable for TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
