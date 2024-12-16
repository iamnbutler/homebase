---
title: Adding a Favicon in 2024
date: 2024-02-27
tags:
  - webdev
series: Astro Site 2024
---

Let's go on a quick side quest to add a favicon to this new site. These always seem to be a bit of a pain because the vendor requirements are always changing. Evil Martians have a really great writeup that covers a lot of this here: [How to Favicon in 2024: Six files that fit most needs
](https://evilmartians.com/chronicles/how-to-favicon-in-2021-six-files-that-fit-most-needs).

We'll mostly follow that, with a few Astro-isms tossed in.

To start, we'll need to create an asset for our favicon. I'd actually recommend calling this something other than `favicon` to prevent the browser from loading it in a way you don't expect, due to it looking for things named `favicon`.

In the end, these are the assets I ended up with:

```
public/
  apple-touch-icon.png    // 180x180 for iOS
  favicon.ico             // 32x32 for any non-svg supporting browsers
                          // You may want a custom 16x16 icon if your
                          // favicon won't look good scaled down automatically
  icon.svg                // More on this below
  icon-dev.svg            // Alternate icon to indicate development environments
  icon_192.png            // 192x192 for Android
  icon_512.png            // 512x512 for Android
  manifest.webmanifest    // Web App Manifest
```

## The SVG Favicon

`icon.svg` does double duty as both the light and dark mode icon.

```html
<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" fill="none">
  <style>
    @media (prefers-color-scheme: dark) {
      .a {
        fill: #fff;
      }
    }
  </style>
  <path
    fill="#000"
    fill-rule="evenodd"
    d="M20 4h-4v8h-4V4H8v12H4v4h4v4H4v4h24v-4H12v-8h16V4h-4v8h-4z"
    class="a"
    clip-rule="evenodd"
  />
</svg>
```

We use a media query to change the fill color of the icon based on the user's preferred color scheme. This means that even if a user's browser uses a dark color tab bar, the favicon will still look good.

This approach feels a bit more elegant than swapping out the icon with JavaScript, or having multiple icons in the markup.

## The Dev-Mode Favicon

A cool little trick I picked up from the Evil Martians article is to use a different icon in development environments. The approach they used was a little different, involving a custom webpack plugin, but we can do something far easier in Astro.

Here is my current `head.astro` file:

```astro
<!-- src/components/head.astro -->
---
interface HeadProps {
    title: string;
}

const is_dev = import.meta.env.DEV;
const { title } = Astro.props as HeadProps;
---

<head>
    <meta charset="utf-8" />

    <link rel="manifest" href="/manifest.webmanifest" />
    <link rel="icon" href="/favicon.ico" sizes="32x32" />
    <!-- Show a different favicon if in development env -->
    <link rel="icon" href={`/icon${is_dev && "-dev"}.svg`} type="image/svg+xml" />
    <link rel="apple-touch-icon" href="/apple-touch-icon.png" />

    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <meta name="generator" content={Astro.generator} />

    <title>{title}</title>
</head>
```

Astro exposes these handy [base environment variables](https://docs.astro.build/en/guides/environment-variables/) that we can use to conditionally load different assets based on the environment.

Using them we can conditionally load the `icon-dev.svg` in development environments, and the regular `icon.svg` in production to make it super clear from the tab bar which environment we're in.

I love this so much as I constantly get mixed up with a prod tab open, and spend time debugging only to realize I'm in the wrong environment... maybe just me 😅

## The Web App Manifest

The web app manifest is a JSON file that tells the browser about your web application and how it should behave when 'installed' on the user's mobile device or desktop.

Mine is pretty simple:

```json
{
  "name": "nate.rip",
  "icons": [
    { "src": "/icon-192.png", "type": "image/png", "sizes": "192x192" },
    { "src": "/icon-512.png", "type": "image/png", "sizes": "512x512" }
  ]
}
```

This will ensure places that use it will get the higher definition icons, and the browser will know what to do when the user tries to add the site to their home screen.

And that is it!

Let me know if I missed anything...

There are a million different sizes and shapes for favicons, but I feel like this is a resonable amount of coverage for something simple like a personal site or blog.
