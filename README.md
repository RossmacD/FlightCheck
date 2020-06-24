[![Build Status](https://travis-ci.com/RossmacD/FlightCheck.svg?branch=master)](https://travis-ci.com/RossmacD/FlightCheck)
# FlightCheck ✈✔
> A simple Command line checklist

Flightcheck is a CLI tool for creating a checklist that must be completed before another command is run.
This is useful for sanity checking certain actions such as publishing to npm, pushing to the master/production branch and more

## Installation
Install globally with:

```npm install -g flightcheck```

or in a project with

```npm install -D flightcheck```

## Use
Flightcheck will run through any file line by line as a checklist requiring confirmation before proceeding.
Prefix any command with `flightcheck &&` and if the checklist is not complete the next command will not run

By default flightcheck will look for a `.fcheck` file to use as a checklist but a path can be passed in as an argument:

For example:
```
    flightcheck prePublishChecklist.txt && npm publish
```

## Example:
Before publishing to npm you must update your version number and update documentation. This can be done by creating a file called `.fcheck` at the root of your npm package containing these items:



*`.fcheck`*
```
Update version number
Update documentation
```

Then create a script in your `package.json`

```
"scripts": {
    "pub": "flightcheck && npm publish",
  },
```
When the command `npm run pub` is run the command line will prommpt the user to check off each item on the list by entering `y` or `Y`
If the publisher does not complete the checklist the next command: `npm publish` will not run

## Issues
If there are any issues with the tool feel free to leave an issue or pull request to the library

### Development
Flightcheck is a CLI tool built with rust and distributed using npm.
To develop flightcheck rust must be installed.
