///Register `DCR1` reader
pub struct R(crate::R<DCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DCR1` writer
pub struct W(crate::W<DCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCR1_SPEC>;
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
impl From<crate::W<DCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CKMODE` reader - Mode 0/Mode 3 This bit indicates the level taken by the CLK between commands (when NCS = 1).
pub type CKMODE_R = crate::BitReader<bool>;
///Field `CKMODE` writer - Mode 0/Mode 3 This bit indicates the level taken by the CLK between commands (when NCS = 1).
pub type CKMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCR1_SPEC, bool, O>;
///Field `FRCK` reader - Free running clock This bit configures the free running clock.
pub type FRCK_R = crate::BitReader<bool>;
///Field `FRCK` writer - Free running clock This bit configures the free running clock.
pub type FRCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCR1_SPEC, bool, O>;
///Field `DLYBYP` reader - Delay block bypass
pub type DLYBYP_R = crate::BitReader<bool>;
///Field `DLYBYP` writer - Delay block bypass
pub type DLYBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCR1_SPEC, bool, O>;
///Field `CSHT` reader - Chip-select high time CSHT + 1 defines the minimum number of CLK cycles where the chip-select (NCS) must remain high between commands issued to the external device. ...
pub type CSHT_R = crate::FieldReader<u8, u8>;
///Field `CSHT` writer - Chip-select high time CSHT + 1 defines the minimum number of CLK cycles where the chip-select (NCS) must remain high between commands issued to the external device. ...
pub type CSHT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR1_SPEC, u8, u8, 6, O>;
///Field `DEVSIZE` reader - Device size This field defines the size of the external device using the following formula: Number of bytes in device = 2\[DEVSIZE+1\]. DEVSIZE+1 is effectively the number of address bits required to address the external device. The device capacity can be up to 4 Gbytes (addressed using 32-bits) in Indirect mode, but the addressable space in Memory-mapped mode is limited to 256 Mbytes. In Regular-command protocol, if DMM = 1, DEVSIZE\[4:0\]
///indicates the total capacity of the two devices together.
pub type DEVSIZE_R = crate::FieldReader<u8, u8>;
///Field `DEVSIZE` writer - Device size This field defines the size of the external device using the following formula: Number of bytes in device = 2\[DEVSIZE+1\]. DEVSIZE+1 is effectively the number of address bits required to address the external device. The device capacity can be up to 4 Gbytes (addressed using 32-bits) in Indirect mode, but the addressable space in Memory-mapped mode is limited to 256 Mbytes. In Regular-command protocol, if DMM = 1, DEVSIZE\[4:0\]
///indicates the total capacity of the two devices together.
pub type DEVSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR1_SPEC, u8, u8, 5, O>;
///Field `MTYP` reader - Memory type This bit indicates the type of memory to be supported. Note: In this mode, DQS signal polarity is inverted with respect to the memory clock signal. This is the default value and care must be taken to change MTYP\[2:0\]
///for memories different from Micron. Others: Reserved
pub type MTYP_R = crate::FieldReader<u8, u8>;
///Field `MTYP` writer - Memory type This bit indicates the type of memory to be supported. Note: In this mode, DQS signal polarity is inverted with respect to the memory clock signal. This is the default value and care must be taken to change MTYP\[2:0\]
///for memories different from Micron. Others: Reserved
pub type MTYP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCR1_SPEC, u8, u8, 3, O>;
impl R {
    ///Bit 0 - Mode 0/Mode 3 This bit indicates the level taken by the CLK between commands (when NCS = 1).
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Free running clock This bit configures the free running clock.
    #[inline(always)]
    pub fn frck(&self) -> FRCK_R {
        FRCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Delay block bypass
    #[inline(always)]
    pub fn dlybyp(&self) -> DLYBYP_R {
        DLYBYP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 8:13 - Chip-select high time CSHT + 1 defines the minimum number of CLK cycles where the chip-select (NCS) must remain high between commands issued to the external device. ...
    #[inline(always)]
    pub fn csht(&self) -> CSHT_R {
        CSHT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 16:20 - Device size This field defines the size of the external device using the following formula: Number of bytes in device = 2\[DEVSIZE+1\]. DEVSIZE+1 is effectively the number of address bits required to address the external device. The device capacity can be up to 4 Gbytes (addressed using 32-bits) in Indirect mode, but the addressable space in Memory-mapped mode is limited to 256 Mbytes. In Regular-command protocol, if DMM = 1, DEVSIZE\[4:0\]
    ///indicates the total capacity of the two devices together.
    #[inline(always)]
    pub fn devsize(&self) -> DEVSIZE_R {
        DEVSIZE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:26 - Memory type This bit indicates the type of memory to be supported. Note: In this mode, DQS signal polarity is inverted with respect to the memory clock signal. This is the default value and care must be taken to change MTYP\[2:0\]
    ///for memories different from Micron. Others: Reserved
    #[inline(always)]
    pub fn mtyp(&self) -> MTYP_R {
        MTYP_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    ///Bit 0 - Mode 0/Mode 3 This bit indicates the level taken by the CLK between commands (when NCS = 1).
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CKMODE_W<0> {
        CKMODE_W::new(self)
    }
    ///Bit 1 - Free running clock This bit configures the free running clock.
    #[inline(always)]
    #[must_use]
    pub fn frck(&mut self) -> FRCK_W<1> {
        FRCK_W::new(self)
    }
    ///Bit 3 - Delay block bypass
    #[inline(always)]
    #[must_use]
    pub fn dlybyp(&mut self) -> DLYBYP_W<3> {
        DLYBYP_W::new(self)
    }
    ///Bits 8:13 - Chip-select high time CSHT + 1 defines the minimum number of CLK cycles where the chip-select (NCS) must remain high between commands issued to the external device. ...
    #[inline(always)]
    #[must_use]
    pub fn csht(&mut self) -> CSHT_W<8> {
        CSHT_W::new(self)
    }
    ///Bits 16:20 - Device size This field defines the size of the external device using the following formula: Number of bytes in device = 2\[DEVSIZE+1\]. DEVSIZE+1 is effectively the number of address bits required to address the external device. The device capacity can be up to 4 Gbytes (addressed using 32-bits) in Indirect mode, but the addressable space in Memory-mapped mode is limited to 256 Mbytes. In Regular-command protocol, if DMM = 1, DEVSIZE\[4:0\]
    ///indicates the total capacity of the two devices together.
    #[inline(always)]
    #[must_use]
    pub fn devsize(&mut self) -> DEVSIZE_W<16> {
        DEVSIZE_W::new(self)
    }
    ///Bits 24:26 - Memory type This bit indicates the type of memory to be supported. Note: In this mode, DQS signal polarity is inverted with respect to the memory clock signal. This is the default value and care must be taken to change MTYP\[2:0\]
    ///for memories different from Micron. Others: Reserved
    #[inline(always)]
    #[must_use]
    pub fn mtyp(&mut self) -> MTYP_W<24> {
        MTYP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OCTOSPI device configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dcr1](index.html) module
pub struct DCR1_SPEC;
impl crate::RegisterSpec for DCR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [dcr1::R](R) reader structure
impl crate::Readable for DCR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dcr1::W](W) writer structure
impl crate::Writable for DCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DCR1 to value 0
impl crate::Resettable for DCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
