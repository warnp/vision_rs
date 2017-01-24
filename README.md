# vision_rs [![Build Status](https://travis-ci.org/warnp/vision_rs.svg?branch=master)](https://travis-ci.org/warnp/vision_rs)
A  reimplementation of warnp/visionneuse in rust.

## Description
A simple web photo gallery written in rust with iron, maud and serde. Actually only working with the nightly build due to maud. It produce a docker image wich is runnable directly on the server. For the moment it's only for demonstration purpose and it's not mean't to become a framework or whatever.
Also it doesn't support any kind of database for the moment.

## Usage
Just modifiy the dockerfile at your conveniance, pull the image from docker hub or wherever it's hosted, and go to `localhost:3000`. You can modify the static image database in `content/text.json`.
