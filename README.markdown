# UTF RENDER :hindu_temple: :scroll:

***A special web app to render fancy UTF-8 sequences. :hindu_temple: :scroll:***

![GitHub CI](https://github.com/angeldollface/utf-render/actions/workflows/yew.yml/badge.svg)

## ABOUT :books:

Emojis and fancy symbols are part of the UTF-8 character standard (The unicode standard.). Each character has a special unicode sequence assocociated with it. To render this character out, you need the sequence. 
This app helps you by rendering the sequence as a symbol for you to copy  and use for whatever you want. This repository contains some 

## DEPLOYED PROJECT ON GITHUB PAGES :rocket:

To view a live deployed version of this project, click here: [VIEW](https://angeldollface.art/utf-render)

## USAGE :hammer:

- 1.) Visit [this link](https://angeldollface.art/utf-render).
- 2.) Get some unicode sequences at [this link](https://unicode-table.com/)! (They should look something like `U+13080`.)
- 3.) Copy them.
- 4.) Type *one* of them into the input field.
- 5.) Click "RENDER"!
- 6.) The symbol should be rendered and copied to your clipboard for you to use.

## TRY THE CODE FOR YOURSELF! :inbox_tray:

You should have the following tools installed and available from the command line:

- Rust
- Git

To try *UTF Render* on your own machine, follow these steps:

- 1.) Install `trunk` from [crates.io](https://crates.io/crates/trunk):

```bash
cargo install trunk
```

- 2.) Clone this project's source code:

```bash
git clone https://github.com/angeldollface/utf-render
```

- 3.) Change directory into the source code's root directory:

```bash
cd utf-render
```

- 4.) Serve the app locally (This will serve the app locally on [`http://127.0.0.1:8080/utf-render/`](http://127.0.0.1:8080/utf-render/).):

```bash
trunk --config trunk.toml serve --release
```

- 5.) If you want to build the app into a bundle to deploy to a server, run the command below. This will produce a directory called `dist` with the bundle inside it.

```bash
trunk --config trunk.toml build --release
```

- 5.) Enjoy! :heart_on_fire:


## CHANGELOG :black_nib:

### Version 1.0.0

- Initial release.
- Upload to GitHub.
- Deployment to GitHub Pages.

## NOTE :scroll:

- *UTF Render :hindu_temple: :scroll:* by Alexander Abraham :black_heart: a.k.a. *"Angel Dollface" :dolls: :ribbon:*
- Licensed under the MIT license.
