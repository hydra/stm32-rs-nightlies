///Register `SLOTR` reader
pub struct R(crate::R<SLOTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLOTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLOTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLOTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SLOTR` writer
pub struct W(crate::W<SLOTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLOTR_SPEC>;
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
impl From<crate::W<SLOTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLOTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FBOFF` reader - First bit offset
pub type FBOFF_R = crate::FieldReader<u8, u8>;
///Field `FBOFF` writer - First bit offset
pub type FBOFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SLOTR_SPEC, u8, u8, 5, O>;
///Field `SLOTSZ` reader - Slot size
pub type SLOTSZ_R = crate::FieldReader<u8, SLOTSZ_A>;
///Slot size
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
///Field `SLOTSZ` writer - Slot size
pub type SLOTSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SLOTR_SPEC, u8, SLOTSZ_A, 2, O>;
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
///Field `NBSLOT` reader - Number of slots in an audio frame
pub type NBSLOT_R = crate::FieldReader<u8, u8>;
///Field `NBSLOT` writer - Number of slots in an audio frame
pub type NBSLOT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SLOTR_SPEC, u8, u8, 4, O>;
///Field `SLOTEN` reader - Slot enable
pub type SLOTEN_R = crate::FieldReader<u16, SLOTEN_A>;
///Slot enable
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
///Field `SLOTEN` writer - Slot enable
pub type SLOTEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SLOTR_SPEC, u16, SLOTEN_A, 16, O>;
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
    ///Bits 0:4 - First bit offset
    #[inline(always)]
    pub fn fboff(&self) -> FBOFF_R {
        FBOFF_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 6:7 - Slot size
    #[inline(always)]
    pub fn slotsz(&self) -> SLOTSZ_R {
        SLOTSZ_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:11 - Number of slots in an audio frame
    #[inline(always)]
    pub fn nbslot(&self) -> NBSLOT_R {
        NBSLOT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:31 - Slot enable
    #[inline(always)]
    pub fn sloten(&self) -> SLOTEN_R {
        SLOTEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:4 - First bit offset
    #[inline(always)]
    #[must_use]
    pub fn fboff(&mut self) -> FBOFF_W<0> {
        FBOFF_W::new(self)
    }
    ///Bits 6:7 - Slot size
    #[inline(always)]
    #[must_use]
    pub fn slotsz(&mut self) -> SLOTSZ_W<6> {
        SLOTSZ_W::new(self)
    }
    ///Bits 8:11 - Number of slots in an audio frame
    #[inline(always)]
    #[must_use]
    pub fn nbslot(&mut self) -> NBSLOT_W<8> {
        NBSLOT_W::new(self)
    }
    ///Bits 16:31 - Slot enable
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
///ASlot register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [slotr](index.html) module
pub struct SLOTR_SPEC;
impl crate::RegisterSpec for SLOTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [slotr::R](R) reader structure
impl crate::Readable for SLOTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [slotr::W](W) writer structure
impl crate::Writable for SLOTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SLOTR to value 0
impl crate::Resettable for SLOTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
