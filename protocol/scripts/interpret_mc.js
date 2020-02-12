// this file takes a decompilation using offical mappings to generate a scaffold for a protocol implementation
// its a bunch of hacks

const fs = require('fs');

const path = process.argv[2] || './';

// <>/DecompilerMC/src/1.15.2/server/net/minecraft/network

const packetCorrections = {
    'handshaking': 'handshake',
    'play': 'game',
}

const packetIds = (() => {
    const protocolFile = fs.readFileSync(`${path}/ConnectionProtocol.java`, 'UTF-8');
    const lines = protocolFile.split("\n").filter(line => line.includes('.addFlow'));
    const maps = Object.fromEntries(lines.map(line => {
        const segments = line.split('.addFlow');
        const name = segments[0].trim().toLowerCase().match(/^[a-z]+/)[0];

        const parseSegment = (segment) => {
            const rawPackets = segment.match(/\.addPacket\(.*?\)/g);
            const idOrder = rawPackets.map(packet => packet.match(/\(([a-zA-Z0-9_.]+?)\.class/)[1]);
            return idOrder;
        }

        const serverboundLine = segments.find(segment => segment.includes('PacketFlow.SERVERBOUND'));
        const clientboundLine = segments.find(segment => segment.includes('PacketFlow.CLIENTBOUND'));

        let serverbound = [];
        if (serverboundLine != null) {
            serverbound = parseSegment(serverboundLine);
        }
        let clientbound = [];
        if (clientboundLine != null) {
            clientbound = parseSegment(clientboundLine);
        }
        return [
            packetCorrections[name] || name,
            {
                clientbound: Object.fromEntries(clientbound.map((packetName, i) => [packetName, i])),
                serverbound: Object.fromEntries(serverbound.map((packetName, i) => [packetName, i])),
            }
        ];
    }));
    return maps;
})();

const packets = {};

const parsePacket = java => {
    const rawReadFunc = java.match(/public void read.*((?:\n      [^ ].*)*)\n   }/);
    const readFunc = rawReadFunc == null ? null : rawReadFunc[1].trim().split("\n").map(line => line.trim()).filter(line => line.length > 0);
    const rawWriteFunc = java.match(/public void write.*((?:\n      [^ ].*)*)\n   }/);
    const writeFunc = rawWriteFunc == null ? null : rawWriteFunc[1].trim().split("\n").map(line => line.trim()).filter(line => line.length > 0);
    const fields = java.match(/(?:public|private|protected) [a-zA-Z.<>\[\]]+ [a-zA-Z_]+(?: = .*?)?;/g) || [];
    
    if (fields.length == 0 && (readFunc != null && readFunc.length > 0 || writeFunc != null && writeFunc.length > 0)) { // no data can be gathered
        return {
            type: 'unknown',
        };
    }
    const struct = Object.fromEntries(fields.map(field => {
        const [_, type, name] = field.match(/(?:public|private|protected) ([a-zA-Z.<>\[\]]+) ([a-zA-Z_]+)/);
        return [name, type];
    }));
    let readOps = null;
    let writeOps = null;
    if (readFunc != null) {
        readOps = [];
        for (const line of readFunc) {
            if (!line.includes('var1.read')) {
                readOps.push({
                    type: 'extra',
                    raw: line,
                });
                continue;
            }
            const simpleMatch = line.match(/^this.([a-zA-Z0-9_]*) = var1.read([a-zA-Z0-9_]*)\(\);$/);
            if (simpleMatch != null) {
                readOps.push({
                    type: simpleMatch[2],
                    field: simpleMatch[1],
                });
                continue;
            }
            const utfMatch = line.match(/^this.([a-zA-Z0-9_]*) = var1.readUtf\(([0-9]+)\);$/);
            if (utfMatch != null) {
                readOps.push({
                    type: 'Utf',
                    field: utfMatch[1],
                    max: parseInt(utfMatch[2]),
                });
                continue;
            }
            readOps.push({
                type: 'raw',
                raw: line,
            });
        }
    }
    if (writeFunc != null) {
        writeOps = [];
        for (const line of writeFunc) {
            if (!line.includes('var1.write')) {
                writeOps.push({
                    type: 'extra',
                    raw: line,
                });
                continue;
            }
            const simpleMatch = line.match(/^var1.write([a-zA-Z0-9_]*)\(this.([a-zA-Z0-9_]*)\);$/);
            if (simpleMatch != null) {
                writeOps.push({
                    type: simpleMatch[1],
                    field: simpleMatch[2],
                });
                continue;
            }
            writeOps.push({
                type: 'raw',
                raw: line,
            });
        }
    }
    return {
        type: 'guess',
        struct,
        writeOps,
        readOps,
    };
};

const addPackets = (dir, typeName, output) => {
    output.serverbound = [];
    output.clientbound = [];
    for (const file of fs.readdirSync(dir)) {
        const filePath = `${dir}/${file}`;
        let lstat = fs.lstatSync(filePath);
        if (!lstat.isFile()) {
            throw new Error('unexpected non-file: ' + filePath);
        }
        const packetName = file.replace('.java', '');
        const cleanPacketName = packetName.replace('Serverbound', '').replace('Clientbound', '');
        if (file.startsWith('Serverbound') || packetName == 'ClientIntentionPacket') {
            const packetId = packetIds[typeName].serverbound[packetName];
            output.serverbound[packetId] = { name: cleanPacketName, ...parsePacket(fs.readFileSync(filePath, 'UTF-8')) };
        } else if (file.startsWith('Clientbound')) {
            const packetId = packetIds[typeName].clientbound[packetName];
            output.clientbound[packetId] = { name: cleanPacketName, ...parsePacket(fs.readFileSync(filePath, 'UTF-8')) };
        } else {
            // ignore
        }
    }
};

const packetsPath = `${path}/protocol`;

for (const dir of fs.readdirSync(packetsPath)) {
    const dirPath = `${packetsPath}/${dir}`;
    if (fs.lstatSync(dirPath).isDirectory()) {
        packets[dir] = {};
        addPackets(dirPath, dir, packets[dir]);
    }
}
console.log(JSON.stringify(packets, null, 2));

const flatPackets = [].concat.apply([], [].concat.apply([], Object.values(packets).map(state => Object.values(state))));

console.error(`${flatPackets.length} packets found`);
console.error(`${flatPackets.filter(packet => packet.type == 'unknown').length} packets could not have fields identified: ${flatPackets.filter(packet => packet.type == 'unknown').map(packet => packet.name).join(', ')}`);
console.error(`All read types: ${[].concat.apply([], flatPackets.map(packet => {
    if (packet.readOps == null) {
        return [];
    } else {
        return packet.readOps.map(op => op.type);
    }
})).filter((x, i, self) => self.indexOf(x) == i).join(", ")}`);
console.error(`All written types: ${[].concat.apply([], flatPackets.map(packet => {
    if (packet.writeOps == null) {
        return [];
    } else {
        return packet.writeOps.map(op => op.type);
    }
})).filter((x, i, self) => self.indexOf(x) == i).join(", ")}`);
console.error(`All struct types: ${[].concat.apply([], flatPackets.map(packet => {
    return Object.values(packet.struct);
})).filter((x, i, self) => self.indexOf(x) == i).join(", ")}`);
console.error(`Unknown read ops: \n${[].concat.apply([], flatPackets.map(packet => {
    if (packet.readOps == null) {
        return [];
    } else {
        return packet.readOps.filter(op => op.type == 'raw').map(op => op.raw);
    }
})).filter((x, i, self) => self.indexOf(x) == i).join("\n")}`);
console.error(`Unknown write ops: \n${[].concat.apply([], flatPackets.map(packet => {
    if (packet.writeOps == null) {
        return [];
    } else {
        return packet.writeOps.filter(op => op.type == 'raw').map(op => op.raw);
    }
})).filter((x, i, self) => self.indexOf(x) == i).join("\n")}`);

