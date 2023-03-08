# mcrust
Minecraft plugin example using Rust.
This plugin writes *simple welcome message* when you join the server

## How to use
1. Compile using `cargo build --release`
2. Copy file **libmcrust.rlib** to *Your Minecraft Server Dir*\**libs**
3. Create file **libconf.json**
4. Add the content to this file as shown below:
```json
{
    "libs": {
        "libmcrust.rlib": {
            "name": "MCRust",
            "execute-class": "WelcomeMessage",
            "main-func": true,
            "imports-list": 0
        }
    }
}
```
5. Place **libconf.json** in the **libmcrust.rlib** dir
6. *(NEW)* ⚠️ If you're planning to use it on Purpur, download updated sources, compile and change **libconf.json** to:
```json
{
    "libs": {
        "libmcrust.rlib": {
            "name": "MCRust",
            "execute-class": "WelcomeMessage",
            "main-func": true,
            "imports-list": 0,
            "entry-point": "main",
            "load-var": "isLoaded",
            "default-val-of-load-var": false
        }
    }
}
```
