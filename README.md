# iqfeed-rs
A client built in Rust to connect with IQFeed's Connection Manager.

As of 5/27/2021 progress has stalled on this as it reached a minimum viability to be used in production. On a decent CPU you will be able to parse trades at under one micro second easily as they come in.

## Unsafe
This library does make use of unsafe where it gives a speed boost. We know the data coming from iqfeed and mostly trust it. If it breaks a lot of things break anyway so the trade off is worth it. You can also fork and remove unsafe if you do not want to use it.

## Endpoints
Currently there's only support for the trade events and timestamp. If you'd like support for fundamentals or any of the other data you can get feel free to open an issue and I will add it.
