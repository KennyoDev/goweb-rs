## Installation
To install, just run

```
cargo install --path .
```

## Basic usage

```
goweb duck
```

This will open DuckDuckGo on Firefox.
<br>
<br>
(You can see the keywords and the associated websites in json/webSites.json)

To open in a specific browser, you can call it like this

```
goweb -f duck 
```
To open DuckDuckGo in Firefox

```
goweb -c duck
```
To open DuckDuckGo in Chromium

```
goweb -z duck
```
To open DuckDuckGo in Zen-Browser
