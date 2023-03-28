///Register `CHSELR1` reader
pub struct R(crate::R<CHSELR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHSELR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHSELR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHSELR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CHSELR1` writer
pub struct W(crate::W<CHSELR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHSELR1_SPEC>;
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
impl From<crate::W<CHSELR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHSELR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SQ1` reader - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ1_R = crate::FieldReader<u8, u8>;
///Field `SQ1` writer - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHSELR1_SPEC, u8, u8, 4, O>;
///Field `SQ2` reader - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ2_R = crate::FieldReader<u8, u8>;
///Field `SQ2` writer - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHSELR1_SPEC, u8, u8, 4, O>;
///Field `SQ3` reader - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ3_R = crate::FieldReader<u8, u8>;
///Field `SQ3` writer - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHSELR1_SPEC, u8, u8, 4, O>;
///Field `SQ4` reader - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ4_R = crate::FieldReader<u8, u8>;
///Field `SQ4` writer - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHSELR1_SPEC, u8, u8, 4, O>;
///Field `SQ5` reader - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ5_R = crate::FieldReader<u8, u8>;
///Field `SQ5` writer - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHSELR1_SPEC, u8, u8, 4, O>;
///Field `SQ6` reader - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ6_R = crate::FieldReader<u8, u8>;
///Field `SQ6` writer - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHSELR1_SPEC, u8, u8, 4, O>;
///Field `SQ7` reader - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ7_R = crate::FieldReader<u8, u8>;
///Field `SQ7` writer - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHSELR1_SPEC, u8, u8, 4, O>;
///Field `SQ8` reader - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ8_R = crate::FieldReader<u8, u8>;
///Field `SQ8` writer - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
pub type SQ8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHSELR1_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:3 - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq1(&self) -> SQ1_R {
        SQ1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq2(&self) -> SQ2_R {
        SQ2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq3(&self) -> SQ3_R {
        SQ3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq4(&self) -> SQ4_R {
        SQ4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq5(&self) -> SQ5_R {
        SQ5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq6(&self) -> SQ6_R {
        SQ6_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq7(&self) -> SQ7_R {
        SQ7_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq8(&self) -> SQ8_R {
        SQ8_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn sq1(&mut self) -> SQ1_W<0> {
        SQ1_W::new(self)
    }
    ///Bits 4:7 - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn sq2(&mut self) -> SQ2_W<4> {
        SQ2_W::new(self)
    }
    ///Bits 8:11 - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn sq3(&mut self) -> SQ3_W<8> {
        SQ3_W::new(self)
    }
    ///Bits 12:15 - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn sq4(&mut self) -> SQ4_W<12> {
        SQ4_W::new(self)
    }
    ///Bits 16:19 - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn sq5(&mut self) -> SQ5_W<16> {
        SQ5_W::new(self)
    }
    ///Bits 20:23 - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn sq6(&mut self) -> SQ6_W<20> {
        SQ6_W::new(self)
    }
    ///Bits 24:27 - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn sq7(&mut self) -> SQ7_W<24> {
        SQ7_W::new(self)
    }
    ///Bits 28:31 - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn sq8(&mut self) -> SQ8_W<28> {
        SQ8_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC channel selection register \[alternate\]
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chselr1](index.html) module
pub struct CHSELR1_SPEC;
impl crate::RegisterSpec for CHSELR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [chselr1::R](R) reader structure
impl crate::Readable for CHSELR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [chselr1::W](W) writer structure
impl crate::Writable for CHSELR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CHSELR1 to value 0
impl crate::Resettable for CHSELR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
