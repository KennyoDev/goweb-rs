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
(You can see the keywords and the associated websites by running goweb -a)

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

<br>

To add a new site run it like this
```
goweb -n <keyword> <url>

e.g.

goweb -n test example.com
```

To delete a site, do like this
```
goweb -d <keyword>

e.g.

goweb -d test
```

To display all sites run
```
goweb -a
```
