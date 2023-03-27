///Register `FLASH_SECWM2R1` reader
pub struct R(crate::R<FLASH_SECWM2R1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_SECWM2R1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_SECWM2R1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_SECWM2R1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLASH_SECWM2R1` writer
pub struct W(crate::W<FLASH_SECWM2R1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_SECWM2R1_SPEC>;
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
impl From<crate::W<FLASH_SECWM2R1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_SECWM2R1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SECWM2_PSTRT` reader - Start page of second secure area This field contains the first page of the secure area in bank 2.
pub type SECWM2_PSTRT_R = crate::FieldReader<u8, u8>;
///Field `SECWM2_PSTRT` writer - Start page of second secure area This field contains the first page of the secure area in bank 2.
pub type SECWM2_PSTRT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_SECWM2R1_SPEC, u8, u8, 7, O>;
///Field `SECWM2_PEND` reader - End page of second secure area This field contains the last page of the secure area in bank 2.
pub type SECWM2_PEND_R = crate::FieldReader<u8, u8>;
///Field `SECWM2_PEND` writer - End page of second secure area This field contains the last page of the secure area in bank 2.
pub type SECWM2_PEND_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_SECWM2R1_SPEC, u8, u8, 7, O>;
impl R {
    ///Bits 0:6 - Start page of second secure area This field contains the first page of the secure area in bank 2.
    #[inline(always)]
    pub fn secwm2_pstrt(&self) -> SECWM2_PSTRT_R {
        SECWM2_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - End page of second secure area This field contains the last page of the secure area in bank 2.
    #[inline(always)]
    pub fn secwm2_pend(&self) -> SECWM2_PEND_R {
        SECWM2_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:6 - Start page of second secure area This field contains the first page of the secure area in bank 2.
    #[inline(always)]
    #[must_use]
    pub fn secwm2_pstrt(&mut self) -> SECWM2_PSTRT_W<0> {
        SECWM2_PSTRT_W::new(self)
    }
    ///Bits 16:22 - End page of second secure area This field contains the last page of the secure area in bank 2.
    #[inline(always)]
    #[must_use]
    pub fn secwm2_pend(&mut self) -> SECWM2_PEND_W<16> {
        SECWM2_PEND_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH secure watermark2 register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flash_secwm2r1](index.html) module
pub struct FLASH_SECWM2R1_SPEC;
impl crate::RegisterSpec for FLASH_SECWM2R1_SPEC {
    type Ux = u32;
}
///`read()` method returns [flash_secwm2r1::R](R) reader structure
impl crate::Readable for FLASH_SECWM2R1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [flash_secwm2r1::W](W) writer structure
impl crate::Writable for FLASH_SECWM2R1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLASH_SECWM2R1 to value 0xff00_ff00
impl crate::Resettable for FLASH_SECWM2R1_SPEC {
    const RESET_VALUE: Self::Ux = 0xff00_ff00;
}
