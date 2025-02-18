///Register `FRCR` reader
pub struct R(crate::R<FRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FRCR` writer
pub struct W(crate::W<FRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRCR_SPEC>;
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
impl From<crate::W<FRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FRL` reader - Frame length. These bits are set and cleared by software. They define the audio frame length expressed in number of SCK clock cycles: the number of bits in the frame is equal to FRL\[7:0\]
///+ 1. The minimum number of bits to transfer in an audio frame must be equal to 8, otherwise the audio block will behaves in an unexpected way. This is the case when the data size is 8 bits and only one slot 0 is defined in NBSLOT\[4:0\]
///of SAI_xSLOTR register (NBSLOT\[3:0\]
///= 0000). In master mode, if the master clock (available on MCLK_x pin) is used, the frame length should be aligned with a number equal to a power of 2, ranging from 8 to 256. When the master clock is not used (NODIV = 1), it is recommended to program the frame length to an value ranging from 8 to 256. These bits are meaningless and are not used in AC97 or SPDIF audio block configuration.
pub type FRL_R = crate::FieldReader<u8, u8>;
///Field `FRL` writer - Frame length. These bits are set and cleared by software. They define the audio frame length expressed in number of SCK clock cycles: the number of bits in the frame is equal to FRL\[7:0\]
///+ 1. The minimum number of bits to transfer in an audio frame must be equal to 8, otherwise the audio block will behaves in an unexpected way. This is the case when the data size is 8 bits and only one slot 0 is defined in NBSLOT\[4:0\]
///of SAI_xSLOTR register (NBSLOT\[3:0\]
///= 0000). In master mode, if the master clock (available on MCLK_x pin) is used, the frame length should be aligned with a number equal to a power of 2, ranging from 8 to 256. When the master clock is not used (NODIV = 1), it is recommended to program the frame length to an value ranging from 8 to 256. These bits are meaningless and are not used in AC97 or SPDIF audio block configuration.
pub type FRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRCR_SPEC, u8, u8, 8, O>;
///Field `FSALL` reader - Frame synchronization active level length. These bits are set and cleared by software. They specify the length in number of bit clock (SCK) + 1 (FSALL\[6:0\]
///+ 1) of the active level of the FS signal in the audio frame These bits are meaningless and are not used in AC97 or SPDIF audio block configuration. They must be configured when the audio block is disabled.
pub type FSALL_R = crate::FieldReader<u8, u8>;
///Field `FSALL` writer - Frame synchronization active level length. These bits are set and cleared by software. They specify the length in number of bit clock (SCK) + 1 (FSALL\[6:0\]
///+ 1) of the active level of the FS signal in the audio frame These bits are meaningless and are not used in AC97 or SPDIF audio block configuration. They must be configured when the audio block is disabled.
pub type FSALL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRCR_SPEC, u8, u8, 7, O>;
///Field `FSDEF` reader - Frame synchronization definition. This bit is set and cleared by software. When the bit is set, the number of slots defined in the SAI_xSLOTR register has to be even. It means that half of this number of slots will be dedicated to the left channel and the other slots for the right channel (e.g: this bit has to be set for I2S or MSB/LSB-justified protocols...). This bit is meaningless and is not used in AC97 or SPDIF audio block configuration. It must be configured when the audio block is disabled.
pub type FSDEF_R = crate::BitReader<bool>;
///Field `FSDEF` writer - Frame synchronization definition. This bit is set and cleared by software. When the bit is set, the number of slots defined in the SAI_xSLOTR register has to be even. It means that half of this number of slots will be dedicated to the left channel and the other slots for the right channel (e.g: this bit has to be set for I2S or MSB/LSB-justified protocols...). This bit is meaningless and is not used in AC97 or SPDIF audio block configuration. It must be configured when the audio block is disabled.
pub type FSDEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRCR_SPEC, bool, O>;
///Field `FSPOL` reader - Frame synchronization polarity. This bit is set and cleared by software. It is used to configure the level of the start of frame on the FS signal. It is meaningless and is not used in AC97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.
pub type FSPOL_R = crate::BitReader<FSPOL_A>;
///Frame synchronization polarity. This bit is set and cleared by software. It is used to configure the level of the start of frame on the FS signal. It is meaningless and is not used in AC97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSPOL_A {
    ///0: FS is active low (falling edge)
    FallingEdge = 0,
    ///1: FS is active high (rising edge)
    RisingEdge = 1,
}
impl From<FSPOL_A> for bool {
    #[inline(always)]
    fn from(variant: FSPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl FSPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FSPOL_A {
        match self.bits {
            false => FSPOL_A::FallingEdge,
            true => FSPOL_A::RisingEdge,
        }
    }
    ///Checks if the value of the field is `FallingEdge`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == FSPOL_A::FallingEdge
    }
    ///Checks if the value of the field is `RisingEdge`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == FSPOL_A::RisingEdge
    }
}
///Field `FSPOL` writer - Frame synchronization polarity. This bit is set and cleared by software. It is used to configure the level of the start of frame on the FS signal. It is meaningless and is not used in AC97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.
pub type FSPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRCR_SPEC, FSPOL_A, O>;
impl<'a, const O: u8> FSPOL_W<'a, O> {
    ///FS is active low (falling edge)
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(FSPOL_A::FallingEdge)
    }
    ///FS is active high (rising edge)
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(FSPOL_A::RisingEdge)
    }
}
///Field `FSOFF` reader - Frame synchronization offset. This bit is set and cleared by software. It is meaningless and is not used in AC97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.
pub type FSOFF_R = crate::BitReader<FSOFF_A>;
///Frame synchronization offset. This bit is set and cleared by software. It is meaningless and is not used in AC97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSOFF_A {
    ///0: FS is asserted on the first bit of the slot 0
    OnFirst = 0,
    ///1: FS is asserted one bit before the first bit of the slot 0
    BeforeFirst = 1,
}
impl From<FSOFF_A> for bool {
    #[inline(always)]
    fn from(variant: FSOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl FSOFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FSOFF_A {
        match self.bits {
            false => FSOFF_A::OnFirst,
            true => FSOFF_A::BeforeFirst,
        }
    }
    ///Checks if the value of the field is `OnFirst`
    #[inline(always)]
    pub fn is_on_first(&self) -> bool {
        *self == FSOFF_A::OnFirst
    }
    ///Checks if the value of the field is `BeforeFirst`
    #[inline(always)]
    pub fn is_before_first(&self) -> bool {
        *self == FSOFF_A::BeforeFirst
    }
}
///Field `FSOFF` writer - Frame synchronization offset. This bit is set and cleared by software. It is meaningless and is not used in AC97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.
pub type FSOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRCR_SPEC, FSOFF_A, O>;
impl<'a, const O: u8> FSOFF_W<'a, O> {
    ///FS is asserted on the first bit of the slot 0
    #[inline(always)]
    pub fn on_first(self) -> &'a mut W {
        self.variant(FSOFF_A::OnFirst)
    }
    ///FS is asserted one bit before the first bit of the slot 0
    #[inline(always)]
    pub fn before_first(self) -> &'a mut W {
        self.variant(FSOFF_A::BeforeFirst)
    }
}
impl R {
    ///Bits 0:7 - Frame length. These bits are set and cleared by software. They define the audio frame length expressed in number of SCK clock cycles: the number of bits in the frame is equal to FRL\[7:0\]
    ///+ 1. The minimum number of bits to transfer in an audio frame must be equal to 8, otherwise the audio block will behaves in an unexpected way. This is the case when the data size is 8 bits and only one slot 0 is defined in NBSLOT\[4:0\]
    ///of SAI_xSLOTR register (NBSLOT\[3:0\]
    ///= 0000). In master mode, if the master clock (available on MCLK_x pin) is used, the frame length should be aligned with a number equal to a power of 2, ranging from 8 to 256. When the master clock is not used (NODIV = 1), it is recommended to program the frame length to an value ranging from 8 to 256. These bits are meaningless and are not used in AC97 or SPDIF audio block configuration.
    #[inline(always)]
    pub fn frl(&self) -> FRL_R {
        FRL_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:14 - Frame synchronization active level length. These bits are set and cleared by software. They specify the length in number of bit clock (SCK) + 1 (FSALL\[6:0\]
    ///+ 1) of the active level of the FS signal in the audio frame These bits are meaningless and are not used in AC97 or SPDIF audio block configuration. They must be configured when the audio block is disabled.
    #[inline(always)]
    pub fn fsall(&self) -> FSALL_R {
        FSALL_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 16 - Frame synchronization definition. This bit is set and cleared by software. When the bit is set, the number of slots defined in the SAI_xSLOTR register has to be even. It means that half of this number of slots will be dedicated to the left channel and the other slots for the right channel (e.g: this bit has to be set for I2S or MSB/LSB-justified protocols...). This bit is meaningless and is not used in AC97 or SPDIF audio block configuration. It must be configured when the audio block is disabled.
    #[inline(always)]
    pub fn fsdef(&self) -> FSDEF_R {
        FSDEF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Frame synchronization polarity. This bit is set and cleared by software. It is used to configure the level of the start of frame on the FS signal. It is meaningless and is not used in AC97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.
    #[inline(always)]
    pub fn fspol(&self) -> FSPOL_R {
        FSPOL_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Frame synchronization offset. This bit is set and cleared by software. It is meaningless and is not used in AC97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.
    #[inline(always)]
    pub fn fsoff(&self) -> FSOFF_R {
        FSOFF_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - Frame length. These bits are set and cleared by software. They define the audio frame length expressed in number of SCK clock cycles: the number of bits in the frame is equal to FRL\[7:0\]
    ///+ 1. The minimum number of bits to transfer in an audio frame must be equal to 8, otherwise the audio block will behaves in an unexpected way. This is the case when the data size is 8 bits and only one slot 0 is defined in NBSLOT\[4:0\]
    ///of SAI_xSLOTR register (NBSLOT\[3:0\]
    ///= 0000). In master mode, if the master clock (available on MCLK_x pin) is used, the frame length should be aligned with a number equal to a power of 2, ranging from 8 to 256. When the master clock is not used (NODIV = 1), it is recommended to program the frame length to an value ranging from 8 to 256. These bits are meaningless and are not used in AC97 or SPDIF audio block configuration.
    #[inline(always)]
    #[must_use]
    pub fn frl(&mut self) -> FRL_W<0> {
        FRL_W::new(self)
    }
    ///Bits 8:14 - Frame synchronization active level length. These bits are set and cleared by software. They specify the length in number of bit clock (SCK) + 1 (FSALL\[6:0\]
    ///+ 1) of the active level of the FS signal in the audio frame These bits are meaningless and are not used in AC97 or SPDIF audio block configuration. They must be configured when the audio block is disabled.
    #[inline(always)]
    #[must_use]
    pub fn fsall(&mut self) -> FSALL_W<8> {
        FSALL_W::new(self)
    }
    ///Bit 16 - Frame synchronization definition. This bit is set and cleared by software. When the bit is set, the number of slots defined in the SAI_xSLOTR register has to be even. It means that half of this number of slots will be dedicated to the left channel and the other slots for the right channel (e.g: this bit has to be set for I2S or MSB/LSB-justified protocols...). This bit is meaningless and is not used in AC97 or SPDIF audio block configuration. It must be configured when the audio block is disabled.
    #[inline(always)]
    #[must_use]
    pub fn fsdef(&mut self) -> FSDEF_W<16> {
        FSDEF_W::new(self)
    }
    ///Bit 17 - Frame synchronization polarity. This bit is set and cleared by software. It is used to configure the level of the start of frame on the FS signal. It is meaningless and is not used in AC97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.
    #[inline(always)]
    #[must_use]
    pub fn fspol(&mut self) -> FSPOL_W<17> {
        FSPOL_W::new(self)
    }
    ///Bit 18 - Frame synchronization offset. This bit is set and cleared by software. It is meaningless and is not used in AC97 or SPDIF audio block configuration. This bit must be configured when the audio block is disabled.
    #[inline(always)]
    #[must_use]
    pub fn fsoff(&mut self) -> FSOFF_W<18> {
        FSOFF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register has no meaning in AC97 and SPDIF audio protocol
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [frcr](index.html) module
pub struct FRCR_SPEC;
impl crate::RegisterSpec for FRCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [frcr::R](R) reader structure
impl crate::Readable for FRCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [frcr::W](W) writer structure
impl crate::Writable for FRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FRCR to value 0x07
impl crate::Resettable for FRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
