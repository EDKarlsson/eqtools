# Bard

- [Bard](https://everquest.allakhazam.com/wiki/EQ:Bard)

## Spells

- [Bard Spells](https://everquest.allakhazam.com/db/spelllist.html?name=&type=brd&level=1&opt=And+Higher&expansion=original&action=search)

## Macros

### 1. The mez melody.

```shell
/stopsong
/melody 6 1 2 3
```

This melody enables a bard to continue meleeing one target while mezzing another target simultaneously. 
To do so, target mob 2, and click this melody. Then hit TAB to go back to your target. Mob 2 is mezzed, and you begin 
singing your three songs. Now, before song 3 finishes, like 1/2 to 1/4 of a second before song 3 finishes, hit your TAB 
key to target mob 2. As soon as song 6, or your mez song, begins, hit TAB again to go back to mob 1. Songs are placed 
on your target you have before the song is begun.

### 2: the 2-target mez melody.

```shell
/stopsong
/melody 6 1 2 6 3 1 6 2 3
```

This melody enables the bard to continue meleeing one target while mezzing two targets. 
The mode is similar to the single target mez, however this time, the bard doesn't use TAB to toggle targets, but 
instead selects the targets with his cursor. 
Remember to target the mob -before- the mez song begins, then go back to the mob you're fighting. 
Keep in mind which mob you have mezzed and which one is due for a refresh. 
The order of the songs in the melody above allows your group to benefit from 3 songs without drops and breaks.

### 3: the dot chant.

```shell
/stopsong
/melody 1 2 3 4 5
```

this is pretty straightforward. however, since our DOT songs last 3 ticks instead of two (or 18 seconds), a bard can twist five DOTs together instead of four, because by the time he finishes song 5 and begins song 1 again, song 1 has -just- worn off. this enables five dots to tick constantly. note, each song in this melody must not be similar ie, only 1 fire, 1 ice, 1 poison, 1 disease, and 1 magic chant.

### 4: the fade spam

```shell
/stopsong
/attack off
/G Fading Aggro!!
/alt activate 212
```

it may seem silly to spam your group with fade messages, but it's actually the polite thing to do.