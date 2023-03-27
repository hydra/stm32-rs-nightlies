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
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
pub type SQ1_R = crate::FieldReader<u8, SQ1_A>;
///1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SQ1_A {
    ///0: Channel 0 selected for the Nth conversion
    Ch0 = 0,
    ///1: Channel 1 selected for the Nth conversion
    Ch1 = 1,
    ///2: Channel 2 selected for the Nth conversion
    Ch2 = 2,
    ///3: Channel 3 selected for the Nth conversion
    Ch3 = 3,
    ///4: Channel 4 selected for the Nth conversion
    Ch4 = 4,
    ///5: Channel 5 selected for the Nth conversion
    Ch5 = 5,
    ///6: Channel 6 selected for the Nth conversion
    Ch6 = 6,
    ///7: Channel 7 selected for the Nth conversion
    Ch7 = 7,
    ///8: Channel 8 selected for the Nth conversion
    Ch8 = 8,
    ///9: Channel 9 selected for the Nth conversion
    Ch9 = 9,
    ///10: Channel 10 selected for the Nth conversion
    Ch10 = 10,
    ///11: Channel 11 selected for the Nth conversion
    Ch11 = 11,
    ///12: Channel 12 selected for the Nth conversion
    Ch12 = 12,
    ///13: Channel 13 selected for the Nth conversion
    Ch13 = 13,
    ///14: Channel 14 selected for the Nth conversion
    Ch14 = 14,
    ///15: End of sequence
    Eos = 15,
}
impl From<SQ1_A> for u8 {
    #[inline(always)]
    fn from(variant: SQ1_A) -> Self {
        variant as _
    }
}
impl SQ1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SQ1_A {
        match self.bits {
            0 => SQ1_A::Ch0,
            1 => SQ1_A::Ch1,
            2 => SQ1_A::Ch2,
            3 => SQ1_A::Ch3,
            4 => SQ1_A::Ch4,
            5 => SQ1_A::Ch5,
            6 => SQ1_A::Ch6,
            7 => SQ1_A::Ch7,
            8 => SQ1_A::Ch8,
            9 => SQ1_A::Ch9,
            10 => SQ1_A::Ch10,
            11 => SQ1_A::Ch11,
            12 => SQ1_A::Ch12,
            13 => SQ1_A::Ch13,
            14 => SQ1_A::Ch14,
            15 => SQ1_A::Eos,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Ch0`
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        *self == SQ1_A::Ch0
    }
    ///Checks if the value of the field is `Ch1`
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        *self == SQ1_A::Ch1
    }
    ///Checks if the value of the field is `Ch2`
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        *self == SQ1_A::Ch2
    }
    ///Checks if the value of the field is `Ch3`
    #[inline(always)]
    pub fn is_ch3(&self) -> bool {
        *self == SQ1_A::Ch3
    }
    ///Checks if the value of the field is `Ch4`
    #[inline(always)]
    pub fn is_ch4(&self) -> bool {
        *self == SQ1_A::Ch4
    }
    ///Checks if the value of the field is `Ch5`
    #[inline(always)]
    pub fn is_ch5(&self) -> bool {
        *self == SQ1_A::Ch5
    }
    ///Checks if the value of the field is `Ch6`
    #[inline(always)]
    pub fn is_ch6(&self) -> bool {
        *self == SQ1_A::Ch6
    }
    ///Checks if the value of the field is `Ch7`
    #[inline(always)]
    pub fn is_ch7(&self) -> bool {
        *self == SQ1_A::Ch7
    }
    ///Checks if the value of the field is `Ch8`
    #[inline(always)]
    pub fn is_ch8(&self) -> bool {
        *self == SQ1_A::Ch8
    }
    ///Checks if the value of the field is `Ch9`
    #[inline(always)]
    pub fn is_ch9(&self) -> bool {
        *self == SQ1_A::Ch9
    }
    ///Checks if the value of the field is `Ch10`
    #[inline(always)]
    pub fn is_ch10(&self) -> bool {
        *self == SQ1_A::Ch10
    }
    ///Checks if the value of the field is `Ch11`
    #[inline(always)]
    pub fn is_ch11(&self) -> bool {
        *self == SQ1_A::Ch11
    }
    ///Checks if the value of the field is `Ch12`
    #[inline(always)]
    pub fn is_ch12(&self) -> bool {
        *self == SQ1_A::Ch12
    }
    ///Checks if the value of the field is `Ch13`
    #[inline(always)]
    pub fn is_ch13(&self) -> bool {
        *self == SQ1_A::Ch13
    }
    ///Checks if the value of the field is `Ch14`
    #[inline(always)]
    pub fn is_ch14(&self) -> bool {
        *self == SQ1_A::Ch14
    }
    ///Checks if the value of the field is `Eos`
    #[inline(always)]
    pub fn is_eos(&self) -> bool {
        *self == SQ1_A::Eos
    }
}
///Field `SQ1` writer - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
pub type SQ1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CHSELR1_SPEC, u8, SQ1_A, 4, O>;
impl<'a, const O: u8> SQ1_W<'a, O> {
    ///Channel 0 selected for the Nth conversion
    #[inline(always)]
    pub fn ch0(self) -> &'a mut W {
        self.variant(SQ1_A::Ch0)
    }
    ///Channel 1 selected for the Nth conversion
    #[inline(always)]
    pub fn ch1(self) -> &'a mut W {
        self.variant(SQ1_A::Ch1)
    }
    ///Channel 2 selected for the Nth conversion
    #[inline(always)]
    pub fn ch2(self) -> &'a mut W {
        self.variant(SQ1_A::Ch2)
    }
    ///Channel 3 selected for the Nth conversion
    #[inline(always)]
    pub fn ch3(self) -> &'a mut W {
        self.variant(SQ1_A::Ch3)
    }
    ///Channel 4 selected for the Nth conversion
    #[inline(always)]
    pub fn ch4(self) -> &'a mut W {
        self.variant(SQ1_A::Ch4)
    }
    ///Channel 5 selected for the Nth conversion
    #[inline(always)]
    pub fn ch5(self) -> &'a mut W {
        self.variant(SQ1_A::Ch5)
    }
    ///Channel 6 selected for the Nth conversion
    #[inline(always)]
    pub fn ch6(self) -> &'a mut W {
        self.variant(SQ1_A::Ch6)
    }
    ///Channel 7 selected for the Nth conversion
    #[inline(always)]
    pub fn ch7(self) -> &'a mut W {
        self.variant(SQ1_A::Ch7)
    }
    ///Channel 8 selected for the Nth conversion
    #[inline(always)]
    pub fn ch8(self) -> &'a mut W {
        self.variant(SQ1_A::Ch8)
    }
    ///Channel 9 selected for the Nth conversion
    #[inline(always)]
    pub fn ch9(self) -> &'a mut W {
        self.variant(SQ1_A::Ch9)
    }
    ///Channel 10 selected for the Nth conversion
    #[inline(always)]
    pub fn ch10(self) -> &'a mut W {
        self.variant(SQ1_A::Ch10)
    }
    ///Channel 11 selected for the Nth conversion
    #[inline(always)]
    pub fn ch11(self) -> &'a mut W {
        self.variant(SQ1_A::Ch11)
    }
    ///Channel 12 selected for the Nth conversion
    #[inline(always)]
    pub fn ch12(self) -> &'a mut W {
        self.variant(SQ1_A::Ch12)
    }
    ///Channel 13 selected for the Nth conversion
    #[inline(always)]
    pub fn ch13(self) -> &'a mut W {
        self.variant(SQ1_A::Ch13)
    }
    ///Channel 14 selected for the Nth conversion
    #[inline(always)]
    pub fn ch14(self) -> &'a mut W {
        self.variant(SQ1_A::Ch14)
    }
    ///End of sequence
    #[inline(always)]
    pub fn eos(self) -> &'a mut W {
        self.variant(SQ1_A::Eos)
    }
}
///Field `SQ2` reader - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
pub use SQ1_R as SQ2_R;
///Field `SQ3` reader - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
pub use SQ1_R as SQ3_R;
///Field `SQ4` reader - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
pub use SQ1_R as SQ4_R;
///Field `SQ5` reader - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
pub use SQ1_R as SQ5_R;
///Field `SQ6` reader - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
pub use SQ1_R as SQ6_R;
///Field `SQ7` reader - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
pub use SQ1_R as SQ7_R;
///Field `SQ8` reader - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
pub use SQ1_R as SQ8_R;
///Field `SQ2` writer - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
pub use SQ1_W as SQ2_W;
///Field `SQ3` writer - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
pub use SQ1_W as SQ3_W;
///Field `SQ4` writer - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
pub use SQ1_W as SQ4_W;
///Field `SQ5` writer - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
pub use SQ1_W as SQ5_W;
///Field `SQ6` writer - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
pub use SQ1_W as SQ6_W;
///Field `SQ7` writer - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
pub use SQ1_W as SQ7_W;
///Field `SQ8` writer - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
pub use SQ1_W as SQ8_W;
impl R {
    ///Bits 0:3 - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq1(&self) -> SQ1_R {
        SQ1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq2(&self) -> SQ2_R {
        SQ2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq3(&self) -> SQ3_R {
        SQ3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq4(&self) -> SQ4_R {
        SQ4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq5(&self) -> SQ5_R {
        SQ5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq6(&self) -> SQ6_R {
        SQ6_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq7(&self) -> SQ7_R {
        SQ7_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    pub fn sq8(&self) -> SQ8_R {
        SQ8_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - 1st conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn sq1(&mut self) -> SQ1_W<0> {
        SQ1_W::new(self)
    }
    ///Bits 4:7 - 2nd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn sq2(&mut self) -> SQ2_W<4> {
        SQ2_W::new(self)
    }
    ///Bits 8:11 - 3rd conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn sq3(&mut self) -> SQ3_W<8> {
        SQ3_W::new(self)
    }
    ///Bits 12:15 - 4th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn sq4(&mut self) -> SQ4_W<12> {
        SQ4_W::new(self)
    }
    ///Bits 16:19 - 5th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn sq5(&mut self) -> SQ5_W<16> {
        SQ5_W::new(self)
    }
    ///Bits 20:23 - 6th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn sq6(&mut self) -> SQ6_W<20> {
        SQ6_W::new(self)
    }
    ///Bits 24:27 - 7th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. Refer to SQ8\[3:0\]
    ///for a definition of channel selection. Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
    #[inline(always)]
    #[must_use]
    pub fn sq7(&mut self) -> SQ7_W<24> {
        SQ7_W::new(self)
    }
    ///Bits 28:31 - 8th conversion of the sequence These bits are programmed by software with the channel number (0...14) assigned to the 8th conversion of the sequence. 0b1111 indicates the end of the sequence. When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are ignored. ... Note: The software is allowed to write this bit only when ADSTARTÂ =Â 0 (which ensures that no conversion is ongoing).
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
///ADC channel selection register
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
