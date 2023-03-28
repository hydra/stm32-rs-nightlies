///Register `TXBC` reader
pub struct R(crate::R<TXBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBC_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TXBC` writer
pub struct W(crate::W<TXBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBC_SPEC>;
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
impl From<crate::W<TXBC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBC_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TBSA` reader - TBSA
pub type TBSA_R = crate::FieldReader<u16, u16>;
///Field `TBSA` writer - TBSA
pub type TBSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXBC_SPEC, u16, u16, 14, O>;
///Field `NDTB` reader - NDTB
pub type NDTB_R = crate::FieldReader<u8, u8>;
///Field `NDTB` writer - NDTB
pub type NDTB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXBC_SPEC, u8, u8, 6, O>;
///Field `TFQS` reader - TFQS
pub type TFQS_R = crate::FieldReader<u8, u8>;
///Field `TFQS` writer - TFQS
pub type TFQS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXBC_SPEC, u8, u8, 6, O>;
///Field `TFQM` reader - TFQM
pub type TFQM_R = crate::BitReader<bool>;
///Field `TFQM` writer - TFQM
pub type TFQM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBC_SPEC, bool, O>;
impl R {
    ///Bits 2:15 - TBSA
    #[inline(always)]
    pub fn tbsa(&self) -> TBSA_R {
        TBSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    ///Bits 16:21 - NDTB
    #[inline(always)]
    pub fn ndtb(&self) -> NDTB_R {
        NDTB_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:29 - TFQS
    #[inline(always)]
    pub fn tfqs(&self) -> TFQS_R {
        TFQS_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    ///Bit 30 - TFQM
    #[inline(always)]
    pub fn tfqm(&self) -> TFQM_R {
        TFQM_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bits 2:15 - TBSA
    #[inline(always)]
    #[must_use]
    pub fn tbsa(&mut self) -> TBSA_W<2> {
        TBSA_W::new(self)
    }
    ///Bits 16:21 - NDTB
    #[inline(always)]
    #[must_use]
    pub fn ndtb(&mut self) -> NDTB_W<16> {
        NDTB_W::new(self)
    }
    ///Bits 24:29 - TFQS
    #[inline(always)]
    #[must_use]
    pub fn tfqs(&mut self) -> TFQS_W<24> {
        TFQS_W::new(self)
    }
    ///Bit 30 - TFQM
    #[inline(always)]
    #[must_use]
    pub fn tfqm(&mut self) -> TFQM_W<30> {
        TFQM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN Tx Buffer Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [txbc](index.html) module
pub struct TXBC_SPEC;
impl crate::RegisterSpec for TXBC_SPEC {
    type Ux = u32;
}
///`read()` method returns [txbc::R](R) reader structure
impl crate::Readable for TXBC_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [txbc::W](W) writer structure
impl crate::Writable for TXBC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TXBC to value 0
impl crate::Resettable for TXBC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
