#!/usr/bin/env node
'use strict';
const path = require('path');
const {spawn} = require('child_process');

// Get the path of the rust binary - generated on install
const binaryPath = path.join(__dirname, `../bin/flightcheck${process.platform === 'win32' ? '.exe' : ''}`);
// Get the commands that are passed into 
let args = process.argv;
// The first two arguments are irrelevant - its the original arguments that were passed into the node script
args=args.slice(2)
// console.log(args)

// Where item is not prefixed with `-` assume its a path, prefix the current directory that the command was called from
args=args.map(arg=>{return arg.charAt(0)!=='-'?process.cwd()+"/"+arg:arg})
// console.log(`Sending binary to ${binaryPath} ${args.length!==0?args.join(" "):''}`)

// Spawn the binary in a new process and pipe the input / output between the two
let child = spawn(`${binaryPath}`,args)
// child.stdin.setEncoding('utf-8');
child.stdout.pipe(process.stdout);
child.stderr.pipe(process.stderr);
process.stdin.pipe(child.stdin);