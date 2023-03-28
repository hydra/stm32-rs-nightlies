///Register `PCROP1ER` reader
pub struct R(crate::R<PCROP1ER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCROP1ER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCROP1ER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCROP1ER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PCROP1ER` writer
pub struct W(crate::W<PCROP1ER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCROP1ER_SPEC>;
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
impl From<crate::W<PCROP1ER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCROP1ER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PCROP1_END` reader - Bank 1 PCROP area end offset
pub type PCROP1_END_R = crate::FieldReader<u16, u16>;
///Field `PCROP1_END` writer - Bank 1 PCROP area end offset
pub type PCROP1_END_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PCROP1ER_SPEC, u16, u16, 15, O>;
///Field `PCROP_RDP` reader - PCROP area preserved when RDP level decreased
pub type PCROP_RDP_R = crate::BitReader<bool>;
///Field `PCROP_RDP` writer - PCROP area preserved when RDP level decreased
pub type PCROP_RDP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCROP1ER_SPEC, bool, O>;
impl R {
    ///Bits 0:14 - Bank 1 PCROP area end offset
    #[inline(always)]
    pub fn pcrop1_end(&self) -> PCROP1_END_R {
        PCROP1_END_R::new((self.bits & 0x7fff) as u16)
    }
    ///Bit 31 - PCROP area preserved when RDP level decreased
    #[inline(always)]
    pub fn pcrop_rdp(&self) -> PCROP_RDP_R {
        PCROP_RDP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:14 - Bank 1 PCROP area end offset
    #[inline(always)]
    #[must_use]
    pub fn pcrop1_end(&mut self) -> PCROP1_END_W<0> {
        PCROP1_END_W::new(self)
    }
    ///Bit 31 - PCROP area preserved when RDP level decreased
    #[inline(always)]
    #[must_use]
    pub fn pcrop_rdp(&mut self) -> PCROP_RDP_W<31> {
        PCROP_RDP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flash Bank 1 PCROP End address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pcrop1er](index.html) module
pub struct PCROP1ER_SPEC;
impl crate::RegisterSpec for PCROP1ER_SPEC {
    type Ux = u32;
}
///`read()` method returns [pcrop1er::R](R) reader structure
impl crate::Readable for PCROP1ER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pcrop1er::W](W) writer structure
impl crate::Writable for PCROP1ER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PCROP1ER to value 0x0fff_0000
impl crate::Resettable for PCROP1ER_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_0000;
}
