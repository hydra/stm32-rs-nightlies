///Register `FLASH_WRP1AR` reader
pub struct R(crate::R<FLASH_WRP1AR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_WRP1AR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_WRP1AR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_WRP1AR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLASH_WRP1AR` writer
pub struct W(crate::W<FLASH_WRP1AR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_WRP1AR_SPEC>;
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
impl From<crate::W<FLASH_WRP1AR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_WRP1AR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WRP1A_PSTRT` reader - bank 1 WPR first area A start page This field contains the first page of the first WPR area for bank 1.
pub type WRP1A_PSTRT_R = crate::FieldReader<u8, u8>;
///Field `WRP1A_PSTRT` writer - bank 1 WPR first area A start page This field contains the first page of the first WPR area for bank 1.
pub type WRP1A_PSTRT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_WRP1AR_SPEC, u8, u8, 7, O>;
///Field `WRP1A_PEND` reader - Bank 1 WPR first area A end page This field contains the last page of the first WPR area in bank 1.
pub type WRP1A_PEND_R = crate::FieldReader<u8, u8>;
///Field `WRP1A_PEND` writer - Bank 1 WPR first area A end page This field contains the last page of the first WPR area in bank 1.
pub type WRP1A_PEND_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_WRP1AR_SPEC, u8, u8, 7, O>;
///Field `UNLOCK` reader - Bank 1 WPR first area A unlock
pub type UNLOCK_R = crate::BitReader<bool>;
///Field `UNLOCK` writer - Bank 1 WPR first area A unlock
pub type UNLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_WRP1AR_SPEC, bool, O>;
impl R {
    ///Bits 0:6 - bank 1 WPR first area A start page This field contains the first page of the first WPR area for bank 1.
    #[inline(always)]
    pub fn wrp1a_pstrt(&self) -> WRP1A_PSTRT_R {
        WRP1A_PSTRT_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:22 - Bank 1 WPR first area A end page This field contains the last page of the first WPR area in bank 1.
    #[inline(always)]
    pub fn wrp1a_pend(&self) -> WRP1A_PEND_R {
        WRP1A_PEND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    ///Bit 31 - Bank 1 WPR first area A unlock
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:6 - bank 1 WPR first area A start page This field contains the first page of the first WPR area for bank 1.
    #[inline(always)]
    #[must_use]
    pub fn wrp1a_pstrt(&mut self) -> WRP1A_PSTRT_W<0> {
        WRP1A_PSTRT_W::new(self)
    }
    ///Bits 16:22 - Bank 1 WPR first area A end page This field contains the last page of the first WPR area in bank 1.
    #[inline(always)]
    #[must_use]
    pub fn wrp1a_pend(&mut self) -> WRP1A_PEND_W<16> {
        WRP1A_PEND_W::new(self)
    }
    ///Bit 31 - Bank 1 WPR first area A unlock
    #[inline(always)]
    #[must_use]
    pub fn unlock(&mut self) -> UNLOCK_W<31> {
        UNLOCK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH WRP1 area A address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flash_wrp1ar](index.html) module
pub struct FLASH_WRP1AR_SPEC;
impl crate::RegisterSpec for FLASH_WRP1AR_SPEC {
    type Ux = u32;
}
///`read()` method returns [flash_wrp1ar::R](R) reader structure
impl crate::Readable for FLASH_WRP1AR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [flash_wrp1ar::W](W) writer structure
impl crate::Writable for FLASH_WRP1AR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLASH_WRP1AR to value 0x0f00_ff00
impl crate::Resettable for FLASH_WRP1AR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f00_ff00;
}
