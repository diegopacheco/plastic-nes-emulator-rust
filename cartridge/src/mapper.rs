use common::Device;

pub trait Mapper {
    fn init(&mut self, pgr_count: u8, chr_count: u8);
    fn map_read(&self, address: u16, device: Device) -> u16;
    // used for mapping internal changes, as cartridge is only ROM
    fn map_write(&self, address: u16, data: u8, device: Device);
}
