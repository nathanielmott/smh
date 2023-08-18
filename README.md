# smh: a head-shake tracker

smh is a command-line utility that allows you to keep track of how many times you've shaken your head at something and why. It started as a joke, and it still kind of is.

## usage

You can shake your head at something using the `at` command:

```
smh at dad for making a terrible joke
```

You can then view an entry for a particular subject using the `entry` command:

```
smh entry dad

// outputs

Name: dad, Count: 1, Reasons: {"Aug 18, 2023 15:13": "making a terrible joke"}

```

(Note that the date an entry was added is automatically generated.)

You can view an entire log with the, well, `log` command:

```
smh log

// outputs

Name: dad, Count: 1, Reasons: {"Aug 18, 2023 15:13": "making a terrible joke"}
Name: example, Count: 1, Reasons: {"Aug 18, 2023 15:15": "idk lol"}
```

You can also remove a subject using the `remove` command. (Note that there isn't currently--and might not ever be--a way to remove a particular reason for s'ing your h.)

```
smh remove example
```

---

## why does this exist?

I thought "smh" would make a good command for a CLI. Then I realized an "smh" tracker could prove useful when I want to remember silly jokes, frustrating things about particular software, etc. and decided to make this utility as a learning project for creating CLIs, managing files, and working with JSON in Rust. So here we are!

## will this work on my system?

Beats me!

---

## roadmap

smh is a learning project; its features are dictated by my progress through learning Rust, my expectations for a utility of this type, and a certain amount of whimsy.

That said, the following changes at least seem interesting:

* making it so printing the reasons for shaking your head at a subject isn't terrible
* adding the ability to remove a particular reason rather than the entire subject
* creating a backup of the data file to minimize the chance of losing data

These features might never arrive, however, and this utility may remain forever cursed.