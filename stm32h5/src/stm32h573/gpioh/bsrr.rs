///Register `BSRR` writer
pub struct W(crate::W<BSRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BSRR_SPEC>;
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
impl From<crate::W<BSRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BSRR_SPEC>) -> Self {
        W(writer)
    }
}
///Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BS0W_AW {
    ///1: Sets the corresponding ODx bit
    Set = 1,
}
impl From<BS0W_AW> for bool {
    #[inline(always)]
    fn from(variant: BS0W_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `BS0` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub type BS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, BS0W_AW, O>;
impl<'a, const O: u8> BS0_W<'a, O> {
    ///Sets the corresponding ODx bit
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BS0W_AW::Set)
    }
}
///Field `BS1` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS1_W;
///Field `BS2` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS2_W;
///Field `BS3` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS3_W;
///Field `BS4` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS4_W;
///Field `BS5` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS5_W;
///Field `BS6` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS6_W;
///Field `BS7` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS7_W;
///Field `BS8` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS8_W;
///Field `BS9` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS9_W;
///Field `BS10` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS10_W;
///Field `BS11` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS11_W;
///Field `BS12` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS12_W;
///Field `BS13` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS13_W;
///Field `BS14` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS14_W;
///Field `BS15` writer - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BS0_W as BS15_W;
///Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BR0W_AW {
    ///1: Resets the corresponding ODx bit
    Reset = 1,
}
impl From<BR0W_AW> for bool {
    #[inline(always)]
    fn from(variant: BR0W_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `BR0` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub type BR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BSRR_SPEC, BR0W_AW, O>;
impl<'a, const O: u8> BR0_W<'a, O> {
    ///Resets the corresponding ODx bit
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR0W_AW::Reset)
    }
}
///Field `BR1` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR1_W;
///Field `BR2` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR2_W;
///Field `BR3` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR3_W;
///Field `BR4` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR4_W;
///Field `BR5` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR5_W;
///Field `BR6` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR6_W;
///Field `BR7` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR7_W;
///Field `BR8` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR8_W;
///Field `BR9` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR9_W;
///Field `BR10` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR10_W;
///Field `BR11` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR11_W;
///Field `BR12` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR12_W;
///Field `BR13` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR13_W;
///Field `BR14` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR14_W;
///Field `BR15` writer - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
pub use BR0_W as BR15_W;
impl W {
    ///Bit 0 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs0(&mut self) -> BS0_W<0> {
        BS0_W::new(self)
    }
    ///Bit 1 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs1(&mut self) -> BS1_W<1> {
        BS1_W::new(self)
    }
    ///Bit 2 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs2(&mut self) -> BS2_W<2> {
        BS2_W::new(self)
    }
    ///Bit 3 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs3(&mut self) -> BS3_W<3> {
        BS3_W::new(self)
    }
    ///Bit 4 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs4(&mut self) -> BS4_W<4> {
        BS4_W::new(self)
    }
    ///Bit 5 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs5(&mut self) -> BS5_W<5> {
        BS5_W::new(self)
    }
    ///Bit 6 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs6(&mut self) -> BS6_W<6> {
        BS6_W::new(self)
    }
    ///Bit 7 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs7(&mut self) -> BS7_W<7> {
        BS7_W::new(self)
    }
    ///Bit 8 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs8(&mut self) -> BS8_W<8> {
        BS8_W::new(self)
    }
    ///Bit 9 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs9(&mut self) -> BS9_W<9> {
        BS9_W::new(self)
    }
    ///Bit 10 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs10(&mut self) -> BS10_W<10> {
        BS10_W::new(self)
    }
    ///Bit 11 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs11(&mut self) -> BS11_W<11> {
        BS11_W::new(self)
    }
    ///Bit 12 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs12(&mut self) -> BS12_W<12> {
        BS12_W::new(self)
    }
    ///Bit 13 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs13(&mut self) -> BS13_W<13> {
        BS13_W::new(self)
    }
    ///Bit 14 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs14(&mut self) -> BS14_W<14> {
        BS14_W::new(self)
    }
    ///Bit 15 - Port x set I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn bs15(&mut self) -> BS15_W<15> {
        BS15_W::new(self)
    }
    ///Bit 16 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br0(&mut self) -> BR0_W<16> {
        BR0_W::new(self)
    }
    ///Bit 17 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br1(&mut self) -> BR1_W<17> {
        BR1_W::new(self)
    }
    ///Bit 18 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br2(&mut self) -> BR2_W<18> {
        BR2_W::new(self)
    }
    ///Bit 19 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<19> {
        BR3_W::new(self)
    }
    ///Bit 20 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br4(&mut self) -> BR4_W<20> {
        BR4_W::new(self)
    }
    ///Bit 21 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br5(&mut self) -> BR5_W<21> {
        BR5_W::new(self)
    }
    ///Bit 22 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br6(&mut self) -> BR6_W<22> {
        BR6_W::new(self)
    }
    ///Bit 23 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br7(&mut self) -> BR7_W<23> {
        BR7_W::new(self)
    }
    ///Bit 24 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br8(&mut self) -> BR8_W<24> {
        BR8_W::new(self)
    }
    ///Bit 25 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br9(&mut self) -> BR9_W<25> {
        BR9_W::new(self)
    }
    ///Bit 26 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br10(&mut self) -> BR10_W<26> {
        BR10_W::new(self)
    }
    ///Bit 27 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br11(&mut self) -> BR11_W<27> {
        BR11_W::new(self)
    }
    ///Bit 28 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br12(&mut self) -> BR12_W<28> {
        BR12_W::new(self)
    }
    ///Bit 29 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br13(&mut self) -> BR13_W<29> {
        BR13_W::new(self)
    }
    ///Bit 30 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br14(&mut self) -> BR14_W<30> {
        BR14_W::new(self)
    }
    ///Bit 31 - Port x reset I/O pin y (y = 15 to 0) These bits are write-only. A read to these bits returns the value 0x0000. Note: If both BSy and BRy are set, BSy has priority. The bit is reserved and must be kept to reset value when the corresponding I/O is not available on the selected package.
    #[inline(always)]
    #[must_use]
    pub fn br15(&mut self) -> BR15_W<31> {
        BR15_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bsrr](index.html) module
pub struct BSRR_SPEC;
impl crate::RegisterSpec for BSRR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [bsrr::W](W) writer structure
impl crate::Writable for BSRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BSRR to value 0
impl crate::Resettable for BSRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
