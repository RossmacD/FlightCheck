#!/usr/bin/env node
'use strict';
const path = require('path');
const child_process = require('child_process');

binaryPath = path.join(__dirname, `../bin/flightcheck${process.platform === 'win32' ? '.exe' : ''}`);

//Execute the binary with the command line arguments that are passed into node
child_process.exec(`${binaryPath} ${process.argv.join(" ")}`, err => {
    if (err) {
        reject(err);
        return;
    }
    resolve();
});