///Register `FLASH_SECWM1R1` reader
pub struct R(crate::R<FLASH_SECWM1R1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_SECWM1R1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_SECWM1R1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_SECWM1R1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLASH_SECWM1R1` writer
pub struct W(crate::W<FLASH_SECWM1R1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_SECWM1R1_SPEC>;
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
impl From<crate::W<FLASH_SECWM1R1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_SECWM1R1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SECWM1_PSTRT` reader - Start page of first secure area This field contains the first page of the secure area in bank 1.
pub type SECWM1_PSTRT_R = crate::FieldReader<u8, u8>;
///Field `SECWM1_PSTRT` writer - Start page of first secure area This field contains the first page of the secure area in bank 1.
pub type SECWM1_PSTRT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_SECWM1R1_SPEC, u8, u8, 7, O>;
///Field `SECWM1_PEND` reader - End page of first secure area This field contains the last page of the secure area in bank 1.
pub type SECWM1_PEND_R = crate::FieldReader<u8, u8>;
///Field `SECWM1_PEND` writer - End page of first secure area This field contains the last page of the secure area in bank 1.
pub type SECWM1_PEND_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_SECWM1R1_SPEC, u8, u8, 7, O>;
impl R {
    ///Bits 0:6 - Start page of first secure area This field contains the first page of the secure area in bank 1.
    #[inline(always)]
    pub fn secwm1_pstrt(&self) -> SECWM1_PSTRT_R {
        SECWM1_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - End page of first secure area This field contains the last page of the secure area in bank 1.
    #[inline(always)]
    pub fn secwm1_pend(&self) -> SECWM1_PEND_R {
        SECWM1_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:6 - Start page of first secure area This field contains the first page of the secure area in bank 1.
    #[inline(always)]
    #[must_use]
    pub fn secwm1_pstrt(&mut self) -> SECWM1_PSTRT_W<0> {
        SECWM1_PSTRT_W::new(self)
    }
    ///Bits 16:22 - End page of first secure area This field contains the last page of the secure area in bank 1.
    #[inline(always)]
    #[must_use]
    pub fn secwm1_pend(&mut self) -> SECWM1_PEND_W<16> {
        SECWM1_PEND_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH secure watermark1 register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flash_secwm1r1](index.html) module
pub struct FLASH_SECWM1R1_SPEC;
impl crate::RegisterSpec for FLASH_SECWM1R1_SPEC {
    type Ux = u32;
}
///`read()` method returns [flash_secwm1r1::R](R) reader structure
impl crate::Readable for FLASH_SECWM1R1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [flash_secwm1r1::W](W) writer structure
impl crate::Writable for FLASH_SECWM1R1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLASH_SECWM1R1 to value 0xff00_ff00
impl crate::Resettable for FLASH_SECWM1R1_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00_ff00;
}
