## The Project

Music Stats is a visual art & infographic site. It's generated from my personal music listening history, showing 3 axis of data – total plays, recency of track, and "hotness" – custom score based on how frequently a song is played in relation to when it was added.

content/posts/Music Stats.md

Individual bars represent songs. Taller bars are more frequently played, while warmer colored ones are hotter (played very frequently.)

![](https://res.cloudinary.com/yaminateo/image/upload/v1636936185/post/music-stats/Screenshot_2015-10-20_22.39.09_dhxriu.png)

Clicking on a bar opens a modal with its info, and does a search for the song on Youtube.

## The Tech

![](https://res.cloudinary.com/yaminateo/image/upload/v1636936182/post/music-stats/Screenshot_2015-10-17_05.02.35_p6cfwi.jpg)

The site is built in Javascript using [d3.js](https://d3js.org/) to generate the charts from TSV data sets, and a custom implementation of the Youtube API & I'm Feeling Lucky to grab the first video available. [Check out the script](https://github.com/iamnbutler/music-stats/blob/master/js/scripts.js).

![](https://res.cloudinary.com/yaminateo/image/upload/v1636936183/post/music-stats/Screenshot_2015-10-17_19.23.28_yl2nli.png)

The gradient on each bar is unique – it's created by an SVG polyfill filter generated using each song's plays, recency and hotness.

## The Result

As a visual piece I think the work was sucessful – to show off interesting listening trends visually. I wish I could have got filtering built in time so the data could be explored futher – I think this overall would have made the piece more exciting over all.

I had plans to make it so anyone could add their own TSV/CSV to see their own results, and did get this working in code, but didn't have time to solve how to allow people to upload their own data. Maybe in the future!

[Go to Site](http://iamnbutler.github.io/music-stats/) [View on Github](https://github.com/iamnbutler/music-stats)
