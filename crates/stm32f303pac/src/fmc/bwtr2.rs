#[doc = "Register `BWTR2` reader"]
pub type R = crate::R<Bwtr2Spec>;
#[doc = "Register `BWTR2` writer"]
pub type W = crate::W<Bwtr2Spec>;
#[doc = "Field `ADDSET` reader - ADDSET"]
pub type AddsetR = crate::FieldReader;
#[doc = "Field `ADDSET` writer - ADDSET"]
pub type AddsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDHLD` reader - ADDHLD"]
pub type AddhldR = crate::FieldReader;
#[doc = "Field `ADDHLD` writer - ADDHLD"]
pub type AddhldW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATAST` reader - DATAST"]
pub type DatastR = crate::FieldReader;
#[doc = "Field `DATAST` writer - DATAST"]
pub type DatastW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BUSTURN` reader - Bus turnaround phase duration"]
pub type BusturnR = crate::FieldReader;
#[doc = "Field `BUSTURN` writer - Bus turnaround phase duration"]
pub type BusturnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLKDIV` reader - CLKDIV"]
pub type ClkdivR = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - CLKDIV"]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATLAT` reader - DATLAT"]
pub type DatlatR = crate::FieldReader;
#[doc = "Field `DATLAT` writer - DATLAT"]
pub type DatlatW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ACCMOD` reader - ACCMOD"]
pub type AccmodR = crate::FieldReader;
#[doc = "Field `ACCMOD` writer - ACCMOD"]
pub type AccmodW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - ADDSET"]
    #[inline(always)]
    pub fn addset(&self) -> AddsetR {
        AddsetR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - ADDHLD"]
    #[inline(always)]
    pub fn addhld(&self) -> AddhldR {
        AddhldR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - DATAST"]
    #[inline(always)]
    pub fn datast(&self) -> DatastR {
        DatastR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Bus turnaround phase duration"]
    #[inline(always)]
    pub fn busturn(&self) -> BusturnR {
        BusturnR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - CLKDIV"]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DATLAT"]
    #[inline(always)]
    pub fn datlat(&self) -> DatlatR {
        DatlatR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - ACCMOD"]
    #[inline(always)]
    pub fn accmod(&self) -> AccmodR {
        AccmodR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDSET"]
    #[inline(always)]
    pub fn addset(&mut self) -> AddsetW<'_, Bwtr2Spec> {
        AddsetW::new(self, 0)
    }
    #[doc = "Bits 4:7 - ADDHLD"]
    #[inline(always)]
    pub fn addhld(&mut self) -> AddhldW<'_, Bwtr2Spec> {
        AddhldW::new(self, 4)
    }
    #[doc = "Bits 8:15 - DATAST"]
    #[inline(always)]
    pub fn datast(&mut self) -> DatastW<'_, Bwtr2Spec> {
        DatastW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Bus turnaround phase duration"]
    #[inline(always)]
    pub fn busturn(&mut self) -> BusturnW<'_, Bwtr2Spec> {
        BusturnW::new(self, 16)
    }
    #[doc = "Bits 20:23 - CLKDIV"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> ClkdivW<'_, Bwtr2Spec> {
        ClkdivW::new(self, 20)
    }
    #[doc = "Bits 24:27 - DATLAT"]
    #[inline(always)]
    pub fn datlat(&mut self) -> DatlatW<'_, Bwtr2Spec> {
        DatlatW::new(self, 24)
    }
    #[doc = "Bits 28:29 - ACCMOD"]
    #[inline(always)]
    pub fn accmod(&mut self) -> AccmodW<'_, Bwtr2Spec> {
        AccmodW::new(self, 28)
    }
}
#[doc = "SRAM/NOR-Flash write timing registers 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bwtr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bwtr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bwtr2Spec;
impl crate::RegisterSpec for Bwtr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bwtr2::R`](R) reader structure"]
impl crate::Readable for Bwtr2Spec {}
#[doc = "`write(|w| ..)` method takes [`bwtr2::W`](W) writer structure"]
impl crate::Writable for Bwtr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BWTR2 to value 0x0fff_ffff"]
impl crate::Resettable for Bwtr2Spec {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
