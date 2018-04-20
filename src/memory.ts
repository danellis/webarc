export class Memory {
    private ramBuffer = new ArrayBuffer(4 * 1024 * 1024); // 4MiB RAM
    private ram = new DataView(this.ramBuffer);
    private rom: DataView;

    private romMapped = true;

    constructor(romData: ArrayBuffer) {
        this.rom = new DataView(romData);
        console.debug('ROM size: 0x' + this.rom.byteLength.toString(16));
    }

    load(address: number): number {
        address &= 0x03fffffc;

        // Logically mapped RAM unless ROM is mapped low
        if (address < 0x02000000) {
            if (this.romMapped) {
                console.debug("Fetching from ROM mapped low");
                return this.rom.getUint32(address, true);
            }

            // This should be mapped logical-to-physical
            console.debug("Fetching from logical RAM");
            return this.ram.getUint32(address, true);
        }

        // Physically mapped RAM
        if (address < 0x03000000) {
            console.debug("Fetching from physical RAM");
            return this.ram.getUint32(address - 0x02000000, true);
        }

        // I/O controllers
        if (address < 0x03400000) {
            console.debug("Fetching from I/O controllers");
            return 0;
        }

        // Low ROM
        if (address < 0x03800000) {
            console.debug("Fetching from low ROM");
            return 0;
        }

        // High ROM
        let romAddress = address - 0x03800000;
        console.debug(`Fetching from high ROM ${romAddress.toString(16)}`);

        if (romAddress < this.rom.byteLength) {
            return this.rom.getUint32(romAddress, true);
        } else {
            console.warn("Read past end of ROM; returning 0");
            return 0;
        }
    }

    store(address: number, data: number): void {
        address &= 0x03fffffc;
        this.ram.setUint32(address, data, true);
    }

    loadByte(address: number): number {
        let word = this.load(address);
        let field = address & 3;
        return (word >> (field * 8)) & 0xff;
    }

    storeByte(address: number, data: number): void {
        let field = address & 3;
        let word = (data & 0xff) << (field * 8);
        this.store(address, word);
    }
}
