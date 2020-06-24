#!/usr/bin/env node
'use strict';
const path = require('path');
const child_process = require('child_process');
const { stdout } = require('process');

const binaryPath = path.join(__dirname, `../bin/flightcheck${process.platform === 'win32' ? '.exe' : ''}`);
let args = process.argv;
args=args.slice(2)
console.log(args)
args=args.map(arg=>{return arg.charAt(0)!=='-'?process.cwd()+arg:arg})
console.log(`Sending binary to ${binaryPath} ${args.length!==0?args.join(" "):''}`)
//Execute the binary with the command line arguments that are passed into node
child_process.exec(`${binaryPath} ${args.length!==0?args.join(" "):''}`, (err, stdout, stderr) => {
    if (err || stdout || stderr) {
        console.log(err || stderr || stdout || "AH yeh");
        console.log(stdout || "AH no")
    } else { console.log("Nothing") }
});