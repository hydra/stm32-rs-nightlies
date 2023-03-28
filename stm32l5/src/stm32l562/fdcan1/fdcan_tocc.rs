///Register `FDCAN_TOCC` reader
pub struct R(crate::R<FDCAN_TOCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TOCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TOCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TOCC_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FDCAN_TOCC` writer
pub struct W(crate::W<FDCAN_TOCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TOCC_SPEC>;
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
impl From<crate::W<FDCAN_TOCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TOCC_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ETOC` reader - Enable Timeout Counter
pub type ETOC_R = crate::BitReader<bool>;
///Field `ETOC` writer - Enable Timeout Counter
pub type ETOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TOCC_SPEC, bool, O>;
///Field `TOS` reader - Timeout Select
pub type TOS_R = crate::FieldReader<u8, u8>;
///Field `TOS` writer - Timeout Select
pub type TOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TOCC_SPEC, u8, u8, 2, O>;
///Field `TOP` reader - Timeout Period
pub type TOP_R = crate::FieldReader<u16, u16>;
///Field `TOP` writer - Timeout Period
pub type TOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TOCC_SPEC, u16, u16, 16, O>;
impl R {
    ///Bit 0 - Enable Timeout Counter
    #[inline(always)]
    pub fn etoc(&self) -> ETOC_R {
        ETOC_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Timeout Select
    #[inline(always)]
    pub fn tos(&self) -> TOS_R {
        TOS_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 16:31 - Timeout Period
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bit 0 - Enable Timeout Counter
    #[inline(always)]
    #[must_use]
    pub fn etoc(&mut self) -> ETOC_W<0> {
        ETOC_W::new(self)
    }
    ///Bits 1:2 - Timeout Select
    #[inline(always)]
    #[must_use]
    pub fn tos(&mut self) -> TOS_W<1> {
        TOS_W::new(self)
    }
    ///Bits 16:31 - Timeout Period
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TOP_W<16> {
        TOP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Timeout Counter Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_tocc](index.html) module
pub struct FDCAN_TOCC_SPEC;
impl crate::RegisterSpec for FDCAN_TOCC_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_tocc::R](R) reader structure
impl crate::Readable for FDCAN_TOCC_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fdcan_tocc::W](W) writer structure
impl crate::Writable for FDCAN_TOCC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FDCAN_TOCC to value 0xffff_0000
impl crate::Resettable for FDCAN_TOCC_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_0000;
}
