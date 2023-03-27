///Register `FLASH_NSSR` reader
pub struct R(crate::R<FLASH_NSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_NSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_NSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_NSSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLASH_NSSR` writer
pub struct W(crate::W<FLASH_NSSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_NSSR_SPEC>;
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
impl From<crate::W<FLASH_NSSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_NSSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EOP` reader - Non-secure end of operation
pub type EOP_R = crate::BitReader<bool>;
///Field `EOP` writer - Non-secure end of operation
pub type EOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_NSSR_SPEC, bool, O>;
///Field `OPERR` reader - Non-secure operation error
pub type OPERR_R = crate::BitReader<bool>;
///Field `OPERR` writer - Non-secure operation error
pub type OPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_NSSR_SPEC, bool, O>;
///Field `PROGERR` reader - Non-secure programming error This bit is set by hardware when a non-secure quad-word address to be programmed contains a value different from all 1 before programming, except if the data to write is all 0. This bit is cleared by writing 1.
pub type PROGERR_R = crate::BitReader<bool>;
///Field `PROGERR` writer - Non-secure programming error This bit is set by hardware when a non-secure quad-word address to be programmed contains a value different from all 1 before programming, except if the data to write is all 0. This bit is cleared by writing 1.
pub type PROGERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_NSSR_SPEC, bool, O>;
///Field `WRPERR` reader - Non-secure write protection error This bit is set by hardware when an non-secure address to be erased/programmed belongs to a write-protected part (by WRP, HDP or RDP level 1) of the Flash memory. This bit is cleared by writing 1. Refer to for full conditions of error flag setting.
pub type WRPERR_R = crate::BitReader<bool>;
///Field `WRPERR` writer - Non-secure write protection error This bit is set by hardware when an non-secure address to be erased/programmed belongs to a write-protected part (by WRP, HDP or RDP level 1) of the Flash memory. This bit is cleared by writing 1. Refer to for full conditions of error flag setting.
pub type WRPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_NSSR_SPEC, bool, O>;
///Field `PGAERR` reader - Non-secure programming alignment error This bit is set by hardware when the first word to be programmed is not aligned with a quad-word address, or the second, third or forth word does not belong to the same quad-word address. This bit is cleared by writing 1.
pub type PGAERR_R = crate::BitReader<bool>;
///Field `PGAERR` writer - Non-secure programming alignment error This bit is set by hardware when the first word to be programmed is not aligned with a quad-word address, or the second, third or forth word does not belong to the same quad-word address. This bit is cleared by writing 1.
pub type PGAERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_NSSR_SPEC, bool, O>;
///Field `SIZERR` reader - Non-secure size error This bit is set by hardware when the size of the access is a byte or half-word during a non-secure program sequence. Only quad-word programming is allowed by means of successive word accesses. This bit is cleared by writing 1.
pub type SIZERR_R = crate::BitReader<bool>;
///Field `SIZERR` writer - Non-secure size error This bit is set by hardware when the size of the access is a byte or half-word during a non-secure program sequence. Only quad-word programming is allowed by means of successive word accesses. This bit is cleared by writing 1.
pub type SIZERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_NSSR_SPEC, bool, O>;
///Field `PGSERR` reader - Non-secure programming sequence error This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1. Refer to for full conditions of error flag setting.
pub type PGSERR_R = crate::BitReader<bool>;
///Field `PGSERR` writer - Non-secure programming sequence error This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1. Refer to for full conditions of error flag setting.
pub type PGSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_NSSR_SPEC, bool, O>;
///Field `OPTWERR` reader - Option write error This bit is set by hardware when the options bytes are written with an invalid configuration. It is cleared by writing 1. Refer to for full conditions of error flag setting.
pub type OPTWERR_R = crate::BitReader<bool>;
///Field `OPTWERR` writer - Option write error This bit is set by hardware when the options bytes are written with an invalid configuration. It is cleared by writing 1. Refer to for full conditions of error flag setting.
pub type OPTWERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_NSSR_SPEC, bool, O>;
///Field `BSY` reader - Non-secure busy This indicates that a Flash memory secure or non-secure operation is in progress. This bit is set at the beginning of a Flash operation and reset when the operation finishes or when an error occurs.
pub type BSY_R = crate::BitReader<bool>;
///Field `WDW` reader - Non-secure wait data to write This bit indicates that the Flash memory write buffer has been written by a secure or non-secure operation. It is set when the first data is stored in the buffer and cleared when the write is performed in the Flash memory.
pub type WDW_R = crate::BitReader<bool>;
///Field `OEM1LOCK` reader - OEM1 lock This bit indicates that the OEM1 RDP key read during the OBL is not virgin. When set, the OEM1 RDP lock mechanism is active.
pub type OEM1LOCK_R = crate::BitReader<bool>;
///Field `OEM2LOCK` reader - OEM2 lock This bit indicates that the OEM2 RDP key read during the OBL is not virgin. When set, the OEM2 RDP lock mechanism is active.
pub type OEM2LOCK_R = crate::BitReader<bool>;
///Field `PD1` reader - Bank 1 in power-down mode This bit indicates that the Flash memory bank 1 is in power-down state. It is reset when bank 1 is in normal mode or being awaken.
pub type PD1_R = crate::BitReader<bool>;
///Field `PD2` reader - Bank 2 in power-down mode This bit indicates that the Flash memory bank 2 is in power-down state. It is reset when bank 2 is in normal mode or being awaken.
pub type PD2_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Non-secure end of operation
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Non-secure operation error
    #[inline(always)]
    pub fn operr(&self) -> OPERR_R {
        OPERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Non-secure programming error This bit is set by hardware when a non-secure quad-word address to be programmed contains a value different from all 1 before programming, except if the data to write is all 0. This bit is cleared by writing 1.
    #[inline(always)]
    pub fn progerr(&self) -> PROGERR_R {
        PROGERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Non-secure write protection error This bit is set by hardware when an non-secure address to be erased/programmed belongs to a write-protected part (by WRP, HDP or RDP level 1) of the Flash memory. This bit is cleared by writing 1. Refer to for full conditions of error flag setting.
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Non-secure programming alignment error This bit is set by hardware when the first word to be programmed is not aligned with a quad-word address, or the second, third or forth word does not belong to the same quad-word address. This bit is cleared by writing 1.
    #[inline(always)]
    pub fn pgaerr(&self) -> PGAERR_R {
        PGAERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Non-secure size error This bit is set by hardware when the size of the access is a byte or half-word during a non-secure program sequence. Only quad-word programming is allowed by means of successive word accesses. This bit is cleared by writing 1.
    #[inline(always)]
    pub fn sizerr(&self) -> SIZERR_R {
        SIZERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Non-secure programming sequence error This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1. Refer to for full conditions of error flag setting.
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 13 - Option write error This bit is set by hardware when the options bytes are written with an invalid configuration. It is cleared by writing 1. Refer to for full conditions of error flag setting.
    #[inline(always)]
    pub fn optwerr(&self) -> OPTWERR_R {
        OPTWERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - Non-secure busy This indicates that a Flash memory secure or non-secure operation is in progress. This bit is set at the beginning of a Flash operation and reset when the operation finishes or when an error occurs.
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Non-secure wait data to write This bit indicates that the Flash memory write buffer has been written by a secure or non-secure operation. It is set when the first data is stored in the buffer and cleared when the write is performed in the Flash memory.
    #[inline(always)]
    pub fn wdw(&self) -> WDW_R {
        WDW_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - OEM1 lock This bit indicates that the OEM1 RDP key read during the OBL is not virgin. When set, the OEM1 RDP lock mechanism is active.
    #[inline(always)]
    pub fn oem1lock(&self) -> OEM1LOCK_R {
        OEM1LOCK_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - OEM2 lock This bit indicates that the OEM2 RDP key read during the OBL is not virgin. When set, the OEM2 RDP lock mechanism is active.
    #[inline(always)]
    pub fn oem2lock(&self) -> OEM2LOCK_R {
        OEM2LOCK_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Bank 1 in power-down mode This bit indicates that the Flash memory bank 1 is in power-down state. It is reset when bank 1 is in normal mode or being awaken.
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Bank 2 in power-down mode This bit indicates that the Flash memory bank 2 is in power-down state. It is reset when bank 2 is in normal mode or being awaken.
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Non-secure end of operation
    #[inline(always)]
    #[must_use]
    pub fn eop(&mut self) -> EOP_W<0> {
        EOP_W::new(self)
    }
    ///Bit 1 - Non-secure operation error
    #[inline(always)]
    #[must_use]
    pub fn operr(&mut self) -> OPERR_W<1> {
        OPERR_W::new(self)
    }
    ///Bit 3 - Non-secure programming error This bit is set by hardware when a non-secure quad-word address to be programmed contains a value different from all 1 before programming, except if the data to write is all 0. This bit is cleared by writing 1.
    #[inline(always)]
    #[must_use]
    pub fn progerr(&mut self) -> PROGERR_W<3> {
        PROGERR_W::new(self)
    }
    ///Bit 4 - Non-secure write protection error This bit is set by hardware when an non-secure address to be erased/programmed belongs to a write-protected part (by WRP, HDP or RDP level 1) of the Flash memory. This bit is cleared by writing 1. Refer to for full conditions of error flag setting.
    #[inline(always)]
    #[must_use]
    pub fn wrperr(&mut self) -> WRPERR_W<4> {
        WRPERR_W::new(self)
    }
    ///Bit 5 - Non-secure programming alignment error This bit is set by hardware when the first word to be programmed is not aligned with a quad-word address, or the second, third or forth word does not belong to the same quad-word address. This bit is cleared by writing 1.
    #[inline(always)]
    #[must_use]
    pub fn pgaerr(&mut self) -> PGAERR_W<5> {
        PGAERR_W::new(self)
    }
    ///Bit 6 - Non-secure size error This bit is set by hardware when the size of the access is a byte or half-word during a non-secure program sequence. Only quad-word programming is allowed by means of successive word accesses. This bit is cleared by writing 1.
    #[inline(always)]
    #[must_use]
    pub fn sizerr(&mut self) -> SIZERR_W<6> {
        SIZERR_W::new(self)
    }
    ///Bit 7 - Non-secure programming sequence error This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1. Refer to for full conditions of error flag setting.
    #[inline(always)]
    #[must_use]
    pub fn pgserr(&mut self) -> PGSERR_W<7> {
        PGSERR_W::new(self)
    }
    ///Bit 13 - Option write error This bit is set by hardware when the options bytes are written with an invalid configuration. It is cleared by writing 1. Refer to for full conditions of error flag setting.
    #[inline(always)]
    #[must_use]
    pub fn optwerr(&mut self) -> OPTWERR_W<13> {
        OPTWERR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH non-secure status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flash_nssr](index.html) module
pub struct FLASH_NSSR_SPEC;
impl crate::RegisterSpec for FLASH_NSSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [flash_nssr::R](R) reader structure
impl crate::Readable for FLASH_NSSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [flash_nssr::W](W) writer structure
impl crate::Writable for FLASH_NSSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLASH_NSSR to value 0
impl crate::Resettable for FLASH_NSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
