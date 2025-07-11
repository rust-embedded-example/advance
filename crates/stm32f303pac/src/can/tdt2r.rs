#[doc = "Register `TDT2R` reader"]
pub type R = crate::R<Tdt2rSpec>;
#[doc = "Register `TDT2R` writer"]
pub type W = crate::W<Tdt2rSpec>;
#[doc = "Field `DLC` reader - DLC"]
pub type DlcR = crate::FieldReader;
#[doc = "Field `DLC` writer - DLC"]
pub type DlcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TGT` reader - TGT"]
pub type TgtR = crate::BitReader;
#[doc = "Field `TGT` writer - TGT"]
pub type TgtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME` reader - TIME"]
pub type TimeR = crate::FieldReader<u16>;
#[doc = "Field `TIME` writer - TIME"]
pub type TimeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - DLC"]
    #[inline(always)]
    pub fn dlc(&self) -> DlcR {
        DlcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - TGT"]
    #[inline(always)]
    pub fn tgt(&self) -> TgtR {
        TgtR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - TIME"]
    #[inline(always)]
    pub fn time(&self) -> TimeR {
        TimeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - DLC"]
    #[inline(always)]
    pub fn dlc(&mut self) -> DlcW<'_, Tdt2rSpec> {
        DlcW::new(self, 0)
    }
    #[doc = "Bit 8 - TGT"]
    #[inline(always)]
    pub fn tgt(&mut self) -> TgtW<'_, Tdt2rSpec> {
        TgtW::new(self, 8)
    }
    #[doc = "Bits 16:31 - TIME"]
    #[inline(always)]
    pub fn time(&mut self) -> TimeW<'_, Tdt2rSpec> {
        TimeW::new(self, 16)
    }
}
#[doc = "mailbox data length control and time stamp register\n\nYou can [`read`](crate::Reg::read) this register and get [`tdt2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdt2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tdt2rSpec;
impl crate::RegisterSpec for Tdt2rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdt2r::R`](R) reader structure"]
impl crate::Readable for Tdt2rSpec {}
#[doc = "`write(|w| ..)` method takes [`tdt2r::W`](W) writer structure"]
impl crate::Writable for Tdt2rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TDT2R to value 0"]
impl crate::Resettable for Tdt2rSpec {}
