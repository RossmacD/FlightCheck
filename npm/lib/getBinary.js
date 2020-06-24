const { Binary } = require('binary-install');
const os = require('os');

function getPlatform() {
    const type = os.type();
    const arch = os.arch();

    // if (type === 'Windows_NT' && arch === 'x64') return 'win64';
    if (type === 'Windows_NT' && arch === 'x64') return 'x86_64-pc-windows-gnu';
    if (type === 'Windows_NT') return 'i686-pc-windows-gnu';
    // if (type === 'Linux' && arch === 'x64') return 'linux';
    if (type === 'Linux' && arch === 'x64') return 'x86_64-unknown-linux-gnu';
    if (type === 'Darwin' && arch === 'x64') return 'x86_64-pc-windows-gnu';

    throw new Error(`Unsupported platform: ${type} ${arch}`);
}

function getBinary() {
    const version = require('../package.json').version;
    const url = `https://https://github.com/RossmacD/FlightCheck/releases/download/v${ version }/FlightCheck-Release-${getPlatform()}.tar.gz`;
    const name = 'flightcheck';
    return new Binary(url, { name });
}

module.exports = getBinary;