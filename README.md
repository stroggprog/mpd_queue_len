#mpd_queue_len
This fetches the current queue from mpd and iterates through it, accumulating track length.

It then returns the queue length as a time string (00:00:00).

It's a fairly simple program written in Rust. You just need to compile it and put it somewhere on your path. You might want to edit it first though to change the following line to point at localhost or whatever hostname or IP address your mpd is running on.

```
const HOST: &str = "romeo:6600";
```
Once I wrote this, I thought about calculating how much of the queue has been played, and once you have that and the total length of the queue, you can calculate how much playtime there is remaining. However, this should all be done at once to avoid chugging resources like they're going out of fashion. I'll work on that as a separate project.

I wrote this to get the playtime length of the queue for display in conky. It might have use elsewhere.