///Register `ASLOTR` reader
pub struct R(crate::R<ASLOTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASLOTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASLOTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASLOTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ASLOTR` writer
pub struct W(crate::W<ASLOTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASLOTR_SPEC>;
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
impl From<crate::W<ASLOTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASLOTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FBOFF` reader - First bit offset These bits are set and cleared by software. The value set in this bitfield defines the position of the first data transfer bit in the slot. It represents an offset value. In transmission mode, the bits outside the data field are forced to 0. In reception mode, the extra received bits are discarded. These bits must be set when the audio block is disabled. They are ignored in AC’97 or SPDIF mode.
pub type FBOFF_R = crate::FieldReader<u8, u8>;
///Field `FBOFF` writer - First bit offset These bits are set and cleared by software. The value set in this bitfield defines the position of the first data transfer bit in the slot. It represents an offset value. In transmission mode, the bits outside the data field are forced to 0. In reception mode, the extra received bits are discarded. These bits must be set when the audio block is disabled. They are ignored in AC’97 or SPDIF mode.
pub type FBOFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ASLOTR_SPEC, u8, u8, 5, O>;
///Field `SLOTSZ` reader - Slot size This bits is set and cleared by software. The slot size must be higher or equal to the data size. If this condition is not respected, the behavior of the SAI is undetermined. Refer to for information on how to drive SD line. These bits must be set when the audio block is disabled. They are ignored in AC’97 or SPDIF mode.
pub type SLOTSZ_R = crate::FieldReader<u8, SLOTSZ_A>;
///Slot size This bits is set and cleared by software. The slot size must be higher or equal to the data size. If this condition is not respected, the behavior of the SAI is undetermined. Refer to for information on how to drive SD line. These bits must be set when the audio block is disabled. They are ignored in AC’97 or SPDIF mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SLOTSZ_A {
    ///0: The slot size is equivalent to the data size (specified in DS\[3:0\]
    ///in the SAI_xCR1 register)
    DataSize = 0,
    ///1: 16-bit
    Bit16 = 1,
    ///2: 32-bit
    Bit32 = 2,
}
impl From<SLOTSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: SLOTSZ_A) -> Self {
        variant as _
    }
}
impl SLOTSZ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SLOTSZ_A> {
        match self.bits {
            0 => Some(SLOTSZ_A::DataSize),
            1 => Some(SLOTSZ_A::Bit16),
            2 => Some(SLOTSZ_A::Bit32),
            _ => None,
        }
    }
    ///Checks if the value of the field is `DataSize`
    #[inline(always)]
    pub fn is_data_size(&self) -> bool {
        *self == SLOTSZ_A::DataSize
    }
    ///Checks if the value of the field is `Bit16`
    #[inline(always)]
    pub fn is_bit16(&self) -> bool {
        *self == SLOTSZ_A::Bit16
    }
    ///Checks if the value of the field is `Bit32`
    #[inline(always)]
    pub fn is_bit32(&self) -> bool {
        *self == SLOTSZ_A::Bit32
    }
}
///Field `SLOTSZ` writer - Slot size This bits is set and cleared by software. The slot size must be higher or equal to the data size. If this condition is not respected, the behavior of the SAI is undetermined. Refer to for information on how to drive SD line. These bits must be set when the audio block is disabled. They are ignored in AC’97 or SPDIF mode.
pub type SLOTSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ASLOTR_SPEC, u8, SLOTSZ_A, 2, O>;
impl<'a, const O: u8> SLOTSZ_W<'a, O> {
    ///The slot size is equivalent to the data size (specified in DS\[3:0\]
    ///in the SAI_xCR1 register)
    #[inline(always)]
    pub fn data_size(self) -> &'a mut W {
        self.variant(SLOTSZ_A::DataSize)
    }
    ///16-bit
    #[inline(always)]
    pub fn bit16(self) -> &'a mut W {
        self.variant(SLOTSZ_A::Bit16)
    }
    ///32-bit
    #[inline(always)]
    pub fn bit32(self) -> &'a mut W {
        self.variant(SLOTSZ_A::Bit32)
    }
}
///Field `NBSLOT` reader - Number of slots in an audio frame. These bits are set and cleared by software. The value set in this bitfield represents the number of slots + 1 in the audio frame (including the number of inactive slots). The maximum number of slots is 16. The number of slots should be even if FSDEF bit in the SAI_xFRCR register is set. The number of slots must be configured when the audio block is disabled. They are ignored in AC’97 or SPDIF mode.
pub type NBSLOT_R = crate::FieldReader<u8, u8>;
///Field `NBSLOT` writer - Number of slots in an audio frame. These bits are set and cleared by software. The value set in this bitfield represents the number of slots + 1 in the audio frame (including the number of inactive slots). The maximum number of slots is 16. The number of slots should be even if FSDEF bit in the SAI_xFRCR register is set. The number of slots must be configured when the audio block is disabled. They are ignored in AC’97 or SPDIF mode.
pub type NBSLOT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ASLOTR_SPEC, u8, u8, 4, O>;
///Field `SLOTEN` reader - Slot enable. These bits are set and cleared by software. Each SLOTEN bit corresponds to a slot position from 0 to 15 (maximum 16 slots). The slot must be enabled when the audio block is disabled. They are ignored in AC’97 or SPDIF mode.
pub type SLOTEN_R = crate::FieldReader<u16, SLOTEN_A>;
///Slot enable. These bits are set and cleared by software. Each SLOTEN bit corresponds to a slot position from 0 to 15 (maximum 16 slots). The slot must be enabled when the audio block is disabled. They are ignored in AC’97 or SPDIF mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum SLOTEN_A {
    ///0: Inactive slot
    Inactive = 0,
    ///1: Active slot
    Active = 1,
}
impl From<SLOTEN_A> for u16 {
    #[inline(always)]
    fn from(variant: SLOTEN_A) -> Self {
        variant as _
    }
}
impl SLOTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SLOTEN_A> {
        match self.bits {
            0 => Some(SLOTEN_A::Inactive),
            1 => Some(SLOTEN_A::Active),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Inactive`
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == SLOTEN_A::Inactive
    }
    ///Checks if the value of the field is `Active`
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == SLOTEN_A::Active
    }
}
///Field `SLOTEN` writer - Slot enable. These bits are set and cleared by software. Each SLOTEN bit corresponds to a slot position from 0 to 15 (maximum 16 slots). The slot must be enabled when the audio block is disabled. They are ignored in AC’97 or SPDIF mode.
pub type SLOTEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ASLOTR_SPEC, u16, SLOTEN_A, 16, O>;
impl<'a, const O: u8> SLOTEN_W<'a, O> {
    ///Inactive slot
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(SLOTEN_A::Inactive)
    }
    ///Active slot
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(SLOTEN_A::Active)
    }
}
impl R {
    ///Bits 0:4 - First bit offset These bits are set and cleared by software. The value set in this bitfield defines the position of the first data transfer bit in the slot. It represents an offset value. In transmission mode, the bits outside the data field are forced to 0. In reception mode, the extra received bits are discarded. These bits must be set when the audio block is disabled. They are ignored in AC’97 or SPDIF mode.
    #[inline(always)]
    pub fn fboff(&self) -> FBOFF_R {
        FBOFF_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 6:7 - Slot size This bits is set and cleared by software. The slot size must be higher or equal to the data size. If this condition is not respected, the behavior of the SAI is undetermined. Refer to for information on how to drive SD line. These bits must be set when the audio block is disabled. They are ignored in AC’97 or SPDIF mode.
    #[inline(always)]
    pub fn slotsz(&self) -> SLOTSZ_R {
        SLOTSZ_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:11 - Number of slots in an audio frame. These bits are set and cleared by software. The value set in this bitfield represents the number of slots + 1 in the audio frame (including the number of inactive slots). The maximum number of slots is 16. The number of slots should be even if FSDEF bit in the SAI_xFRCR register is set. The number of slots must be configured when the audio block is disabled. They are ignored in AC’97 or SPDIF mode.
    #[inline(always)]
    pub fn nbslot(&self) -> NBSLOT_R {
        NBSLOT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:31 - Slot enable. These bits are set and cleared by software. Each SLOTEN bit corresponds to a slot position from 0 to 15 (maximum 16 slots). The slot must be enabled when the audio block is disabled. They are ignored in AC’97 or SPDIF mode.
    #[inline(always)]
    pub fn sloten(&self) -> SLOTEN_R {
        SLOTEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:4 - First bit offset These bits are set and cleared by software. The value set in this bitfield defines the position of the first data transfer bit in the slot. It represents an offset value. In transmission mode, the bits outside the data field are forced to 0. In reception mode, the extra received bits are discarded. These bits must be set when the audio block is disabled. They are ignored in AC’97 or SPDIF mode.
    #[inline(always)]
    #[must_use]
    pub fn fboff(&mut self) -> FBOFF_W<0> {
        FBOFF_W::new(self)
    }
    ///Bits 6:7 - Slot size This bits is set and cleared by software. The slot size must be higher or equal to the data size. If this condition is not respected, the behavior of the SAI is undetermined. Refer to for information on how to drive SD line. These bits must be set when the audio block is disabled. They are ignored in AC’97 or SPDIF mode.
    #[inline(always)]
    #[must_use]
    pub fn slotsz(&mut self) -> SLOTSZ_W<6> {
        SLOTSZ_W::new(self)
    }
    ///Bits 8:11 - Number of slots in an audio frame. These bits are set and cleared by software. The value set in this bitfield represents the number of slots + 1 in the audio frame (including the number of inactive slots). The maximum number of slots is 16. The number of slots should be even if FSDEF bit in the SAI_xFRCR register is set. The number of slots must be configured when the audio block is disabled. They are ignored in AC’97 or SPDIF mode.
    #[inline(always)]
    #[must_use]
    pub fn nbslot(&mut self) -> NBSLOT_W<8> {
        NBSLOT_W::new(self)
    }
    ///Bits 16:31 - Slot enable. These bits are set and cleared by software. Each SLOTEN bit corresponds to a slot position from 0 to 15 (maximum 16 slots). The slot must be enabled when the audio block is disabled. They are ignored in AC’97 or SPDIF mode.
    #[inline(always)]
    #[must_use]
    pub fn sloten(&mut self) -> SLOTEN_W<16> {
        SLOTEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SAI slot register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [aslotr](index.html) module
pub struct ASLOTR_SPEC;
impl crate::RegisterSpec for ASLOTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [aslotr::R](R) reader structure
impl crate::Readable for ASLOTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [aslotr::W](W) writer structure
impl crate::Writable for ASLOTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ASLOTR to value 0
impl crate::Resettable for ASLOTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
