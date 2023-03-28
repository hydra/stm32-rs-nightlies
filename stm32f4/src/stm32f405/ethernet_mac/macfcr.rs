///Register `MACFCR` reader
pub struct R(crate::R<MACFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACFCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACFCR` writer
pub struct W(crate::W<MACFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACFCR_SPEC>;
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
impl From<crate::W<MACFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACFCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FCB` reader - FCB
pub type FCB_R = crate::BitReader<bool>;
///Field `FCB` writer - FCB
pub type FCB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFCR_SPEC, bool, O>;
///Field `TFCE` reader - TFCE
pub type TFCE_R = crate::BitReader<bool>;
///Field `TFCE` writer - TFCE
pub type TFCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFCR_SPEC, bool, O>;
///Field `RFCE` reader - RFCE
pub type RFCE_R = crate::BitReader<bool>;
///Field `RFCE` writer - RFCE
pub type RFCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFCR_SPEC, bool, O>;
///Field `UPFD` reader - UPFD
pub type UPFD_R = crate::BitReader<bool>;
///Field `UPFD` writer - UPFD
pub type UPFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFCR_SPEC, bool, O>;
///Field `PLT` reader - PLT
pub type PLT_R = crate::FieldReader<u8, u8>;
///Field `PLT` writer - PLT
pub type PLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACFCR_SPEC, u8, u8, 2, O>;
///Field `ZQPD` reader - ZQPD
pub type ZQPD_R = crate::BitReader<bool>;
///Field `ZQPD` writer - ZQPD
pub type ZQPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFCR_SPEC, bool, O>;
///Field `PT` reader - PT
pub type PT_R = crate::FieldReader<u16, u16>;
///Field `PT` writer - PT
pub type PT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACFCR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bit 0 - FCB
    #[inline(always)]
    pub fn fcb(&self) -> FCB_R {
        FCB_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TFCE
    #[inline(always)]
    pub fn tfce(&self) -> TFCE_R {
        TFCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RFCE
    #[inline(always)]
    pub fn rfce(&self) -> RFCE_R {
        RFCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - UPFD
    #[inline(always)]
    pub fn upfd(&self) -> UPFD_R {
        UPFD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - PLT
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 7 - ZQPD
    #[inline(always)]
    pub fn zqpd(&self) -> ZQPD_R {
        ZQPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 16:31 - PT
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bit 0 - FCB
    #[inline(always)]
    #[must_use]
    pub fn fcb(&mut self) -> FCB_W<0> {
        FCB_W::new(self)
    }
    ///Bit 1 - TFCE
    #[inline(always)]
    #[must_use]
    pub fn tfce(&mut self) -> TFCE_W<1> {
        TFCE_W::new(self)
    }
    ///Bit 2 - RFCE
    #[inline(always)]
    #[must_use]
    pub fn rfce(&mut self) -> RFCE_W<2> {
        RFCE_W::new(self)
    }
    ///Bit 3 - UPFD
    #[inline(always)]
    #[must_use]
    pub fn upfd(&mut self) -> UPFD_W<3> {
        UPFD_W::new(self)
    }
    ///Bits 4:5 - PLT
    #[inline(always)]
    #[must_use]
    pub fn plt(&mut self) -> PLT_W<4> {
        PLT_W::new(self)
    }
    ///Bit 7 - ZQPD
    #[inline(always)]
    #[must_use]
    pub fn zqpd(&mut self) -> ZQPD_W<7> {
        ZQPD_W::new(self)
    }
    ///Bits 16:31 - PT
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PT_W<16> {
        PT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet MAC flow control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macfcr](index.html) module
pub struct MACFCR_SPEC;
impl crate::RegisterSpec for MACFCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macfcr::R](R) reader structure
impl crate::Readable for MACFCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macfcr::W](W) writer structure
impl crate::Writable for MACFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACFCR to value 0
impl crate::Resettable for MACFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
