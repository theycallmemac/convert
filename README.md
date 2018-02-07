# convert

[![Build Status](https://travis-ci.org/theycallmemac/convert.svg?branch=master)](https://travis-ci.org/theycallmemac/convert) [![Crates.io](https://img.shields.io/crates/v/convert.svg)](https://crates.io/crates/convert) [![GitHub license](https://img.shields.io/github/license/theycallmemac/dcurooms.svg)](https://github.com/theycallmemac/convert/blob/master/LICENSE)

A command line tool written in Rust which converts between 33 different currencies using real-time information.

#### Version:
To check the current version of __convert__:

```convert -V```

#### Conversions:
To carry out any given conversion between currencies try something like:

```convert 180.50 EUR USD```

This will convert EUR 180.50 to it's USD equivalent using real-time information. 

#### Get Currency Rate:
To get the current rate between to currencies use:

```convert -r JPY ZAR```

In the above example, JPY is acting as teh base currency to get the current rate of ZAR from.

#### List Supported Currencies:
To view the currencies supported by __convert__:

```convert -l```

This returns the a list of currency codes in the form of USD, EUR, JPY... etc.

#### Help

To see how __convert__ works use:

```convert -h```
