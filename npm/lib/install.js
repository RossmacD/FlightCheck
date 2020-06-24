'use strict';

const os = require('os');
const fs = require('fs');
const path = require('path');
const util = require('util');

const download = require('./download');

const fsExists = util.promisify(fs.exists);

const forceInstall = process.argv.includes('--force');
if (forceInstall) {
    console.log('--force, ignoring caches');
}

const VERSION = 'v1.0.6';
const BIN_PATH = path.join(__dirname, '../bin');

let exec = require('child_process').exec;

function getPlatform() {
    const type = os.type();
    const arch = os.arch();
    
    // if (type === 'Windows_NT' && arch === 'x64') return 'win64';
    if (type === 'Windows_NT' && arch === 'x64') {
        return 'x86_64-pc-windows-gnu';}
    if (type === 'Windows_NT') return 'i686-pc-windows-gnu';
    // if (type === 'Linux' && arch === 'x64') return 'linux';
    if (type === 'Linux' && arch === 'x64') return 'x86_64-unknown-linux-gnu';
    if (type === 'Darwin' && arch === 'x64') return 'x86_64-pc-windows-gnu';

    throw new Error(`Unsupported platform: ${type} ${arch} - open up an issue on github and I will add it for you!`);
}



async function main() {
    const binExists = await fsExists(BIN_PATH);
    if (!forceInstall && binExists) {
        console.log('bin/ folder already exists, exiting');
        process.exit(0);
    }

    const opts = {
        version: VERSION,
        // token: process.env['GITHUB_TOKEN'],
        target: getPlatform(),
        destDir: BIN_PATH,
        force: forceInstall
    };
    try {
        await download(opts);
    } catch (err) {
        console.error(`Downloading flightcheck failed: ${err.stack}`);
        process.exit(1);
    }
}

main();