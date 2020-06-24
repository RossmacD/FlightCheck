#!/usr/bin/env node
'use strict';

const path = require('path');

module.exports.rgPath = path.join(__dirname, `../bin/flightcheck${process.platform === 'win32' ? '.exe' : ''}`);